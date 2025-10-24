use crate::{config::Config, routes::make_app};

mod app_state;
mod config;
mod database;
mod models;
mod routes;

pub use app_state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();

    // Bind TCP listener to the configured port
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.settings_http_port))
        .await
        .expect("Could not bind TcpListener.");

    // Create the Axum application with all routes and middleware
    let app = make_app(config).await.expect("Could not create app.");

    println!("🚀 Server started successfully");

    // Start serving HTTP requests
    axum::serve(listener, app)
        .await
        .expect("Could not serve axum server.");

    Ok(())
}
