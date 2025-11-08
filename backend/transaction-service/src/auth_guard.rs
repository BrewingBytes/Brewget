use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{AppState, grpc::auth_service::service::VerifyTokenRequest};
use shared_types::{Error, TranslationKey};

pub async fn auth_guard(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    tracing::debug!("Auth guard: Processing request");

    let received_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or_else(|| {
            tracing::warn!("Auth guard: No Authorization token provided");
            (StatusCode::UNAUTHORIZED, TranslationKey::NotLoggedIn)
        })?;

    tracing::debug!("Auth guard: Token extracted from header");

    let mut client = state.get_auth_service().await;

    tracing::debug!("Auth guard: Using persistent auth service connection, calling verify_token");

    let request = tonic::Request::new(VerifyTokenRequest {
        token: received_token.to_string(),
    });

    let response = client.verify_token(request).await.map_err(|e| {
        tracing::error!("Auth guard: Failed to verify token: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
    })?;

    let response_inner = response.into_inner();

    let user_id = response_inner.user_id.ok_or_else(|| {
        let error_reason = response_inner
            .error_reason
            .as_deref()
            .unwrap_or("TOKEN_INVALID");
        tracing::warn!("Auth guard: Token validation failed - {}", error_reason);

        if error_reason == "TOKEN_EXPIRED" {
            (StatusCode::UNAUTHORIZED, TranslationKey::TokenExpired)
        } else {
            (StatusCode::UNAUTHORIZED, TranslationKey::TokenInvalid)
        }
    })?;

    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| {
        tracing::error!("Auth guard: Invalid user ID format: {}", e);
        (StatusCode::UNAUTHORIZED, TranslationKey::TokenInvalid)
    })?;

    tracing::info!(
        "Auth guard: Token verified successfully for user: {}",
        user_uuid
    );

    req.extensions_mut().insert(user_uuid);
    Ok(next.run(req).await)
}
