use async_graphql::{Context, Object, Result, ErrorExtensions};
use uuid::Uuid;
use common::pagination::PaginationParams;
use crate::domain::*;
use crate::service::EquipmentService;

// ============================================================================
// Query Root
// ============================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get equipment by ID
    async fn equipment(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<Equipment>> {
        let service = ctx.data::<EquipmentService>()?;
        let equipment = service.get_equipment(id).await?;
        Ok(Some(equipment))
    }

    /// Get equipment by code
    async fn equipment_by_code(
        &self,
        ctx: &Context<'_>,
        equipment_code: String,
    ) -> Result<Option<Equipment>> {
        let service = ctx.data::<EquipmentService>()?;
        let equipment = service.get_equipment_by_code(equipment_code).await?;
        Ok(Some(equipment))
    }

    /// List equipment with filtering and pagination
    async fn equipment_list(
        &self,
        ctx: &Context<'_>,
        filter: EquipmentFilter,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<EquipmentPaginated> {
        let service = ctx.data::<EquipmentService>()?;

        let pagination = PaginationParams {
            page: page.unwrap_or(1) as u32,
            page_size: page_size.unwrap_or(20) as u32,
        };

        let paginated = service.list_equipment(filter, pagination).await?;

        Ok(EquipmentPaginated {
            data: paginated.edges.into_iter().map(|edge| edge.node).collect(),
            total: paginated.page_info.total_count as i32,
            page: paginated.page_info.current_page as i32,
            page_size: paginated.page_info.page_size as i32,
            total_pages: paginated.page_info.total_pages as i32,
        })
    }

    /// Get maintenance record by ID
    async fn maintenance(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<EquipmentMaintenance>> {
        let service = ctx.data::<EquipmentService>()?;
        let maintenance = service.get_maintenance(id).await?;
        Ok(Some(maintenance))
    }

    /// List maintenance records with filtering
    async fn maintenance_list(
        &self,
        ctx: &Context<'_>,
        filter: MaintenanceFilter,
    ) -> Result<Vec<EquipmentMaintenance>> {
        let service = ctx.data::<EquipmentService>()?;
        Ok(service.list_maintenance(filter).await.map_err(|e| e.extend())?)
    }

    /// Get calibration record by ID
    async fn calibration(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<EquipmentCalibration>> {
        let service = ctx.data::<EquipmentService>()?;
        let calibration = service.get_calibration(id).await?;
        Ok(Some(calibration))
    }

    /// List calibration records with filtering
    async fn calibration_list(
        &self,
        ctx: &Context<'_>,
        filter: CalibrationFilter,
    ) -> Result<Vec<EquipmentCalibration>> {
        let service = ctx.data::<EquipmentService>()?;
        Ok(service.list_calibrations(filter).await.map_err(|e| e.extend())?)
    }

    /// List test assignments for equipment
    async fn test_assignments(
        &self,
        ctx: &Context<'_>,
        equipment_id: Uuid,
    ) -> Result<Vec<EquipmentTestAssignment>> {
        let service = ctx.data::<EquipmentService>()?;
        Ok(service.list_test_assignments(equipment_id).await.map_err(|e| e.extend())?)
    }

    /// Get performance logs for equipment
    async fn performance_logs(
        &self,
        ctx: &Context<'_>,
        equipment_id: Uuid,
        limit: Option<i32>,
    ) -> Result<Vec<EquipmentPerformanceLog>> {
        let service = ctx.data::<EquipmentService>()?;
        Ok(service.get_performance_logs(equipment_id, limit.unwrap_or(30)).await.map_err(|e| e.extend())?)
    }

    /// Get alert by ID
    async fn alert(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<EquipmentAlert>> {
        let service = ctx.data::<EquipmentService>()?;
        let alert = service.get_alert(id).await?;
        Ok(Some(alert))
    }

    /// List alerts with filtering
    async fn alert_list(
        &self,
        ctx: &Context<'_>,
        filter: AlertFilter,
    ) -> Result<Vec<EquipmentAlert>> {
        let service = ctx.data::<EquipmentService>()?;
        Ok(service.list_alerts(filter).await.map_err(|e| e.extend())?)
    }
}

// ============================================================================
// Mutation Root
// ============================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create new equipment
    async fn create_equipment(
        &self,
        ctx: &Context<'_>,
        input: CreateEquipmentInput,
    ) -> Result<Equipment> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.create_equipment(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Update equipment
    async fn update_equipment(
        &self,
        ctx: &Context<'_>,
        input: UpdateEquipmentInput,
    ) -> Result<Equipment> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_equipment(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Update equipment status
    async fn update_equipment_status(
        &self,
        ctx: &Context<'_>,
        input: UpdateEquipmentStatusInput,
    ) -> Result<Equipment> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_equipment_status(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Delete equipment (soft delete)
    async fn delete_equipment(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get deleted_by from authenticated user context
        let deleted_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.delete_equipment(id, deleted_by).await.map_err(|e| e.extend())?)
    }

    /// Schedule maintenance
    async fn schedule_maintenance(
        &self,
        ctx: &Context<'_>,
        input: ScheduleMaintenanceInput,
    ) -> Result<EquipmentMaintenance> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.schedule_maintenance(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Complete maintenance
    async fn complete_maintenance(
        &self,
        ctx: &Context<'_>,
        input: CompleteMaintenanceInput,
    ) -> Result<EquipmentMaintenance> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.complete_maintenance(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Cancel maintenance
    async fn cancel_maintenance(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<EquipmentService>()?;
        Ok(service.cancel_maintenance(id).await.map_err(|e| e.extend())?)
    }

    /// Record calibration
    async fn record_calibration(
        &self,
        ctx: &Context<'_>,
        input: RecordCalibrationInput,
    ) -> Result<EquipmentCalibration> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.record_calibration(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Assign test to equipment
    async fn assign_test(
        &self,
        ctx: &Context<'_>,
        input: AssignTestInput,
    ) -> Result<EquipmentTestAssignment> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.assign_test(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Unassign test from equipment
    async fn unassign_test(
        &self,
        ctx: &Context<'_>,
        equipment_id: Uuid,
        test_id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<EquipmentService>()?;
        Ok(service.unassign_test(equipment_id, test_id).await.map_err(|e| e.extend())?)
    }

    /// Log equipment performance
    async fn log_performance(
        &self,
        ctx: &Context<'_>,
        input: LogPerformanceInput,
    ) -> Result<EquipmentPerformanceLog> {
        let service = ctx.data::<EquipmentService>()?;
        Ok(service.log_performance(input).await.map_err(|e| e.extend())?)
    }

    /// Acknowledge alert
    async fn acknowledge_alert(
        &self,
        ctx: &Context<'_>,
        input: AcknowledgeAlertInput,
    ) -> Result<EquipmentAlert> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get acknowledged_by from authenticated user context
        let acknowledged_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.acknowledge_alert(input, acknowledged_by).await.map_err(|e| e.extend())?)
    }

    /// Resolve alert
    async fn resolve_alert(
        &self,
        ctx: &Context<'_>,
        input: ResolveAlertInput,
    ) -> Result<EquipmentAlert> {
        let service = ctx.data::<EquipmentService>()?;

        // In production, get resolved_by from authenticated user context
        let resolved_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.resolve_alert(input, resolved_by).await.map_err(|e| e.extend())?)
    }

    /// Increment test counter (called by other services when processing tests)
    async fn increment_test_counter(
        &self,
        ctx: &Context<'_>,
        equipment_id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<EquipmentService>()?;
        service.increment_test_counter(equipment_id).await?;
        Ok(true)
    }
}

// ============================================================================
// GraphQL Types
// ============================================================================

#[derive(async_graphql::SimpleObject)]
pub struct EquipmentPaginated {
    pub data: Vec<Equipment>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}
