use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    models::{
        response::error::Error,
        settings::{NewSettings, Settings, UpdateSettings},
    },
    schema::user_settings::dsl::*,
};

/// Inserts blank default settings for a new user
///
/// This function creates a new settings record with default values for a user.
/// It's typically called when a new user registers and needs initial settings.
///
/// # Arguments
///
/// * `insert_uuid` - The UUID of the user to create settings for
/// * `conn` - Database connection from the pool
///
/// # Returns
///
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert_blank(
    insert_uuid: Uuid,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::insert_into(user_settings)
        .values(NewSettings::new(insert_uuid))
        .execute(conn)
        .await
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
/// * `conn` - Database connection from the pool
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
pub async fn find_by_uuid(
    find_uuid: Uuid,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<Settings, Error> {
    let mut result = user_settings
        .filter(user_id.eq(find_uuid))
        .select(Settings::as_select())
        .first(conn)
        .await;

    if result.is_err() {
        insert_blank(find_uuid, conn).await?;
        result = user_settings
            .filter(user_id.eq(find_uuid))
            .select(Settings::as_select())
            .first(conn)
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
/// * `conn` - Database connection from the pool
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
/// update(uuid, update, &mut conn).await?;
/// ```
pub async fn update(
    uuid: Uuid,
    update_settings: UpdateSettings,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    Ok(diesel::update(user_settings.filter(user_id.eq(uuid)))
        .set(&update_settings)
        .execute(conn)
        .await?)
}
