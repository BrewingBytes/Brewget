use std::env::var;

/// Application configuration loaded from environment variables
///
/// # Fields
///
/// ## Server Configuration
/// * `auth_http_port` - Port number for the HTTP server to listen on
/// * `auth_grpc_port` - Port number for the gRPC server to listen on
///
/// ## Database Configuration
/// * `pg_url` - PostgreSQL server hostname or IP address
/// * `pg_username` - Database username for authentication
/// * `pg_password` - Database password for authentication
/// * `pg_database` - Name of the auth-service database to connect to (default: brewget_auth)
///
/// ## Security Configuration
/// * `cors_url` - Allowed CORS origin URL for frontend integration
/// * `jwt_secret` - Secret key used for signing and verifying JWT tokens
/// * `jwt_expires_in` - JWT access token expiration time in seconds
/// * `jwt_max_age` - Maximum age for JWT refresh tokens in seconds
///
/// ## Service Integration
/// * `email_hostname` - Hostname of the email service for gRPC communication
/// * `email_grpc_port` - Port number for the email service gRPC server
/// * `frontend_hostname` - Hostname of the frontend application for URL generation
///
/// ## Captcha Configuration
/// * `turnstile_secret` - Cloudflare Turnstile secret key for captcha verification
#[derive(Clone)]
pub struct Config {
    pub auth_http_port: u32,
    pub auth_grpc_port: u32,
    pub pg_url: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_database: String,
    pub cors_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: u32,
    pub jwt_max_age: u32,
    pub email_hostname: String,
    pub email_grpc_port: u32,
    pub frontend_hostname: String,
    pub turnstile_secret: String,
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
    /// - `AUTH_HTTP_PORT` - Must be a valid u32 port number
    /// - `AUTH_GRPC_PORT` - Must be a valid u32 port number
    /// - `PG_URL` - PostgreSQL server URL
    /// - `PG_USERNAME` - Database username
    /// - `PG_PASSWORD` - Database password
    /// - `AUTH_PG_DATABASE` - Auth service database name (falls back to PG_DATABASE if not set)
    /// - `CORS_URL` - Allowed CORS origin URL
    /// - `JWT_SECRET` - Secret key for JWT signing
    /// - `JWT_EXPIRES_IN` - Must be a valid u32 (seconds)
    /// - `JWT_MAX_AGE` - Must be a valid u32 (seconds)
    /// - `EMAIL_HOSTNAME` - Email service hostname
    /// - `EMAIL_GRPC_PORT` - Must be a valid u32 port number
    /// - `FRONTEND_HOSTNAME` - Frontend application hostname
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - Any required environment variable is missing
    /// - `AUTH_HTTP_PORT`, `AUTH_GRPC_PORT`, `JWT_EXPIRES_IN`, `JWT_MAX_AGE`, or `EMAIL_GRPC_PORT`
    ///   cannot be parsed as u32
    ///
    /// # Returns
    ///
    /// Returns a new `Config` instance with all values loaded from environment variables.
    ///
    /// # Example
    ///
    /// ```rust
    /// use auth_service::Config;
    ///
    /// // Ensure environment variables are set before calling
    /// let config = Config::init();
    /// println!("Server will run on port: {}", config.auth_http_port);
    /// ```
    pub fn init() -> Self {
        let auth_http_port = var("AUTH_HTTP_PORT")
            .map(|port| port.parse::<u32>())
            .expect("AUTH_HTTP_PORT must be provided.")
            .expect("AUTH_HTTP_PORT must be an u32.");
        let auth_grpc_port = var("AUTH_GRPC_PORT")
            .map(|port| port.parse::<u32>())
            .expect("AUTH_GRPC_PORT must be provided.")
            .expect("AUTH_GRPC_PORT must be an u32.");
        let pg_url = var("PG_URL").expect("PG_URL must be provided.");
        let pg_username = var("PG_USERNAME").expect("PG_USERNAME must be provided.");
        let pg_password = var("PG_PASSWORD").expect("PG_PASSWORD must be provided.");
        // Use AUTH_PG_DATABASE if provided, otherwise fall back to PG_DATABASE for backwards compatibility
        let pg_database = var("AUTH_PG_DATABASE")
            .or_else(|_| var("PG_DATABASE"))
            .expect("AUTH_PG_DATABASE or PG_DATABASE must be provided.");
        let cors_url = var("CORS_URL").expect("CORS_URL must be provided.");
        let jwt_secret = var("JWT_SECRET").expect("JWT_SECRET must be provided.");
        let jwt_expires_in = var("JWT_EXPIRES_IN")
            .map(|expiry| expiry.parse::<u32>())
            .expect("JWT_EXPIRES_IN must be provided.")
            .expect("JWT_EXPIRES_IN must be an u32.");
        let jwt_max_age = var("JWT_MAX_AGE")
            .map(|max_age| max_age.parse::<u32>())
            .expect("JWT_MAX_AGE must be provided.")
            .expect("JWT_MAX_AGE must be an u32.");
        let email_hostname = var("EMAIL_HOSTNAME").expect("EMAIL_HOSTNAME must be provided");
        let email_grpc_port = var("EMAIL_GRPC_PORT")
            .map(|port| port.parse::<u32>())
            .expect("EMAIL_GRPC_PORT must be provided.")
            .expect("EMAIL_GRPC_PORT must be an u32.");
        let frontend_hostname =
            var("FRONTEND_HOSTNAME").expect("FRONTEND_HOSTNAME must be provided.");
        let turnstile_secret = var("TURNSTILE_SECRET").expect("TURNSTILE_SECRET must be provided.");

        Self {
            auth_http_port,
            auth_grpc_port,
            pg_url,
            pg_username,
            pg_password,
            pg_database,
            cors_url,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age,
            email_hostname,
            email_grpc_port,
            frontend_hostname,
            turnstile_secret,
        }
    }
}
