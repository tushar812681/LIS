use sqlx::{PgPool, Row};
use uuid::Uuid;
use common::error::{Error, Result};

use crate::domain::*;

// ============================================================================
// Test Result Repository
// ============================================================================

#[derive(Clone)]
pub struct TestResultRepository {
    pool: PgPool,
}

impl TestResultRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateResultInput, org_id: Uuid, user_id: Uuid) -> Result<TestResult> {
        input.validate()?;

        // Generate result number
        let result_number = self.generate_result_number(&org_id, &input.test_id).await?;

        let result = sqlx::query_as::<_, TestResult>(
            r#"
            INSERT INTO test_result (
                id, result_number, patient_id, order_id, order_item_id,
                test_id, sample_id, organization_id,
                test_code, test_name,
                result_value, result_unit, result_type,
                result_status, verification_status,
                entry_method, entered_by, entry_date,
                instrument_id, run_number, technician_notes,
                created_by, updated_by
            )
            SELECT
                $1, $2,
                (SELECT patient_id FROM test_order WHERE id = $3),
                $3, $4, $5, $6, $7,
                (SELECT test_code FROM test_catalog WHERE id = $5),
                (SELECT test_name FROM test_catalog WHERE id = $5),
                $8, $9, 'NUMERIC',
                'PENDING', 'NOT_VERIFIED',
                $10, $11, NOW(),
                $12, $13, $14,
                $15, $15
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&result_number)
        .bind(input.order_id)
        .bind(input.order_item_id)
        .bind(input.test_id)
        .bind(input.sample_id)
        .bind(org_id)
        .bind(&input.result_value)
        .bind(&input.result_unit)
        .bind(&input.entry_method)
        .bind(user_id)
        .bind(input.instrument_id)
        .bind(&input.run_number)
        .bind(&input.technician_notes)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<TestResult>> {
        let result = sqlx::query_as::<_, TestResult>(
            "SELECT * FROM test_result WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn find_by_result_number(&self, result_number: &str) -> Result<Option<TestResult>> {
        let result = sqlx::query_as::<_, TestResult>(
            "SELECT * FROM test_result WHERE result_number = $1 AND is_deleted = FALSE"
        )
        .bind(result_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn find_by_patient(&self, patient_id: Uuid, limit: i64) -> Result<Vec<TestResult>> {
        let results = sqlx::query_as::<_, TestResult>(
            r#"
            SELECT * FROM test_result
            WHERE patient_id = $1 AND is_deleted = FALSE
            ORDER BY result_date DESC
            LIMIT $2
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(results)
    }

    pub async fn find_by_order(&self, order_id: Uuid) -> Result<Vec<TestResult>> {
        let results = sqlx::query_as::<_, TestResult>(
            "SELECT * FROM test_result WHERE order_id = $1 AND is_deleted = FALSE ORDER BY created_at"
        )
        .bind(order_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(results)
    }

    pub async fn find_by_sample(&self, sample_id: Uuid) -> Result<Vec<TestResult>> {
        let results = sqlx::query_as::<_, TestResult>(
            "SELECT * FROM test_result WHERE sample_id = $1 AND is_deleted = FALSE ORDER BY created_at"
        )
        .bind(sample_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(results)
    }

    pub async fn search(&self, filter: ResultFilter, org_id: Uuid, limit: i64) -> Result<Vec<TestResult>> {
        let mut query = String::from(
            "SELECT * FROM test_result WHERE organization_id = $1 AND is_deleted = FALSE"
        );
        let mut param_count = 1;

        if filter.patient_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND patient_id = ${}", param_count));
        }
        if filter.order_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND order_id = ${}", param_count));
        }
        if filter.sample_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND sample_id = ${}", param_count));
        }
        if filter.result_status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND result_status = ${}", param_count));
        }
        if filter.verification_status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND verification_status = ${}", param_count));
        }
        if filter.is_critical.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND is_critical = ${}", param_count));
        }

        query.push_str(" ORDER BY result_date DESC LIMIT $");
        param_count += 1;
        query.push_str(&param_count.to_string());

        let mut sql_query = sqlx::query_as::<_, TestResult>(&query).bind(org_id);

        if let Some(patient_id) = filter.patient_id {
            sql_query = sql_query.bind(patient_id);
        }
        if let Some(order_id) = filter.order_id {
            sql_query = sql_query.bind(order_id);
        }
        if let Some(sample_id) = filter.sample_id {
            sql_query = sql_query.bind(sample_id);
        }
        if let Some(status) = filter.result_status {
            sql_query = sql_query.bind(status);
        }
        if let Some(verification) = filter.verification_status {
            sql_query = sql_query.bind(verification);
        }
        if let Some(is_critical) = filter.is_critical {
            sql_query = sql_query.bind(is_critical);
        }

        sql_query = sql_query.bind(limit);

        let results = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(results)
    }

    pub async fn update_result(&self, input: UpdateResultInput, user_id: Uuid) -> Result<TestResult> {
        input.validate()?;

        let result = sqlx::query_as::<_, TestResult>(
            r#"
            UPDATE test_result
            SET
                result_value = $1,
                result_unit = $2,
                technician_notes = $3,
                updated_by = $4,
                updated_at = NOW()
            WHERE id = $5 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&input.result_value)
        .bind(&input.result_unit)
        .bind(&input.technician_notes)
        .bind(user_id)
        .bind(input.result_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn verify_result(&self, input: VerifyResultInput, user_id: Uuid) -> Result<TestResult> {
        let verification_status = if input.verification_method == "AUTO" {
            VerificationStatus::AutoVerified
        } else {
            VerificationStatus::ManuallyVerified
        };

        let result = sqlx::query_as::<_, TestResult>(
            r#"
            UPDATE test_result
            SET
                verification_status = $1,
                verified_by = $2,
                verification_date = NOW(),
                pathologist_notes = $3,
                updated_by = $2,
                updated_at = NOW()
            WHERE id = $4 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(verification_status)
        .bind(user_id)
        .bind(&input.pathologist_notes)
        .bind(input.result_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn approve_result(&self, input: ApproveResultInput, user_id: Uuid) -> Result<TestResult> {
        let result = sqlx::query_as::<_, TestResult>(
            r#"
            UPDATE test_result
            SET
                result_status = 'FINAL',
                approved_by = $1,
                approval_date = NOW(),
                approval_notes = $2,
                reported_date = NOW(),
                updated_by = $1,
                updated_at = NOW()
            WHERE id = $3 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(&input.approval_notes)
        .bind(input.result_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn correct_result(&self, input: CorrectResultInput, user_id: Uuid) -> Result<TestResult> {
        input.validate()?;

        // Get the original result
        let original = self.find_by_id(input.result_id).await?
            .ok_or_else(|| Error::NotFound(format!("Result not found: {}", input.result_id)))?;

        // Create corrected result
        let result = sqlx::query_as::<_, TestResult>(
            r#"
            INSERT INTO test_result (
                id, result_number, patient_id, order_id, order_item_id,
                test_id, sample_id, organization_id,
                test_code, test_name,
                result_value, result_unit, result_type,
                result_status, verification_status,
                is_corrected, corrected_from_result_id, correction_reason,
                corrected_by, correction_date,
                created_by, updated_by
            )
            VALUES (
                $1, $2 || '-C', $3, $4, $5, $6, $7, $8,
                $9, $10, $11, $12, $13,
                'CORRECTED', 'NOT_VERIFIED',
                TRUE, $14, $15,
                $16, NOW(),
                $16, $16
            )
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&original.result_number)
        .bind(original.patient_id)
        .bind(original.order_id)
        .bind(original.order_item_id)
        .bind(original.test_id)
        .bind(original.sample_id)
        .bind(original.organization_id)
        .bind(&original.test_code)
        .bind(&original.test_name)
        .bind(&input.new_result_value)
        .bind(&original.result_unit)
        .bind(&original.result_type)
        .bind(input.result_id)
        .bind(&input.correction_reason)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        // Mark original as corrected
        sqlx::query(
            "UPDATE test_result SET is_corrected = TRUE, updated_at = NOW() WHERE id = $1"
        )
        .bind(input.result_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    pub async fn get_pending_verification(&self, org_id: Uuid, limit: i64) -> Result<Vec<TestResult>> {
        let results = sqlx::query_as::<_, TestResult>(
            r#"
            SELECT * FROM test_result
            WHERE organization_id = $1
              AND verification_status = 'NOT_VERIFIED'
              AND result_status IN ('PENDING', 'IN_PROGRESS')
              AND is_deleted = FALSE
            ORDER BY created_at
            LIMIT $2
            "#
        )
        .bind(org_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(results)
    }

    pub async fn get_critical_results(&self, org_id: Uuid, limit: i64) -> Result<Vec<TestResult>> {
        let results = sqlx::query_as::<_, TestResult>(
            r#"
            SELECT * FROM test_result
            WHERE organization_id = $1
              AND is_critical = TRUE
              AND is_deleted = FALSE
            ORDER BY result_date DESC
            LIMIT $2
            "#
        )
        .bind(org_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(results)
    }

    pub async fn get_previous_result(&self, patient_id: Uuid, test_id: Uuid) -> Result<Option<TestResult>> {
        let result = sqlx::query_as::<_, TestResult>(
            r#"
            SELECT * FROM test_result
            WHERE patient_id = $1
              AND test_id = $2
              AND result_status = 'FINAL'
              AND is_deleted = FALSE
            ORDER BY result_date DESC
            LIMIT 1
            "#
        )
        .bind(patient_id)
        .bind(test_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result)
    }

    async fn generate_result_number(&self, org_id: &Uuid, test_id: &Uuid) -> Result<String> {
        let org_code = "LAB"; // Should fetch from organization service
        let test_code = "TST"; // Should fetch from test catalog

        let row = sqlx::query("SELECT nextval('result_sequence')")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        let sequence: i64 = row.try_get(0).map_err(|e| Error::Database(sqlx::Error::Decode(Box::new(e))))?;

        let base_id = format!(
            "{}-RES-{}-{}-{:06}",
            org_code,
            test_code,
            chrono::Utc::now().format("%Y%m%d"),
            sequence
        );

        let checksum = common::utils::calculate_luhn_check_digit(&base_id);

        Ok(format!("{}{}", base_id, checksum))
    }
}

// ============================================================================
// Reference Range Repository
// ============================================================================

#[derive(Clone)]
pub struct ReferenceRangeRepository {
    pool: PgPool,
}

impl ReferenceRangeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_test(&self, test_id: Uuid) -> Result<Vec<ReferenceRange>> {
        let ranges = sqlx::query_as::<_, ReferenceRange>(
            "SELECT * FROM reference_range WHERE test_id = $1 AND is_active = TRUE ORDER BY age_min"
        )
        .bind(test_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(ranges)
    }

    pub async fn find_applicable_range(
        &self,
        test_id: Uuid,
        age: i32,
        gender: &str
    ) -> Result<Option<ReferenceRange>> {
        let ranges = self.find_by_test(test_id).await?;

        for range in ranges {
            if range.is_applicable(age, gender) {
                return Ok(Some(range));
            }
        }

        Ok(None)
    }
}

// ============================================================================
// Auto-Verification Rule Repository
// ============================================================================

#[derive(Clone)]
pub struct AutoVerificationRuleRepository {
    pool: PgPool,
}

impl AutoVerificationRuleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_test(&self, test_id: Uuid) -> Result<Vec<AutoVerificationRule>> {
        let rules = sqlx::query_as::<_, AutoVerificationRule>(
            r#"
            SELECT * FROM auto_verification_rule
            WHERE (test_id = $1 OR is_global = TRUE)
              AND is_active = TRUE
            ORDER BY priority
            "#
        )
        .bind(test_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(rules)
    }

    pub async fn find_by_department(&self, department: &str) -> Result<Vec<AutoVerificationRule>> {
        let rules = sqlx::query_as::<_, AutoVerificationRule>(
            r#"
            SELECT * FROM auto_verification_rule
            WHERE (department = $1 OR is_global = TRUE)
              AND is_active = TRUE
            ORDER BY priority
            "#
        )
        .bind(department)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(rules)
    }
}

// ============================================================================
// Critical Result Notification Repository
// ============================================================================

#[derive(Clone)]
pub struct CriticalResultNotificationRepository {
    pool: PgPool,
}

impl CriticalResultNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: RecordCriticalNotificationInput, user_id: Uuid) -> Result<CriticalResultNotification> {
        input.validate()?;

        let notification = sqlx::query_as::<_, CriticalResultNotification>(
            r#"
            INSERT INTO critical_result_notification (
                id, result_id, notified_to, notification_method,
                caller_name, call_back_number, notes, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(input.result_id)
        .bind(&input.notified_to)
        .bind(&input.notification_method)
        .bind(&input.caller_name)
        .bind(&input.call_back_number)
        .bind(&input.notes)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(notification)
    }

    pub async fn find_by_result(&self, result_id: Uuid) -> Result<Vec<CriticalResultNotification>> {
        let notifications = sqlx::query_as::<_, CriticalResultNotification>(
            "SELECT * FROM critical_result_notification WHERE result_id = $1 ORDER BY notification_date DESC"
        )
        .bind(result_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(notifications)
    }

    pub async fn acknowledge(&self, notification_id: Uuid, acknowledged_by: &str, method: &str) -> Result<CriticalResultNotification> {
        let notification = sqlx::query_as::<_, CriticalResultNotification>(
            r#"
            UPDATE critical_result_notification
            SET
                acknowledged = TRUE,
                acknowledged_by = $1,
                acknowledgment_date = NOW(),
                acknowledgment_method = $2
            WHERE id = $3
            RETURNING *
            "#
        )
        .bind(acknowledged_by)
        .bind(method)
        .bind(notification_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(notification)
    }

    pub async fn get_unacknowledged(&self, org_id: Uuid) -> Result<Vec<CriticalResultNotification>> {
        let notifications = sqlx::query_as::<_, CriticalResultNotification>(
            r#"
            SELECT crn.* FROM critical_result_notification crn
            INNER JOIN test_result tr ON crn.result_id = tr.id
            WHERE tr.organization_id = $1
              AND crn.acknowledged = FALSE
            ORDER BY crn.notification_date DESC
            "#
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(notifications)
    }
}
