use sqlx::Postgres;
use uuid::Uuid;

use crate::models::{authentication_audit_log::AuthMethod, response::Error};

/// Inserts an authentication audit log entry into the database
///
/// # Arguments
/// * `user_id` - The UUID of the user attempting authentication
/// * `auth_method` - The authentication method used (password, passkey, otp)
/// * `success` - Whether the authentication attempt was successful
/// * `ip_address` - Optional IP address from which the authentication was attempted
/// * `user_agent` - Optional user agent string from the authentication request
/// * `metadata` - Optional additional metadata as JSON
/// * `executor` - Database connection pool or transaction
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert<'a, E>(
    user_id: Uuid,
    auth_method: AuthMethod,
    success: bool,
    ip_address: Option<String>,
    user_agent: Option<String>,
    metadata: Option<serde_json::Value>,
    executor: E,
) -> Result<usize, Error>
where
    E: sqlx::Executor<'a, Database = Postgres>,
{
    sqlx::query(
        r#"
        INSERT INTO authentication_audit_log 
            (user_id, auth_method, success, ip_address, user_agent, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(user_id)
    .bind(auth_method)
    .bind(success)
    .bind(ip_address)
    .bind(user_agent)
    .bind(metadata)
    .execute(executor)
    .await
    .map(|result| result.rows_affected() as usize)
    .map_err(|e| e.into())
}
