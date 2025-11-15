use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub database_max_connections: u32,
    pub redis_url: String,
    pub kafka_brokers: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .set_default("host", "0.0.0.0")?
            .set_default("port", 8081)?
            .set_default("database_url", "postgresql://postgres:postgres@localhost:5432/lis_patient")?
            .set_default("database_max_connections", 32)?
            .set_default("redis_url", "redis://localhost:6379")?
            .set_default("kafka_brokers", "localhost:9092")?
            .set_default("jwt_secret", "development-secret-change-in-production")?
            .build()?
            .try_deserialize()
    }
}
