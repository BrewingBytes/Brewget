use std::env::var;

/// Application configuration loaded from environment variables
#[derive(Clone)]
pub struct Config {
    pub transaction_http_port: u32,
    pub pg_url: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_database: String,
    pub cors_url: String,
    pub auth_hostname: String,
    pub auth_grpc_port: u32,
}

impl Config {
    /// Initializes configuration from environment variables
    pub fn init() -> Self {
        let transaction_http_port = var("TRANSACTION_HTTP_PORT")
            .map(|val| val.parse::<u32>())
            .expect("TRANSACTION_HTTP_PORT must be provided.")
            .expect("TRANSACTION_HTTP_PORT must be a valid u32.");
        let pg_url = var("PG_URL").expect("PG_URL must be provided.");
        let pg_username = var("PG_USERNAME").expect("PG_USERNAME must be provided.");
        let pg_password = var("PG_PASSWORD").expect("PG_PASSWORD must be provided.");
        let pg_database = var("TRANSACTION_PG_DATABASE")
            .or_else(|_| var("PG_DATABASE"))
            .expect("TRANSACTION_PG_DATABASE or PG_DATABASE must be provided.");
        let cors_url = var("CORS_URL").expect("CORS_URL must be provided.");
        let auth_hostname = var("AUTH_HOSTNAME").expect("AUTH_HOSTNAME must be provided.");
        let auth_grpc_port = var("AUTH_GRPC_PORT")
            .map(|val| val.parse::<u32>())
            .expect("AUTH_GRPC_PORT must be provided.")
            .expect("AUTH_GRPC_PORT must be a valid u32.");

        Self {
            transaction_http_port,
            pg_url,
            pg_username,
            pg_password,
            pg_database,
            cors_url,
            auth_hostname,
            auth_grpc_port,
        }
    }
}
