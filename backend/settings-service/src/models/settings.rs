use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents user settings stored in the database
///
/// This struct maps to the `user_settings` table and contains all user-specific
/// configuration and preferences for the Brewget application.
///
/// # Fields
///
/// * `user_id` - Unique identifier of the user these settings belong to
/// * `language` - User's preferred language (e.g., "en", "es", "fr")
/// * `currency` - User's preferred currency code (e.g., "USD", "EUR", "GBP")
/// * `alarm_set` - Whether the user has enabled alarm notifications
/// * `alarm_time` - The time when the alarm should trigger
/// * `alarm_offset_minutes` - Additional offset in minutes for the alarm
/// * `night_mode` - Whether the user has enabled dark/night mode
#[derive(FromRow, Clone, Serialize)]
pub struct Settings {
    user_id: Uuid,
    language: String,
    currency: String,
    alarm_set: bool,
    alarm_time: NaiveTime,
    alarm_offset_minutes: i32,
    night_mode: bool,
}

/// Represents updates to user settings
///
/// This struct is used for partial updates to user settings. All fields are optional,
/// allowing for selective updates without affecting unchanged fields.
///
/// # Fields
///
/// * `language` - Optional new language preference
/// * `currency` - Optional new currency preference
/// * `alarm_set` - Optional alarm enabled status
/// * `alarm_time` - Optional new alarm time
/// * `alarm_offset_minutes` - Optional new alarm offset
/// * `night_mode` - Optional night mode status
#[derive(Deserialize)]
pub struct UpdateSettings {
    pub language: Option<String>,
    pub currency: Option<String>,
    pub alarm_set: Option<bool>,
    pub alarm_time: Option<NaiveTime>,
    pub alarm_offset_minutes: Option<i32>,
    pub night_mode: Option<bool>,
}
