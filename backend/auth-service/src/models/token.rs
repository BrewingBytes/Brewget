use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::user::User;

/// Represents a token stored in the database
///
/// This struct maps to the tokens table and contains authentication token information
///
/// # Fields
/// * `user_id` - ID of the user this token belongs to
/// * `token` - The actual token string
/// * `expires_at` - Timestamp when the token expires
#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    user_id: Uuid,
    token: String,
    expires_at: DateTime<Utc>,
}

impl Token {
    /// Returns the UUID of the user associated with this token
    pub fn get_uuid(&self) -> Uuid {
        self.user_id
    }

    /// Returns the JWT
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Checks if the token has expired
    ///
    /// # Returns
    /// * `true` if the current time is past the token's expiration time
    /// * `false` if the token is still valid
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();

        self.expires_at.timestamp() < now
    }
}

/// Represents a new token to be inserted into the database
///
/// This struct is used for creating new token records
///
/// # Fields
/// * `user_id` - ID of the user this token belongs to
/// * `token` - The actual token string
/// * `token_type` - Type of token
/// * `expires_at` - When the token expires
#[derive(Insertable)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewToken {
    user_id: Uuid,
    token: String,
    token_type: String,
    expires_at: DateTime<Utc>,
}

impl NewToken {
    /// Creates a new token record
    ///
    /// # Arguments
    /// * `user` - Reference to the user the token belongs to
    /// * `token` - The token string
    /// * `tip` - Optional token type, defaults to empty string
    /// * `expiry` - Optional expiration time, defaults to 2 days from now
    ///
    /// # Returns
    /// A new `NewToken` instance ready for database insertion
    pub fn new(user: &User, token: &str, tip: Option<&str>, expiry: Option<DateTime<Utc>>) -> Self {
        Self {
            user_id: user.get_uuid(),
            token: token.into(),
            token_type: tip.unwrap_or_default().into(),
            expires_at: expiry.unwrap_or(Utc::now() + Duration::days(2)),
        }
    }
}
