use axum::http::StatusCode;
use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    models::{
        response::error::Error,
        settings::{NewSettings, Settings},
    },
    schema::user_settings::dsl::*,
};

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
