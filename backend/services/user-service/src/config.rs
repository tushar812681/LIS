use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub enable_caching: bool,
    pub enable_events: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("database_max_connections", 32)?
            .set_default("host", "0.0.0.0")?
            .set_default("port", 8080)?
            .set_default("jwt_secret", "development-secret-change-in-production")?
            .set_default("enable_caching", false)?
            .set_default("enable_events", false)?
            .add_source(config::Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }

    pub fn default() -> Self {
        Self {
            database_url: "postgresql://postgres:postgres@localhost:5432/lis_user".to_string(),
            database_max_connections: 32,
            host: "0.0.0.0".to_string(),
            port: 8080,
            jwt_secret: "development-secret-change-in-production".to_string(),
            enable_caching: false,
            enable_events: false,
        }
    }
}
