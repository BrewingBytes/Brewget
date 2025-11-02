use moka::future::Cache;
use sqlx::PgPool;
use std::time::Duration;
use tokio::sync::Mutex;
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
/// * `passkey_registrations` - TTL cache for WebAuthn registration challenges (5 minute expiry)
/// * `passkey_authentications` - TTL cache for WebAuthn authentication challenges (5 minute expiry)
/// * `pending_users` - TTL cache for pending user registration data (5 minute expiry)
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
    passkey_registrations: Cache<Uuid, PasskeyRegistration>,
    passkey_authentications: Cache<String, PasskeyAuthentication>,
    pending_users: Cache<Uuid, (String, String)>,
}

impl AppState {
    /// Creates a new AppState
    ///
    /// # Returns
    /// * `AppState` - the AppState that contains all the necessary configs
    pub fn new(config: Config, db: PgPool, email_service: EmailServiceClient<Channel>) -> Self {
        // Create caches with 5 minute TTL for WebAuthn challenges
        let passkey_registrations = Cache::builder()
            .time_to_live(Duration::from_secs(300))
            .build();

        let passkey_authentications = Cache::builder()
            .time_to_live(Duration::from_secs(300))
            .build();

        let pending_users = Cache::builder()
            .time_to_live(Duration::from_secs(300))
            .build();

        Self {
            config,
            db,
            email_service: Mutex::new(email_service),
            passkey_registrations,
            passkey_authentications,
            pending_users,
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
        self.passkey_registrations.insert(user_id, reg).await;
    }

    /// Retrieve and remove a passkey registration challenge
    pub async fn get_passkey_registration(&self, user_id: Uuid) -> Option<PasskeyRegistration> {
        self.passkey_registrations.remove(&user_id).await
    }

    /// Store a passkey authentication challenge temporarily (5 minute expiry)
    pub async fn store_passkey_authentication(
        &self,
        username: String,
        auth: PasskeyAuthentication,
    ) {
        self.passkey_authentications.insert(username, auth).await;
    }

    /// Retrieve and remove a passkey authentication challenge
    pub async fn get_passkey_authentication(
        &self,
        username: &str,
    ) -> Option<PasskeyAuthentication> {
        self.passkey_authentications.remove(username).await
    }

    /// Store pending user registration data temporarily (5 minute expiry)
    pub async fn store_pending_user(&self, user_id: Uuid, username: String, email: String) {
        self.pending_users.insert(user_id, (username, email)).await;
    }

    /// Retrieve and remove pending user registration data
    pub async fn get_pending_user(&self, user_id: Uuid) -> Option<(String, String)> {
        self.pending_users.remove(&user_id).await
    }
}
