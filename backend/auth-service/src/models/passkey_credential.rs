use diesel::prelude::*;
use uuid::Uuid;
use webauthn_rs::prelude::*;

/// Represents a passkey credential stored in the database
///
/// This struct maps to the passkey_credentials table
#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::passkey_credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
#[derive(Insertable)]
#[diesel(table_name = crate::schema::passkey_credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

    /// Updates the counter after successful authentication
    pub fn update_counter(&mut self, new_counter: u32) {
        self.counter = new_counter as i64;
        self.last_used_at = Some(chrono::Utc::now());
    }
}

/// Update struct for counter after authentication
#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::passkey_credentials)]
pub struct UpdatePasskeyCounter {
    pub counter: i64,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
}
