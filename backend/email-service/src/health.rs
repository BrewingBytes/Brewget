use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};

/// Health status response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Health {
    pub status: String,
    pub version: String,
}

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
///     "status": "healthy",
///     "version": "0.0.9"
/// }
/// ```
async fn health_checker_handler() -> Json<Health> {
    Json(Health {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
