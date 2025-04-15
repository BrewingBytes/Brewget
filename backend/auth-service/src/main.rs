use axum::{Router, http::Method, routing::get};
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};

use crate::config::app_config::AppConfig;

mod config;

#[tokio::main]
async fn main() {
    // Initialize env vars
    dotenv().ok();

    // Intialize the config from env vars
    let config = AppConfig::init();
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    // Build the app and routes
    let app = Router::new()
        .route("/health", get(health_check))
        .fallback(not_found)
        .layer(cors_layer);

    // Bind and serve the server
    println!("Server listening on {}", config.server.get_address());
    let listener = tokio::net::TcpListener::bind(config.server.get_address())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn not_found() -> &'static str {
    "Not Found"
}
