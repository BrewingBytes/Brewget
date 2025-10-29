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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_settings_serialization() {
        let settings = Settings {
            user_id: Uuid::new_v4(),
            language: "en".to_string(),
            currency: "USD".to_string(),
            alarm_set: true,
            alarm_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            alarm_offset_minutes: 15,
            night_mode: false,
        };
        
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"en\""));
        assert!(json.contains("\"USD\""));
        assert!(json.contains("15"));
    }

    #[test]
    fn test_update_settings_deserialization_all_fields() {
        let json = r#"{
            "language": "es",
            "currency": "EUR",
            "alarm_set": true,
            "alarm_time": "09:30:00",
            "alarm_offset_minutes": 30,
            "night_mode": true
        }"#;
        
        let update: UpdateSettings = serde_json::from_str(json).unwrap();
        assert_eq!(update.language, Some("es".to_string()));
        assert_eq!(update.currency, Some("EUR".to_string()));
        assert_eq!(update.alarm_set, Some(true));
        assert_eq!(update.alarm_offset_minutes, Some(30));
        assert_eq!(update.night_mode, Some(true));
    }

    #[test]
    fn test_update_settings_deserialization_partial() {
        let json = r#"{
            "language": "fr",
            "night_mode": false
        }"#;
        
        let update: UpdateSettings = serde_json::from_str(json).unwrap();
        assert_eq!(update.language, Some("fr".to_string()));
        assert_eq!(update.night_mode, Some(false));
        assert_eq!(update.currency, None);
        assert_eq!(update.alarm_set, None);
    }

    #[test]
    fn test_update_settings_deserialization_empty() {
        let json = r#"{}"#;
        
        let update: UpdateSettings = serde_json::from_str(json).unwrap();
        assert_eq!(update.language, None);
        assert_eq!(update.currency, None);
        assert_eq!(update.alarm_set, None);
        assert_eq!(update.alarm_time, None);
        assert_eq!(update.alarm_offset_minutes, None);
        assert_eq!(update.night_mode, None);
    }

    #[test]
    fn test_settings_with_different_languages() {
        let languages = vec!["en", "es", "fr", "de", "it", "pt"];
        
        for lang in languages {
            let settings = Settings {
                user_id: Uuid::new_v4(),
                language: lang.to_string(),
                currency: "USD".to_string(),
                alarm_set: false,
                alarm_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                alarm_offset_minutes: 0,
                night_mode: false,
            };
            
            let json = serde_json::to_string(&settings).unwrap();
            assert!(json.contains(&format!("\"{}\"", lang)));
        }
    }

    #[test]
    fn test_settings_with_different_currencies() {
        let currencies = vec!["USD", "EUR", "GBP", "JPY", "CAD"];
        
        for currency in currencies {
            let settings = Settings {
                user_id: Uuid::new_v4(),
                language: "en".to_string(),
                currency: currency.to_string(),
                alarm_set: false,
                alarm_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                alarm_offset_minutes: 0,
                night_mode: false,
            };
            
            let json = serde_json::to_string(&settings).unwrap();
            assert!(json.contains(&format!("\"{}\"", currency)));
        }
    }

    #[test]
    fn test_settings_alarm_offset_range() {
        let offsets = vec![-60, -30, 0, 15, 30, 60];
        
        for offset in offsets {
            let settings = Settings {
                user_id: Uuid::new_v4(),
                language: "en".to_string(),
                currency: "USD".to_string(),
                alarm_set: true,
                alarm_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                alarm_offset_minutes: offset,
                night_mode: false,
            };
            
            let json = serde_json::to_string(&settings).unwrap();
            assert!(json.contains(&offset.to_string()));
        }
    }
}
