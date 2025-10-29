use settings_service::{Config, routes::make_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("🚀 Starting Settings Service...");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();
    tracing::info!("✅ Configuration loaded successfully");
    tracing::debug!("HTTP port: {}", config.settings_http_port);
    tracing::debug!(
        "Auth service: {}:{}",
        config.auth_hostname,
        config.auth_grpc_port
    );

    // Bind TCP listener to the configured port
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.settings_http_port))
        .await
        .expect("Could not bind TcpListener.");
    tracing::info!(
        "✅ HTTP listener bound to port {}",
        config.settings_http_port
    );

    // Create the Axum application with all routes and middleware
    let app = make_app(config).await.expect("Could not create app.");
    tracing::info!("✅ Routes and middleware configured");

    tracing::info!(
        "🚀 Server started successfully on port {}",
        listener.local_addr()?.port()
    );
    tracing::info!("📡 Server accepting connections");

    // Start serving HTTP requests
    axum::serve(listener, app)
        .await
        .expect("Could not serve axum server.");

    Ok(())
}
