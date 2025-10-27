use std::env::var;

/// Application configuration loaded from environment variables
///
/// This struct contains all the configuration parameters needed to run the Settings Service.
/// All values are loaded from environment variables at startup and are used throughout
/// the application lifecycle.
///
/// # Fields
///
/// ## Server Configuration
/// * `settings_http_port` - Port number for the HTTP server to listen on
///
/// ## Database Configuration
/// * `pg_url` - PostgreSQL server hostname or IP address
/// * `pg_username` - Database username for authentication
/// * `pg_password` - Database password for authentication
/// * `pg_database` - Name of the settings-service database to connect to (default: brewget_settings)
///
/// ## Security Configuration
/// * `cors_url` - Allowed CORS origin URL for frontend integration
#[derive(Clone)]
pub struct Config {
    pub settings_http_port: u32,
    pub pg_url: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_database: String,
    pub cors_url: String,
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
    /// - `SETTINGS_HTTP_PORT` - Must be a valid u32 port number
    /// - `PG_URL` - PostgreSQL server URL
    /// - `PG_USERNAME` - Database username
    /// - `PG_PASSWORD` - Database password
    /// - `SETTINGS_PG_DATABASE` - Settings service database name (falls back to PG_DATABASE if not set)
    /// - `CORS_URL` - Allowed CORS origin URL
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - Any required environment variable is missing
    /// - `SETTINGS_HTTP_PORT` cannot be parsed as u32
    ///
    /// # Returns
    ///
    /// Returns a new `Config` instance with all values loaded from environment variables.
    ///
    /// # Example
    ///
    /// ```rust
    /// use settings_service::Config;
    ///
    /// // Ensure environment variables are set before calling
    /// let config = Config::init();
    /// println!("Server will run on port: {}", config.settings_http_port);
    /// ```
    pub fn init() -> Self {
        let settings_http_port = var("SETTINGS_HTTP_PORT")
            .map(|val| val.parse::<u32>())
            .expect("SETTINGS_HTTP_PORT must be provided.")
            .expect("SETTINGS_HTTP_PORT must be a valid u32.");
        let pg_url = var("PG_URL").expect("PG_URL must be provided.");
        let pg_username = var("PG_USERNAME").expect("PG_USERNAME must be provided.");
        let pg_password = var("PG_PASSWORD").expect("PG_PASSWORD must be provided.");
        // Use SETTINGS_PG_DATABASE if provided, otherwise fall back to PG_DATABASE for backwards compatibility
        let pg_database = var("SETTINGS_PG_DATABASE")
            .or_else(|_| var("PG_DATABASE"))
            .expect("SETTINGS_PG_DATABASE or PG_DATABASE must be provided.");
        let cors_url = var("CORS_URL").expect("CORS_URL must be provided.");

        Self {
            settings_http_port,
            pg_url,
            pg_username,
            pg_password,
            pg_database,
            cors_url,
        }
    }
}
