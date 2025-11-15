use async_graphql::{Context, Object, Result};

use crate::domain::*;
use crate::service::SyncService;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get sync status for a device
    async fn sync_status(
        &self,
        ctx: &Context<'_>,
        device_id: String,
    ) -> Result<SyncStatusResponse> {
        let service = ctx.data::<SyncService>()?;
        Ok(service.get_sync_status(&device_id).await?)
    }

    /// Get pending sync operations for a device
    async fn pending_operations(
        &self,
        ctx: &Context<'_>,
        device_id: String,
        #[graphql(default = 100)] limit: i64,
    ) -> Result<Vec<SyncQueueEntry>> {
        let service = ctx.data::<SyncService>()?;
        Ok(service.repository.get_pending_operations(&device_id, limit).await?)
    }

    /// Get pending conflicts for a device
    async fn pending_conflicts(
        &self,
        ctx: &Context<'_>,
        device_id: String,
    ) -> Result<Vec<SyncConflict>> {
        let service = ctx.data::<SyncService>()?;
        Ok(service.get_pending_conflicts(&device_id).await?)
    }

    /// Get device information
    async fn device(
        &self,
        ctx: &Context<'_>,
        device_id: String,
    ) -> Result<Option<SyncDevice>> {
        let service = ctx.data::<SyncService>()?;
        Ok(service.repository.get_device(&device_id).await?)
    }

    /// Get queue statistics for a device
    async fn queue_stats(
        &self,
        ctx: &Context<'_>,
        device_id: String,
    ) -> Result<QueueStatsResponse> {
        let service = ctx.data::<SyncService>()?;
        let stats = service.repository.get_queue_stats(&device_id).await?;

        Ok(QueueStatsResponse {
            pending_count: stats.pending_count,
            completed_count: stats.completed_count,
            failed_count: stats.failed_count,
            conflict_count: stats.conflict_count,
        })
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Queue an operation for offline sync
    async fn queue_operation(
        &self,
        ctx: &Context<'_>,
        input: QueueOperationInput,
    ) -> Result<SyncQueueEntry> {
        let service = ctx.data::<SyncService>()?;
        Ok(service.queue_operation(input).await?)
    }

    /// Sync all pending operations for a device
    async fn sync_pending_operations(
        &self,
        ctx: &Context<'_>,
        device_id: String,
    ) -> Result<SyncResponse> {
        let service = ctx.data::<SyncService>()?;
        Ok(service.sync_pending_operations(&device_id).await?)
    }

    /// Resolve a conflict manually
    async fn resolve_conflict(
        &self,
        ctx: &Context<'_>,
        input: ResolveConflictInput,
    ) -> Result<SyncConflict> {
        let service = ctx.data::<SyncService>()?;
        Ok(service.resolve_conflict(input).await?)
    }

    /// Register a new device for sync
    async fn register_device(
        &self,
        ctx: &Context<'_>,
        input: RegisterDeviceInput,
    ) -> Result<SyncDevice> {
        let service = ctx.data::<SyncService>()?;
        Ok(service.register_device(input).await?)
    }

    /// Update network status for a device
    async fn update_network_status(
        &self,
        ctx: &Context<'_>,
        input: UpdateNetworkStatusInput,
    ) -> Result<bool> {
        let service = ctx.data::<SyncService>()?;
        service.update_network_status(input).await?;
        Ok(true)
    }
}

// Response types

use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct QueueStatsResponse {
    pub pending_count: i32,
    pub completed_count: i32,
    pub failed_count: i32,
    pub conflict_count: i32,
}
