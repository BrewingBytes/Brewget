use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a passkey credential stored in the database
///
/// This struct maps to the passkey_credentials table and contains WebAuthn credential data
///
/// # Fields
/// * `id` - Unique identifier for this credential record
/// * `user_id` - ID of the user who owns this credential
/// * `credential_id` - WebAuthn credential ID (unique identifier from authenticator)
/// * `public_key` - Public key associated with this credential
/// * `counter` - Signature counter for replay attack prevention
/// * `aaguid` - Authenticator attestation GUID (optional)
/// * `credential_device_type` - Type of device used (optional)
/// * `credential_backed_up` - Whether credential is backed up (optional)
/// * `device_name` - User-friendly name for the device (optional)
/// * `user_agent` - User agent string from registration (optional)
/// * `created_at` - When this credential was created
/// * `last_used_at` - When this credential was last used for authentication
/// * `is_active` - Whether this credential is active or has been revoked
#[derive(FromRow, Clone)]
#[allow(dead_code)]
pub struct PasskeyCredential {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub counter: i64,
    pub aaguid: Option<Vec<u8>>,
    pub credential_device_type: Option<String>,
    pub credential_backed_up: Option<bool>,
    pub device_name: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// Represents a new passkey credential to be inserted into the database
///
/// # Fields
/// * `user_id` - ID of the user who owns this credential
/// * `credential_id` - WebAuthn credential ID
/// * `public_key` - Public key for this credential
/// * `counter` - Initial signature counter value
/// * `aaguid` - Authenticator attestation GUID (optional)
/// * `device_name` - User-friendly name for the device (optional)
/// * `user_agent` - User agent string from registration (optional)
pub struct NewPasskeyCredential {
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub counter: i64,
    pub aaguid: Option<Vec<u8>>,
    pub device_name: Option<String>,
    pub user_agent: Option<String>,
}
