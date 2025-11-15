use uuid::Uuid;
use common::error::{Error, Result};

use crate::domain::*;
use crate::repository::*;

// ============================================================================
// Result Service
// ============================================================================

#[derive(Clone)]
pub struct ResultService {
    result_repo: TestResultRepository,
    reference_range_repo: ReferenceRangeRepository,
    auto_verification_repo: AutoVerificationRuleRepository,
    critical_notification_repo: CriticalResultNotificationRepository,
}

impl ResultService {
    pub fn new(
        result_repo: TestResultRepository,
        reference_range_repo: ReferenceRangeRepository,
        auto_verification_repo: AutoVerificationRuleRepository,
        critical_notification_repo: CriticalResultNotificationRepository,
    ) -> Self {
        Self {
            result_repo,
            reference_range_repo,
            auto_verification_repo,
            critical_notification_repo,
        }
    }

    // ========================================================================
    // Result Operations
    // ========================================================================

    pub async fn create_result(&self, input: CreateResultInput, org_id: Uuid, user_id: Uuid) -> Result<TestResult> {
        input.validate()?;

        // Create result
        let mut result = self.result_repo.create(input, org_id, user_id).await?;

        // Get patient age and gender (TODO: from patient service)
        let patient_age = 30;
        let patient_gender = "MALE";

        // Apply reference range
        if let Some(range) = self.reference_range_repo
            .find_applicable_range(result.test_id, patient_age, patient_gender)
            .await?
        {
            result.reference_range_min = range.range_min;
            result.reference_range_max = range.range_max;
            result.reference_range_text = range.range_text;
        }

        // Calculate interpretation
        result.calculate_interpretation();

        // Check for critical values
        result = self.check_critical_values(result).await?;

        // Perform delta check
        result = self.perform_delta_check(result).await?;

        // Attempt auto-verification
        result = self.attempt_auto_verification(result).await?;

        // TODO: Publish RESULT_CREATED event
        // TODO: Cache result

        tracing::info!("Result created: {}", result.result_number);
        Ok(result)
    }

    pub async fn get_result(&self, id: Uuid) -> Result<TestResult> {
        self.result_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Result not found: {}", id)))
    }

    pub async fn get_result_by_number(&self, result_number: &str) -> Result<TestResult> {
        self.result_repo
            .find_by_result_number(result_number)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Result not found: {}", result_number)))
    }

    pub async fn get_results_by_patient(&self, patient_id: Uuid, limit: i64) -> Result<Vec<TestResult>> {
        self.result_repo.find_by_patient(patient_id, limit).await
    }

    pub async fn get_results_by_order(&self, order_id: Uuid) -> Result<Vec<TestResult>> {
        self.result_repo.find_by_order(order_id).await
    }

    pub async fn get_results_by_sample(&self, sample_id: Uuid) -> Result<Vec<TestResult>> {
        self.result_repo.find_by_sample(sample_id).await
    }

    pub async fn search_results(&self, filter: ResultFilter, org_id: Uuid, limit: i64) -> Result<Vec<TestResult>> {
        self.result_repo.search(filter, org_id, limit).await
    }

    pub async fn update_result(&self, input: UpdateResultInput, user_id: Uuid) -> Result<TestResult> {
        input.validate()?;

        // Verify result can be updated
        let current = self.get_result(input.result_id).await?;

        if current.is_approved() {
            return Err(Error::Validation(
                "Cannot update approved results. Use correction workflow instead.".to_string()
            ));
        }

        let mut result = self.result_repo.update_result(input, user_id).await?;

        // Recalculate interpretation
        result.calculate_interpretation();

        // Re-check critical values
        result = self.check_critical_values(result).await?;

        // TODO: Publish RESULT_UPDATED event
        // TODO: Invalidate cache

        tracing::info!("Result updated: {}", result.result_number);
        Ok(result)
    }

    // ========================================================================
    // Verification Operations
    // ========================================================================

    pub async fn verify_result(&self, input: VerifyResultInput, user_id: Uuid) -> Result<TestResult> {
        let current = self.get_result(input.result_id).await?;

        // Business rules validation
        if current.result_status == ResultStatus::Cancelled {
            return Err(Error::Validation("Cannot verify cancelled results".to_string()));
        }

        if current.is_verified() {
            return Err(Error::Validation("Result is already verified".to_string()));
        }

        let result = self.result_repo.verify_result(input, user_id).await?;

        // TODO: Publish RESULT_VERIFIED event
        // TODO: Invalidate cache

        tracing::info!("Result verified: {}", result.result_number);
        Ok(result)
    }

    pub async fn approve_result(&self, input: ApproveResultInput, user_id: Uuid) -> Result<TestResult> {
        let current = self.get_result(input.result_id).await?;

        // Business rules validation
        if !current.is_verified() {
            return Err(Error::Validation(
                "Result must be verified before approval".to_string()
            ));
        }

        if current.is_approved() {
            return Err(Error::Validation("Result is already approved".to_string()));
        }

        let result = self.result_repo.approve_result(input, user_id).await?;

        // If critical, ensure notification is recorded
        if result.is_critical {
            // TODO: Check if notification exists
            tracing::warn!("Critical result approved: {}. Ensure notification is documented.", result.result_number);
        }

        // TODO: Publish RESULT_APPROVED event
        // TODO: Trigger report generation
        // TODO: Invalidate cache

        tracing::info!("Result approved: {}", result.result_number);
        Ok(result)
    }

    pub async fn correct_result(&self, input: CorrectResultInput, user_id: Uuid) -> Result<TestResult> {
        input.validate()?;

        let current = self.get_result(input.result_id).await?;

        // Business rules validation
        if !current.is_approved() {
            return Err(Error::Validation(
                "Can only correct approved results. Use update for unapproved results.".to_string()
            ));
        }

        let result = self.result_repo.correct_result(input, user_id).await?;

        // TODO: Publish RESULT_CORRECTED event
        // TODO: Notify stakeholders
        // TODO: Invalidate cache

        tracing::info!("Result corrected: {} -> {}", current.result_number, result.result_number);
        Ok(result)
    }

    pub async fn get_pending_verification(&self, org_id: Uuid, limit: i64) -> Result<Vec<TestResult>> {
        self.result_repo.get_pending_verification(org_id, limit).await
    }

    // ========================================================================
    // Critical Value Management
    // ========================================================================

    pub async fn get_critical_results(&self, org_id: Uuid, limit: i64) -> Result<Vec<TestResult>> {
        self.result_repo.get_critical_results(org_id, limit).await
    }

    pub async fn record_critical_notification(
        &self,
        input: RecordCriticalNotificationInput,
        user_id: Uuid
    ) -> Result<CriticalResultNotification> {
        input.validate()?;

        // Verify result is critical
        let result = self.get_result(input.result_id).await?;

        if !result.is_critical {
            return Err(Error::Validation(
                "Cannot record critical notification for non-critical result".to_string()
            ));
        }

        let notification = self.critical_notification_repo.create(input, user_id).await?;

        // TODO: Publish CRITICAL_NOTIFICATION_RECORDED event

        tracing::info!("Critical notification recorded for result: {}", result.result_number);
        Ok(notification)
    }

    pub async fn get_critical_notifications(&self, result_id: Uuid) -> Result<Vec<CriticalResultNotification>> {
        self.critical_notification_repo.find_by_result(result_id).await
    }

    pub async fn acknowledge_critical_notification(
        &self,
        notification_id: Uuid,
        acknowledged_by: &str,
        method: &str
    ) -> Result<CriticalResultNotification> {
        let notification = self.critical_notification_repo
            .acknowledge(notification_id, acknowledged_by, method)
            .await?;

        // TODO: Publish CRITICAL_NOTIFICATION_ACKNOWLEDGED event

        tracing::info!("Critical notification acknowledged: {}", notification_id);
        Ok(notification)
    }

    pub async fn get_unacknowledged_critical_notifications(&self, org_id: Uuid) -> Result<Vec<CriticalResultNotification>> {
        self.critical_notification_repo.get_unacknowledged(org_id).await
    }

    // ========================================================================
    // Auto-Verification Engine
    // ========================================================================

    async fn attempt_auto_verification(&self, mut result: TestResult) -> Result<TestResult> {
        // Get applicable rules
        let rules = self.auto_verification_repo.find_by_test(result.test_id).await?;

        if rules.is_empty() {
            return Ok(result);
        }

        let mut passed_rules = Vec::new();
        let mut failed_rules = Vec::new();
        let mut can_auto_verify = true;

        for rule in rules {
            let passed = self.evaluate_verification_rule(&result, &rule).await?;

            if passed {
                passed_rules.push(rule.rule_code.clone());
            } else {
                failed_rules.push(rule.rule_code.clone());
                if rule.is_blocking {
                    can_auto_verify = false;
                }
            }
        }

        result.verification_rules_passed = Some(serde_json::to_value(&passed_rules).unwrap());
        result.verification_rules_failed = Some(serde_json::to_value(&failed_rules).unwrap());

        if can_auto_verify && failed_rules.is_empty() {
            result.verification_status = VerificationStatus::AutoVerified;
            result.auto_verification_confidence = Some(rust_decimal::Decimal::from(95)); // 95% confidence
            tracing::info!("Result auto-verified: {}", result.result_number);
        } else {
            result.verification_status = VerificationStatus::PendingReview;
            tracing::info!("Result requires manual review: {}", result.result_number);
        }

        Ok(result)
    }

    async fn evaluate_verification_rule(&self, result: &TestResult, rule: &AutoVerificationRule) -> Result<bool> {
        match rule.rule_type.as_str() {
            "RANGE_CHECK" => {
                // Check if result is within reference range
                Ok(result.is_within_reference_range())
            }
            "DELTA_CHECK" => {
                // Check if delta is acceptable
                if result.delta_flag == DeltaFlag::Normal || result.delta_flag == DeltaFlag::NoPreviousResult {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "QC_CHECK" => {
                // Check if QC passed
                Ok(result.qc_passed.unwrap_or(true))
            }
            "CRITICAL_CHECK" => {
                // Check if result is not critical
                Ok(!result.is_critical)
            }
            _ => {
                tracing::warn!("Unknown verification rule type: {}", rule.rule_type);
                Ok(true)
            }
        }
    }

    // ========================================================================
    // Critical Value Detection
    // ========================================================================

    async fn check_critical_values(&self, mut result: TestResult) -> Result<TestResult> {
        if let Some(value_str) = &result.result_value {
            if let Ok(value) = value_str.parse::<f64>() {
                // Get reference range with critical values
                let patient_age = 30; // TODO: Get from patient service
                let patient_gender = "MALE"; // TODO: Get from patient service

                if let Some(range) = self.reference_range_repo
                    .find_applicable_range(result.test_id, patient_age, patient_gender)
                    .await?
                {
                    // Check panic values
                    if let Some(panic_low) = range.panic_low {
                        let panic_low_f = panic_low.to_string().parse::<f64>().unwrap_or(f64::MIN);
                        if value < panic_low_f {
                            result.critical_flag = CriticalFlag::PanicLow;
                            result.is_critical = true;
                        }
                    }

                    if let Some(panic_high) = range.panic_high {
                        let panic_high_f = panic_high.to_string().parse::<f64>().unwrap_or(f64::MAX);
                        if value > panic_high_f {
                            result.critical_flag = CriticalFlag::PanicHigh;
                            result.is_critical = true;
                        }
                    }

                    // Check critical values (less severe than panic)
                    if !result.is_critical {
                        if let Some(critical_low) = range.critical_low {
                            let critical_low_f = critical_low.to_string().parse::<f64>().unwrap_or(f64::MIN);
                            if value < critical_low_f {
                                result.critical_flag = CriticalFlag::Low;
                                result.is_critical = true;
                            }
                        }

                        if let Some(critical_high) = range.critical_high {
                            let critical_high_f = critical_high.to_string().parse::<f64>().unwrap_or(f64::MAX);
                            if value > critical_high_f {
                                result.critical_flag = CriticalFlag::High;
                                result.is_critical = true;
                            }
                        }
                    }
                }
            }
        }

        if result.is_critical {
            tracing::warn!("CRITICAL RESULT DETECTED: {} - {}", result.result_number, result.test_name);
            // TODO: Trigger immediate notification workflow
        }

        Ok(result)
    }

    // ========================================================================
    // Delta Check Analysis
    // ========================================================================

    async fn perform_delta_check(&self, mut result: TestResult) -> Result<TestResult> {
        // Get previous result for this patient and test
        if let Some(previous) = self.result_repo
            .get_previous_result(result.patient_id, result.test_id)
            .await?
        {
            result.previous_result_value = previous.result_value.clone();
            result.previous_result_date = Some(previous.result_date);

            // Calculate delta if both are numeric
            if let (Some(current_str), Some(prev_str)) = (&result.result_value, &previous.result_value) {
                if let (Ok(current_val), Ok(prev_val)) = (
                    current_str.parse::<f64>(),
                    prev_str.parse::<f64>()
                ) {
                    if prev_val != 0.0 {
                        let delta_abs = current_val - prev_val;
                        let delta_pct = ((current_val - prev_val) / prev_val) * 100.0;

                        result.delta_absolute = Some(rust_decimal::Decimal::try_from(delta_abs).unwrap_or_default());
                        result.delta_percentage = Some(rust_decimal::Decimal::try_from(delta_pct).unwrap_or_default());

                        // Classify delta
                        let threshold = 50.0; // 50% change threshold
                        result.delta_flag = if delta_pct.abs() > threshold {
                            if delta_pct > 0.0 {
                                DeltaFlag::SignificantIncrease
                            } else {
                                DeltaFlag::SignificantDecrease
                            }
                        } else {
                            DeltaFlag::Normal
                        };

                        if result.delta_flag != DeltaFlag::Normal {
                            tracing::warn!(
                                "Significant delta detected for {}: {}%",
                                result.result_number,
                                delta_pct
                            );
                        }
                    }
                }
            }
        } else {
            result.delta_flag = DeltaFlag::NoPreviousResult;
        }

        Ok(result)
    }

    // ========================================================================
    // Reference Range Operations
    // ========================================================================

    pub async fn get_reference_ranges(&self, test_id: Uuid) -> Result<Vec<ReferenceRange>> {
        self.reference_range_repo.find_by_test(test_id).await
    }

    pub async fn get_applicable_reference_range(
        &self,
        test_id: Uuid,
        age: i32,
        gender: &str
    ) -> Result<Option<ReferenceRange>> {
        self.reference_range_repo.find_applicable_range(test_id, age, gender).await
    }
}
