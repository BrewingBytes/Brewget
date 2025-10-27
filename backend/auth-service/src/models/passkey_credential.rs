use sqlx::FromRow;
use uuid::Uuid;
use webauthn_rs::prelude::*;

/// Represents a passkey credential stored in the database
///
/// This struct maps to the passkey_credentials table
#[derive(FromRow, Clone, Debug)]
pub struct PasskeyCredential {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub counter: i64,
    pub transports: Option<Vec<String>>,
    pub backup_eligible: bool,
    pub backup_state: bool,
    pub attestation_type: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Represents a new passkey credential to be inserted
pub struct NewPasskeyCredential {
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub counter: i64,
    pub transports: Option<Vec<String>>,
    pub backup_eligible: bool,
    pub backup_state: bool,
    pub attestation_type: Option<String>,
}

impl NewPasskeyCredential {
    /// Creates a new passkey credential from a Passkey
    pub fn from_passkey(user_id: Uuid, passkey: &Passkey) -> Self {
        Self {
            user_id,
            credential_id: passkey.cred_id().as_ref().to_vec(),
            public_key: serde_json::to_vec(passkey).unwrap_or_default(),
            counter: 0,
            transports: None,
            backup_eligible: false,
            backup_state: false,
            attestation_type: None,
        }
    }
}

impl PasskeyCredential {
    /// Converts the database credential to a Passkey for WebAuthn validation
    pub fn to_passkey(&self) -> Result<Passkey, ()> {
        serde_json::from_slice(&self.public_key).map_err(|_| ())
    }
}
