use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tonic::{Response, Status, transport::Channel};
use uuid::Uuid;
use webauthn_rs::prelude::{PasskeyAuthentication, PasskeyRegistration};

use crate::{
    Config,
    grpc::email_service::service::{
        ActivateAccountRequest, ActivateAccountResponse, ForgotPasswordRequest,
        ForgotPasswordResponse, email_service_client::EmailServiceClient,
    },
};

/// Application state shared across all routes
///
/// Contains configuration and database connection pool
/// that can be accessed by route handlers
///
/// # Fields
/// * `config` - Application configuration settings
/// * `db` - PostgreSQL connection pool for async database operations
/// * `email_service` - A mutex for the EmailServiceClient GRPC
/// * `passkey_registrations` - Temporary storage for WebAuthn registration challenges
/// * `passkey_authentications` - Temporary storage for WebAuthn authentication challenges
/// * `pending_users` - Temporary storage for pending user registration data (user_id -> (username, email))
///
/// # Usage
/// ```rust
/// use axum::extract::State;
///
/// async fn handler(State(state): State<Arc<AppState>>) {
///     let mut conn = state.db.get().await?;
///     // Use connection...
/// }
/// ```
pub struct AppState {
    pub config: Config,
    db: PgPool,
    email_service: Mutex<EmailServiceClient<Channel>>,
    passkey_registrations: Arc<RwLock<HashMap<Uuid, PasskeyRegistration>>>,
    passkey_authentications: Arc<RwLock<HashMap<String, PasskeyAuthentication>>>,
    pending_users: Arc<RwLock<HashMap<Uuid, (String, String)>>>,
}

impl AppState {
    /// Creates a new AppState
    ///
    /// # Returns
    /// * `AppState` - the AppState that contains all the necessary configs
    pub fn new(config: Config, db: PgPool, email_service: EmailServiceClient<Channel>) -> Self {
        Self {
            config,
            db,
            email_service: Mutex::new(email_service),
            passkey_registrations: Arc::new(RwLock::new(HashMap::new())),
            passkey_authentications: Arc::new(RwLock::new(HashMap::new())),
            pending_users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Gets a reference to the database pool
    ///
    /// # Returns
    /// * `&PgPool` - A reference to the database pool
    ///
    /// # Example
    /// ```rust
    /// let pool = state.get_database_pool();
    /// // Use pool for database operations
    /// ```
    pub fn get_database_pool(&self) -> &PgPool {
        &self.db
    }

    /// Call the send_activate_account GRPC from the email-service
    ///
    /// # Arguments
    /// * `ActivateAccountRequest` - A request of type `ActivateAccountRequest`
    ///
    /// # Returns
    /// * `Ok(Response<ActivateAccountResponse>)` - A response of type `ActivateAccountResponse`
    /// * `Err(Status)` - A GRPC status
    pub async fn send_activate_account(
        &self,
        request: ActivateAccountRequest,
    ) -> Result<Response<ActivateAccountResponse>, Status> {
        self.email_service
            .lock()
            .await
            .send_activate_account(request)
            .await
    }

    /// Call the send_forgot_password GRPC from the email-service
    ///
    /// # Arguments
    /// * `ForgotPasswordRequest` - A request of type `ForgotPasswordRequest`
    ///
    /// # Returns
    /// * `Ok(Response<ForgotPasswordResponse>)` - A response of type `ForgotPasswordResponse`
    /// * `Err(Status)` - A GRPC status
    pub async fn send_forgot_password(
        &self,
        request: ForgotPasswordRequest,
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        self.email_service
            .lock()
            .await
            .send_forgot_password(request)
            .await
    }

    /// Store a passkey registration challenge temporarily (5 minute expiry)
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

    /// Retrieve and remove a passkey registration challenge
    pub async fn get_passkey_registration(&self, user_id: Uuid) -> Option<PasskeyRegistration> {
        self.passkey_registrations.write().await.remove(&user_id)
    }

    /// Store a passkey authentication challenge temporarily (5 minute expiry)
    pub async fn store_passkey_authentication(
        &self,
        username: String,
        auth: PasskeyAuthentication,
    ) {
        let mut map = self.passkey_authentications.write().await;
        map.insert(username.clone(), auth);

        // Set expiry
        let authentications = Arc::clone(&self.passkey_authentications);
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
            authentications.write().await.remove(&username);
        });
    }

    /// Retrieve and remove a passkey authentication challenge
    pub async fn get_passkey_authentication(
        &self,
        username: &str,
    ) -> Option<PasskeyAuthentication> {
        self.passkey_authentications.write().await.remove(username)
    }

    /// Store pending user registration data temporarily (5 minute expiry)
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

    /// Retrieve and remove pending user registration data
    pub async fn get_pending_user(&self, user_id: Uuid) -> Option<(String, String)> {
        self.pending_users.write().await.remove(&user_id)
    }
}
