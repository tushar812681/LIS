use redis::{Client, Commands, Connection, RedisError, aio::ConnectionManager};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, error};

use common::error::{Error, Result};

#[derive(Clone)]
pub struct CacheClient {
    client: Client,
}

impl CacheClient {
    pub fn new(redis_url: &str) -> Result<Self> {
        info!("Connecting to Redis: {}", redis_url);

        let client = Client::open(redis_url)
            .map_err(|e| {
                error!("Failed to create Redis client: {}", e);
                Error::Redis(e)
            })?;

        // Test connection
        let mut conn = client.get_connection()
            .map_err(|e| {
                error!("Failed to connect to Redis: {}", e);
                Error::Redis(e)
            })?;

        redis::cmd("PING")
            .query::<String>(&mut conn)
            .map_err(|e| {
                error!("Redis health check failed: {}", e);
                Error::Redis(e)
            })?;

        info!("Successfully connected to Redis");

        Ok(Self { client })
    }

    pub fn get_connection(&self) -> Result<Connection> {
        self.client.get_connection().map_err(Error::Redis)
    }

    pub async fn get_async_connection(&self) -> Result<ConnectionManager> {
        ConnectionManager::new(self.client.clone())
            .await
            .map_err(Error::Redis)
    }

    // String operations
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.get_connection()?;
        conn.set(key, value).map_err(Error::Redis)
    }

    pub fn set_with_expiry(&self, key: &str, value: &str, seconds: usize) -> Result<()> {
        let mut conn = self.get_connection()?;
        conn.set_ex(key, value, seconds as u64).map_err(Error::Redis)
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.get_connection()?;
        conn.get(key).map_err(Error::Redis)
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.get_connection()?;
        conn.del(key).map_err(Error::Redis)
    }

    pub fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.get_connection()?;
        conn.exists(key).map_err(Error::Redis)
    }

    // JSON operations (serialize/deserialize)
    pub fn set_json<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let json = serde_json::to_string(value)
            .map_err(Error::Serialization)?;
        self.set(key, &json)
    }

    pub fn set_json_with_expiry<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        seconds: usize
    ) -> Result<()> {
        let json = serde_json::to_string(value)
            .map_err(Error::Serialization)?;
        self.set_with_expiry(key, &json, seconds)
    }

    pub fn get_json<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        match self.get(key)? {
            Some(json) => {
                let value = serde_json::from_str(&json)
                    .map_err(Error::Serialization)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    // Hash operations
    pub fn hset(&self, key: &str, field: &str, value: &str) -> Result<()> {
        let mut conn = self.get_connection()?;
        conn.hset(key, field, value).map_err(Error::Redis)
    }

    pub fn hget(&self, key: &str, field: &str) -> Result<Option<String>> {
        let mut conn = self.get_connection()?;
        conn.hget(key, field).map_err(Error::Redis)
    }

    pub fn hgetall(&self, key: &str) -> Result<Vec<(String, String)>> {
        let mut conn = self.get_connection()?;
        conn.hgetall(key).map_err(Error::Redis)
    }

    // List operations
    pub fn lpush(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.get_connection()?;
        conn.lpush(key, value).map_err(Error::Redis)
    }

    pub fn rpush(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.get_connection()?;
        conn.rpush(key, value).map_err(Error::Redis)
    }

    pub fn lrange(&self, key: &str, start: isize, stop: isize) -> Result<Vec<String>> {
        let mut conn = self.get_connection()?;
        conn.lrange(key, start, stop).map_err(Error::Redis)
    }

    // Set operations
    pub fn sadd(&self, key: &str, member: &str) -> Result<()> {
        let mut conn = self.get_connection()?;
        conn.sadd(key, member).map_err(Error::Redis)
    }

    pub fn smembers(&self, key: &str) -> Result<Vec<String>> {
        let mut conn = self.get_connection()?;
        conn.smembers(key).map_err(Error::Redis)
    }

    pub fn sismember(&self, key: &str, member: &str) -> Result<bool> {
        let mut conn = self.get_connection()?;
        conn.sismember(key, member).map_err(Error::Redis)
    }

    // Expiry operations
    pub fn expire(&self, key: &str, seconds: usize) -> Result<()> {
        let mut conn = self.get_connection()?;
        conn.expire(key, seconds as i64).map_err(Error::Redis)
    }

    pub fn ttl(&self, key: &str) -> Result<i64> {
        let mut conn = self.get_connection()?;
        conn.ttl(key).map_err(Error::Redis)
    }

    // Increment operations
    pub fn incr(&self, key: &str) -> Result<i64> {
        let mut conn = self.get_connection()?;
        conn.incr(key, 1).map_err(Error::Redis)
    }

    pub fn incr_by(&self, key: &str, amount: i64) -> Result<i64> {
        let mut conn = self.get_connection()?;
        conn.incr(key, amount).map_err(Error::Redis)
    }

    // Pattern matching
    pub fn keys(&self, pattern: &str) -> Result<Vec<String>> {
        let mut conn = self.get_connection()?;
        conn.keys(pattern).map_err(Error::Redis)
    }

    // Health check
    pub fn health_check(&self) -> Result<()> {
        let mut conn = self.get_connection()?;
        redis::cmd("PING")
            .query::<String>(&mut conn)
            .map_err(Error::Redis)?;
        Ok(())
    }
}

/// Cache key builder helpers
pub mod keys {
    use uuid::Uuid;

    pub fn patient(patient_id: Uuid) -> String {
        format!("patient:{}", patient_id)
    }

    pub fn patient_by_mrn(mrn: &str) -> String {
        format!("patient:mrn:{}", mrn)
    }

    pub fn sample(sample_id: Uuid) -> String {
        format!("sample:{}", sample_id)
    }

    pub fn order(order_id: Uuid) -> String {
        format!("order:{}", order_id)
    }

    pub fn test_result(result_id: Uuid) -> String {
        format!("result:{}", result_id)
    }

    pub fn session(session_id: &str) -> String {
        format!("session:{}", session_id)
    }

    pub fn rate_limit(user_id: Uuid, endpoint: &str) -> String {
        format!("ratelimit:{}:{}", user_id, endpoint)
    }

    pub fn equipment_status(equipment_id: Uuid) -> String {
        format!("equipment:{}:status", equipment_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    #[ignore] // Requires Redis connection
    fn test_redis_operations() {
        let cache = CacheClient::new("redis://localhost:6379").unwrap();

        // String operations
        cache.set("test_key", "test_value").unwrap();
        assert_eq!(cache.get("test_key").unwrap(), Some("test_value".to_string()));

        // Expiry
        cache.set_with_expiry("temp_key", "temp_value", 60).unwrap();
        assert!(cache.exists("temp_key").unwrap());

        // Delete
        cache.delete("test_key").unwrap();
        assert_eq!(cache.get("test_key").unwrap(), None);
    }

    #[test]
    fn test_cache_keys() {
        let patient_id = Uuid::new_v4();
        let key = keys::patient(patient_id);
        assert_eq!(key, format!("patient:{}", patient_id));
    }
}
