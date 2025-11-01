use serde::{Deserialize, Serialize};
use webauthn_rs_proto::RequestChallengeResponse;

/// Request to start passkey login
///
/// # Fields
/// * `username` - Username of the account to authenticate
/// * `captcha_token` - Turnstile captcha verification token
#[derive(Deserialize)]
pub struct PasskeyLoginStartRequest {
    pub username: String,
    pub captcha_token: String,
}

/// Response from starting passkey login
///
/// # Fields
/// * `request_options` - WebAuthn authentication challenge options
#[derive(Serialize)]
pub struct PasskeyLoginStartResponse {
    pub request_options: RequestChallengeResponse,
}

/// Request to complete passkey login
///
/// # Fields
/// * `username` - Username of the account to authenticate
/// * `credential` - WebAuthn assertion response from the authenticator
#[derive(Deserialize)]
pub struct PasskeyLoginFinishRequest {
    pub username: String,
    pub credential: serde_json::Value,
}
