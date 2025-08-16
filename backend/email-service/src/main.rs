use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};
use tonic::transport::Server;

use crate::{
    config::Config,
    service::{Service, service::email_service_server::EmailServiceServer},
};

mod config;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = Config::init();

    let creds = Credentials::new(config.smtp_username, config.smtp_password);
    let _mailer = SmtpTransport::starttls_relay(&config.smtp_relay)?
        .credentials(creds)
        .build();

    let addr = "[::1]:8082".parse()?;
    let service = Service::default();

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(EmailServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
