use serde::Deserialize;

/// Request to start passkey registration
#[derive(Deserialize)]
pub struct PasskeyRegisterStartInfo {
    pub username: String,
    pub email: String,
}

/// Request to finish passkey registration
#[derive(Deserialize)]
pub struct PasskeyRegisterFinishInfo {
    pub username: String,
    pub email: String,
    pub registration_response: String, // JSON string of the WebAuthn registration response
}

/// Request to start passkey authentication
#[derive(Deserialize)]
pub struct PasskeyAuthStartInfo {
    pub username: String,
}

/// Request to finish passkey authentication
#[derive(Deserialize)]
pub struct PasskeyAuthFinishInfo {
    pub username: String,
    pub authentication_response: String, // JSON string of the WebAuthn authentication response
}
