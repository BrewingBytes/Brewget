use serde::Serialize;

/// A generic message response structure
///
/// This struct is used to serialize response messages into JSON format
///
/// # Fields
/// * `message` - The message content to be sent in the response
///
/// # Example
/// ```json
/// {
///     "message": "Operation completed successfully"
/// }
/// ```
#[derive(Serialize)]
pub struct Message {
    pub message: String,
}
