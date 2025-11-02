#!/bin/bash
# Setup script for creating the notification service
# Run this script from the repository root directory

set -e

echo "Creating notification service directory structure..."

# Create main directory
mkdir -p backend/notification-service/src/clients

# Create Cargo.toml
cat > backend/notification-service/Cargo.toml << 'EOF'
[package]
name = "notification-service"
version = "0.0.1"
edition = "2024"
build = "build.rs"

[dependencies]
axum = "0.8.6"
chrono = { version = "0.4.42", features = ["serde"] }
dotenv = "0.15.0"
prost = "0.14.1"
serde = { version = "1.0.228", features = ["derive"] }
shared-types = { path = "../shared-types" }
tokio = { version = "1.48.0", features = ["full"] }
tokio-cron-scheduler = "0.14.0"
tonic = "0.14.2"
tonic-prost = "0.14.2"
tracing = "0.1.41"
tracing-subscriber = { version = "0.3.20", features = ["env-filter"] }
uuid = { version = "1.18.0", features = ["serde", "v4"] }
sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "postgres",
    "uuid",
    "chrono",
] }

[build-dependencies]
tonic-prost-build = "0.14.2"
EOF

# Create build.rs
cat > backend/notification-service/build.rs << 'EOF'
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .compile_protos(
            &[
                "../proto/notification_service.proto",
                "../proto/email_service.proto",
            ],
            &["../proto"],
        )?;

    Ok(())
}
EOF

# Create Dockerfile
cat > backend/notification-service/Dockerfile << 'EOF'
# Runtime stage
FROM debian:bookworm-slim

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=base-image /build/target/release/notification-service .

LABEL org.opencontainers.image.source=https://github.com/BrewingBytes/BrewGet
CMD [ "./notification-service" ]
EOF

# Create config.rs
cat > backend/notification-service/src/config.rs << 'EOF'
use std::env::var;

/// Application configuration loaded from environment variables
#[derive(Clone)]
pub struct Config {
    pub notification_http_port: u32,
    pub notification_grpc_port: u32,
    pub email_grpc_hostname: String,
    pub settings_grpc_hostname: String,
    pub pg_url: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_database: String,
}

impl Config {
    pub fn init() -> Self {
        let notification_http_port = var("NOTIFICATION_HTTP_PORT")
            .map(|val| val.parse::<u32>())
            .expect("NOTIFICATION_HTTP_PORT must be provided.")
            .expect("NOTIFICATION_HTTP_PORT must be a valid u32.");
        let notification_grpc_port = var("NOTIFICATION_GRPC_PORT")
            .map(|val| val.parse::<u32>())
            .expect("NOTIFICATION_GRPC_PORT must be provided.")
            .expect("NOTIFICATION_GRPC_PORT must be a valid u32.");
        let email_grpc_hostname = var("EMAIL_GRPC_HOSTNAME")
            .expect("EMAIL_GRPC_HOSTNAME must be provided.");
        let settings_grpc_hostname = var("SETTINGS_GRPC_HOSTNAME")
            .expect("SETTINGS_GRPC_HOSTNAME must be provided.");
        let pg_url = var("PG_URL").expect("PG_URL must be provided.");
        let pg_username = var("PG_USERNAME").expect("PG_USERNAME must be provided.");
        let pg_password = var("PG_PASSWORD").expect("PG_PASSWORD must be provided.");
        let pg_database = var("NOTIFICATION_PG_DATABASE")
            .or_else(|_| var("PG_DATABASE"))
            .expect("NOTIFICATION_PG_DATABASE or PG_DATABASE must be provided.");

        Self {
            notification_http_port,
            notification_grpc_port,
            email_grpc_hostname,
            settings_grpc_hostname,
            pg_url,
            pg_username,
            pg_password,
            pg_database,
        }
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}/{}",
            self.pg_username, self.pg_password, self.pg_url, self.pg_database
        )
    }
}
EOF

# Create health.rs
cat > backend/notification-service/src/health.rs << 'EOF'
use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    service: String,
    version: String,
    status: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "notification-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: "healthy".to_string(),
    })
}

pub fn get_router() -> Router {
    Router::new().route("/", get(health))
}
EOF

# Create clients/mod.rs
cat > backend/notification-service/src/clients/mod.rs << 'EOF'
pub mod email_client;
EOF

# Create clients/email_client.rs
cat > backend/notification-service/src/clients/email_client.rs << 'EOF'
use tonic::transport::Channel;
use tonic::{Request, Status};

pub mod email_service {
    tonic::include_proto!("email_service");
}

use email_service::{
    email_service_client::EmailServiceClient,
    ActivateAccountRequest, ForgotPasswordRequest,
};

pub struct EmailClient {
    client: EmailServiceClient<Channel>,
}

impl EmailClient {
    pub async fn new(email_grpc_hostname: String) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = format!("http://{}", email_grpc_hostname);
        let client = EmailServiceClient::connect(addr).await?;
        Ok(Self { client })
    }

    pub async fn send_reminder_email(
        &mut self,
        username: String,
        email: String,
        link: String,
    ) -> Result<bool, Status> {
        let request = Request::new(ActivateAccountRequest {
            username,
            email,
            link,
        });

        let response = self.client.send_activate_account(request).await?;
        Ok(response.into_inner().success)
    }

    pub async fn send_budget_warning_email(
        &mut self,
        username: String,
        email: String,
        link: String,
    ) -> Result<bool, Status> {
        let request = Request::new(ForgotPasswordRequest {
            username,
            email,
            link,
        });

        let response = self.client.send_forgot_password(request).await?;
        Ok(response.into_inner().success)
    }
}
EOF

# Create service.rs
cat > backend/notification-service/src/service.rs << 'EOF'
use std::sync::Arc;
use tonic::{Request, Response, Result, Status};
use tracing::{error, info, instrument};

use crate::clients::email_client::EmailClient;
use crate::config::Config;
use crate::service::notification_service::{
    SendNotificationRequest, SendNotificationResponse,
    notification_service_server::NotificationService,
};

pub mod notification_service {
    tonic::include_proto!("notification_service");
}

pub struct Service {
    config: Arc<Config>,
}

impl Service {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    async fn send_email_notification(
        &self,
        user_id: &str,
        notification_type: &str,
        subject: &str,
        message: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut email_client = EmailClient::new(self.config.email_grpc_hostname.clone()).await?;

        let link = format!("https://brewget.app/notifications/{}", user_id);

        match notification_type {
            "daily_reminder" => {
                email_client
                    .send_reminder_email(user_id.to_string(), message.to_string(), link)
                    .await?;
            }
            "budget_warning" => {
                email_client
                    .send_budget_warning_email(user_id.to_string(), message.to_string(), link)
                    .await?;
            }
            _ => {
                return Err(format!("Unknown notification type: {}", notification_type).into());
            }
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl NotificationService for Service {
    #[instrument(skip(self, request))]
    async fn send_notification(
        &self,
        request: Request<SendNotificationRequest>,
    ) -> Result<Response<SendNotificationResponse>, Status> {
        let req = request.into_inner();
        info!(
            user_id = %req.user_id,
            notification_type = %req.notification_type,
            "Received request to send notification"
        );

        match self
            .send_email_notification(&req.user_id, &req.notification_type, &req.subject, &req.message)
            .await
        {
            Ok(_) => {
                info!(
                    user_id = %req.user_id,
                    notification_type = %req.notification_type,
                    "Notification sent successfully"
                );
                Ok(Response::new(SendNotificationResponse {
                    success: true,
                    message: "Notification sent successfully".to_string(),
                }))
            }
            Err(e) => {
                error!(
                    user_id = %req.user_id,
                    notification_type = %req.notification_type,
                    error = %e,
                    "Failed to send notification"
                );
                Ok(Response::new(SendNotificationResponse {
                    success: false,
                    message: format!("Failed to send notification: {}", e),
                }))
            }
        }
    }
}
EOF

# Create main.rs
cat > backend/notification-service/src/main.rs << 'EOF'
use std::sync::Arc;
use tonic::transport::Server;

use crate::{
    config::Config,
    service::{Service, notification_service::notification_service_server::NotificationServiceServer},
};

mod clients;
mod config;
mod health;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("🚀 Starting Notification Service...");

    dotenv::dotenv().ok();

    let config = Arc::new(Config::init());
    tracing::info!("✅ Configuration loaded successfully");
    tracing::debug!(
        http_port = config.notification_http_port,
        grpc_port = config.notification_grpc_port,
        "Configuration details"
    );

    let grpc_addr: std::net::SocketAddr =
        format!("0.0.0.0:{}", config.notification_grpc_port).parse()?;
    tracing::info!(
        grpc_port = config.notification_grpc_port,
        grpc_addr = %grpc_addr,
        "✅ gRPC listener configured"
    );

    let service = Service::new(config.clone());
    tracing::info!("✅ Created the notification service");

    let app = axum::Router::new().nest("/health", health::get_router());

    let http_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.notification_http_port))
            .await
            .expect("Could not bind TcpListener for HTTP.");
    tracing::info!(
        http_port = config.notification_http_port,
        "✅ HTTP listener bound successfully"
    );

    let http_server = tokio::spawn(async move {
        axum::serve(http_listener, app)
            .await
            .expect("Could not start http app.");
    });

    let grpc_server = tokio::spawn(async move {
        Server::builder()
            .add_service(NotificationServiceServer::new(service))
            .serve(grpc_addr)
            .await
            .expect("Could not start grpc server.");
    });

    tracing::info!(
        http_port = config.notification_http_port,
        grpc_port = config.notification_grpc_port,
        "🚀 Starting HTTP and gRPC servers"
    );

    tracing::info!("✅ Both servers are running and ready to accept requests");
    tokio::try_join!(http_server, grpc_server).expect("Server error");

    Ok(())
}
EOF

echo "✅ Notification service created successfully!"
echo ""
echo "Next steps:"
echo "1. Install protobuf-compiler if not already installed"
echo "2. Run 'cargo build' from the backend directory"
echo "3. Set up environment variables in .env file"
echo "4. Run the service with 'cargo run -p notification-service'"
