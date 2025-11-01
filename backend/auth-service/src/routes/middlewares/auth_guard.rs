use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    AppState, database,
    models::{response::Error, response::TranslationKey, token_claim::TokenClaim},
};

/// Authentication middleware guard for protected routes
///
/// Validates JWT tokens and ensures they exist in the database
///
/// # Flow
/// 1. Extracts Bearer token from Authorization header
/// 2. Decodes and validates the JWT
/// 3. Checks if token exists in database and is not expired
/// 4. Verifies token belongs to correct user
/// 5. Adds user ID to request extensions
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
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
///   - Token not found in database
///   - Token user mismatch
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
        .ok_or((StatusCode::UNAUTHORIZED, TranslationKey::NotLoggedIn))?;

    // Decode and validate JWT token
    let decoded_token = decode::<TokenClaim>(
        received_token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, TranslationKey::TokenInvalid))?;

    // Check if token exists in database
    let pool = state.get_database_pool();
    let token_res = database::tokens::find(received_token, pool)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, TranslationKey::TokenInvalid))?;

    // Verify token is not expired
    if token_res.is_expired() {
        database::tokens::delete_by_token(token_res.get_token(), pool).await?;
        return Err((StatusCode::UNAUTHORIZED, TranslationKey::TokenExpired).into());
    }

    // Verify token belongs to correct user
    if token_res.get_uuid().to_string() != *decoded_token.claims.sub {
        return Err((StatusCode::UNAUTHORIZED, TranslationKey::TokenInvalid).into());
    }

    // Add user ID to request extensions and continue
    req.extensions_mut()
        .insert(decoded_token.claims.sub.to_string());
    Ok(next.run(req).await)
}
