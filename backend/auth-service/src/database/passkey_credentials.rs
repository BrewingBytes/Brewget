use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    passkey_credential::{NewPasskeyCredential, PasskeyCredential},
    response::Error,
};

/// Inserts a new passkey credential into the database
pub async fn insert(new_credential: NewPasskeyCredential, pool: &PgPool) -> Result<usize, Error> {
    sqlx::query(
        r#"
        INSERT INTO passkey_credentials (user_id, credential_id, public_key, counter, transports, backup_eligible, backup_state, attestation_type)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(new_credential.user_id)
    .bind(new_credential.credential_id)
    .bind(new_credential.public_key)
    .bind(new_credential.counter)
    .bind(new_credential.transports)
    .bind(new_credential.backup_eligible)
    .bind(new_credential.backup_state)
    .bind(new_credential.attestation_type)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}

/// Get all passkey credentials for a user
pub async fn filter_by_user_id(
    find_user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<PasskeyCredential>, Error> {
    sqlx::query_as::<_, PasskeyCredential>(
        r#"
        SELECT id, user_id, credential_id, public_key, counter, transports, backup_eligible, backup_state, attestation_type, created_at, last_used_at
        FROM passkey_credentials
        WHERE user_id = $1
        "#,
    )
    .bind(find_user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.into())
}

/// Get a passkey credential by credential_id
pub async fn filter_by_credential_id(
    find_credential_id: &[u8],
    pool: &PgPool,
) -> Result<PasskeyCredential, Error> {
    sqlx::query_as::<_, PasskeyCredential>(
        r#"
        SELECT id, user_id, credential_id, public_key, counter, transports, backup_eligible, backup_state, attestation_type, created_at, last_used_at
        FROM passkey_credentials
        WHERE credential_id = $1
        "#,
    )
    .bind(find_credential_id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.into())
}

/// Update counter for a passkey credential after successful authentication
pub async fn update_counter(
    credential_uuid: Uuid,
    new_counter: i64,
    pool: &PgPool,
) -> Result<usize, Error> {
    sqlx::query(
        r#"
        UPDATE passkey_credentials
        SET counter = $1, last_used_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(new_counter)
    .bind(credential_uuid)
    .execute(pool)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}
