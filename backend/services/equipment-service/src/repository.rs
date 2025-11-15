use sqlx::PgPool;
use uuid::Uuid;
use common::error::{Error, Result};
use common::pagination::{Paginated, PaginationParams};
use crate::domain::*;

// ============================================================================
// Equipment Repository
// ============================================================================

#[derive(Clone)]
pub struct EquipmentRepository {
    pool: PgPool,
}

impl EquipmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateEquipmentInput, created_by: Uuid) -> Result<Equipment> {
        let id = Uuid::new_v4();

        // Generate equipment code
        let equipment_code: (String,) = sqlx::query_as("SELECT generate_equipment_code()")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        let equipment = sqlx::query_as::<_, Equipment>(
            r#"
            INSERT INTO equipment (
                id, equipment_code, equipment_name, equipment_type,
                manufacturer, model_number, serial_number,
                equipment_status,
                organization_id, branch_id, department_id, location,
                purchase_date, purchase_cost, vendor, warranty_expiry_date,
                installation_date, commissioning_date,
                capacity, interface_type,
                maintenance_frequency_days, calibration_frequency_days,
                lis_integration_enabled,
                created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(equipment_code.0)
        .bind(&input.equipment_name)
        .bind(&input.equipment_type)
        .bind(&input.manufacturer)
        .bind(&input.model_number)
        .bind(&input.serial_number)
        .bind(EquipmentStatus::Inactive) // Default to inactive until commissioned
        .bind(input.organization_id)
        .bind(input.branch_id)
        .bind(input.department_id)
        .bind(&input.location)
        .bind(input.purchase_date)
        .bind(input.purchase_cost)
        .bind(&input.vendor)
        .bind(input.warranty_expiry_date)
        .bind(input.installation_date)
        .bind(input.commissioning_date)
        .bind(&input.capacity)
        .bind(&input.interface_type)
        .bind(input.maintenance_frequency_days)
        .bind(input.calibration_frequency_days)
        .bind(input.lis_integration_enabled.unwrap_or(false))
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(equipment)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Equipment>> {
        let equipment = sqlx::query_as::<_, Equipment>(
            "SELECT * FROM equipment WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(equipment)
    }

    pub async fn find_by_code(&self, equipment_code: &str) -> Result<Option<Equipment>> {
        let equipment = sqlx::query_as::<_, Equipment>(
            "SELECT * FROM equipment WHERE equipment_code = $1 AND is_deleted = FALSE"
        )
        .bind(equipment_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(equipment)
    }

    pub async fn list(
        &self,
        filter: EquipmentFilter,
        pagination: PaginationParams,
    ) -> Result<Paginated<Equipment>> {
        let mut query = String::from(
            "FROM equipment WHERE organization_id = $1 AND is_deleted = FALSE"
        );
        let mut bindings = vec![];

        if let Some(status) = filter.equipment_status {
            bindings.push(format!("equipment_status = '{:?}'", status));
        }
        if let Some(eq_type) = filter.equipment_type {
            bindings.push(format!("equipment_type = '{:?}'", eq_type));
        }
        if let Some(branch_id) = filter.branch_id {
            bindings.push(format!("branch_id = '{}'", branch_id));
        }
        if let Some(dept_id) = filter.department_id {
            bindings.push(format!("department_id = '{}'", dept_id));
        }
        if let Some(search) = filter.search_query {
            bindings.push(format!(
                "(equipment_name ILIKE '%{}%' OR manufacturer ILIKE '%{}%' OR model_number ILIKE '%{}%')",
                search, search, search
            ));
        }

        if !bindings.is_empty() {
            let where_clause = format!(" AND {}", bindings.join(" AND "));
            query.push_str(&where_clause);
        }

        // Get total count
        let count_query = format!("SELECT COUNT(*) {}", query);
        let total: (i64,) = sqlx::query_as(&count_query)
            .bind(filter.organization_id)
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        // Get paginated results
        let select_query = format!(
            "SELECT * {} ORDER BY equipment_name ASC LIMIT $2 OFFSET $3",
            query
        );

        let equipment = sqlx::query_as::<_, Equipment>(&select_query)
            .bind(filter.organization_id)
            .bind(pagination.page_size as i64)
            .bind(((pagination.page - 1) * pagination.page_size) as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(Paginated::new(
            equipment,
            &pagination,
            total.0 as u64,
        ))
    }

    pub async fn update(&self, input: UpdateEquipmentInput, updated_by: Uuid) -> Result<Equipment> {
        let equipment = sqlx::query_as::<_, Equipment>(
            r#"
            UPDATE equipment
            SET equipment_name = COALESCE($2, equipment_name),
                location = COALESCE($3, location),
                capacity = COALESCE($4, capacity),
                warranty_expiry_date = COALESCE($5, warranty_expiry_date),
                interface_type = COALESCE($6, interface_type),
                ip_address = COALESCE($7, ip_address),
                mac_address = COALESCE($8, mac_address),
                lis_integration_enabled = COALESCE($9, lis_integration_enabled),
                maintenance_frequency_days = COALESCE($10, maintenance_frequency_days),
                calibration_frequency_days = COALESCE($11, calibration_frequency_days),
                user_manual_url = COALESCE($12, user_manual_url),
                service_manual_url = COALESCE($13, service_manual_url),
                sop_document_url = COALESCE($14, sop_document_url),
                notes = COALESCE($15, notes),
                updated_by = $16
            WHERE id = $1 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.equipment_name)
        .bind(input.location)
        .bind(input.capacity)
        .bind(input.warranty_expiry_date)
        .bind(input.interface_type)
        .bind(input.ip_address)
        .bind(input.mac_address)
        .bind(input.lis_integration_enabled)
        .bind(input.maintenance_frequency_days)
        .bind(input.calibration_frequency_days)
        .bind(input.user_manual_url)
        .bind(input.service_manual_url)
        .bind(input.sop_document_url)
        .bind(input.notes)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(equipment)
    }

    pub async fn update_status(&self, input: UpdateEquipmentStatusInput, updated_by: Uuid) -> Result<Equipment> {
        let equipment = sqlx::query_as::<_, Equipment>(
            "UPDATE equipment SET equipment_status = $2, updated_by = $3 WHERE id = $1 AND is_deleted = FALSE RETURNING *"
        )
        .bind(input.id)
        .bind(input.status)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(equipment)
    }

    pub async fn delete(&self, id: Uuid, deleted_by: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE equipment SET is_deleted = TRUE, deleted_at = NOW(), deleted_by = $2 WHERE id = $1"
        )
        .bind(id)
        .bind(deleted_by)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn increment_test_counter(&self, equipment_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE equipment SET total_tests_processed = total_tests_processed + 1 WHERE id = $1"
        )
        .bind(equipment_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }
}

// ============================================================================
// Equipment Maintenance Repository
// ============================================================================

#[derive(Clone)]
pub struct EquipmentMaintenanceRepository {
    pool: PgPool,
}

impl EquipmentMaintenanceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: ScheduleMaintenanceInput, created_by: Uuid) -> Result<EquipmentMaintenance> {
        let id = Uuid::new_v4();

        let maintenance = sqlx::query_as::<_, EquipmentMaintenance>(
            r#"
            INSERT INTO equipment_maintenance (
                id, equipment_id, maintenance_type, maintenance_status,
                scheduled_date, technician_id, technician_name, vendor_name,
                work_description, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.equipment_id)
        .bind(input.maintenance_type)
        .bind(MaintenanceStatus::Scheduled)
        .bind(input.scheduled_date)
        .bind(input.technician_id)
        .bind(&input.technician_name)
        .bind(&input.vendor_name)
        .bind(&input.work_description)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(maintenance)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<EquipmentMaintenance>> {
        let maintenance = sqlx::query_as::<_, EquipmentMaintenance>(
            "SELECT * FROM equipment_maintenance WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(maintenance)
    }

    pub async fn list_by_filter(&self, filter: MaintenanceFilter) -> Result<Vec<EquipmentMaintenance>> {
        let mut query = String::from("SELECT * FROM equipment_maintenance WHERE 1=1");
        let mut conditions = vec![];

        if let Some(equipment_id) = filter.equipment_id {
            conditions.push(format!("equipment_id = '{}'", equipment_id));
        }
        if let Some(m_type) = filter.maintenance_type {
            conditions.push(format!("maintenance_type = '{:?}'", m_type));
        }
        if let Some(status) = filter.maintenance_status {
            conditions.push(format!("maintenance_status = '{:?}'", status));
        }
        if let Some(from_date) = filter.from_date {
            conditions.push(format!("scheduled_date >= '{}'", from_date));
        }
        if let Some(to_date) = filter.to_date {
            conditions.push(format!("scheduled_date <= '{}'", to_date));
        }

        if !conditions.is_empty() {
            query.push_str(&format!(" AND {}", conditions.join(" AND ")));
        }

        query.push_str(" ORDER BY scheduled_date DESC");

        let maintenance = sqlx::query_as::<_, EquipmentMaintenance>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(maintenance)
    }

    pub async fn complete(&self, input: CompleteMaintenanceInput, updated_by: Uuid) -> Result<EquipmentMaintenance> {
        let maintenance = sqlx::query_as::<_, EquipmentMaintenance>(
            r#"
            UPDATE equipment_maintenance
            SET maintenance_status = $2,
                completed_date = $3,
                work_description = COALESCE($4, work_description),
                parts_replaced = COALESCE($5, parts_replaced),
                cost = COALESCE($6, cost),
                before_condition = COALESCE($7, before_condition),
                after_condition = COALESCE($8, after_condition),
                findings = COALESCE($9, findings),
                recommendations = COALESCE($10, recommendations),
                next_maintenance_date = COALESCE($11, next_maintenance_date),
                report_url = COALESCE($12, report_url)
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(MaintenanceStatus::Completed)
        .bind(input.completed_date)
        .bind(input.work_description)
        .bind(input.parts_replaced)
        .bind(input.cost)
        .bind(input.before_condition)
        .bind(input.after_condition)
        .bind(input.findings)
        .bind(input.recommendations)
        .bind(input.next_maintenance_date)
        .bind(input.report_url)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(maintenance)
    }

    pub async fn cancel(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE equipment_maintenance SET maintenance_status = $2 WHERE id = $1"
        )
        .bind(id)
        .bind(MaintenanceStatus::Cancelled)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }
}

// ============================================================================
// Equipment Calibration Repository
// ============================================================================

#[derive(Clone)]
pub struct EquipmentCalibrationRepository {
    pool: PgPool,
}

impl EquipmentCalibrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: RecordCalibrationInput, created_by: Uuid) -> Result<EquipmentCalibration> {
        let id = Uuid::new_v4();

        let calibration = sqlx::query_as::<_, EquipmentCalibration>(
            r#"
            INSERT INTO equipment_calibration (
                id, equipment_id, calibration_date, due_date, calibration_status,
                performed_by_id, performed_by_name, calibration_agency,
                certificate_number, certificate_url,
                before_accuracy, after_accuracy, within_specification,
                deviations_found, corrective_actions,
                next_calibration_date, cost, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.equipment_id)
        .bind(input.calibration_date)
        .bind(input.due_date)
        .bind(input.calibration_status)
        .bind(input.performed_by_id)
        .bind(&input.performed_by_name)
        .bind(&input.calibration_agency)
        .bind(&input.certificate_number)
        .bind(&input.certificate_url)
        .bind(input.before_accuracy)
        .bind(input.after_accuracy)
        .bind(input.within_specification)
        .bind(&input.deviations_found)
        .bind(&input.corrective_actions)
        .bind(input.next_calibration_date)
        .bind(input.cost)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(calibration)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<EquipmentCalibration>> {
        let calibration = sqlx::query_as::<_, EquipmentCalibration>(
            "SELECT * FROM equipment_calibration WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(calibration)
    }

    pub async fn list_by_filter(&self, filter: CalibrationFilter) -> Result<Vec<EquipmentCalibration>> {
        let mut query = String::from("SELECT * FROM equipment_calibration WHERE 1=1");
        let mut conditions = vec![];

        if let Some(equipment_id) = filter.equipment_id {
            conditions.push(format!("equipment_id = '{}'", equipment_id));
        }
        if let Some(status) = filter.calibration_status {
            conditions.push(format!("calibration_status = '{:?}'", status));
        }
        if let Some(from_date) = filter.from_date {
            conditions.push(format!("calibration_date >= '{}'", from_date));
        }
        if let Some(to_date) = filter.to_date {
            conditions.push(format!("calibration_date <= '{}'", to_date));
        }

        if !conditions.is_empty() {
            query.push_str(&format!(" AND {}", conditions.join(" AND ")));
        }

        query.push_str(" ORDER BY calibration_date DESC");

        let calibrations = sqlx::query_as::<_, EquipmentCalibration>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(calibrations)
    }
}

// ============================================================================
// Equipment Test Assignment Repository
// ============================================================================

#[derive(Clone)]
pub struct EquipmentTestAssignmentRepository {
    pool: PgPool,
}

impl EquipmentTestAssignmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: AssignTestInput, created_by: Uuid) -> Result<EquipmentTestAssignment> {
        let id = Uuid::new_v4();

        let assignment = sqlx::query_as::<_, EquipmentTestAssignment>(
            r#"
            INSERT INTO equipment_test_assignment (
                id, equipment_id, test_id, is_primary, is_backup, is_active, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.equipment_id)
        .bind(input.test_id)
        .bind(input.is_primary.unwrap_or(false))
        .bind(input.is_backup.unwrap_or(false))
        .bind(true)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(assignment)
    }

    pub async fn list_by_equipment(&self, equipment_id: Uuid) -> Result<Vec<EquipmentTestAssignment>> {
        let assignments = sqlx::query_as::<_, EquipmentTestAssignment>(
            "SELECT * FROM equipment_test_assignment WHERE equipment_id = $1 AND is_active = TRUE"
        )
        .bind(equipment_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(assignments)
    }

    pub async fn unassign(&self, equipment_id: Uuid, test_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE equipment_test_assignment SET is_active = FALSE WHERE equipment_id = $1 AND test_id = $2"
        )
        .bind(equipment_id)
        .bind(test_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }
}

// ============================================================================
// Equipment Performance Log Repository
// ============================================================================

#[derive(Clone)]
pub struct EquipmentPerformanceLogRepository {
    pool: PgPool,
}

impl EquipmentPerformanceLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: LogPerformanceInput) -> Result<EquipmentPerformanceLog> {
        let id = Uuid::new_v4();

        let success_rate = if input.tests_processed > 0 {
            ((input.tests_processed - input.tests_failed) as f64 / input.tests_processed as f64) * 100.0
        } else {
            0.0
        };

        let error_rate = if input.tests_processed > 0 {
            (input.tests_failed as f64 / input.tests_processed as f64) * 100.0
        } else {
            0.0
        };

        let log = sqlx::query_as::<_, EquipmentPerformanceLog>(
            r#"
            INSERT INTO equipment_performance_log (
                id, equipment_id, log_date,
                tests_processed, tests_failed, downtime_minutes,
                success_rate, error_rate, issues_reported
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.equipment_id)
        .bind(input.log_date)
        .bind(input.tests_processed)
        .bind(input.tests_failed)
        .bind(input.downtime_minutes)
        .bind(rust_decimal::Decimal::from_f64_retain(success_rate).unwrap())
        .bind(rust_decimal::Decimal::from_f64_retain(error_rate).unwrap())
        .bind(&input.issues_reported)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(log)
    }

    pub async fn list_by_equipment(
        &self,
        equipment_id: Uuid,
        limit: i32,
    ) -> Result<Vec<EquipmentPerformanceLog>> {
        let logs = sqlx::query_as::<_, EquipmentPerformanceLog>(
            "SELECT * FROM equipment_performance_log WHERE equipment_id = $1 ORDER BY log_date DESC LIMIT $2"
        )
        .bind(equipment_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(logs)
    }
}

// ============================================================================
// Equipment Alert Repository
// ============================================================================

#[derive(Clone)]
pub struct EquipmentAlertRepository {
    pool: PgPool,
}

impl EquipmentAlertRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<EquipmentAlert>> {
        let alert = sqlx::query_as::<_, EquipmentAlert>(
            "SELECT * FROM equipment_alert WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(alert)
    }

    pub async fn list_by_filter(&self, filter: AlertFilter) -> Result<Vec<EquipmentAlert>> {
        let mut query = String::from("SELECT * FROM equipment_alert WHERE 1=1");
        let mut conditions = vec![];

        if let Some(equipment_id) = filter.equipment_id {
            conditions.push(format!("equipment_id = '{}'", equipment_id));
        }
        if let Some(alert_type) = filter.alert_type {
            conditions.push(format!("alert_type = '{}'", alert_type));
        }
        if let Some(severity) = filter.severity {
            conditions.push(format!("severity = '{}'", severity));
        }
        if let Some(is_resolved) = filter.is_resolved {
            conditions.push(format!("is_resolved = {}", is_resolved));
        }

        if !conditions.is_empty() {
            query.push_str(&format!(" AND {}", conditions.join(" AND ")));
        }

        query.push_str(" ORDER BY created_at DESC");

        let alerts = sqlx::query_as::<_, EquipmentAlert>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(alerts)
    }

    pub async fn acknowledge(&self, id: Uuid, acknowledged_by: Uuid) -> Result<EquipmentAlert> {
        let alert = sqlx::query_as::<_, EquipmentAlert>(
            "UPDATE equipment_alert SET is_acknowledged = TRUE, acknowledged_by = $2, acknowledged_at = NOW() WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .bind(acknowledged_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(alert)
    }

    pub async fn resolve(&self, input: ResolveAlertInput, resolved_by: Uuid) -> Result<EquipmentAlert> {
        let alert = sqlx::query_as::<_, EquipmentAlert>(
            "UPDATE equipment_alert SET is_resolved = TRUE, resolved_by = $2, resolved_at = NOW(), resolution_notes = $3 WHERE id = $1 RETURNING *"
        )
        .bind(input.id)
        .bind(resolved_by)
        .bind(&input.resolution_notes)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(alert)
    }
}
