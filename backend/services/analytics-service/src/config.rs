use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub database_max_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/lis_analytics".to_string());

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8093".to_string())
            .parse()
            .expect("PORT must be a number");

        let database_max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "32".to_string())
            .parse()
            .expect("DATABASE_MAX_CONNECTIONS must be a number");

        Ok(Config {
            database_url,
            host,
            port,
            database_max_connections,
        })
    }
}
