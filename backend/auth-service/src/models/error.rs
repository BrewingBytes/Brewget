use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::models::dto::message::Message;

pub struct Error {
    code: StatusCode,
    body: Json<Message>,
}

impl Error {
    pub fn new(code: StatusCode, message: &str) -> Self {
        Self {
            code,
            body: Json(Message::new(message)),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.code, self.body).into_response()
    }
}

impl From<(StatusCode, &str)> for Error {
    fn from(value: (StatusCode, &str)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, &value.to_string())
    }
}
