use tonic::transport::Server;

use crate::{
    config::Config,
    service::{Service, email_service::email_service_server::EmailServiceServer},
};

mod config;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = Config::init();

    let addr = format!("0.0.0.0:{}", config.email_grpc_port).parse()?;
    let service = Service::new(config.into())?;

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(EmailServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
