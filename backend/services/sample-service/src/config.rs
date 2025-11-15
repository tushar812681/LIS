use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    // Server
    pub host: String,
    pub port: u16,

    // Database
    pub database_url: String,
    pub database_max_connections: u32,

    // Service URLs (for inter-service communication)
    pub patient_service_url: String,
    pub order_service_url: String,

    // Redis
    pub redis_url: String,

    // Kafka
    pub kafka_brokers: String,

    // Feature flags
    pub enable_caching: bool,
    pub enable_events: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .set_default("host", "0.0.0.0")?
            .set_default("port", 8082)?
            .set_default("database_max_connections", 32)?
            .set_default("patient_service_url", "http://localhost:8081")?
            .set_default("order_service_url", "http://localhost:8083")?
            .set_default("redis_url", "redis://localhost:6379")?
            .set_default("kafka_brokers", "localhost:9092")?
            .set_default("enable_caching", false)?
            .set_default("enable_events", false)?
            .build()?
            .try_deserialize()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8082,
            database_url: "postgres://postgres:postgres@localhost:5432/lis_sample".to_string(),
            database_max_connections: 32,
            patient_service_url: "http://localhost:8081".to_string(),
            order_service_url: "http://localhost:8083".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            kafka_brokers: "localhost:9092".to_string(),
            enable_caching: false,
            enable_events: false,
        }
    }
}
