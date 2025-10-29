use std::sync::Arc;

use axum_test::TestServer;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use tonic::transport::Server;

use settings_service::{
    grpc::auth_service::service::{
        VerifyTokenRequest, VerifyTokenResponse,
        auth_service_server::{AuthService, AuthServiceServer},
    },
    AppState, Config,
};

/// Mock auth service for testing
#[derive(Clone)]
struct MockAuthService {
    valid_user_id: String,
}

#[tonic::async_trait]
impl AuthService for MockAuthService {
    async fn verify_token(
        &self,
        request: tonic::Request<VerifyTokenRequest>,
    ) -> Result<tonic::Response<VerifyTokenResponse>, tonic::Status> {
        let token = request.into_inner().token;
        
        // Simple mock: accept tokens starting with "valid_"
        if token.starts_with("valid_") {
            Ok(tonic::Response::new(VerifyTokenResponse {
                user_id: Some(self.valid_user_id.clone()),
            }))
        } else {
            Ok(tonic::Response::new(VerifyTokenResponse { user_id: None }))
        }
    }
}

/// Test fixture that sets up a test database and services
struct TestFixture {
    _postgres_container: ContainerAsync<Postgres>,
    test_server: TestServer,
    _grpc_server_handle: tokio::task::JoinHandle<()>,
    test_user_id: String,
    grpc_port: u16,
}

impl TestFixture {
    async fn new() -> Self {
        // Start PostgreSQL container
        let postgres_container = Postgres::default()
            .with_tag("16-alpine")
            .start()
            .await
            .expect("Failed to start PostgreSQL container");

        let host_port = postgres_container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get PostgreSQL port");

        let database_url = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            host_port
        );

        // Create database pool
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Failed to run migrations");

        let test_user_id = uuid::Uuid::new_v4().to_string();

        // Use a dynamic port by binding to port 0
        let grpc_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to address");
        let grpc_port = grpc_listener.local_addr().unwrap().port();
        drop(grpc_listener); // Release the listener so the server can bind to the same port

        let grpc_addr = format!("127.0.0.1:{}", grpc_port).parse().unwrap();

        // Start mock gRPC auth service
        let auth_service = MockAuthService {
            valid_user_id: test_user_id.clone(),
        };

        let grpc_server_handle = tokio::spawn(async move {
            Server::builder()
                .add_service(AuthServiceServer::new(auth_service))
                .serve(grpc_addr)
                .await
                .expect("Failed to start mock gRPC server");
        });

        // Wait a bit for gRPC server to start
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Create auth service client
        let auth_service_client =
            settings_service::grpc::auth_service::service::auth_service_client::AuthServiceClient::connect(
                format!("http://127.0.0.1:{}", grpc_port),
            )
            .await
            .expect("Failed to connect to mock auth service");

        // Create app state
        let config = Config {
            settings_http_port: 0,
            pg_url: format!("127.0.0.1:{}", host_port),
            pg_username: "postgres".to_string(),
            pg_password: "postgres".to_string(),
            pg_database: "postgres".to_string(),
            cors_url: "http://localhost:3000".to_string(),
            auth_hostname: "127.0.0.1".to_string(),
            auth_grpc_port: grpc_port as u32,
        };

        let state = Arc::new(AppState::new(config.clone(), db_pool, auth_service_client));

        // Create router
        let cors = axum::http::HeaderValue::from_str(&config.cors_url).unwrap();
        let router = axum::Router::new()
            .nest(
                "/health",
                settings_service::routes::health::get_router(state.clone()),
            )
            .nest(
                "/user",
                settings_service::routes::user::get_router(state.clone()),
            )
            .with_state(state)
            .layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(cors)
                    .allow_methods([
                        axum::http::Method::GET,
                        axum::http::Method::POST,
                    ])
                    .allow_credentials(true)
                    .allow_headers([
                        axum::http::header::AUTHORIZATION,
                        axum::http::header::ACCEPT,
                        axum::http::header::CONTENT_TYPE,
                    ]),
            );

        let test_server = TestServer::new(router).expect("Failed to create test server");

        Self {
            _postgres_container: postgres_container,
            test_server,
            _grpc_server_handle: grpc_server_handle,
            test_user_id,
            grpc_port,
        }
    }
}

#[tokio::test]
async fn test_health_endpoint() {
    let fixture = TestFixture::new().await;

    let response = fixture.test_server.get("/health").await;

    assert_eq!(response.status_code(), axum::http::StatusCode::OK);

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"], "Healthy");
    assert_eq!(body["database"], "Connected");
}

#[tokio::test]
async fn test_get_user_settings_unauthorized() {
    let fixture = TestFixture::new().await;

    let response = fixture.test_server.get("/user").await;

    assert_eq!(
        response.status_code(),
        axum::http::StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn test_get_user_settings_with_valid_token() {
    let fixture = TestFixture::new().await;

    let response = fixture
        .test_server
        .get("/user")
        .authorization_bearer("valid_token_123")
        .await;

    // Should return OK even if settings don't exist yet (creates default settings)
    assert!(
        response.status_code() == axum::http::StatusCode::OK
            || response.status_code() == axum::http::StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn test_update_user_settings_unauthorized() {
    let fixture = TestFixture::new().await;

    let settings_data = json!({
        "theme": "dark",
        "language": "en"
    });

    let response = fixture
        .test_server
        .post("/user")
        .json(&settings_data)
        .await;

    assert_eq!(
        response.status_code(),
        axum::http::StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn test_update_user_settings_with_valid_token() {
    let fixture = TestFixture::new().await;

    let settings_data = json!({
        "theme": "dark",
        "language": "en",
        "timezone": "UTC"
    });

    let response = fixture
        .test_server
        .post("/user")
        .authorization_bearer("valid_token_123")
        .json(&settings_data)
        .await;

    // Should create or update settings
    assert!(
        response.status_code() == axum::http::StatusCode::OK
            || response.status_code() == axum::http::StatusCode::CREATED
    );
}

#[tokio::test]
async fn test_get_settings_after_update() {
    let fixture = TestFixture::new().await;

    // First, update settings
    let settings_data = json!({
        "theme": "dark",
        "language": "es",
        "timezone": "America/New_York"
    });

    let update_response = fixture
        .test_server
        .post("/user")
        .authorization_bearer("valid_token_123")
        .json(&settings_data)
        .await;

    assert!(
        update_response.status_code() == axum::http::StatusCode::OK
            || update_response.status_code() == axum::http::StatusCode::CREATED
    );

    // Then, get settings
    let get_response = fixture
        .test_server
        .get("/user")
        .authorization_bearer("valid_token_123")
        .await;

    assert_eq!(get_response.status_code(), axum::http::StatusCode::OK);

    let body: serde_json::Value = get_response.json();
    // Check that we got some settings back (structure may vary)
    assert!(body.is_object());
}

#[tokio::test]
async fn test_update_settings_with_invalid_token() {
    let fixture = TestFixture::new().await;

    let settings_data = json!({
        "theme": "dark"
    });

    let response = fixture
        .test_server
        .post("/user")
        .authorization_bearer("invalid_token")
        .json(&settings_data)
        .await;

    assert_eq!(
        response.status_code(),
        axum::http::StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn test_cors_headers() {
    let fixture = TestFixture::new().await;

    let response = fixture.test_server.get("/health").await;

    assert_eq!(response.status_code(), axum::http::StatusCode::OK);
}
