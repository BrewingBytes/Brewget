use chrono::NaiveTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Clone, Serialize)]
#[diesel(table_name = crate::schema::user_settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Settings {
    user_id: Uuid,
    language: String,
    currency: String,
    alarm_set: bool,
    alarm_time: NaiveTime,
    alarm_offset_minutes: i32,
    night_mode: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSettings {
    user_id: Uuid,
}

impl NewSettings {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::user_settings)]
pub struct UpdateSettings {
    pub language: Option<String>,
    pub currency: Option<String>,
    pub alarm_set: Option<bool>,
    pub alarm_time: Option<NaiveTime>,
    pub alarm_offset_minutes: Option<i32>,
    pub night_mode: Option<bool>,
}
