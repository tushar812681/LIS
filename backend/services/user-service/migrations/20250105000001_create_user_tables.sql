-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Custom Types
-- ============================================================================

CREATE TYPE user_status AS ENUM (
    'ACTIVE',
    'INACTIVE',
    'SUSPENDED',
    'LOCKED',
    'PENDING_VERIFICATION'
);

CREATE TYPE user_type AS ENUM (
    'SUPER_ADMIN',
    'ORG_ADMIN',
    'MANAGER',
    'DOCTOR',
    'TECHNICIAN',
    'NURSE',
    'RECEPTIONIST',
    'BILLING_STAFF',
    'LAB_ASSISTANT',
    'QUALITY_MANAGER',
    'PATIENT'
);

CREATE TYPE session_status AS ENUM (
    'ACTIVE',
    'EXPIRED',
    'LOGGED_OUT',
    'REVOKED'
);

-- ============================================================================
-- User Table
-- ============================================================================

CREATE TABLE users (
    -- Identity
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_code VARCHAR(50) UNIQUE NOT NULL,

    -- Organization
    organization_id UUID,
    department VARCHAR(100),

    -- Personal Information
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    middle_name VARCHAR(100),
    display_name VARCHAR(200),

    -- Contact Information
    email VARCHAR(255) UNIQUE NOT NULL,
    mobile_number VARCHAR(20) UNIQUE,
    alternate_phone VARCHAR(20),

    -- Authentication
    password_hash VARCHAR(255) NOT NULL,
    password_salt VARCHAR(100),
    password_changed_at TIMESTAMPTZ,
    password_expires_at TIMESTAMPTZ,
    must_change_password BOOLEAN DEFAULT FALSE,

    -- User Type and Status
    user_type user_type NOT NULL,
    user_status user_status NOT NULL DEFAULT 'PENDING_VERIFICATION',

    -- Professional Information
    professional_title VARCHAR(100),
    qualification VARCHAR(500),
    specialization VARCHAR(200),
    license_number VARCHAR(100),
    license_expiry DATE,
    registration_number VARCHAR(100),

    -- Signature and Identification
    digital_signature_path VARCHAR(500),
    photo_path VARCHAR(500),
    employee_id VARCHAR(50),

    -- Security
    two_factor_enabled BOOLEAN DEFAULT FALSE,
    two_factor_secret VARCHAR(100),
    backup_codes JSONB,

    -- Email and Mobile Verification
    email_verified BOOLEAN DEFAULT FALSE,
    email_verification_token VARCHAR(255),
    email_verified_at TIMESTAMPTZ,

    mobile_verified BOOLEAN DEFAULT FALSE,
    mobile_verification_token VARCHAR(10),
    mobile_verified_at TIMESTAMPTZ,

    -- Login Tracking
    last_login_at TIMESTAMPTZ,
    last_login_ip INET,
    login_count INTEGER DEFAULT 0,
    failed_login_attempts INTEGER DEFAULT 0,
    last_failed_login_at TIMESTAMPTZ,
    locked_until TIMESTAMPTZ,

    -- Password Reset
    password_reset_token VARCHAR(255),
    password_reset_expires_at TIMESTAMPTZ,

    -- Preferences
    timezone VARCHAR(50) DEFAULT 'Asia/Kolkata',
    language VARCHAR(10) DEFAULT 'en',
    date_format VARCHAR(20) DEFAULT 'DD/MM/YYYY',
    theme VARCHAR(20) DEFAULT 'light',
    preferences JSONB,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID,

    -- Constraints
    CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    CONSTRAINT valid_mobile CHECK (mobile_number ~ '^[0-9]{10,15}$')
);

-- Indexes
CREATE INDEX idx_users_email ON users(email) WHERE is_deleted = FALSE;
CREATE INDEX idx_users_mobile ON users(mobile_number) WHERE is_deleted = FALSE;
CREATE INDEX idx_users_org ON users(organization_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_users_type ON users(user_type) WHERE is_deleted = FALSE;
CREATE INDEX idx_users_status ON users(user_status) WHERE is_deleted = FALSE;
CREATE INDEX idx_users_code ON users(user_code) WHERE is_deleted = FALSE;
CREATE INDEX idx_users_employee_id ON users(employee_id) WHERE is_deleted = FALSE AND employee_id IS NOT NULL;

-- Full-text search
CREATE INDEX idx_users_name_search ON users USING GIN(
    to_tsvector('english', first_name || ' ' || last_name || ' ' || COALESCE(email, ''))
) WHERE is_deleted = FALSE;

-- ============================================================================
-- Role Table
-- ============================================================================

CREATE TABLE role (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    role_code VARCHAR(50) UNIQUE NOT NULL,
    role_name VARCHAR(100) NOT NULL,
    description TEXT,

    -- Organization scope
    organization_id UUID,
    is_system_role BOOLEAN DEFAULT FALSE,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

CREATE INDEX idx_role_org ON role(organization_id) WHERE is_active = TRUE;
CREATE INDEX idx_role_code ON role(role_code) WHERE is_active = TRUE;

-- ============================================================================
-- Permission Table
-- ============================================================================

CREATE TABLE permission (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    permission_code VARCHAR(100) UNIQUE NOT NULL,
    permission_name VARCHAR(200) NOT NULL,
    description TEXT,

    -- Categorization
    module VARCHAR(50) NOT NULL, -- PATIENT, ORDER, RESULT, BILLING, etc.
    action VARCHAR(50) NOT NULL, -- CREATE, READ, UPDATE, DELETE, APPROVE, etc.

    -- System permission (cannot be deleted)
    is_system_permission BOOLEAN DEFAULT TRUE,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_permission_module ON permission(module) WHERE is_active = TRUE;
CREATE INDEX idx_permission_code ON permission(permission_code) WHERE is_active = TRUE;

-- ============================================================================
-- User Role Assignment Table
-- ============================================================================

CREATE TABLE user_role (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES role(id) ON DELETE CASCADE,

    -- Assignment details
    assigned_by UUID,
    assigned_at TIMESTAMPTZ DEFAULT NOW(),

    -- Validity
    valid_from TIMESTAMPTZ DEFAULT NOW(),
    valid_until TIMESTAMPTZ,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Unique constraint
    CONSTRAINT unique_user_role UNIQUE(user_id, role_id)
);

CREATE INDEX idx_user_role_user ON user_role(user_id);
CREATE INDEX idx_user_role_role ON user_role(role_id);

-- ============================================================================
-- Role Permission Assignment Table
-- ============================================================================

CREATE TABLE role_permission (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    role_id UUID NOT NULL REFERENCES role(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permission(id) ON DELETE CASCADE,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Unique constraint
    CONSTRAINT unique_role_permission UNIQUE(role_id, permission_id)
);

CREATE INDEX idx_role_permission_role ON role_permission(role_id);
CREATE INDEX idx_role_permission_permission ON role_permission(permission_id);

-- ============================================================================
-- User Session Table
-- ============================================================================

CREATE TABLE user_session (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_token VARCHAR(255) UNIQUE NOT NULL,

    -- User and device
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id VARCHAR(255),
    device_name VARCHAR(200),
    device_type VARCHAR(50), -- WEB, MOBILE, TABLET

    -- Session details
    ip_address INET,
    user_agent TEXT,
    location VARCHAR(200),

    -- Session status
    session_status session_status NOT NULL DEFAULT 'ACTIVE',

    -- Timing
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    logged_out_at TIMESTAMPTZ,

    -- JWT tokens
    access_token TEXT,
    refresh_token TEXT,
    refresh_token_expires_at TIMESTAMPTZ
);

CREATE INDEX idx_session_token ON user_session(session_token) WHERE session_status = 'ACTIVE';
CREATE INDEX idx_session_user ON user_session(user_id) WHERE session_status = 'ACTIVE';
CREATE INDEX idx_session_status ON user_session(session_status);
CREATE INDEX idx_session_expires ON user_session(expires_at) WHERE session_status = 'ACTIVE';

-- ============================================================================
-- Activity Log Table
-- ============================================================================

CREATE TABLE activity_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Actor
    user_id UUID REFERENCES users(id),
    session_id UUID REFERENCES user_session(id),

    -- Action
    action VARCHAR(100) NOT NULL,
    module VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50),
    entity_id UUID,

    -- Details
    description TEXT,
    metadata JSONB,

    -- Request details
    ip_address INET,
    user_agent TEXT,

    -- Timing
    performed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_activity_user ON activity_log(user_id);
CREATE INDEX idx_activity_date ON activity_log(performed_at DESC);
CREATE INDEX idx_activity_action ON activity_log(action);
CREATE INDEX idx_activity_module ON activity_log(module);

-- ============================================================================
-- API Key Table
-- ============================================================================

CREATE TABLE api_key (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key_hash VARCHAR(255) UNIQUE NOT NULL,
    key_prefix VARCHAR(20) NOT NULL,

    -- Owner
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    organization_id UUID,

    -- Details
    name VARCHAR(200) NOT NULL,
    description TEXT,

    -- Permissions
    scopes JSONB, -- Array of allowed permissions

    -- Rate limiting
    rate_limit INTEGER, -- Requests per hour

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Timing
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    usage_count INTEGER DEFAULT 0,

    -- Metadata
    created_by UUID
);

CREATE INDEX idx_api_key_user ON api_key(user_id) WHERE is_active = TRUE;
CREATE INDEX idx_api_key_org ON api_key(organization_id) WHERE is_active = TRUE;
CREATE INDEX idx_api_key_prefix ON api_key(key_prefix) WHERE is_active = TRUE;

-- ============================================================================
-- Functions
-- ============================================================================

-- Generate User Code
CREATE OR REPLACE FUNCTION generate_user_code(org_code VARCHAR, p_user_type VARCHAR)
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
    checksum INTEGER;
BEGIN
    sequence_num := nextval('user_sequence');

    base_id := org_code || '-' ||
               CASE
                   WHEN p_user_type = 'DOCTOR' THEN 'DOC'
                   WHEN p_user_type = 'TECHNICIAN' THEN 'TCH'
                   WHEN p_user_type = 'NURSE' THEN 'NRS'
                   ELSE 'USR'
               END || '-' ||
               LPAD(sequence_num::TEXT, 6, '0');

    checksum := (sequence_num % 10);

    RETURN base_id || checksum::TEXT;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS user_sequence START 1;

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_role_updated_at
    BEFORE UPDATE ON role
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Log user activities
CREATE OR REPLACE FUNCTION log_user_activity()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        -- Log status changes
        IF OLD.user_status IS DISTINCT FROM NEW.user_status THEN
            INSERT INTO activity_log (user_id, action, module, entity_type, entity_id, description)
            VALUES (NEW.id, 'STATUS_CHANGED', 'USER', 'USER', NEW.id,
                    'Status changed from ' || OLD.user_status || ' to ' || NEW.user_status);
        END IF;

        -- Log password changes
        IF OLD.password_hash IS DISTINCT FROM NEW.password_hash THEN
            INSERT INTO activity_log (user_id, action, module, entity_type, entity_id, description)
            VALUES (NEW.id, 'PASSWORD_CHANGED', 'USER', 'USER', NEW.id, 'Password changed');
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER log_user_changes
    AFTER UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION log_user_activity();

-- ============================================================================
-- Sample Data - System Permissions
-- ============================================================================

INSERT INTO permission (id, permission_code, permission_name, module, action) VALUES
-- Patient permissions
(uuid_generate_v4(), 'PATIENT_CREATE', 'Create Patient', 'PATIENT', 'CREATE'),
(uuid_generate_v4(), 'PATIENT_READ', 'View Patient', 'PATIENT', 'READ'),
(uuid_generate_v4(), 'PATIENT_UPDATE', 'Update Patient', 'PATIENT', 'UPDATE'),
(uuid_generate_v4(), 'PATIENT_DELETE', 'Delete Patient', 'PATIENT', 'DELETE'),

-- Order permissions
(uuid_generate_v4(), 'ORDER_CREATE', 'Create Order', 'ORDER', 'CREATE'),
(uuid_generate_v4(), 'ORDER_READ', 'View Order', 'ORDER', 'READ'),
(uuid_generate_v4(), 'ORDER_UPDATE', 'Update Order', 'ORDER', 'UPDATE'),
(uuid_generate_v4(), 'ORDER_DELETE', 'Delete Order', 'ORDER', 'DELETE'),
(uuid_generate_v4(), 'ORDER_CONFIRM', 'Confirm Order', 'ORDER', 'CONFIRM'),
(uuid_generate_v4(), 'ORDER_CANCEL', 'Cancel Order', 'ORDER', 'CANCEL'),

-- Sample permissions
(uuid_generate_v4(), 'SAMPLE_CREATE', 'Create Sample', 'SAMPLE', 'CREATE'),
(uuid_generate_v4(), 'SAMPLE_READ', 'View Sample', 'SAMPLE', 'READ'),
(uuid_generate_v4(), 'SAMPLE_RECEIVE', 'Receive Sample', 'SAMPLE', 'RECEIVE'),
(uuid_generate_v4(), 'SAMPLE_REJECT', 'Reject Sample', 'SAMPLE', 'REJECT'),

-- Result permissions
(uuid_generate_v4(), 'RESULT_CREATE', 'Create Result', 'RESULT', 'CREATE'),
(uuid_generate_v4(), 'RESULT_READ', 'View Result', 'RESULT', 'READ'),
(uuid_generate_v4(), 'RESULT_UPDATE', 'Update Result', 'RESULT', 'UPDATE'),
(uuid_generate_v4(), 'RESULT_VERIFY', 'Verify Result', 'RESULT', 'VERIFY'),
(uuid_generate_v4(), 'RESULT_APPROVE', 'Approve Result', 'RESULT', 'APPROVE'),
(uuid_generate_v4(), 'RESULT_CORRECT', 'Correct Result', 'RESULT', 'CORRECT'),

-- Report permissions
(uuid_generate_v4(), 'REPORT_READ', 'View Report', 'REPORT', 'READ'),
(uuid_generate_v4(), 'REPORT_GENERATE', 'Generate Report', 'REPORT', 'GENERATE'),
(uuid_generate_v4(), 'REPORT_DOWNLOAD', 'Download Report', 'REPORT', 'DOWNLOAD'),

-- Billing permissions
(uuid_generate_v4(), 'BILLING_CREATE', 'Create Bill', 'BILLING', 'CREATE'),
(uuid_generate_v4(), 'BILLING_READ', 'View Bill', 'BILLING', 'READ'),
(uuid_generate_v4(), 'BILLING_UPDATE', 'Update Bill', 'BILLING', 'UPDATE'),

-- Admin permissions
(uuid_generate_v4(), 'USER_MANAGE', 'Manage Users', 'ADMIN', 'MANAGE'),
(uuid_generate_v4(), 'ROLE_MANAGE', 'Manage Roles', 'ADMIN', 'MANAGE'),
(uuid_generate_v4(), 'SETTINGS_MANAGE', 'Manage Settings', 'ADMIN', 'MANAGE');

-- Sample Roles
INSERT INTO role (id, role_code, role_name, description, is_system_role) VALUES
(uuid_generate_v4(), 'SUPER_ADMIN', 'Super Administrator', 'Full system access', TRUE),
(uuid_generate_v4(), 'LAB_MANAGER', 'Laboratory Manager', 'Lab operations management', TRUE),
(uuid_generate_v4(), 'PATHOLOGIST', 'Pathologist', 'Result verification and approval', TRUE),
(uuid_generate_v4(), 'TECHNICIAN', 'Laboratory Technician', 'Sample and result entry', TRUE),
(uuid_generate_v4(), 'RECEPTIONIST', 'Receptionist', 'Patient and order management', TRUE);
