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
        token::{NewToken, Token},
    },
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

pub async fn delete_by_token(
    tkn: &str,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    Ok(diesel::delete(tokens)
        .filter(token.eq(tkn))
        .execute(conn)
        .await?)
}

pub async fn find(
    find_token: &str,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<Token, Error> {
    tokens
        .filter(token.eq(find_token))
        .select(Token::as_select())
        .first(conn)
        .await
        .map_err(|e: diesel::result::Error| -> Error {
            match e {
                diesel::result::Error::NotFound => {
                    (StatusCode::UNAUTHORIZED, "Token has expired.").into()
                }
                _ => e.into(),
            }
        })
}
