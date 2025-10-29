use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{password_history::PasswordHistory, response::Error};

/// Inserts a password into the password history
///
/// # Arguments
/// * `user_id` - The UUID of the user
/// * `password_hash` - The hashed password to store in history
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert(user_id: Uuid, password_hash: String, pool: &PgPool) -> Result<usize, Error> {
    sqlx::query(
        r#"
        INSERT INTO password_history (user_id, password_hash)
        VALUES ($1, $2)
        "#,
    )
    .bind(user_id)
    .bind(password_hash)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}

/// Deletes old password history entries beyond the specified limit for a user
///
/// This function keeps only the N most recent passwords and removes older entries
/// to prevent unbounded table growth.
///
/// # Arguments
/// * `user_id` - The UUID of the user
/// * `keep_limit` - Number of recent passwords to keep
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(usize)` - Number of rows deleted
/// * `Err(Error)` - Database operation error
pub async fn cleanup_old_passwords(
    user_id: Uuid,
    keep_limit: i64,
    pool: &PgPool,
) -> Result<usize, Error> {
    sqlx::query(
        r#"
        DELETE FROM password_history
        WHERE user_id = $1
        AND id NOT IN (
            SELECT id FROM password_history
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
        )
        "#,
    )
    .bind(user_id)
    .bind(keep_limit)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}

/// Retrieves the last N password hashes for a user
///
/// # Arguments
/// * `user_id` - The UUID of the user
/// * `limit` - Number of recent passwords to retrieve
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(Vec<PasswordHistory>)` - List of password history entries, most recent first
/// * `Err(Error)` - Database operation error
pub async fn get_recent_passwords(
    user_id: Uuid,
    limit: i64,
    pool: &PgPool,
) -> Result<Vec<PasswordHistory>, Error> {
    sqlx::query_as::<_, PasswordHistory>(
        r#"
        SELECT id, user_id, password_hash, created_at
        FROM password_history
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|_e: sqlx::Error| -> Error {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve password history.",
        )
            .into()
    })
}
