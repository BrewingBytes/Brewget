use axum::http::StatusCode;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

use crate::models::{
    response::{Error, TranslationKey},
    user::{NewUser, User},
};

/// Inserts a new user into the database
///
/// # Arguments
/// * `new_user` - The user record to insert
/// * `executor` - Database connection pool or transaction
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert<'a, E>(new_user: NewUser, executor: E) -> Result<usize, Error>
where
    E: sqlx::Executor<'a, Database = Postgres>,
{
    sqlx::query(
        r#"
        INSERT INTO users (id, username, password, email)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(new_user.id)
    .bind(new_user.username)
    .bind(new_user.password)
    .bind(new_user.email)
    .execute(executor)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}

/// Search for a user by username
///
/// # Arguments
/// * `find_username` - The username to find
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(User)` - The `User` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_by_username(find_username: &str, pool: &PgPool) -> Result<User, Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password, email, is_verified, is_active
        FROM users
        WHERE username = $1
        "#,
    )
    .bind(find_username)
    .fetch_one(pool)
    .await
    .map_err(|e: sqlx::Error| -> Error {
        match e {
            sqlx::Error::RowNotFound => {
                (StatusCode::BAD_REQUEST, TranslationKey::UsernameNotFound).into()
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
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(User)` - The `User` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_by_username_or_email(
    find_username: &str,
    find_email: &str,
    pool: &PgPool,
) -> Result<User, Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password, email, is_verified, is_active
        FROM users
        WHERE username = $1 OR email = $2
        "#,
    )
    .bind(find_username)
    .bind(find_email)
    .fetch_one(pool)
    .await
    .map_err(|e: sqlx::Error| -> Error {
        match e {
            sqlx::Error::RowNotFound => (
                StatusCode::BAD_REQUEST,
                TranslationKey::UsernameOrEmailNotFound,
            )
                .into(),
            _ => e.into(),
        }
    })
}

/// Search for a user by email
///
/// # Arguments
/// * `find_email` - The email to find
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(User)` - The `User` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_by_email(find_email: &str, pool: &PgPool) -> Result<User, Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password, email, is_verified, is_active
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(find_email)
    .fetch_one(pool)
    .await
    .map_err(|e: sqlx::Error| -> Error {
        match e {
            sqlx::Error::RowNotFound => (
                StatusCode::BAD_REQUEST,
                TranslationKey::UsernameOrEmailNotFound,
            )
                .into(),
            _ => e.into(),
        }
    })
}

/// Set the email for a user as verified
///
/// # Arguments
/// * `find_uuid` - The user account to find
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(usize)` - The amount of users set as verified, 1 means successfull
/// * `Err(Error)` - Database operation error
pub async fn set_verified(find_uuid: Uuid, pool: &PgPool) -> Result<usize, Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET is_verified = true
        WHERE id = $1
        "#,
    )
    .bind(find_uuid)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::CouldNotVerifyAccount,
        )
            .into()
    })
}

/// Set a new password for a user
///
/// # Arguments
/// * `find_uuid` - The user account to update
/// * `new_hashed_password` - The new hashed password for the user account
/// * `executor` - Database connection pool or transaction
///
/// # Returns
/// * `Ok(usize)` - The amount of users set as verified, 1 means successfull
/// * `Err(Error)` - Database operation error
pub async fn change_password<'a, E>(
    find_uuid: Uuid,
    new_hashed_password: String,
    executor: E,
) -> Result<usize, Error>
where
    E: sqlx::Executor<'a, Database = Postgres>,
{
    sqlx::query(
        r#"
        UPDATE users
        SET password = $1
        WHERE id = $2
        "#,
    )
    .bind(new_hashed_password)
    .bind(find_uuid)
    .execute(executor)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::CouldNotUpdatePassword,
        )
            .into()
    })
}
