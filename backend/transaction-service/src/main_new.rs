use std::sync::Arc;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

mod app_state;
mod auth_guard;
mod config;
mod grpc;
mod health;
mod models;
mod wallet_db;
mod wallet_model;
mod wallet_routes;

use app_state::AppState;
use config::Config;
use grpc::auth_service::service::auth_service_client::AuthServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("🚀 Starting Transaction Service...");

    dotenv::dotenv().ok();

    let config = Config::init();
    tracing::info!("✅ Configuration loaded successfully");
    tracing::debug!("HTTP port: {}", config.transaction_http_port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.transaction_http_port))
        .await
        .expect("Could not bind TcpListener.");
    tracing::info!(
        "✅ HTTP listener bound to port {}",
        config.transaction_http_port
    );

    let postgres_url = format!(
        "postgres://{}:{}@{}/{}",
        config.pg_username, config.pg_password, config.pg_url, config.pg_database
    );

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await
        .expect("Unable to create database pool");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Unable to run migrations");

    println!("✅ Database migrations completed successfully");

    let auth_service_url = format!("http://{}:{}", config.auth_hostname, config.auth_grpc_port);

    tracing::info!("Connecting to auth service at {}", auth_service_url);
    let auth_service = AuthServiceClient::connect(auth_service_url)
        .await
        .expect("Failed to connect to auth service");

    tracing::info!("✅ Connected to auth service gRPC");

    let state = Arc::new(AppState::new(config.clone(), db, auth_service));

    let cors = HeaderValue::from_str(&config.cors_url)?;
    let cors = CorsLayer::new()
        .allow_origin(cors)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = Router::new()
        .nest("/health", health::get_router(state.clone()))
        .nest("/", wallet_routes::get_router(state.clone()))
        .with_state(state)
        .layer(cors);

    tracing::info!("✅ Routes and middleware configured");

    tracing::info!(
        "🚀 Server started successfully on port {}",
        listener.local_addr()?.port()
    );

    axum::serve(listener, router)
        .await
        .expect("Could not serve axum server.");

    Ok(())
}
