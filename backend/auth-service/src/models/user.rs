use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::utils::password::{hash_password, verify_password};

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
    pub fn get_uuid(&self) -> Uuid {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn is_password_valid(&self, password: &str) -> bool {
        verify_password(password, &self.password).is_ok()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    username: String,
    password: String,
    email: String,
}

impl NewUser {
    pub fn new(username: String, password: String, email: String, salt_str: &str) -> Self {
        let hash = hash_password(&password, salt_str);

        Self {
            username,
            password: hash,
            email,
        }
    }
}
