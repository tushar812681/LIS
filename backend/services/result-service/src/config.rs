use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub host: String,
    pub port: u16,
    pub enable_caching: bool,
    pub enable_events: bool,
    pub patient_service_url: String,
    pub sample_service_url: String,
    pub order_service_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("database_max_connections", 32)?
            .set_default("host", "0.0.0.0")?
            .set_default("port", 8084)?
            .set_default("enable_caching", false)?
            .set_default("enable_events", false)?
            .set_default("patient_service_url", "http://localhost:8081")?
            .set_default("sample_service_url", "http://localhost:8082")?
            .set_default("order_service_url", "http://localhost:8083")?
            .add_source(config::Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }

    pub fn default() -> Self {
        Self {
            database_url: "postgresql://postgres:postgres@localhost:5432/lis_result".to_string(),
            database_max_connections: 32,
            host: "0.0.0.0".to_string(),
            port: 8084,
            enable_caching: false,
            enable_events: false,
            patient_service_url: "http://localhost:8081".to_string(),
            sample_service_url: "http://localhost:8082".to_string(),
            order_service_url: "http://localhost:8083".to_string(),
        }
    }
}
