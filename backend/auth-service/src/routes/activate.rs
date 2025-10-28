use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::{StatusCode, HeaderMap, header::ACCEPT_LANGUAGE},
    response::IntoResponse,
    routing::get,
};
use uuid::Uuid;
use shared_types::i18n;

use crate::{
    AppState, database,
    models::response::{Error, Message},
};

/// Creates a router for the activate routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(activate_account_handler))
        .with_state(state)
}

/// Activate account endpoint handler
///
/// Activates the account if the id is valid
///
/// # Returns
/// * JSON response with message "Account has been verified." if sucessfull
/// * JSON response with message "Account link is invalid." if unsucessfull
///
/// # Example Response
/// ```json
/// {
///     "message": "Account has been verified."
/// }
/// ```
async fn activate_account_handler(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, Error> {
    // Extract language from Accept-Language header
    let lang = i18n::extract_language(
        headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok()),
    );

    // Get the activation link from the db
    let pool = state.get_database_pool();
    let activation_link = database::activation_links::filter_and_delete_by_id(id, pool).await?;

    // Set the account as verified and delete the activation link
    if database::users::set_verified(activation_link.get_uuid(), pool).await? != 1 {
        let msg = i18n::translate("auth.user_not_exist", &lang);
        return Err((StatusCode::BAD_REQUEST, msg.as_str()).into());
    }

    Ok(Json(Message {
        message: "Account has been verified.".into(),
    }))
}
