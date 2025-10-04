use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use deadpool::managed::PoolError;

use crate::response::Message;

/// Custom error type for handling API errors across all services
///
/// Combines an HTTP status code with a JSON message response
///
/// # Fields
/// * `code` - HTTP status code for the error response
/// * `body` - JSON message containing error details
pub struct Error {
    code: StatusCode,
    body: Json<Message>,
}

impl Error {
    /// Creates a new Error instance with the specified status code and message
    ///
    /// # Arguments
    /// * `code` - The HTTP status code to return
    /// * `message` - The error message to include in the response
    ///
    /// # Returns
    /// Returns a new `Error` instance
    pub fn new(code: StatusCode, message: &str) -> Self {
        Self {
            code,
            body: Json(Message {
                message: message.into(),
            }),
        }
    }
}

/// Implements conversion into an Axum Response
///
/// Allows the Error type to be returned directly from route handlers
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.code, self.body).into_response()
    }
}

/// Implements conversion from a tuple of (StatusCode, &str)
///
/// Provides a convenient way to create errors from status codes and messages
impl From<(StatusCode, &str)> for Error {
    fn from(value: (StatusCode, &str)) -> Self {
        Self::new(value.0, value.1)
    }
}

/// Converts JWT errors into the application Error type
impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, &value.to_string())
    }
}

/// Converts database pool errors into the application Error type
impl From<PoolError<diesel_async::pooled_connection::PoolError>> for Error {
    fn from(value: PoolError<diesel_async::pooled_connection::PoolError>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, &value.to_string())
    }
}

/// Converts Diesel database errors into the application Error type
impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, &value.to_string())
    }
}

/// Converts Uuid errors into the application Error type
impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, &value.to_string())
    }
}

/// Converts tonic gRPC errors into the application Error type
impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, &value.message().to_string())
    }
}

/// Converts general std::error::Error into the application Error type
impl From<Box<dyn std::error::Error>> for Error {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, &value.to_string())
    }
}
