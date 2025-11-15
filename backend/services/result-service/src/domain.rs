use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use common::error::{Error, Result};

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "result_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResultStatus {
    Pending,
    InProgress,
    Preliminary,
    Final,
    Corrected,
    Cancelled,
    Amended,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "verification_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationStatus {
    NotVerified,
    AutoVerified,
    ManuallyVerified,
    VerificationFailed,
    PendingReview,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "critical_flag", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CriticalFlag {
    None,
    Low,
    High,
    PanicLow,
    PanicHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "delta_flag", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeltaFlag {
    Normal,
    SignificantIncrease,
    SignificantDecrease,
    NoPreviousResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "interpretation_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterpretationType {
    Normal,
    AbnormalLow,
    AbnormalHigh,
    CriticalLow,
    CriticalHigh,
    Indeterminate,
}

// ============================================================================
// Test Result Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestResult {
    // Identity
    pub id: Uuid,
    pub result_number: String,

    // Linkage
    pub patient_id: Uuid,
    pub order_id: Uuid,
    pub order_item_id: Uuid,
    pub test_id: Uuid,
    pub sample_id: Uuid,
    pub organization_id: Uuid,

    // Test Information
    pub test_code: String,
    pub test_name: String,
    pub department: Option<String>,

    // Result Values
    pub result_value: Option<String>,
    pub result_unit: Option<String>,
    pub result_type: String,

    // Reference Ranges
    pub reference_range_text: Option<String>,
    pub reference_range_min: Option<rust_decimal::Decimal>,
    pub reference_range_max: Option<rust_decimal::Decimal>,

    // Interpretation
    pub interpretation: InterpretationType,
    pub clinical_interpretation: Option<String>,

    // Flags
    pub critical_flag: CriticalFlag,
    pub delta_flag: DeltaFlag,
    pub is_abnormal: bool,
    pub is_critical: bool,

    // Delta Check
    pub previous_result_value: Option<String>,
    pub previous_result_date: Option<DateTime<Utc>>,
    pub delta_percentage: Option<rust_decimal::Decimal>,
    pub delta_absolute: Option<rust_decimal::Decimal>,

    // Status and Workflow
    pub result_status: ResultStatus,
    pub verification_status: VerificationStatus,

    // Entry Information
    pub entry_method: Option<String>,
    pub entered_by: Option<Uuid>,
    pub entry_date: DateTime<Utc>,

    // Verification
    pub verified_by: Option<Uuid>,
    pub verification_date: Option<DateTime<Utc>>,
    pub auto_verification_confidence: Option<rust_decimal::Decimal>,
    pub verification_rules_passed: Option<serde_json::Value>,
    pub verification_rules_failed: Option<serde_json::Value>,

    // Approval
    pub approved_by: Option<Uuid>,
    pub approval_date: Option<DateTime<Utc>>,
    pub approval_notes: Option<String>,

    // Instrument Information
    pub instrument_id: Option<Uuid>,
    pub instrument_name: Option<String>,
    pub run_number: Option<String>,

    // Quality Control
    pub qc_lot_number: Option<String>,
    pub qc_passed: Option<bool>,
    pub qc_notes: Option<String>,

    // Timing
    pub result_date: DateTime<Utc>,
    pub reported_date: Option<DateTime<Utc>>,
    pub tat_hours: Option<rust_decimal::Decimal>,

    // Additional Information
    pub method_used: Option<String>,
    pub reagent_lot: Option<String>,
    pub dilution_factor: Option<rust_decimal::Decimal>,
    pub specimen_condition: Option<String>,

    // Comments and Notes
    pub technician_notes: Option<String>,
    pub pathologist_notes: Option<String>,
    pub internal_notes: Option<String>,

    // Correction/Amendment
    pub is_corrected: bool,
    pub corrected_from_result_id: Option<Uuid>,
    pub correction_reason: Option<String>,
    pub correction_date: Option<DateTime<Utc>>,
    pub corrected_by: Option<Uuid>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

impl TestResult {
    pub fn is_within_reference_range(&self) -> bool {
        if let Some(value_str) = &self.result_value {
            if let Ok(value) = value_str.parse::<f64>() {
                if let (Some(min), Some(max)) = (self.reference_range_min, self.reference_range_max) {
                    let min_f = min.to_string().parse::<f64>().unwrap_or(f64::MIN);
                    let max_f = max.to_string().parse::<f64>().unwrap_or(f64::MAX);
                    return value >= min_f && value <= max_f;
                }
            }
        }
        false
    }

    pub fn calculate_interpretation(&mut self) {
        if let Some(value_str) = &self.result_value {
            if let Ok(value) = value_str.parse::<f64>() {
                if let (Some(min), Some(max)) = (self.reference_range_min, self.reference_range_max) {
                    let min_f = min.to_string().parse::<f64>().unwrap_or(f64::MIN);
                    let max_f = max.to_string().parse::<f64>().unwrap_or(f64::MAX);

                    self.interpretation = if value < min_f {
                        self.is_abnormal = true;
                        if self.critical_flag == CriticalFlag::PanicLow || self.critical_flag == CriticalFlag::Low {
                            InterpretationType::CriticalLow
                        } else {
                            InterpretationType::AbnormalLow
                        }
                    } else if value > max_f {
                        self.is_abnormal = true;
                        if self.critical_flag == CriticalFlag::PanicHigh || self.critical_flag == CriticalFlag::High {
                            InterpretationType::CriticalHigh
                        } else {
                            InterpretationType::AbnormalHigh
                        }
                    } else {
                        self.is_abnormal = false;
                        InterpretationType::Normal
                    };
                }
            }
        }
    }

    pub fn is_verified(&self) -> bool {
        matches!(
            self.verification_status,
            VerificationStatus::AutoVerified | VerificationStatus::ManuallyVerified
        )
    }

    pub fn is_approved(&self) -> bool {
        self.approved_by.is_some() && self.approval_date.is_some()
    }

    pub fn is_reportable(&self) -> bool {
        self.is_verified() && self.result_status == ResultStatus::Final
    }
}

// ============================================================================
// Reference Range Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReferenceRange {
    pub id: Uuid,
    pub test_id: Uuid,
    pub organization_id: Option<Uuid>,

    // Range Criteria
    pub age_min: Option<i32>,
    pub age_max: Option<i32>,
    pub gender: Option<String>,

    // Range Values
    pub range_min: Option<rust_decimal::Decimal>,
    pub range_max: Option<rust_decimal::Decimal>,
    pub range_text: Option<String>,

    // Critical Values
    pub panic_low: Option<rust_decimal::Decimal>,
    pub panic_high: Option<rust_decimal::Decimal>,
    pub critical_low: Option<rust_decimal::Decimal>,
    pub critical_high: Option<rust_decimal::Decimal>,

    // Unit
    pub unit: Option<String>,

    // Validity
    pub is_active: bool,
    pub effective_from: Option<NaiveDate>,
    pub effective_to: Option<NaiveDate>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl ReferenceRange {
    pub fn is_applicable(&self, age: i32, gender: &str) -> bool {
        let age_match = match (self.age_min, self.age_max) {
            (Some(min), Some(max)) => age >= min && age <= max,
            (Some(min), None) => age >= min,
            (None, Some(max)) => age <= max,
            (None, None) => true,
        };

        let gender_match = if let Some(ref range_gender) = self.gender {
            range_gender == "ALL" || range_gender.eq_ignore_ascii_case(gender)
        } else {
            true
        };

        age_match && gender_match && self.is_active
    }
}

// ============================================================================
// Auto-Verification Rule Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AutoVerificationRule {
    pub id: Uuid,
    pub rule_code: String,
    pub rule_name: String,

    // Applicability
    pub test_id: Option<Uuid>,
    pub department: Option<String>,
    pub organization_id: Option<Uuid>,
    pub is_global: bool,

    // Rule Configuration
    pub rule_type: String,
    pub rule_definition: serde_json::Value,

    // Thresholds
    pub min_value: Option<rust_decimal::Decimal>,
    pub max_value: Option<rust_decimal::Decimal>,
    pub delta_percentage_limit: Option<rust_decimal::Decimal>,
    pub delta_absolute_limit: Option<rust_decimal::Decimal>,

    // Priority and Weight
    pub priority: i32,
    pub is_blocking: bool,

    // Status
    pub is_active: bool,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ============================================================================
// Critical Result Notification Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CriticalResultNotification {
    pub id: Uuid,
    pub result_id: Uuid,

    // Notification Details
    pub notified_to: String,
    pub notification_method: String,
    pub notification_date: DateTime<Utc>,

    // Acknowledgment
    pub acknowledged: bool,
    pub acknowledged_by: Option<String>,
    pub acknowledgment_date: Option<DateTime<Utc>>,
    pub acknowledgment_method: Option<String>,

    // Documentation
    pub caller_name: Option<String>,
    pub call_back_number: Option<String>,
    pub notes: Option<String>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

// ============================================================================
// Input DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResultInput {
    pub order_id: Uuid,
    pub order_item_id: Uuid,
    pub test_id: Uuid,
    pub sample_id: Uuid,
    pub result_value: String,
    pub result_unit: Option<String>,
    pub entry_method: String,
    pub instrument_id: Option<Uuid>,
    pub run_number: Option<String>,
    pub technician_notes: Option<String>,
}

impl CreateResultInput {
    pub fn validate(&self) -> Result<()> {
        if self.result_value.trim().is_empty() {
            return Err(Error::Validation("Result value cannot be empty".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResultInput {
    pub result_id: Uuid,
    pub result_value: String,
    pub result_unit: Option<String>,
    pub technician_notes: Option<String>,
}

impl UpdateResultInput {
    pub fn validate(&self) -> Result<()> {
        if self.result_value.trim().is_empty() {
            return Err(Error::Validation("Result value cannot be empty".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResultInput {
    pub result_id: Uuid,
    pub verification_method: String, // AUTO, MANUAL
    pub pathologist_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveResultInput {
    pub result_id: Uuid,
    pub approval_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectResultInput {
    pub result_id: Uuid,
    pub new_result_value: String,
    pub correction_reason: String,
}

impl CorrectResultInput {
    pub fn validate(&self) -> Result<()> {
        if self.new_result_value.trim().is_empty() {
            return Err(Error::Validation("New result value cannot be empty".to_string()));
        }
        if self.correction_reason.trim().is_empty() {
            return Err(Error::Validation("Correction reason is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordCriticalNotificationInput {
    pub result_id: Uuid,
    pub notified_to: String,
    pub notification_method: String,
    pub caller_name: Option<String>,
    pub call_back_number: Option<String>,
    pub notes: Option<String>,
}

impl RecordCriticalNotificationInput {
    pub fn validate(&self) -> Result<()> {
        if self.notified_to.trim().is_empty() {
            return Err(Error::Validation("Notified to field is required".to_string()));
        }
        if self.notification_method.trim().is_empty() {
            return Err(Error::Validation("Notification method is required".to_string()));
        }
        Ok(())
    }
}

// ============================================================================
// Query Filters
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultFilter {
    pub patient_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub sample_id: Option<Uuid>,
    pub test_id: Option<Uuid>,
    pub result_status: Option<ResultStatus>,
    pub verification_status: Option<VerificationStatus>,
    pub is_critical: Option<bool>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}
