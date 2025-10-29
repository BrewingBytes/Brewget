use auth_service::{
    AppState, Config,
    grpc::auth_service::{AuthServiceImpl, service::auth_service_server::AuthServiceServer},
    routes::make_app,
};

#[tokio::main]
async fn main() {
    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("🚀 Starting Auth Service...");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize configuration from environment variables
    let config = Config::init();
    tracing::info!("✅ Configuration loaded successfully");
    tracing::debug!(
        "HTTP port: {}, gRPC port: {}",
        config.auth_http_port,
        config.auth_grpc_port
    );

    // Bind TCP listener to the configured HTTP port
    let http_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.auth_http_port))
        .await
        .expect("Could not bind TcpListener for HTTP.");
    tracing::info!("✅ HTTP listener bound to port {}", config.auth_http_port);

    // Bind gRPC server to the configured gRPC port
    let grpc_addr = format!("0.0.0.0:{}", config.auth_grpc_port)
        .parse()
        .expect("Invalid gRPC address");
    tracing::info!("✅ gRPC address configured: {}", grpc_addr);

    // Create the Axum application with all routes and middleware
    let app = make_app(config.clone())
        .await
        .expect("Could not create app.");
    tracing::info!("✅ HTTP routes configured");

    tracing::info!("🚀 HTTP Server started on port {}", config.auth_http_port);
    tracing::info!("🚀 gRPC Server starting on port {}", config.auth_grpc_port);

    // Spawn HTTP server
    let http_server = tokio::spawn(async move {
        tracing::info!("📡 HTTP server accepting connections");
        axum::serve(http_listener, app)
            .await
            .expect("Could not serve axum server.");
    });

    // Start gRPC server
    let grpc_server = tokio::spawn(async move {
        // Create state for gRPC service (we need to recreate it as app consumed the first one)
        let grpc_config = Config::init();

        use auth_service::grpc::email_service::service::email_service_client::EmailServiceClient;
        use sqlx::postgres::PgPoolOptions;

        tracing::debug!("Creating database connection pool for gRPC service");
        let postgres_url = format!(
            "postgres://{}:{}@{}/{}",
            grpc_config.pg_username,
            grpc_config.pg_password,
            grpc_config.pg_url,
            grpc_config.pg_database
        );
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&postgres_url)
            .await
            .expect("Unable to create database pool for gRPC");
        tracing::info!("✅ Database pool created for gRPC service");

        tracing::debug!(
            "Connecting to email service at {}:{}",
            grpc_config.email_hostname,
            grpc_config.email_grpc_port
        );
        let email_service = EmailServiceClient::connect(format!(
            "{}:{}",
            grpc_config.email_hostname, grpc_config.email_grpc_port
        ))
        .await
        .expect("Could not connect to email service");
        tracing::info!("✅ Email service client connected");

        let state = std::sync::Arc::new(AppState::new(grpc_config, db, email_service));

        let auth_service = AuthServiceImpl::new(state);
        tracing::info!("✅ gRPC service initialized");

        tracing::info!("📡 gRPC server accepting connections");
        tonic::transport::Server::builder()
            .add_service(AuthServiceServer::new(auth_service))
            .serve(grpc_addr)
            .await
            .expect("Could not serve gRPC server");
    });

    // Wait for both servers
    tracing::info!("✅ Both servers are running");
    tokio::try_join!(http_server, grpc_server).expect("Server error");
}
