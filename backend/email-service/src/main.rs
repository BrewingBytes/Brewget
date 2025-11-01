use tonic::transport::Server;

use crate::{
    config::Config,
    service::{Service, email_service::email_service_server::EmailServiceServer},
};

mod config;
mod health;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing/logging with structured output
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "🚀 Starting Email Service"
    );

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();
    tracing::info!("✅ Configuration loaded successfully");
    tracing::debug!(
        http_port = config.email_http_port,
        grpc_port = config.email_grpc_port,
        smtp_relay = %config.smtp_relay,
        smtp_email = %config.smtp_email,
        "Configuration details"
    );

    // Parse the gRPC server address
    let grpc_addr: std::net::SocketAddr = format!("0.0.0.0:{}", config.email_grpc_port).parse()?;
    tracing::info!(
        grpc_port = config.email_grpc_port,
        grpc_addr = %grpc_addr,
        "✅ gRPC listener configured"
    );

    // Create the email service instance with SMTP configuration
    let service = Service::new(config.clone().into())?;
    tracing::info!("✅ Created the mailer service");

    // Create main router with health endpoint
    let app = axum::Router::new().nest("/health", health::get_router());

    // Spawn HTTP server for health checks
    let http_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.email_http_port))
            .await
            .expect("Could not bind TcpListener for HTTP.");
    tracing::info!(
        http_port = config.email_http_port,
        "✅ HTTP listener bound successfully"
    );

    let http_server = tokio::spawn(async move {
        axum::serve(http_listener, app)
            .await
            .expect("Could not start http app.");
    });

    // Start the gRPC server
    let grpc_server = tokio::spawn(async move {
        Server::builder()
            .add_service(EmailServiceServer::new(service))
            .serve(grpc_addr)
            .await
            .expect("Could not start grpc server.");
    });

    tracing::info!(
        http_port = config.email_http_port,
        grpc_port = config.email_grpc_port,
        "🚀 Starting HTTP and gRPC servers"
    );

    // Wait for both servers
    tracing::info!("✅ Both servers are running and ready to accept requests");
    tokio::try_join!(http_server, grpc_server).expect("Server error");

    Ok(())
}
