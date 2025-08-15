use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::utils::password::{hash_password, verify_password};

/// Represents a user in the database
///
/// This struct maps to the users table and contains user account information
///
/// # Fields
/// * `id` - Unique identifier for the user
/// * `username` - User's chosen username
/// * `password` - Hashed password string
/// * `email` - User's email address
/// * `is_verified` - Email verification status
/// * `role` - User's role/permissions level
/// * `is_active` - Account active status
/// * `created_at` - Account creation timestamp
/// * `updated_at` - Last update timestamp
/// * `last_login_at` - Most recent login timestamp
#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct User {
    id: Uuid,
    username: String,
    password: String,
    email: String,
    is_verified: bool,
    role: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_login_at: Option<DateTime<Utc>>,
}

impl User {
    /// Returns the user's unique identifier
    pub fn get_uuid(&self) -> Uuid {
        self.id
    }

    /// Returns the user's username
    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    /// Returns the user's email address
    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    /// Validates if the provided password matches the stored hash
    ///
    /// # Arguments
    /// * `password` - The plain text password to verify
    ///
    /// # Returns
    /// * `true` if the password matches
    /// * `false` if the password is invalid
    pub fn is_password_valid(&self, password: &str) -> bool {
        verify_password(password, &self.password).is_ok()
    }
}

/// Represents a new user to be inserted into the database
///
/// This struct is used for creating new user accounts
///
/// # Fields
/// * `username` - Chosen username for the new account
/// * `password` - Password that will be hashed before storage
/// * `email` - Email address for the account
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    username: String,
    password: String,
    email: String,
}

impl NewUser {
    /// Creates a new user account
    ///
    /// # Arguments
    /// * `username` - Chosen username
    /// * `password` - Plain text password that will be hashed
    /// * `email` - Email address
    /// * `salt_str` - Salt string used for password hashing
    ///
    /// # Returns
    /// * `Ok(NewUser)` - A new `NewUser` instance ready for database insertion
    /// * `Err(())` - If the `NewUser` could not be created
    pub fn new(username: &str, password: &str, email: &str) -> Result<Self, ()> {
        let hash = hash_password(&password)?;

        Ok(Self {
            username: username.to_string(),
            password: hash,
            email: email.to_string(),
        })
    }
}
