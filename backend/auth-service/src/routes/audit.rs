use std::{str::FromStr, sync::Arc};

use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    middleware,
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState, database,
    models::{authentication_audit_log::AuthenticationAuditLogResponse, response::Error},
    routes::middlewares::auth_guard::auth_guard,
};

/// Query parameters for audit log listing
#[derive(Debug, Deserialize)]
pub struct AuditLogsQuery {
    /// Maximum number of logs to return (default: 50, max: 100)
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 {
    50
}

/// Creates a router for the authentication audit routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_audit_logs))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard))
        .with_state(state)
}

/// List recent authentication audit logs for the authenticated user
///
/// This endpoint retrieves the authentication audit history for the current user,
/// including login attempts, authentication methods used, and success status.
///
/// # Arguments
/// * `state` - Application state containing DB connection
/// * `user_uuid` - Authenticated user's ID from middleware
/// * `query` - Query parameters including optional limit (default: 50, max: 100)
///
/// # Returns
/// * `Ok(Json<Vec<AuthenticationAuditLogResponse>>)` - List of audit log entries
/// * `Err(Error)` - Database error
///
/// # Example Response
/// ```json
/// [
///   {
///     "id": "550e8400-e29b-41d4-a716-446655440000",
///     "auth_method": "password",
///     "success": true,
///     "ip_address": "192.168.1.1",
///     "user_agent": "Mozilla/5.0...",
///     "attempted_at": "2024-01-01T12:00:00Z"
///   }
/// ]
/// ```
async fn list_audit_logs(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
    Query(query): Query<AuditLogsQuery>,
) -> Result<impl IntoResponse, Error> {
    let user_id = Uuid::from_str(&user_uuid)?;

    // Cap the limit to a maximum of 100 to prevent abuse
    let limit = query.limit.clamp(1, 100);

    tracing::info!("Fetching {} audit logs for user: {}", limit, user_id);

    let pool = state.get_database_pool();
    let audit_logs =
        database::authentication_audit_logs::find_by_user_id(user_id, limit, pool).await?;

    let response: Vec<AuthenticationAuditLogResponse> =
        audit_logs.into_iter().map(Into::into).collect();

    Ok(Json(response))
}
