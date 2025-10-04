use tonic::transport::Server;

use crate::{
    config::Config,
    service::{Service, email_service::email_service_server::EmailServiceServer},
};

mod config;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();

    // Parse the gRPC server address
    let addr = format!("0.0.0.0:{}", config.email_grpc_port).parse()?;

    // Create the email service instance with SMTP configuration
    let service = Service::new(config.into())?;

    println!("Server listening on {}", addr);

    // Start the gRPC server
    Server::builder()
        .add_service(EmailServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
