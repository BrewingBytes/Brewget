use axum::{Extension, Json, response::IntoResponse};

use crate::models::{dto::message::Message, user::User};

pub async fn logout_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    println!("User {} has been logged out.", user.email);

    // Remove the token from the DB

    Json(Message::new("Ok"))
}
