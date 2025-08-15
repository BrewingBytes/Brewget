use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    models::{response::error::Error, token::NewToken},
    schema::tokens::dsl::*,
};

/// Inserts a new token into the database
///
/// # Arguments
/// * `new_token` - The token record to insert
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert(
    new_token: NewToken,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::insert_into(tokens)
        .values(new_token)
        .execute(conn)
        .await
        .map_err(|e| e.into())
}

/// Deletes all tokens associated with a user
///
/// # Arguments
/// * `uuid` - User ID whose tokens should be deleted
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(usize)` - Number of tokens deleted
/// * `Err(Error)` - Database operation error
pub async fn delete_by_uuid(
    uuid: Uuid,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    Ok(diesel::delete(tokens)
        .filter(user_id.eq(uuid))
        .execute(conn)
        .await?)
}
