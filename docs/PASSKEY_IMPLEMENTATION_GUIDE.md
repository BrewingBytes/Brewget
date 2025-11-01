# Passkey Authentication - Technical Implementation Guide

## Overview

This document provides technical details for implementing passkey authentication in the BrewGet application using WebAuthn API.

## Architecture

### High-Level Components

```
┌─────────────────────────────────────────────────┐
│              Frontend (Vue.js)                  │
├─────────────────────────────────────────────────┤
│  - WebAuthn API Integration                     │
│  - Passkey Registration UI                      │
│  - Passkey Login UI                             │
│  - Passkey Management in Settings               │
└────────────────┬────────────────────────────────┘
                 │ HTTPS/JSON
                 ↓
┌─────────────────────────────────────────────────┐
│         Auth Service (Rust/Axum)                │
├─────────────────────────────────────────────────┤
│  - WebAuthn Server (webauthn-rs crate)         │
│  - Passkey Registration Endpoints               │
│  - Passkey Authentication Endpoints             │
│  - Credential Storage & Verification            │
└────────────────┬────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────────────┐
│           PostgreSQL Database                    │
├─────────────────────────────────────────────────┤
│  - users table (modified: password optional)    │
│  - passkey_credentials table (new)              │
│  - authentication_audit_log table (new)         │
└─────────────────────────────────────────────────┘
```

## Database Schema Changes

### 1. Modify `users` table

```sql
-- Migration: Make password optional for passkey-only accounts
ALTER TABLE users 
ALTER COLUMN password DROP NOT NULL;

-- Add column to track authentication methods
ALTER TABLE users 
ADD COLUMN has_passkey BOOLEAN NOT NULL DEFAULT FALSE;

-- Add index for faster passkey lookups
CREATE INDEX idx_users_has_passkey ON users(has_passkey);
```

### 2. Create `passkey_credentials` table

```sql
CREATE TABLE passkey_credentials (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- WebAuthn credential data
    credential_id BYTEA NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,
    counter BIGINT NOT NULL DEFAULT 0,
    
    -- Credential metadata
    aaguid BYTEA,
    credential_device_type TEXT,
    credential_backed_up BOOLEAN DEFAULT FALSE,
    
    -- User-facing information
    device_name TEXT,
    user_agent TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,
    
    -- Additional security
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Indexes
CREATE INDEX idx_passkey_user_id ON passkey_credentials(user_id);
CREATE INDEX idx_passkey_credential_id ON passkey_credentials(credential_id);
CREATE INDEX idx_passkey_active ON passkey_credentials(is_active) WHERE is_active = TRUE;

-- Trigger to update users.has_passkey
CREATE OR REPLACE FUNCTION update_user_has_passkey()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE users SET has_passkey = TRUE WHERE id = NEW.user_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE users 
        SET has_passkey = EXISTS(
            SELECT 1 FROM passkey_credentials 
            WHERE user_id = OLD.user_id AND is_active = TRUE
        )
        WHERE id = OLD.user_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER passkey_user_sync
AFTER INSERT OR DELETE ON passkey_credentials
FOR EACH ROW
EXECUTE FUNCTION update_user_has_passkey();
```

### 3. Create `authentication_audit_log` table (optional but recommended)

```sql
CREATE TABLE authentication_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Authentication details
    auth_method VARCHAR(20) NOT NULL, -- 'password', 'passkey', 'otp'
    success BOOLEAN NOT NULL,
    
    -- Request context
    ip_address INET,
    user_agent TEXT,
    
    -- Timestamps
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Additional data (JSON for flexibility)
    metadata JSONB
);

-- Indexes for audit queries
CREATE INDEX idx_auth_log_user_id ON authentication_audit_log(user_id);
CREATE INDEX idx_auth_log_attempted_at ON authentication_audit_log(attempted_at DESC);
CREATE INDEX idx_auth_log_method ON authentication_audit_log(auth_method);
```

## Backend Implementation (Rust)

### Dependencies (Cargo.toml)

```toml
[dependencies]
# Existing dependencies...
webauthn-rs = "0.5"
webauthn-rs-proto = "0.5"
base64 = "0.22"
```

### WebAuthn Configuration

```rust
// src/config.rs - Add to existing config
use webauthn_rs::{Webauthn, WebauthnBuilder};
use url::Url;

#[derive(Clone)]
pub struct Config {
    // ... existing config fields ...
    
    // WebAuthn config
    pub rp_id: String,          // "brewget.com"
    pub rp_origin: Url,         // "https://brewget.com"
    pub rp_name: String,        // "BrewGet"
}

impl Config {
    pub fn build_webauthn(&self) -> Result<Webauthn, Box<dyn std::error::Error>> {
        let builder = WebauthnBuilder::new(&self.rp_id, &self.rp_origin)?
            .rp_name(&self.rp_name);
        
        Ok(builder.build()?)
    }
}
```

### Models

```rust
// src/models/passkey_credential.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use webauthn_rs::prelude::*;

#[derive(FromRow, Clone)]
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

impl PasskeyCredential {
    /// Convert to Webauthn's Passkey type for authentication
    pub fn to_passkey(&self) -> Result<Passkey, Box<dyn std::error::Error>> {
        // Implementation depends on webauthn-rs version
        // This is a simplified example
        Ok(Passkey {
            cred_id: self.credential_id.clone().into(),
            cred: COSEKey::try_from(self.public_key.as_slice())?,
            counter: self.counter as u32,
            transports: None,
            user_verified: true,
            backup_eligible: self.credential_backed_up.unwrap_or(false),
            backup_state: self.credential_backed_up.unwrap_or(false),
            registration_policy: UserVerificationPolicy::Required,
            extensions: RegisteredExtensions::default(),
            attestation: ParsedAttestationData::None,
            attestation_format: AttestationFormat::None,
        })
    }
}

#[derive(Deserialize)]
pub struct NewPasskeyCredential {
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub counter: i64,
    pub aaguid: Option<Vec<u8>>,
    pub device_name: Option<String>,
    pub user_agent: Option<String>,
}
```

```rust
// src/models/request/passkey_register_info.rs

use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::*;

#[derive(Deserialize)]
pub struct PasskeyRegisterStartRequest {
    pub username: String,
    pub email: String,
    pub captcha_token: String,
}

#[derive(Serialize)]
pub struct PasskeyRegisterStartResponse {
    pub user_id: Uuid,
    pub creation_options: CreationChallengeResponse,
}

#[derive(Deserialize)]
pub struct PasskeyRegisterFinishRequest {
    pub user_id: Uuid,
    pub credential: RegisterPublicKeyCredential,
    pub device_name: Option<String>,
}
```

```rust
// src/models/request/passkey_login_info.rs

use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::*;

#[derive(Deserialize)]
pub struct PasskeyLoginStartRequest {
    pub username: String,
    pub captcha_token: String,
}

#[derive(Serialize)]
pub struct PasskeyLoginStartResponse {
    pub request_options: RequestChallengeResponse,
}

#[derive(Deserialize)]
pub struct PasskeyLoginFinishRequest {
    pub username: String,
    pub credential: PublicKeyCredential,
}
```

### Database Operations

```rust
// src/database/passkey_credentials.rs

use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use crate::models::passkey_credential::{PasskeyCredential, NewPasskeyCredential};
use crate::models::response::{Error, TranslationKey};
use axum::http::StatusCode;

pub async fn insert(
    credential: NewPasskeyCredential,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<PasskeyCredential, Error> {
    sqlx::query_as::<_, PasskeyCredential>(
        r#"
        INSERT INTO passkey_credentials 
            (user_id, credential_id, public_key, counter, aaguid, device_name, user_agent)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#
    )
    .bind(credential.user_id)
    .bind(credential.credential_id)
    .bind(credential.public_key)
    .bind(credential.counter)
    .bind(credential.aaguid)
    .bind(credential.device_name)
    .bind(credential.user_agent)
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert passkey credential: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::SomethingWentWrong).into()
    })
}

pub async fn find_by_user_id(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<PasskeyCredential>, Error> {
    sqlx::query_as::<_, PasskeyCredential>(
        r#"
        SELECT * FROM passkey_credentials
        WHERE user_id = $1 AND is_active = TRUE
        ORDER BY created_at DESC
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch passkey credentials: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::SomethingWentWrong).into()
    })
}

pub async fn find_by_credential_id(
    credential_id: &[u8],
    pool: &PgPool,
) -> Result<PasskeyCredential, Error> {
    sqlx::query_as::<_, PasskeyCredential>(
        r#"
        SELECT * FROM passkey_credentials
        WHERE credential_id = $1 AND is_active = TRUE
        "#
    )
    .bind(credential_id)
    .fetch_one(pool)
    .await
    .map_err(|_| {
        (StatusCode::NOT_FOUND, TranslationKey::PasskeyNotFound).into()
    })
}

pub async fn update_counter(
    credential_id: &[u8],
    new_counter: i64,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query(
        r#"
        UPDATE passkey_credentials
        SET counter = $2, last_used_at = NOW()
        WHERE credential_id = $1
        "#
    )
    .bind(credential_id)
    .bind(new_counter)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update passkey counter: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::SomethingWentWrong).into()
    })?;
    
    Ok(())
}

pub async fn delete(
    credential_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query(
        r#"
        UPDATE passkey_credentials
        SET is_active = FALSE
        WHERE id = $1 AND user_id = $2
        "#
    )
    .bind(credential_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete passkey credential: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::SomethingWentWrong).into()
    })?;
    
    Ok(())
}
```

### API Endpoints

```rust
// src/routes/passkey_register.rs

use std::sync::Arc;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use webauthn_rs::prelude::*;

use crate::{
    AppState, database,
    models::{
        passkey_credential::NewPasskeyCredential,
        request::passkey_register_info::*,
        response::{Error, TranslationKey, TranslationKeyMessage},
        user::NewUser,
        activation_link::NewActivationLink,
    },
};

pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/start", post(passkey_register_start))
        .route("/finish", post(passkey_register_finish))
        .with_state(state)
}

/// Start passkey registration - generate challenge
async fn passkey_register_start(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyRegisterStartRequest>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Passkey registration start for: {}", body.username);
    
    // Verify captcha
    crate::utils::captcha::verify_turnstile(&body.captcha_token, &state.config.turnstile_secret)
        .await
        .map_err(|_| -> Error {
            (StatusCode::BAD_REQUEST, TranslationKey::CaptchaVerificationFailed).into()
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
        ).into());
    }
    
    // Create user with no password (will be created in finish step)
    let user_id = Uuid::new_v4();
    
    // Generate WebAuthn challenge
    let webauthn = state.config.build_webauthn()
        .map_err(|_| -> Error {
            (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::InternalServerError).into()
        })?;
    
    let user_unique_id = user_id.as_bytes().to_vec();
    
    let (creation_challenge_response, passkey_registration) = webauthn
        .start_passkey_registration(
            user_unique_id.into(),
            &body.username,
            &body.username,
            None, // No existing credentials
        )
        .map_err(|e| -> Error {
            tracing::error!("WebAuthn challenge generation failed: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::InternalServerError).into()
        })?;
    
    // Store challenge state temporarily (in-memory cache or Redis)
    // For this example, we'll use the app state's in-memory store
    state.store_passkey_registration(user_id, passkey_registration).await;
    
    // Also store user registration data temporarily
    state.store_pending_user(user_id, body.username.clone(), body.email.clone()).await;
    
    Ok(Json(PasskeyRegisterStartResponse {
        user_id,
        creation_options: creation_challenge_response,
    }))
}

/// Finish passkey registration - verify and store credential
async fn passkey_register_finish(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyRegisterFinishRequest>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Passkey registration finish for user: {}", body.user_id);
    
    // Retrieve stored challenge and user data
    let passkey_registration = state.get_passkey_registration(body.user_id).await
        .ok_or_else(|| -> Error {
            (StatusCode::BAD_REQUEST, TranslationKey::RegistrationSessionExpired).into()
        })?;
    
    let (username, email) = state.get_pending_user(body.user_id).await
        .ok_or_else(|| -> Error {
            (StatusCode::BAD_REQUEST, TranslationKey::RegistrationSessionExpired).into()
        })?;
    
    // Verify credential
    let webauthn = state.config.build_webauthn()
        .map_err(|_| -> Error {
            (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::InternalServerError).into()
        })?;
    
    let passkey = webauthn
        .finish_passkey_registration(&body.credential, &passkey_registration)
        .map_err(|e| -> Error {
            tracing::error!("Passkey registration verification failed: {}", e);
            (StatusCode::BAD_REQUEST, TranslationKey::PasskeyRegistrationFailed).into()
        })?;
    
    // Create user and credential in transaction
    let pool = state.get_database_pool();
    let mut tx = pool.begin().await.map_err(|_| -> Error {
        (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::SomethingWentWrong).into()
    })?;
    
    // Create user with no password
    let new_user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, email, is_verified)
        VALUES ($1, $2, $3, FALSE)
        RETURNING *
        "#
    )
    .bind(body.user_id)
    .bind(&username)
    .bind(&email)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| -> Error {
        (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::CouldNotCreateAccount).into()
    })?;
    
    // Store passkey credential
    let new_credential = NewPasskeyCredential {
        user_id: body.user_id,
        credential_id: passkey.cred_id().to_vec(),
        public_key: passkey.cred().to_vec(),
        counter: passkey.counter() as i64,
        aaguid: None, // Can extract from attestation if needed
        device_name: body.device_name,
        user_agent: None, // Can extract from request headers
    };
    
    database::passkey_credentials::insert(new_credential, &mut tx).await?;
    
    // Create activation link
    let new_activation_link = NewActivationLink::new(body.user_id);
    database::activation_links::insert(new_activation_link, &mut tx).await?;
    
    tx.commit().await.map_err(|_| -> Error {
        (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::SomethingWentWrong).into()
    })?;
    
    // Send activation email
    let link = new_activation_link.get_link(&state.config);
    state.send_activate_account(username.clone(), email.clone(), link).await?;
    
    // Clean up temporary data
    state.remove_passkey_registration(body.user_id).await;
    state.remove_pending_user(body.user_id).await;
    
    tracing::info!("Passkey registration successful for: {}", username);
    
    Ok(Json(TranslationKeyMessage {
        translation_key: TranslationKey::AccountCreated,
    }))
}
```

```rust
// src/routes/passkey_login.rs

use std::sync::Arc;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use webauthn_rs::prelude::*;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    AppState, database,
    models::{
        request::passkey_login_info::*,
        response::{Error, Token, TranslationKey},
        token::NewToken,
        token_claim::TokenClaim,
    },
};

pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/start", post(passkey_login_start))
        .route("/finish", post(passkey_login_finish))
        .with_state(state)
}

/// Start passkey login - generate challenge
async fn passkey_login_start(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyLoginStartRequest>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Passkey login start for: {}", body.username);
    
    // Verify captcha
    crate::utils::captcha::verify_turnstile(&body.captcha_token, &state.config.turnstile_secret)
        .await
        .map_err(|_| -> Error {
            (StatusCode::BAD_REQUEST, TranslationKey::CaptchaVerificationFailed).into()
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
    let passkeys: Vec<Passkey> = credentials.iter()
        .filter_map(|c| c.to_passkey().ok())
        .collect();
    
    // Generate challenge
    let webauthn = state.config.build_webauthn()
        .map_err(|_| -> Error {
            (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::InternalServerError).into()
        })?;
    
    let (request_challenge_response, passkey_authentication) = webauthn
        .start_passkey_authentication(&passkeys)
        .map_err(|e| -> Error {
            tracing::error!("WebAuthn challenge generation failed: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::InternalServerError).into()
        })?;
    
    // Store challenge state temporarily
    state.store_passkey_authentication(body.username.clone(), passkey_authentication).await;
    
    Ok(Json(PasskeyLoginStartResponse {
        request_options: request_challenge_response,
    }))
}

/// Finish passkey login - verify and issue token
async fn passkey_login_finish(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasskeyLoginFinishRequest>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Passkey login finish for: {}", body.username);
    
    // Retrieve stored challenge
    let passkey_authentication = state.get_passkey_authentication(&body.username).await
        .ok_or_else(|| -> Error {
            (StatusCode::BAD_REQUEST, TranslationKey::AuthenticationSessionExpired).into()
        })?;
    
    // Verify credential
    let webauthn = state.config.build_webauthn()
        .map_err(|_| -> Error {
            (StatusCode::INTERNAL_SERVER_ERROR, TranslationKey::InternalServerError).into()
        })?;
    
    let authentication_result = webauthn
        .finish_passkey_authentication(&body.credential, &passkey_authentication)
        .map_err(|e| -> Error {
            tracing::error!("Passkey authentication failed: {}", e);
            (StatusCode::UNAUTHORIZED, TranslationKey::PasskeyAuthenticationFailed).into()
        })?;
    
    // Get user
    let pool = state.get_database_pool();
    let user = database::users::filter_by_username(&body.username, pool).await?;
    
    // Update credential counter
    database::passkey_credentials::update_counter(
        authentication_result.cred_id(),
        authentication_result.counter() as i64,
        pool
    ).await?;
    
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
    
    // Clean up temporary data
    state.remove_passkey_authentication(&body.username).await;
    
    tracing::info!("Passkey login successful for: {}", body.username);
    
    Ok(Json(Token { token }))
}
```

### App State Updates

```rust
// src/app_state.rs - Add to existing AppState

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use webauthn_rs::prelude::*;
use uuid::Uuid;

pub struct AppState {
    // ... existing fields ...
    
    // Temporary storage for WebAuthn challenges
    passkey_registrations: Arc<RwLock<HashMap<Uuid, PasskeyRegistration>>>,
    passkey_authentications: Arc<RwLock<HashMap<String, PasskeyAuthentication>>>,
    pending_users: Arc<RwLock<HashMap<Uuid, (String, String)>>>, // user_id -> (username, email)
}

impl AppState {
    pub async fn store_passkey_registration(&self, user_id: Uuid, reg: PasskeyRegistration) {
        let mut map = self.passkey_registrations.write().await;
        map.insert(user_id, reg);
        
        // Set expiry (clean up after 5 minutes)
        let registrations = Arc::clone(&self.passkey_registrations);
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
            registrations.write().await.remove(&user_id);
        });
    }
    
    pub async fn get_passkey_registration(&self, user_id: Uuid) -> Option<PasskeyRegistration> {
        self.passkey_registrations.read().await.get(&user_id).cloned()
    }
    
    pub async fn remove_passkey_registration(&self, user_id: Uuid) {
        self.passkey_registrations.write().await.remove(&user_id);
    }
    
    pub async fn store_passkey_authentication(&self, username: String, auth: PasskeyAuthentication) {
        let mut map = self.passkey_authentications.write().await;
        map.insert(username.clone(), auth);
        
        // Set expiry
        let authentications = Arc::clone(&self.passkey_authentications);
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
            authentications.write().await.remove(&username);
        });
    }
    
    pub async fn get_passkey_authentication(&self, username: &str) -> Option<PasskeyAuthentication> {
        self.passkey_authentications.read().await.get(username).cloned()
    }
    
    pub async fn remove_passkey_authentication(&self, username: &str) {
        self.passkey_authentications.write().await.remove(username);
    }
    
    pub async fn store_pending_user(&self, user_id: Uuid, username: String, email: String) {
        let mut map = self.pending_users.write().await;
        map.insert(user_id, (username, email));
        
        // Set expiry
        let users = Arc::clone(&self.pending_users);
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
            users.write().await.remove(&user_id);
        });
    }
    
    pub async fn get_pending_user(&self, user_id: Uuid) -> Option<(String, String)> {
        self.pending_users.read().await.get(&user_id).cloned()
    }
    
    pub async fn remove_pending_user(&self, user_id: Uuid) {
        self.pending_users.write().await.remove(&user_id);
    }
}
```

## Frontend Implementation (Vue.js)

### WebAuthn Service

```typescript
// src/services/webauthn.ts

import { base64URLStringToBuffer, bufferToBase64URLString } from '@/utils/base64';

export interface PasskeySupport {
  available: boolean;
  platformAuthenticator: boolean;
}

export async function checkPasskeySupport(): Promise<PasskeySupport> {
  const available = window.PublicKeyCredential !== undefined;
  
  if (!available) {
    return { available: false, platformAuthenticator: false };
  }
  
  try {
    const platformAuthenticator = 
      await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();
    
    return { available: true, platformAuthenticator };
  } catch (error) {
    console.error('Error checking passkey support:', error);
    return { available: true, platformAuthenticator: false };
  }
}

export async function registerPasskey(
  creationOptions: any
): Promise<PublicKeyCredential> {
  // Convert base64url strings to ArrayBuffers
  const challenge = base64URLStringToBuffer(creationOptions.publicKey.challenge);
  const userId = base64URLStringToBuffer(creationOptions.publicKey.user.id);
  
  const publicKeyOptions: PublicKeyCredentialCreationOptions = {
    ...creationOptions.publicKey,
    challenge,
    user: {
      ...creationOptions.publicKey.user,
      id: userId,
    },
  };
  
  const credential = await navigator.credentials.create({
    publicKey: publicKeyOptions,
  }) as PublicKeyCredential;
  
  if (!credential) {
    throw new Error('Failed to create passkey');
  }
  
  return credential;
}

export async function authenticateWithPasskey(
  requestOptions: any
): Promise<PublicKeyCredential> {
  // Convert base64url strings to ArrayBuffers
  const challenge = base64URLStringToBuffer(requestOptions.publicKey.challenge);
  
  const allowCredentials = requestOptions.publicKey.allowCredentials?.map((cred: any) => ({
    ...cred,
    id: base64URLStringToBuffer(cred.id),
  }));
  
  const publicKeyOptions: PublicKeyCredentialRequestOptions = {
    ...requestOptions.publicKey,
    challenge,
    allowCredentials,
  };
  
  const credential = await navigator.credentials.get({
    publicKey: publicKeyOptions,
  }) as PublicKeyCredential;
  
  if (!credential) {
    throw new Error('Failed to authenticate with passkey');
  }
  
  return credential;
}

export function credentialToJSON(credential: PublicKeyCredential): any {
  const response = credential.response as AuthenticatorAttestationResponse;
  
  return {
    id: credential.id,
    rawId: bufferToBase64URLString(credential.rawId),
    type: credential.type,
    response: {
      clientDataJSON: bufferToBase64URLString(response.clientDataJSON),
      attestationObject: bufferToBase64URLString(response.attestationObject),
    },
  };
}

export function assertionToJSON(credential: PublicKeyCredential): any {
  const response = credential.response as AuthenticatorAssertionResponse;
  
  return {
    id: credential.id,
    rawId: bufferToBase64URLString(credential.rawId),
    type: credential.type,
    response: {
      clientDataJSON: bufferToBase64URLString(response.clientDataJSON),
      authenticatorData: bufferToBase64URLString(response.authenticatorData),
      signature: bufferToBase64URLString(response.signature),
      userHandle: response.userHandle 
        ? bufferToBase64URLString(response.userHandle) 
        : null,
    },
  };
}
```

```typescript
// src/utils/base64.ts

export function base64URLStringToBuffer(base64URLString: string): ArrayBuffer {
  // Convert base64url to base64
  const base64 = base64URLString
    .replace(/-/g, '+')
    .replace(/_/g, '/');
  
  // Decode base64
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  
  return bytes.buffer;
}

export function bufferToBase64URLString(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let binary = '';
  
  for (let i = 0; i < bytes.byteLength; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  
  // Convert to base64
  const base64 = btoa(binary);
  
  // Convert base64 to base64url
  return base64
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}
```

### Auth Service Updates

```typescript
// src/services/auth/index.ts - Add new methods

export const authService = {
  // ... existing methods ...
  
  async passkeyRegisterStart(values: {
    username: string;
    email: string;
    captchaToken: string;
  }): Promise<ServerResponse> {
    try {
      const response = await fetch(`${API_BASE_URL}/auth/passkey/register/start`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(values),
      });
      
      const data = await response.json();
      
      if (!response.ok) {
        return { status: ServerStatus.ERROR, data };
      }
      
      return { status: ServerStatus.NO_ERROR, data };
    } catch (error) {
      console.error('Passkey registration start failed:', error);
      return {
        status: ServerStatus.ERROR,
        data: { translation_key: 'SOMETHING_WENT_WRONG' },
      };
    }
  },
  
  async passkeyRegisterFinish(values: {
    userId: string;
    credential: any;
    deviceName?: string;
  }): Promise<ServerResponse> {
    try {
      const response = await fetch(`${API_BASE_URL}/auth/passkey/register/finish`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(values),
      });
      
      const data = await response.json();
      
      if (!response.ok) {
        return { status: ServerStatus.ERROR, data };
      }
      
      return { status: ServerStatus.NO_ERROR, data };
    } catch (error) {
      console.error('Passkey registration finish failed:', error);
      return {
        status: ServerStatus.ERROR,
        data: { translation_key: 'SOMETHING_WENT_WRONG' },
      };
    }
  },
  
  async passkeyLoginStart(values: {
    username: string;
    captchaToken: string;
  }): Promise<ServerResponse> {
    try {
      const response = await fetch(`${API_BASE_URL}/auth/passkey/login/start`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(values),
      });
      
      const data = await response.json();
      
      if (!response.ok) {
        return { status: ServerStatus.ERROR, data };
      }
      
      return { status: ServerStatus.NO_ERROR, data };
    } catch (error) {
      console.error('Passkey login start failed:', error);
      return {
        status: ServerStatus.ERROR,
        data: { translation_key: 'SOMETHING_WENT_WRONG' },
      };
    }
  },
  
  async passkeyLoginFinish(values: {
    username: string;
    credential: any;
  }): Promise<ServerResponse> {
    try {
      const response = await fetch(`${API_BASE_URL}/auth/passkey/login/finish`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(values),
      });
      
      const data = await response.json();
      
      if (!response.ok) {
        return { status: ServerStatus.ERROR, data };
      }
      
      return { status: ServerStatus.NO_ERROR, data };
    } catch (error) {
      console.error('Passkey login finish failed:', error);
      return {
        status: ServerStatus.ERROR,
        data: { translation_key: 'SOMETHING_WENT_WRONG' },
      };
    }
  },
};
```

## Testing

### Backend Tests

```rust
// tests/passkey_auth_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_passkey_registration_flow() {
        // TODO: Implement integration tests
    }
    
    #[tokio::test]
    async fn test_passkey_authentication_flow() {
        // TODO: Implement integration tests
    }
    
    #[tokio::test]
    async fn test_passkey_counter_validation() {
        // TODO: Test counter anti-replay protection
    }
}
```

### Frontend Tests

```typescript
// tests/passkey.spec.ts

describe('Passkey Authentication', () => {
  it('should detect passkey support', async () => {
    const support = await checkPasskeySupport();
    expect(support).toHaveProperty('available');
    expect(support).toHaveProperty('platformAuthenticator');
  });
  
  // TODO: Add more tests
});
```

## Deployment Checklist

- [ ] Run database migrations
- [ ] Update environment variables (RP_ID, RP_ORIGIN, RP_NAME)
- [ ] Test WebAuthn on production domain (must be HTTPS)
- [ ] Monitor error rates and performance
- [ ] Set up alerts for authentication failures
- [ ] Document rollback procedure

## Security Considerations

1. **HTTPS Required**: WebAuthn only works over HTTPS (except localhost for development)
2. **Origin Validation**: WebAuthn verifies the origin automatically
3. **Counter Validation**: Always check and update credential counters to prevent replay attacks
4. **Rate Limiting**: Implement rate limiting on all authentication endpoints
5. **Audit Logging**: Log all passkey operations for security monitoring
6. **Backup Authentication**: Always provide fallback options (password, email OTP)

## References

- [WebAuthn Specification](https://www.w3.org/TR/webauthn-2/)
- [webauthn-rs Documentation](https://docs.rs/webauthn-rs/)
- [MDN WebAuthn API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Authentication_API)
- [FIDO Alliance](https://fidoalliance.org/)
