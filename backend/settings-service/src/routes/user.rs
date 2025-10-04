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
    models::{response::error::Error, settings::UpdateSettings},
};

/// Creates a router for the user routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(get_user_settings))
        .route("/update/{id}", post(update_user_settings))
        .with_state(state)
}

/// Get user settings
///
/// # Returns
/// * JSON response with UserSettings
async fn get_user_settings(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let conn = &mut state.get_database_connection().await?;
    let settings = database::settings::find_by_uuid(id, conn).await?;

    Ok(Json(settings))
}

/// Update user settings
///
/// # Returns
/// * JSON response with updated UserSettings
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
