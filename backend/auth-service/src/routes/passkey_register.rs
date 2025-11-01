use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use uuid::Uuid;
use webauthn_rs::prelude::*;

use crate::{
    AppState, database,
    grpc::email_service::service::ActivateAccountRequest,
    models::{
        activation_link::NewActivationLink,
        passkey_credential::NewPasskeyCredential,
        request::passkey_register_info::{
            PasskeyRegisterFinishRequest, PasskeyRegisterStartRequest, PasskeyRegisterStartResponse,
        },
        response::{Error, TranslationKey, TranslationKeyMessage},
    },
    utils,
};

/// Creates a router for the passkey registration routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/options", post(passkey_register_start))
        .route("/complete", post(passkey_register_finish))
        .with_state(state)
}

/// Start passkey registration - generate challenge
///
/// This endpoint initiates the passkey registration process by:
/// 1. Validating the input (username, email, captcha)
/// 2. Checking if the user already exists
/// 3. Generating a WebAuthn creation challenge
/// 4. Storing the challenge state temporarily
///
/// # Flow
/// 1. Verify captcha token
/// 2. Validate username length (> 3 chars)
/// 3. Validate email format
/// 4. Check for existing username/email
/// 5. Generate WebAuthn challenge
/// 6. Store challenge and user data temporarily (5 min expiry)
/// 7. Return challenge options to client
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `body` - JSON request body containing registration information
///
/// # Returns
/// * `Ok(Json<PasskeyRegisterStartResponse>)` - Challenge options for the client
/// * `Err(Error)` - Validation or configuration errors
async fn passkey_register_start(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyRegisterStartRequest>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Passkey registration start for: {}", body.username);

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

    // Validate inputs
    if body.username.len() <= 3 {
        return Err((StatusCode::BAD_REQUEST, TranslationKey::UsernameTooShort).into());
    }

    if !email_address::EmailAddress::is_valid(&body.email) {
        return Err((StatusCode::BAD_REQUEST, TranslationKey::EmailAddressInvalid).into());
    }

    // Check if user already exists
    let pool = state.get_database_pool();
    if database::users::filter_by_username_or_email(&body.username, &body.email, pool)
        .await
        .is_ok()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            TranslationKey::UsernameOrEmailAlreadyUsed,
        )
            .into());
    }

    // Generate user ID for this registration session
    let user_id = Uuid::new_v4();

    // Generate WebAuthn challenge
    let webauthn = state.config.build_webauthn().map_err(|e| -> Error {
        tracing::error!("Failed to build WebAuthn: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::InternalServerError,
        )
            .into()
    })?;

    let (creation_challenge_response, passkey_registration) = webauthn
        .start_passkey_registration(
            user_id,
            &body.username,
            &body.username,
            None, // No existing credentials
        )
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
        .store_passkey_registration(user_id, passkey_registration)
        .await;

    // Store user registration data temporarily
    state
        .store_pending_user(user_id, body.username.clone(), body.email.clone())
        .await;

    Ok(Json(PasskeyRegisterStartResponse {
        user_id,
        creation_options: creation_challenge_response,
    }))
}

/// Finish passkey registration - verify and store credential
///
/// This endpoint completes the passkey registration process by:
/// 1. Retrieving the stored challenge and user data
/// 2. Verifying the WebAuthn credential response
/// 3. Creating the user account in the database
/// 4. Storing the passkey credential
/// 5. Sending verification email
///
/// # Flow
/// 1. Retrieve stored challenge and user data
/// 2. Verify credential response from authenticator
/// 3. Start database transaction
/// 4. Create user account (with no password)
/// 5. Store passkey credential
/// 6. Create activation link
/// 7. Commit transaction
/// 8. Send activation email
/// 9. Clean up temporary data
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `body` - JSON request body containing credential response
///
/// # Returns
/// * `Ok(Json<TranslationKeyMessage>)` - Success message on account creation
/// * `Err(Error)` - Validation, verification, or database errors
async fn passkey_register_finish(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyRegisterFinishRequest>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Passkey registration finish for user: {}", body.user_id);

    // Retrieve stored challenge and user data
    let passkey_registration = state
        .get_passkey_registration(body.user_id)
        .await
        .ok_or_else(|| -> Error {
            (
                StatusCode::BAD_REQUEST,
                TranslationKey::RegistrationSessionExpired,
            )
                .into()
        })?;

    let (username, email) =
        state
            .get_pending_user(body.user_id)
            .await
            .ok_or_else(|| -> Error {
                (
                    StatusCode::BAD_REQUEST,
                    TranslationKey::RegistrationSessionExpired,
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
    let credential: RegisterPublicKeyCredential =
        serde_json::from_value(body.credential).map_err(|e| -> Error {
            tracing::error!("Failed to parse credential: {}", e);
            (
                StatusCode::BAD_REQUEST,
                TranslationKey::PasskeyRegistrationFailed,
            )
                .into()
        })?;

    let passkey = webauthn
        .finish_passkey_registration(&credential, &passkey_registration)
        .map_err(|e| -> Error {
            tracing::error!("Passkey registration verification failed: {}", e);
            (
                StatusCode::BAD_REQUEST,
                TranslationKey::PasskeyRegistrationFailed,
            )
                .into()
        })?;

    // Create user and credential in transaction
    let pool = state.get_database_pool();
    let mut tx = pool.begin().await.map_err(|e| -> Error {
        tracing::error!("Failed to start transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        )
            .into()
    })?;

    // Create user with no password
    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, is_verified)
        VALUES ($1, $2, $3, FALSE)
        "#,
    )
    .bind(body.user_id)
    .bind(&username)
    .bind(&email)
    .execute(&mut *tx)
    .await
    .map_err(|e| -> Error {
        tracing::error!("Failed to create user: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::CouldNotCreateAccount,
        )
            .into()
    })?;

    // Store passkey credential
    // The credential ID from webauthn-rs is a HumanBinaryData type that contains raw bytes
    let credential_id_bytes: Vec<u8> = passkey.cred_id().clone().into();

    // Serialize the entire Passkey object as JSON for storage
    let public_key_json = serde_json::to_vec(&passkey).map_err(|e| -> Error {
        tracing::error!("Failed to serialize passkey: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        )
            .into()
    })?;

    let new_credential = NewPasskeyCredential {
        user_id: body.user_id,
        credential_id: credential_id_bytes,
        public_key: public_key_json,
        counter: 0, // Initial counter is 0 for new passkeys
        aaguid: None,
        device_name: body.device_name,
        user_agent: None,
    };

    database::passkey_credentials::insert(new_credential, &mut tx).await?;

    // Create activation link
    let new_activation_link = NewActivationLink::new(body.user_id);
    let link = new_activation_link.get_link(&state.config);
    database::activation_links::insert(new_activation_link, &mut *tx).await?;

    tx.commit().await.map_err(|e| -> Error {
        tracing::error!("Failed to commit transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        )
            .into()
    })?;

    // Send activation email
    let request = ActivateAccountRequest {
        username: username.clone(),
        email: email.clone(),
        link,
    };

    state
        .send_activate_account(request)
        .await
        .map_err(|e| -> Error {
            tracing::error!("Failed to send activation email: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                TranslationKey::SomethingWentWrong,
            )
                .into()
        })?;

    tracing::info!("Passkey registration successful for: {}", username);

    Ok(Json(TranslationKeyMessage {
        translation_key: TranslationKey::AccountCreated,
    }))
}
