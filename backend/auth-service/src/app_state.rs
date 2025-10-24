use sqlx::PgPool;
use tokio::sync::Mutex;
use tonic::{Response, Status, transport::Channel};

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
}

impl AppState {
    /// Creates a new AppState
    ///
    /// # Returns
    /// * `AppState` - the AppState that contains all the necessary configs
    pub fn new(
        config: Config,
        db: PgPool,
        email_service: EmailServiceClient<Channel>,
    ) -> Self {
        Self {
            config,
            db,
            email_service: Mutex::new(email_service),
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
}
