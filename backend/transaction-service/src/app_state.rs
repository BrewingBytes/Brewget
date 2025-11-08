use sqlx::PgPool;

use crate::{Config, grpc::auth_service::service::auth_service_client::AuthServiceClient};

/// Application state shared across all routes
///
/// Contains configuration and database connection pool
/// that can be accessed by route handlers
///
/// # Fields
/// * `config` - Application configuration settings
/// * `db` - PostgreSQL connection pool for async database operations
/// * `auth_service` - gRPC client for authentication service (cloneable for concurrent access)
pub struct AppState {
    pub config: Config,
    db: PgPool,
    auth_service: AuthServiceClient<tonic::transport::Channel>,
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
            auth_service,
        }
    }

    /// Gets a reference to the database pool
    ///
    /// # Returns
    /// * `&PgPool` - A reference to the database pool
    pub fn get_database_pool(&self) -> &PgPool {
        &self.db
    }

    /// Gets a cloned auth service client for concurrent access
    ///
    /// # Returns
    /// * `AuthServiceClient<Channel>` - A cloned auth service client
    pub fn get_auth_service(&self) -> AuthServiceClient<tonic::transport::Channel> {
        self.auth_service.clone()
    }
}
