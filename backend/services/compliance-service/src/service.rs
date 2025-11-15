use uuid::Uuid;

use crate::domain::*;
use crate::repository::ComplianceRepository;
use common::error::Result;

#[derive(Clone)]
pub struct ComplianceService {
    repository: ComplianceRepository,
}

impl ComplianceService {
    pub fn new(repository: ComplianceRepository) -> Self {
        Self { repository }
    }

    // ========================================================================
    // AUDIT LOG
    // ========================================================================

    pub async fn create_audit_log(
        &self,
        input: CreateAuditLogInput,
        org_id: Uuid,
        user_id: Option<Uuid>,
        session_id: Option<Uuid>,
    ) -> Result<AuditLog> {
        self.repository.create_audit_log(input, org_id, user_id, session_id).await
    }

    pub async fn get_audit_logs(
        &self,
        org_id: Uuid,
        filter: Option<AuditLogFilter>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditLog>> {
        self.repository.get_audit_logs(org_id, filter, limit, offset).await
    }

    // ========================================================================
    // DOCUMENT CONTROL
    // ========================================================================

    pub async fn create_document(
        &self,
        input: CreateDocumentInput,
        org_id: Uuid,
        author_id: Uuid,
    ) -> Result<DocumentControl> {
        // Validate input
        if input.document_number.trim().is_empty() {
            return Err(common::error::Error::Validation(
                "Document number is required".to_string(),
            ));
        }
        if input.title.trim().is_empty() {
            return Err(common::error::Error::Validation(
                "Document title is required".to_string(),
            ));
        }

        let document = self.repository.create_document(input.clone(), org_id, author_id).await?;

        // Log the document creation
        let audit_input = CreateAuditLogInput {
            entity_type: "DOCUMENT".to_string(),
            entity_id: document.id.to_string(),
            action: "CREATE".to_string(),
            old_value: None,
            new_value: Some(serde_json::to_value(&document).unwrap_or_default()),
            changes: None,
            reason: Some(format!("Created document: {}", input.title)),
            ip_address: None,
            user_agent: None,
        };
        let _ = self.repository.create_audit_log(audit_input, org_id, Some(author_id), None).await;

        Ok(document)
    }

    pub async fn get_document(&self, id: Uuid) -> Result<DocumentControl> {
        self.repository.get_document(id).await
    }

    pub async fn get_documents(
        &self,
        org_id: Uuid,
        status: Option<DocumentStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DocumentControl>> {
        self.repository.get_documents(org_id, status, limit, offset).await
    }

    pub async fn update_document(
        &self,
        id: Uuid,
        input: UpdateDocumentInput,
        user_id: Uuid,
    ) -> Result<DocumentControl> {
        let old_document = self.repository.get_document(id).await?;
        let updated_document = self.repository.update_document(id, input.clone(), user_id).await?;

        // Log the update
        let audit_input = CreateAuditLogInput {
            entity_type: "DOCUMENT".to_string(),
            entity_id: id.to_string(),
            action: "UPDATE".to_string(),
            old_value: Some(serde_json::to_value(&old_document).unwrap_or_default()),
            new_value: Some(serde_json::to_value(&updated_document).unwrap_or_default()),
            changes: None,
            reason: Some("Document updated".to_string()),
            ip_address: None,
            user_agent: None,
        };
        let _ = self.repository.create_audit_log(audit_input, updated_document.organization_id, Some(user_id), None).await;

        Ok(updated_document)
    }

    pub async fn approve_document(
        &self,
        id: Uuid,
        input: ApproveDocumentInput,
        user_id: Uuid,
    ) -> Result<DocumentControl> {
        let old_document = self.repository.get_document(id).await?;

        // Validate approval workflow
        if old_document.document_status == DocumentStatus::Published {
            return Err(common::error::Error::Validation(
                "Document is already published".to_string(),
            ));
        }

        let approved_document = self.repository.approve_document(id, input.clone(), user_id).await?;

        // Log the approval
        let audit_input = CreateAuditLogInput {
            entity_type: "DOCUMENT".to_string(),
            entity_id: id.to_string(),
            action: "APPROVE".to_string(),
            old_value: Some(serde_json::to_value(&old_document).unwrap_or_default()),
            new_value: Some(serde_json::to_value(&approved_document).unwrap_or_default()),
            changes: None,
            reason: Some("Document approved and published".to_string()),
            ip_address: None,
            user_agent: None,
        };
        let _ = self.repository.create_audit_log(audit_input, approved_document.organization_id, Some(user_id), None).await;

        Ok(approved_document)
    }

    // ========================================================================
    // CAPA (Corrective and Preventive Actions)
    // ========================================================================

    pub async fn create_capa(
        &self,
        input: CreateCAPAInput,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<CAPA> {
        // Validate input
        input.validate()?;

        let capa = self.repository.create_capa(input.clone(), org_id, user_id).await?;

        // Log the CAPA creation
        let audit_input = CreateAuditLogInput {
            entity_type: "CAPA".to_string(),
            entity_id: capa.id.to_string(),
            action: "CREATE".to_string(),
            old_value: None,
            new_value: Some(serde_json::to_value(&capa).unwrap_or_default()),
            changes: None,
            reason: Some(format!("Created CAPA: {}", input.title)),
            ip_address: None,
            user_agent: None,
        };
        let _ = self.repository.create_audit_log(audit_input, org_id, Some(user_id), None).await;

        tracing::info!("CAPA created: {} ({})", capa.capa_number, capa.id);
        Ok(capa)
    }

    pub async fn get_capa(&self, id: Uuid) -> Result<CAPA> {
        self.repository.get_capa(id).await
    }

    pub async fn get_capas(
        &self,
        org_id: Uuid,
        status: Option<CAPAStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CAPA>> {
        self.repository.get_capas(org_id, status, limit, offset).await
    }

    pub async fn update_capa(
        &self,
        id: Uuid,
        input: UpdateCAPAInput,
        user_id: Uuid,
    ) -> Result<CAPA> {
        let old_capa = self.repository.get_capa(id).await?;
        let updated_capa = self.repository.update_capa(id, input.clone(), user_id).await?;

        // Log the update
        let audit_input = CreateAuditLogInput {
            entity_type: "CAPA".to_string(),
            entity_id: id.to_string(),
            action: "UPDATE".to_string(),
            old_value: Some(serde_json::to_value(&old_capa).unwrap_or_default()),
            new_value: Some(serde_json::to_value(&updated_capa).unwrap_or_default()),
            changes: None,
            reason: Some("CAPA updated".to_string()),
            ip_address: None,
            user_agent: None,
        };
        let _ = self.repository.create_audit_log(audit_input, updated_capa.organization_id, Some(user_id), None).await;

        Ok(updated_capa)
    }

    pub async fn close_capa(
        &self,
        id: Uuid,
        input: CloseCAPAInput,
        user_id: Uuid,
    ) -> Result<CAPA> {
        let old_capa = self.repository.get_capa(id).await?;

        // Validate closure
        if old_capa.capa_status == CAPAStatus::Closed {
            return Err(common::error::Error::Validation(
                "CAPA is already closed".to_string(),
            ));
        }

        if input.verification_method.trim().is_empty() {
            return Err(common::error::Error::Validation(
                "Verification method is required for closure".to_string(),
            ));
        }

        let closed_capa = self.repository.close_capa(id, input.clone(), user_id).await?;

        // Log the closure
        let audit_input = CreateAuditLogInput {
            entity_type: "CAPA".to_string(),
            entity_id: id.to_string(),
            action: "CLOSE".to_string(),
            old_value: Some(serde_json::to_value(&old_capa).unwrap_or_default()),
            new_value: Some(serde_json::to_value(&closed_capa).unwrap_or_default()),
            changes: None,
            reason: Some(format!("CAPA closed: {}", input.closure_remarks)),
            ip_address: None,
            user_agent: None,
        };
        let _ = self.repository.create_audit_log(audit_input, closed_capa.organization_id, Some(user_id), None).await;

        tracing::info!("CAPA closed: {} ({})", closed_capa.capa_number, closed_capa.id);
        Ok(closed_capa)
    }

    // ========================================================================
    // TRAINING RECORDS
    // ========================================================================

    pub async fn create_training_record(
        &self,
        input: CreateTrainingRecordInput,
        org_id: Uuid,
        creator_id: Uuid,
    ) -> Result<TrainingRecord> {
        if input.training_title.trim().is_empty() {
            return Err(common::error::Error::Validation(
                "Training title is required".to_string(),
            ));
        }

        let record = self.repository.create_training_record(input.clone(), org_id, creator_id).await?;

        // Log the training record creation
        let audit_input = CreateAuditLogInput {
            entity_type: "TRAINING".to_string(),
            entity_id: record.id.to_string(),
            action: "CREATE".to_string(),
            old_value: None,
            new_value: Some(serde_json::to_value(&record).unwrap_or_default()),
            changes: None,
            reason: Some(format!("Created training record: {}", input.training_title)),
            ip_address: None,
            user_agent: None,
        };
        let _ = self.repository.create_audit_log(audit_input, org_id, Some(creator_id), None).await;

        Ok(record)
    }

    pub async fn get_training_records(
        &self,
        org_id: Uuid,
        user_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TrainingRecord>> {
        self.repository.get_training_records(org_id, user_id, limit, offset).await
    }

    // ========================================================================
    // QUALITY INDICATORS
    // ========================================================================

    pub async fn get_quality_indicators(&self, org_id: Uuid) -> Result<Vec<QualityIndicator>> {
        self.repository.get_quality_indicators(org_id).await
    }

    pub async fn record_quality_indicator_value(
        &self,
        input: RecordQualityIndicatorInput,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<QualityIndicatorValue> {
        let value = self.repository.record_quality_indicator_value(input.clone(), org_id, user_id).await?;

        // Log critical values
        if value.indicator_status == IndicatorStatus::Critical {
            let audit_input = CreateAuditLogInput {
                entity_type: "QUALITY_INDICATOR".to_string(),
                entity_id: value.indicator_id.to_string(),
                action: "CRITICAL_VALUE".to_string(),
                old_value: None,
                new_value: Some(serde_json::to_value(&value).unwrap_or_default()),
                changes: None,
                reason: Some(format!("Critical quality indicator value recorded: {}", value.measured_value)),
                ip_address: None,
                user_agent: None,
            };
            let _ = self.repository.create_audit_log(audit_input, org_id, Some(user_id), None).await;

            tracing::warn!("Critical quality indicator value: {} = {}", value.indicator_id, value.measured_value);
        }

        Ok(value)
    }

    // ========================================================================
    // COMPLIANCE DASHBOARD
    // ========================================================================

    pub async fn get_compliance_dashboard(&self, org_id: Uuid) -> Result<ComplianceDashboard> {
        let open_capas = self.repository.get_open_capas_count(org_id).await?;
        let overdue_capas = self.repository.get_overdue_capas_count(org_id).await?;
        let pending_document_reviews = self.repository.get_documents_pending_review(org_id).await?;
        let expired_trainings = self.repository.get_expired_trainings_count(org_id).await?;
        let quality_indicators_critical = self.repository.get_critical_indicators_count(org_id).await?;
        let recent_assessments = self.repository.get_recent_assessments(org_id, 5).await?;

        Ok(ComplianceDashboard {
            organization_id: org_id,
            open_capas,
            overdue_capas,
            pending_document_reviews,
            expired_trainings,
            quality_indicators_critical,
            recent_assessments,
            upcoming_audits: vec![], // TODO: Implement upcoming audits query
        })
    }
}
