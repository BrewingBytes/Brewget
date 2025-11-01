use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response::{TranslationKey, TranslationKeyMessage};

/// Custom error type for handling API errors across all services
///
/// Combines an HTTP status code with a JSON message response using translation keys
///
/// # Fields
/// * `code` - HTTP status code for the error response
/// * `body` - JSON message containing error translation key
#[derive(Debug)]
pub struct Error {
    code: StatusCode,
    body: Json<TranslationKeyMessage>,
}

impl Error {
    /// Creates a new Error instance with the specified status code and translation key
    ///
    /// # Arguments
    /// * `code` - The HTTP status code to return
    /// * `translation_key` - The translation key for the error message
    ///
    /// # Returns
    /// Returns a new `Error` instance
    pub fn new(code: StatusCode, translation_key: TranslationKey) -> Self {
        Self {
            code,
            body: Json(TranslationKeyMessage { translation_key }),
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

/// Implements conversion from a tuple of (StatusCode, TranslationKey)
///
/// Provides a convenient way to create errors from status codes and translation keys
impl From<(StatusCode, TranslationKey)> for Error {
    fn from(value: (StatusCode, TranslationKey)) -> Self {
        Self::new(value.0, value.1)
    }
}

/// Converts JWT errors into the application Error type
impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        tracing::error!("JWT error: {}", value);
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
    }
}

/// Converts SQLX database errors into the application Error type
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", value);
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
    }
}

/// Converts Uuid errors into the application Error type
impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        tracing::error!("UUID error: {}", value);
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
    }
}

/// Converts tonic gRPC errors into the application Error type
impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        tracing::error!("gRPC error: {}", value);
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
    }
}

/// Converts general std::error::Error into the application Error type
impl From<Box<dyn std::error::Error>> for Error {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        tracing::error!("Generic error: {}", value);
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
    }
}
