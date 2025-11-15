-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Custom Types
-- ============================================================================

CREATE TYPE organization_status AS ENUM (
    'ACTIVE',
    'INACTIVE',
    'SUSPENDED',
    'TRIAL',
    'EXPIRED'
);

CREATE TYPE organization_type AS ENUM (
    'SINGLE_LAB',
    'MULTI_BRANCH',
    'HOSPITAL_LAB',
    'DIAGNOSTIC_CENTER',
    'REFERENCE_LAB',
    'COLLECTION_CENTER'
);

CREATE TYPE subscription_plan AS ENUM (
    'FREE',
    'BASIC',
    'PROFESSIONAL',
    'ENTERPRISE',
    'CUSTOM'
);

CREATE TYPE accreditation_type AS ENUM (
    'NABL',
    'CAP',
    'ISO_15189',
    'ISO_9001',
    'JCI',
    'NABH'
);

-- ============================================================================
-- Organization Table
-- ============================================================================

CREATE TABLE organization (
    -- Identity
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_code VARCHAR(50) UNIQUE NOT NULL,

    -- Basic Information
    organization_name VARCHAR(300) NOT NULL,
    legal_name VARCHAR(300),
    short_name VARCHAR(100),
    organization_type organization_type NOT NULL,

    -- Status
    organization_status organization_status NOT NULL DEFAULT 'TRIAL',

    -- Registration Details
    registration_number VARCHAR(100),
    pan_number VARCHAR(20),
    gstin VARCHAR(20),
    cin VARCHAR(30),

    -- Contact Information
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    fax VARCHAR(20),
    website VARCHAR(200),

    -- Primary Address
    address_line1 VARCHAR(500),
    address_line2 VARCHAR(500),
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100) DEFAULT 'India',
    postal_code VARCHAR(20),

    -- Geographic Coordinates
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),

    -- Subscription and Licensing
    subscription_plan subscription_plan DEFAULT 'FREE',
    subscription_start_date DATE,
    subscription_end_date DATE,
    max_users INTEGER DEFAULT 5,
    max_branches INTEGER DEFAULT 1,
    max_tests_per_month INTEGER DEFAULT 1000,
    current_month_tests INTEGER DEFAULT 0,

    -- Branding
    logo_url VARCHAR(500),
    primary_color VARCHAR(7), -- Hex color
    secondary_color VARCHAR(7),
    header_image_url VARCHAR(500),
    footer_text TEXT,

    -- Settings (JSONB for flexibility)
    settings JSONB,

    -- Features enabled
    features_enabled JSONB, -- Array of feature flags

    -- Parent Organization (for multi-branch)
    parent_organization_id UUID REFERENCES organization(id),

    -- Contact Person
    contact_person_name VARCHAR(200),
    contact_person_email VARCHAR(255),
    contact_person_phone VARCHAR(20),

    -- Business Hours
    business_hours JSONB, -- {"monday": {"open": "09:00", "close": "18:00"}, ...}

    -- Timezone
    timezone VARCHAR(50) DEFAULT 'Asia/Kolkata',

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    deleted_by UUID
);

-- Indexes
CREATE INDEX idx_organization_code ON organization(org_code) WHERE is_deleted = FALSE;
CREATE INDEX idx_organization_type ON organization(organization_type) WHERE is_deleted = FALSE;
CREATE INDEX idx_organization_status ON organization(organization_status) WHERE is_deleted = FALSE;
CREATE INDEX idx_organization_parent ON organization(parent_organization_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_organization_subscription ON organization(subscription_plan, subscription_end_date) WHERE is_deleted = FALSE;

-- Full-text search
CREATE INDEX idx_organization_name_search ON organization USING GIN(
    to_tsvector('english', organization_name || ' ' || COALESCE(legal_name, '') || ' ' || COALESCE(email, ''))
) WHERE is_deleted = FALSE;

-- ============================================================================
-- Organization Branch Table
-- ============================================================================

CREATE TABLE organization_branch (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,

    -- Branch Information
    branch_code VARCHAR(50) NOT NULL,
    branch_name VARCHAR(200) NOT NULL,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    is_main_branch BOOLEAN DEFAULT FALSE,

    -- Contact Information
    email VARCHAR(255),
    phone VARCHAR(20),

    -- Address
    address_line1 VARCHAR(500),
    address_line2 VARCHAR(500),
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100) DEFAULT 'India',
    postal_code VARCHAR(20),

    -- Geographic Coordinates
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),

    -- Branch Manager
    manager_id UUID,
    manager_name VARCHAR(200),
    manager_email VARCHAR(255),
    manager_phone VARCHAR(20),

    -- Capacity
    sample_processing_capacity INTEGER, -- Per day

    -- Business Hours
    business_hours JSONB,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,

    -- Constraints
    CONSTRAINT unique_org_branch_code UNIQUE(organization_id, branch_code)
);

CREATE INDEX idx_branch_org ON organization_branch(organization_id) WHERE is_active = TRUE;
CREATE INDEX idx_branch_code ON organization_branch(branch_code);

-- ============================================================================
-- Accreditation Table
-- ============================================================================

CREATE TABLE accreditation (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,

    -- Accreditation Details
    accreditation_type accreditation_type NOT NULL,
    accreditation_number VARCHAR(100) NOT NULL,

    -- Issuing Authority
    issuing_authority VARCHAR(200),

    -- Validity
    issue_date DATE NOT NULL,
    expiry_date DATE NOT NULL,

    -- Scope
    scope_of_accreditation TEXT,
    accredited_tests JSONB, -- Array of test IDs or codes

    -- Document
    certificate_url VARCHAR(500),

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

CREATE INDEX idx_accreditation_org ON accreditation(organization_id) WHERE is_active = TRUE;
CREATE INDEX idx_accreditation_type ON accreditation(accreditation_type, expiry_date) WHERE is_active = TRUE;

-- ============================================================================
-- Organization Settings Table
-- ============================================================================

CREATE TABLE organization_setting (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,

    -- Setting Details
    setting_category VARCHAR(50) NOT NULL, -- BILLING, REPORTING, NOTIFICATIONS, etc.
    setting_key VARCHAR(100) NOT NULL,
    setting_value TEXT NOT NULL,
    setting_type VARCHAR(20) DEFAULT 'STRING', -- STRING, NUMBER, BOOLEAN, JSON

    -- Description
    description TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    updated_by UUID,

    -- Constraints
    CONSTRAINT unique_org_setting UNIQUE(organization_id, setting_category, setting_key)
);

CREATE INDEX idx_org_setting ON organization_setting(organization_id, setting_category);

-- ============================================================================
-- Department Table
-- ============================================================================

CREATE TABLE department (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    branch_id UUID REFERENCES organization_branch(id),

    -- Department Information
    department_code VARCHAR(50) NOT NULL,
    department_name VARCHAR(200) NOT NULL,
    description TEXT,

    -- Head of Department
    hod_user_id UUID,
    hod_name VARCHAR(200),

    -- Contact
    email VARCHAR(255),
    phone VARCHAR(20),

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,

    -- Constraints
    CONSTRAINT unique_dept_code UNIQUE(organization_id, department_code)
);

CREATE INDEX idx_department_org ON department(organization_id) WHERE is_active = TRUE;
CREATE INDEX idx_department_branch ON department(branch_id) WHERE is_active = TRUE;

-- ============================================================================
-- Working Hours Template Table
-- ============================================================================

CREATE TABLE working_hours_template (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,

    -- Template Information
    template_name VARCHAR(100) NOT NULL,
    description TEXT,

    -- Schedule (JSONB for flexibility)
    schedule JSONB NOT NULL,
    -- Example: {
    --   "monday": {"open": "09:00", "close": "18:00", "breaks": [{"start": "13:00", "end": "14:00"}]},
    --   "tuesday": {"open": "09:00", "close": "18:00"},
    --   ...
    -- }

    -- Holidays
    public_holidays JSONB, -- Array of dates

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_working_hours_org ON working_hours_template(organization_id) WHERE is_active = TRUE;

-- ============================================================================
-- Organization Audit Log Table
-- ============================================================================

CREATE TABLE organization_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,

    -- Action Details
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID,

    -- Changes
    old_value JSONB,
    new_value JSONB,

    -- Actor
    performed_by UUID,
    performed_at TIMESTAMP DEFAULT NOW(),

    -- Context
    ip_address INET,
    user_agent TEXT,
    description TEXT
);

CREATE INDEX idx_org_audit_org ON organization_audit_log(organization_id);
CREATE INDEX idx_org_audit_date ON organization_audit_log(performed_at DESC);

-- ============================================================================
-- Functions
-- ============================================================================

-- Generate Organization Code
CREATE OR REPLACE FUNCTION generate_org_code()
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
    checksum INTEGER;
BEGIN
    sequence_num := nextval('org_sequence');

    base_id := 'ORG-' || LPAD(sequence_num::TEXT, 6, '0');

    checksum := (sequence_num % 10);

    RETURN base_id || checksum::TEXT;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS org_sequence START 1;

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_organization_updated_at
    BEFORE UPDATE ON organization
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_branch_updated_at
    BEFORE UPDATE ON organization_branch
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_department_updated_at
    BEFORE UPDATE ON department
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Log organization changes
CREATE OR REPLACE FUNCTION log_organization_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        -- Log status changes
        IF OLD.organization_status IS DISTINCT FROM NEW.organization_status THEN
            INSERT INTO organization_audit_log (organization_id, action, entity_type, entity_id, old_value, new_value)
            VALUES (NEW.id, 'STATUS_CHANGED', 'ORGANIZATION', NEW.id,
                    jsonb_build_object('status', OLD.organization_status::TEXT),
                    jsonb_build_object('status', NEW.organization_status::TEXT));
        END IF;

        -- Log subscription changes
        IF OLD.subscription_plan IS DISTINCT FROM NEW.subscription_plan
           OR OLD.subscription_end_date IS DISTINCT FROM NEW.subscription_end_date THEN
            INSERT INTO organization_audit_log (organization_id, action, entity_type, entity_id, description)
            VALUES (NEW.id, 'SUBSCRIPTION_CHANGED', 'ORGANIZATION', NEW.id,
                    'Subscription changed from ' || OLD.subscription_plan::TEXT || ' to ' || NEW.subscription_plan::TEXT);
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER log_organization_audit
    AFTER UPDATE ON organization
    FOR EACH ROW
    EXECUTE FUNCTION log_organization_changes();

-- Reset monthly test counter
CREATE OR REPLACE FUNCTION reset_monthly_test_counter()
RETURNS void AS $$
BEGIN
    UPDATE organization
    SET current_month_tests = 0
    WHERE is_deleted = FALSE;
END;
$$ LANGUAGE plpgsql;

-- Check subscription validity
CREATE OR REPLACE FUNCTION check_subscription_validity()
RETURNS void AS $$
BEGIN
    UPDATE organization
    SET organization_status = 'EXPIRED'
    WHERE subscription_end_date < CURRENT_DATE
      AND organization_status NOT IN ('SUSPENDED', 'INACTIVE')
      AND is_deleted = FALSE;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Sample Data
-- ============================================================================

-- Sample Organization
INSERT INTO organization (
    id, org_code, organization_name, legal_name, organization_type,
    organization_status, email, phone, city, state, country,
    subscription_plan, subscription_start_date, subscription_end_date,
    max_users, max_branches, timezone
) VALUES (
    uuid_generate_v4(),
    'ORG-0000011',
    'Apollo Diagnostics',
    'Apollo Diagnostics Private Limited',
    'DIAGNOSTIC_CENTER',
    'ACTIVE',
    'info@apollodiagnostics.com',
    '+91-9876543210',
    'Mumbai',
    'Maharashtra',
    'India',
    'PROFESSIONAL',
    CURRENT_DATE,
    CURRENT_DATE + INTERVAL '1 year',
    50,
    10,
    'Asia/Kolkata'
);

-- Sample settings categories
INSERT INTO organization_setting (id, organization_id, setting_category, setting_key, setting_value, setting_type, description)
SELECT
    uuid_generate_v4(),
    org.id,
    'REPORTING',
    'default_report_format',
    'PDF',
    'STRING',
    'Default format for test reports'
FROM organization org
WHERE org.org_code = 'ORG-0000011'
LIMIT 1;

INSERT INTO organization_setting (id, organization_id, setting_category, setting_key, setting_value, setting_type, description)
SELECT
    uuid_generate_v4(),
    org.id,
    'BILLING',
    'auto_invoice_generation',
    'true',
    'BOOLEAN',
    'Automatically generate invoices on order confirmation'
FROM organization org
WHERE org.org_code = 'ORG-0000011'
LIMIT 1;

-- Sample departments
INSERT INTO department (id, organization_id, department_code, department_name, is_active)
SELECT uuid_generate_v4(), org.id, 'BIOCHEM', 'Biochemistry', TRUE
FROM organization org WHERE org.org_code = 'ORG-0000011' LIMIT 1;

INSERT INTO department (id, organization_id, department_code, department_name, is_active)
SELECT uuid_generate_v4(), org.id, 'HEMATOLOGY', 'Hematology', TRUE
FROM organization org WHERE org.org_code = 'ORG-0000011' LIMIT 1;

INSERT INTO department (id, organization_id, department_code, department_name, is_active)
SELECT uuid_generate_v4(), org.id, 'MICROBIOLOGY', 'Microbiology', TRUE
FROM organization org WHERE org.org_code = 'ORG-0000011' LIMIT 1;

INSERT INTO department (id, organization_id, department_code, department_name, is_active)
SELECT uuid_generate_v4(), org.id, 'PATHOLOGY', 'Pathology', TRUE
FROM organization org WHERE org.org_code = 'ORG-0000011' LIMIT 1;
