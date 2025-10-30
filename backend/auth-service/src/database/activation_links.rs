use axum::http::StatusCode;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

use crate::models::{
    activation_link::{ActivationLink, NewActivationLink},
    response::{Error, TranslationKey},
};

/// Inserts a new activation link into the database
///
/// # Arguments
/// * `new_activation_link` - The activation link record to insert
/// * `executor` - Database connection pool or transaction
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert<'a, E>(
    new_activation_link: NewActivationLink,
    executor: E,
) -> Result<usize, Error>
where
    E: sqlx::Executor<'a, Database = Postgres>,
{
    sqlx::query(
        r#"
        INSERT INTO activation_links (id, user_id)
        VALUES ($1, $2)
        "#,
    )
    .bind(new_activation_link.id)
    .bind(new_activation_link.user_id)
    .execute(executor)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}

/// Search for an activation link by id return it and delete from db
///
/// # Arguments
/// * `find_id` - The id to find
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(User)` - The `ActivationLink` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_and_delete_by_id(
    find_id: Uuid,
    pool: &PgPool,
) -> Result<ActivationLink, Error> {
    let link = sqlx::query_as::<_, ActivationLink>(
        r#"
        SELECT user_id
        FROM activation_links
        WHERE id = $1
        "#,
    )
    .bind(find_id)
    .fetch_one(pool)
    .await
    .map_err(|e: sqlx::Error| -> Error {
        match e {
            sqlx::Error::RowNotFound => (
                StatusCode::BAD_REQUEST,
                TranslationKey::ActivationLinkNotFound,
            )
                .into(),
            _ => e.into(),
        }
    })?;

    sqlx::query(
        r#"
        DELETE FROM activation_links
        WHERE id = $1
        "#,
    )
    .bind(find_id)
    .execute(pool)
    .await?;

    Ok(link)
}
