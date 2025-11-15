use async_graphql::{Context, Object, Result as GqlResult, ID, ErrorExtensions};
use crate::domain::*;
use crate::service::{ReportService, ReportError};
use uuid::Uuid;
use std::str::FromStr;

// Convert service errors to GraphQL errors
impl ErrorExtensions for ReportError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ============================================================================
    // Report Template Queries
    // ============================================================================

    /// Get report template by ID
    async fn report_template(&self, ctx: &Context<'_>, id: ID) -> GqlResult<ReportTemplate> {
        let service = ctx.data::<ReportService>()?;
        let template_id = Uuid::from_str(&id)?;
        let template = service.get_template(template_id).await?;
        Ok(template)
    }

    /// Get report template by code
    async fn report_template_by_code(
        &self,
        ctx: &Context<'_>,
        organization_id: ID,
        template_code: String,
    ) -> GqlResult<ReportTemplate> {
        let service = ctx.data::<ReportService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let template = service.get_template_by_code(org_id, &template_code).await?;
        Ok(template)
    }

    /// List report templates with optional filters
    async fn report_templates(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<ID>,
        template_type: Option<ReportTemplateType>,
        is_active: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<ReportTemplate>> {
        let service = ctx.data::<ReportService>()?;

        let filter = if organization_id.is_some() || template_type.is_some() || is_active.is_some() {
            Some(ReportTemplateFilter {
                organization_id: organization_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                template_type,
                is_active,
            })
        } else {
            None
        };

        let templates = service.list_templates(filter, page, page_size).await?;
        Ok(templates)
    }

    // ============================================================================
    // Generated Report Queries
    // ============================================================================

    /// Get generated report by ID
    async fn report(&self, ctx: &Context<'_>, id: ID) -> GqlResult<GeneratedReport> {
        let service = ctx.data::<ReportService>()?;
        let report_id = Uuid::from_str(&id)?;
        let report = service.get_report(report_id).await?;
        Ok(report)
    }

    /// Get generated report by report number
    async fn report_by_number(
        &self,
        ctx: &Context<'_>,
        report_number: String,
    ) -> GqlResult<GeneratedReport> {
        let service = ctx.data::<ReportService>()?;
        let report = service.get_report_by_number(&report_number).await?;
        Ok(report)
    }

    /// List generated reports with optional filters
    async fn reports(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<ID>,
        patient_id: Option<ID>,
        order_id: Option<ID>,
        report_type: Option<ReportTemplateType>,
        report_status: Option<ReportStatus>,
        from_date: Option<String>,
        to_date: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<GeneratedReport>> {
        let service = ctx.data::<ReportService>()?;

        let filter = if organization_id.is_some() || patient_id.is_some() || order_id.is_some()
            || report_type.is_some() || report_status.is_some() || from_date.is_some() || to_date.is_some() {
            Some(GeneratedReportFilter {
                organization_id: organization_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                patient_id: patient_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                order_id: order_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                report_type,
                report_status,
                from_date: from_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
                to_date: to_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            })
        } else {
            None
        };

        let reports = service.list_reports(filter, page, page_size).await?;
        Ok(reports)
    }

    // ============================================================================
    // Digital Signature Queries
    // ============================================================================

    /// Get digital signatures for a report
    async fn report_signatures(
        &self,
        ctx: &Context<'_>,
        report_id: ID,
    ) -> GqlResult<Vec<DigitalSignature>> {
        let service = ctx.data::<ReportService>()?;
        let report_uuid = Uuid::from_str(&report_id)?;
        let signatures = service.get_report_signatures(report_uuid).await?;
        Ok(signatures)
    }

    // ============================================================================
    // Report Delivery Queries
    // ============================================================================

    /// Get delivery by ID
    async fn delivery(&self, ctx: &Context<'_>, id: ID) -> GqlResult<ReportDelivery> {
        let service = ctx.data::<ReportService>()?;
        let delivery_id = Uuid::from_str(&id)?;
        let delivery = service.get_delivery(delivery_id).await?;
        Ok(delivery)
    }

    /// List deliveries with optional filters
    async fn deliveries(
        &self,
        ctx: &Context<'_>,
        report_id: Option<ID>,
        delivery_channel: Option<DeliveryChannel>,
        delivery_status: Option<DeliveryStatus>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<ReportDelivery>> {
        let service = ctx.data::<ReportService>()?;

        let filter = if report_id.is_some() || delivery_channel.is_some() || delivery_status.is_some() {
            Some(ReportDeliveryFilter {
                report_id: report_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                delivery_channel,
                delivery_status,
            })
        } else {
            None
        };

        let deliveries = service.list_deliveries(filter, page, page_size).await?;
        Ok(deliveries)
    }

    // ============================================================================
    // Report Access Log Queries
    // ============================================================================

    /// Get access logs for a report
    async fn report_access_logs(
        &self,
        ctx: &Context<'_>,
        report_id: ID,
    ) -> GqlResult<Vec<ReportAccessLog>> {
        let service = ctx.data::<ReportService>()?;
        let report_uuid = Uuid::from_str(&report_id)?;
        let logs = service.get_access_logs(report_uuid).await?;
        Ok(logs)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // ============================================================================
    // Report Template Mutations
    // ============================================================================

    /// Create a new report template
    async fn create_report_template(
        &self,
        ctx: &Context<'_>,
        input: CreateReportTemplateInput,
        created_by: ID,
    ) -> GqlResult<ReportTemplate> {
        let service = ctx.data::<ReportService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let template = service.create_template(input, creator_id).await?;
        Ok(template)
    }

    // ============================================================================
    // Report Generation Mutations
    // ============================================================================

    /// Generate a new report
    async fn generate_report(
        &self,
        ctx: &Context<'_>,
        input: GenerateReportInput,
        created_by: ID,
    ) -> GqlResult<GeneratedReport> {
        let service = ctx.data::<ReportService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let report = service.generate_report(input, creator_id).await?;
        Ok(report)
    }

    /// Download a report (logs access)
    async fn download_report(
        &self,
        ctx: &Context<'_>,
        report_id: ID,
        accessed_by: Option<ID>,
        ip_address: Option<String>,
    ) -> GqlResult<GeneratedReport> {
        let service = ctx.data::<ReportService>()?;
        let report_uuid = Uuid::from_str(&report_id)?;
        let accessor_id = accessed_by.as_ref().and_then(|id| Uuid::from_str(id).ok());
        let report = service.download_report(report_uuid, accessor_id, ip_address).await?;
        Ok(report)
    }

    /// Verify access code for a report
    async fn verify_report_access(
        &self,
        ctx: &Context<'_>,
        report_number: String,
        access_code: String,
    ) -> GqlResult<GeneratedReport> {
        let service = ctx.data::<ReportService>()?;
        let report = service.verify_access_code(&report_number, &access_code).await?;
        Ok(report)
    }

    // ============================================================================
    // Digital Signature Mutations
    // ============================================================================

    /// Sign a report
    async fn sign_report(
        &self,
        ctx: &Context<'_>,
        input: SignReportInput,
    ) -> GqlResult<DigitalSignature> {
        let service = ctx.data::<ReportService>()?;
        let signature = service.sign_report(input).await?;
        Ok(signature)
    }

    // ============================================================================
    // Report Delivery Mutations
    // ============================================================================

    /// Deliver a report via specified channel
    async fn deliver_report(
        &self,
        ctx: &Context<'_>,
        input: DeliverReportInput,
        created_by: ID,
    ) -> GqlResult<ReportDelivery> {
        let service = ctx.data::<ReportService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let delivery = service.deliver_report(input, creator_id).await?;
        Ok(delivery)
    }

    /// Retry a failed delivery
    async fn retry_delivery(
        &self,
        ctx: &Context<'_>,
        delivery_id: ID,
    ) -> GqlResult<ReportDelivery> {
        let service = ctx.data::<ReportService>()?;
        let delivery_uuid = Uuid::from_str(&delivery_id)?;
        let delivery = service.retry_delivery(delivery_uuid).await?;
        Ok(delivery)
    }

    // ============================================================================
    // Report Access Mutations
    // ============================================================================

    /// Log report access
    async fn log_report_access(
        &self,
        ctx: &Context<'_>,
        input: LogReportAccessInput,
    ) -> GqlResult<ReportAccessLog> {
        let service = ctx.data::<ReportService>()?;
        let log = service.log_access(input).await?;
        Ok(log)
    }
}
