use sqlx::PgPool;
use redis::{Client as RedisClient, AsyncCommands};
use chrono::Utc;
use uuid::Uuid;

use crate::domain::*;

pub struct SyncRepository {
    db_pool: PgPool,
    redis_client: RedisClient,
}

impl SyncRepository {
    pub fn new(db_pool: PgPool, redis_client: RedisClient) -> Self {
        Self {
            db_pool,
            redis_client,
        }
    }

    // ============================================================================
    // Sync Queue Operations
    // ============================================================================

    pub async fn queue_operation(
        &self,
        entry: &SyncQueueEntry,
    ) -> Result<SyncQueueEntry, sqlx::Error> {
        let result = sqlx::query_as!(
            SyncQueueEntry,
            r#"
            INSERT INTO sync_queue (
                id, device_id, entity_type, entity_id, operation, data,
                client_timestamp, server_timestamp, status, retry_count,
                error_message, created_at, updated_at
            ) VALUES ($1, $2, $3::entity_type, $4, $5::sync_operation, $6, $7, $8, $9::sync_status, $10, $11, $12, $13)
            RETURNING
                id, device_id,
                entity_type as "entity_type!: EntityType",
                entity_id,
                operation as "operation!: SyncOperation",
                data,
                client_timestamp, server_timestamp,
                status as "status!: SyncStatus",
                retry_count, error_message,
                created_at, updated_at
            "#,
            entry.id,
            entry.device_id,
            entry.entity_type as EntityType,
            entry.entity_id,
            entry.operation as SyncOperation,
            entry.data,
            entry.client_timestamp,
            entry.server_timestamp,
            entry.status as SyncStatus,
            entry.retry_count,
            entry.error_message,
            entry.created_at,
            entry.updated_at,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }

    pub async fn get_pending_operations(
        &self,
        device_id: &str,
        limit: i64,
    ) -> Result<Vec<SyncQueueEntry>, sqlx::Error> {
        let results = sqlx::query_as!(
            SyncQueueEntry,
            r#"
            SELECT
                id, device_id,
                entity_type as "entity_type!: EntityType",
                entity_id,
                operation as "operation!: SyncOperation",
                data,
                client_timestamp, server_timestamp,
                status as "status!: SyncStatus",
                retry_count, error_message,
                created_at, updated_at
            FROM sync_queue
            WHERE device_id = $1
              AND status = 'pending'::sync_status
            ORDER BY client_timestamp ASC
            LIMIT $2
            "#,
            device_id,
            limit,
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(results)
    }

    pub async fn update_queue_status(
        &self,
        id: &str,
        status: SyncStatus,
        error_message: Option<String>,
    ) -> Result<SyncQueueEntry, sqlx::Error> {
        let result = sqlx::query_as!(
            SyncQueueEntry,
            r#"
            UPDATE sync_queue
            SET status = $2::sync_status,
                error_message = $3,
                server_timestamp = $4,
                updated_at = $5
            WHERE id = $1
            RETURNING
                id, device_id,
                entity_type as "entity_type!: EntityType",
                entity_id,
                operation as "operation!: SyncOperation",
                data,
                client_timestamp, server_timestamp,
                status as "status!: SyncStatus",
                retry_count, error_message,
                created_at, updated_at
            "#,
            id,
            status as SyncStatus,
            error_message,
            Some(Utc::now()),
            Utc::now(),
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }

    pub async fn increment_retry_count(&self, id: &str) -> Result<i32, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE sync_queue
            SET retry_count = retry_count + 1,
                updated_at = $2
            WHERE id = $1
            RETURNING retry_count
            "#,
            id,
            Utc::now(),
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result.retry_count)
    }

    pub async fn get_queue_stats(&self, device_id: &str) -> Result<QueueStats, sqlx::Error> {
        let stats = sqlx::query!(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE status = 'pending'::sync_status) as "pending_count!",
                COUNT(*) FILTER (WHERE status = 'completed'::sync_status) as "completed_count!",
                COUNT(*) FILTER (WHERE status = 'failed'::sync_status) as "failed_count!",
                COUNT(*) FILTER (WHERE status = 'conflict'::sync_status) as "conflict_count!"
            FROM sync_queue
            WHERE device_id = $1
            "#,
            device_id,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(QueueStats {
            pending_count: stats.pending_count as i32,
            completed_count: stats.completed_count as i32,
            failed_count: stats.failed_count as i32,
            conflict_count: stats.conflict_count as i32,
        })
    }

    // ============================================================================
    // Conflict Management
    // ============================================================================

    pub async fn create_conflict(
        &self,
        conflict: &SyncConflict,
    ) -> Result<SyncConflict, sqlx::Error> {
        let result = sqlx::query_as!(
            SyncConflict,
            r#"
            INSERT INTO sync_conflicts (
                id, device_id, entity_type, entity_id,
                client_data, server_data, client_version, server_version,
                resolution_status, resolution_data, resolved_by, resolved_at,
                created_at, updated_at
            ) VALUES ($1, $2, $3::entity_type, $4, $5, $6, $7, $8, $9::conflict_resolution_status, $10, $11, $12, $13, $14)
            RETURNING
                id, device_id,
                entity_type as "entity_type!: EntityType",
                entity_id,
                client_data, server_data,
                client_version, server_version,
                resolution_status as "resolution_status!: ConflictResolutionStatus",
                resolution_data, resolved_by, resolved_at,
                created_at, updated_at
            "#,
            conflict.id,
            conflict.device_id,
            conflict.entity_type as EntityType,
            conflict.entity_id,
            conflict.client_data,
            conflict.server_data,
            conflict.client_version,
            conflict.server_version,
            conflict.resolution_status as ConflictResolutionStatus,
            conflict.resolution_data,
            conflict.resolved_by,
            conflict.resolved_at,
            conflict.created_at,
            conflict.updated_at,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }

    pub async fn get_pending_conflicts(
        &self,
        device_id: &str,
    ) -> Result<Vec<SyncConflict>, sqlx::Error> {
        let results = sqlx::query_as!(
            SyncConflict,
            r#"
            SELECT
                id, device_id,
                entity_type as "entity_type!: EntityType",
                entity_id,
                client_data, server_data,
                client_version, server_version,
                resolution_status as "resolution_status!: ConflictResolutionStatus",
                resolution_data, resolved_by, resolved_at,
                created_at, updated_at
            FROM sync_conflicts
            WHERE device_id = $1
              AND resolution_status = 'pending'::conflict_resolution_status
            ORDER BY created_at DESC
            "#,
            device_id,
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(results)
    }

    pub async fn resolve_conflict(
        &self,
        conflict_id: &str,
        resolution_status: ConflictResolutionStatus,
        resolution_data: Option<serde_json::Value>,
        resolved_by: &str,
    ) -> Result<SyncConflict, sqlx::Error> {
        let result = sqlx::query_as!(
            SyncConflict,
            r#"
            UPDATE sync_conflicts
            SET resolution_status = $2::conflict_resolution_status,
                resolution_data = $3,
                resolved_by = $4,
                resolved_at = $5,
                updated_at = $6
            WHERE id = $1
            RETURNING
                id, device_id,
                entity_type as "entity_type!: EntityType",
                entity_id,
                client_data, server_data,
                client_version, server_version,
                resolution_status as "resolution_status!: ConflictResolutionStatus",
                resolution_data, resolved_by, resolved_at,
                created_at, updated_at
            "#,
            conflict_id,
            resolution_status as ConflictResolutionStatus,
            resolution_data,
            Some(resolved_by.to_string()),
            Some(Utc::now()),
            Utc::now(),
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }

    // ============================================================================
    // Device Management
    // ============================================================================

    pub async fn register_device(
        &self,
        device: &SyncDevice,
    ) -> Result<SyncDevice, sqlx::Error> {
        let result = sqlx::query_as!(
            SyncDevice,
            r#"
            INSERT INTO sync_devices (
                id, device_id, device_name, device_type, user_id, organization_id,
                last_sync_at, sync_enabled, offline_mode, network_status,
                sync_stats, created_at, updated_at
            ) VALUES ($1, $2, $3, $4::device_type, $5, $6, $7, $8, $9, $10::network_status, $11, $12, $13)
            ON CONFLICT (device_id)
            DO UPDATE SET
                device_name = EXCLUDED.device_name,
                device_type = EXCLUDED.device_type,
                user_id = EXCLUDED.user_id,
                updated_at = EXCLUDED.updated_at
            RETURNING
                id, device_id, device_name,
                device_type as "device_type!: DeviceType",
                user_id, organization_id, last_sync_at,
                sync_enabled, offline_mode,
                network_status as "network_status!: NetworkStatus",
                sync_stats, created_at, updated_at
            "#,
            device.id,
            device.device_id,
            device.device_name,
            device.device_type as DeviceType,
            device.user_id,
            device.organization_id,
            device.last_sync_at,
            device.sync_enabled,
            device.offline_mode,
            device.network_status as NetworkStatus,
            device.sync_stats,
            device.created_at,
            device.updated_at,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }

    pub async fn get_device(&self, device_id: &str) -> Result<Option<SyncDevice>, sqlx::Error> {
        let result = sqlx::query_as!(
            SyncDevice,
            r#"
            SELECT
                id, device_id, device_name,
                device_type as "device_type!: DeviceType",
                user_id, organization_id, last_sync_at,
                sync_enabled, offline_mode,
                network_status as "network_status!: NetworkStatus",
                sync_stats, created_at, updated_at
            FROM sync_devices
            WHERE device_id = $1
            "#,
            device_id,
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(result)
    }

    pub async fn update_last_sync(&self, device_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE sync_devices
            SET last_sync_at = $2,
                updated_at = $3
            WHERE device_id = $1
            "#,
            device_id,
            Some(Utc::now()),
            Utc::now(),
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn update_network_status(
        &self,
        device_id: &str,
        network_status: NetworkStatus,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE sync_devices
            SET network_status = $2::network_status,
                offline_mode = $3,
                updated_at = $4
            WHERE device_id = $1
            "#,
            device_id,
            network_status as NetworkStatus,
            network_status == NetworkStatus::Offline,
            Utc::now(),
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    // ============================================================================
    // Sync Logging
    // ============================================================================

    pub async fn create_sync_log(&self, log: &SyncLog) -> Result<SyncLog, sqlx::Error> {
        let result = sqlx::query_as!(
            SyncLog,
            r#"
            INSERT INTO sync_logs (
                id, device_id, sync_session_id, entity_type, operation,
                entity_count, success_count, failure_count, conflict_count,
                duration_ms, started_at, completed_at
            ) VALUES ($1, $2, $3, $4::entity_type, $5::sync_operation, $6, $7, $8, $9, $10, $11, $12)
            RETURNING
                id, device_id, sync_session_id,
                entity_type as "entity_type!: EntityType",
                operation as "operation!: SyncOperation",
                entity_count, success_count, failure_count, conflict_count,
                duration_ms, started_at, completed_at
            "#,
            log.id,
            log.device_id,
            log.sync_session_id,
            log.entity_type as EntityType,
            log.operation as SyncOperation,
            log.entity_count,
            log.success_count,
            log.failure_count,
            log.conflict_count,
            log.duration_ms,
            log.started_at,
            log.completed_at,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }

    // ============================================================================
    // Redis Cache Operations
    // ============================================================================

    pub async fn cache_sync_status(
        &self,
        device_id: &str,
        status: &SyncStatusResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.redis_client.get_async_connection().await?;
        let key = format!("sync:status:{}", device_id);
        let value = serde_json::to_string(status)?;

        conn.set_ex(&key, value, 300).await?; // Cache for 5 minutes

        Ok(())
    }

    pub async fn get_cached_sync_status(
        &self,
        device_id: &str,
    ) -> Result<Option<SyncStatusResponse>, Box<dyn std::error::Error>> {
        let mut conn = self.redis_client.get_async_connection().await?;
        let key = format!("sync:status:{}", device_id);

        let value: Option<String> = conn.get(&key).await?;

        match value {
            Some(v) => Ok(Some(serde_json::from_str(&v)?)),
            None => Ok(None),
        }
    }
}

// Helper struct for queue statistics
pub struct QueueStats {
    pub pending_count: i32,
    pub completed_count: i32,
    pub failed_count: i32,
    pub conflict_count: i32,
}
