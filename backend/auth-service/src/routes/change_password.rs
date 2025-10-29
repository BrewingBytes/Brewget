use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    AppState, database,
    models::{
        request::reset_password_info::ResetPasswordInfo,
        response::{Error, Message},
    },
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
    Json(body): Json<ResetPasswordInfo>,
) -> Result<impl IntoResponse, Error> {
    // Get the forgot password link from the db
    let pool = state.get_database_pool();
    let link = database::forgot_password_links::filter_by_id(body.id, pool).await?;

    // If the link is expired, remove it from the database and send a BAD_REQUEST
    if link.is_expired() {
        database::forgot_password_links::delete(body.id, pool).await?;
        return Err((StatusCode::BAD_REQUEST, "Link is expired.").into());
    }

    // Check if the password is ok and hash it
    validate_password(&body.password)
        .map_err(|s| -> Error { (StatusCode::BAD_REQUEST, s.as_str()).into() })?;

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
        return Err((
            StatusCode::BAD_REQUEST,
            "Password cannot be the same as any of your recently used passwords.",
        )
            .into());
    }

    let new_hashed_password = hash_password(&body.password).map_err(|_| -> Error {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again!",
        )
            .into()
    })?;

    // Change the password of the user
    database::users::change_password(link.get_uuid(), new_hashed_password.clone(), pool).await?;

    // Store the new password in history
    database::password_history::insert(link.get_uuid(), new_hashed_password, pool).await?;

    // Cleanup old password history entries beyond the limit
    database::password_history::cleanup_old_passwords(
        link.get_uuid(),
        password_history_limit,
        pool,
    )
    .await?;

    // Delete the forgot password link from the db
    if database::forgot_password_links::delete(body.id, pool).await? != 1 {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not delete from the database.",
        )
            .into());
    }

    Ok(Json(Message {
        message: "Password sucessfully changed.".into(),
    }))
}
