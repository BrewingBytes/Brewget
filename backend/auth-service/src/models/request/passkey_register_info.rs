use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs_proto::CreationChallengeResponse;

/// Request to start passkey registration
///
/// # Fields
/// * `username` - Desired username for the new account
/// * `email` - Email address for the new account
/// * `captcha_token` - Turnstile captcha verification token
#[derive(Deserialize)]
pub struct PasskeyRegisterStartRequest {
    pub username: String,
    pub email: String,
    #[serde(rename = "captchaToken")]
    pub captcha_token: String,
}

/// Response from starting passkey registration
///
/// # Fields
/// * `user_id` - Temporary user ID for this registration session
/// * `creation_options` - WebAuthn creation challenge options
#[derive(Serialize)]
pub struct PasskeyRegisterStartResponse {
    pub user_id: Uuid,
    pub creation_options: CreationChallengeResponse,
}

/// Request to complete passkey registration
///
/// # Fields
/// * `user_id` - Temporary user ID from the start request
/// * `credential` - WebAuthn credential response from the authenticator
/// * `device_name` - Optional user-friendly name for the device
#[derive(Deserialize)]
pub struct PasskeyRegisterFinishRequest {
    pub user_id: Uuid,
    pub credential: serde_json::Value,
    pub device_name: Option<String>,
}
