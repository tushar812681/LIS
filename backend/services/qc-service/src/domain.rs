use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_graphql::{Enum, InputObject, SimpleObject};
use rust_decimal::Decimal;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "qc_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QcType {
    Iqc,  // Internal Quality Control
    Eqc,  // External Quality Control
    Pt,   // Proficiency Testing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "qc_material_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QcMaterialStatus {
    Active,
    Expired,
    LowStock,
    OutOfStock,
    Discontinued,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "qc_result_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QcResultStatus {
    InControl,
    OutOfControl,
    Warning,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "qc_rule_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QcRuleType {
    Westgard12s,   // 1 control exceeds 2SD
    Westgard13s,   // 1 control exceeds 3SD
    Westgard22s,   // 2 consecutive controls exceed 2SD (same side)
    WestgardR4s,   // Range between 2 controls exceeds 4SD
    Westgard41s,   // 4 consecutive controls exceed 1SD (same side)
    Westgard10x,   // 10 consecutive controls on same side of mean
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "violation_severity", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "corrective_action_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CorrectiveActionStatus {
    Pending,
    InProgress,
    Completed,
    Verified,
}

// ============================================================================
// QC Material Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcMaterial {
    // Identity
    pub id: Uuid,
    pub material_code: String,

    // Basic Information
    pub material_name: String,
    pub manufacturer: Option<String>,
    pub lot_number: String,
    pub catalog_number: Option<String>,

    // QC Type
    pub qc_type: QcType,

    // Organization
    pub organization_id: Uuid,

    // Test Information
    pub test_id: Uuid,
    pub test_name: Option<String>,

    // Levels
    pub level_number: Option<i32>,
    pub level_name: Option<String>,

    // Target Values
    pub target_mean: Option<Decimal>,
    pub target_sd: Option<Decimal>,

    // Control Limits
    pub mean_value: Option<Decimal>,
    pub sd_value: Option<Decimal>,
    pub cv_value: Option<Decimal>,

    pub sd_1_low: Option<Decimal>,
    pub sd_1_high: Option<Decimal>,
    pub sd_2_low: Option<Decimal>,
    pub sd_2_high: Option<Decimal>,
    pub sd_3_low: Option<Decimal>,
    pub sd_3_high: Option<Decimal>,

    // Stock Management
    pub quantity_in_stock: Option<i32>,
    pub minimum_stock_level: Option<i32>,
    pub unit_of_measure: Option<String>,

    // Validity
    pub manufacture_date: Option<NaiveDate>,
    pub expiry_date: NaiveDate,
    pub opened_date: Option<NaiveDate>,
    pub days_stable_after_opening: Option<i32>,

    // Storage
    pub storage_location: Option<String>,
    pub storage_temperature: Option<String>,

    // Status
    pub material_status: QcMaterialStatus,

    // Equipment Assignment
    pub equipment_id: Option<Uuid>,

    // Documentation
    pub insert_url: Option<String>,
    pub msds_url: Option<String>,

    // Notes
    pub notes: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub is_deleted: Option<bool>,
}

impl QcMaterial {
    pub fn is_active(&self) -> bool {
        self.material_status == QcMaterialStatus::Active
    }

    pub fn is_expired(&self) -> bool {
        self.expiry_date < chrono::Local::now().date_naive()
    }

    pub fn days_until_expiry(&self) -> i64 {
        let today = chrono::Local::now().date_naive();
        (self.expiry_date - today).num_days()
    }

    pub fn needs_restocking(&self) -> bool {
        if let (Some(qty), Some(min)) = (self.quantity_in_stock, self.minimum_stock_level) {
            qty <= min
        } else {
            false
        }
    }

    pub fn is_opened_and_expired(&self) -> bool {
        if let (Some(opened), Some(days_stable)) = (self.opened_date, self.days_stable_after_opening) {
            let today = chrono::Local::now().date_naive();
            let expiry_after_opening = opened + chrono::Duration::days(days_stable as i64);
            today > expiry_after_opening
        } else {
            false
        }
    }

    pub fn calculate_control_limits(&mut self) {
        if let (Some(mean), Some(sd)) = (self.mean_value, self.sd_value) {
            self.sd_1_low = Some(mean - sd);
            self.sd_1_high = Some(mean + sd);
            self.sd_2_low = Some(mean - (sd * Decimal::from(2)));
            self.sd_2_high = Some(mean + (sd * Decimal::from(2)));
            self.sd_3_low = Some(mean - (sd * Decimal::from(3)));
            self.sd_3_high = Some(mean + (sd * Decimal::from(3)));
        }
    }
}

// ============================================================================
// QC Rule Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcRule {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Rule Details
    pub rule_name: String,
    pub rule_type: QcRuleType,
    pub rule_description: Option<String>,

    // Configuration
    #[graphql(skip)]
    pub is_active: Option<bool>,
    #[graphql(skip)]
    pub is_blocking: Option<bool>,

    // Severity
    pub violation_severity: ViolationSeverity,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

impl QcRule {
    pub fn is_active(&self) -> bool {
        self.is_active.unwrap_or(false)
    }

    pub fn is_blocking(&self) -> bool {
        self.is_blocking.unwrap_or(false)
    }
}

// ============================================================================
// QC Material Rule Assignment Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcMaterialRule {
    pub id: Uuid,
    pub qc_material_id: Uuid,
    pub qc_rule_id: Uuid,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

// ============================================================================
// QC Result Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcResult {
    pub id: Uuid,
    pub result_number: String,

    // QC Material
    pub qc_material_id: Uuid,

    // Organization
    pub organization_id: Uuid,

    // Test Information
    pub test_id: Uuid,
    pub test_name: Option<String>,

    // Equipment
    pub equipment_id: Option<Uuid>,

    // Result Data
    pub result_date: NaiveDate,
    pub result_time: NaiveTime,
    pub result_value: Decimal,

    // Statistical Analysis
    pub mean_value: Option<Decimal>,
    pub sd_value: Option<Decimal>,
    pub cv_value: Option<Decimal>,
    pub z_score: Option<Decimal>,

    // Status
    pub result_status: QcResultStatus,

    // Rules Violated
    pub rules_violated: Option<serde_json::Value>,

    // Performer
    pub performed_by: Option<Uuid>,
    pub performed_by_name: Option<String>,

    // Review
    pub reviewed: Option<bool>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<NaiveDateTime>,

    // Comments
    pub comments: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
}

impl QcResult {
    pub fn is_in_control(&self) -> bool {
        self.result_status == QcResultStatus::InControl
    }

    pub fn is_out_of_control(&self) -> bool {
        self.result_status == QcResultStatus::OutOfControl
    }

    pub fn is_within_2sd(&self) -> bool {
        if let Some(z) = self.z_score {
            z.abs() <= Decimal::from(2)
        } else {
            false
        }
    }

    pub fn is_within_3sd(&self) -> bool {
        if let Some(z) = self.z_score {
            z.abs() <= Decimal::from(3)
        } else {
            false
        }
    }
}

// ============================================================================
// QC Violation Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcViolation {
    pub id: Uuid,

    // QC Result
    pub qc_result_id: Uuid,
    pub qc_material_id: Uuid,

    // Organization
    pub organization_id: Uuid,

    // Rule Violated
    pub qc_rule_id: Uuid,
    pub rule_type: QcRuleType,
    pub rule_description: Option<String>,

    // Violation Details
    pub violation_date: NaiveDate,
    pub violation_time: NaiveTime,
    pub severity: ViolationSeverity,

    // Impact
    pub patient_results_affected: Option<i32>,
    pub patient_results_held: Option<i32>,

    // Status
    #[graphql(skip)]
    pub is_acknowledged: Option<bool>,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<NaiveDateTime>,

    #[graphql(skip)]
    pub is_resolved: Option<bool>,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<NaiveDateTime>,

    // Root Cause
    pub root_cause: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
}

impl QcViolation {
    pub fn is_acknowledged(&self) -> bool {
        self.is_acknowledged.unwrap_or(false)
    }

    pub fn is_resolved(&self) -> bool {
        self.is_resolved.unwrap_or(false)
    }

    pub fn is_critical(&self) -> bool {
        self.severity == ViolationSeverity::Critical
    }
}

// ============================================================================
// QC Corrective Action Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcCorrectiveAction {
    pub id: Uuid,

    // Violation
    pub qc_violation_id: Uuid,

    // Action Details
    pub action_description: String,
    pub action_status: CorrectiveActionStatus,

    // Assignment
    pub assigned_to: Option<Uuid>,
    pub assigned_to_name: Option<String>,

    // Timeline
    pub due_date: Option<NaiveDate>,
    pub completed_date: Option<NaiveDate>,

    // Effectiveness
    pub effectiveness_verified: Option<bool>,
    pub verification_notes: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl QcCorrectiveAction {
    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            if self.action_status != CorrectiveActionStatus::Completed {
                return due < chrono::Local::now().date_naive();
            }
        }
        false
    }

    pub fn is_completed(&self) -> bool {
        self.action_status == CorrectiveActionStatus::Completed
    }
}

// ============================================================================
// QC Statistics Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcStatistics {
    pub id: Uuid,
    pub qc_material_id: Uuid,

    // Date
    pub statistics_date: NaiveDate,

    // Sample Size
    pub n_count: i32,

    // Statistics
    pub mean_value: Option<Decimal>,
    pub sd_value: Option<Decimal>,
    pub cv_value: Option<Decimal>,
    pub min_value: Option<Decimal>,
    pub max_value: Option<Decimal>,
    pub range_value: Option<Decimal>,

    // Control Status
    pub in_control_count: Option<i32>,
    pub out_of_control_count: Option<i32>,
    pub warning_count: Option<i32>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
}

// ============================================================================
// QC External Program Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcExternalProgram {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Program Details
    pub program_name: String,
    pub provider: Option<String>,
    pub program_code: Option<String>,

    // Type
    pub qc_type: QcType,

    // Enrollment
    pub enrollment_date: Option<NaiveDate>,
    pub next_shipment_date: Option<NaiveDate>,

    // Status
    pub is_active: Option<bool>,

    // Contact
    pub contact_person: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

// ============================================================================
// QC External Survey Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct QcExternalSurvey {
    pub id: Uuid,
    pub program_id: Uuid,

    // Survey Details
    pub survey_name: String,
    pub survey_number: Option<String>,
    pub survey_date: NaiveDate,

    // Shipment
    pub shipment_date: Option<NaiveDate>,
    pub sample_received_date: Option<NaiveDate>,

    // Deadline
    pub submission_deadline: Option<NaiveDate>,
    pub submitted_date: Option<NaiveDate>,

    // Status
    pub is_completed: Option<bool>,

    // Results Received
    pub results_received: Option<bool>,
    pub results_received_date: Option<NaiveDate>,

    // Performance
    pub acceptable_results: Option<i32>,
    pub unacceptable_results: Option<i32>,
    pub overall_grade: Option<String>,

    // Report
    pub report_url: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// ============================================================================
// Input DTOs
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct CreateQcMaterialInput {
    pub material_name: String,
    pub manufacturer: Option<String>,
    pub lot_number: String,
    pub catalog_number: Option<String>,

    pub qc_type: QcType,
    pub organization_id: Uuid,

    pub test_id: Uuid,
    pub test_name: Option<String>,

    pub level_number: Option<i32>,
    pub level_name: Option<String>,

    pub target_mean: Option<Decimal>,
    pub target_sd: Option<Decimal>,

    pub expiry_date: NaiveDate,
    pub manufacture_date: Option<NaiveDate>,

    pub quantity_in_stock: Option<i32>,
    pub minimum_stock_level: Option<i32>,

    pub storage_location: Option<String>,
    pub storage_temperature: Option<String>,

    pub equipment_id: Option<Uuid>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateQcMaterialInput {
    pub id: Uuid,

    pub quantity_in_stock: Option<i32>,
    pub minimum_stock_level: Option<i32>,

    pub opened_date: Option<NaiveDate>,
    pub days_stable_after_opening: Option<i32>,

    pub storage_location: Option<String>,

    pub mean_value: Option<Decimal>,
    pub sd_value: Option<Decimal>,

    pub notes: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct RecordQcResultInput {
    pub qc_material_id: Uuid,
    pub result_date: NaiveDate,
    pub result_time: NaiveTime,
    pub result_value: Decimal,

    pub equipment_id: Option<Uuid>,
    pub performed_by: Option<Uuid>,
    pub performed_by_name: Option<String>,

    pub comments: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct ReviewQcResultInput {
    pub id: Uuid,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateQcRuleInput {
    pub organization_id: Uuid,
    pub rule_name: String,
    pub rule_type: QcRuleType,
    pub rule_description: Option<String>,
    pub is_blocking: Option<bool>,
    pub violation_severity: ViolationSeverity,
}

#[derive(Debug, Clone, InputObject)]
pub struct AssignRuleToMaterialInput {
    pub qc_material_id: Uuid,
    pub qc_rule_id: Uuid,
}

#[derive(Debug, Clone, InputObject)]
pub struct AcknowledgeViolationInput {
    pub id: Uuid,
}

#[derive(Debug, Clone, InputObject)]
pub struct ResolveViolationInput {
    pub id: Uuid,
    pub root_cause: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateCorrectiveActionInput {
    pub qc_violation_id: Uuid,
    pub action_description: String,
    pub assigned_to: Option<Uuid>,
    pub assigned_to_name: Option<String>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateCorrectiveActionInput {
    pub id: Uuid,
    pub action_status: Option<CorrectiveActionStatus>,
    pub completed_date: Option<NaiveDate>,
    pub effectiveness_verified: Option<bool>,
    pub verification_notes: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateExternalProgramInput {
    pub organization_id: Uuid,
    pub program_name: String,
    pub provider: Option<String>,
    pub program_code: Option<String>,
    pub qc_type: QcType,
    pub enrollment_date: Option<NaiveDate>,
    pub contact_person: Option<String>,
    pub contact_email: Option<String>,
}

// ============================================================================
// Query Filters
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct QcMaterialFilter {
    pub organization_id: Uuid,
    pub qc_type: Option<QcType>,
    pub material_status: Option<QcMaterialStatus>,
    pub test_id: Option<Uuid>,
    pub equipment_id: Option<Uuid>,
    pub search_query: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct QcResultFilter {
    pub qc_material_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub result_status: Option<QcResultStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, InputObject)]
pub struct QcViolationFilter {
    pub organization_id: Uuid,
    pub qc_material_id: Option<Uuid>,
    pub severity: Option<ViolationSeverity>,
    pub is_resolved: Option<bool>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}
