use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    response::{Error, TranslationKey},
    token::{NewToken, Token},
};

/// Inserts a new token into the database
///
/// # Arguments
/// * `new_token` - The token record to insert
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert(new_token: NewToken, pool: &PgPool) -> Result<usize, Error> {
    sqlx::query(
        r#"
        INSERT INTO tokens (user_id, token, token_type, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(new_token.user_id)
    .bind(new_token.token)
    .bind(new_token.token_type)
    .bind(new_token.expires_at)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}

/// Deletes all tokens associated with a user
///
/// # Arguments
/// * `uuid` - User ID whose tokens should be deleted
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(usize)` - Number of tokens deleted
/// * `Err(Error)` - Database operation error
pub async fn delete_by_uuid(uuid: Uuid, pool: &PgPool) -> Result<usize, Error> {
    Ok(sqlx::query(
        r#"
        DELETE FROM tokens
        WHERE user_id = $1
        "#,
    )
    .bind(uuid)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)?)
}

pub async fn delete_by_token(tkn: &str, pool: &PgPool) -> Result<usize, Error> {
    Ok(sqlx::query(
        r#"
        DELETE FROM tokens
        WHERE token = $1
        "#,
    )
    .bind(tkn)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)?)
}

pub async fn find(find_token: &str, pool: &PgPool) -> Result<Token, Error> {
    sqlx::query_as::<_, Token>(
        r#"
        SELECT user_id, token, expires_at
        FROM tokens
        WHERE token = $1
        "#,
    )
    .bind(find_token)
    .fetch_one(pool)
    .await
    .map_err(|e: sqlx::Error| -> Error {
        match e {
            sqlx::Error::RowNotFound => {
                (StatusCode::UNAUTHORIZED, TranslationKey::TokenExpired).into()
            }
            _ => e.into(),
        }
    })
}
