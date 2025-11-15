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

CREATE TYPE equipment_status AS ENUM (
    'ACTIVE',
    'INACTIVE',
    'MAINTENANCE',
    'CALIBRATION',
    'UNDER_REPAIR',
    'RETIRED',
    'QUARANTINE'
);

CREATE TYPE equipment_type_enum AS ENUM (
    'ANALYZER',
    'CENTRIFUGE',
    'MICROSCOPE',
    'REFRIGERATOR',
    'FREEZER',
    'INCUBATOR',
    'AUTOCLAVE',
    'WATER_BATH',
    'PCR_MACHINE',
    'SPECTROPHOTOMETER',
    'HEMATOLOGY_ANALYZER',
    'CHEMISTRY_ANALYZER',
    'IMMUNOASSAY_ANALYZER',
    'COAGULATION_ANALYZER',
    'BLOOD_GAS_ANALYZER',
    'MICROBIOLOGY_ANALYZER',
    'ELISA_READER',
    'PIPETTE',
    'BALANCE',
    'PH_METER',
    'OTHER'
);

CREATE TYPE maintenance AS ENUM (
    'PREVENTIVE',
    'CORRECTIVE',
    'CALIBRATION',
    'VALIDATION',
    'QUALIFICATION'
);

CREATE TYPE maintenance_status AS ENUM (
    'SCHEDULED',
    'IN_PROGRESS',
    'COMPLETED',
    'OVERDUE',
    'CANCELLED'
);

CREATE TYPE calibration_status AS ENUM (
    'PASSED',
    'FAILED',
    'CONDITIONAL',
    'PENDING'
);

-- ============================================================================
-- Equipment Table
-- ============================================================================

CREATE TABLE equipment (
    -- Identity
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    equipment_code VARCHAR(50) UNIQUE NOT NULL,

    -- Basic Information
    equipment_name VARCHAR(200) NOT NULL,
    equipment_type equipment_type_enum NOT NULL,
    manufacturer VARCHAR(200),
    model_number VARCHAR(100),
    serial_number VARCHAR(100) UNIQUE,

    -- Status
    equipment_status equipment_status NOT NULL DEFAULT 'ACTIVE',

    -- Organization and Location
    organization_id UUID NOT NULL,
    branch_id UUID,
    department_id UUID,
    location VARCHAR(200), -- Lab room/area

    -- Purchase Details
    purchase_date DATE,
    purchase_cost DECIMAL(12, 2),
    vendor VARCHAR(200),
    warranty_expiry_date DATE,

    -- Installation
    installation_date DATE,
    commissioning_date DATE,

    -- Specifications
    specifications JSONB, -- Technical specs
    capacity VARCHAR(100), -- e.g., "96 tests/hour"

    -- Connectivity
    interface_type VARCHAR(50), -- LIS, Serial, USB, Network
    ip_address INET,
    mac_address MACADDR,
    lis_integration_enabled BOOLEAN DEFAULT FALSE,

    -- Maintenance Schedule
    maintenance_frequency_days INTEGER, -- Preventive maintenance frequency
    last_maintenance_date DATE,
    next_maintenance_date DATE,

    -- Calibration Schedule
    calibration_frequency_days INTEGER,
    last_calibration_date DATE,
    next_calibration_date DATE,

    -- Performance
    total_tests_processed INTEGER DEFAULT 0,
    uptime_percentage DECIMAL(5, 2), -- 99.99%
    mean_time_between_failures INTEGER, -- Hours

    -- Documentation
    user_manual_url VARCHAR(500),
    service_manual_url VARCHAR(500),
    sop_document_url VARCHAR(500),

    -- Notes
    notes TEXT,

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
CREATE INDEX idx_equipment_code ON equipment(equipment_code) WHERE is_deleted = FALSE;
CREATE INDEX idx_equipment_status ON equipment(equipment_status) WHERE is_deleted = FALSE;
CREATE INDEX idx_equipment_org ON equipment(organization_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_equipment_branch ON equipment(branch_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_equipment_type ON equipment(equipment_type) WHERE is_deleted = FALSE;
CREATE INDEX idx_equipment_dept ON equipment(department_id) WHERE is_deleted = FALSE;

-- Full-text search
CREATE INDEX idx_equipment_search ON equipment USING GIN(
    to_tsvector('english', equipment_name || ' ' || COALESCE(manufacturer, '') || ' ' || COALESCE(model_number, ''))
) WHERE is_deleted = FALSE;

-- ============================================================================
-- Equipment Maintenance Table
-- ============================================================================

CREATE TABLE equipment_maintenance (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    equipment_id UUID NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,

    -- Maintenance Details
    maintenance_type maintenance NOT NULL,
    maintenance_status maintenance_status NOT NULL DEFAULT 'SCHEDULED',

    -- Schedule
    scheduled_date DATE NOT NULL,
    completed_date DATE,

    -- Performed By
    technician_id UUID,
    technician_name VARCHAR(200),
    vendor_name VARCHAR(200), -- External vendor if applicable

    -- Work Done
    work_description TEXT,
    parts_replaced JSONB, -- Array of parts
    cost DECIMAL(10, 2),

    -- Results
    before_condition TEXT,
    after_condition TEXT,
    findings TEXT,
    recommendations TEXT,

    -- Next Maintenance
    next_maintenance_date DATE,

    -- Documentation
    report_url VARCHAR(500),
    checklist_completed JSONB, -- Maintenance checklist items

    -- Downtime Tracking
    downtime_start TIMESTAMP,
    downtime_end TIMESTAMP,
    downtime_hours DECIMAL(6, 2),

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_maintenance_equipment ON equipment_maintenance(equipment_id);
CREATE INDEX idx_maintenance_status ON equipment_maintenance(maintenance_status, scheduled_date);
CREATE INDEX idx_maintenance_type ON equipment_maintenance(maintenance_type);

-- ============================================================================
-- Equipment Calibration Table
-- ============================================================================

CREATE TABLE equipment_calibration (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    equipment_id UUID NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,

    -- Calibration Details
    calibration_date DATE NOT NULL,
    due_date DATE NOT NULL,
    calibration_status calibration_status NOT NULL DEFAULT 'PENDING',

    -- Performed By
    performed_by_id UUID,
    performed_by_name VARCHAR(200),
    calibration_agency VARCHAR(200), -- External calibration agency

    -- Certificate
    certificate_number VARCHAR(100),
    certificate_url VARCHAR(500),

    -- Calibration Points
    calibration_points JSONB, -- Array of {expected, measured, deviation}

    -- Standards Used
    reference_standards_used JSONB, -- Calibrators/standards

    -- Results
    before_accuracy DECIMAL(6, 2), -- Percentage
    after_accuracy DECIMAL(6, 2),
    within_specification BOOLEAN,

    -- Deviations
    deviations_found TEXT,
    corrective_actions TEXT,

    -- Next Calibration
    next_calibration_date DATE,

    -- Cost
    cost DECIMAL(10, 2),

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_calibration_equipment ON equipment_calibration(equipment_id);
CREATE INDEX idx_calibration_status ON equipment_calibration(calibration_status, due_date);
CREATE INDEX idx_calibration_date ON equipment_calibration(calibration_date DESC);

-- ============================================================================
-- Equipment Test Assignment Table
-- ============================================================================

CREATE TABLE equipment_test_assignment (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    equipment_id UUID NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,
    test_id UUID NOT NULL, -- Reference to test catalog

    -- Assignment Details
    is_primary BOOLEAN DEFAULT FALSE, -- Primary equipment for this test
    is_backup BOOLEAN DEFAULT FALSE, -- Backup equipment

    -- Performance
    average_tat_minutes INTEGER, -- Average turnaround time
    success_rate DECIMAL(5, 2), -- Percentage

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,

    -- Constraints
    CONSTRAINT unique_equipment_test UNIQUE(equipment_id, test_id)
);

CREATE INDEX idx_test_assignment_equipment ON equipment_test_assignment(equipment_id) WHERE is_active = TRUE;
CREATE INDEX idx_test_assignment_test ON equipment_test_assignment(test_id) WHERE is_active = TRUE;

-- ============================================================================
-- Equipment Performance Log Table
-- ============================================================================

CREATE TABLE equipment_performance_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    equipment_id UUID NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,

    -- Performance Data
    log_date DATE NOT NULL,
    tests_processed INTEGER DEFAULT 0,
    tests_failed INTEGER DEFAULT 0,
    downtime_minutes INTEGER DEFAULT 0,

    -- Quality Metrics
    error_rate DECIMAL(5, 2), -- Percentage
    success_rate DECIMAL(5, 2),
    average_processing_time_seconds DECIMAL(8, 2),

    -- Reagent Consumption
    reagent_consumption JSONB, -- Array of reagents used

    -- Issues
    issues_reported TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_performance_equipment ON equipment_performance_log(equipment_id, log_date DESC);

-- ============================================================================
-- Equipment Alert Table
-- ============================================================================

CREATE TABLE equipment_alert (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    equipment_id UUID NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,

    -- Alert Details
    alert_type VARCHAR(50) NOT NULL, -- MAINTENANCE_DUE, CALIBRATION_DUE, ERROR, WARNING
    severity VARCHAR(20) NOT NULL, -- LOW, MEDIUM, HIGH, CRITICAL
    message TEXT NOT NULL,

    -- Status
    is_acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_by UUID,
    acknowledged_at TIMESTAMP,

    is_resolved BOOLEAN DEFAULT FALSE,
    resolved_by UUID,
    resolved_at TIMESTAMP,
    resolution_notes TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_alert_equipment ON equipment_alert(equipment_id, is_resolved);
CREATE INDEX idx_alert_severity ON equipment_alert(severity, is_acknowledged) WHERE is_resolved = FALSE;

-- ============================================================================
-- Equipment Audit Log Table
-- ============================================================================

CREATE TABLE equipment_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    equipment_id UUID NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,

    -- Action Details
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,

    -- Changes
    old_value JSONB,
    new_value JSONB,

    -- Actor
    performed_by UUID,
    performed_at TIMESTAMP DEFAULT NOW(),

    -- Context
    ip_address INET,
    description TEXT
);

CREATE INDEX idx_equipment_audit_equipment ON equipment_audit_log(equipment_id);
CREATE INDEX idx_equipment_audit_date ON equipment_audit_log(performed_at DESC);

-- ============================================================================
-- Functions
-- ============================================================================

-- Generate Equipment Code
CREATE OR REPLACE FUNCTION generate_equipment_code()
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
    checksum INTEGER;
BEGIN
    sequence_num := nextval('equipment_sequence');
    base_id := 'EQ-' || LPAD(sequence_num::TEXT, 6, '0');
    checksum := (sequence_num % 10);
    RETURN base_id || checksum::TEXT;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS equipment_sequence START 1;

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_equipment_updated_at
    BEFORE UPDATE ON equipment
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_maintenance_updated_at
    BEFORE UPDATE ON equipment_maintenance
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_calibration_updated_at
    BEFORE UPDATE ON equipment_calibration
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Calculate downtime automatically
CREATE OR REPLACE FUNCTION calculate_downtime()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.downtime_start IS NOT NULL AND NEW.downtime_end IS NOT NULL THEN
        NEW.downtime_hours := EXTRACT(EPOCH FROM (NEW.downtime_end - NEW.downtime_start)) / 3600;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER calculate_maintenance_downtime
    BEFORE INSERT OR UPDATE ON equipment_maintenance
    FOR EACH ROW
    EXECUTE FUNCTION calculate_downtime();

-- Update next maintenance date when maintenance is completed
CREATE OR REPLACE FUNCTION update_equipment_maintenance_dates()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.maintenance_status = 'COMPLETED' AND OLD.maintenance_status != 'COMPLETED' THEN
        UPDATE equipment
        SET last_maintenance_date = NEW.completed_date,
            next_maintenance_date = NEW.next_maintenance_date
        WHERE id = NEW.equipment_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_equipment_maintenance_trigger
    AFTER UPDATE ON equipment_maintenance
    FOR EACH ROW
    EXECUTE FUNCTION update_equipment_maintenance_dates();

-- Update next calibration date when calibration is completed
CREATE OR REPLACE FUNCTION update_equipment_calibration_dates()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.calibration_status IN ('PASSED', 'CONDITIONAL') THEN
        UPDATE equipment
        SET last_calibration_date = NEW.calibration_date,
            next_calibration_date = NEW.next_calibration_date
        WHERE id = NEW.equipment_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_equipment_calibration_trigger
    AFTER INSERT OR UPDATE ON equipment_calibration
    FOR EACH ROW
    EXECUTE FUNCTION update_equipment_calibration_dates();

-- Create alerts for overdue maintenance
CREATE OR REPLACE FUNCTION create_maintenance_alerts()
RETURNS void AS $$
BEGIN
    INSERT INTO equipment_alert (equipment_id, alert_type, severity, message)
    SELECT
        id,
        'MAINTENANCE_DUE',
        CASE
            WHEN next_maintenance_date < CURRENT_DATE THEN 'HIGH'
            WHEN next_maintenance_date <= CURRENT_DATE + INTERVAL '7 days' THEN 'MEDIUM'
            ELSE 'LOW'
        END,
        'Preventive maintenance due for ' || equipment_name
    FROM equipment
    WHERE next_maintenance_date IS NOT NULL
      AND next_maintenance_date <= CURRENT_DATE + INTERVAL '14 days'
      AND equipment_status NOT IN ('MAINTENANCE', 'RETIRED', 'INACTIVE')
      AND is_deleted = FALSE
      AND id NOT IN (
          SELECT equipment_id FROM equipment_alert
          WHERE alert_type = 'MAINTENANCE_DUE'
            AND is_resolved = FALSE
            AND created_at > CURRENT_DATE - INTERVAL '7 days'
      );
END;
$$ LANGUAGE plpgsql;

-- Create alerts for overdue calibration
CREATE OR REPLACE FUNCTION create_calibration_alerts()
RETURNS void AS $$
BEGIN
    INSERT INTO equipment_alert (equipment_id, alert_type, severity, message)
    SELECT
        id,
        'CALIBRATION_DUE',
        CASE
            WHEN next_calibration_date < CURRENT_DATE THEN 'CRITICAL'
            WHEN next_calibration_date <= CURRENT_DATE + INTERVAL '7 days' THEN 'HIGH'
            ELSE 'MEDIUM'
        END,
        'Calibration due for ' || equipment_name
    FROM equipment
    WHERE next_calibration_date IS NOT NULL
      AND next_calibration_date <= CURRENT_DATE + INTERVAL '14 days'
      AND equipment_status NOT IN ('CALIBRATION', 'RETIRED', 'INACTIVE')
      AND is_deleted = FALSE
      AND id NOT IN (
          SELECT equipment_id FROM equipment_alert
          WHERE alert_type = 'CALIBRATION_DUE'
            AND is_resolved = FALSE
            AND created_at > CURRENT_DATE - INTERVAL '7 days'
      );
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Sample Data
-- ============================================================================

-- Sample Equipment
INSERT INTO equipment (
    id, equipment_code, equipment_name, equipment_type,
    manufacturer, model_number, serial_number,
    equipment_status, organization_id,
    purchase_date, installation_date,
    maintenance_frequency_days, calibration_frequency_days,
    lis_integration_enabled
) VALUES (
    uuid_generate_v4(),
    'EQ-0000011',
    'Cobas 6000 Chemistry Analyzer',
    'CHEMISTRY_ANALYZER',
    'Roche Diagnostics',
    'COBAS-6000',
    'SN-2024-001',
    'ACTIVE',
    (SELECT id FROM organization LIMIT 1),
    '2024-01-15',
    '2024-02-01',
    90, -- Maintenance every 90 days
    180, -- Calibration every 180 days
    true
);

INSERT INTO equipment (
    id, equipment_code, equipment_name, equipment_type,
    manufacturer, model_number, serial_number,
    equipment_status, organization_id,
    maintenance_frequency_days, calibration_frequency_days
) VALUES (
    uuid_generate_v4(),
    'EQ-0000022',
    'Sysmex XN-1000 Hematology Analyzer',
    'HEMATOLOGY_ANALYZER',
    'Sysmex',
    'XN-1000',
    'SN-2024-002',
    'ACTIVE',
    (SELECT id FROM organization LIMIT 1),
    60, -- Maintenance every 60 days
    90  -- Calibration every 90 days
);
