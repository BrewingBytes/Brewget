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
    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("🚀 Starting Email Service...");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();
    tracing::info!("✅ Configuration loaded successfully");
    tracing::debug!(
        "HTTP port: {}, gRPC port: {}",
        config.email_http_port,
        config.email_grpc_port
    );

    // Parse the gRPC server address
    let grpc_addr: std::net::SocketAddr = format!("0.0.0.0:{}", config.email_grpc_port).parse()?;
    tracing::info!("✅ GRPC listener bound to port {}", config.email_grpc_port);

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
    tracing::info!("✅ HTTP listener bound to port {}", config.email_http_port);

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

    tracing::info!("🚀 HTTP Server started on port {}", config.email_http_port);
    tracing::info!("🚀 gRPC Server starting on port {}", config.email_grpc_port);

    // Wait for both servers
    tracing::info!("✅ Both servers are running");
    tokio::try_join!(http_server, grpc_server).expect("Server error");

    Ok(())
}
