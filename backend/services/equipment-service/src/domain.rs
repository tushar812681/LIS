use chrono::{NaiveDateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_graphql::{Enum, InputObject, SimpleObject};
use rust_decimal::Decimal;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "equipment_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EquipmentStatus {
    Active,
    Inactive,
    Maintenance,
    Calibration,
    UnderRepair,
    Retired,
    Quarantine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "equipment_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EquipmentType {
    Analyzer,
    Centrifuge,
    Microscope,
    Refrigerator,
    Freezer,
    Incubator,
    Autoclave,
    WaterBath,
    PcrMachine,
    Spectrophotometer,
    HematologyAnalyzer,
    ChemistryAnalyzer,
    ImmunoassayAnalyzer,
    CoagulationAnalyzer,
    BloodGasAnalyzer,
    MicrobiologyAnalyzer,
    ElisaReader,
    Pipette,
    Balance,
    PhMeter,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "maintenance_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaintenanceType {
    Preventive,
    Corrective,
    Calibration,
    Validation,
    Qualification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "maintenance_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaintenanceStatus {
    Scheduled,
    InProgress,
    Completed,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "calibration_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalibrationStatus {
    Passed,
    Failed,
    Conditional,
    Pending,
}

// ============================================================================
// Equipment Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Equipment {
    // Identity
    pub id: Uuid,
    pub equipment_code: String,

    // Basic Information
    pub equipment_name: String,
    pub equipment_type: EquipmentType,
    pub manufacturer: Option<String>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,

    // Status
    pub equipment_status: EquipmentStatus,

    // Organization and Location
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub location: Option<String>,

    // Purchase Details
    pub purchase_date: Option<NaiveDate>,
    pub purchase_cost: Option<Decimal>,
    pub vendor: Option<String>,
    pub warranty_expiry_date: Option<NaiveDate>,

    // Installation
    pub installation_date: Option<NaiveDate>,
    pub commissioning_date: Option<NaiveDate>,

    // Specifications
    pub specifications: Option<serde_json::Value>,
    pub capacity: Option<String>,

    // Connectivity
    pub interface_type: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub lis_integration_enabled: Option<bool>,

    // Maintenance Schedule
    pub maintenance_frequency_days: Option<i32>,
    pub last_maintenance_date: Option<NaiveDate>,
    pub next_maintenance_date: Option<NaiveDate>,

    // Calibration Schedule
    pub calibration_frequency_days: Option<i32>,
    pub last_calibration_date: Option<NaiveDate>,
    pub next_calibration_date: Option<NaiveDate>,

    // Performance
    pub total_tests_processed: Option<i32>,
    pub uptime_percentage: Option<Decimal>,
    pub mean_time_between_failures: Option<i32>,

    // Documentation
    pub user_manual_url: Option<String>,
    pub service_manual_url: Option<String>,
    pub sop_document_url: Option<String>,

    // Notes
    pub notes: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub is_deleted: Option<bool>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

impl Equipment {
    pub fn is_operational(&self) -> bool {
        matches!(
            self.equipment_status,
            EquipmentStatus::Active
        )
    }

    pub fn needs_maintenance(&self) -> bool {
        if let Some(next_date) = self.next_maintenance_date {
            next_date <= chrono::Local::now().date_naive()
        } else {
            false
        }
    }

    pub fn needs_calibration(&self) -> bool {
        if let Some(next_date) = self.next_calibration_date {
            next_date <= chrono::Local::now().date_naive()
        } else {
            false
        }
    }

    pub fn is_under_warranty(&self) -> bool {
        if let Some(expiry) = self.warranty_expiry_date {
            expiry >= chrono::Local::now().date_naive()
        } else {
            false
        }
    }

    pub fn days_since_last_maintenance(&self) -> Option<i64> {
        self.last_maintenance_date.map(|date| {
            let today = chrono::Local::now().date_naive();
            (today - date).num_days()
        })
    }

    pub fn days_until_next_calibration(&self) -> Option<i64> {
        self.next_calibration_date.map(|date| {
            let today = chrono::Local::now().date_naive();
            (date - today).num_days()
        })
    }

    pub fn calculate_age_years(&self) -> Option<i32> {
        self.installation_date.map(|date| {
            let today = chrono::Local::now().date_naive();
            let years = (today - date).num_days() / 365;
            years as i32
        })
    }
}

// ============================================================================
// Equipment Maintenance Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct EquipmentMaintenance {
    pub id: Uuid,
    pub equipment_id: Uuid,

    // Maintenance Details
    pub maintenance_type: MaintenanceType,
    pub maintenance_status: MaintenanceStatus,

    // Schedule
    pub scheduled_date: NaiveDate,
    pub completed_date: Option<NaiveDate>,

    // Performed By
    pub technician_id: Option<Uuid>,
    pub technician_name: Option<String>,
    pub vendor_name: Option<String>,

    // Work Done
    pub work_description: Option<String>,
    pub parts_replaced: Option<serde_json::Value>,
    pub cost: Option<Decimal>,

    // Results
    pub before_condition: Option<String>,
    pub after_condition: Option<String>,
    pub findings: Option<String>,
    pub recommendations: Option<String>,

    // Next Maintenance
    pub next_maintenance_date: Option<NaiveDate>,

    // Documentation
    pub report_url: Option<String>,
    pub checklist_completed: Option<serde_json::Value>,

    // Downtime Tracking
    pub downtime_start: Option<NaiveDateTime>,
    pub downtime_end: Option<NaiveDateTime>,
    pub downtime_hours: Option<Decimal>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

impl EquipmentMaintenance {
    pub fn is_completed(&self) -> bool {
        self.maintenance_status == MaintenanceStatus::Completed
    }

    pub fn is_overdue(&self) -> bool {
        if self.maintenance_status == MaintenanceStatus::Completed {
            return false;
        }
        self.scheduled_date < chrono::Local::now().date_naive()
    }

    pub fn days_overdue(&self) -> Option<i64> {
        if self.is_overdue() {
            let today = chrono::Local::now().date_naive();
            Some((today - self.scheduled_date).num_days())
        } else {
            None
        }
    }
}

// ============================================================================
// Equipment Calibration Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct EquipmentCalibration {
    pub id: Uuid,
    pub equipment_id: Uuid,

    // Calibration Details
    pub calibration_date: NaiveDate,
    pub due_date: NaiveDate,
    pub calibration_status: CalibrationStatus,

    // Performed By
    pub performed_by_id: Option<Uuid>,
    pub performed_by_name: Option<String>,
    pub calibration_agency: Option<String>,

    // Certificate
    pub certificate_number: Option<String>,
    pub certificate_url: Option<String>,

    // Calibration Points
    pub calibration_points: Option<serde_json::Value>,

    // Standards Used
    pub reference_standards_used: Option<serde_json::Value>,

    // Results
    pub before_accuracy: Option<Decimal>,
    pub after_accuracy: Option<Decimal>,
    pub within_specification: Option<bool>,

    // Deviations
    pub deviations_found: Option<String>,
    pub corrective_actions: Option<String>,

    // Next Calibration
    pub next_calibration_date: Option<NaiveDate>,

    // Cost
    pub cost: Option<Decimal>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

impl EquipmentCalibration {
    pub fn is_passed(&self) -> bool {
        matches!(
            self.calibration_status,
            CalibrationStatus::Passed | CalibrationStatus::Conditional
        )
    }

    pub fn is_overdue(&self) -> bool {
        self.due_date < chrono::Local::now().date_naive()
    }

    pub fn accuracy_improvement(&self) -> Option<Decimal> {
        if let (Some(before), Some(after)) = (self.before_accuracy, self.after_accuracy) {
            Some(after - before)
        } else {
            None
        }
    }
}

// ============================================================================
// Equipment Test Assignment Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct EquipmentTestAssignment {
    pub id: Uuid,
    pub equipment_id: Uuid,
    pub test_id: Uuid,

    // Assignment Details
    pub is_primary: Option<bool>,
    pub is_backup: Option<bool>,

    // Performance
    pub average_tat_minutes: Option<i32>,
    pub success_rate: Option<Decimal>,

    // Status
    pub is_active: Option<bool>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

// ============================================================================
// Equipment Performance Log Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct EquipmentPerformanceLog {
    pub id: Uuid,
    pub equipment_id: Uuid,

    // Performance Data
    pub log_date: NaiveDate,
    pub tests_processed: Option<i32>,
    pub tests_failed: Option<i32>,
    pub downtime_minutes: Option<i32>,

    // Quality Metrics
    pub error_rate: Option<Decimal>,
    pub success_rate: Option<Decimal>,
    pub average_processing_time_seconds: Option<Decimal>,

    // Reagent Consumption
    pub reagent_consumption: Option<serde_json::Value>,

    // Issues
    pub issues_reported: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
}

// ============================================================================
// Equipment Alert Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct EquipmentAlert {
    pub id: Uuid,
    pub equipment_id: Uuid,

    // Alert Details
    pub alert_type: String,
    pub severity: String,
    pub message: String,

    // Status
    pub is_acknowledged: Option<bool>,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<NaiveDateTime>,

    pub is_resolved: Option<bool>,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<NaiveDateTime>,
    pub resolution_notes: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
}

// ============================================================================
// Input DTOs
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct CreateEquipmentInput {
    pub equipment_name: String,
    pub equipment_type: EquipmentType,
    pub manufacturer: Option<String>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,

    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub location: Option<String>,

    pub purchase_date: Option<NaiveDate>,
    pub purchase_cost: Option<Decimal>,
    pub vendor: Option<String>,
    pub warranty_expiry_date: Option<NaiveDate>,

    pub installation_date: Option<NaiveDate>,
    pub commissioning_date: Option<NaiveDate>,

    pub capacity: Option<String>,
    pub interface_type: Option<String>,

    pub maintenance_frequency_days: Option<i32>,
    pub calibration_frequency_days: Option<i32>,

    pub lis_integration_enabled: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateEquipmentInput {
    pub id: Uuid,

    pub equipment_name: Option<String>,
    pub location: Option<String>,
    pub capacity: Option<String>,

    pub warranty_expiry_date: Option<NaiveDate>,

    pub interface_type: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub lis_integration_enabled: Option<bool>,

    pub maintenance_frequency_days: Option<i32>,
    pub calibration_frequency_days: Option<i32>,

    pub user_manual_url: Option<String>,
    pub service_manual_url: Option<String>,
    pub sop_document_url: Option<String>,

    pub notes: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateEquipmentStatusInput {
    pub id: Uuid,
    pub status: EquipmentStatus,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct ScheduleMaintenanceInput {
    pub equipment_id: Uuid,
    pub maintenance_type: MaintenanceType,
    pub scheduled_date: NaiveDate,
    pub technician_id: Option<Uuid>,
    pub technician_name: Option<String>,
    pub vendor_name: Option<String>,
    pub work_description: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CompleteMaintenanceInput {
    pub id: Uuid,
    pub completed_date: NaiveDate,
    pub work_description: Option<String>,
    pub parts_replaced: Option<serde_json::Value>,
    pub cost: Option<Decimal>,
    pub before_condition: Option<String>,
    pub after_condition: Option<String>,
    pub findings: Option<String>,
    pub recommendations: Option<String>,
    pub next_maintenance_date: Option<NaiveDate>,
    pub report_url: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct RecordCalibrationInput {
    pub equipment_id: Uuid,
    pub calibration_date: NaiveDate,
    pub due_date: NaiveDate,
    pub calibration_status: CalibrationStatus,

    pub performed_by_id: Option<Uuid>,
    pub performed_by_name: Option<String>,
    pub calibration_agency: Option<String>,

    pub certificate_number: Option<String>,
    pub certificate_url: Option<String>,

    pub before_accuracy: Option<Decimal>,
    pub after_accuracy: Option<Decimal>,
    pub within_specification: Option<bool>,

    pub deviations_found: Option<String>,
    pub corrective_actions: Option<String>,

    pub next_calibration_date: Option<NaiveDate>,
    pub cost: Option<Decimal>,
}

#[derive(Debug, Clone, InputObject)]
pub struct AssignTestInput {
    pub equipment_id: Uuid,
    pub test_id: Uuid,
    pub is_primary: Option<bool>,
    pub is_backup: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct LogPerformanceInput {
    pub equipment_id: Uuid,
    pub log_date: NaiveDate,
    pub tests_processed: i32,
    pub tests_failed: i32,
    pub downtime_minutes: i32,
    pub issues_reported: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct AcknowledgeAlertInput {
    pub id: Uuid,
}

#[derive(Debug, Clone, InputObject)]
pub struct ResolveAlertInput {
    pub id: Uuid,
    pub resolution_notes: Option<String>,
}

// ============================================================================
// Query Filters
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct EquipmentFilter {
    pub organization_id: Uuid,
    pub equipment_status: Option<EquipmentStatus>,
    pub equipment_type: Option<EquipmentType>,
    pub branch_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub search_query: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct MaintenanceFilter {
    pub equipment_id: Option<Uuid>,
    pub maintenance_type: Option<MaintenanceType>,
    pub maintenance_status: Option<MaintenanceStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CalibrationFilter {
    pub equipment_id: Option<Uuid>,
    pub calibration_status: Option<CalibrationStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, InputObject)]
pub struct AlertFilter {
    pub equipment_id: Option<Uuid>,
    pub alert_type: Option<String>,
    pub severity: Option<String>,
    pub is_resolved: Option<bool>,
}
