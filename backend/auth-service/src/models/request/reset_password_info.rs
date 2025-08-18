use serde::Deserialize;
use uuid::Uuid;

/// Represents credentials required for setting a new password
///
/// This struct is used to deserialize JSON data sent to the `/change-password` endpoint
///
/// # Fields
/// * `id` - The id of the forgot password link
/// * `password` - The new user's password for authentication
///
/// # Example
/// ```json
/// {
///     "id": "abcd-efgh-aaaa",
///     "password": "secretpassword123"
/// }
/// ```
#[derive(Deserialize)]
pub struct ResetPasswordInfo {
    pub id: Uuid,
    pub password: String,
}
