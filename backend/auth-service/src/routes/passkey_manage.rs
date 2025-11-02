use std::{str::FromStr, sync::Arc};

use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
};
use uuid::Uuid;
use webauthn_rs::prelude::*;

use crate::{
    AppState, database,
    models::{
        passkey_credential::{NewPasskeyCredential, PasskeyCredentialResponse},
        request::passkey_register_info::{
            PasskeyRegisterFinishRequest, PasskeyRegisterStartResponse,
        },
        response::{Error, TranslationKey, TranslationKeyMessage},
    },
    routes::middlewares::auth_guard::auth_guard,
};

/// Creates a router for the passkey management routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(list_passkeys))
        .route("/add/options", post(add_passkey_start))
        .route("/add/complete", post(add_passkey_finish))
        .route("/:id", delete(remove_passkey))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard))
        .with_state(state)
}

/// List all passkeys for the authenticated user
///
/// This endpoint retrieves all active passkey credentials for the authenticated user.
///
/// # Returns
/// * `Ok(Json<Vec<PasskeyCredentialResponse>>)` - List of passkey credentials
/// * `Err(Error)` - Database error
async fn list_passkeys(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
) -> Result<impl IntoResponse, Error> {
    let user_id = Uuid::from_str(&user_uuid)?;
    tracing::info!("Listing passkeys for user: {}", user_id);

    let pool = state.get_database_pool();
    let credentials = database::passkey_credentials::find_by_user_id(user_id, pool).await?;

    let response: Vec<PasskeyCredentialResponse> =
        credentials.into_iter().map(Into::into).collect();

    Ok(Json(response))
}

/// Start adding a new passkey for an authenticated user
///
/// This endpoint initiates the process of adding a new passkey to an existing account.
/// Unlike registration, this requires the user to be already authenticated.
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `user_uuid` - Authenticated user's ID from middleware
///
/// # Returns
/// * `Ok(Json<PasskeyRegisterStartResponse>)` - Challenge options for the client
/// * `Err(Error)` - Configuration or database errors
async fn add_passkey_start(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
) -> Result<impl IntoResponse, Error> {
    let user_id = Uuid::from_str(&user_uuid)?;
    tracing::info!("Starting passkey addition for user: {}", user_id);

    let pool = state.get_database_pool();

    // Get user info
    let user = database::users::filter_by_uuid(user_id, pool).await?;

    // Get existing credentials
    let existing_credentials =
        database::passkey_credentials::find_by_user_id(user_id, pool).await?;

    // Convert credentials to Passkey format
    let passkeys: Vec<Passkey> = existing_credentials
        .iter()
        .filter_map(|c| serde_json::from_slice::<Passkey>(&c.public_key).ok())
        .collect();

    // Get credentials ids to exclude
    let credentials: Vec<CredentialID> = passkeys.iter().map(|pk| pk.cred_id()).cloned().collect();

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
            &user.get_username(),
            &user.get_username(),
            Some(credentials),
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

    Ok(Json(PasskeyRegisterStartResponse {
        user_id,
        creation_options: creation_challenge_response,
    }))
}

/// Finish adding a new passkey for an authenticated user
///
/// This endpoint completes the process of adding a new passkey by verifying
/// the WebAuthn credential response and storing it.
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `user_uuid` - Authenticated user's ID from middleware
/// * `body` - JSON request body containing credential response
///
/// # Returns
/// * `Ok(Json<TranslationKeyMessage>)` - Success message
/// * `Err(Error)` - Verification or database errors
async fn add_passkey_finish(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
    Json(body): Json<PasskeyRegisterFinishRequest>,
) -> Result<impl IntoResponse, Error> {
    let user_id = Uuid::from_str(&user_uuid)?;
    tracing::info!("Finishing passkey addition for user: {}", user_id);

    // Retrieve stored challenge
    let passkey_registration =
        state
            .get_passkey_registration(user_id)
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

    // Store passkey credential
    let pool = state.get_database_pool();
    let mut tx = pool.begin().await.map_err(|e| -> Error {
        tracing::error!("Failed to start transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        )
            .into()
    })?;

    let credential_id_bytes: Vec<u8> = passkey.cred_id().clone().into();
    let public_key_json = serde_json::to_vec(&passkey).map_err(|e| -> Error {
        tracing::error!("Failed to serialize passkey: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        )
            .into()
    })?;

    let new_credential = NewPasskeyCredential {
        user_id,
        credential_id: credential_id_bytes,
        public_key: public_key_json,
        counter: 0,
        aaguid: None,
        device_name: body.device_name,
        user_agent: None,
    };

    database::passkey_credentials::insert(new_credential, &mut tx).await?;

    tx.commit().await.map_err(|e| -> Error {
        tracing::error!("Failed to commit transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            TranslationKey::SomethingWentWrong,
        )
            .into()
    })?;

    tracing::info!("Passkey addition successful for user: {}", user_id);

    Ok(Json(TranslationKeyMessage {
        translation_key: TranslationKey::PasskeyAddedSuccessfully,
    }))
}

/// Remove a passkey for the authenticated user
///
/// This endpoint deactivates a specific passkey credential.
///
/// # Arguments
/// * `state` - Application state containing DB connection
/// * `user_uuid` - Authenticated user's ID from middleware
/// * `credential_id` - ID of the credential to remove
///
/// # Returns
/// * `Ok(Json<TranslationKeyMessage>)` - Success message
/// * `Err(Error)` - Database error or credential not found
async fn remove_passkey(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
    axum::extract::Path(credential_id): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let user_id = Uuid::from_str(&user_uuid)?;
    tracing::info!("Removing passkey {} for user: {}", credential_id, user_id);

    let pool = state.get_database_pool();
    database::passkey_credentials::delete(credential_id, user_id, pool).await?;

    tracing::info!("Passkey removal successful for user: {}", user_id);

    Ok(Json(TranslationKeyMessage {
        translation_key: TranslationKey::PasskeyRemovedSuccessfully,
    }))
}
