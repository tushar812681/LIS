use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Sync Queue Entry - represents an operation waiting to be synced
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, SimpleObject)]
#[graphql(name = "SyncQueueEntry")]
pub struct SyncQueueEntry {
    pub id: String,
    pub device_id: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub operation: SyncOperation,
    pub data: serde_json::Value,
    pub client_timestamp: DateTime<Utc>,
    pub server_timestamp: Option<DateTime<Utc>>,
    pub status: SyncStatus,
    pub retry_count: i32,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Sync Conflict - represents a conflict that needs resolution
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, SimpleObject)]
#[graphql(name = "SyncConflict")]
pub struct SyncConflict {
    pub id: String,
    pub device_id: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub client_data: serde_json::Value,
    pub server_data: serde_json::Value,
    pub client_version: i64,
    pub server_version: i64,
    pub resolution_status: ConflictResolutionStatus,
    pub resolution_data: Option<serde_json::Value>,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Sync Device - represents a device that can sync
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, SimpleObject)]
#[graphql(name = "SyncDevice")]
pub struct SyncDevice {
    pub id: String,
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub user_id: Option<String>,
    pub organization_id: String,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_enabled: bool,
    pub offline_mode: bool,
    pub network_status: NetworkStatus,
    pub sync_stats: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Sync Log - audit trail of sync operations
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, SimpleObject)]
#[graphql(name = "SyncLog")]
pub struct SyncLog {
    pub id: String,
    pub device_id: String,
    pub sync_session_id: String,
    pub entity_type: EntityType,
    pub operation: SyncOperation,
    pub entity_count: i32,
    pub success_count: i32,
    pub failure_count: i32,
    pub conflict_count: i32,
    pub duration_ms: i64,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

/// Entity Type - types of entities that can be synced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "entity_type", rename_all = "lowercase")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    Patient,
    Sample,
    Order,
    Result,
    Invoice,
    Payment,
    Report,
    Inventory,
    Equipment,
    QcResult,
    Notification,
}

/// Sync Operation - type of operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "sync_operation", rename_all = "lowercase")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncOperation {
    Create,
    Update,
    Delete,
    SoftDelete,
}

/// Sync Status - status of sync queue entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "sync_status", rename_all = "lowercase")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Conflict,
    Skipped,
}

/// Conflict Resolution Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "conflict_resolution_status", rename_all = "lowercase")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConflictResolutionStatus {
    Pending,
    ClientWins,
    ServerWins,
    ManualResolution,
    Merged,
}

/// Device Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "device_type", rename_all = "lowercase")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    Web,
    Mobile,
    Tablet,
    Desktop,
}

/// Network Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "network_status", rename_all = "lowercase")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NetworkStatus {
    Online,
    Offline,
    SlowNetwork,
    Unknown,
}

// Input types for GraphQL mutations

#[derive(Debug, InputObject)]
pub struct QueueOperationInput {
    pub device_id: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub operation: SyncOperation,
    pub data: serde_json::Value,
    pub client_timestamp: DateTime<Utc>,
}

#[derive(Debug, InputObject)]
pub struct ResolveConflictInput {
    pub conflict_id: String,
    pub resolution: ConflictResolutionStatus,
    pub resolution_data: Option<serde_json::Value>,
    pub resolved_by: String,
}

#[derive(Debug, InputObject)]
pub struct RegisterDeviceInput {
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub user_id: Option<String>,
    pub organization_id: String,
}

#[derive(Debug, InputObject)]
pub struct UpdateNetworkStatusInput {
    pub device_id: String,
    pub network_status: NetworkStatus,
}

// Response types

#[derive(Debug, SimpleObject)]
pub struct SyncResponse {
    pub synced_count: i32,
    pub conflict_count: i32,
    pub failed_count: i32,
    pub pending_count: i32,
    pub sync_session_id: String,
}

#[derive(Debug, SimpleObject)]
pub struct SyncStatusResponse {
    pub device_id: String,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub pending_count: i32,
    pub conflict_count: i32,
    pub offline_mode: bool,
    pub network_status: NetworkStatus,
}

impl SyncQueueEntry {
    pub fn new(
        device_id: String,
        entity_type: EntityType,
        entity_id: String,
        operation: SyncOperation,
        data: serde_json::Value,
        client_timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            device_id,
            entity_type,
            entity_id,
            operation,
            data,
            client_timestamp,
            server_timestamp: None,
            status: SyncStatus::Pending,
            retry_count: 0,
            error_message: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl SyncConflict {
    pub fn new(
        device_id: String,
        entity_type: EntityType,
        entity_id: String,
        client_data: serde_json::Value,
        server_data: serde_json::Value,
        client_version: i64,
        server_version: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            device_id,
            entity_type,
            entity_id,
            client_data,
            server_data,
            client_version,
            server_version,
            resolution_status: ConflictResolutionStatus::Pending,
            resolution_data: None,
            resolved_by: None,
            resolved_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl SyncDevice {
    pub fn new(
        device_id: String,
        device_name: String,
        device_type: DeviceType,
        user_id: Option<String>,
        organization_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            device_id,
            device_name,
            device_type,
            user_id,
            organization_id,
            last_sync_at: None,
            sync_enabled: true,
            offline_mode: false,
            network_status: NetworkStatus::Online,
            sync_stats: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
