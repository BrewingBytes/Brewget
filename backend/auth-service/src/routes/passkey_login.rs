use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use webauthn_rs::prelude::*;

use crate::{
    AppState, database,
    models::{
        authentication_audit_log::AuthMethod,
        request::passkey_login_info::{
            PasskeyLoginFinishRequest, PasskeyLoginStartRequest, PasskeyLoginStartResponse,
        },
        response::{Error, Token, TranslationKey},
        token::NewToken,
        token_claim::TokenClaim,
    },
    utils,
};

/// Creates a router for the passkey login routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/options", post(passkey_login_start))
        .route("/complete", post(passkey_login_finish))
        .with_state(state)
}

/// Start passkey login - generate challenge
///
/// This endpoint initiates the passkey authentication process by:
/// 1. Validating the input (username, captcha)
/// 2. Finding the user and their passkeys
/// 3. Generating a WebAuthn authentication challenge
/// 4. Storing the challenge state temporarily
///
/// # Flow
/// 1. Verify captcha token
/// 2. Find user by username
/// 3. Check if user has passkeys configured
/// 4. Retrieve user's active passkeys
/// 5. Generate WebAuthn authentication challenge
/// 6. Store challenge temporarily (5 min expiry)
/// 7. Return challenge options to client
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `body` - JSON request body containing login information
///
/// # Returns
/// * `Ok(Json<PasskeyLoginStartResponse>)` - Challenge options for the client
/// * `Err(Error)` - Validation or configuration errors
async fn passkey_login_start(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyLoginStartRequest>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Passkey login start for: {}", body.username);

    // Verify captcha
    utils::captcha::verify_turnstile(&body.captcha_token, &state.config.turnstile_secret)
        .await
        .map_err(|_| -> Error {
            (
                StatusCode::BAD_REQUEST,
                TranslationKey::CaptchaVerificationFailed,
            )
                .into()
        })?;

    // Find user
    let pool = state.get_database_pool();
    let user = database::users::filter_by_username(&body.username, pool).await?;

    // Check if user has passkeys
    if !user.has_passkey() {
        return Err((StatusCode::BAD_REQUEST, TranslationKey::NoPasskeyConfigured).into());
    }

    // Get user's passkeys
    let credentials = database::passkey_credentials::find_by_user_id(user.get_uuid(), pool).await?;

    if credentials.is_empty() {
        return Err((StatusCode::BAD_REQUEST, TranslationKey::NoPasskeyConfigured).into());
    }

    // Convert credentials to Passkey format
    let passkeys: Vec<Passkey> = credentials
        .iter()
        .filter_map(|c| {
            // Deserialize the passkey from stored JSON
            serde_json::from_slice::<Passkey>(&c.public_key).ok()
        })
        .collect();

    if passkeys.is_empty() {
        tracing::error!("Failed to parse any passkeys for user: {}", body.username);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
            .into());
    }

    // Generate challenge
    let webauthn = state.config.build_webauthn().map_err(|e| -> Error {
        tracing::error!("Failed to build WebAuthn: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
            .into()
    })?;

    let (request_challenge_response, passkey_authentication) = webauthn
        .start_passkey_authentication(&passkeys)
        .map_err(|e| -> Error {
            tracing::error!("WebAuthn challenge generation failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                TranslationKey::InternalServerError,
            )
                .into()
        })?;

    // Store challenge state temporarily
    state
        .store_passkey_authentication(body.username.clone(), passkey_authentication)
        .await;

    Ok(Json(PasskeyLoginStartResponse {
        request_options: request_challenge_response,
    }))
}

/// Finish passkey login - verify and issue token
///
/// This endpoint completes the passkey authentication process by:
/// 1. Retrieving the stored challenge
/// 2. Verifying the WebAuthn assertion response
/// 3. Updating the credential counter
/// 4. Generating a JWT token
/// 5. Storing the token in the database
///
/// # Flow
/// 1. Retrieve stored authentication challenge
/// 2. Verify assertion response from authenticator
/// 3. Find user by username
/// 4. Update credential counter (replay attack prevention)
/// 5. Generate JWT token
/// 6. Store token in database
/// 7. Clean up temporary data
/// 8. Return JWT token
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `body` - JSON request body containing assertion response
///
/// # Returns
/// * `Ok(Json<Token>)` - JWT token for authenticated session
/// * `Err(Error)` - Verification or database errors
async fn passkey_login_finish(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<PasskeyLoginFinishRequest>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Passkey login finish for: {}", body.username);

    // Extract IP address and user agent from headers
    let (ip_address, user_agent) = utils::audit::extract_request_metadata(&headers);

    // Retrieve stored challenge
    let passkey_authentication = state
        .get_passkey_authentication(&body.username)
        .await
        .ok_or_else(|| -> Error {
            (
                StatusCode::BAD_REQUEST,
                TranslationKey::AuthenticationSessionExpired,
            )
                .into()
        })?;

    // Verify credential
    let webauthn = state.config.build_webauthn().map_err(|e| -> Error {
        tracing::error!("Failed to build WebAuthn: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
            .into()
    })?;

    // Parse the credential from JSON
    let credential: PublicKeyCredential =
        serde_json::from_value(body.credential.clone()).map_err(|e| -> Error {
            tracing::error!("Failed to parse credential: {}", e);
            (
                StatusCode::BAD_REQUEST,
                TranslationKey::PasskeyAuthenticationFailed,
            )
                .into()
        })?;

    // Get user and pool early so we can log audit events
    let pool = state.get_database_pool();
    let user = database::users::filter_by_username(&body.username, pool).await?;

    // Attempt authentication
    let authentication_result = match webauthn
        .finish_passkey_authentication(&credential, &passkey_authentication)
    {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Passkey authentication failed: {}", e);

            // Log failed authentication attempt
            utils::audit::log_authentication_attempt(
                user.get_uuid(),
                AuthMethod::Passkey,
                false,
                ip_address.clone(),
                user_agent.clone(),
                Some("passkey_verification_failed"),
                pool,
            )
            .await;

            return Err((
                StatusCode::UNAUTHORIZED,
                TranslationKey::PasskeyAuthenticationFailed,
            )
                .into());
        }
    };

    // Check if account is verified
    if !user.is_account_verified() {
        // Log failed authentication attempt
        utils::audit::log_authentication_attempt(
            user.get_uuid(),
            AuthMethod::Passkey,
            false,
            ip_address.clone(),
            user_agent.clone(),
            Some("account_not_verified"),
            pool,
        )
        .await;

        return Err((StatusCode::FORBIDDEN, TranslationKey::EmailNotVerified).into());
    }

    // Check if account is active
    if !user.is_account_active() {
        // Log failed authentication attempt
        utils::audit::log_authentication_attempt(
            user.get_uuid(),
            AuthMethod::Passkey,
            false,
            ip_address.clone(),
            user_agent.clone(),
            Some("account_inactive"),
            pool,
        )
        .await;

        return Err((
            StatusCode::FORBIDDEN,
            TranslationKey::AccountDeletedTemporarily,
        )
            .into());
    }

    // Update credential counter
    // Extract credential ID as raw bytes for comparison
    let auth_cred_id_bytes: Vec<u8> = authentication_result.cred_id().clone().into();

    // Find the credential to get its stored format
    let stored_credential = database::passkey_credentials::find_by_user_id(user.get_uuid(), pool)
        .await?
        .into_iter()
        .find(|c| c.credential_id == auth_cred_id_bytes)
        .ok_or_else(|| -> Error {
            (StatusCode::NOT_FOUND, TranslationKey::PasskeyNotFound).into()
        })?;

    database::passkey_credentials::update_counter(
        &stored_credential.credential_id,
        authentication_result.counter() as i64,
        pool,
    )
    .await?;

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

    // Store token
    let new_token = NewToken::new(&user, &token, None, None);
    database::tokens::insert(new_token, pool).await?;

    // Log successful authentication attempt
    utils::audit::log_authentication_attempt(
        user.get_uuid(),
        AuthMethod::Passkey,
        true,
        ip_address,
        user_agent,
        None,
        pool,
    )
    .await;

    tracing::info!("Passkey login successful for: {}", body.username);

    Ok(Json(Token { token }))
}
