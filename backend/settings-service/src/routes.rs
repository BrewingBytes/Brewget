mod health;
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

use crate::{AppState, config::Config};

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

    // Create all the GRPCs Clients
    // We don't use any for now

    let state = Arc::new(AppState::new(config, db));

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
