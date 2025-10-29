use serde::Deserialize;

/// Represents credentials required for user registration
///
/// This struct is used to deserialize JSON data sent to the `/register` endpoint
///
/// # Fields
/// * `username` - The user's register identifier
/// * `email`    - The user's email
/// * `password` - The user's password for authentication
///
/// # Example
/// ```json
/// {
///     "username": "user",
///     "email": "user@example.com",
///     "password": "secretpassword123",
///     "captchaToken": "token123"
/// }
/// ```
#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "captchaToken")]
    pub captcha_token: String,
}
