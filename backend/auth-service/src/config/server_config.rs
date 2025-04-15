use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig)]
pub struct ServerConfig {
    #[env_config(name = "SERVER_HOST", default = "localhost")]
    host: String,
    #[env_config(name = "SERVER_PORT", default = 8081)]
    port: u16,
}

impl ServerConfig {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
