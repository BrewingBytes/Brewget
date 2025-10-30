use axum::{
    Json,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

use crate::i18n::{TranslationKey, Translator};
use crate::response::Message;

/// Extracts language code from Accept-Language header
///
/// Parses the Accept-Language header value and returns the first supported language code.
/// Falls back to "en" if the header is not present or cannot be parsed.
///
/// # Arguments
/// * `headers` - Optional HTTP headers containing Accept-Language
///
/// # Returns
/// Language code string (e.g., "en", "es", "fr", "de", "ro")
pub fn extract_language_from_headers(headers: Option<&HeaderMap>) -> &str {
    headers
        .and_then(|h| h.get("accept-language"))
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            // Extract first language code from Accept-Language header
            // Format: "en-US,en;q=0.9,es;q=0.8"
            s.split(',')
                .next()
                .and_then(|l| l.split(';').next())
                .map(|l| l.trim())
        })
        .unwrap_or("en")
}

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

    /// Creates a new Error instance with translation support
    ///
    /// # Arguments
    /// * `code` - The HTTP status code to return
    /// * `key` - The translation key to use
    /// * `headers` - Optional request headers to extract Accept-Language from
    ///
    /// # Returns
    /// Returns a new `Error` instance with translated message
    pub fn translated(code: StatusCode, key: TranslationKey, headers: Option<&HeaderMap>) -> Self {
        let lang = extract_language_from_headers(headers);
        let translator = Translator::from_code(lang);
        let message = translator.translate(key);

        Self {
            code,
            body: Json(Message { message }),
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

/// Converts SQLX database errors into the application Error type
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
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
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, value.message())
    }
}

/// Converts general std::error::Error into the application Error type
impl From<Box<dyn std::error::Error>> for Error {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, &value.to_string())
    }
}
