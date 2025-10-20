use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    models::{
        passkey_credential::{NewPasskeyCredential, PasskeyCredential, UpdatePasskeyCounter},
        response::Error,
    },
    schema::passkey_credentials::{self, dsl::*},
};

/// Inserts a new passkey credential into the database
pub async fn insert(
    new_credential: NewPasskeyCredential,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::insert_into(passkey_credentials::table)
        .values(new_credential)
        .execute(conn)
        .await
        .map_err(|e| e.into())
}

/// Get all passkey credentials for a user
pub async fn filter_by_user_id(
    find_user_id: Uuid,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<Vec<PasskeyCredential>, Error> {
    passkey_credentials::table
        .filter(user_id.eq(find_user_id))
        .select(PasskeyCredential::as_select())
        .load(conn)
        .await
        .map_err(|e| e.into())
}

/// Get a passkey credential by credential_id
pub async fn filter_by_credential_id(
    find_credential_id: &[u8],
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<PasskeyCredential, Error> {
    passkey_credentials::table
        .filter(credential_id.eq(find_credential_id))
        .select(PasskeyCredential::as_select())
        .first(conn)
        .await
        .map_err(|e| e.into())
}

/// Update counter for a passkey credential after successful authentication
pub async fn update_counter(
    credential_uuid: Uuid,
    new_counter: i64,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    let update = UpdatePasskeyCounter {
        counter: new_counter,
        last_used_at: Some(chrono::Utc::now()),
    };

    diesel::update(passkey_credentials.filter(id.eq(credential_uuid)))
        .set(&update)
        .execute(conn)
        .await
        .map_err(|e| e.into())
}
