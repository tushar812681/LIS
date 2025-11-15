use sqlx::PgPool;
use uuid::Uuid;
use common::error::{Error, Result};
use common::pagination::{Paginated, PaginationParams};
use crate::domain::*;

// ============================================================================
// QC Material Repository
// ============================================================================

#[derive(Clone)]
pub struct QcMaterialRepository {
    pool: PgPool,
}

impl QcMaterialRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateQcMaterialInput, created_by: Uuid) -> Result<QcMaterial> {
        let id = Uuid::new_v4();

        // Generate material code
        let material_code: (String,) = sqlx::query_as("SELECT generate_qc_material_code()")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        let material = sqlx::query_as::<_, QcMaterial>(
            r#"
            INSERT INTO qc_material (
                id, material_code, material_name, manufacturer, lot_number, catalog_number,
                qc_type, organization_id, test_id, test_name,
                level_number, level_name,
                target_mean, target_sd,
                expiry_date, manufacture_date,
                quantity_in_stock, minimum_stock_level,
                storage_location, storage_temperature,
                equipment_id, material_status, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(material_code.0)
        .bind(&input.material_name)
        .bind(&input.manufacturer)
        .bind(&input.lot_number)
        .bind(&input.catalog_number)
        .bind(input.qc_type)
        .bind(input.organization_id)
        .bind(input.test_id)
        .bind(&input.test_name)
        .bind(input.level_number)
        .bind(&input.level_name)
        .bind(input.target_mean)
        .bind(input.target_sd)
        .bind(input.expiry_date)
        .bind(input.manufacture_date)
        .bind(input.quantity_in_stock.unwrap_or(0))
        .bind(input.minimum_stock_level.unwrap_or(10))
        .bind(&input.storage_location)
        .bind(&input.storage_temperature)
        .bind(input.equipment_id)
        .bind(QcMaterialStatus::Active)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(material)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<QcMaterial>> {
        let material = sqlx::query_as::<_, QcMaterial>(
            "SELECT * FROM qc_material WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(material)
    }

    pub async fn find_by_code(&self, material_code: &str) -> Result<Option<QcMaterial>> {
        let material = sqlx::query_as::<_, QcMaterial>(
            "SELECT * FROM qc_material WHERE material_code = $1 AND is_deleted = FALSE"
        )
        .bind(material_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(material)
    }

    pub async fn list(
        &self,
        filter: QcMaterialFilter,
        pagination: PaginationParams,
    ) -> Result<Paginated<QcMaterial>> {
        let mut query = String::from(
            "FROM qc_material WHERE organization_id = $1 AND is_deleted = FALSE"
        );
        let mut bindings = vec![];

        if let Some(qc_type) = filter.qc_type {
            bindings.push(format!("qc_type = '{:?}'", qc_type));
        }
        if let Some(status) = filter.material_status {
            bindings.push(format!("material_status = '{:?}'", status));
        }
        if let Some(test_id) = filter.test_id {
            bindings.push(format!("test_id = '{}'", test_id));
        }
        if let Some(equipment_id) = filter.equipment_id {
            bindings.push(format!("equipment_id = '{}'", equipment_id));
        }
        if let Some(search) = filter.search_query {
            bindings.push(format!(
                "(material_name ILIKE '%{}%' OR manufacturer ILIKE '%{}%' OR lot_number ILIKE '%{}%')",
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
            "SELECT * {} ORDER BY material_name ASC LIMIT $2 OFFSET $3",
            query
        );

        let materials = sqlx::query_as::<_, QcMaterial>(&select_query)
            .bind(filter.organization_id)
            .bind(pagination.page_size as i64)
            .bind(((pagination.page - 1) * pagination.page_size) as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(Paginated::new(
            materials,
            &pagination,
            total.0 as u64,
        ))
    }

    pub async fn update(&self, input: UpdateQcMaterialInput, updated_by: Uuid) -> Result<QcMaterial> {
        let material = sqlx::query_as::<_, QcMaterial>(
            r#"
            UPDATE qc_material
            SET quantity_in_stock = COALESCE($2, quantity_in_stock),
                minimum_stock_level = COALESCE($3, minimum_stock_level),
                opened_date = COALESCE($4, opened_date),
                days_stable_after_opening = COALESCE($5, days_stable_after_opening),
                storage_location = COALESCE($6, storage_location),
                mean_value = COALESCE($7, mean_value),
                sd_value = COALESCE($8, sd_value),
                notes = COALESCE($9, notes),
                updated_by = $10
            WHERE id = $1 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.quantity_in_stock)
        .bind(input.minimum_stock_level)
        .bind(input.opened_date)
        .bind(input.days_stable_after_opening)
        .bind(input.storage_location)
        .bind(input.mean_value)
        .bind(input.sd_value)
        .bind(input.notes)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(material)
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE qc_material SET is_deleted = TRUE WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }
}

// ============================================================================
// QC Rule Repository
// ============================================================================

#[derive(Clone)]
pub struct QcRuleRepository {
    pool: PgPool,
}

impl QcRuleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateQcRuleInput, created_by: Uuid) -> Result<QcRule> {
        let id = Uuid::new_v4();

        let rule = sqlx::query_as::<_, QcRule>(
            r#"
            INSERT INTO qc_rule (
                id, organization_id, rule_name, rule_type, rule_description,
                is_active, is_blocking, violation_severity, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.organization_id)
        .bind(&input.rule_name)
        .bind(input.rule_type)
        .bind(&input.rule_description)
        .bind(true)
        .bind(input.is_blocking.unwrap_or(false))
        .bind(input.violation_severity)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(rule)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<QcRule>> {
        let rule = sqlx::query_as::<_, QcRule>(
            "SELECT * FROM qc_rule WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(rule)
    }

    pub async fn list_by_organization(&self, organization_id: Uuid) -> Result<Vec<QcRule>> {
        let rules = sqlx::query_as::<_, QcRule>(
            "SELECT * FROM qc_rule WHERE organization_id = $1 AND is_active = TRUE ORDER BY rule_name"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(rules)
    }

    pub async fn list_by_material(&self, qc_material_id: Uuid) -> Result<Vec<QcRule>> {
        let rules = sqlx::query_as::<_, QcRule>(
            r#"
            SELECT r.* FROM qc_rule r
            INNER JOIN qc_material_rule mr ON r.id = mr.qc_rule_id
            WHERE mr.qc_material_id = $1 AND mr.is_active = TRUE AND r.is_active = TRUE
            ORDER BY r.rule_name
            "#
        )
        .bind(qc_material_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(rules)
    }
}

// ============================================================================
// QC Material Rule Repository
// ============================================================================

#[derive(Clone)]
pub struct QcMaterialRuleRepository {
    pool: PgPool,
}

impl QcMaterialRuleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn assign(&self, input: AssignRuleToMaterialInput, created_by: Uuid) -> Result<QcMaterialRule> {
        let id = Uuid::new_v4();

        let assignment = sqlx::query_as::<_, QcMaterialRule>(
            r#"
            INSERT INTO qc_material_rule (id, qc_material_id, qc_rule_id, is_active, created_by)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (qc_material_id, qc_rule_id) DO UPDATE SET is_active = TRUE
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.qc_material_id)
        .bind(input.qc_rule_id)
        .bind(true)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(assignment)
    }

    pub async fn unassign(&self, qc_material_id: Uuid, qc_rule_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE qc_material_rule SET is_active = FALSE WHERE qc_material_id = $1 AND qc_rule_id = $2"
        )
        .bind(qc_material_id)
        .bind(qc_rule_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }
}

// ============================================================================
// QC Result Repository
// ============================================================================

#[derive(Clone)]
pub struct QcResultRepository {
    pool: PgPool,
}

impl QcResultRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: RecordQcResultInput, organization_id: Uuid, test_name: Option<String>) -> Result<QcResult> {
        let id = Uuid::new_v4();

        // Generate result number
        let result_number: (String,) = sqlx::query_as("SELECT generate_qc_result_number()")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        // Get material to fetch test_id and current mean/sd
        let material = sqlx::query_as::<_, QcMaterial>(
            "SELECT * FROM qc_material WHERE id = $1"
        )
        .bind(input.qc_material_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        let result = sqlx::query_as::<_, QcResult>(
            r#"
            INSERT INTO qc_result (
                id, result_number, qc_material_id, organization_id,
                test_id, test_name, equipment_id,
                result_date, result_time, result_value,
                mean_value, sd_value,
                result_status,
                performed_by, performed_by_name, comments
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(result_number.0)
        .bind(input.qc_material_id)
        .bind(organization_id)
        .bind(material.test_id)
        .bind(test_name)
        .bind(input.equipment_id)
        .bind(input.result_date)
        .bind(input.result_time)
        .bind(input.result_value)
        .bind(material.mean_value)
        .bind(material.sd_value)
        .bind(QcResultStatus::Pending)
        .bind(input.performed_by)
        .bind(&input.performed_by_name)
        .bind(&input.comments)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<QcResult>> {
        let result = sqlx::query_as::<_, QcResult>(
            "SELECT * FROM qc_result WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn list_by_filter(&self, filter: QcResultFilter) -> Result<Vec<QcResult>> {
        let mut query = String::from("SELECT * FROM qc_result WHERE 1=1");
        let mut conditions = vec![];

        if let Some(material_id) = filter.qc_material_id {
            conditions.push(format!("qc_material_id = '{}'", material_id));
        }
        if let Some(org_id) = filter.organization_id {
            conditions.push(format!("organization_id = '{}'", org_id));
        }
        if let Some(status) = filter.result_status {
            conditions.push(format!("result_status = '{:?}'", status));
        }
        if let Some(from_date) = filter.from_date {
            conditions.push(format!("result_date >= '{}'", from_date));
        }
        if let Some(to_date) = filter.to_date {
            conditions.push(format!("result_date <= '{}'", to_date));
        }

        if !conditions.is_empty() {
            query.push_str(&format!(" AND {}", conditions.join(" AND ")));
        }

        query.push_str(" ORDER BY result_date DESC, result_time DESC");

        let results = sqlx::query_as::<_, QcResult>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(results)
    }

    pub async fn update_status(&self, id: Uuid, status: QcResultStatus, rules_violated: Option<serde_json::Value>) -> Result<QcResult> {
        let result = sqlx::query_as::<_, QcResult>(
            "UPDATE qc_result SET result_status = $2, rules_violated = $3 WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .bind(status)
        .bind(rules_violated)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn review(&self, input: ReviewQcResultInput, reviewed_by: Uuid) -> Result<QcResult> {
        let result = sqlx::query_as::<_, QcResult>(
            r#"
            UPDATE qc_result
            SET reviewed = TRUE,
                reviewed_by = $2,
                reviewed_at = NOW(),
                comments = COALESCE($3, comments)
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(reviewed_by)
        .bind(&input.comments)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn get_recent_results(&self, qc_material_id: Uuid, limit: i32) -> Result<Vec<QcResult>> {
        let results = sqlx::query_as::<_, QcResult>(
            "SELECT * FROM qc_result WHERE qc_material_id = $1 ORDER BY result_date DESC, result_time DESC LIMIT $2"
        )
        .bind(qc_material_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(results)
    }
}

// ============================================================================
// QC Violation Repository
// ============================================================================

#[derive(Clone)]
pub struct QcViolationRepository {
    pool: PgPool,
}

impl QcViolationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        qc_result_id: Uuid,
        qc_material_id: Uuid,
        organization_id: Uuid,
        qc_rule: &QcRule,
        violation_date: chrono::NaiveDate,
        violation_time: chrono::NaiveTime,
    ) -> Result<QcViolation> {
        let id = Uuid::new_v4();

        let violation = sqlx::query_as::<_, QcViolation>(
            r#"
            INSERT INTO qc_violation (
                id, qc_result_id, qc_material_id, organization_id,
                qc_rule_id, rule_type, rule_description,
                violation_date, violation_time, severity
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(qc_result_id)
        .bind(qc_material_id)
        .bind(organization_id)
        .bind(qc_rule.id)
        .bind(qc_rule.rule_type)
        .bind(&qc_rule.rule_description)
        .bind(violation_date)
        .bind(violation_time)
        .bind(qc_rule.violation_severity)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(violation)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<QcViolation>> {
        let violation = sqlx::query_as::<_, QcViolation>(
            "SELECT * FROM qc_violation WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(violation)
    }

    pub async fn list_by_filter(&self, filter: QcViolationFilter) -> Result<Vec<QcViolation>> {
        let mut query = String::from("SELECT * FROM qc_violation WHERE organization_id = $1");
        let mut conditions = vec![];

        if let Some(material_id) = filter.qc_material_id {
            conditions.push(format!("qc_material_id = '{}'", material_id));
        }
        if let Some(severity) = filter.severity {
            conditions.push(format!("severity = '{:?}'", severity));
        }
        if let Some(is_resolved) = filter.is_resolved {
            conditions.push(format!("is_resolved = {}", is_resolved));
        }
        if let Some(from_date) = filter.from_date {
            conditions.push(format!("violation_date >= '{}'", from_date));
        }
        if let Some(to_date) = filter.to_date {
            conditions.push(format!("violation_date <= '{}'", to_date));
        }

        if !conditions.is_empty() {
            query.push_str(&format!(" AND {}", conditions.join(" AND ")));
        }

        query.push_str(" ORDER BY violation_date DESC, violation_time DESC");

        let violations = sqlx::query_as::<_, QcViolation>(&query)
            .bind(filter.organization_id)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(violations)
    }

    pub async fn acknowledge(&self, id: Uuid, acknowledged_by: Uuid) -> Result<QcViolation> {
        let violation = sqlx::query_as::<_, QcViolation>(
            "UPDATE qc_violation SET is_acknowledged = TRUE, acknowledged_by = $2, acknowledged_at = NOW() WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .bind(acknowledged_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(violation)
    }

    pub async fn resolve(&self, input: ResolveViolationInput, resolved_by: Uuid) -> Result<QcViolation> {
        let violation = sqlx::query_as::<_, QcViolation>(
            "UPDATE qc_violation SET is_resolved = TRUE, resolved_by = $2, resolved_at = NOW(), root_cause = $3 WHERE id = $1 RETURNING *"
        )
        .bind(input.id)
        .bind(resolved_by)
        .bind(&input.root_cause)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(violation)
    }
}

// ============================================================================
// QC Corrective Action Repository
// ============================================================================

#[derive(Clone)]
pub struct QcCorrectiveActionRepository {
    pool: PgPool,
}

impl QcCorrectiveActionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateCorrectiveActionInput, created_by: Uuid) -> Result<QcCorrectiveAction> {
        let id = Uuid::new_v4();

        let action = sqlx::query_as::<_, QcCorrectiveAction>(
            r#"
            INSERT INTO qc_corrective_action (
                id, qc_violation_id, action_description, action_status,
                assigned_to, assigned_to_name, due_date, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.qc_violation_id)
        .bind(&input.action_description)
        .bind(CorrectiveActionStatus::Pending)
        .bind(input.assigned_to)
        .bind(&input.assigned_to_name)
        .bind(input.due_date)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(action)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<QcCorrectiveAction>> {
        let action = sqlx::query_as::<_, QcCorrectiveAction>(
            "SELECT * FROM qc_corrective_action WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(action)
    }

    pub async fn list_by_violation(&self, qc_violation_id: Uuid) -> Result<Vec<QcCorrectiveAction>> {
        let actions = sqlx::query_as::<_, QcCorrectiveAction>(
            "SELECT * FROM qc_corrective_action WHERE qc_violation_id = $1 ORDER BY created_at DESC"
        )
        .bind(qc_violation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(actions)
    }

    pub async fn update(&self, input: UpdateCorrectiveActionInput, updated_by: Uuid) -> Result<QcCorrectiveAction> {
        let action = sqlx::query_as::<_, QcCorrectiveAction>(
            r#"
            UPDATE qc_corrective_action
            SET action_status = COALESCE($2, action_status),
                completed_date = COALESCE($3, completed_date),
                effectiveness_verified = COALESCE($4, effectiveness_verified),
                verification_notes = COALESCE($5, verification_notes),
                updated_by = $6
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.action_status)
        .bind(input.completed_date)
        .bind(input.effectiveness_verified)
        .bind(&input.verification_notes)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(action)
    }
}

// ============================================================================
// QC External Program Repository
// ============================================================================

#[derive(Clone)]
pub struct QcExternalProgramRepository {
    pool: PgPool,
}

impl QcExternalProgramRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateExternalProgramInput, created_by: Uuid) -> Result<QcExternalProgram> {
        let id = Uuid::new_v4();

        let program = sqlx::query_as::<_, QcExternalProgram>(
            r#"
            INSERT INTO qc_external_program (
                id, organization_id, program_name, provider, program_code,
                qc_type, enrollment_date, contact_person, contact_email,
                is_active, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.organization_id)
        .bind(&input.program_name)
        .bind(&input.provider)
        .bind(&input.program_code)
        .bind(input.qc_type)
        .bind(input.enrollment_date)
        .bind(&input.contact_person)
        .bind(&input.contact_email)
        .bind(true)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(program)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<QcExternalProgram>> {
        let program = sqlx::query_as::<_, QcExternalProgram>(
            "SELECT * FROM qc_external_program WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(program)
    }

    pub async fn list_by_organization(&self, organization_id: Uuid) -> Result<Vec<QcExternalProgram>> {
        let programs = sqlx::query_as::<_, QcExternalProgram>(
            "SELECT * FROM qc_external_program WHERE organization_id = $1 AND is_active = TRUE ORDER BY program_name"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(programs)
    }
}
