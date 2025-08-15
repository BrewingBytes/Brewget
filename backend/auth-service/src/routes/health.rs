use axum::{Json, response::IntoResponse};

use crate::models::response::message::Message;

/// Health check endpoint handler
///
/// Returns a simple message indicating the service is operational
///
/// # Returns
/// JSON response with message "Auth-Service is working."
///
/// # Example Response
/// ```json
/// {
///     "message": "Auth-Service is working."
/// }
/// ```
pub async fn health_checker_handler() -> impl IntoResponse {
    Json(Message {
        message: "Auth-Service is working.".into(),
    })
}
