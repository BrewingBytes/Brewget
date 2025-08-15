use diesel::{
    BoolExpressionMethods, ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;

use crate::{
    models::{
        response::error::Error,
        user::{NewUser, User},
    },
    schema::users::{self, dsl::*},
};

/// Inserts a new user into the database
///
/// # Arguments
/// * `new_user` - The user record to insert
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert(
    new_user: NewUser,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
        .await
        .map_err(|e| e.into())
}

/// Search for a user by username
///
/// # Arguments
/// * `find_username` - The username to find
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(User)` - The `User` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_by_username(
    find_username: &str,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<User, Error> {
    Ok(users::table
        .filter(username.eq(find_username))
        .select(User::as_select())
        .first(conn)
        .await?)
}

/// Search for a user by username or email
///
/// # Arguments
/// * `find_username` - The username to find
/// * `find_email` - The email to find
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(User)` - The `User` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_by_username_or_email(
    find_username: &str,
    find_email: &str,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<User, Error> {
    Ok(users::table
        .filter(username.eq(find_username).or(email.eq(find_email)))
        .select(User::as_select())
        .first(conn)
        .await?)
}
