use axum::http::HeaderMap;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{database, models::authentication_audit_log::AuthMethod};

/// Extracts request metadata from HTTP headers
///
/// # Arguments
/// * `headers` - HTTP headers from the request
///
/// # Returns
/// A tuple containing (ip_address, user_agent) as optional strings
pub fn extract_request_metadata(headers: &HeaderMap) -> (Option<String>, Option<String>) {
    let ip_address = headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    (ip_address, user_agent)
}

/// Logs an authentication attempt to the audit log
///
/// This is a fire-and-forget operation that won't block authentication.
///
/// # Arguments
/// * `user_id` - UUID of the user attempting authentication
/// * `auth_method` - Authentication method used
/// * `success` - Whether the authentication was successful
/// * `ip_address` - Optional IP address of the request
/// * `user_agent` - Optional user agent string
/// * `reason` - Optional reason for failure (e.g., "invalid_password", "account_not_verified")
/// * `pool` - Database connection pool
pub async fn log_authentication_attempt(
    user_id: Uuid,
    auth_method: AuthMethod,
    success: bool,
    ip_address: Option<String>,
    user_agent: Option<String>,
    reason: Option<&str>,
    pool: &PgPool,
) {
    let metadata = reason.map(|r| serde_json::json!({"reason": r}));

    let _ = database::authentication_audit_logs::insert(
        user_id,
        auth_method,
        success,
        ip_address,
        user_agent,
        metadata,
        pool,
    )
    .await;
}
