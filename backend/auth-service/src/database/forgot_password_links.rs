use axum::http::StatusCode;
use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    models::{
        forgot_password_link::{ForgotPasswordLink, NewForgotPasswordLink},
        response::Error,
    },
    schema::forgot_password_links::{self, id},
};

/// Inserts a new forgot password link into the database
///
/// # Arguments
/// * `new_forgot_password_link` - The forgot password link record to insert
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert(
    new_forgot_password_link: NewForgotPasswordLink,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::insert_into(forgot_password_links::table)
        .values(new_forgot_password_link)
        .execute(conn)
        .await
        .map_err(|e| e.into())
}

/// Search for a forgot password link by id return it
///
/// # Arguments
/// * `find_id` - The id to find
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(User)` - The `ForgotPasswordLink` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_by_id(
    find_id: Uuid,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<ForgotPasswordLink, Error> {
    forgot_password_links::table
        .filter(id.eq(find_id))
        .select(ForgotPasswordLink::as_select())
        .first(conn)
        .await
        .map_err(|e: diesel::result::Error| -> Error {
            match e {
                diesel::result::Error::NotFound => {
                    (StatusCode::BAD_REQUEST, "Activation link not found.").into()
                }
                _ => e.into(),
            }
        })
}

/// Delete a forgot password link by id
///
/// # Arguments
/// * `find_id` - The id to find and delete
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(usize)` - The amount of lines that have been deleted from database
/// * `Err(Error)` - Database operation error
pub async fn delete(
    find_id: Uuid,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::delete(forgot_password_links::table)
        .filter(id.eq(find_id))
        .execute(conn)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not delete from database",
            )
                .into()
        })
}
