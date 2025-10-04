use axum::http::StatusCode;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    models::{
        response::Error,
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
    users::table
        .filter(username.eq(find_username))
        .select(User::as_select())
        .first(conn)
        .await
        .map_err(|e: diesel::result::Error| -> Error {
            match e {
                diesel::result::Error::NotFound => {
                    (StatusCode::BAD_REQUEST, "Username not found.").into()
                }
                _ => e.into(),
            }
        })
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
    users::table
        .filter(username.eq(find_username).or(email.eq(find_email)))
        .select(User::as_select())
        .first(conn)
        .await
        .map_err(|e: diesel::result::Error| -> Error {
            match e {
                diesel::result::Error::NotFound => {
                    (StatusCode::BAD_REQUEST, "Username or email not found.").into()
                }
                _ => e.into(),
            }
        })
}

/// Search for a user by email
///
/// # Arguments
/// * `find_email` - The email to find
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(User)` - The `User` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_by_email(
    find_email: &str,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<User, Error> {
    users::table
        .filter(email.eq(find_email))
        .select(User::as_select())
        .first(conn)
        .await
        .map_err(|e: diesel::result::Error| -> Error {
            match e {
                diesel::result::Error::NotFound => {
                    (StatusCode::BAD_REQUEST, "Username or email not found.").into()
                }
                _ => e.into(),
            }
        })
}

/// Set the email for a user as verified
///
/// # Arguments
/// * `find_uuid` - The user account to find
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(usize)` - The amount of users set as verified, 1 means successfull
/// * `Err(Error)` - Database operation error
pub async fn set_verified(
    find_uuid: Uuid,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::update(users)
        .filter(id.eq(find_uuid))
        .set(is_verified.eq(true))
        .execute(conn)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not verify account.",
            )
                .into()
        })
}

/// Set a new password for a user
///
/// # Arguments
/// * `find_uuid` - The user account to update
/// * `new_hashed_password` - The new hashed password for the user account
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(usize)` - The amount of users set as verified, 1 means successfull
/// * `Err(Error)` - Database operation error
pub async fn change_password(
    find_uuid: Uuid,
    new_hashed_password: String,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::update(users)
        .filter(id.eq(find_uuid))
        .set(password.eq(new_hashed_password))
        .execute(conn)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not update password",
            )
                .into()
        })
}
