use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// AUDIT LOG
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
#[graphql(name = "AuditLog")]
pub struct AuditLog {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
    pub reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateAuditLogInput {
    pub entity_type: String,
    pub entity_id: String,
    pub action: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
    pub reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct AuditLogFilter {
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// ============================================================================
// DOCUMENT CONTROL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "document_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentType {
    Sop,
    Policy,
    Procedure,
    Form,
    Manual,
    WorkInstruction,
    Specification,
    Certificate,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "document_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentStatus {
    Draft,
    UnderReview,
    Approved,
    Published,
    Obsolete,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
#[graphql(name = "DocumentControl")]
pub struct DocumentControl {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub document_number: String,
    pub document_type: DocumentType,
    pub title: String,
    pub description: Option<String>,
    pub version: String,
    pub revision_number: i32,
    pub document_status: DocumentStatus,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub file_mime_type: Option<String>,
    pub content_hash: Option<String>,
    pub author_id: Uuid,
    pub reviewer_id: Option<Uuid>,
    pub approver_id: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub effective_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub next_review_date: Option<NaiveDate>,
    pub department: Option<String>,
    pub applicable_to: Vec<String>,
    pub keywords: Vec<String>,
    pub related_documents: Vec<Uuid>,
    pub supersedes_document_id: Option<Uuid>,
    pub view_count: i32,
    pub last_viewed_at: Option<DateTime<Utc>>,
    pub acknowledgement_required: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateDocumentInput {
    pub document_number: String,
    pub document_type: DocumentType,
    pub title: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub department: Option<String>,
    pub applicable_to: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub effective_date: Option<String>,
    pub next_review_date: Option<String>,
    pub acknowledgement_required: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateDocumentInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub department: Option<String>,
    pub applicable_to: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub next_review_date: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct ApproveDocumentInput {
    pub reviewer_id: Option<String>,
    pub approver_id: String,
    pub effective_date: Option<String>,
}

// ============================================================================
// DOCUMENT ACKNOWLEDGEMENT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct DocumentAcknowledgement {
    pub id: Uuid,
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub acknowledged_at: DateTime<Utc>,
    pub version_acknowledged: String,
    pub ip_address: Option<String>,
}

// ============================================================================
// CAPA (Corrective and Preventive Actions)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "capa_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CAPAType {
    Corrective,
    Preventive,
    Improvement,
}

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "capa_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CAPAStatus {
    Open,
    Investigation,
    ActionPlan,
    Implementation,
    Verification,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "capa_priority", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CAPAPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
#[graphql(name = "CAPA")]
pub struct CAPA {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub capa_number: String,
    pub capa_type: CAPAType,
    pub priority: CAPAPriority,
    pub capa_status: CAPAStatus,
    pub title: String,
    pub description: String,
    pub source: Option<String>,
    pub source_reference: Option<String>,
    pub date_identified: NaiveDate,
    pub root_cause_analysis: Option<String>,
    pub root_cause_identified_date: Option<NaiveDate>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub action_plan: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub target_completion_date: Option<NaiveDate>,
    pub actual_completion_date: Option<NaiveDate>,
    pub verification_method: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verification_date: Option<NaiveDate>,
    pub verification_result: Option<String>,
    pub effectiveness_check: bool,
    pub effectiveness_check_date: Option<NaiveDate>,
    pub closed_by: Option<Uuid>,
    pub closed_at: Option<DateTime<Utc>>,
    pub closure_remarks: Option<String>,
    pub related_documents: Vec<Uuid>,
    pub attachments: serde_json::Value,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateCAPAInput {
    pub capa_type: CAPAType,
    pub priority: CAPAPriority,
    pub title: String,
    pub description: String,
    pub source: Option<String>,
    pub source_reference: Option<String>,
    pub date_identified: String,
    pub assigned_to: Option<String>,
    pub target_completion_date: Option<String>,
}

impl CreateCAPAInput {
    pub fn validate(&self) -> common::error::Result<()> {
        if self.title.trim().is_empty() {
            return Err(common::error::Error::Validation("Title is required".to_string()));
        }
        if self.description.trim().is_empty() {
            return Err(common::error::Error::Validation("Description is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateCAPAInput {
    pub priority: Option<CAPAPriority>,
    pub capa_status: Option<CAPAStatus>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub root_cause_analysis: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub action_plan: Option<String>,
    pub assigned_to: Option<String>,
    pub target_completion_date: Option<String>,
    pub actual_completion_date: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CloseCAPAInput {
    pub verification_method: String,
    pub verification_result: String,
    pub effectiveness_check: bool,
    pub closure_remarks: String,
}

// ============================================================================
// TRAINING RECORDS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "training_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrainingType {
    Orientation,
    Technical,
    Safety,
    Quality,
    Compliance,
    Software,
    Equipment,
    Refresher,
}

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "training_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrainingStatus {
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct TrainingRecord {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub training_type: TrainingType,
    pub training_status: TrainingStatus,
    pub training_title: String,
    pub training_description: Option<String>,
    pub trainer_name: Option<String>,
    pub trainer_id: Option<Uuid>,
    pub training_method: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub training_start_date: Option<NaiveDate>,
    pub training_end_date: Option<NaiveDate>,
    pub duration_hours: Option<rust_decimal::Decimal>,
    pub assessment_required: bool,
    pub assessment_score: Option<rust_decimal::Decimal>,
    pub passing_score: rust_decimal::Decimal,
    pub assessment_date: Option<NaiveDate>,
    pub assessment_result: Option<String>,
    pub certificate_issued: bool,
    pub certificate_number: Option<String>,
    pub certificate_issued_date: Option<NaiveDate>,
    pub certificate_expiry_date: Option<NaiveDate>,
    pub competency_achieved: bool,
    pub competency_assessor_id: Option<Uuid>,
    pub competency_assessment_date: Option<NaiveDate>,
    pub competency_remarks: Option<String>,
    pub training_materials: serde_json::Value,
    pub attendance_record: Option<serde_json::Value>,
    pub certificates: serde_json::Value,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateTrainingRecordInput {
    pub user_id: String,
    pub training_type: TrainingType,
    pub training_title: String,
    pub training_description: Option<String>,
    pub trainer_name: Option<String>,
    pub training_method: Option<String>,
    pub scheduled_date: Option<String>,
    pub duration_hours: Option<f64>,
    pub assessment_required: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateTrainingRecordInput {
    pub training_status: Option<TrainingStatus>,
    pub training_start_date: Option<String>,
    pub training_end_date: Option<String>,
    pub assessment_score: Option<f64>,
    pub assessment_result: Option<String>,
    pub competency_achieved: Option<bool>,
}

// ============================================================================
// QUALITY INDICATORS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "indicator_category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndicatorCategory {
    PreAnalytical,
    Analytical,
    PostAnalytical,
    CustomerService,
    Safety,
    TurnaroundTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QualityIndicator {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub indicator_name: String,
    pub indicator_category: IndicatorCategory,
    pub description: Option<String>,
    pub measurement_unit: Option<String>,
    pub target_value: Option<rust_decimal::Decimal>,
    pub threshold_warning: Option<rust_decimal::Decimal>,
    pub threshold_critical: Option<rust_decimal::Decimal>,
    pub calculation_method: Option<String>,
    pub measurement_frequency: Option<String>,
    pub responsible_person_id: Option<Uuid>,
    pub is_active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "indicator_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndicatorStatus {
    OnTarget,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QualityIndicatorValue {
    pub id: Uuid,
    pub indicator_id: Uuid,
    pub organization_id: Uuid,
    pub measurement_date: NaiveDate,
    pub measured_value: rust_decimal::Decimal,
    pub indicator_status: IndicatorStatus,
    pub analysis_notes: Option<String>,
    pub action_taken: Option<String>,
    pub measured_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, InputObject)]
pub struct RecordQualityIndicatorInput {
    pub indicator_id: String,
    pub measurement_date: String,
    pub measured_value: f64,
    pub analysis_notes: Option<String>,
    pub action_taken: Option<String>,
}

// ============================================================================
// COMPLIANCE CHECKLIST
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "checklist_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChecklistType {
    InternalAudit,
    ExternalAudit,
    NablAssessment,
    DailyCheck,
    MonthlyCheck,
    EquipmentCheck,
    ProcessCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ComplianceChecklist {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub checklist_name: String,
    pub checklist_type: ChecklistType,
    pub description: Option<String>,
    pub items: serde_json::Value,
    pub frequency: Option<String>,
    pub is_active: bool,
    pub responsible_role: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "assessment_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssessmentStatus {
    InProgress,
    Completed,
    Reviewed,
    Approved,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ComplianceAssessment {
    pub id: Uuid,
    pub checklist_id: Uuid,
    pub organization_id: Uuid,
    pub assessment_date: NaiveDate,
    pub assessment_status: AssessmentStatus,
    pub total_items: i32,
    pub items_passed: i32,
    pub items_failed: i32,
    pub items_na: i32,
    pub compliance_score: Option<rust_decimal::Decimal>,
    pub assessment_results: serde_json::Value,
    pub findings: Option<String>,
    pub recommendations: Option<String>,
    pub assessor_id: Uuid,
    pub reviewed_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub capa_required: bool,
    pub capa_ids: Vec<Uuid>,
    pub attachments: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateComplianceAssessmentInput {
    pub checklist_id: String,
    pub assessment_date: String,
    pub assessment_results: serde_json::Value,
    pub findings: Option<String>,
    pub recommendations: Option<String>,
}

// ============================================================================
// COMPLIANCE DASHBOARD
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ComplianceDashboard {
    pub organization_id: Uuid,
    pub open_capas: i64,
    pub overdue_capas: i64,
    pub pending_document_reviews: i64,
    pub expired_trainings: i64,
    pub quality_indicators_critical: i64,
    pub recent_assessments: Vec<ComplianceAssessment>,
    pub upcoming_audits: Vec<ComplianceChecklist>,
}
