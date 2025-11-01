use axum::http::StatusCode;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::{
    passkey_credential::{NewPasskeyCredential, PasskeyCredential},
    response::{Error, TranslationKey},
};

/// Insert a new passkey credential into the database
///
/// # Arguments
/// * `credential` - The new passkey credential to insert
/// * `tx` - Database transaction
///
/// # Returns
/// * `Ok(PasskeyCredential)` - The inserted credential with generated fields
/// * `Err(Error)` - Database error
pub async fn insert(
    credential: NewPasskeyCredential,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PasskeyCredential, Error> {
    sqlx::query_as::<_, PasskeyCredential>(
        r#"
        INSERT INTO passkey_credentials 
            (user_id, credential_id, public_key, counter, aaguid, device_name, user_agent)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(credential.user_id)
    .bind(credential.credential_id)
    .bind(credential.public_key)
    .bind(credential.counter)
    .bind(credential.aaguid)
    .bind(credential.device_name)
    .bind(credential.user_agent)
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert passkey credential: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        )
            .into()
    })
}

/// Find all active passkey credentials for a user
///
/// # Arguments
/// * `user_id` - UUID of the user
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(Vec<PasskeyCredential>)` - List of active credentials for the user
/// * `Err(Error)` - Database error
pub async fn find_by_user_id(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<PasskeyCredential>, Error> {
    sqlx::query_as::<_, PasskeyCredential>(
        r#"
        SELECT * FROM passkey_credentials
        WHERE user_id = $1 AND is_active = TRUE
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch passkey credentials: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        )
            .into()
    })
}

/// Find a passkey credential by its credential ID
///
/// # Arguments
/// * `credential_id` - The credential ID to search for
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(PasskeyCredential)` - The found credential
/// * `Err(Error)` - Credential not found or database error
pub async fn find_by_credential_id(
    credential_id: &[u8],
    pool: &PgPool,
) -> Result<PasskeyCredential, Error> {
    sqlx::query_as::<_, PasskeyCredential>(
        r#"
        SELECT * FROM passkey_credentials
        WHERE credential_id = $1 AND is_active = TRUE
        "#,
    )
    .bind(credential_id)
    .fetch_one(pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, TranslationKey::PasskeyNotFound).into())
}

/// Update the counter value for a passkey credential after successful authentication
///
/// # Arguments
/// * `credential_id` - The credential ID to update
/// * `new_counter` - The new counter value
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(())` - Counter updated successfully
/// * `Err(Error)` - Database error
pub async fn update_counter(
    credential_id: &[u8],
    new_counter: i64,
    pool: &PgPool,
) -> Result<(), Error> {
    let result: Result<_, sqlx::Error> = sqlx::query(
        r#"
        UPDATE passkey_credentials
        SET counter = $2, last_used_at = NOW()
        WHERE credential_id = $1
        "#,
    )
    .bind(credential_id)
    .bind(new_counter)
    .execute(pool)
    .await;

    result.map_err(|e: sqlx::Error| {
        tracing::error!("Failed to update passkey counter: {}", e);
        Error::from((
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        ))
    })?;

    Ok(())
}

/// Deactivate a passkey credential (soft delete)
///
/// # Arguments
/// * `credential_id` - UUID of the credential record to deactivate
/// * `user_id` - UUID of the user who owns the credential (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(())` - Credential deactivated successfully
/// * `Err(Error)` - Database error
pub async fn delete(credential_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<(), Error> {
    let result: Result<_, sqlx::Error> = sqlx::query(
        r#"
        UPDATE passkey_credentials
        SET is_active = FALSE
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(credential_id)
    .bind(user_id)
    .execute(pool)
    .await;

    result.map_err(|e: sqlx::Error| {
        tracing::error!("Failed to delete passkey credential: {}", e);
        Error::from((
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        ))
    })?;

    Ok(())
}
