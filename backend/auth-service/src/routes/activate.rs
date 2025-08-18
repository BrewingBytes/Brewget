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
    models::response::{error::Error, message::Message},
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
    // Get the activation link from the db
    let mut conn = state.get_database_connection().await?;
    let activation_link =
        database::activation_links::filter_and_delete_by_id(id, &mut conn).await?;

    // Set the account as verified and delete the activation link
    if database::users::set_verified(activation_link.get_uuid(), &mut conn).await? != 1 {
        return Err((StatusCode::BAD_REQUEST, "User does not exist.").into());
    }

    Ok(Json(Message {
        message: "Account has been verified.".into(),
    }))
}
