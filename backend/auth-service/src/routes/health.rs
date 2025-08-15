use axum::{Json, response::IntoResponse};

use crate::models::response::message::Message;

pub async fn health_checker_handler() -> impl IntoResponse {
    Json(Message {
        message: "Auth-Service is working.".into(),
    })
}
