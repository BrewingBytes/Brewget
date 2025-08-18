use chrono::{DateTime, Duration, Utc};
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use uuid::Uuid;

/// Represents an forgot password link stored in the database
///
/// This struct maps to the forgot password links table
///
/// # Fields
/// * `id` - Unique identifier for the forgot password link
/// * `user_id` - ID of the user this forgot password link belongs to
/// * `expires_at` - Timestamp when the link will be invalid
#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::forgot_password_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForgotPasswordLink {
    user_id: Uuid,
    expires_at: DateTime<Utc>,
}

impl ForgotPasswordLink {
    /// Get the User ID of the activation link
    ///
    /// # Returns
    /// * `Uuid` - The User ID associated to the activation link
    pub fn get_uuid(&self) -> Uuid {
        self.user_id
    }

    /// Check if the forgot password link is expired
    ///
    /// # Returns
    /// * `true` - if the link is expired
    /// * `false` - if the link is still active
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Represents a new forgot password link to be inserted into the database
///
/// This struct is used for creating new forgot password link records
///
/// # Fields
/// * `id` - UUIDv4 for the forgot password link
/// * `user_id` - The user account uuid it is generated for
/// * `expires_at` - Timestamp of the moment the forgot password link expires
#[derive(Insertable)]
#[diesel(table_name = crate::schema::forgot_password_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewForgotPasswordLink {
    id: Uuid,
    user_id: Uuid,
    expires_at: DateTime<Utc>,
}

impl NewForgotPasswordLink {
    /// Creates a new forgot password link record
    ///
    /// # Arguments
    /// * `user_id` - The user account uuid it is generated for
    ///
    /// # Returns
    /// A new `NewForgotPasswordLink` instance ready for database insertion
    pub fn new(user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            expires_at: Utc::now() + Duration::hours(1),
        }
    }

    /// Get the ID of the forgot password link
    ///
    /// # Returns
    /// * `Uuid` - The ID associated to the forgot password link
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    // Get the forgot password link
    ///
    /// # Returns
    /// * `String` - The forgot password link
    pub fn get_link(&self) -> String {
        format!(
            "https://brewget.brewingbytes.com/forgot-password/{}",
            self.get_id()
        )
    }
}
