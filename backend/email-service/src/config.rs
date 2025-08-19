use std::env::var;

/// Application configuration loaded from environment variables
///
/// # Fields
/// * `smtp_email` - Email address to send the mail from
/// * `smtp_name` - Email address name to send the mail from
/// * `smtp_relay` - SMTP Relay
/// * `smtp_username` - SMTP Username
/// * `smtp_password` - SMTP Password
#[derive(Clone)]
pub struct Config {
    pub email_grpc_port: u32,
    pub smtp_email: String,
    pub smtp_name: String,
    pub smtp_relay: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl Config {
    // Initializes configuration from environment variables
    ///
    /// # Panics
    /// Panics if any required environment variable is missing or invalid
    ///
    /// # Returns
    /// Returns a new Config instance with values from environment
    pub fn init() -> Self {
        let email_grpc_port = var("EMAIL_GRPC_PORT")
            .map(|val| val.parse::<u32>())
            .expect("EMAIL_GRPC_PORT must be provided.")
            .expect("EMAIL_GRPC_PORT must be a valid u32.");
        let smtp_email = var("SMTP_EMAIL").expect("SMTP_EMAIL must be provided.");
        let smtp_name = var("SMTP_NAME").expect("SMTP_NAME must be provided.");
        let smtp_relay = var("SMTP_RELAY").expect("SMTP_RELAY must be provided.");
        let smtp_username = var("SMTP_USERNAME").expect("SMTP_USERNAME must be provided.");
        let smtp_password = var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be provided.");

        Self {
            email_grpc_port,
            smtp_email,
            smtp_name,
            smtp_relay,
            smtp_username,
            smtp_password,
        }
    }
}
