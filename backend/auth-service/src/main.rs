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
use grpc::auth_service::{AuthServiceImpl, service::auth_service_server::AuthServiceServer};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();

    // Bind TCP listener to the configured HTTP port
    let http_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.auth_http_port))
        .await
        .expect("Could not bind TcpListener for HTTP.");

    // Bind gRPC server to the configured gRPC port
    let grpc_addr = format!("0.0.0.0:{}", config.auth_grpc_port)
        .parse()
        .expect("Invalid gRPC address");

    // Create the Axum application with all routes and middleware
    let app = make_app(config.clone()).await.expect("Could not create app.");

    println!("🚀 HTTP Server started on port {}", config.auth_http_port);
    println!("🚀 gRPC Server started on port {}", config.auth_grpc_port);

    // Spawn HTTP server
    let http_server = tokio::spawn(async move {
        axum::serve(http_listener, app)
            .await
            .expect("Could not serve axum server.");
    });

    // Start gRPC server
    let grpc_server = tokio::spawn(async move {
        // Create state for gRPC service (we need to recreate it as app consumed the first one)
        let grpc_config = Config::init();
        
        use diesel_async::{
            AsyncPgConnection,
            pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
        };
        use grpc::email_service::service::email_service_client::EmailServiceClient;
        
        let postgres_url = format!(
            "postgres://{}:{}@{}/{}",
            grpc_config.pg_username, grpc_config.pg_password, grpc_config.pg_url, grpc_config.pg_database
        );
        let db = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&postgres_url);
        let db = Pool::builder(db)
            .build()
            .expect("Unable to create new db pool for gRPC");
            
        let email_service = EmailServiceClient::connect(format!(
            "{}:{}",
            grpc_config.email_hostname, grpc_config.email_grpc_port
        ))
        .await
        .expect("Could not connect to email service");
        
        let state = std::sync::Arc::new(AppState::new(grpc_config, db, email_service));
        
        let auth_service = AuthServiceImpl::new(state);
        
        tonic::transport::Server::builder()
            .add_service(AuthServiceServer::new(auth_service))
            .serve(grpc_addr)
            .await
            .expect("Could not serve gRPC server");
    });

    // Wait for both servers
    tokio::try_join!(http_server, grpc_server).expect("Server error");
}
