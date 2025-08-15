use serde::Serialize;

/// A JWT response structure
///
/// This struct is used to serialize response JWT into JSON format
///
/// # Fields
/// * `token` - The JWT to be sent in the response after login
///
/// # Example
/// ```json
/// {
///     "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
/// }
/// ```
#[derive(Serialize)]
pub struct Token {
    pub token: String,
}
