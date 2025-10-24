use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::{
        forgot_password_link::{ForgotPasswordLink, NewForgotPasswordLink},
        response::Error,
    },
};

/// Inserts a new forgot password link into the database
///
/// # Arguments
/// * `new_forgot_password_link` - The forgot password link record to insert
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert(
    new_forgot_password_link: NewForgotPasswordLink,
    pool: &PgPool,
) -> Result<usize, Error> {
    sqlx::query(
        r#"
        INSERT INTO forgot_password_links (id, user_id, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(new_forgot_password_link.id)
    .bind(new_forgot_password_link.user_id)
    .bind(new_forgot_password_link.expires_at)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}

/// Search for a forgot password link by id return it
///
/// # Arguments
/// * `find_id` - The id to find
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(User)` - The `ForgotPasswordLink` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_by_id(find_id: Uuid, pool: &PgPool) -> Result<ForgotPasswordLink, Error> {
    sqlx::query_as::<_, ForgotPasswordLink>(
        r#"
        SELECT user_id, expires_at
        FROM forgot_password_links
        WHERE id = $1
        "#,
    )
    .bind(find_id)
    .fetch_one(pool)
    .await
    .map_err(|e: sqlx::Error| -> Error {
        match e {
            sqlx::Error::RowNotFound => {
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
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(usize)` - The amount of lines that have been deleted from database
/// * `Err(Error)` - Database operation error
pub async fn delete(find_id: Uuid, pool: &PgPool) -> Result<usize, Error> {
    sqlx::query(
        r#"
        DELETE FROM forgot_password_links
        WHERE id = $1
        "#,
    )
    .bind(find_id)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not delete from database",
        )
            .into()
    })
}
