-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Organization Stub Table (for foreign key reference)
-- ============================================================================

CREATE TABLE IF NOT EXISTS organization (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Insert default organization for testing
INSERT INTO organization (id, name, code)
VALUES ('00000000-0000-0000-0000-000000000001', 'Default Lab', 'DEFAULT')
ON CONFLICT DO NOTHING;

-- ============================================================================
-- Custom Types
-- ============================================================================

CREATE TYPE qc AS ENUM (
    'IQC',  -- Internal Quality Control
    'EQC',  -- External Quality Control
    'PT'    -- Proficiency Testing
);

CREATE TYPE qc_material_status AS ENUM (
    'ACTIVE',
    'EXPIRED',
    'LOW_STOCK',
    'OUT_OF_STOCK',
    'DISCONTINUED'
);

CREATE TYPE qc_result_status AS ENUM (
    'IN_CONTROL',
    'OUT_OF_CONTROL',
    'WARNING',
    'PENDING'
);

CREATE TYPE qc_rule_enum AS ENUM (
    'WESTGARD_12S',    -- 1 control exceeds 2SD
    'WESTGARD_13S',    -- 1 control exceeds 3SD
    'WESTGARD_22S',    -- 2 consecutive controls exceed 2SD (same side)
    'WESTGARD_R4S',    -- Range between 2 controls exceeds 4SD
    'WESTGARD_41S',    -- 4 consecutive controls exceed 1SD (same side)
    'WESTGARD_10X',    -- 10 consecutive controls on same side of mean
    'CUSTOM'
);

CREATE TYPE violation_severity AS ENUM (
    'LOW',
    'MEDIUM',
    'HIGH',
    'CRITICAL'
);

CREATE TYPE corrective_action_status AS ENUM (
    'PENDING',
    'IN_PROGRESS',
    'COMPLETED',
    'VERIFIED'
);

-- ============================================================================
-- QC Material Table
-- ============================================================================

CREATE TABLE qc_material (
    -- Identity
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    material_code VARCHAR(50) UNIQUE NOT NULL,

    -- Basic Information
    material_name VARCHAR(200) NOT NULL,
    manufacturer VARCHAR(200),
    lot_number VARCHAR(100) NOT NULL,
    catalog_number VARCHAR(100),

    -- QC Type
    qc_type qc NOT NULL,

    -- Organization
    organization_id UUID NOT NULL,

    -- Test Information
    test_id UUID NOT NULL, -- Reference to test catalog
    test_name VARCHAR(200),

    -- Levels
    level_number INTEGER, -- Level 1, 2, 3 (Normal, Abnormal Low, Abnormal High)
    level_name VARCHAR(50), -- e.g., "Normal", "Level 1", "Pathological"

    -- Target Values
    target_mean DECIMAL(15, 4),
    target_sd DECIMAL(15, 4),

    -- Control Limits
    mean_value DECIMAL(15, 4), -- Calculated from lab data
    sd_value DECIMAL(15, 4),
    cv_value DECIMAL(6, 2), -- Coefficient of Variation %

    sd_1_low DECIMAL(15, 4),
    sd_1_high DECIMAL(15, 4),
    sd_2_low DECIMAL(15, 4),
    sd_2_high DECIMAL(15, 4),
    sd_3_low DECIMAL(15, 4),
    sd_3_high DECIMAL(15, 4),

    -- Stock Management
    quantity_in_stock INTEGER DEFAULT 0,
    minimum_stock_level INTEGER DEFAULT 10,
    unit_of_measure VARCHAR(20), -- vials, tests, ml

    -- Validity
    manufacture_date DATE,
    expiry_date DATE NOT NULL,
    opened_date DATE,
    days_stable_after_opening INTEGER, -- e.g., 30 days

    -- Storage
    storage_location VARCHAR(200),
    storage_temperature VARCHAR(50), -- e.g., "2-8°C", "-20°C"

    -- Status
    material_status qc_material_status NOT NULL DEFAULT 'ACTIVE',

    -- Equipment Assignment
    equipment_id UUID,

    -- Documentation
    insert_url VARCHAR(500), -- Package insert
    msds_url VARCHAR(500),   -- Material Safety Data Sheet

    -- Notes
    notes TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    is_deleted BOOLEAN DEFAULT FALSE
);

-- Indexes
CREATE INDEX idx_qc_material_code ON qc_material(material_code) WHERE is_deleted = FALSE;
CREATE INDEX idx_qc_material_org ON qc_material(organization_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_qc_material_test ON qc_material(test_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_qc_material_status ON qc_material(material_status) WHERE is_deleted = FALSE;
CREATE INDEX idx_qc_material_expiry ON qc_material(expiry_date) WHERE is_deleted = FALSE;

-- ============================================================================
-- QC Rule Table
-- ============================================================================

CREATE TABLE qc_rule (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Rule Details
    rule_name VARCHAR(100) NOT NULL,
    rule_type qc_rule_enum NOT NULL,
    rule_description TEXT,

    -- Configuration
    is_active BOOLEAN DEFAULT TRUE,
    is_blocking BOOLEAN DEFAULT FALSE, -- Block results if violated

    -- Severity
    violation_severity violation_severity NOT NULL DEFAULT 'MEDIUM',

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_qc_rule_org ON qc_rule(organization_id) WHERE is_active = TRUE;

-- ============================================================================
-- QC Material Rule Assignment Table
-- ============================================================================

CREATE TABLE qc_material_rule (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    qc_material_id UUID NOT NULL REFERENCES qc_material(id) ON DELETE CASCADE,
    qc_rule_id UUID NOT NULL REFERENCES qc_rule(id) ON DELETE CASCADE,

    is_active BOOLEAN DEFAULT TRUE,

    created_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,

    CONSTRAINT unique_material_rule UNIQUE(qc_material_id, qc_rule_id)
);

CREATE INDEX idx_material_rule_material ON qc_material_rule(qc_material_id) WHERE is_active = TRUE;

-- ============================================================================
-- QC Result Table
-- ============================================================================

CREATE TABLE qc_result (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    result_number VARCHAR(50) UNIQUE NOT NULL,

    -- QC Material
    qc_material_id UUID NOT NULL REFERENCES qc_material(id) ON DELETE CASCADE,

    -- Organization
    organization_id UUID NOT NULL,

    -- Test Information
    test_id UUID NOT NULL,
    test_name VARCHAR(200),

    -- Equipment
    equipment_id UUID,

    -- Result Data
    result_date DATE NOT NULL,
    result_time TIME NOT NULL,
    result_value DECIMAL(15, 4) NOT NULL,

    -- Statistical Analysis
    mean_value DECIMAL(15, 4), -- Mean at time of result
    sd_value DECIMAL(15, 4),   -- SD at time of result
    cv_value DECIMAL(6, 2),
    z_score DECIMAL(6, 3),      -- Number of SDs from mean

    -- Status
    result_status qc_result_status NOT NULL DEFAULT 'PENDING',

    -- Rules Violated
    rules_violated JSONB, -- Array of rule violations

    -- Performer
    performed_by UUID,
    performed_by_name VARCHAR(200),

    -- Review
    reviewed BOOLEAN DEFAULT FALSE,
    reviewed_by UUID,
    reviewed_at TIMESTAMP,

    -- Comments
    comments TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_qc_result_material ON qc_result(qc_material_id);
CREATE INDEX idx_qc_result_org ON qc_result(organization_id);
CREATE INDEX idx_qc_result_date ON qc_result(result_date DESC);
CREATE INDEX idx_qc_result_status ON qc_result(result_status);
CREATE INDEX idx_qc_result_test ON qc_result(test_id);

-- ============================================================================
-- QC Violation Table
-- ============================================================================

CREATE TABLE qc_violation (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- QC Result
    qc_result_id UUID NOT NULL REFERENCES qc_result(id) ON DELETE CASCADE,
    qc_material_id UUID NOT NULL REFERENCES qc_material(id) ON DELETE CASCADE,

    -- Organization
    organization_id UUID NOT NULL,

    -- Rule Violated
    qc_rule_id UUID NOT NULL REFERENCES qc_rule(id),
    rule_type qc_rule_enum NOT NULL,
    rule_description TEXT,

    -- Violation Details
    violation_date DATE NOT NULL,
    violation_time TIME NOT NULL,
    severity violation_severity NOT NULL,

    -- Impact
    patient_results_affected INTEGER DEFAULT 0,
    patient_results_held INTEGER DEFAULT 0,

    -- Status
    is_acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_by UUID,
    acknowledged_at TIMESTAMP,

    is_resolved BOOLEAN DEFAULT FALSE,
    resolved_by UUID,
    resolved_at TIMESTAMP,

    -- Root Cause
    root_cause TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_qc_violation_result ON qc_violation(qc_result_id);
CREATE INDEX idx_qc_violation_material ON qc_violation(qc_material_id);
CREATE INDEX idx_qc_violation_org ON qc_violation(organization_id);
CREATE INDEX idx_qc_violation_date ON qc_violation(violation_date DESC);
CREATE INDEX idx_qc_violation_status ON qc_violation(is_resolved);

-- ============================================================================
-- QC Corrective Action Table
-- ============================================================================

CREATE TABLE qc_corrective_action (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Violation
    qc_violation_id UUID NOT NULL REFERENCES qc_violation(id) ON DELETE CASCADE,

    -- Action Details
    action_description TEXT NOT NULL,
    action_status corrective_action_status NOT NULL DEFAULT 'PENDING',

    -- Assignment
    assigned_to UUID,
    assigned_to_name VARCHAR(200),

    -- Timeline
    due_date DATE,
    completed_date DATE,

    -- Effectiveness
    effectiveness_verified BOOLEAN DEFAULT FALSE,
    verification_notes TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

CREATE INDEX idx_corrective_action_violation ON qc_corrective_action(qc_violation_id);
CREATE INDEX idx_corrective_action_status ON qc_corrective_action(action_status);

-- ============================================================================
-- QC Statistics Table (Daily aggregates)
-- ============================================================================

CREATE TABLE qc_statistics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    qc_material_id UUID NOT NULL REFERENCES qc_material(id) ON DELETE CASCADE,

    -- Date
    statistics_date DATE NOT NULL,

    -- Sample Size
    n_count INTEGER NOT NULL,

    -- Statistics
    mean_value DECIMAL(15, 4),
    sd_value DECIMAL(15, 4),
    cv_value DECIMAL(6, 2),
    min_value DECIMAL(15, 4),
    max_value DECIMAL(15, 4),
    range_value DECIMAL(15, 4),

    -- Control Status
    in_control_count INTEGER DEFAULT 0,
    out_of_control_count INTEGER DEFAULT 0,
    warning_count INTEGER DEFAULT 0,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),

    CONSTRAINT unique_material_date UNIQUE(qc_material_id, statistics_date)
);

CREATE INDEX idx_qc_stats_material ON qc_statistics(qc_material_id, statistics_date DESC);

-- ============================================================================
-- QC External Program Table (EQC/PT)
-- ============================================================================

CREATE TABLE qc_external_program (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Program Details
    program_name VARCHAR(200) NOT NULL,
    provider VARCHAR(200), -- CAP, EQAS, etc.
    program_code VARCHAR(100),

    -- Type
    qc_type qc NOT NULL, -- EQC or PT

    -- Enrollment
    enrollment_date DATE,
    next_shipment_date DATE,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Contact
    contact_person VARCHAR(200),
    contact_email VARCHAR(255),
    contact_phone VARCHAR(20),

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_external_program_org ON qc_external_program(organization_id) WHERE is_active = TRUE;

-- ============================================================================
-- QC External Survey Table
-- ============================================================================

CREATE TABLE qc_external_survey (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    program_id UUID NOT NULL REFERENCES qc_external_program(id) ON DELETE CASCADE,

    -- Survey Details
    survey_name VARCHAR(200) NOT NULL,
    survey_number VARCHAR(100),
    survey_date DATE NOT NULL,

    -- Shipment
    shipment_date DATE,
    sample_received_date DATE,

    -- Deadline
    submission_deadline DATE,
    submitted_date DATE,

    -- Status
    is_completed BOOLEAN DEFAULT FALSE,

    -- Results Received
    results_received BOOLEAN DEFAULT FALSE,
    results_received_date DATE,

    -- Performance
    acceptable_results INTEGER,
    unacceptable_results INTEGER,
    overall_grade VARCHAR(10), -- A, B, C, F

    -- Report
    report_url VARCHAR(500),

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_external_survey_program ON qc_external_survey(program_id);
CREATE INDEX idx_external_survey_date ON qc_external_survey(survey_date DESC);

-- ============================================================================
-- Functions
-- ============================================================================

-- Generate QC Material Code
CREATE OR REPLACE FUNCTION generate_qc_material_code()
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
    checksum INTEGER;
BEGIN
    sequence_num := nextval('qc_material_sequence');
    base_id := 'QC-' || LPAD(sequence_num::TEXT, 6, '0');
    checksum := (sequence_num % 10);
    RETURN base_id || checksum::TEXT;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS qc_material_sequence START 1;

-- Generate QC Result Number
CREATE OR REPLACE FUNCTION generate_qc_result_number()
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
BEGIN
    sequence_num := nextval('qc_result_sequence');
    base_id := 'QCR-' || TO_CHAR(CURRENT_DATE, 'YYYYMMDD') || '-' || LPAD(sequence_num::TEXT, 5, '0');
    RETURN base_id;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS qc_result_sequence START 1;

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_qc_material_updated_at
    BEFORE UPDATE ON qc_material
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_qc_rule_updated_at
    BEFORE UPDATE ON qc_rule
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Automatically check expiry and update status
CREATE OR REPLACE FUNCTION check_qc_material_expiry()
RETURNS void AS $$
BEGIN
    UPDATE qc_material
    SET material_status = 'EXPIRED'
    WHERE expiry_date < CURRENT_DATE
      AND material_status = 'ACTIVE'
      AND is_deleted = FALSE;
END;
$$ LANGUAGE plpgsql;

-- Calculate QC statistics after result insertion
CREATE OR REPLACE FUNCTION calculate_qc_statistics()
RETURNS TRIGGER AS $$
BEGIN
    -- Calculate z-score
    IF NEW.mean_value IS NOT NULL AND NEW.sd_value IS NOT NULL AND NEW.sd_value > 0 THEN
        NEW.z_score := (NEW.result_value - NEW.mean_value) / NEW.sd_value;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER calculate_qc_stats_trigger
    BEFORE INSERT OR UPDATE ON qc_result
    FOR EACH ROW
    EXECUTE FUNCTION calculate_qc_statistics();

-- Create violation when QC result is out of control
CREATE OR REPLACE FUNCTION create_qc_violation()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.result_status = 'OUT_OF_CONTROL' AND NEW.rules_violated IS NOT NULL THEN
        -- This would be implemented to parse rules_violated JSONB
        -- and create violation records with appropriate severity
        NULL;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_violation_trigger
    AFTER INSERT OR UPDATE ON qc_result
    FOR EACH ROW
    EXECUTE FUNCTION create_qc_violation();

-- ============================================================================
-- Sample Data
-- ============================================================================

-- Default Westgard Rules
INSERT INTO qc_rule (id, organization_id, rule_name, rule_type, rule_description, is_active, is_blocking, violation_severity)
VALUES
(uuid_generate_v4(), (SELECT id FROM organization LIMIT 1), '1-2s Warning', 'WESTGARD_12S', 'One control observation exceeds 2SD limit', TRUE, FALSE, 'LOW'),
(uuid_generate_v4(), (SELECT id FROM organization LIMIT 1), '1-3s Rejection', 'WESTGARD_13S', 'One control observation exceeds 3SD limit', TRUE, TRUE, 'CRITICAL'),
(uuid_generate_v4(), (SELECT id FROM organization LIMIT 1), '2-2s Rejection', 'WESTGARD_22S', 'Two consecutive controls exceed 2SD on same side', TRUE, TRUE, 'HIGH'),
(uuid_generate_v4(), (SELECT id FROM organization LIMIT 1), 'R-4s Rejection', 'WESTGARD_R4S', 'Range between two controls exceeds 4SD', TRUE, TRUE, 'HIGH'),
(uuid_generate_v4(), (SELECT id FROM organization LIMIT 1), '4-1s Rejection', 'WESTGARD_41S', 'Four consecutive controls exceed 1SD on same side', TRUE, TRUE, 'MEDIUM'),
(uuid_generate_v4(), (SELECT id FROM organization LIMIT 1), '10-x Rejection', 'WESTGARD_10X', 'Ten consecutive controls on same side of mean', TRUE, TRUE, 'MEDIUM');

-- Sample QC Material
INSERT INTO qc_material (
    id, material_code, material_name, manufacturer, lot_number,
    qc_type, organization_id, test_id, test_name,
    level_number, level_name,
    target_mean, target_sd,
    expiry_date, quantity_in_stock, minimum_stock_level,
    storage_temperature, material_status
) VALUES (
    uuid_generate_v4(),
    'QC-0000011',
    'BioRad Liquichek Chemistry Control Level 1',
    'Bio-Rad Laboratories',
    'LOT-2024-001',
    'IQC',
    (SELECT id FROM organization LIMIT 1),
    uuid_generate_v4(),
    'Glucose',
    1,
    'Normal',
    100.0,
    5.0,
    '2025-12-31',
    50,
    10,
    '2-8°C',
    'ACTIVE'
);
