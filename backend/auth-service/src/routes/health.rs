use axum::{Json, response::IntoResponse};

use crate::models::dto::message::Message;

pub async fn health_checker_handler() -> impl IntoResponse {
    Json(Message::new("Auth-Service is working."))
}
