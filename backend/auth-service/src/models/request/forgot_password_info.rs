use serde::Deserialize;

/// Represents the info required for sending a forgot password email
///
/// This struct is used to deserialize JSON data sent to the `/forgot-password` endpoint
///
/// # Fields
/// * `email` - The email of the user account
///
/// # Example
/// ```json
/// {
///     "email": "test@test.com",
/// }
/// ```
#[derive(Deserialize)]
pub struct ForgotPasswordInfo {
    pub email: String,
}
