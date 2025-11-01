use std::env::var;

/// Application configuration loaded from environment variables
///
/// This struct contains all the configuration parameters needed to run the Email Service.
/// All values are loaded from environment variables at startup and are used throughout
/// the application lifecycle for SMTP configuration and gRPC server setup.
///
/// # Fields
///
/// ## Server Configuration
/// * `email_grpc_port` - Port number for the gRPC server to listen on
/// * `email_http_port` - Port number for the HTTP health check server to listen on
///
/// ## SMTP Configuration
/// * `smtp_email` - Email address to send emails from
/// * `smtp_name` - Display name for the sender
/// * `smtp_relay` - SMTP server hostname for sending emails
/// * `smtp_username` - SMTP authentication username
/// * `smtp_password` - SMTP authentication password
#[derive(Clone)]
pub struct Config {
    pub email_grpc_port: u32,
    pub email_http_port: u32,
    pub smtp_email: String,
    pub smtp_name: String,
    pub smtp_relay: String,
    pub smtp_username: String,
    pub smtp_password: String,
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
    /// - `EMAIL_GRPC_PORT` - Must be a valid u32 port number
    /// - `EMAIL_HTTP_PORT` - Must be a valid u32 port number for health checks
    /// - `SMTP_EMAIL` - Email address to send from
    /// - `SMTP_NAME` - Display name for the sender
    /// - `SMTP_RELAY` - SMTP server hostname
    /// - `SMTP_USERNAME` - SMTP authentication username
    /// - `SMTP_PASSWORD` - SMTP authentication password
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - Any required environment variable is missing
    /// - `EMAIL_GRPC_PORT` or `EMAIL_HTTP_PORT` cannot be parsed as u32
    ///
    /// # Returns
    ///
    /// Returns a new `Config` instance with all values loaded from environment variables.
    ///
    /// # Example
    ///
    /// ```rust
    /// use email_service::Config;
    ///
    /// // Ensure environment variables are set before calling
    /// let config = Config::init();
    /// println!("SMTP server: {}", config.smtp_relay);
    /// ```
    pub fn init() -> Self {
        let email_grpc_port = var("EMAIL_GRPC_PORT")
            .map(|val| val.parse::<u32>())
            .expect("EMAIL_GRPC_PORT must be provided.")
            .expect("EMAIL_GRPC_PORT must be a valid u32.");
        let email_http_port = var("EMAIL_HTTP_PORT")
            .map(|val| val.parse::<u32>())
            .expect("EMAIL_HTTP_PORT must be provided.")
            .expect("EMAIL_HTTP_PORT must be a valid u32.");
        let smtp_email = var("SMTP_EMAIL").expect("SMTP_EMAIL must be provided.");
        let smtp_name = var("SMTP_NAME").expect("SMTP_NAME must be provided.");
        let smtp_relay = var("SMTP_RELAY").expect("SMTP_RELAY must be provided.");
        let smtp_username = var("SMTP_USERNAME").expect("SMTP_USERNAME must be provided.");
        let smtp_password = var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be provided.");

        Self {
            email_grpc_port,
            email_http_port,
            smtp_email,
            smtp_name,
            smtp_relay,
            smtp_username,
            smtp_password,
        }
    }
}
