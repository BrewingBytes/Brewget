use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppState, database,
    models::{response::Error, settings::UpdateSettings},
};

/// Creates a router for the user settings routes
///
/// This function sets up the user settings endpoints and returns a configured Axum router.
///
/// # Arguments
///
/// * `state` - Shared application state containing configuration and database connection
///
/// # Returns
///
/// Returns an Axum router configured with the user settings endpoints.
///
/// # Routes
///
/// - `GET /{id}` - Retrieve user settings by user ID
/// - `POST /update/{id}` - Update user settings by user ID
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(get_user_settings))
        .route("/update/{id}", post(update_user_settings))
        .with_state(state)
}

/// Retrieves user settings by user ID
///
/// This endpoint fetches the settings for a specific user. If no settings exist
/// for the user, default settings will be created and returned.
///
/// # Arguments
///
/// * `id` - The UUID of the user whose settings to retrieve
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(Json<Settings>)` - The user's settings as JSON
/// * `Err(Error)` - Database operation error
///
/// # Example Request
///
/// ```http
/// GET /user/550e8400-e29b-41d4-a716-446655440000
/// ```
///
/// # Example Response
///
/// ```json
/// {
///     "user_id": "550e8400-e29b-41d4-a716-446655440000",
///     "language": "en",
///     "currency": "USD",
///     "alarm_set": false,
///     "alarm_time": "08:00:00",
///     "alarm_offset_minutes": 0,
///     "night_mode": false
/// }
/// ```
async fn get_user_settings(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let conn = &mut state.get_database_connection().await?;
    let settings = database::settings::find_by_uuid(id, conn).await?;

    Ok(Json(settings))
}

/// Updates user settings by user ID
///
/// This endpoint allows partial updates to user settings. Only the fields
/// provided in the request body will be updated, leaving other fields unchanged.
///
/// # Arguments
///
/// * `id` - The UUID of the user whose settings to update
/// * `state` - Shared application state
/// * `settings` - The settings update data (only non-None fields will be updated)
///
/// # Returns
///
/// * `Ok(Json<Settings>)` - The updated user settings as JSON
/// * `Err(Error)` - Database operation error
///
/// # Example Request
///
/// ```http
/// POST /user/update/550e8400-e29b-41d4-a716-446655440000
/// Content-Type: application/json
///
/// {
///     "language": "es",
///     "night_mode": true,
///     "alarm_time": "09:30:00"
/// }
/// ```
///
/// # Example Response
///
/// ```json
/// {
///     "user_id": "550e8400-e29b-41d4-a716-446655440000",
///     "language": "es",
///     "currency": "USD",
///     "alarm_set": false,
///     "alarm_time": "09:30:00",
///     "alarm_offset_minutes": 0,
///     "night_mode": true
/// }
/// ```
async fn update_user_settings(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(settings): Json<UpdateSettings>,
) -> Result<impl IntoResponse, Error> {
    let conn = &mut state.get_database_connection().await?;
    database::settings::update(id, settings, conn).await?;

    let settings = database::settings::find_by_uuid(id, conn).await?;

    Ok(Json(settings))
}
