use chrono::NaiveTime;
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Clone)]
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
