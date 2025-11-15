-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Custom Types
-- ============================================================================

CREATE TYPE result_status AS ENUM (
    'PENDING',
    'IN_PROGRESS',
    'PRELIMINARY',
    'FINAL',
    'CORRECTED',
    'CANCELLED',
    'AMENDED'
);

CREATE TYPE verification_status AS ENUM (
    'NOT_VERIFIED',
    'AUTO_VERIFIED',
    'MANUALLY_VERIFIED',
    'VERIFICATION_FAILED',
    'PENDING_REVIEW'
);

CREATE TYPE critical_flag AS ENUM (
    'NONE',
    'LOW',
    'HIGH',
    'PANIC_LOW',
    'PANIC_HIGH'
);

CREATE TYPE delta_flag AS ENUM (
    'NORMAL',
    'SIGNIFICANT_INCREASE',
    'SIGNIFICANT_DECREASE',
    'NO_PREVIOUS_RESULT'
);

CREATE TYPE interpretation AS ENUM (
    'NORMAL',
    'ABNORMAL_LOW',
    'ABNORMAL_HIGH',
    'CRITICAL_LOW',
    'CRITICAL_HIGH',
    'INDETERMINATE'
);

-- ============================================================================
-- Test Result Table
-- ============================================================================

CREATE TABLE test_result (
    -- Identity
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    result_number VARCHAR(50) UNIQUE NOT NULL,

    -- Linkage
    patient_id UUID NOT NULL,
    order_id UUID NOT NULL,
    order_item_id UUID NOT NULL,
    test_id UUID NOT NULL,
    sample_id UUID NOT NULL,
    organization_id UUID NOT NULL,

    -- Test Information
    test_code VARCHAR(50) NOT NULL,
    test_name VARCHAR(300) NOT NULL,
    department VARCHAR(100),

    -- Result Values
    result_value TEXT,
    result_unit VARCHAR(50),
    result_type VARCHAR(50) NOT NULL, -- NUMERIC, TEXT, CODED, etc.

    -- Reference Ranges
    reference_range_text VARCHAR(500),
    reference_range_min DECIMAL(15, 4),
    reference_range_max DECIMAL(15, 4),

    -- Interpretation
    interpretation interpretation DEFAULT 'NORMAL',
    clinical_interpretation TEXT,

    -- Flags
    critical_flag critical_flag DEFAULT 'NONE',
    delta_flag delta_flag DEFAULT 'NORMAL',
    is_abnormal BOOLEAN DEFAULT FALSE,
    is_critical BOOLEAN DEFAULT FALSE,

    -- Delta Check
    previous_result_value TEXT,
    previous_result_date TIMESTAMP,
    delta_percentage DECIMAL(10, 2),
    delta_absolute DECIMAL(15, 4),

    -- Status and Workflow
    result_status result_status NOT NULL DEFAULT 'PENDING',
    verification_status verification_status NOT NULL DEFAULT 'NOT_VERIFIED',

    -- Entry Information
    entry_method VARCHAR(50), -- MANUAL, INSTRUMENT_INTERFACE, AUTO_CALCULATED
    entered_by UUID,
    entry_date TIMESTAMP DEFAULT NOW(),

    -- Verification
    verified_by UUID,
    verification_date TIMESTAMP,
    auto_verification_confidence DECIMAL(5, 2),
    verification_rules_passed JSONB, -- Array of rule IDs that passed
    verification_rules_failed JSONB, -- Array of rule IDs that failed

    -- Approval
    approved_by UUID,
    approval_date TIMESTAMP,
    approval_notes TEXT,

    -- Instrument Information
    instrument_id UUID,
    instrument_name VARCHAR(200),
    run_number VARCHAR(100),

    -- Quality Control
    qc_lot_number VARCHAR(100),
    qc_passed BOOLEAN,
    qc_notes TEXT,

    -- Timing
    result_date TIMESTAMP NOT NULL DEFAULT NOW(),
    reported_date TIMESTAMP,
    tat_hours DECIMAL(10, 2),

    -- Additional Information
    method_used VARCHAR(200),
    reagent_lot VARCHAR(100),
    dilution_factor DECIMAL(10, 4),
    specimen_condition VARCHAR(100),

    -- Comments and Notes
    technician_notes TEXT,
    pathologist_notes TEXT,
    internal_notes TEXT,

    -- Correction/Amendment
    is_corrected BOOLEAN DEFAULT FALSE,
    corrected_from_result_id UUID REFERENCES test_result(id),
    correction_reason TEXT,
    correction_date TIMESTAMP,
    corrected_by UUID,

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
CREATE INDEX idx_test_result_patient ON test_result(patient_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_test_result_order ON test_result(order_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_test_result_sample ON test_result(sample_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_test_result_test ON test_result(test_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_test_result_status ON test_result(result_status) WHERE is_deleted = FALSE;
CREATE INDEX idx_test_result_verification ON test_result(verification_status) WHERE is_deleted = FALSE;
CREATE INDEX idx_test_result_critical ON test_result(is_critical) WHERE is_critical = TRUE AND is_deleted = FALSE;
CREATE INDEX idx_test_result_date ON test_result(result_date DESC) WHERE is_deleted = FALSE;
CREATE INDEX idx_test_result_org ON test_result(organization_id) WHERE is_deleted = FALSE;

-- ============================================================================
-- Reference Range Table
-- ============================================================================

CREATE TABLE reference_range (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    test_id UUID NOT NULL,
    organization_id UUID,

    -- Range Criteria
    age_min INTEGER, -- in years
    age_max INTEGER,
    gender VARCHAR(20), -- MALE, FEMALE, OTHER, ALL

    -- Range Values
    range_min DECIMAL(15, 4),
    range_max DECIMAL(15, 4),
    range_text VARCHAR(500),

    -- Critical Values
    panic_low DECIMAL(15, 4),
    panic_high DECIMAL(15, 4),
    critical_low DECIMAL(15, 4),
    critical_high DECIMAL(15, 4),

    -- Unit
    unit VARCHAR(50),

    -- Validity
    is_active BOOLEAN DEFAULT TRUE,
    effective_from DATE,
    effective_to DATE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

CREATE INDEX idx_reference_range_test ON reference_range(test_id) WHERE is_active = TRUE;
CREATE INDEX idx_reference_range_org ON reference_range(organization_id) WHERE is_active = TRUE;

-- ============================================================================
-- Auto-Verification Rule Table
-- ============================================================================

CREATE TABLE auto_verification_rule (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    rule_code VARCHAR(50) UNIQUE NOT NULL,
    rule_name VARCHAR(200) NOT NULL,

    -- Applicability
    test_id UUID,
    department VARCHAR(100),
    organization_id UUID,
    is_global BOOLEAN DEFAULT FALSE,

    -- Rule Configuration
    rule_type VARCHAR(50) NOT NULL, -- RANGE_CHECK, DELTA_CHECK, QC_CHECK, DUPLICATE_CHECK, etc.
    rule_definition JSONB NOT NULL, -- JSON configuration for the rule

    -- Thresholds
    min_value DECIMAL(15, 4),
    max_value DECIMAL(15, 4),
    delta_percentage_limit DECIMAL(10, 2),
    delta_absolute_limit DECIMAL(15, 4),

    -- Priority and Weight
    priority INTEGER DEFAULT 100,
    is_blocking BOOLEAN DEFAULT FALSE, -- If true, failure blocks auto-verification

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

CREATE INDEX idx_auto_verification_test ON auto_verification_rule(test_id) WHERE is_active = TRUE;
CREATE INDEX idx_auto_verification_dept ON auto_verification_rule(department) WHERE is_active = TRUE;
CREATE INDEX idx_auto_verification_type ON auto_verification_rule(rule_type) WHERE is_active = TRUE;

-- ============================================================================
-- Result Comment Templates Table
-- ============================================================================

CREATE TABLE result_comment_template (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    template_code VARCHAR(50) UNIQUE NOT NULL,
    template_name VARCHAR(200) NOT NULL,

    -- Applicability
    test_id UUID,
    department VARCHAR(100),

    -- Content
    comment_text TEXT NOT NULL,
    interpretation_type interpretation,

    -- Usage
    is_active BOOLEAN DEFAULT TRUE,
    usage_count INTEGER DEFAULT 0,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

CREATE INDEX idx_result_comment_test ON result_comment_template(test_id) WHERE is_active = TRUE;
CREATE INDEX idx_result_comment_dept ON result_comment_template(department) WHERE is_active = TRUE;

-- ============================================================================
-- Result Audit Log Table
-- ============================================================================

CREATE TABLE result_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    result_id UUID NOT NULL REFERENCES test_result(id),

    -- Change Information
    action VARCHAR(50) NOT NULL, -- CREATED, UPDATED, VERIFIED, APPROVED, CORRECTED, CANCELLED
    field_changed VARCHAR(100),
    old_value TEXT,
    new_value TEXT,

    -- Actor Information
    performed_by UUID NOT NULL,
    performed_at TIMESTAMP DEFAULT NOW(),

    -- Context
    reason TEXT,
    ip_address INET,
    user_agent TEXT,

    -- Metadata
    metadata JSONB
);

CREATE INDEX idx_result_audit_result ON result_audit_log(result_id);
CREATE INDEX idx_result_audit_date ON result_audit_log(performed_at DESC);
CREATE INDEX idx_result_audit_action ON result_audit_log(action);

-- ============================================================================
-- Critical Result Notification Table
-- ============================================================================

CREATE TABLE critical_result_notification (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    result_id UUID NOT NULL REFERENCES test_result(id),

    -- Notification Details
    notified_to VARCHAR(200) NOT NULL, -- Doctor, Nurse, Patient, etc.
    notification_method VARCHAR(50) NOT NULL, -- PHONE, SMS, EMAIL, WHATSAPP, IN_PERSON
    notification_date TIMESTAMP NOT NULL DEFAULT NOW(),

    -- Acknowledgment
    acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_by VARCHAR(200),
    acknowledgment_date TIMESTAMP,
    acknowledgment_method VARCHAR(50),

    -- Documentation
    caller_name VARCHAR(200),
    call_back_number VARCHAR(50),
    notes TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    created_by UUID NOT NULL
);

CREATE INDEX idx_critical_notification_result ON critical_result_notification(result_id);
CREATE INDEX idx_critical_notification_date ON critical_result_notification(notification_date DESC);
CREATE INDEX idx_critical_notification_ack ON critical_result_notification(acknowledged) WHERE acknowledged = FALSE;

-- ============================================================================
-- Functions
-- ============================================================================

-- Generate Result Number
CREATE OR REPLACE FUNCTION generate_result_number(org_code VARCHAR, test_code VARCHAR)
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
    checksum INTEGER;
BEGIN
    sequence_num := nextval('result_sequence');

    base_id := org_code || '-RES-' || test_code || '-' ||
               TO_CHAR(NOW(), 'YYYYMMDD') || '-' ||
               LPAD(sequence_num::TEXT, 6, '0');

    -- Calculate Luhn checksum (simplified)
    checksum := (sequence_num % 10);

    RETURN base_id || checksum::TEXT;
END;
$$ LANGUAGE plpgsql;

-- Create sequence for result numbers
CREATE SEQUENCE IF NOT EXISTS result_sequence START 1;

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_test_result_updated_at
    BEFORE UPDATE ON test_result
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_reference_range_updated_at
    BEFORE UPDATE ON reference_range
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_auto_verification_rule_updated_at
    BEFORE UPDATE ON auto_verification_rule
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Log result changes
CREATE OR REPLACE FUNCTION log_result_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        -- Log status changes
        IF OLD.result_status IS DISTINCT FROM NEW.result_status THEN
            INSERT INTO result_audit_log (result_id, action, field_changed, old_value, new_value, performed_by)
            VALUES (NEW.id, 'UPDATED', 'result_status', OLD.result_status::TEXT, NEW.result_status::TEXT, NEW.updated_by);
        END IF;

        -- Log verification changes
        IF OLD.verification_status IS DISTINCT FROM NEW.verification_status THEN
            INSERT INTO result_audit_log (result_id, action, field_changed, old_value, new_value, performed_by)
            VALUES (NEW.id, 'VERIFIED', 'verification_status', OLD.verification_status::TEXT, NEW.verification_status::TEXT, NEW.verified_by);
        END IF;

        -- Log result value changes
        IF OLD.result_value IS DISTINCT FROM NEW.result_value THEN
            INSERT INTO result_audit_log (result_id, action, field_changed, old_value, new_value, performed_by)
            VALUES (NEW.id, 'UPDATED', 'result_value', OLD.result_value, NEW.result_value, NEW.updated_by);
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER log_test_result_changes
    AFTER UPDATE ON test_result
    FOR EACH ROW
    EXECUTE FUNCTION log_result_changes();

-- ============================================================================
-- Sample Data
-- ============================================================================

-- Sample reference ranges for common tests
INSERT INTO reference_range (id, test_id, gender, age_min, age_max, range_min, range_max, unit, critical_low, critical_high, panic_low, panic_high) VALUES
(uuid_generate_v4(), uuid_nil(), 'ALL', 0, 150, 13.0, 17.0, 'g/dL', 7.0, 20.0, 5.0, 22.0),  -- Hemoglobin
(uuid_generate_v4(), uuid_nil(), 'ALL', 0, 150, 70.0, 110.0, 'mg/dL', 40.0, 400.0, 30.0, 600.0),  -- Blood Sugar
(uuid_generate_v4(), uuid_nil(), 'ALL', 0, 150, 3.5, 5.5, 'mEq/L', 2.5, 6.5, 2.0, 7.0);  -- Potassium

-- Sample auto-verification rules
INSERT INTO auto_verification_rule (id, rule_code, rule_name, rule_type, rule_definition, is_global, is_blocking) VALUES
(uuid_generate_v4(), 'RANGE_CHECK', 'Within Reference Range', 'RANGE_CHECK', '{"check_type": "within_range"}'::JSONB, TRUE, TRUE),
(uuid_generate_v4(), 'DELTA_CHECK', 'Delta Check 50%', 'DELTA_CHECK', '{"max_delta_percent": 50}'::JSONB, TRUE, FALSE),
(uuid_generate_v4(), 'QC_PASS', 'QC Must Pass', 'QC_CHECK', '{"require_qc_pass": true}'::JSONB, TRUE, TRUE),
(uuid_generate_v4(), 'NO_CRITICAL', 'No Critical Values', 'CRITICAL_CHECK', '{"block_if_critical": true}'::JSONB, TRUE, TRUE);

-- Sample comment templates
INSERT INTO result_comment_template (id, template_code, template_name, comment_text, interpretation_type) VALUES
(uuid_generate_v4(), 'HIGH_GLUCOSE', 'High Glucose Comment', 'Elevated glucose levels detected. Please correlate with clinical findings and consider diabetes screening.', 'ABNORMAL_HIGH'),
(uuid_generate_v4(), 'LOW_HB', 'Low Hemoglobin Comment', 'Hemoglobin below normal range. Possible anemia. Further evaluation recommended.', 'ABNORMAL_LOW'),
(uuid_generate_v4(), 'CRITICAL_K', 'Critical Potassium', 'CRITICAL: Potassium level outside safe range. Immediate clinical intervention required.', 'CRITICAL_HIGH');
