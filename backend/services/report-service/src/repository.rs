use crate::domain::*;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{NaiveDate, Local};

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    Database(String),
    InvalidInput(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// ============================================================================
// Report Template Repository
// ============================================================================

#[derive(Clone)]
pub struct ReportTemplateRepository {
    pool: PgPool,
}

impl ReportTemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateReportTemplateInput, created_by: Uuid) -> Result<ReportTemplate> {
        let template_content: serde_json::Value = serde_json::from_str(&input.template_content)
            .map_err(|e| Error::InvalidInput(format!("Invalid template_content JSON: {}", e)))?;

        let header_content: Option<serde_json::Value> = input.header_content
            .map(|s| serde_json::from_str(&s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid header_content JSON: {}", e)))?;

        let footer_content: Option<serde_json::Value> = input.footer_content
            .map(|s| serde_json::from_str(&s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid footer_content JSON: {}", e)))?;

        let styles: Option<serde_json::Value> = input.styles
            .map(|s| serde_json::from_str(&s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid styles JSON: {}", e)))?;

        let fields_config: Option<serde_json::Value> = input.fields_config
            .map(|s| serde_json::from_str(&s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid fields_config JSON: {}", e)))?;

        let template = sqlx::query_as::<_, ReportTemplate>(
            r#"
            INSERT INTO report_template (
                organization_id, template_name, template_code, template_type, description,
                template_content, header_content, footer_content, styles, fields_config,
                page_size, page_orientation, requires_signature, is_default, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.template_name)
        .bind(input.template_code)
        .bind(input.template_type)
        .bind(input.description)
        .bind(template_content)
        .bind(header_content)
        .bind(footer_content)
        .bind(styles)
        .bind(fields_config)
        .bind(input.page_size)
        .bind(input.page_orientation)
        .bind(input.requires_signature.unwrap_or(false))
        .bind(input.is_default.unwrap_or(false))
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(template)
    }

    pub async fn get_by_id(&self, template_id: Uuid) -> Result<ReportTemplate> {
        let template = sqlx::query_as::<_, ReportTemplate>(
            "SELECT * FROM report_template WHERE id = $1 AND is_deleted = false"
        )
        .bind(template_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Template with ID {} not found", template_id)))?;

        Ok(template)
    }

    pub async fn get_by_code(&self, organization_id: Uuid, template_code: &str) -> Result<ReportTemplate> {
        let template = sqlx::query_as::<_, ReportTemplate>(
            "SELECT * FROM report_template WHERE organization_id = $1 AND template_code = $2 AND is_deleted = false"
        )
        .bind(organization_id)
        .bind(template_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Template '{}' not found", template_code)))?;

        Ok(template)
    }

    pub async fn list(
        &self,
        filter: Option<ReportTemplateFilter>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<Vec<ReportTemplate>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM report_template WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(org_id) = f.organization_id {
                query.push_str(&format!(" AND organization_id = '{}'", org_id));
            }
            if let Some(template_type) = f.template_type {
                let type_str = match template_type {
                    ReportTemplateType::PatientReport => "PATIENT_REPORT",
                    ReportTemplateType::BatchReport => "BATCH_REPORT",
                    ReportTemplateType::QcReport => "QC_REPORT",
                    ReportTemplateType::SummaryReport => "SUMMARY_REPORT",
                    ReportTemplateType::CustomReport => "CUSTOM_REPORT",
                };
                query.push_str(&format!(" AND template_type = '{}'", type_str));
            }
            if let Some(is_active) = f.is_active {
                query.push_str(&format!(" AND is_active = {}", is_active));
            }
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", page_size, offset));

        let templates = sqlx::query_as::<_, ReportTemplate>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(templates)
    }

    pub async fn get_default_template(&self, organization_id: Uuid, template_type: ReportTemplateType) -> Result<ReportTemplate> {
        let template = sqlx::query_as::<_, ReportTemplate>(
            "SELECT * FROM report_template WHERE organization_id = $1 AND template_type = $2 AND is_default = true AND is_deleted = false LIMIT 1"
        )
        .bind(organization_id)
        .bind(template_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound("Default template not found".to_string()))?;

        Ok(template)
    }
}

// ============================================================================
// Generated Report Repository
// ============================================================================

#[derive(Clone)]
pub struct GeneratedReportRepository {
    pool: PgPool,
}

impl GeneratedReportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: GenerateReportInput, created_by: Uuid) -> Result<GeneratedReport> {
        // Generate report number
        let report_number: (String,) = sqlx::query_as("SELECT generate_report_number()")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let report_data: serde_json::Value = serde_json::from_str(&input.report_data)
            .map_err(|e| Error::InvalidInput(format!("Invalid report_data JSON: {}", e)))?;

        let report_date = if let Some(date_str) = input.report_date {
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| Error::InvalidInput(format!("Invalid date format: {}", e)))?
        } else {
            Local::now().date_naive()
        };

        // Generate access code if requested
        let access_code = if input.generate_access_code.unwrap_or(false) {
            Some(Self::generate_access_code())
        } else {
            None
        };

        let report = sqlx::query_as::<_, GeneratedReport>(
            r#"
            INSERT INTO generated_report (
                organization_id, template_id, report_number, report_title, report_type,
                patient_id, order_id, result_id, batch_id, report_data, report_format,
                report_date, requires_signature, access_code, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.template_id)
        .bind(report_number.0)
        .bind(input.report_title)
        .bind(input.report_type)
        .bind(input.patient_id)
        .bind(input.order_id)
        .bind(input.result_id)
        .bind(input.batch_id)
        .bind(report_data)
        .bind(input.report_format.unwrap_or(ReportFormat::Pdf))
        .bind(report_date)
        .bind(input.requires_signature.unwrap_or(false))
        .bind(access_code)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(report)
    }

    pub async fn get_by_id(&self, report_id: Uuid) -> Result<GeneratedReport> {
        let report = sqlx::query_as::<_, GeneratedReport>(
            "SELECT * FROM generated_report WHERE id = $1 AND is_deleted = false"
        )
        .bind(report_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Report with ID {} not found", report_id)))?;

        Ok(report)
    }

    pub async fn get_by_report_number(&self, report_number: &str) -> Result<GeneratedReport> {
        let report = sqlx::query_as::<_, GeneratedReport>(
            "SELECT * FROM generated_report WHERE report_number = $1 AND is_deleted = false"
        )
        .bind(report_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Report '{}' not found", report_number)))?;

        Ok(report)
    }

    pub async fn list(
        &self,
        filter: Option<GeneratedReportFilter>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<Vec<GeneratedReport>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM generated_report WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(org_id) = f.organization_id {
                query.push_str(&format!(" AND organization_id = '{}'", org_id));
            }
            if let Some(patient_id) = f.patient_id {
                query.push_str(&format!(" AND patient_id = '{}'", patient_id));
            }
            if let Some(order_id) = f.order_id {
                query.push_str(&format!(" AND order_id = '{}'", order_id));
            }
            if let Some(report_type) = f.report_type {
                let type_str = match report_type {
                    ReportTemplateType::PatientReport => "PATIENT_REPORT",
                    ReportTemplateType::BatchReport => "BATCH_REPORT",
                    ReportTemplateType::QcReport => "QC_REPORT",
                    ReportTemplateType::SummaryReport => "SUMMARY_REPORT",
                    ReportTemplateType::CustomReport => "CUSTOM_REPORT",
                };
                query.push_str(&format!(" AND report_type = '{}'", type_str));
            }
            if let Some(status) = f.report_status {
                let status_str = match status {
                    ReportStatus::Pending => "PENDING",
                    ReportStatus::Generating => "GENERATING",
                    ReportStatus::Generated => "GENERATED",
                    ReportStatus::Failed => "FAILED",
                    ReportStatus::Delivered => "DELIVERED",
                    ReportStatus::Archived => "ARCHIVED",
                };
                query.push_str(&format!(" AND report_status = '{}'", status_str));
            }
            if let Some(from_date) = f.from_date {
                query.push_str(&format!(" AND report_date >= '{}'", from_date));
            }
            if let Some(to_date) = f.to_date {
                query.push_str(&format!(" AND report_date <= '{}'", to_date));
            }
        }

        query.push_str(&format!(" ORDER BY report_date DESC, created_at DESC LIMIT {} OFFSET {}", page_size, offset));

        let reports = sqlx::query_as::<_, GeneratedReport>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(reports)
    }

    pub async fn update_status(
        &self,
        report_id: Uuid,
        status: ReportStatus,
        error_message: Option<String>,
    ) -> Result<GeneratedReport> {
        let report = sqlx::query_as::<_, GeneratedReport>(
            r#"
            UPDATE generated_report
            SET report_status = $2,
                error_message = $3,
                generated_at = CASE WHEN $2 = 'GENERATED' THEN CURRENT_TIMESTAMP ELSE generated_at END
            WHERE id = $1 AND is_deleted = false
            RETURNING *
            "#
        )
        .bind(report_id)
        .bind(status)
        .bind(error_message)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(report)
    }

    pub async fn update_file_info(
        &self,
        report_id: Uuid,
        file_path: String,
        file_size_bytes: i64,
        file_hash: String,
    ) -> Result<GeneratedReport> {
        let report = sqlx::query_as::<_, GeneratedReport>(
            "UPDATE generated_report SET file_path = $2, file_size_bytes = $3, file_hash = $4 WHERE id = $1 AND is_deleted = false RETURNING *"
        )
        .bind(report_id)
        .bind(file_path)
        .bind(file_size_bytes)
        .bind(file_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(report)
    }

    pub async fn mark_as_signed(
        &self,
        report_id: Uuid,
        signed_by: Uuid,
    ) -> Result<GeneratedReport> {
        let report = sqlx::query_as::<_, GeneratedReport>(
            "UPDATE generated_report SET is_signed = true, signed_at = CURRENT_TIMESTAMP, signed_by = $2 WHERE id = $1 AND is_deleted = false RETURNING *"
        )
        .bind(report_id)
        .bind(signed_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(report)
    }

    fn generate_access_code() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789"; // Removed ambiguous chars
        let mut rng = rand::thread_rng();
        (0..8)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

// ============================================================================
// Digital Signature Repository
// ============================================================================

#[derive(Clone)]
pub struct DigitalSignatureRepository {
    pool: PgPool,
}

impl DigitalSignatureRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: SignReportInput, signature_hash: String) -> Result<DigitalSignature> {
        let signature = sqlx::query_as::<_, DigitalSignature>(
            r#"
            INSERT INTO digital_signature (
                report_id, organization_id, signatory_id, signatory_name, signatory_role,
                signatory_designation, signatory_qualification, signature_type,
                signature_image_path, signature_location, signature_ip_address,
                signature_hash
            )
            SELECT $1, organization_id, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            FROM generated_report WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.report_id)
        .bind(input.signatory_id)
        .bind(input.signatory_name)
        .bind(input.signatory_role)
        .bind(input.signatory_designation)
        .bind(input.signatory_qualification)
        .bind(input.signature_type)
        .bind(input.signature_image_path)
        .bind(input.signature_location)
        .bind(input.signature_ip_address)
        .bind(signature_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(signature)
    }

    pub async fn get_by_report_id(&self, report_id: Uuid) -> Result<Vec<DigitalSignature>> {
        let signatures = sqlx::query_as::<_, DigitalSignature>(
            "SELECT * FROM digital_signature WHERE report_id = $1 AND is_deleted = false ORDER BY signature_timestamp ASC"
        )
        .bind(report_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(signatures)
    }
}

// ============================================================================
// Report Delivery Repository
// ============================================================================

#[derive(Clone)]
pub struct ReportDeliveryRepository {
    pool: PgPool,
}

impl ReportDeliveryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: DeliverReportInput, created_by: Uuid) -> Result<ReportDelivery> {
        let scheduled_at = if let Some(schedule_str) = input.scheduled_at {
            Some(chrono::NaiveDateTime::parse_from_str(&schedule_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|e| Error::InvalidInput(format!("Invalid scheduled_at format: {}", e)))?)
        } else {
            None
        };

        let delivery = sqlx::query_as::<_, ReportDelivery>(
            r#"
            INSERT INTO report_delivery (
                report_id, organization_id, delivery_channel, recipient_name,
                recipient_contact, subject, message, scheduled_at, created_by
            )
            SELECT $1, organization_id, $2, $3, $4, $5, $6, $7, $8
            FROM generated_report WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.report_id)
        .bind(input.delivery_channel)
        .bind(input.recipient_name)
        .bind(input.recipient_contact)
        .bind(input.subject)
        .bind(input.message)
        .bind(scheduled_at)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(delivery)
    }

    pub async fn get_by_id(&self, delivery_id: Uuid) -> Result<ReportDelivery> {
        let delivery = sqlx::query_as::<_, ReportDelivery>(
            "SELECT * FROM report_delivery WHERE id = $1 AND is_deleted = false"
        )
        .bind(delivery_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Delivery with ID {} not found", delivery_id)))?;

        Ok(delivery)
    }

    pub async fn list(
        &self,
        filter: Option<ReportDeliveryFilter>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<Vec<ReportDelivery>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM report_delivery WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(report_id) = f.report_id {
                query.push_str(&format!(" AND report_id = '{}'", report_id));
            }
            if let Some(channel) = f.delivery_channel {
                let channel_str = match channel {
                    DeliveryChannel::Email => "EMAIL",
                    DeliveryChannel::Whatsapp => "WHATSAPP",
                    DeliveryChannel::Sms => "SMS",
                    DeliveryChannel::Download => "DOWNLOAD",
                    DeliveryChannel::Print => "PRINT",
                };
                query.push_str(&format!(" AND delivery_channel = '{}'", channel_str));
            }
            if let Some(status) = f.delivery_status {
                let status_str = match status {
                    DeliveryStatus::Pending => "PENDING",
                    DeliveryStatus::Sent => "SENT",
                    DeliveryStatus::Delivered => "DELIVERED",
                    DeliveryStatus::Failed => "FAILED",
                    DeliveryStatus::Bounced => "BOUNCED",
                };
                query.push_str(&format!(" AND delivery_status = '{}'", status_str));
            }
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", page_size, offset));

        let deliveries = sqlx::query_as::<_, ReportDelivery>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(deliveries)
    }

    pub async fn update_status(
        &self,
        delivery_id: Uuid,
        status: DeliveryStatus,
        provider_message_id: Option<String>,
        error_message: Option<String>,
    ) -> Result<ReportDelivery> {
        let delivery = sqlx::query_as::<_, ReportDelivery>(
            r#"
            UPDATE report_delivery
            SET delivery_status = $2,
                provider_message_id = $3,
                error_message = $4,
                sent_at = CASE WHEN $2 = 'SENT' OR $2 = 'DELIVERED' THEN CURRENT_TIMESTAMP ELSE sent_at END,
                delivered_at = CASE WHEN $2 = 'DELIVERED' THEN CURRENT_TIMESTAMP ELSE delivered_at END,
                retry_count = CASE WHEN $2 = 'FAILED' THEN retry_count + 1 ELSE retry_count END
            WHERE id = $1 AND is_deleted = false
            RETURNING *
            "#
        )
        .bind(delivery_id)
        .bind(status)
        .bind(provider_message_id)
        .bind(error_message)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(delivery)
    }
}

// ============================================================================
// Report Access Log Repository
// ============================================================================

#[derive(Clone)]
pub struct ReportAccessLogRepository {
    pool: PgPool,
}

impl ReportAccessLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn log_access(&self, input: LogReportAccessInput) -> Result<ReportAccessLog> {
        let log = sqlx::query_as::<_, ReportAccessLog>(
            r#"
            INSERT INTO report_access_log (
                report_id, accessed_by, access_code_used, ip_address,
                user_agent, access_method, session_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(input.report_id)
        .bind(input.accessed_by)
        .bind(input.access_code_used)
        .bind(input.ip_address)
        .bind(input.user_agent)
        .bind(input.access_method)
        .bind(input.session_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(log)
    }

    pub async fn get_report_access_logs(&self, report_id: Uuid) -> Result<Vec<ReportAccessLog>> {
        let logs = sqlx::query_as::<_, ReportAccessLog>(
            "SELECT * FROM report_access_log WHERE report_id = $1 ORDER BY access_timestamp DESC"
        )
        .bind(report_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(logs)
    }
}
