use serde::Deserialize;

/// Represents credentials required for user authentication
///
/// This struct is used to deserialize JSON data sent to the `/login` endpoint
///
/// # Fields
/// * `username` - The user's login identifier
/// * `password` - The user's password for authentication
///
/// # Example
/// ```json
/// {
///     "username": "user",
///     "password": "secretpassword123",
///     "captchaToken": "token123"
/// }
/// ```
#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
    #[serde(rename = "captchaToken")]
    pub captcha_token: String,
}
