use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a password history entry in the database
///
/// This struct tracks historical passwords for a user to prevent password reuse
///
/// # Fields
/// * `id` - Unique identifier for the history entry
/// * `user_id` - Foreign key to the users table
/// * `password_hash` - Hashed password string
/// * `created_at` - Timestamp when the password was set
#[derive(FromRow, Clone)]
#[allow(dead_code)]
pub struct PasswordHistory {
    id: Uuid,
    user_id: Uuid,
    password_hash: String,
    created_at: DateTime<Utc>,
}

impl PasswordHistory {
    /// Returns the password hash
    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }
}
