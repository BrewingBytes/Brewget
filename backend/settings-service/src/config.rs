use std::env::var;

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
    // Initializes configuration from environment variables
    ///
    /// # Panics
    /// Panics if any required environment variable is missing or invalid
    ///
    /// # Returns
    /// Returns a new Config instance with values from environment
    pub fn init() -> Self {
        let settings_http_port = var("SETTINGS_HTTP_PORT")
            .map(|val| val.parse::<u32>())
            .expect("SETTINGS_HTTP_PORT must be provided.")
            .expect("SETTINGS_HTTP_PORT must be a valid u32.");
        let pg_url = var("PG_URL").expect("PG_URL must be provided.");
        let pg_username = var("PG_USERNAME").expect("PG_USERNAME must be provided.");
        let pg_password = var("PG_PASSWORD").expect("PG_PASSWORD must be provided.");
        let pg_database = var("PG_DATABASE").expect("PG_DATABASE must be provided.");
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
