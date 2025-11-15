use uuid::Uuid;
use common::error::{Error, Result};
use common::pagination::{Paginated, PaginationParams};
use crate::domain::*;
use crate::repository::*;
use rust_decimal::Decimal;

// ============================================================================
// QC Service
// ============================================================================

#[derive(Clone)]
pub struct QcService {
    material_repo: QcMaterialRepository,
    rule_repo: QcRuleRepository,
    material_rule_repo: QcMaterialRuleRepository,
    result_repo: QcResultRepository,
    violation_repo: QcViolationRepository,
    corrective_action_repo: QcCorrectiveActionRepository,
    external_program_repo: QcExternalProgramRepository,
}

impl QcService {
    pub fn new(
        material_repo: QcMaterialRepository,
        rule_repo: QcRuleRepository,
        material_rule_repo: QcMaterialRuleRepository,
        result_repo: QcResultRepository,
        violation_repo: QcViolationRepository,
        corrective_action_repo: QcCorrectiveActionRepository,
        external_program_repo: QcExternalProgramRepository,
    ) -> Self {
        Self {
            material_repo,
            rule_repo,
            material_rule_repo,
            result_repo,
            violation_repo,
            corrective_action_repo,
            external_program_repo,
        }
    }

    // ========================================================================
    // QC Material Operations
    // ========================================================================

    pub async fn create_qc_material(
        &self,
        input: CreateQcMaterialInput,
        created_by: Uuid,
    ) -> Result<QcMaterial> {
        // Validate material name
        if input.material_name.trim().is_empty() {
            return Err(Error::Validation(
                "Material name cannot be empty".to_string(),
            ));
        }

        // Validate lot number
        if input.lot_number.trim().is_empty() {
            return Err(Error::Validation(
                "Lot number cannot be empty".to_string(),
            ));
        }

        // Validate expiry date
        if input.expiry_date <= chrono::Local::now().date_naive() {
            return Err(Error::Validation(
                "Expiry date must be in the future".to_string(),
            ));
        }

        let material = self.material_repo.create(input, created_by).await?;

        tracing::info!(
            "QC Material created: {} ({})",
            material.material_name,
            material.material_code
        );

        Ok(material)
    }

    pub async fn get_qc_material(&self, id: Uuid) -> Result<QcMaterial> {
        self.material_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("QC Material not found".to_string()))
    }

    pub async fn get_qc_material_by_code(&self, material_code: String) -> Result<QcMaterial> {
        self.material_repo
            .find_by_code(&material_code)
            .await?
            .ok_or_else(|| Error::NotFound("QC Material not found".to_string()))
    }

    pub async fn list_qc_materials(
        &self,
        filter: QcMaterialFilter,
        pagination: PaginationParams,
    ) -> Result<Paginated<QcMaterial>> {
        self.material_repo.list(filter, pagination).await
    }

    pub async fn update_qc_material(
        &self,
        input: UpdateQcMaterialInput,
        updated_by: Uuid,
    ) -> Result<QcMaterial> {
        // Check if material exists
        let _ = self.get_qc_material(input.id).await?;

        let mut material = self.material_repo.update(input, updated_by).await?;

        // Recalculate control limits if mean and SD are updated
        if material.mean_value.is_some() && material.sd_value.is_some() {
            material.calculate_control_limits();
        }

        tracing::info!(
            "QC Material updated: {} ({})",
            material.material_name,
            material.material_code
        );

        Ok(material)
    }

    pub async fn delete_qc_material(&self, id: Uuid) -> Result<bool> {
        // Check if material exists
        let material = self.get_qc_material(id).await?;

        // Check if material has recent results
        let recent_results = self.result_repo.get_recent_results(id, 1).await?;
        if !recent_results.is_empty() {
            return Err(Error::Validation(
                "Cannot delete QC material with existing results".to_string(),
            ));
        }

        let deleted = self.material_repo.delete(id).await?;

        if deleted {
            tracing::info!(
                "QC Material deleted: {} ({})",
                material.material_name,
                material.material_code
            );
        }

        Ok(deleted)
    }

    // ========================================================================
    // QC Rule Operations
    // ========================================================================

    pub async fn create_qc_rule(
        &self,
        input: CreateQcRuleInput,
        created_by: Uuid,
    ) -> Result<QcRule> {
        // Validate rule name
        if input.rule_name.trim().is_empty() {
            return Err(Error::Validation("Rule name cannot be empty".to_string()));
        }

        let rule = self.rule_repo.create(input, created_by).await?;

        tracing::info!("QC Rule created: {} ({:?})", rule.rule_name, rule.rule_type);

        Ok(rule)
    }

    pub async fn get_qc_rule(&self, id: Uuid) -> Result<QcRule> {
        self.rule_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("QC Rule not found".to_string()))
    }

    pub async fn list_qc_rules(&self, organization_id: Uuid) -> Result<Vec<QcRule>> {
        self.rule_repo.list_by_organization(organization_id).await
    }

    pub async fn assign_rule_to_material(
        &self,
        input: AssignRuleToMaterialInput,
        created_by: Uuid,
    ) -> Result<QcMaterialRule> {
        // Verify material exists
        let _ = self.get_qc_material(input.qc_material_id).await?;

        // Verify rule exists
        let _ = self.get_qc_rule(input.qc_rule_id).await?;

        let assignment = self.material_rule_repo.assign(input, created_by).await?;

        tracing::info!(
            "Rule assigned to QC material: material={}, rule={}",
            assignment.qc_material_id,
            assignment.qc_rule_id
        );

        Ok(assignment)
    }

    pub async fn unassign_rule_from_material(
        &self,
        qc_material_id: Uuid,
        qc_rule_id: Uuid,
    ) -> Result<bool> {
        let unassigned = self
            .material_rule_repo
            .unassign(qc_material_id, qc_rule_id)
            .await?;

        if unassigned {
            tracing::info!(
                "Rule unassigned from QC material: material={}, rule={}",
                qc_material_id,
                qc_rule_id
            );
        }

        Ok(unassigned)
    }

    // ========================================================================
    // QC Result Operations
    // ========================================================================

    pub async fn record_qc_result(
        &self,
        input: RecordQcResultInput,
        created_by: Uuid,
    ) -> Result<QcResult> {
        // Get QC material
        let material = self.get_qc_material(input.qc_material_id).await?;

        // Validate material is active
        if !material.is_active() {
            return Err(Error::Validation(
                "Cannot record result for inactive QC material".to_string(),
            ));
        }

        // Validate material is not expired
        if material.is_expired() || material.is_opened_and_expired() {
            return Err(Error::Validation(
                "Cannot record result for expired QC material".to_string(),
            ));
        }

        // Create result
        let mut result = self
            .result_repo
            .create(input, material.organization_id, material.test_name.clone())
            .await?;

        // Evaluate Westgard rules
        result = self.evaluate_westgard_rules(result).await?;

        tracing::info!(
            "QC Result recorded: {} - Status: {:?}",
            result.result_number,
            result.result_status
        );

        Ok(result)
    }

    pub async fn get_qc_result(&self, id: Uuid) -> Result<QcResult> {
        self.result_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("QC Result not found".to_string()))
    }

    pub async fn list_qc_results(&self, filter: QcResultFilter) -> Result<Vec<QcResult>> {
        self.result_repo.list_by_filter(filter).await
    }

    pub async fn review_qc_result(
        &self,
        input: ReviewQcResultInput,
        reviewed_by: Uuid,
    ) -> Result<QcResult> {
        // Check if result exists
        let _ = self.get_qc_result(input.id).await?;

        let result = self.result_repo.review(input, reviewed_by).await?;

        tracing::info!("QC Result reviewed: {}", result.result_number);

        Ok(result)
    }

    // ========================================================================
    // Westgard Rules Evaluation
    // ========================================================================

    async fn evaluate_westgard_rules(&self, mut result: QcResult) -> Result<QcResult> {
        // Get rules for this material
        let rules = self.rule_repo.list_by_material(result.qc_material_id).await?;

        if rules.is_empty() {
            // No rules assigned, mark as in control
            result = self
                .result_repo
                .update_status(result.id, QcResultStatus::InControl, None)
                .await?;
            return Ok(result);
        }

        // Get recent results for trend analysis
        let recent_results = self
            .result_repo
            .get_recent_results(result.qc_material_id, 20)
            .await?;

        let mut violations = vec![];
        let mut is_blocking_violation = false;

        for rule in &rules {
            if !rule.is_active() {
                continue;
            }

            let violated = self.check_rule(&result, &recent_results, rule).await?;

            if violated {
                violations.push(serde_json::json!({
                    "rule_id": rule.id,
                    "rule_name": rule.rule_name,
                    "rule_type": format!("{:?}", rule.rule_type),
                    "severity": format!("{:?}", rule.violation_severity),
                }));

                // Create violation record
                let _ = self
                    .violation_repo
                    .create(
                        result.id,
                        result.qc_material_id,
                        result.organization_id,
                        rule,
                        result.result_date,
                        result.result_time,
                    )
                    .await?;

                if rule.is_blocking() {
                    is_blocking_violation = true;
                }
            }
        }

        // Update result status based on violations
        let status = if violations.is_empty() {
            QcResultStatus::InControl
        } else if is_blocking_violation {
            QcResultStatus::OutOfControl
        } else {
            QcResultStatus::Warning
        };

        let rules_violated = if violations.is_empty() {
            None
        } else {
            Some(serde_json::Value::Array(violations))
        };

        result = self
            .result_repo
            .update_status(result.id, status, rules_violated)
            .await?;

        Ok(result)
    }

    async fn check_rule(
        &self,
        current_result: &QcResult,
        recent_results: &[QcResult],
        rule: &QcRule,
    ) -> Result<bool> {
        match rule.rule_type {
            QcRuleType::Westgard12s => self.check_1_2s(current_result),
            QcRuleType::Westgard13s => self.check_1_3s(current_result),
            QcRuleType::Westgard22s => self.check_2_2s(current_result, recent_results),
            QcRuleType::WestgardR4s => self.check_r_4s(current_result, recent_results),
            QcRuleType::Westgard41s => self.check_4_1s(current_result, recent_results),
            QcRuleType::Westgard10x => self.check_10_x(current_result, recent_results),
            QcRuleType::Custom => Ok(false), // Custom rules not implemented yet
        }
    }

    fn check_1_2s(&self, result: &QcResult) -> Result<bool> {
        // 1-2s: One control observation exceeds ±2SD
        if let Some(z) = result.z_score {
            Ok(z.abs() > Decimal::from(2))
        } else {
            Ok(false)
        }
    }

    fn check_1_3s(&self, result: &QcResult) -> Result<bool> {
        // 1-3s: One control observation exceeds ±3SD
        if let Some(z) = result.z_score {
            Ok(z.abs() > Decimal::from(3))
        } else {
            Ok(false)
        }
    }

    fn check_2_2s(&self, current: &QcResult, recent: &[QcResult]) -> Result<bool> {
        // 2-2s: Two consecutive controls exceed ±2SD on the same side of the mean
        if let Some(current_z) = current.z_score {
            if current_z.abs() <= Decimal::from(2) {
                return Ok(false);
            }

            // Check the most recent result
            if let Some(previous) = recent.first() {
                if let Some(prev_z) = previous.z_score {
                    if prev_z.abs() > Decimal::from(2) {
                        // Both exceed 2SD, check if on same side
                        return Ok((current_z > Decimal::ZERO && prev_z > Decimal::ZERO)
                            || (current_z < Decimal::ZERO && prev_z < Decimal::ZERO));
                    }
                }
            }
        }
        Ok(false)
    }

    fn check_r_4s(&self, current: &QcResult, recent: &[QcResult]) -> Result<bool> {
        // R-4s: Range between two consecutive controls exceeds 4SD
        if let Some(current_z) = current.z_score {
            if let Some(previous) = recent.first() {
                if let Some(prev_z) = previous.z_score {
                    let range = (current_z - prev_z).abs();
                    return Ok(range > Decimal::from(4));
                }
            }
        }
        Ok(false)
    }

    fn check_4_1s(&self, current: &QcResult, recent: &[QcResult]) -> Result<bool> {
        // 4-1s: Four consecutive controls exceed ±1SD on the same side
        if let Some(current_z) = current.z_score {
            if current_z.abs() <= Decimal::from(1) {
                return Ok(false);
            }

            let is_positive = current_z > Decimal::ZERO;
            let mut consecutive_count = 1;

            for result in recent.iter().take(3) {
                if let Some(z) = result.z_score {
                    if z.abs() > Decimal::from(1) {
                        let same_side = (z > Decimal::ZERO) == is_positive;
                        if same_side {
                            consecutive_count += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            return Ok(consecutive_count >= 4);
        }
        Ok(false)
    }

    fn check_10_x(&self, current: &QcResult, recent: &[QcResult]) -> Result<bool> {
        // 10-x: Ten consecutive controls on same side of mean
        if let Some(current_z) = current.z_score {
            let is_positive = current_z > Decimal::ZERO;
            let mut consecutive_count = 1;

            for result in recent.iter().take(9) {
                if let Some(z) = result.z_score {
                    let same_side = (z > Decimal::ZERO) == is_positive;
                    if same_side {
                        consecutive_count += 1;
                    } else {
                        break;
                    }
                }
            }

            return Ok(consecutive_count >= 10);
        }
        Ok(false)
    }

    // ========================================================================
    // Violation Operations
    // ========================================================================

    pub async fn get_violation(&self, id: Uuid) -> Result<QcViolation> {
        self.violation_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("QC Violation not found".to_string()))
    }

    pub async fn list_violations(&self, filter: QcViolationFilter) -> Result<Vec<QcViolation>> {
        self.violation_repo.list_by_filter(filter).await
    }

    pub async fn acknowledge_violation(
        &self,
        input: AcknowledgeViolationInput,
        acknowledged_by: Uuid,
    ) -> Result<QcViolation> {
        // Check if violation exists
        let existing = self.get_violation(input.id).await?;

        if existing.is_acknowledged() {
            return Err(Error::Validation(
                "Violation is already acknowledged".to_string(),
            ));
        }

        let violation = self.violation_repo.acknowledge(input.id, acknowledged_by).await?;

        tracing::info!("QC Violation acknowledged: {}", violation.id);

        Ok(violation)
    }

    pub async fn resolve_violation(
        &self,
        input: ResolveViolationInput,
        resolved_by: Uuid,
    ) -> Result<QcViolation> {
        // Check if violation exists
        let existing = self.get_violation(input.id).await?;

        if existing.is_resolved() {
            return Err(Error::Validation(
                "Violation is already resolved".to_string(),
            ));
        }

        let violation = self.violation_repo.resolve(input, resolved_by).await?;

        tracing::info!("QC Violation resolved: {}", violation.id);

        Ok(violation)
    }

    // ========================================================================
    // Corrective Action Operations
    // ========================================================================

    pub async fn create_corrective_action(
        &self,
        input: CreateCorrectiveActionInput,
        created_by: Uuid,
    ) -> Result<QcCorrectiveAction> {
        // Verify violation exists
        let _ = self.get_violation(input.qc_violation_id).await?;

        // Validate action description
        if input.action_description.trim().is_empty() {
            return Err(Error::Validation(
                "Action description cannot be empty".to_string(),
            ));
        }

        let action = self
            .corrective_action_repo
            .create(input, created_by)
            .await?;

        tracing::info!("Corrective action created for violation: {}", action.qc_violation_id);

        Ok(action)
    }

    pub async fn get_corrective_action(&self, id: Uuid) -> Result<QcCorrectiveAction> {
        self.corrective_action_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Corrective action not found".to_string()))
    }

    pub async fn list_corrective_actions(
        &self,
        qc_violation_id: Uuid,
    ) -> Result<Vec<QcCorrectiveAction>> {
        // Verify violation exists
        let _ = self.get_violation(qc_violation_id).await?;

        self.corrective_action_repo
            .list_by_violation(qc_violation_id)
            .await
    }

    pub async fn update_corrective_action(
        &self,
        input: UpdateCorrectiveActionInput,
        updated_by: Uuid,
    ) -> Result<QcCorrectiveAction> {
        // Check if action exists
        let _ = self.get_corrective_action(input.id).await?;

        let action = self.corrective_action_repo.update(input, updated_by).await?;

        tracing::info!("Corrective action updated: {}", action.id);

        Ok(action)
    }

    // ========================================================================
    // External Program Operations
    // ========================================================================

    pub async fn create_external_program(
        &self,
        input: CreateExternalProgramInput,
        created_by: Uuid,
    ) -> Result<QcExternalProgram> {
        // Validate program name
        if input.program_name.trim().is_empty() {
            return Err(Error::Validation(
                "Program name cannot be empty".to_string(),
            ));
        }

        let program = self.external_program_repo.create(input, created_by).await?;

        tracing::info!("External QC program created: {}", program.program_name);

        Ok(program)
    }

    pub async fn get_external_program(&self, id: Uuid) -> Result<QcExternalProgram> {
        self.external_program_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("External program not found".to_string()))
    }

    pub async fn list_external_programs(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<QcExternalProgram>> {
        self.external_program_repo
            .list_by_organization(organization_id)
            .await
    }
}
