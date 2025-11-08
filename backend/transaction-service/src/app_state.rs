use sqlx::PgPool;
use tokio::sync::Mutex;

use crate::{Config, grpc::auth_service::service::auth_service_client::AuthServiceClient};

/// Application state shared across all routes
///
/// Contains configuration and database connection pool
/// that can be accessed by route handlers
///
/// # Fields
/// * `config` - Application configuration settings
/// * `db` - PostgreSQL connection pool for async database operations
/// * `auth_service` - A mutex for the AuthServiceClient gRPC
pub struct AppState {
    pub config: Config,
    db: PgPool,
    auth_service: Mutex<AuthServiceClient<tonic::transport::Channel>>,
}

impl AppState {
    /// Creates a new AppState
    ///
    /// # Returns
    /// * `AppState` - the AppState that contains all the necessary configs
    pub fn new(
        config: Config,
        db: PgPool,
        auth_service: AuthServiceClient<tonic::transport::Channel>,
    ) -> Self {
        Self {
            config,
            db,
            auth_service: Mutex::new(auth_service),
        }
    }

    /// Gets a reference to the database pool
    ///
    /// # Returns
    /// * `&PgPool` - A reference to the database pool
    pub fn get_database_pool(&self) -> &PgPool {
        &self.db
    }

    /// Gets a lock on the auth service client
    ///
    /// # Returns
    /// * `MutexGuard<AuthServiceClient<Channel>>` - A locked auth service client
    pub async fn get_auth_service(
        &self,
    ) -> tokio::sync::MutexGuard<'_, AuthServiceClient<tonic::transport::Channel>> {
        self.auth_service.lock().await
    }
}
