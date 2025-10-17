use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    grpc::auth_service::service::{VerifyTokenRequest, auth_service_client::AuthServiceClient},
    models::response::Error,
};

/// Authentication middleware guard for protected routes
///
/// Validates JWT tokens by calling the auth service gRPC endpoint
///
/// # Flow
/// 1. Extracts Bearer token from Authorization header
/// 2. Calls auth service via gRPC to verify token
/// 3. Auth service returns Option<Uuid> with user ID if valid
/// 4. Adds user ID to request extensions if token is valid
///
/// # Arguments
/// * `state` - Application state containing config
/// * `req` - The incoming HTTP request
/// * `next` - Next middleware in chain
///
/// # Returns
/// * `Ok(Response)` - If authentication succeeds
/// * `Err(Error)` - If any validation step fails
///
/// # Errors
/// * Returns 401 Unauthorized if:
///   - No token provided
///   - Token is invalid/expired
///   - Auth service returns None
pub async fn auth_guard(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    // Extract Bearer token from Authorization header
    let received_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "You are not logged in, please provide token",
        ))?;

    // Connect to auth service via gRPC
    let auth_service_url = format!(
        "http://{}:{}",
        state.config.auth_hostname, state.config.auth_grpc_port
    );
    
    let mut client = AuthServiceClient::connect(auth_service_url)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to auth service"))?;

    // Call verify_token on auth service
    let request = tonic::Request::new(VerifyTokenRequest {
        token: received_token.to_string(),
    });

    let response = client
        .verify_token(request)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to verify token"))?;

    // Check if token is valid (auth service returns Some(user_id) if valid)
    let user_id = response
        .into_inner()
        .user_id
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

    // Parse user_id as UUID
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID format"))?;

    // Add user UUID to request extensions and continue
    req.extensions_mut().insert(user_uuid);
    Ok(next.run(req).await)
}
