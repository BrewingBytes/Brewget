mod app_state;
mod config;
mod database;
mod grpc;
mod models;
mod routes;
mod utils;

pub use app_state::AppState;
pub use config::Config;

use crate::routes::make_app;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();

    // Bind TCP listener to the configured port
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.auth_http_port))
        .await
        .expect("Could not bind TcpListener.");

    // Create the Axum application with all routes and middleware
    let app = make_app(config).await.expect("Could not create app.");

    println!("🚀 Server started successfully");

    // Start serving HTTP requests
    axum::serve(listener, app)
        .await
        .expect("Could not serve axum server.");
}
