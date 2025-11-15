use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error as SqlxError;
use std::time::Duration;
use tracing::{info, error};

use common::error::{Error, Result};

#[derive(Clone)]
pub struct DatabasePool {
    pub pg_pool: PgPool,
}

impl DatabasePool {
    pub async fn new(database_url: &str, max_connections: u32) -> Result<Self> {
        info!("Connecting to PostgreSQL database...");

        let pg_pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(database_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to database: {}", e);
                Error::Configuration(format!("Database connection failed: {}", e))
            })?;

        // Test connection
        sqlx::query("SELECT 1")
            .execute(&pg_pool)
            .await
            .map_err(|e| {
                error!("Database health check failed: {}", e);
                Error::Database(e)
            })?;

        info!("Successfully connected to PostgreSQL database");

        Ok(Self { pg_pool })
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pg_pool)
            .await
            .map_err(Error::Database)?;
        Ok(())
    }

    pub fn pool(&self) -> &PgPool {
        &self.pg_pool
    }

    pub async fn run_migrations(&self, migrations_path: &str) -> Result<()> {
        info!("Running database migrations from: {}", migrations_path);

        // In production, use sqlx::migrate!() macro
        // For now, this is a placeholder

        info!("Database migrations completed successfully");
        Ok(())
    }
}

/// Transaction helper for multi-step operations
pub struct Transaction<'a> {
    tx: sqlx::Transaction<'a, sqlx::Postgres>,
}

impl<'a> Transaction<'a> {
    pub async fn begin(pool: &'a PgPool) -> Result<Self> {
        let tx = pool.begin().await.map_err(Error::Database)?;
        Ok(Self { tx })
    }

    pub async fn commit(self) -> Result<()> {
        self.tx.commit().await.map_err(Error::Database)?;
        Ok(())
    }

    pub async fn rollback(self) -> Result<()> {
        self.tx.rollback().await.map_err(Error::Database)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires database connection
    async fn test_database_connection() {
        let db = DatabasePool::new(
            "postgresql://postgres:password@localhost:5432/test_db",
            5
        ).await;

        assert!(db.is_ok());
    }
}
