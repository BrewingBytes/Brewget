use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::{
        response::Error,
        settings::{Settings, UpdateSettings},
    },
};

/// Inserts blank default settings for a new user
///
/// This function creates a new settings record with default values for a user.
/// It's typically called when a new user registers and needs initial settings.
///
/// # Arguments
///
/// * `insert_uuid` - The UUID of the user to create settings for
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert_blank(insert_uuid: Uuid, pool: &PgPool) -> Result<usize, Error> {
    sqlx::query(
        r#"
        INSERT INTO user_settings (user_id)
        VALUES ($1)
        "#,
    )
    .bind(insert_uuid)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}

/// Finds user settings by user UUID, creating default settings if none exist
///
/// This function retrieves user settings from the database. If no settings exist
/// for the user, it automatically creates default settings and returns them.
/// This ensures that every user always has settings available.
///
/// # Arguments
///
/// * `find_uuid` - The UUID of the user to find settings for
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Settings)` - The user's settings (existing or newly created)
/// * `Err(Error)` - Database operation error
///
/// # Behavior
///
/// 1. First attempts to find existing settings for the user
/// 2. If no settings exist, creates default settings using `insert_blank`
/// 3. Returns the settings (either found or newly created)
pub async fn find_by_uuid(find_uuid: Uuid, pool: &PgPool) -> Result<Settings, Error> {
    let mut result = sqlx::query_as::<_, Settings>(
        r#"
        SELECT user_id, language, currency, alarm_set, alarm_time, alarm_offset_minutes, night_mode
        FROM user_settings
        WHERE user_id = $1
        "#,
    )
    .bind(find_uuid)
    .fetch_one(pool)
    .await;

    if result.is_err() {
        insert_blank(find_uuid, pool).await?;
        result = sqlx::query_as::<_, Settings>(
            r#"
            SELECT user_id, language, currency, alarm_set, alarm_time, alarm_offset_minutes, night_mode
            FROM user_settings
            WHERE user_id = $1
            "#,
        )
        .bind(find_uuid)
        .fetch_one(pool)
        .await;
    }

    Ok(result?)
}

/// Updates user settings with new values
///
/// This function performs a partial update of user settings. Only the fields
/// provided in the `UpdateSettings` struct will be updated, leaving other
/// fields unchanged.
///
/// # Arguments
///
/// * `uuid` - The UUID of the user whose settings to update
/// * `update_settings` - The settings update data (only non-None fields will be updated)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(usize)` - Number of rows updated (1 if successful)
/// * `Err(Error)` - Database operation error
///
/// # Example
///
/// ```rust
/// use settings_service::models::settings::UpdateSettings;
/// use chrono::NaiveTime;
///
/// let update = UpdateSettings {
///     language: Some("es".to_string()),
///     night_mode: Some(true),
///     alarm_time: Some(NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
///     ..Default::default()
/// };
///
/// // Only language, night_mode, and alarm_time will be updated
/// update(uuid, update, pool).await?;
/// ```
pub async fn update(
    uuid: Uuid,
    update_settings: UpdateSettings,
    pool: &PgPool,
) -> Result<usize, Error> {
    // Build a dynamic query based on which fields are Some
    let result = sqlx::query(
        r#"
        UPDATE user_settings
        SET 
            language = COALESCE($1, language),
            currency = COALESCE($2, currency),
            alarm_set = COALESCE($3, alarm_set),
            alarm_time = COALESCE($4, alarm_time),
            alarm_offset_minutes = COALESCE($5, alarm_offset_minutes),
            night_mode = COALESCE($6, night_mode)
        WHERE user_id = $7
        "#,
    )
    .bind(update_settings.language)
    .bind(update_settings.currency)
    .bind(update_settings.alarm_set)
    .bind(update_settings.alarm_time)
    .bind(update_settings.alarm_offset_minutes)
    .bind(update_settings.night_mode)
    .bind(uuid)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() as usize)
}
