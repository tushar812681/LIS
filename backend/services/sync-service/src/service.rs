use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use crate::config::{Config, ConflictResolutionStrategy};
use crate::domain::*;
use crate::repository::{SyncRepository, QueueStats};

pub struct SyncService {
    repository: Arc<SyncRepository>,
    config: Config,
}

impl SyncService {
    pub fn new(repository: SyncRepository, config: Config) -> Self {
        Self {
            repository: Arc::new(repository),
            config,
        }
    }

    // ============================================================================
    // Queue Management
    // ============================================================================

    pub async fn queue_operation(
        &self,
        input: QueueOperationInput,
    ) -> Result<SyncQueueEntry, Box<dyn std::error::Error>> {
        let entry = SyncQueueEntry::new(
            input.device_id,
            input.entity_type,
            input.entity_id,
            input.operation,
            input.data,
            input.client_timestamp,
        );

        let result = self.repository.queue_operation(&entry).await?;

        tracing::info!(
            "Queued {} operation for {} {} on device {}",
            result.operation,
            result.entity_type,
            result.entity_id,
            result.device_id
        );

        Ok(result)
    }

    pub async fn sync_pending_operations(
        &self,
        device_id: &str,
    ) -> Result<SyncResponse, Box<dyn std::error::Error>> {
        let sync_session_id = Uuid::new_v4().to_string();
        let started_at = Utc::now();

        tracing::info!("Starting sync session {} for device {}", sync_session_id, device_id);

        // Get pending operations
        let pending_ops = self.repository.get_pending_operations(device_id, 100).await?;

        let mut synced_count = 0;
        let mut conflict_count = 0;
        let mut failed_count = 0;

        for op in pending_ops {
            match self.process_sync_operation(&op).await {
                Ok(ProcessResult::Success) => {
                    self.repository
                        .update_queue_status(&op.id, SyncStatus::Completed, None)
                        .await?;
                    synced_count += 1;
                }
                Ok(ProcessResult::Conflict(conflict)) => {
                    self.repository
                        .update_queue_status(&op.id, SyncStatus::Conflict, None)
                        .await?;
                    self.repository.create_conflict(&conflict).await?;
                    conflict_count += 1;
                }
                Err(e) => {
                    let retry_count = self.repository.increment_retry_count(&op.id).await?;

                    if retry_count >= 3 {
                        self.repository
                            .update_queue_status(&op.id, SyncStatus::Failed, Some(e.to_string()))
                            .await?;
                        failed_count += 1;
                    } else {
                        self.repository
                            .update_queue_status(&op.id, SyncStatus::Pending, Some(e.to_string()))
                            .await?;
                    }
                }
            }
        }

        // Get remaining pending count
        let stats = self.repository.get_queue_stats(device_id).await?;
        let pending_count = stats.pending_count;

        // Update last sync time
        self.repository.update_last_sync(device_id).await?;

        let completed_at = Utc::now();
        let duration_ms = (completed_at - started_at).num_milliseconds();

        tracing::info!(
            "Sync session {} completed: synced={}, conflicts={}, failed={}, pending={}, duration={}ms",
            sync_session_id, synced_count, conflict_count, failed_count, pending_count, duration_ms
        );

        Ok(SyncResponse {
            synced_count,
            conflict_count,
            failed_count,
            pending_count,
            sync_session_id,
        })
    }

    async fn process_sync_operation(
        &self,
        op: &SyncQueueEntry,
    ) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        // Check for conflicts
        if let Some(conflict) = self.detect_conflict(op).await? {
            return Ok(ProcessResult::Conflict(conflict));
        }

        // Apply operation to target service
        self.apply_operation(op).await?;

        Ok(ProcessResult::Success)
    }

    async fn detect_conflict(
        &self,
        op: &SyncQueueEntry,
    ) -> Result<Option<SyncConflict>, Box<dyn std::error::Error>> {
        // Fetch current server version of the entity
        let server_data = self.fetch_entity_from_service(&op.entity_type, &op.entity_id).await?;

        if let Some(server_data) = server_data {
            // Check if data has changed on server since client last synced
            let client_version = op.data.get("version").and_then(|v| v.as_i64()).unwrap_or(0);
            let server_version = server_data.get("version").and_then(|v| v.as_i64()).unwrap_or(0);

            if server_version > client_version {
                // Conflict detected
                let conflict = SyncConflict::new(
                    op.device_id.clone(),
                    op.entity_type,
                    op.entity_id.clone(),
                    op.data.clone(),
                    server_data.clone(),
                    client_version,
                    server_version,
                );

                tracing::warn!(
                    "Conflict detected for {} {}: client_version={}, server_version={}",
                    op.entity_type, op.entity_id, client_version, server_version
                );

                return Ok(Some(conflict));
            }
        }

        Ok(None)
    }

    async fn apply_operation(
        &self,
        op: &SyncQueueEntry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Route to appropriate service based on entity type
        let service_url = self.get_service_url(&op.entity_type);

        let client = reqwest::Client::new();

        match op.operation {
            SyncOperation::Create => {
                client
                    .post(&format!("{}/api/create", service_url))
                    .json(&op.data)
                    .send()
                    .await?;
            }
            SyncOperation::Update => {
                client
                    .put(&format!("{}/api/{}", service_url, op.entity_id))
                    .json(&op.data)
                    .send()
                    .await?;
            }
            SyncOperation::Delete | SyncOperation::SoftDelete => {
                client
                    .delete(&format!("{}/api/{}", service_url, op.entity_id))
                    .send()
                    .await?;
            }
        }

        Ok(())
    }

    async fn fetch_entity_from_service(
        &self,
        entity_type: &EntityType,
        entity_id: &str,
    ) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let service_url = self.get_service_url(entity_type);

        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/{}", service_url, entity_id))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(Some(response.json().await?))
        } else {
            Ok(None)
        }
    }

    fn get_service_url(&self, entity_type: &EntityType) -> String {
        match entity_type {
            EntityType::Patient => "http://patient-service:8081",
            EntityType::Sample => "http://sample-service:8082",
            EntityType::Order => "http://order-service:8083",
            EntityType::Result => "http://result-service:8084",
            EntityType::Invoice => "http://billing-service:8089",
            EntityType::Payment => "http://billing-service:8089",
            EntityType::Report => "http://report-service:8090",
            EntityType::Inventory => "http://inventory-service:8091",
            EntityType::Equipment => "http://equipment-service:8087",
            EntityType::QcResult => "http://qc-service:8088",
            EntityType::Notification => "http://notification-service:8092",
        }
        .to_string()
    }

    // ============================================================================
    // Conflict Resolution
    // ============================================================================

    pub async fn resolve_conflict(
        &self,
        input: ResolveConflictInput,
    ) -> Result<SyncConflict, Box<dyn std::error::Error>> {
        let conflict = self.repository.resolve_conflict(
            &input.conflict_id,
            input.resolution,
            input.resolution_data.clone(),
            &input.resolved_by,
        ).await?;

        // If resolution data provided, apply it
        if let Some(data) = input.resolution_data {
            // Create a sync operation to apply the resolved data
            let queue_entry = SyncQueueEntry::new(
                conflict.device_id.clone(),
                conflict.entity_type,
                conflict.entity_id.clone(),
                SyncOperation::Update,
                data,
                Utc::now(),
            );

            self.repository.queue_operation(&queue_entry).await?;
        }

        tracing::info!(
            "Conflict {} resolved with strategy {:?} by {}",
            conflict.id, conflict.resolution_status, conflict.resolved_by.as_deref().unwrap_or("system")
        );

        Ok(conflict)
    }

    pub async fn auto_resolve_conflict(
        &self,
        conflict: &SyncConflict,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let resolution_data = match self.config.conflict_resolution_strategy {
            ConflictResolutionStrategy::LastWriteWins => {
                // Use the data with the most recent timestamp
                conflict.client_data.clone()
            }
            ConflictResolutionStrategy::ServerWins => {
                // Always use server data
                conflict.server_data.clone()
            }
            ConflictResolutionStrategy::ClientWins => {
                // Always use client data
                conflict.client_data.clone()
            }
            ConflictResolutionStrategy::ManualResolution => {
                // Cannot auto-resolve, return error
                return Err("Manual resolution required".into());
            }
        };

        Ok(resolution_data)
    }

    pub async fn get_pending_conflicts(
        &self,
        device_id: &str,
    ) -> Result<Vec<SyncConflict>, Box<dyn std::error::Error>> {
        Ok(self.repository.get_pending_conflicts(device_id).await?)
    }

    // ============================================================================
    // Device Management
    // ============================================================================

    pub async fn register_device(
        &self,
        input: RegisterDeviceInput,
    ) -> Result<SyncDevice, Box<dyn std::error::Error>> {
        let device = SyncDevice::new(
            input.device_id,
            input.device_name,
            input.device_type,
            input.user_id,
            input.organization_id,
        );

        let result = self.repository.register_device(&device).await?;

        tracing::info!("Device registered: {} ({})", result.device_name, result.device_id);

        Ok(result)
    }

    pub async fn update_network_status(
        &self,
        input: UpdateNetworkStatusInput,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.repository
            .update_network_status(&input.device_id, input.network_status)
            .await?;

        tracing::info!(
            "Network status updated for device {}: {:?}",
            input.device_id, input.network_status
        );

        Ok(())
    }

    pub async fn get_sync_status(
        &self,
        device_id: &str,
    ) -> Result<SyncStatusResponse, Box<dyn std::error::Error>> {
        // Try to get from cache first
        if let Some(cached) = self.repository.get_cached_sync_status(device_id).await? {
            return Ok(cached);
        }

        // Get device info
        let device = self.repository.get_device(device_id).await?
            .ok_or("Device not found")?;

        // Get queue stats
        let stats = self.repository.get_queue_stats(device_id).await?;

        let response = SyncStatusResponse {
            device_id: device_id.to_string(),
            last_sync_at: device.last_sync_at,
            pending_count: stats.pending_count,
            conflict_count: stats.conflict_count,
            offline_mode: device.offline_mode,
            network_status: device.network_status,
        };

        // Cache the response
        self.repository.cache_sync_status(device_id, &response).await?;

        Ok(response)
    }
}

enum ProcessResult {
    Success,
    Conflict(SyncConflict),
}
