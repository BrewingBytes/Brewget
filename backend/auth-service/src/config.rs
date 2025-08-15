use std::env::var;

/// Application configuration loaded from environment variables
///
/// # Fields
/// * `pg_url` - PostgreSQL server URL
/// * `pg_username` - Database username
/// * `pg_password` - Database password
/// * `pg_database` - Database name
/// * `cors_url` - Allowed CORS origin URL
/// * `jwt_secret` - Secret key for JWT signing
/// * `jwt_expires_in` - JWT token expiration time in seconds
/// * `jwt_max_age` - Maximum age for JWT refresh tokens in seconds
#[derive(Clone)]
pub struct Config {
    pub pg_url: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_database: String,
    pub cors_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: u32,
    pub jwt_max_age: u32,
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
        let pg_url = var("PG_URL").expect("PG_URL must be provided");
        let pg_username = var("PG_USERNAME").expect("PG_USERNAME must be provided.");
        let pg_password = var("PG_PASSWORD").expect("PG_PASSWORD must be provided.");
        let pg_database = var("PG_DATABASE").expect("PG_DATABASE must be provided");
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

        Self {
            pg_url,
            pg_username,
            pg_password,
            pg_database,
            cors_url,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age,
        }
    }
}
