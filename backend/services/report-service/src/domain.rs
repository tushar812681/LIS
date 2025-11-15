use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "report_template_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportTemplateType {
    PatientReport,
    BatchReport,
    QcReport,
    SummaryReport,
    CustomReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "report_format", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportFormat {
    Pdf,
    Html,
    Csv,
    Excel,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "report_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportStatus {
    Pending,
    Generating,
    Generated,
    Failed,
    Delivered,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "delivery_channel", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryChannel {
    Email,
    Whatsapp,
    Sms,
    Download,
    Print,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "delivery_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Bounced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "signature_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SignatureStatus {
    Pending,
    Signed,
    Rejected,
    Expired,
}

// ============================================================================
// Report Template Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ReportTemplate {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Template identification
    pub template_name: String,
    pub template_code: String,
    pub template_type: ReportTemplateType,
    pub description: Option<String>,

    // Template configuration
    #[sqlx(json)]
    pub template_content: serde_json::Value,
    #[sqlx(json)]
    pub header_content: Option<serde_json::Value>,
    #[sqlx(json)]
    pub footer_content: Option<serde_json::Value>,
    #[sqlx(json)]
    pub styles: Option<serde_json::Value>,

    // Page settings
    pub page_size: Option<String>,
    pub page_orientation: Option<String>,
    pub margin_top: Option<i32>,
    pub margin_bottom: Option<i32>,
    pub margin_left: Option<i32>,
    pub margin_right: Option<i32>,

    // Report fields configuration
    #[sqlx(json)]
    pub fields_config: Option<serde_json::Value>,
    #[sqlx(json)]
    pub sections_config: Option<serde_json::Value>,

    // Branding
    pub show_logo: Option<bool>,
    pub show_watermark: Option<bool>,
    pub watermark_text: Option<String>,

    // Digital signature settings
    pub requires_signature: Option<bool>,
    #[sqlx(json)]
    pub signature_fields: Option<serde_json::Value>,

    // Output settings
    pub default_format: Option<ReportFormat>,
    pub enable_auto_delivery: Option<bool>,
    #[sqlx(json)]
    pub auto_delivery_channels: Option<serde_json::Value>,

    // Status
    pub is_active: Option<bool>,
    pub is_default: Option<bool>,
    pub version: Option<i32>,

    // Audit fields
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

// ============================================================================
// Generated Report Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct GeneratedReport {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub template_id: Option<Uuid>,

    // Report identification
    pub report_number: String,
    pub report_title: String,
    pub report_type: ReportTemplateType,

    // Associated entities
    pub patient_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub result_id: Option<Uuid>,
    pub batch_id: Option<Uuid>,

    // Report content
    #[sqlx(json)]
    pub report_data: serde_json::Value,
    pub generated_content: Option<String>,
    pub report_format: Option<ReportFormat>,

    // File storage
    pub file_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub file_hash: Option<String>,
    pub storage_location: Option<String>,

    // Report metadata
    pub report_date: NaiveDate,
    pub generated_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,

    // Status
    pub report_status: ReportStatus,
    pub error_message: Option<String>,

    // Access control
    pub is_confidential: Option<bool>,
    pub access_code: Option<String>,
    pub download_count: Option<i32>,
    pub last_downloaded_at: Option<NaiveDateTime>,

    // Digital signatures
    pub requires_signature: Option<bool>,
    pub is_signed: Option<bool>,
    pub signed_at: Option<NaiveDateTime>,
    pub signed_by: Option<Uuid>,

    // Audit fields
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

impl GeneratedReport {
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            expires_at < chrono::Local::now().naive_local()
        } else {
            false
        }
    }

    pub fn is_downloadable(&self) -> bool {
        self.report_status == ReportStatus::Generated && !self.is_expired()
    }
}

// ============================================================================
// Digital Signature Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct DigitalSignature {
    pub id: Uuid,
    pub report_id: Uuid,
    pub organization_id: Uuid,

    // Signatory information
    pub signatory_id: Uuid,
    pub signatory_name: String,
    pub signatory_role: Option<String>,
    pub signatory_designation: Option<String>,
    pub signatory_qualification: Option<String>,

    // Signature details
    pub signature_type: String,
    pub signature_image_path: Option<String>,
    pub digital_certificate: Option<String>,

    // Signature metadata
    pub signature_timestamp: NaiveDateTime,
    pub signature_location: Option<String>,
    pub signature_ip_address: Option<String>,
    #[sqlx(json)]
    pub signature_device_info: Option<serde_json::Value>,

    // Verification
    pub signature_hash: String,
    pub verification_code: Option<String>,
    pub signature_status: Option<SignatureStatus>,

    // Comments
    pub signature_comments: Option<String>,

    // Audit fields
    pub created_at: NaiveDateTime,
    pub is_deleted: Option<bool>,
}

// ============================================================================
// Report Delivery Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ReportDelivery {
    pub id: Uuid,
    pub report_id: Uuid,
    pub organization_id: Uuid,

    // Delivery details
    pub delivery_channel: DeliveryChannel,
    pub recipient_name: String,
    pub recipient_contact: String,

    // Delivery configuration
    pub subject: Option<String>,
    pub message: Option<String>,
    pub attachment_url: Option<String>,

    // Status tracking
    pub delivery_status: Option<DeliveryStatus>,
    pub scheduled_at: Option<NaiveDateTime>,
    pub sent_at: Option<NaiveDateTime>,
    pub delivered_at: Option<NaiveDateTime>,

    // Provider details
    pub provider_name: Option<String>,
    pub provider_message_id: Option<String>,
    #[sqlx(json)]
    pub provider_response: Option<serde_json::Value>,

    // Error handling
    pub error_message: Option<String>,
    pub retry_count: Option<i32>,
    pub max_retries: Option<i32>,

    // Audit fields
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

impl ReportDelivery {
    pub fn can_retry(&self) -> bool {
        let retry_count = self.retry_count.unwrap_or(0);
        let max_retries = self.max_retries.unwrap_or(3);
        retry_count < max_retries
    }
}

// ============================================================================
// Report Access Log Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ReportAccessLog {
    pub id: Uuid,
    pub report_id: Uuid,

    // Access details
    pub accessed_by: Option<Uuid>,
    pub access_code_used: Option<String>,
    pub access_timestamp: NaiveDateTime,

    // Request details
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub access_method: Option<String>,

    // Session info
    pub session_id: Option<String>,
    pub duration_seconds: Option<i32>,
}

// ============================================================================
// Input Types
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct CreateReportTemplateInput {
    pub organization_id: Uuid,
    pub template_name: String,
    pub template_code: String,
    pub template_type: ReportTemplateType,
    pub description: Option<String>,
    pub template_content: String, // JSON string
    pub header_content: Option<String>,
    pub footer_content: Option<String>,
    pub styles: Option<String>,
    pub page_size: Option<String>,
    pub page_orientation: Option<String>,
    pub fields_config: Option<String>,
    pub requires_signature: Option<bool>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct GenerateReportInput {
    pub organization_id: Uuid,
    pub template_id: Option<Uuid>,
    pub report_title: String,
    pub report_type: ReportTemplateType,
    pub patient_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub result_id: Option<Uuid>,
    pub batch_id: Option<Uuid>,
    pub report_data: String, // JSON string
    pub report_format: Option<ReportFormat>,
    pub report_date: Option<String>,
    pub requires_signature: Option<bool>,
    pub generate_access_code: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct SignReportInput {
    pub report_id: Uuid,
    pub signatory_id: Uuid,
    pub signatory_name: String,
    pub signatory_role: Option<String>,
    pub signatory_designation: Option<String>,
    pub signatory_qualification: Option<String>,
    pub signature_type: String,
    pub signature_image_path: Option<String>,
    pub signature_location: Option<String>,
    pub signature_ip_address: Option<String>,
    pub signature_comments: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct DeliverReportInput {
    pub report_id: Uuid,
    pub delivery_channel: DeliveryChannel,
    pub recipient_name: String,
    pub recipient_contact: String,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub scheduled_at: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct LogReportAccessInput {
    pub report_id: Uuid,
    pub accessed_by: Option<Uuid>,
    pub access_code_used: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub access_method: String,
    pub session_id: Option<String>,
}

// ============================================================================
// Filter Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct ReportTemplateFilter {
    pub organization_id: Option<Uuid>,
    pub template_type: Option<ReportTemplateType>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct GeneratedReportFilter {
    pub organization_id: Option<Uuid>,
    pub patient_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub report_type: Option<ReportTemplateType>,
    pub report_status: Option<ReportStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct ReportDeliveryFilter {
    pub report_id: Option<Uuid>,
    pub delivery_channel: Option<DeliveryChannel>,
    pub delivery_status: Option<DeliveryStatus>,
}
