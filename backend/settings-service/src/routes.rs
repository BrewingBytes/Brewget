mod health;
mod middlewares;
mod user;

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

use crate::{AppState, config::Config, grpc::auth_service::service::auth_service_client::AuthServiceClient};

pub async fn make_app(config: Config) -> Result<Router, Box<dyn std::error::Error>> {
    let cors = HeaderValue::from_str(&config.cors_url)?;
    let postgres_url = format!(
        "postgres://{}:{}@{}/{}",
        config.pg_username, config.pg_password, config.pg_url, config.pg_database
    );

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await
        .expect("Unable to create database pool");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Unable to run migrations");

    println!("✅ Database migrations completed successfully");

    // Create gRPC client connection to auth service
    let auth_service_url = format!(
        "http://{}:{}",
        config.auth_hostname, config.auth_grpc_port
    );
    
    tracing::info!("Connecting to auth service at {}", auth_service_url);
    let auth_service = AuthServiceClient::connect(auth_service_url)
        .await
        .expect("Failed to connect to auth service");
    
    tracing::info!("✅ Connected to auth service gRPC");

    let state = Arc::new(AppState::new(config, db, auth_service));

    let cors = CorsLayer::new()
        .allow_origin(cors)
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = Router::new()
        .nest("/health", health::get_router(state.clone()))
        .nest("/user", user::get_router(state.clone()))
        .with_state(state)
        .layer(cors);
    Ok(router)
}
