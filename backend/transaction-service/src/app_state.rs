use sqlx::PgPool;
use tokio::sync::Mutex;

use crate::{Config, grpc::auth_service::service::auth_service_client::AuthServiceClient};

/// Application state shared across all routes
pub struct AppState {
    pub config: Config,
    db: PgPool,
    auth_service: Mutex<AuthServiceClient<tonic::transport::Channel>>,
}

impl AppState {
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

    pub fn get_database_pool(&self) -> &PgPool {
        &self.db
    }

    pub async fn get_auth_service(
        &self,
    ) -> tokio::sync::MutexGuard<'_, AuthServiceClient<tonic::transport::Channel>> {
        self.auth_service.lock().await
    }
}
