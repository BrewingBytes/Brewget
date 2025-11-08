use std::env::var;

/// Application configuration loaded from environment variables
///
/// This struct contains all the configuration parameters needed to run the Transaction Service.
/// All values are loaded from environment variables at startup and are used throughout
/// the application lifecycle.
///
/// # Fields
///
/// ## Server Configuration
/// * `transaction_http_port` - Port number for the HTTP server to listen on
///
/// ## Database Configuration
/// * `pg_url` - PostgreSQL server hostname or IP address
/// * `pg_username` - Database username for authentication
/// * `pg_password` - Database password for authentication
/// * `pg_database` - Name of the transaction-service database to connect to (default: brewget_transactions)
///
/// ## Security Configuration
/// * `cors_url` - Allowed CORS origin URL for frontend integration
///
/// ## Service Integration
/// * `auth_hostname` - Hostname of the auth service for gRPC communication
/// * `auth_grpc_port` - Port number for the auth service gRPC server
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
    ///
    /// This method loads all required configuration values from environment variables.
    /// It performs validation to ensure all required variables are present and properly formatted.
    ///
    /// # Environment Variables
    ///
    /// The following environment variables must be set:
    /// - `TRANSACTION_HTTP_PORT` - Must be a valid u32 port number
    /// - `PG_URL` - PostgreSQL server URL
    /// - `PG_USERNAME` - Database username
    /// - `PG_PASSWORD` - Database password
    /// - `TRANSACTION_PG_DATABASE` - Transaction service database name (falls back to PG_DATABASE if not set)
    /// - `CORS_URL` - Allowed CORS origin URL
    /// - `AUTH_HOSTNAME` - Auth service hostname
    /// - `AUTH_GRPC_PORT` - Must be a valid u32 port number
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - Any required environment variable is missing
    /// - `TRANSACTION_HTTP_PORT` or `AUTH_GRPC_PORT` cannot be parsed as u32
    ///
    /// # Returns
    ///
    /// Returns a new `Config` instance with all values loaded from environment variables.
    pub fn init() -> Self {
        let transaction_http_port = var("TRANSACTION_HTTP_PORT")
            .map(|val| val.parse::<u32>())
            .expect("TRANSACTION_HTTP_PORT must be provided.")
            .expect("TRANSACTION_HTTP_PORT must be a valid u32.");
        let pg_url = var("PG_URL").expect("PG_URL must be provided.");
        let pg_username = var("PG_USERNAME").expect("PG_USERNAME must be provided.");
        let pg_password = var("PG_PASSWORD").expect("PG_PASSWORD must be provided.");
        // Use TRANSACTION_PG_DATABASE if provided, otherwise fall back to PG_DATABASE
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
