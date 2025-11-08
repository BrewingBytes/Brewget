use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::get,
};

use crate::AppState;
use shared_types::{DatabaseConnection, Health, HealthStatus};

pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(health_check))
        .with_state(state)
}

async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::info!("GET /health - Health check requested");

    let db_status = match sqlx::query("SELECT 1")
        .fetch_one(state.get_database_pool())
        .await
    {
        Ok(_) => {
            tracing::debug!("Database connection is healthy");
            DatabaseConnection::Connected
        }
        Err(e) => {
            tracing::error!("Database connection failed: {}", e);
            DatabaseConnection::Disconnected
        }
    };

    let health = Health {
        status: HealthStatus::Healthy,
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
    };

    tracing::info!("Health check completed: {:?}", health.status);
    Json(health)
}
