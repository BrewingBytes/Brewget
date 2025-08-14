use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Token {
    id: Uuid,
    user_id: Uuid,
    token: String,
    token_type: String,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl Token {
    pub fn get_uuid(&self) -> Uuid {
        self.user_id
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();

        self.expires_at.timestamp() < now
    }
}

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
    pub fn new(user: &User, token: &str, tip: Option<&str>, expiry: Option<DateTime<Utc>>) -> Self {
        Self {
            user_id: user.get_uuid(),
            token: token.into(),
            token_type: tip.unwrap_or_default().into(),
            expires_at: expiry.unwrap_or(Utc::now() + Duration::days(2)),
        }
    }
}
