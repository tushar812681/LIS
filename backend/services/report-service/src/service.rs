use crate::domain::*;
use crate::repository::*;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug)]
pub enum ReportError {
    NotFound(String),
    ValidationError(String),
    GenerationFailed(String),
    SignatureRequired,
    ReportNotReady,
    AccessDenied,
    DatabaseError(String),
}

impl std::fmt::Display for ReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::GenerationFailed(msg) => write!(f, "Report generation failed: {}", msg),
            Self::SignatureRequired => write!(f, "Digital signature required before delivery"),
            Self::ReportNotReady => write!(f, "Report is not yet generated or failed"),
            Self::AccessDenied => write!(f, "Access denied to this report"),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for ReportError {}

impl From<Error> for ReportError {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound(msg) => ReportError::NotFound(msg),
            Error::Database(msg) => ReportError::DatabaseError(msg),
            Error::InvalidInput(msg) => ReportError::ValidationError(msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, ReportError>;

#[derive(Clone)]
pub struct ReportService {
    template_repo: ReportTemplateRepository,
    report_repo: GeneratedReportRepository,
    signature_repo: DigitalSignatureRepository,
    delivery_repo: ReportDeliveryRepository,
    access_log_repo: ReportAccessLogRepository,
}

impl ReportService {
    pub fn new(
        template_repo: ReportTemplateRepository,
        report_repo: GeneratedReportRepository,
        signature_repo: DigitalSignatureRepository,
        delivery_repo: ReportDeliveryRepository,
        access_log_repo: ReportAccessLogRepository,
    ) -> Self {
        Self {
            template_repo,
            report_repo,
            signature_repo,
            delivery_repo,
            access_log_repo,
        }
    }

    // ============================================================================
    // Report Template Operations
    // ============================================================================

    /// Create a new report template
    pub async fn create_template(
        &self,
        input: CreateReportTemplateInput,
        created_by: Uuid,
    ) -> Result<ReportTemplate> {
        // Validate inputs
        if input.template_name.is_empty() {
            return Err(ReportError::ValidationError("Template name is required".to_string()));
        }

        if input.template_code.is_empty() {
            return Err(ReportError::ValidationError("Template code is required".to_string()));
        }

        // Validate JSON content
        serde_json::from_str::<serde_json::Value>(&input.template_content)
            .map_err(|e| ReportError::ValidationError(format!("Invalid template content JSON: {}", e)))?;

        let template = self.template_repo.create(input, created_by).await?;
        Ok(template)
    }

    /// Get template by ID
    pub async fn get_template(&self, template_id: Uuid) -> Result<ReportTemplate> {
        let template = self.template_repo.get_by_id(template_id).await?;
        Ok(template)
    }

    /// Get template by code
    pub async fn get_template_by_code(
        &self,
        organization_id: Uuid,
        template_code: &str,
    ) -> Result<ReportTemplate> {
        let template = self.template_repo.get_by_code(organization_id, template_code).await?;
        Ok(template)
    }

    /// List templates with filters
    pub async fn list_templates(
        &self,
        filter: Option<ReportTemplateFilter>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<Vec<ReportTemplate>> {
        let templates = self.template_repo.list(filter, page, page_size).await?;
        Ok(templates)
    }

    // ============================================================================
    // Report Generation Operations
    // ============================================================================

    /// Generate a new report
    pub async fn generate_report(
        &self,
        input: GenerateReportInput,
        created_by: Uuid,
    ) -> Result<GeneratedReport> {
        // Validate inputs
        if input.report_title.is_empty() {
            return Err(ReportError::ValidationError("Report title is required".to_string()));
        }

        // Validate JSON data
        let report_data: serde_json::Value = serde_json::from_str(&input.report_data)
            .map_err(|e| ReportError::ValidationError(format!("Invalid report data JSON: {}", e)))?;

        // Get template if specified
        let template = if let Some(template_id) = input.template_id {
            Some(self.template_repo.get_by_id(template_id).await?)
        } else {
            // Try to get default template
            self.template_repo
                .get_default_template(input.organization_id, input.report_type)
                .await
                .ok()
        };

        // Create report entry
        let mut report = self.report_repo.create(input, created_by).await?;

        // Update status to generating
        report = self.report_repo.update_status(
            report.id,
            ReportStatus::Generating,
            None,
        ).await?;

        // Generate PDF content
        match self.generate_pdf_content(&report, &report_data, template.as_ref()).await {
            Ok((file_path, file_size, file_hash)) => {
                // Update report with file info
                report = self.report_repo.update_file_info(
                    report.id,
                    file_path,
                    file_size,
                    file_hash,
                ).await?;

                // Update status to generated
                report = self.report_repo.update_status(
                    report.id,
                    ReportStatus::Generated,
                    None,
                ).await?;
            },
            Err(e) => {
                // Mark as failed
                let _report = self.report_repo.update_status(
                    report.id,
                    ReportStatus::Failed,
                    Some(e.clone()),
                ).await?;
                return Err(ReportError::GenerationFailed(e));
            }
        }

        Ok(report)
    }

    /// Generate PDF content (simplified - in production, use proper PDF generation library)
    async fn generate_pdf_content(
        &self,
        report: &GeneratedReport,
        data: &serde_json::Value,
        _template: Option<&ReportTemplate>,
    ) -> std::result::Result<(String, i64, String), String> {
        // In a real implementation, this would:
        // 1. Use the template to generate HTML
        // 2. Convert HTML to PDF using printpdf or similar
        // 3. Save to file system or S3
        // 4. Return file path, size, and hash

        // For now, create a simple placeholder
        let data_str = serde_json::to_string_pretty(data)
            .map_err(|e| e.to_string())?;

        let content = format!(
            "Report: {}\nType: {:?}\nDate: {}\nData: {}",
            report.report_title,
            report.report_type,
            report.report_date,
            data_str
        );

        // Calculate hash
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        // In production: save to file system/S3
        let file_path = format!("/tmp/reports/{}.pdf", report.report_number);
        let file_size = content.len() as i64;

        Ok((file_path, file_size, hash))
    }

    /// Get report by ID
    pub async fn get_report(&self, report_id: Uuid) -> Result<GeneratedReport> {
        let report = self.report_repo.get_by_id(report_id).await?;
        Ok(report)
    }

    /// Get report by number
    pub async fn get_report_by_number(&self, report_number: &str) -> Result<GeneratedReport> {
        let report = self.report_repo.get_by_report_number(report_number).await?;
        Ok(report)
    }

    /// List reports with filters
    pub async fn list_reports(
        &self,
        filter: Option<GeneratedReportFilter>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<Vec<GeneratedReport>> {
        let reports = self.report_repo.list(filter, page, page_size).await?;
        Ok(reports)
    }

    // ============================================================================
    // Digital Signature Operations
    // ============================================================================

    /// Sign a report
    pub async fn sign_report(&self, input: SignReportInput) -> Result<DigitalSignature> {
        // Get report and validate
        let report = self.report_repo.get_by_id(input.report_id).await?;

        if report.report_status != ReportStatus::Generated {
            return Err(ReportError::ReportNotReady);
        }

        if report.is_signed.unwrap_or(false) {
            return Err(ReportError::ValidationError(
                "Report is already signed".to_string()
            ));
        }

        // Generate signature hash (hash of report content + signatory + timestamp)
        let signature_content = format!(
            "{}|{}|{}",
            report.file_hash.as_ref().unwrap_or(&String::new()),
            input.signatory_id,
            chrono::Local::now().to_rfc3339()
        );
        let mut hasher = Sha256::new();
        hasher.update(signature_content.as_bytes());
        let signature_hash = format!("{:x}", hasher.finalize());

        // Create signature
        let signature = self.signature_repo.create(input.clone(), signature_hash).await?;

        // Mark report as signed
        self.report_repo.mark_as_signed(input.report_id, input.signatory_id).await?;

        Ok(signature)
    }

    /// Get signatures for a report
    pub async fn get_report_signatures(&self, report_id: Uuid) -> Result<Vec<DigitalSignature>> {
        let signatures = self.signature_repo.get_by_report_id(report_id).await?;
        Ok(signatures)
    }

    // ============================================================================
    // Report Delivery Operations
    // ============================================================================

    /// Deliver a report via specified channel
    pub async fn deliver_report(
        &self,
        input: DeliverReportInput,
        created_by: Uuid,
    ) -> Result<ReportDelivery> {
        // Get report and validate
        let report = self.report_repo.get_by_id(input.report_id).await?;

        if report.report_status != ReportStatus::Generated {
            return Err(ReportError::ReportNotReady);
        }

        // Check if signature is required but not present
        if report.requires_signature.unwrap_or(false) && !report.is_signed.unwrap_or(false) {
            return Err(ReportError::SignatureRequired);
        }

        // Validate recipient contact based on channel
        match input.delivery_channel {
            DeliveryChannel::Email => {
                if !input.recipient_contact.contains('@') {
                    return Err(ReportError::ValidationError(
                        "Invalid email address".to_string()
                    ));
                }
            },
            DeliveryChannel::Whatsapp | DeliveryChannel::Sms => {
                if input.recipient_contact.len() < 10 {
                    return Err(ReportError::ValidationError(
                        "Invalid phone number".to_string()
                    ));
                }
            },
            _ => {}
        }

        // Create delivery record
        let delivery = self.delivery_repo.create(input, created_by).await?;

        // In production: trigger actual delivery via external service (SendGrid, Twilio, etc.)
        // For now, just mark as sent
        let delivery = self.delivery_repo.update_status(
            delivery.id,
            DeliveryStatus::Sent,
            Some(format!("MSG_{}", Uuid::new_v4())),
            None,
        ).await?;

        Ok(delivery)
    }

    /// Get delivery status
    pub async fn get_delivery(&self, delivery_id: Uuid) -> Result<ReportDelivery> {
        let delivery = self.delivery_repo.get_by_id(delivery_id).await?;
        Ok(delivery)
    }

    /// List deliveries with filters
    pub async fn list_deliveries(
        &self,
        filter: Option<ReportDeliveryFilter>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<Vec<ReportDelivery>> {
        let deliveries = self.delivery_repo.list(filter, page, page_size).await?;
        Ok(deliveries)
    }

    /// Retry failed delivery
    pub async fn retry_delivery(&self, delivery_id: Uuid) -> Result<ReportDelivery> {
        let delivery = self.delivery_repo.get_by_id(delivery_id).await?;

        if delivery.delivery_status != Some(DeliveryStatus::Failed) {
            return Err(ReportError::ValidationError(
                "Can only retry failed deliveries".to_string()
            ));
        }

        if !delivery.can_retry() {
            return Err(ReportError::ValidationError(
                "Maximum retry attempts reached".to_string()
            ));
        }

        // Reset to pending status for retry
        let delivery = self.delivery_repo.update_status(
            delivery_id,
            DeliveryStatus::Pending,
            None,
            None,
        ).await?;

        Ok(delivery)
    }

    // ============================================================================
    // Report Access Operations
    // ============================================================================

    /// Log report access
    pub async fn log_access(&self, input: LogReportAccessInput) -> Result<ReportAccessLog> {
        let log = self.access_log_repo.log_access(input).await?;
        Ok(log)
    }

    /// Get access logs for a report
    pub async fn get_access_logs(&self, report_id: Uuid) -> Result<Vec<ReportAccessLog>> {
        let logs = self.access_log_repo.get_report_access_logs(report_id).await?;
        Ok(logs)
    }

    /// Verify access code
    pub async fn verify_access_code(
        &self,
        report_number: &str,
        access_code: &str,
    ) -> Result<GeneratedReport> {
        let report = self.report_repo.get_by_report_number(report_number).await?;

        if let Some(stored_code) = &report.access_code {
            if stored_code == access_code {
                return Ok(report);
            }
        }

        Err(ReportError::AccessDenied)
    }

    /// Download report (generates access log entry)
    pub async fn download_report(
        &self,
        report_id: Uuid,
        accessed_by: Option<Uuid>,
        ip_address: Option<String>,
    ) -> Result<GeneratedReport> {
        let report = self.report_repo.get_by_id(report_id).await?;

        // Check if report is downloadable
        if !report.is_downloadable() {
            return Err(ReportError::ReportNotReady);
        }

        // Log access
        let _ = self.access_log_repo.log_access(LogReportAccessInput {
            report_id,
            accessed_by,
            access_code_used: None,
            ip_address,
            user_agent: None,
            access_method: "DOWNLOAD".to_string(),
            session_id: None,
        }).await;

        Ok(report)
    }
}
