use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use common::error::{Error, Result};

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Locked,
    PendingVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserType {
    SuperAdmin,
    OrgAdmin,
    Manager,
    Doctor,
    Technician,
    Nurse,
    Receptionist,
    BillingStaff,
    LabAssistant,
    QualityManager,
    Patient,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "session_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionStatus {
    Active,
    Expired,
    LoggedOut,
    Revoked,
}

// ============================================================================
// User Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    // Identity
    pub id: Uuid,
    pub user_code: String,

    // Organization
    pub organization_id: Option<Uuid>,
    pub department: Option<String>,

    // Personal Information
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub display_name: Option<String>,

    // Contact Information
    pub email: String,
    pub mobile_number: Option<String>,
    pub alternate_phone: Option<String>,

    // Authentication
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub password_salt: Option<String>,
    pub password_changed_at: Option<DateTime<Utc>>,
    pub password_expires_at: Option<DateTime<Utc>>,
    pub must_change_password: bool,

    // User Type and Status
    pub user_type: UserType,
    pub user_status: UserStatus,

    // Professional Information
    pub professional_title: Option<String>,
    pub qualification: Option<String>,
    pub specialization: Option<String>,
    pub license_number: Option<String>,
    pub license_expiry: Option<NaiveDate>,
    pub registration_number: Option<String>,

    // Signature and Identification
    pub digital_signature_path: Option<String>,
    pub photo_path: Option<String>,
    pub employee_id: Option<String>,

    // Security
    pub two_factor_enabled: bool,
    #[serde(skip_serializing)]
    pub two_factor_secret: Option<String>,
    pub backup_codes: Option<serde_json::Value>,

    // Email and Mobile Verification
    pub email_verified: bool,
    #[serde(skip_serializing)]
    pub email_verification_token: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,

    pub mobile_verified: bool,
    #[serde(skip_serializing)]
    pub mobile_verification_token: Option<String>,
    pub mobile_verified_at: Option<DateTime<Utc>>,

    // Login Tracking
    pub last_login_at: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,
    pub login_count: i32,
    pub failed_login_attempts: i32,
    pub last_failed_login_at: Option<DateTime<Utc>>,
    pub locked_until: Option<DateTime<Utc>>,

    // Password Reset
    #[serde(skip_serializing)]
    pub password_reset_token: Option<String>,
    pub password_reset_expires_at: Option<DateTime<Utc>>,

    // Preferences
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub date_format: Option<String>,
    pub theme: Option<String>,
    pub preferences: Option<serde_json::Value>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

impl User {
    pub fn full_name(&self) -> String {
        if let Some(middle) = &self.middle_name {
            format!("{} {} {}", self.first_name, middle, self.last_name)
        } else {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    pub fn is_active(&self) -> bool {
        self.user_status == UserStatus::Active && !self.is_deleted
    }

    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            locked_until > Utc::now()
        } else {
            false
        }
    }

    pub fn should_change_password(&self) -> bool {
        self.must_change_password ||
        self.password_expires_at.map(|exp| exp < Utc::now()).unwrap_or(false)
    }

    pub fn can_login(&self) -> bool {
        self.is_active() && !self.is_locked() && self.email_verified
    }
}

// ============================================================================
// Role Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub role_code: String,
    pub role_name: String,
    pub description: Option<String>,

    pub organization_id: Option<Uuid>,
    pub is_system_role: bool,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ============================================================================
// Permission Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: Uuid,
    pub permission_code: String,
    pub permission_name: String,
    pub description: Option<String>,

    pub module: String,
    pub action: String,

    pub is_system_permission: bool,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// User Session Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub session_token: String,

    pub user_id: Uuid,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub device_type: Option<String>,

    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub location: Option<String>,

    pub session_status: SessionStatus,

    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub logged_out_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing)]
    pub access_token: Option<String>,
    #[serde(skip_serializing)]
    pub refresh_token: Option<String>,
    pub refresh_token_expires_at: Option<DateTime<Utc>>,
}

impl UserSession {
    pub fn is_valid(&self) -> bool {
        self.session_status == SessionStatus::Active && self.expires_at > Utc::now()
    }

    pub fn is_refresh_token_valid(&self) -> bool {
        if let Some(expires_at) = self.refresh_token_expires_at {
            expires_at > Utc::now()
        } else {
            false
        }
    }
}

// ============================================================================
// Activity Log Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActivityLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,

    pub action: String,
    pub module: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,

    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,

    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    pub performed_at: DateTime<Utc>,
}

// ============================================================================
// Input DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserInput {
    pub organization_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub mobile_number: Option<String>,
    pub password: String,
    pub user_type: UserType,
    pub department: Option<String>,
}

impl RegisterUserInput {
    pub fn validate(&self) -> Result<()> {
        if self.first_name.trim().is_empty() {
            return Err(Error::Validation("First name is required".to_string()));
        }
        if self.last_name.trim().is_empty() {
            return Err(Error::Validation("Last name is required".to_string()));
        }
        if self.email.trim().is_empty() || !self.email.contains('@') {
            return Err(Error::Validation("Valid email is required".to_string()));
        }
        if self.password.len() < 8 {
            return Err(Error::Validation("Password must be at least 8 characters".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
}

impl LoginInput {
    pub fn validate(&self) -> Result<()> {
        if self.email.trim().is_empty() {
            return Err(Error::Validation("Email is required".to_string()));
        }
        if self.password.trim().is_empty() {
            return Err(Error::Validation("Password is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordInput {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}

impl ChangePasswordInput {
    pub fn validate(&self) -> Result<()> {
        if self.new_password.len() < 8 {
            return Err(Error::Validation("New password must be at least 8 characters".to_string()));
        }
        if self.current_password == self.new_password {
            return Err(Error::Validation("New password must be different from current password".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordInput {
    pub email: String,
}

impl ResetPasswordInput {
    pub fn validate(&self) -> Result<()> {
        if self.email.trim().is_empty() {
            return Err(Error::Validation("Email is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmPasswordResetInput {
    pub token: String,
    pub new_password: String,
}

impl ConfirmPasswordResetInput {
    pub fn validate(&self) -> Result<()> {
        if self.token.trim().is_empty() {
            return Err(Error::Validation("Reset token is required".to_string()));
        }
        if self.new_password.len() < 8 {
            return Err(Error::Validation("Password must be at least 8 characters".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile_number: Option<String>,
    pub professional_title: Option<String>,
    pub qualification: Option<String>,
    pub specialization: Option<String>,
    pub department: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleInput {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleInput {
    pub role_code: String,
    pub role_name: String,
    pub description: Option<String>,
    pub organization_id: Option<Uuid>,
    pub permission_ids: Vec<Uuid>,
}

impl CreateRoleInput {
    pub fn validate(&self) -> Result<()> {
        if self.role_code.trim().is_empty() {
            return Err(Error::Validation("Role code is required".to_string()));
        }
        if self.role_name.trim().is_empty() {
            return Err(Error::Validation("Role name is required".to_string()));
        }
        Ok(())
    }
}

// ============================================================================
// Output DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

// ============================================================================
// Query Filters
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFilter {
    pub organization_id: Option<Uuid>,
    pub user_type: Option<UserType>,
    pub user_status: Option<UserStatus>,
    pub department: Option<String>,
    pub search_query: Option<String>,
}
