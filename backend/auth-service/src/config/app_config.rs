use super::server_config::ServerConfig;

pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn init() -> Self {
        AppConfig {
            server: ServerConfig::init().unwrap(),
        }
    }
}
