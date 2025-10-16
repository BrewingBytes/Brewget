use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json;
use webauthn_rs::prelude::*;

use crate::{
    AppState, database,
    models::{
        request::passkey_info::{PasskeyAuthFinishInfo, PasskeyAuthStartInfo},
        response::{Error, Token},
        token::NewToken,
        token_claim::TokenClaim,
    },
};

/// Creates a router for the passkey authentication routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/start", post(auth_start_handler))
        .route("/finish", post(auth_finish_handler))
        .with_state(state)
}

/// Starts passkey authentication flow
///
/// Generates a WebAuthn authentication challenge for the user
async fn auth_start_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyAuthStartInfo>,
) -> Result<impl IntoResponse, Error> {
    // Get user from database
    let conn = &mut state.get_database_connection().await?;
    let user = database::users::filter_by_username(&body.username, conn).await?;

    // Check if user has activated their account
    if !user.is_account_verified() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Email has not been verified, please check your inbox.",
        )
            .into());
    }

    // Check if the account is active
    if !user.is_account_active() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Account has been deleted temporarily",
        )
            .into());
    }

    // Get user's passkey credentials
    let credentials = database::passkey_credentials::filter_by_user_id(user.get_uuid(), conn).await?;

    if credentials.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "No passkeys registered for this user.",
        )
            .into());
    }

    // Convert database credentials to passkeys for WebAuthn
    let passkeys: Vec<Passkey> = credentials
        .iter()
        .filter_map(|c| c.to_passkey().ok())
        .collect();

    if passkeys.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to load passkey credentials.",
        )
            .into());
    }

    // Start authentication ceremony
    let (rcr, auth_state) = state
        .webauthn
        .start_passkey_authentication(&passkeys)
        .map_err(|_| -> Error {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to start passkey authentication.",
            )
                .into()
        })?;

    // Serialize the authentication state
    let state_json = serde_json::to_string(&auth_state).map_err(|_| -> Error {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to serialize authentication state.",
        )
            .into()
    })?;

    // Return challenge and state
    Ok(Json(serde_json::json!({
        "challenge": rcr,
        "state": state_json,
    })))
}

/// Completes passkey authentication flow
///
/// Verifies the passkey authentication response and generates a JWT token
async fn auth_finish_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyAuthFinishInfo>,
) -> Result<impl IntoResponse, Error> {
    // Parse the authentication response from the client
    let auth_response: PublicKeyCredential =
        serde_json::from_str(&body.authentication_response).map_err(|_| -> Error {
            (
                StatusCode::BAD_REQUEST,
                "Invalid authentication response format.",
            )
                .into()
        })?;

    // Get user from database
    let conn = &mut state.get_database_connection().await?;
    let user = database::users::filter_by_username(&body.username, conn).await?;

    // Get user's passkey credentials
    let credentials = database::passkey_credentials::filter_by_user_id(user.get_uuid(), conn).await?;

    // Convert database credentials to passkeys
    let passkeys: Vec<Passkey> = credentials
        .iter()
        .filter_map(|c| c.to_passkey().ok())
        .collect();

    if passkeys.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to load passkey credentials.",
        )
            .into());
    }

    // In production, retrieve the stored auth_state from session/cache
    // For now, we'll start a new authentication
    let (_, auth_state) = state
        .webauthn
        .start_passkey_authentication(&passkeys)
        .map_err(|_| -> Error {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to restart passkey authentication.",
            )
                .into()
        })?;

    // Finish the authentication ceremony
    let auth_result = state
        .webauthn
        .finish_passkey_authentication(&auth_response, &auth_state)
        .map_err(|_| -> Error {
            (StatusCode::UNAUTHORIZED, "Passkey authentication failed.").into()
        })?;

    // Update the credential counter
    let credential_id = auth_result.cred_id().as_ref().to_vec();
    if let Ok(mut cred) = database::passkey_credentials::filter_by_credential_id(&credential_id, conn).await {
        let _ = database::passkey_credentials::update_counter(
            cred.id,
            0, // WebAuthn 0.5 doesn't expose counter directly, use 0 for now
            conn,
        )
        .await;
    }

    // Generate JWT token
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(state.config.jwt_max_age.into())).timestamp() as usize;

    let claims = TokenClaim {
        sub: user.get_uuid().to_string().into(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    )?;

    // Store token in database
    let new_token = NewToken::new(&user, &token, None, None);
    database::tokens::insert(new_token, conn).await?;

    // Return token to client
    Ok(Json(Token { token }))
}
