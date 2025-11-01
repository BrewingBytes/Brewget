use axum::{Json, Router, routing::get};
use shared_types::response::{Health, HealthStatus};

/// Creates a router for the health routes
pub fn get_router() -> Router {
    Router::new().route("/", get(health_checker_handler))
}

/// Health check endpoint handler
///
/// Returns a health message indicating the service is operational
///
/// # Returns
/// JSON response with a health message
///
/// # Example Response
/// ```json
/// {
///     "status": "Healthy",
///     "version": "0.0.9"
/// }
/// ```
async fn health_checker_handler() -> Json<Health> {
    Json(Health {
        status: HealthStatus::Healthy,
        database: None,
        version: env!("CARGO_PKG_VERSION").into(),
    })
}
