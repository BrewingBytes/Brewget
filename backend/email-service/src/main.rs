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
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();

    // Parse the gRPC server address
    let grpc_addr = format!("0.0.0.0:{}", config.email_grpc_port).parse()?;
    
    // HTTP health check will run on gRPC port + 1000 (e.g., 9001 -> 10001)
    let http_port = config.email_grpc_port + 1000;
    let http_addr = format!("0.0.0.0:{}", http_port).parse()?;

    // Create the email service instance with SMTP configuration
    let service = Service::new(config.into())?;

    println!("gRPC Server listening on {}", grpc_addr);
    println!("HTTP Health endpoint listening on {}", http_addr);

    // Create health router
    let health_router = health::get_router();

    // Spawn HTTP server for health checks
    let http_server = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(http_addr).await.unwrap();
        axum::serve(listener, health_router).await.unwrap();
    });

    // Start the gRPC server
    let grpc_server = Server::builder()
        .add_service(EmailServiceServer::new(service))
        .serve(grpc_addr);

    // Run both servers concurrently
    tokio::try_join!(
        async { grpc_server.await.map_err(|e| e.into()) },
        async { http_server.await.map_err(|e| e.into()) }
    )?;

    Ok(())
}
