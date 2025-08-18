use deadpool::managed::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use tokio::sync::Mutex;
use tonic::{Response, Status, transport::Channel};

use crate::{
    Config,
    grpc::email_service::service::{
        ActivateAccountRequest, ActivateAccountResponse, email_service_client::EmailServiceClient,
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
    db: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    email_service: Mutex<EmailServiceClient<Channel>>,
}

impl AppState {
    /// Creates a new AppState
    ///
    /// # Returns
    /// * `AppState` - the AppState that contains all the necessary configs
    pub fn new(
        config: Config,
        db: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
        email_service: EmailServiceClient<Channel>,
    ) -> Self {
        Self {
            config,
            db,
            email_service: Mutex::new(email_service),
        }
    }

    /// Gets a connection from the database pool
    ///
    /// # Returns
    /// * `Ok(Object)` - A connection from the pool
    /// * `Err(PoolError)` - If connection acquisition fails
    ///
    /// # Example
    /// ```rust
    /// let conn = state.get_database_connection().await?;
    /// // Use connection for database operations
    /// ```
    ///
    /// # Errors
    /// Returns error if:
    /// * Pool is exhausted (too many connections)
    /// * Connection establishment fails
    /// * Database is unreachable
    pub async fn get_database_connection(
        &self,
    ) -> Result<
        deadpool::managed::Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
        deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>,
    > {
        self.db.get().await
    }

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
}
