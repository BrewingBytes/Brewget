use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Enum representing the authentication method used
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "auth_method_enum", rename_all = "lowercase")]
pub enum AuthMethod {
    Password,
    Passkey,
    Otp,
}

/// Represents an authentication audit log entry in the database
///
/// This struct tracks authentication attempts for security and compliance purposes
///
/// # Fields
/// * `id` - Unique identifier for the audit log entry
/// * `user_id` - Foreign key to the users table
/// * `auth_method` - The authentication method used (password, passkey, otp)
/// * `success` - Whether the authentication attempt was successful
/// * `ip_address` - The IP address from which the authentication was attempted
/// * `user_agent` - The user agent string from the authentication request
/// * `attempted_at` - Timestamp when the authentication was attempted
/// * `metadata` - Additional metadata stored as JSON (optional)
#[derive(FromRow, Clone, Debug)]
#[allow(dead_code)]
pub struct AuthenticationAuditLog {
    id: Uuid,
    user_id: Uuid,
    auth_method: AuthMethod,
    success: bool,
    ip_address: Option<String>,
    user_agent: Option<String>,
    attempted_at: DateTime<Utc>,
    metadata: Option<serde_json::Value>,
}
