use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::*;
use common::error::{Error, Result};

#[derive(Clone)]
pub struct ComplianceRepository {
    pool: PgPool,
}

impl ComplianceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // AUDIT LOG
    // ========================================================================

    pub async fn create_audit_log(&self, input: CreateAuditLogInput, org_id: Uuid, user_id: Option<Uuid>, session_id: Option<Uuid>) -> Result<AuditLog> {
        let id = Uuid::new_v4();
        let entity_id = Uuid::parse_str(&input.entity_id).map_err(|e| Error::InvalidInput(e.to_string()))?;

        let audit_log = sqlx::query_as::<_, AuditLog>(
            r#"
            INSERT INTO audit_log (
                id, organization_id, user_id, entity_type, entity_id, action,
                old_value, new_value, changes, reason, ip_address, user_agent, session_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, organization_id, user_id, entity_type, entity_id, action,
                      old_value, new_value, changes, reason, ip_address, user_agent,
                      session_id, created_at
            "#
        )
        .bind(id)
        .bind(org_id)
        .bind(user_id)
        .bind(&input.entity_type)
        .bind(entity_id)
        .bind(&input.action)
        .bind(&input.old_value)
        .bind(&input.new_value)
        .bind(&input.changes)
        .bind(&input.reason)
        .bind(&input.ip_address)
        .bind(&input.user_agent)
        .bind(session_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(audit_log)
    }

    pub async fn get_audit_logs(&self, org_id: Uuid, filter: Option<AuditLogFilter>, limit: i64, offset: i64) -> Result<Vec<AuditLog>> {
        let mut query = String::from("SELECT * FROM audit_log WHERE organization_id = $1");
        let mut bind_count = 2;

        if let Some(f) = &filter {
            if f.entity_type.is_some() {
                query.push_str(&format!(" AND entity_type = ${}", bind_count));
                bind_count += 1;
            }
            if f.entity_id.is_some() {
                query.push_str(&format!(" AND entity_id = ${}", bind_count));
                bind_count += 1;
            }
            if f.user_id.is_some() {
                query.push_str(&format!(" AND user_id = ${}", bind_count));
                bind_count += 1;
            }
            if f.action.is_some() {
                query.push_str(&format!(" AND action = ${}", bind_count));
                bind_count += 1;
            }
            if f.start_date.is_some() {
                query.push_str(&format!(" AND created_at >= ${}", bind_count));
                bind_count += 1;
            }
            if f.end_date.is_some() {
                query.push_str(&format!(" AND created_at <= ${}", bind_count));
                bind_count += 1;
            }
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", bind_count, bind_count + 1));

        let mut query_builder = sqlx::query_as::<_, AuditLog>(&query).bind(org_id);

        if let Some(f) = filter {
            if let Some(entity_type) = f.entity_type {
                query_builder = query_builder.bind(entity_type);
            }
            if let Some(entity_id) = f.entity_id {
                query_builder = query_builder.bind(Uuid::parse_str(&entity_id).map_err(|e| Error::InvalidInput(e.to_string()))?);
            }
            if let Some(user_id) = f.user_id {
                query_builder = query_builder.bind(Uuid::parse_str(&user_id).map_err(|e| Error::InvalidInput(e.to_string()))?);
            }
            if let Some(action) = f.action {
                query_builder = query_builder.bind(action);
            }
            if let Some(start_date) = f.start_date {
                let parsed = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string()))?;
                query_builder = query_builder.bind(parsed);
            }
            if let Some(end_date) = f.end_date {
                let parsed = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string()))?;
                query_builder = query_builder.bind(parsed);
            }
        }

        let logs = query_builder
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(logs)
    }

    // ========================================================================
    // DOCUMENT CONTROL
    // ========================================================================

    pub async fn create_document(&self, input: CreateDocumentInput, org_id: Uuid, author_id: Uuid) -> Result<DocumentControl> {
        let id = Uuid::new_v4();
        let version = input.version.unwrap_or_else(|| "1.0".to_string());
        let effective_date = input.effective_date.as_ref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;
        let next_review_date = input.next_review_date.as_ref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;

        let document = sqlx::query_as::<_, DocumentControl>(
            r#"
            INSERT INTO document_control (
                id, organization_id, document_number, document_type, title, description,
                version, revision_number, document_status, author_id, created_by,
                effective_date, next_review_date, department, applicable_to, keywords,
                acknowledgement_required
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, 1, 'DRAFT', $8, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id, organization_id, document_number, document_type, title, description,
                      version, revision_number, document_status, file_path, file_size,
                      file_mime_type, content_hash, author_id, reviewer_id, approver_id,
                      reviewed_at, approved_at, published_at, effective_date, expiry_date,
                      next_review_date, department, applicable_to, keywords, related_documents,
                      supersedes_document_id, view_count, last_viewed_at, acknowledgement_required,
                      created_by, created_at, updated_by, updated_at
            "#
        )
        .bind(id)
        .bind(org_id)
        .bind(&input.document_number)
        .bind(input.document_type)
        .bind(&input.title)
        .bind(&input.description)
        .bind(&version)
        .bind(author_id)
        .bind(effective_date)
        .bind(next_review_date)
        .bind(&input.department)
        .bind(input.applicable_to.unwrap_or_default())
        .bind(input.keywords.unwrap_or_default())
        .bind(input.acknowledgement_required.unwrap_or(false))
        .fetch_one(&self.pool)
        .await?;

        Ok(document)
    }

    pub async fn get_document(&self, id: Uuid) -> Result<DocumentControl> {
        let document = sqlx::query_as::<_, DocumentControl>(
            "SELECT * FROM document_control WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::NotFound(format!("Document with id {} not found", id)))?;

        Ok(document)
    }

    pub async fn get_documents(&self, org_id: Uuid, status: Option<DocumentStatus>, limit: i64, offset: i64) -> Result<Vec<DocumentControl>> {
        let mut query = String::from("SELECT * FROM document_control WHERE organization_id = $1 AND is_deleted = FALSE");

        if status.is_some() {
            query.push_str(" AND document_status = $2");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT $3 OFFSET $4");

        let mut query_builder = sqlx::query_as::<_, DocumentControl>(&query).bind(org_id);

        if let Some(s) = status {
            query_builder = query_builder.bind(s);
            query_builder = query_builder.bind(limit).bind(offset);
        } else {
            query_builder = query_builder.bind(limit).bind(offset);
        }

        let documents = query_builder.fetch_all(&self.pool).await?;
        Ok(documents)
    }

    pub async fn update_document(&self, id: Uuid, input: UpdateDocumentInput, user_id: Uuid) -> Result<DocumentControl> {
        let next_review_date = input.next_review_date.as_ref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;

        let document = sqlx::query_as::<_, DocumentControl>(
            r#"
            UPDATE document_control
            SET title = COALESCE($1, title),
                description = COALESCE($2, description),
                department = COALESCE($3, department),
                applicable_to = COALESCE($4, applicable_to),
                keywords = COALESCE($5, keywords),
                next_review_date = COALESCE($6, next_review_date),
                updated_by = $7,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $8 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&input.title)
        .bind(&input.description)
        .bind(&input.department)
        .bind(&input.applicable_to)
        .bind(&input.keywords)
        .bind(next_review_date)
        .bind(user_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::NotFound(format!("Document with id {} not found", id)))?;

        Ok(document)
    }

    pub async fn approve_document(&self, id: Uuid, input: ApproveDocumentInput, user_id: Uuid) -> Result<DocumentControl> {
        let approver_id = Uuid::parse_str(&input.approver_id).map_err(|e| Error::InvalidInput(e.to_string()))?;
        let reviewer_id = input.reviewer_id.as_ref()
            .map(|r| Uuid::parse_str(r).map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;
        let effective_date = input.effective_date.as_ref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;

        let document = sqlx::query_as::<_, DocumentControl>(
            r#"
            UPDATE document_control
            SET document_status = 'PUBLISHED',
                reviewer_id = COALESCE($1, reviewer_id),
                approver_id = $2,
                reviewed_at = CASE WHEN $1 IS NOT NULL THEN CURRENT_TIMESTAMP ELSE reviewed_at END,
                approved_at = CURRENT_TIMESTAMP,
                published_at = CURRENT_TIMESTAMP,
                effective_date = COALESCE($3, effective_date),
                updated_by = $4,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $5 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(reviewer_id)
        .bind(approver_id)
        .bind(effective_date)
        .bind(user_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::NotFound(format!("Document with id {} not found", id)))?;

        Ok(document)
    }

    pub async fn get_documents_pending_review(&self, org_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM document_control WHERE organization_id = $1 AND next_review_date < CURRENT_DATE AND document_status = 'PUBLISHED'"
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    // ========================================================================
    // CAPA
    // ========================================================================

    pub async fn create_capa(&self, input: CreateCAPAInput, org_id: Uuid, user_id: Uuid) -> Result<CAPA> {
        let id = Uuid::new_v4();
        let capa_number = self.generate_capa_number(org_id).await?;
        let date_identified = NaiveDate::parse_from_str(&input.date_identified, "%Y-%m-%d")
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        let assigned_to = input.assigned_to.as_ref()
            .map(|a| Uuid::parse_str(a).map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;
        let target_date = input.target_completion_date.as_ref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;

        let capa = sqlx::query_as::<_, CAPA>(
            r#"
            INSERT INTO capa (
                id, organization_id, capa_number, capa_type, priority, capa_status,
                title, description, source, source_reference, date_identified,
                assigned_to, target_completion_date, created_by
            )
            VALUES ($1, $2, $3, $4, $5, 'OPEN', $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(org_id)
        .bind(&capa_number)
        .bind(input.capa_type)
        .bind(input.priority)
        .bind(&input.title)
        .bind(&input.description)
        .bind(&input.source)
        .bind(&input.source_reference)
        .bind(date_identified)
        .bind(assigned_to)
        .bind(target_date)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(capa)
    }

    async fn generate_capa_number(&self, org_id: Uuid) -> Result<String> {
        let year = Utc::now().format("%Y");
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) + 1 FROM capa WHERE organization_id = $1 AND EXTRACT(YEAR FROM created_at) = EXTRACT(YEAR FROM CURRENT_DATE)"
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(format!("CAPA-{}-{:04}", year, count.0))
    }

    pub async fn get_capa(&self, id: Uuid) -> Result<CAPA> {
        let capa = sqlx::query_as::<_, CAPA>(
            "SELECT * FROM capa WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::NotFound(format!("CAPA with id {} not found", id)))?;

        Ok(capa)
    }

    pub async fn get_capas(&self, org_id: Uuid, status: Option<CAPAStatus>, limit: i64, offset: i64) -> Result<Vec<CAPA>> {
        let mut query = String::from("SELECT * FROM capa WHERE organization_id = $1 AND is_deleted = FALSE");

        if status.is_some() {
            query.push_str(" AND capa_status = $2");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT $3 OFFSET $4");

        let mut query_builder = sqlx::query_as::<_, CAPA>(&query).bind(org_id);

        if let Some(s) = status {
            query_builder = query_builder.bind(s);
            query_builder = query_builder.bind(limit).bind(offset);
        } else {
            query_builder = query_builder.bind(limit).bind(offset);
        }

        let capas = query_builder.fetch_all(&self.pool).await?;
        Ok(capas)
    }

    pub async fn update_capa(&self, id: Uuid, input: UpdateCAPAInput, user_id: Uuid) -> Result<CAPA> {
        let assigned_to = input.assigned_to.as_ref()
            .map(|a| Uuid::parse_str(a).map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;
        let target_date = input.target_completion_date.as_ref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;
        let actual_date = input.actual_completion_date.as_ref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;

        let capa = sqlx::query_as::<_, CAPA>(
            r#"
            UPDATE capa
            SET priority = COALESCE($1, priority),
                capa_status = COALESCE($2, capa_status),
                title = COALESCE($3, title),
                description = COALESCE($4, description),
                root_cause_analysis = COALESCE($5, root_cause_analysis),
                corrective_action = COALESCE($6, corrective_action),
                preventive_action = COALESCE($7, preventive_action),
                action_plan = COALESCE($8, action_plan),
                assigned_to = COALESCE($9, assigned_to),
                target_completion_date = COALESCE($10, target_completion_date),
                actual_completion_date = COALESCE($11, actual_completion_date),
                updated_by = $12,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $13 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(input.priority)
        .bind(input.capa_status)
        .bind(&input.title)
        .bind(&input.description)
        .bind(&input.root_cause_analysis)
        .bind(&input.corrective_action)
        .bind(&input.preventive_action)
        .bind(&input.action_plan)
        .bind(assigned_to)
        .bind(target_date)
        .bind(actual_date)
        .bind(user_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::NotFound(format!("CAPA with id {} not found", id)))?;

        Ok(capa)
    }

    pub async fn close_capa(&self, id: Uuid, input: CloseCAPAInput, user_id: Uuid) -> Result<CAPA> {
        let capa = sqlx::query_as::<_, CAPA>(
            r#"
            UPDATE capa
            SET capa_status = 'CLOSED',
                verification_method = $1,
                verified_by = $2,
                verification_date = CURRENT_DATE,
                verification_result = $3,
                effectiveness_check = $4,
                effectiveness_check_date = CASE WHEN $4 THEN CURRENT_DATE ELSE NULL END,
                closure_remarks = $5,
                closed_by = $2,
                closed_at = CURRENT_TIMESTAMP,
                updated_by = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $6 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&input.verification_method)
        .bind(user_id)
        .bind(&input.verification_result)
        .bind(input.effectiveness_check)
        .bind(&input.closure_remarks)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::NotFound(format!("CAPA with id {} not found", id)))?;

        Ok(capa)
    }

    pub async fn get_open_capas_count(&self, org_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM capa WHERE organization_id = $1 AND capa_status NOT IN ('CLOSED', 'CANCELLED') AND is_deleted = FALSE"
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    pub async fn get_overdue_capas_count(&self, org_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM capa WHERE organization_id = $1 AND capa_status NOT IN ('CLOSED', 'CANCELLED') AND target_completion_date < CURRENT_DATE AND is_deleted = FALSE"
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    // ========================================================================
    // TRAINING RECORDS
    // ========================================================================

    pub async fn create_training_record(&self, input: CreateTrainingRecordInput, org_id: Uuid, creator_id: Uuid) -> Result<TrainingRecord> {
        let id = Uuid::new_v4();
        let user_id = Uuid::parse_str(&input.user_id).map_err(|e| Error::InvalidInput(e.to_string()))?;
        let scheduled_date = input.scheduled_date.as_ref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| Error::InvalidInput(e.to_string())))
            .transpose()?;
        let duration = input.duration_hours.map(rust_decimal::Decimal::from_f64_retain).flatten();

        let record = sqlx::query_as::<_, TrainingRecord>(
            r#"
            INSERT INTO training_record (
                id, organization_id, user_id, training_type, training_status,
                training_title, training_description, trainer_name, training_method,
                scheduled_date, duration_hours, assessment_required, created_by
            )
            VALUES ($1, $2, $3, $4, 'SCHEDULED', $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(org_id)
        .bind(user_id)
        .bind(input.training_type)
        .bind(&input.training_title)
        .bind(&input.training_description)
        .bind(&input.trainer_name)
        .bind(&input.training_method)
        .bind(scheduled_date)
        .bind(duration)
        .bind(input.assessment_required.unwrap_or(false))
        .bind(creator_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_training_records(&self, org_id: Uuid, user_id: Option<Uuid>, limit: i64, offset: i64) -> Result<Vec<TrainingRecord>> {
        let mut query = String::from("SELECT * FROM training_record WHERE organization_id = $1");

        if user_id.is_some() {
            query.push_str(" AND user_id = $2");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT $3 OFFSET $4");

        let mut query_builder = sqlx::query_as::<_, TrainingRecord>(&query).bind(org_id);

        if let Some(uid) = user_id {
            query_builder = query_builder.bind(uid);
            query_builder = query_builder.bind(limit).bind(offset);
        } else {
            query_builder = query_builder.bind(limit).bind(offset);
        }

        let records = query_builder.fetch_all(&self.pool).await?;
        Ok(records)
    }

    pub async fn get_expired_trainings_count(&self, org_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM training_record WHERE organization_id = $1 AND certificate_expiry_date < CURRENT_DATE AND training_status = 'COMPLETED'"
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    // ========================================================================
    // QUALITY INDICATORS
    // ========================================================================

    pub async fn get_quality_indicators(&self, org_id: Uuid) -> Result<Vec<QualityIndicator>> {
        let indicators = sqlx::query_as::<_, QualityIndicator>(
            "SELECT * FROM quality_indicator WHERE organization_id = $1 AND is_active = TRUE ORDER BY indicator_name"
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(indicators)
    }

    pub async fn record_quality_indicator_value(&self, input: RecordQualityIndicatorInput, org_id: Uuid, user_id: Uuid) -> Result<QualityIndicatorValue> {
        let id = Uuid::new_v4();
        let indicator_id = Uuid::parse_str(&input.indicator_id).map_err(|e| Error::InvalidInput(e.to_string()))?;
        let measurement_date = NaiveDate::parse_from_str(&input.measurement_date, "%Y-%m-%d")
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        let measured_value = rust_decimal::Decimal::from_f64_retain(input.measured_value)
            .ok_or(Error::Validation("Invalid measured value".to_string()))?;

        // Get indicator thresholds to determine status
        let indicator: QualityIndicator = sqlx::query_as(
            "SELECT * FROM quality_indicator WHERE id = $1"
        )
        .bind(indicator_id)
        .fetch_one(&self.pool)
        .await?;

        let status = self.calculate_indicator_status(&indicator, measured_value);

        let value = sqlx::query_as::<_, QualityIndicatorValue>(
            r#"
            INSERT INTO quality_indicator_value (
                id, indicator_id, organization_id, measurement_date, measured_value,
                indicator_status, analysis_notes, action_taken, measured_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (indicator_id, measurement_date)
            DO UPDATE SET
                measured_value = EXCLUDED.measured_value,
                indicator_status = EXCLUDED.indicator_status,
                analysis_notes = EXCLUDED.analysis_notes,
                action_taken = EXCLUDED.action_taken,
                measured_by = EXCLUDED.measured_by
            RETURNING *
            "#
        )
        .bind(id)
        .bind(indicator_id)
        .bind(org_id)
        .bind(measurement_date)
        .bind(measured_value)
        .bind(status)
        .bind(&input.analysis_notes)
        .bind(&input.action_taken)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(value)
    }

    fn calculate_indicator_status(&self, indicator: &QualityIndicator, measured_value: rust_decimal::Decimal) -> IndicatorStatus {
        if let Some(critical) = indicator.threshold_critical {
            if measured_value >= critical {
                return IndicatorStatus::Critical;
            }
        }
        if let Some(warning) = indicator.threshold_warning {
            if measured_value >= warning {
                return IndicatorStatus::Warning;
            }
        }
        IndicatorStatus::OnTarget
    }

    pub async fn get_critical_indicators_count(&self, org_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT qiv.indicator_id)
            FROM quality_indicator_value qiv
            INNER JOIN (
                SELECT indicator_id, MAX(measurement_date) as latest_date
                FROM quality_indicator_value
                WHERE organization_id = $1
                GROUP BY indicator_id
            ) latest ON qiv.indicator_id = latest.indicator_id AND qiv.measurement_date = latest.latest_date
            WHERE qiv.organization_id = $1 AND qiv.indicator_status = 'CRITICAL'
            "#
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    // ========================================================================
    // COMPLIANCE ASSESSMENTS
    // ========================================================================

    pub async fn get_recent_assessments(&self, org_id: Uuid, limit: i64) -> Result<Vec<ComplianceAssessment>> {
        let assessments = sqlx::query_as::<_, ComplianceAssessment>(
            "SELECT * FROM compliance_assessment WHERE organization_id = $1 ORDER BY assessment_date DESC LIMIT $2"
        )
        .bind(org_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(assessments)
    }
}
