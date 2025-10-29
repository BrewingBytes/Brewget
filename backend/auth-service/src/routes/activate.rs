use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use uuid::Uuid;

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
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Account activation request for link_id: {}", id);

    // Get the activation link from the db
    let pool = state.get_database_pool();
    tracing::debug!("Fetching activation link from database for link_id: {}", id);
    let activation_link = database::activation_links::filter_and_delete_by_id(id, pool).await?;

    // Set the account as verified and delete the activation link
    tracing::debug!(
        "Setting account as verified for user_id: {}",
        activation_link.get_uuid()
    );
    if database::users::set_verified(activation_link.get_uuid(), pool).await? != 1 {
        tracing::error!("User does not exist for activation link_id: {}", id);
        return Err((StatusCode::BAD_REQUEST, "User does not exist.").into());
    }

    tracing::info!(
        "Account activation successful for user_id: {}",
        activation_link.get_uuid()
    );
    Ok(Json(Message {
        message: "Account has been verified.".into(),
    }))
}
