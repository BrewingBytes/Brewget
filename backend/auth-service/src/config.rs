use std::env::var;

#[derive(Clone)]
pub struct Config {
    pub cors_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: u32,
    pub jwt_max_age: u32,
}

impl Config {
    pub fn init() -> Self {
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
            cors_url,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age,
        }
    }
}
