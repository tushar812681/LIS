use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::*;
use crate::repository::ComplianceRepository;
use crate::service::ComplianceService;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get audit logs with optional filtering
    async fn audit_logs(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        filter: Option<AuditLogFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AuditLog>> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let logs = service
            .get_audit_logs(org_id, filter, limit.unwrap_or(50), offset.unwrap_or(0))
            .await?;
        Ok(logs)
    }

    /// Get a specific document by ID
    async fn document(&self, ctx: &Context<'_>, id: String) -> Result<DocumentControl> {
        let document_id = Uuid::parse_str(&id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let document = service.get_document(document_id).await?;
        Ok(document)
    }

    /// Get documents for an organization
    async fn documents(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        status: Option<DocumentStatus>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<DocumentControl>> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let documents = service
            .get_documents(org_id, status, limit.unwrap_or(50), offset.unwrap_or(0))
            .await?;
        Ok(documents)
    }

    /// Get a specific CAPA by ID
    async fn capa(&self, ctx: &Context<'_>, id: String) -> Result<CAPA> {
        let capa_id = Uuid::parse_str(&id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let capa = service.get_capa(capa_id).await?;
        Ok(capa)
    }

    /// Get CAPAs for an organization
    async fn capas(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        status: Option<CAPAStatus>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CAPA>> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let capas = service
            .get_capas(org_id, status, limit.unwrap_or(50), offset.unwrap_or(0))
            .await?;
        Ok(capas)
    }

    /// Get training records
    async fn training_records(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        user_id: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<TrainingRecord>> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let uid = user_id.map(|id| Uuid::parse_str(&id)).transpose()?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let records = service
            .get_training_records(org_id, uid, limit.unwrap_or(50), offset.unwrap_or(0))
            .await?;
        Ok(records)
    }

    /// Get quality indicators for an organization
    async fn quality_indicators(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
    ) -> Result<Vec<QualityIndicator>> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let indicators = service.get_quality_indicators(org_id).await?;
        Ok(indicators)
    }

    /// Get compliance dashboard summary
    async fn compliance_dashboard(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
    ) -> Result<ComplianceDashboard> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let dashboard = service.get_compliance_dashboard(org_id).await?;
        Ok(dashboard)
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create an audit log entry
    async fn create_audit_log(
        &self,
        ctx: &Context<'_>,
        input: CreateAuditLogInput,
        organization_id: String,
        user_id: Option<String>,
    ) -> Result<AuditLog> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let uid = user_id.map(|id| Uuid::parse_str(&id)).transpose()?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let audit_log = service.create_audit_log(input, org_id, uid, None).await?;
        Ok(audit_log)
    }

    /// Create a new controlled document
    async fn create_document(
        &self,
        ctx: &Context<'_>,
        input: CreateDocumentInput,
        organization_id: String,
        author_id: String,
    ) -> Result<DocumentControl> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let user_id = Uuid::parse_str(&author_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let document = service.create_document(input, org_id, user_id).await?;

        tracing::info!("Document created: {} ({})", document.document_number, document.id);
        Ok(document)
    }

    /// Update a document
    async fn update_document(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateDocumentInput,
        user_id: String,
    ) -> Result<DocumentControl> {
        let document_id = Uuid::parse_str(&id)?;
        let uid = Uuid::parse_str(&user_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let document = service.update_document(document_id, input, uid).await?;
        Ok(document)
    }

    /// Approve and publish a document
    async fn approve_document(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: ApproveDocumentInput,
        user_id: String,
    ) -> Result<DocumentControl> {
        let document_id = Uuid::parse_str(&id)?;
        let uid = Uuid::parse_str(&user_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let document = service.approve_document(document_id, input, uid).await?;

        tracing::info!("Document approved: {} ({})", document.document_number, document.id);
        Ok(document)
    }

    /// Create a new CAPA
    async fn create_capa(
        &self,
        ctx: &Context<'_>,
        input: CreateCAPAInput,
        organization_id: String,
        created_by: String,
    ) -> Result<CAPA> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let user_id = Uuid::parse_str(&created_by)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let capa = service.create_capa(input, org_id, user_id).await?;
        Ok(capa)
    }

    /// Update a CAPA
    async fn update_capa(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateCAPAInput,
        user_id: String,
    ) -> Result<CAPA> {
        let capa_id = Uuid::parse_str(&id)?;
        let uid = Uuid::parse_str(&user_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let capa = service.update_capa(capa_id, input, uid).await?;
        Ok(capa)
    }

    /// Close a CAPA
    async fn close_capa(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: CloseCAPAInput,
        user_id: String,
    ) -> Result<CAPA> {
        let capa_id = Uuid::parse_str(&id)?;
        let uid = Uuid::parse_str(&user_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let capa = service.close_capa(capa_id, input, uid).await?;
        Ok(capa)
    }

    /// Create a training record
    async fn create_training_record(
        &self,
        ctx: &Context<'_>,
        input: CreateTrainingRecordInput,
        organization_id: String,
        created_by: String,
    ) -> Result<TrainingRecord> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let creator_id = Uuid::parse_str(&created_by)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let record = service.create_training_record(input, org_id, creator_id).await?;

        tracing::info!("Training record created: {} ({})", record.training_title, record.id);
        Ok(record)
    }

    /// Record a quality indicator value
    async fn record_quality_indicator_value(
        &self,
        ctx: &Context<'_>,
        input: RecordQualityIndicatorInput,
        organization_id: String,
        measured_by: String,
    ) -> Result<QualityIndicatorValue> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let user_id = Uuid::parse_str(&measured_by)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = ComplianceRepository::new(pool.clone());
        let service = ComplianceService::new(repository);

        let value = service
            .record_quality_indicator_value(input, org_id, user_id)
            .await?;
        Ok(value)
    }
}
