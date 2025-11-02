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

/// Update the counter value for a passkey credential after successful authentication
///
/// This function enforces monotonic counter increases to prevent replay attacks.
/// If the new counter is not greater than the stored counter, the update will fail.
///
/// # Arguments
/// * `credential_id` - The credential ID to update
/// * `new_counter` - The new counter value (must be greater than current counter)
/// * `pool` - Database connection pool
///
/// # Returns
/// * `Ok(())` - Counter updated successfully
/// * `Err(Error)` - Database error or counter validation failure
pub async fn update_counter(
    credential_id: &[u8],
    new_counter: i64,
    pool: &PgPool,
) -> Result<(), Error> {
    let result: Result<_, sqlx::Error> = sqlx::query(
        r#"
        UPDATE passkey_credentials
        SET counter = $2, last_used_at = NOW()
        WHERE credential_id = $1 AND (counter < $2 OR ($2 = 0 AND counter = 0))
        "#,
    )
    .bind(credential_id)
    .bind(new_counter)
    .execute(pool)
    .await;

    match result {
        Ok(query_result) => {
            if query_result.rows_affected() == 0 {
                tracing::error!(
                    "Failed to update passkey counter: counter validation failed or credential not found"
                );
                return Err(Error::from((
                    StatusCode::UNAUTHORIZED,
                    TranslationKey::PasskeyAuthenticationFailed,
                )));
            }
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to update passkey counter: {}", e);
            Err(Error::from((
                StatusCode::INTERNAL_SERVER_ERROR,
                TranslationKey::SomethingWentWrong,
            )))
        }
    }
}
