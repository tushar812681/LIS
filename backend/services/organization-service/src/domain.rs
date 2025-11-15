use chrono::{NaiveDateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_graphql::{Enum, InputObject, SimpleObject};
use rust_decimal::Decimal;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "organization_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrganizationStatus {
    Active,
    Inactive,
    Suspended,
    Trial,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "organization_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrganizationType {
    SingleLab,
    MultiBranch,
    HospitalLab,
    DiagnosticCenter,
    ReferenceLab,
    CollectionCenter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "subscription_plan", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionPlan {
    Free,
    Basic,
    Professional,
    Enterprise,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "accreditation_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccreditationType {
    Nabl,
    Cap,
    Iso15189,
    Iso9001,
    Jci,
    Nabh,
}

// ============================================================================
// Organization Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Organization {
    // Identity
    pub id: Uuid,
    pub org_code: String,

    // Basic Information
    pub organization_name: String,
    pub legal_name: Option<String>,
    pub short_name: Option<String>,
    pub organization_type: OrganizationType,

    // Status
    pub organization_status: OrganizationStatus,

    // Registration Details
    pub registration_number: Option<String>,
    pub pan_number: Option<String>,
    pub gstin: Option<String>,
    pub cin: Option<String>,

    // Contact Information
    pub email: String,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub website: Option<String>,

    // Primary Address
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,

    // Geographic Coordinates
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,

    // Subscription and Licensing
    pub subscription_plan: Option<SubscriptionPlan>,
    pub subscription_start_date: Option<NaiveDate>,
    pub subscription_end_date: Option<NaiveDate>,
    pub max_users: Option<i32>,
    pub max_branches: Option<i32>,
    pub max_tests_per_month: Option<i32>,
    pub current_month_tests: Option<i32>,

    // Branding
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub header_image_url: Option<String>,
    pub footer_text: Option<String>,

    // Settings
    pub settings: Option<serde_json::Value>,
    pub features_enabled: Option<serde_json::Value>,

    // Parent Organization
    pub parent_organization_id: Option<Uuid>,

    // Contact Person
    pub contact_person_name: Option<String>,
    pub contact_person_email: Option<String>,
    pub contact_person_phone: Option<String>,

    // Business Hours
    pub business_hours: Option<serde_json::Value>,

    // Timezone
    pub timezone: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub is_deleted: Option<bool>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

impl Organization {
    pub fn is_active(&self) -> bool {
        self.organization_status == OrganizationStatus::Active
    }

    pub fn is_trial(&self) -> bool {
        self.organization_status == OrganizationStatus::Trial
    }

    pub fn is_subscription_valid(&self) -> bool {
        if let Some(end_date) = self.subscription_end_date {
            end_date >= chrono::Local::now().date_naive()
        } else {
            false
        }
    }

    pub fn can_add_user(&self, current_user_count: i32) -> bool {
        if let Some(max_users) = self.max_users {
            current_user_count < max_users
        } else {
            false
        }
    }

    pub fn can_add_branch(&self, current_branch_count: i32) -> bool {
        if let Some(max_branches) = self.max_branches {
            current_branch_count < max_branches
        } else {
            false
        }
    }

    pub fn can_process_test(&self) -> bool {
        if let Some(max_tests) = self.max_tests_per_month {
            if let Some(current_tests) = self.current_month_tests {
                return current_tests < max_tests;
            }
        }
        false
    }

    pub fn days_until_expiry(&self) -> Option<i64> {
        self.subscription_end_date.map(|end_date| {
            let today = chrono::Local::now().date_naive();
            (end_date - today).num_days()
        })
    }

    pub fn is_multi_branch(&self) -> bool {
        matches!(self.organization_type, OrganizationType::MultiBranch)
    }
}

// ============================================================================
// Organization Branch Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct OrganizationBranch {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Branch Information
    pub branch_code: String,
    pub branch_name: String,

    // Status
    #[graphql(skip)]
    pub is_active: Option<bool>,
    pub is_main_branch: Option<bool>,

    // Contact Information
    pub email: Option<String>,
    pub phone: Option<String>,

    // Address
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,

    // Geographic Coordinates
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,

    // Branch Manager
    pub manager_id: Option<Uuid>,
    pub manager_name: Option<String>,
    pub manager_email: Option<String>,
    pub manager_phone: Option<String>,

    // Capacity
    pub sample_processing_capacity: Option<i32>,

    // Business Hours
    pub business_hours: Option<serde_json::Value>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl OrganizationBranch {
    pub fn is_active(&self) -> bool {
        self.is_active.unwrap_or(false)
    }

    pub fn is_main(&self) -> bool {
        self.is_main_branch.unwrap_or(false)
    }

    pub fn has_capacity(&self, current_samples: i32) -> bool {
        if let Some(capacity) = self.sample_processing_capacity {
            current_samples < capacity
        } else {
            true // No capacity limit
        }
    }
}

// ============================================================================
// Accreditation Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Accreditation {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Accreditation Details
    pub accreditation_type: AccreditationType,
    pub accreditation_number: String,

    // Issuing Authority
    pub issuing_authority: Option<String>,

    // Validity
    pub issue_date: NaiveDate,
    pub expiry_date: NaiveDate,

    // Scope
    pub scope_of_accreditation: Option<String>,
    pub accredited_tests: Option<serde_json::Value>,

    // Document
    pub certificate_url: Option<String>,

    // Status
    pub is_active: Option<bool>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Accreditation {
    pub fn is_valid(&self) -> bool {
        let today = chrono::Local::now().date_naive();
        self.expiry_date >= today && self.is_active.unwrap_or(false)
    }

    pub fn days_until_expiry(&self) -> i64 {
        let today = chrono::Local::now().date_naive();
        (self.expiry_date - today).num_days()
    }

    pub fn is_expired(&self) -> bool {
        let today = chrono::Local::now().date_naive();
        self.expiry_date < today
    }

    pub fn needs_renewal(&self, days_threshold: i64) -> bool {
        self.days_until_expiry() <= days_threshold
    }
}

// ============================================================================
// Organization Setting Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct OrganizationSetting {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Setting Details
    pub setting_category: String,
    pub setting_key: String,
    pub setting_value: String,
    pub setting_type: Option<String>,

    // Description
    pub description: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<Uuid>,
}

// ============================================================================
// Department Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Department {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,

    // Department Information
    pub department_code: String,
    pub department_name: String,
    pub description: Option<String>,

    // Head of Department
    pub hod_user_id: Option<Uuid>,
    pub hod_name: Option<String>,

    // Contact
    pub email: Option<String>,
    pub phone: Option<String>,

    // Status
    #[graphql(skip)]
    pub is_active: Option<bool>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Department {
    pub fn is_active(&self) -> bool {
        self.is_active.unwrap_or(false)
    }
}

// ============================================================================
// Working Hours Template Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct WorkingHoursTemplate {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Template Information
    pub template_name: String,
    pub description: Option<String>,

    // Schedule
    pub schedule: serde_json::Value,

    // Holidays
    pub public_holidays: Option<serde_json::Value>,

    // Status
    pub is_active: Option<bool>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

// ============================================================================
// Organization Audit Log Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct OrganizationAuditLog {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Action Details
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,

    // Changes
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,

    // Actor
    pub performed_by: Option<Uuid>,
    pub performed_at: Option<NaiveDateTime>,

    // Context
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub description: Option<String>,
}

// ============================================================================
// Input DTOs
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct CreateOrganizationInput {
    pub organization_name: String,
    pub legal_name: Option<String>,
    pub short_name: Option<String>,
    pub organization_type: OrganizationType,

    // Contact Information
    pub email: String,
    pub phone: Option<String>,
    pub website: Option<String>,

    // Address
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,

    // Contact Person
    pub contact_person_name: Option<String>,
    pub contact_person_email: Option<String>,
    pub contact_person_phone: Option<String>,

    // Subscription
    pub subscription_plan: Option<SubscriptionPlan>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateOrganizationInput {
    pub id: Uuid,

    pub organization_name: Option<String>,
    pub legal_name: Option<String>,
    pub short_name: Option<String>,

    // Contact Information
    pub email: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub website: Option<String>,

    // Address
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,

    // Branding
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub header_image_url: Option<String>,
    pub footer_text: Option<String>,

    // Contact Person
    pub contact_person_name: Option<String>,
    pub contact_person_email: Option<String>,
    pub contact_person_phone: Option<String>,

    // Timezone
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateOrganizationStatusInput {
    pub id: Uuid,
    pub status: OrganizationStatus,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateSubscriptionInput {
    pub id: Uuid,
    pub subscription_plan: SubscriptionPlan,
    pub subscription_start_date: NaiveDate,
    pub subscription_end_date: NaiveDate,
    pub max_users: Option<i32>,
    pub max_branches: Option<i32>,
    pub max_tests_per_month: Option<i32>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateBranchInput {
    pub organization_id: Uuid,
    pub branch_code: String,
    pub branch_name: String,

    pub email: Option<String>,
    pub phone: Option<String>,

    // Address
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,

    // Branch Manager
    pub manager_id: Option<Uuid>,
    pub manager_name: Option<String>,
    pub manager_email: Option<String>,
    pub manager_phone: Option<String>,

    pub sample_processing_capacity: Option<i32>,
    pub is_main_branch: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateBranchInput {
    pub id: Uuid,
    pub branch_name: Option<String>,

    pub email: Option<String>,
    pub phone: Option<String>,

    // Address
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,

    // Branch Manager
    pub manager_id: Option<Uuid>,
    pub manager_name: Option<String>,
    pub manager_email: Option<String>,
    pub manager_phone: Option<String>,

    pub sample_processing_capacity: Option<i32>,
}

#[derive(Debug, Clone, InputObject)]
pub struct AddAccreditationInput {
    pub organization_id: Uuid,
    pub accreditation_type: AccreditationType,
    pub accreditation_number: String,
    pub issuing_authority: Option<String>,
    pub issue_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub scope_of_accreditation: Option<String>,
    pub certificate_url: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateAccreditationInput {
    pub id: Uuid,
    pub expiry_date: Option<NaiveDate>,
    pub scope_of_accreditation: Option<String>,
    pub certificate_url: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateDepartmentInput {
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub department_code: String,
    pub department_name: String,
    pub description: Option<String>,

    pub hod_user_id: Option<Uuid>,
    pub hod_name: Option<String>,

    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateDepartmentInput {
    pub id: Uuid,
    pub department_name: Option<String>,
    pub description: Option<String>,

    pub hod_user_id: Option<Uuid>,
    pub hod_name: Option<String>,

    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateOrganizationSettingInput {
    pub organization_id: Uuid,
    pub setting_category: String,
    pub setting_key: String,
    pub setting_value: String,
    pub setting_type: Option<String>,
    pub description: Option<String>,
}

// ============================================================================
// Query Filters
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct OrganizationFilter {
    pub organization_status: Option<OrganizationStatus>,
    pub organization_type: Option<OrganizationType>,
    pub subscription_plan: Option<SubscriptionPlan>,
    pub search_query: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct BranchFilter {
    pub organization_id: Uuid,
    pub is_active: Option<bool>,
    pub city: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct DepartmentFilter {
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub is_active: Option<bool>,
}
