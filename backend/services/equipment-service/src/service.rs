use uuid::Uuid;
use common::error::{Error, Result};
use common::pagination::{Paginated, PaginationParams};
use crate::domain::*;
use crate::repository::*;

// ============================================================================
// Equipment Service
// ============================================================================

#[derive(Clone)]
pub struct EquipmentService {
    equipment_repo: EquipmentRepository,
    maintenance_repo: EquipmentMaintenanceRepository,
    calibration_repo: EquipmentCalibrationRepository,
    test_assignment_repo: EquipmentTestAssignmentRepository,
    performance_log_repo: EquipmentPerformanceLogRepository,
    alert_repo: EquipmentAlertRepository,
}

impl EquipmentService {
    pub fn new(
        equipment_repo: EquipmentRepository,
        maintenance_repo: EquipmentMaintenanceRepository,
        calibration_repo: EquipmentCalibrationRepository,
        test_assignment_repo: EquipmentTestAssignmentRepository,
        performance_log_repo: EquipmentPerformanceLogRepository,
        alert_repo: EquipmentAlertRepository,
    ) -> Self {
        Self {
            equipment_repo,
            maintenance_repo,
            calibration_repo,
            test_assignment_repo,
            performance_log_repo,
            alert_repo,
        }
    }

    // ========================================================================
    // Equipment Operations
    // ========================================================================

    pub async fn create_equipment(
        &self,
        input: CreateEquipmentInput,
        created_by: Uuid,
    ) -> Result<Equipment> {
        // Validate equipment name
        if input.equipment_name.trim().is_empty() {
            return Err(Error::Validation(
                "Equipment name cannot be empty".to_string(),
            ));
        }

        // Validate serial number uniqueness if provided
        if let Some(ref serial) = input.serial_number {
            if !serial.trim().is_empty() {
                // TODO: Check serial number uniqueness
            }
        }

        // Validate frequency values
        if let Some(freq) = input.maintenance_frequency_days {
            if freq <= 0 {
                return Err(Error::Validation(
                    "Maintenance frequency must be greater than 0".to_string(),
                ));
            }
        }

        if let Some(freq) = input.calibration_frequency_days {
            if freq <= 0 {
                return Err(Error::Validation(
                    "Calibration frequency must be greater than 0".to_string(),
                ));
            }
        }

        let equipment = self.equipment_repo.create(input, created_by).await?;

        tracing::info!(
            "Equipment created: {} ({})",
            equipment.equipment_name,
            equipment.equipment_code
        );

        Ok(equipment)
    }

    pub async fn get_equipment(&self, id: Uuid) -> Result<Equipment> {
        self.equipment_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Equipment not found".to_string()))
    }

    pub async fn get_equipment_by_code(&self, equipment_code: String) -> Result<Equipment> {
        self.equipment_repo
            .find_by_code(&equipment_code)
            .await?
            .ok_or_else(|| Error::NotFound("Equipment not found".to_string()))
    }

    pub async fn list_equipment(
        &self,
        filter: EquipmentFilter,
        pagination: PaginationParams,
    ) -> Result<Paginated<Equipment>> {
        self.equipment_repo.list(filter, pagination).await
    }

    pub async fn update_equipment(
        &self,
        input: UpdateEquipmentInput,
        updated_by: Uuid,
    ) -> Result<Equipment> {
        // Check if equipment exists
        let _ = self.get_equipment(input.id).await?;

        // Validate frequency values if provided
        if let Some(freq) = input.maintenance_frequency_days {
            if freq <= 0 {
                return Err(Error::Validation(
                    "Maintenance frequency must be greater than 0".to_string(),
                ));
            }
        }

        if let Some(freq) = input.calibration_frequency_days {
            if freq <= 0 {
                return Err(Error::Validation(
                    "Calibration frequency must be greater than 0".to_string(),
                ));
            }
        }

        let equipment = self.equipment_repo.update(input, updated_by).await?;

        tracing::info!(
            "Equipment updated: {} ({})",
            equipment.equipment_name,
            equipment.equipment_code
        );

        Ok(equipment)
    }

    pub async fn update_equipment_status(
        &self,
        input: UpdateEquipmentStatusInput,
        updated_by: Uuid,
    ) -> Result<Equipment> {
        // Check if equipment exists
        let existing = self.get_equipment(input.id).await?;

        // Validate status transition
        if existing.equipment_status == EquipmentStatus::Retired
            && input.status != EquipmentStatus::Retired
        {
            return Err(Error::Validation(
                "Cannot change status of retired equipment".to_string(),
            ));
        }

        let equipment = self.equipment_repo.update_status(input, updated_by).await?;

        tracing::info!(
            "Equipment status updated: {} -> {:?}",
            equipment.equipment_code,
            equipment.equipment_status
        );

        Ok(equipment)
    }

    pub async fn delete_equipment(&self, id: Uuid, deleted_by: Uuid) -> Result<bool> {
        // Check if equipment exists
        let equipment = self.get_equipment(id).await?;

        // Check if equipment has pending maintenance
        let pending_maintenance = self
            .maintenance_repo
            .list_by_filter(MaintenanceFilter {
                equipment_id: Some(id),
                maintenance_type: None,
                maintenance_status: Some(MaintenanceStatus::Scheduled),
                from_date: None,
                to_date: None,
            })
            .await?;

        if !pending_maintenance.is_empty() {
            return Err(Error::Validation(
                "Cannot delete equipment with pending maintenance".to_string(),
            ));
        }

        let deleted = self.equipment_repo.delete(id, deleted_by).await?;

        if deleted {
            tracing::info!(
                "Equipment deleted: {} ({})",
                equipment.equipment_name,
                equipment.equipment_code
            );
        }

        Ok(deleted)
    }

    pub async fn increment_test_counter(&self, equipment_id: Uuid) -> Result<()> {
        let equipment = self.get_equipment(equipment_id).await?;

        if !equipment.is_operational() {
            return Err(Error::Validation(
                "Equipment is not operational".to_string(),
            ));
        }

        self.equipment_repo.increment_test_counter(equipment_id).await
    }

    // ========================================================================
    // Maintenance Operations
    // ========================================================================

    pub async fn schedule_maintenance(
        &self,
        input: ScheduleMaintenanceInput,
        created_by: Uuid,
    ) -> Result<EquipmentMaintenance> {
        // Check if equipment exists
        let equipment = self.get_equipment(input.equipment_id).await?;

        // Validate scheduled date
        if input.scheduled_date < chrono::Local::now().date_naive() {
            return Err(Error::Validation(
                "Scheduled date cannot be in the past".to_string(),
            ));
        }

        let maintenance = self.maintenance_repo.create(input, created_by).await?;

        tracing::info!(
            "Maintenance scheduled for equipment: {} on {}",
            equipment.equipment_code,
            maintenance.scheduled_date
        );

        Ok(maintenance)
    }

    pub async fn get_maintenance(&self, id: Uuid) -> Result<EquipmentMaintenance> {
        self.maintenance_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Maintenance record not found".to_string()))
    }

    pub async fn list_maintenance(&self, filter: MaintenanceFilter) -> Result<Vec<EquipmentMaintenance>> {
        self.maintenance_repo.list_by_filter(filter).await
    }

    pub async fn complete_maintenance(
        &self,
        input: CompleteMaintenanceInput,
        updated_by: Uuid,
    ) -> Result<EquipmentMaintenance> {
        // Check if maintenance exists
        let existing = self.get_maintenance(input.id).await?;

        if existing.is_completed() {
            return Err(Error::Validation(
                "Maintenance is already completed".to_string(),
            ));
        }

        // Validate completed date
        if input.completed_date < existing.scheduled_date {
            return Err(Error::Validation(
                "Completed date cannot be before scheduled date".to_string(),
            ));
        }

        let maintenance = self.maintenance_repo.complete(input, updated_by).await?;

        tracing::info!(
            "Maintenance completed for equipment: {:?}",
            maintenance.equipment_id
        );

        Ok(maintenance)
    }

    pub async fn cancel_maintenance(&self, id: Uuid) -> Result<bool> {
        // Check if maintenance exists
        let existing = self.get_maintenance(id).await?;

        if existing.is_completed() {
            return Err(Error::Validation(
                "Cannot cancel completed maintenance".to_string(),
            ));
        }

        let cancelled = self.maintenance_repo.cancel(id).await?;

        if cancelled {
            tracing::info!("Maintenance cancelled: {}", id);
        }

        Ok(cancelled)
    }

    // ========================================================================
    // Calibration Operations
    // ========================================================================

    pub async fn record_calibration(
        &self,
        input: RecordCalibrationInput,
        created_by: Uuid,
    ) -> Result<EquipmentCalibration> {
        // Check if equipment exists
        let equipment = self.get_equipment(input.equipment_id).await?;

        // Validate dates
        if input.due_date <= input.calibration_date {
            return Err(Error::Validation(
                "Due date must be after calibration date".to_string(),
            ));
        }

        // Validate accuracy values
        if let (Some(before), Some(after)) = (input.before_accuracy, input.after_accuracy) {
            if before < rust_decimal::Decimal::ZERO
                || after < rust_decimal::Decimal::ZERO
                || before > rust_decimal::Decimal::from(100)
                || after > rust_decimal::Decimal::from(100)
            {
                return Err(Error::Validation(
                    "Accuracy must be between 0 and 100".to_string(),
                ));
            }
        }

        let calibration = self.calibration_repo.create(input, created_by).await?;

        tracing::info!(
            "Calibration recorded for equipment: {} - Status: {:?}",
            equipment.equipment_code,
            calibration.calibration_status
        );

        Ok(calibration)
    }

    pub async fn get_calibration(&self, id: Uuid) -> Result<EquipmentCalibration> {
        self.calibration_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Calibration record not found".to_string()))
    }

    pub async fn list_calibrations(&self, filter: CalibrationFilter) -> Result<Vec<EquipmentCalibration>> {
        self.calibration_repo.list_by_filter(filter).await
    }

    // ========================================================================
    // Test Assignment Operations
    // ========================================================================

    pub async fn assign_test(
        &self,
        input: AssignTestInput,
        created_by: Uuid,
    ) -> Result<EquipmentTestAssignment> {
        // Check if equipment exists and is operational
        let equipment = self.get_equipment(input.equipment_id).await?;

        if !equipment.is_operational() {
            return Err(Error::Validation(
                "Cannot assign tests to non-operational equipment".to_string(),
            ));
        }

        let assignment = self.test_assignment_repo.create(input, created_by).await?;

        tracing::info!(
            "Test assigned to equipment: {} - Test ID: {}",
            equipment.equipment_code,
            assignment.test_id
        );

        Ok(assignment)
    }

    pub async fn list_test_assignments(&self, equipment_id: Uuid) -> Result<Vec<EquipmentTestAssignment>> {
        // Verify equipment exists
        let _ = self.get_equipment(equipment_id).await?;

        self.test_assignment_repo.list_by_equipment(equipment_id).await
    }

    pub async fn unassign_test(&self, equipment_id: Uuid, test_id: Uuid) -> Result<bool> {
        // Verify equipment exists
        let _ = self.get_equipment(equipment_id).await?;

        let unassigned = self
            .test_assignment_repo
            .unassign(equipment_id, test_id)
            .await?;

        if unassigned {
            tracing::info!("Test unassigned from equipment: {}", equipment_id);
        }

        Ok(unassigned)
    }

    // ========================================================================
    // Performance Logging Operations
    // ========================================================================

    pub async fn log_performance(&self, input: LogPerformanceInput) -> Result<EquipmentPerformanceLog> {
        // Verify equipment exists
        let equipment = self.get_equipment(input.equipment_id).await?;

        // Validate data
        if input.tests_processed < 0 {
            return Err(Error::Validation(
                "Tests processed cannot be negative".to_string(),
            ));
        }

        if input.tests_failed < 0 {
            return Err(Error::Validation(
                "Tests failed cannot be negative".to_string(),
            ));
        }

        if input.tests_failed > input.tests_processed {
            return Err(Error::Validation(
                "Tests failed cannot exceed tests processed".to_string(),
            ));
        }

        if input.downtime_minutes < 0 {
            return Err(Error::Validation(
                "Downtime cannot be negative".to_string(),
            ));
        }

        let log = self.performance_log_repo.create(input).await?;

        tracing::info!(
            "Performance logged for equipment: {} on {}",
            equipment.equipment_code,
            log.log_date
        );

        Ok(log)
    }

    pub async fn get_performance_logs(
        &self,
        equipment_id: Uuid,
        limit: i32,
    ) -> Result<Vec<EquipmentPerformanceLog>> {
        // Verify equipment exists
        let _ = self.get_equipment(equipment_id).await?;

        self.performance_log_repo
            .list_by_equipment(equipment_id, limit)
            .await
    }

    // ========================================================================
    // Alert Operations
    // ========================================================================

    pub async fn get_alert(&self, id: Uuid) -> Result<EquipmentAlert> {
        self.alert_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Alert not found".to_string()))
    }

    pub async fn list_alerts(&self, filter: AlertFilter) -> Result<Vec<EquipmentAlert>> {
        self.alert_repo.list_by_filter(filter).await
    }

    pub async fn acknowledge_alert(&self, input: AcknowledgeAlertInput, acknowledged_by: Uuid) -> Result<EquipmentAlert> {
        // Check if alert exists
        let existing = self.get_alert(input.id).await?;

        if existing.is_acknowledged.unwrap_or(false) {
            return Err(Error::Validation(
                "Alert is already acknowledged".to_string(),
            ));
        }

        let alert = self.alert_repo.acknowledge(input.id, acknowledged_by).await?;

        tracing::info!("Alert acknowledged: {}", alert.id);

        Ok(alert)
    }

    pub async fn resolve_alert(&self, input: ResolveAlertInput, resolved_by: Uuid) -> Result<EquipmentAlert> {
        // Check if alert exists
        let existing = self.get_alert(input.id).await?;

        if existing.is_resolved.unwrap_or(false) {
            return Err(Error::Validation("Alert is already resolved".to_string()));
        }

        let alert = self.alert_repo.resolve(input, resolved_by).await?;

        tracing::info!("Alert resolved: {}", alert.id);

        Ok(alert)
    }
}
