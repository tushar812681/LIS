use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub database_max_connections: u32,
    pub redis_url: String,
    pub enable_events: bool,
    pub kafka_brokers: String,
    pub sync_interval_seconds: u64,
    pub conflict_resolution_strategy: ConflictResolutionStrategy,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictResolutionStrategy {
    LastWriteWins,
    ManualResolution,
    ServerWins,
    ClientWins,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8095".to_string())
                .parse()?,
            database_max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "32".to_string())
                .parse()?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            enable_events: std::env::var("ENABLE_EVENTS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            kafka_brokers: std::env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            sync_interval_seconds: std::env::var("SYNC_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            conflict_resolution_strategy: match std::env::var("CONFLICT_RESOLUTION_STRATEGY")
                .unwrap_or_else(|_| "last_write_wins".to_string())
                .to_lowercase()
                .as_str()
            {
                "manual_resolution" | "manual" => ConflictResolutionStrategy::ManualResolution,
                "server_wins" | "server" => ConflictResolutionStrategy::ServerWins,
                "client_wins" | "client" => ConflictResolutionStrategy::ClientWins,
                _ => ConflictResolutionStrategy::LastWriteWins,
            },
        })
    }
}
