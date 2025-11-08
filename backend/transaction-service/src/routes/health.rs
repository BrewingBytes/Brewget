use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};

use crate::{
    AppState,
    models::response::{DatabaseConnection, Health, HealthStatus},
};

/// Creates a router for the health routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(health_checker_handler))
        .with_state(state)
}

/// Health check endpoint handler
///
/// Returns a health message indicating the service is operational
///
/// # Returns
/// JSON response with a health message
async fn health_checker_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pool = state.get_database_pool();
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => Json(Health {
            status: HealthStatus::Healthy,
            database: Some(DatabaseConnection::Connected),
            version: env!("CARGO_PKG_VERSION").into(),
        })
        .into_response(),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(Health {
                status: HealthStatus::Unhealthy,
                database: Some(DatabaseConnection::Disconnected),
                version: env!("CARGO_PKG_VERSION").into(),
            }),
        )
            .into_response(),
    }
}
