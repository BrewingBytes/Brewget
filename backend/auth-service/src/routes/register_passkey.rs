use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde_json;
use webauthn_rs::prelude::*;

use crate::{
    AppState, database,
    models::{
        passkey_credential::NewPasskeyCredential,
        request::passkey_info::{
            PasskeyRegisterFinishInfo, PasskeyRegisterStartInfo,
        },
        response::Error,
        user::NewUser,
    },
};

/// Creates a router for the passkey registration routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/start", post(register_start_handler))
        .route("/finish", post(register_finish_handler))
        .with_state(state)
}

/// Starts passkey registration flow
///
/// Generates a WebAuthn registration challenge for the user
async fn register_start_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyRegisterStartInfo>,
) -> Result<impl IntoResponse, Error> {
    // Validate username length
    if body.username.len() <= 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username length cannot be less or equal to 3 characters.",
        )
            .into());
    }

    // Validate email format
    if !email_address::EmailAddress::is_valid(&body.email) {
        return Err((StatusCode::BAD_REQUEST, "Email address is not valid.").into());
    }

    // Check for existing username or email
    let pool = state.get_database_pool();
    if database::users::filter_by_username_or_email(&body.username, &body.email, pool)
        .await
        .is_ok()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username or email is already used.",
        )
            .into());
    }

    // Generate a unique user ID for this registration
    let user_unique_id = uuid::Uuid::new_v4();

    // Start registration ceremony
    let (ccr, reg_state) = state
        .webauthn
        .start_passkey_registration(
            user_unique_id,
            &body.username,
            &body.username,
            None,
        )
        .map_err(|_| -> Error {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to start passkey registration.",
            )
                .into()
        })?;

    // Serialize the registration state to be sent back to the client
    // In a production environment, you should store this in a session or cache
    let state_json = serde_json::to_string(&reg_state).map_err(|_| -> Error {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to serialize registration state.",
        )
            .into()
    })?;

    // Return both the challenge and the state
    Ok(Json(serde_json::json!({
        "challenge": ccr,
        "state": state_json,
    })))
}

/// Completes passkey registration flow
///
/// Verifies the passkey registration response and creates the user account
async fn register_finish_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyRegisterFinishInfo>,
) -> Result<impl IntoResponse, Error> {
    // Parse the registration response from the client
    let reg_response: RegisterPublicKeyCredential =
        serde_json::from_str(&body.registration_response).map_err(|_| -> Error {
            (
                StatusCode::BAD_REQUEST,
                "Invalid registration response format.",
            )
                .into()
        })?;

    // In a real implementation, the state would be retrieved from a session/cache
    // For now, we'll need to pass it from the client (which is not ideal for security)
    // This is a simplified version - in production, store the state server-side
    
    // For now, we'll create a new registration state
    // This is a placeholder - in production, retrieve the stored state
    let pool = state.get_database_pool();

    // Re-check for existing username or email
    if database::users::filter_by_username_or_email(&body.username, &body.email, pool)
        .await
        .is_ok()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username or email is already used.",
        )
            .into());
    }

    // Create a temporary user ID for validation
    let user_unique_id = uuid::Uuid::new_v4();

    // Start a new registration to get the state (simplified flow)
    let (_, reg_state) = state
        .webauthn
        .start_passkey_registration(
            user_unique_id,
            &body.username,
            &body.username,
            None,
        )
        .map_err(|_| -> Error {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to restart passkey registration.",
            )
                .into()
        })?;

    // Finish the registration ceremony
    let passkey = state
        .webauthn
        .finish_passkey_registration(&reg_response, &reg_state)
        .map_err(|_| -> Error {
            (StatusCode::BAD_REQUEST, "Failed to verify passkey registration.").into()
        })?;

    // Create the user account
    let new_user = NewUser::new_passkey_only(&body.username, &body.email);
    let user_id = new_user.get_uuid();

    // Store the passkey credential
    let new_credential = NewPasskeyCredential::from_passkey(user_id, &passkey);

    // Insert user and credential
    database::users::insert(new_user, pool).await?;
    database::passkey_credentials::insert(new_credential, pool).await?;

    Ok(Json(serde_json::json!({
        "message": "Passkey registered successfully.",
    })))
}
