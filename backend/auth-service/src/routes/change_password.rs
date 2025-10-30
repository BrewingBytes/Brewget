use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
};
use shared_types::{Error, Message, TranslationKey, extract_language_from_headers};

use crate::{
    AppState, database,
    models::request::reset_password_info::ResetPasswordInfo,
    utils::password::{hash_password, is_password_in_history, validate_password},
};

/// Creates a router for the change password routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(change_password_handler))
        .with_state(state)
}

/// Change password endpoint handler
///
/// # Returns
/// JSON response with "Password sucessfully changed." if the password was changed.
///
/// # Example Response
/// ```json
/// {
///     "message": "Password sucessfully changed."
/// }
/// ```
async fn change_password_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<ResetPasswordInfo>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Password change request for link_id: {}", body.id);

    // Get the forgot password link from the db
    let pool = state.get_database_pool();
    tracing::debug!(
        "Fetching forgot password link from database for link_id: {}",
        body.id
    );
    let link = database::forgot_password_links::filter_by_id(body.id, pool).await?;

    // If the link is expired, remove it from the database and send a BAD_REQUEST
    if link.is_expired() {
        tracing::warn!("Expired forgot password link used: {}", body.id);
        database::forgot_password_links::delete(body.id, pool).await?;
        return Err(Error::translated(
            StatusCode::BAD_REQUEST,
            TranslationKey::LinkExpired,
            Some(&headers),
        ));
    }

    // Check if the password is ok and hash it
    validate_password(&body.password).map_err(|key| {
        tracing::warn!(
            "Invalid password format for password change, link_id: {}",
            body.id
        );
        Error::translated(StatusCode::BAD_REQUEST, key, Some(&headers))
    })?;

    // Check if the password has been used in recent passwords
    let password_history_limit = state.config.password_history_limit;
    let recent_passwords = database::password_history::get_recent_passwords(
        link.get_uuid(),
        password_history_limit,
        pool,
    )
    .await?;
    let recent_hashes: Vec<String> = recent_passwords
        .iter()
        .map(|ph| ph.get_password_hash())
        .collect();

    if is_password_in_history(&body.password, &recent_hashes) {
        tracing::warn!("Password reuse attempt for user_id: {}", link.get_uuid());
        return Err(Error::translated(
            StatusCode::BAD_REQUEST,
            TranslationKey::PasswordInHistory,
            Some(&headers),
        ));
    }

    tracing::debug!("Hashing new password for user_id: {}", link.get_uuid());
    let new_hashed_password = hash_password(&body.password).map_err(|_| {
        tracing::error!("Failed to hash password for user_id: {}", link.get_uuid());
        Error::translated(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalError,
            Some(&headers),
        )
    })?;

    // Use a transaction to ensure atomicity of password update and history insertion
    let mut tx = pool.begin().await.map_err(|_| {
        Error::translated(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::DatabaseError,
            Some(&headers),
        )
    })?;

    // Change the password of the user
    database::users::change_password(link.get_uuid(), new_hashed_password.clone(), &mut *tx)
        .await?;

    // Store the new password in history
    database::password_history::insert(link.get_uuid(), new_hashed_password, &mut *tx).await?;

    // Cleanup old password history entries beyond the limit
    database::password_history::cleanup_old_passwords(
        link.get_uuid(),
        password_history_limit,
        &mut *tx,
    )
    .await?;

    // Commit the transaction
    tx.commit().await.map_err(|_| {
        tracing::error!(
            "Failed to commit password change transaction for user_id: {}",
            link.get_uuid()
        );
        Error::translated(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::DatabaseError,
            Some(&headers),
        )
    })?;

    // Delete the forgot password link from the db
    tracing::debug!("Deleting forgot password link: {}", body.id);
    if database::forgot_password_links::delete(body.id, pool).await? != 1 {
        tracing::error!("Failed to delete forgot password link: {}", body.id);
        return Err(Error::translated(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::DatabaseError,
            Some(&headers),
        ));
    }

    tracing::info!(
        "Password change successful for user_id: {}",
        link.get_uuid()
    );

    // Use centralized language extraction
    let lang = extract_language_from_headers(Some(&headers));
    let translator = shared_types::Translator::from_code(lang);
    Ok(Json(Message {
        message: translator.translate(TranslationKey::PasswordChanged),
    }))
}
