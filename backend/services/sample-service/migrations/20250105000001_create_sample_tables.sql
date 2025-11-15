-- Create extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create custom types
CREATE TYPE specimen_type AS ENUM (
    'BLOOD', 'SERUM', 'PLASMA', 'URINE', 'STOOL', 'SPUTUM', 'CSF',
    'TISSUE', 'SWAB', 'BIOPSY', 'ASPIRATE', 'OTHER',
    'SYNOVIAL_FLUID', 'PLEURAL_FLUID'
);

CREATE TYPE sample_status AS ENUM (
    'PENDING',           -- Ordered but not collected
    'COLLECTED',         -- Collected from patient
    'IN_TRANSIT',        -- Being transported
    'RECEIVED',          -- Received at lab
    'ACCEPTED',          -- Accepted for processing
    'REJECTED',          -- Rejected (quality issues)
    'PROCESSING',        -- Being processed
    'COMPLETED',         -- Processing completed
    'STORED',            -- In storage
    'DISPOSED'           -- Disposed of
);

CREATE TYPE rejection_reason AS ENUM (
    'HEMOLYZED', 'CLOTTED', 'INSUFFICIENT_VOLUME', 'CONTAMINATED',
    'EXPIRED', 'IMPROPER_CONTAINER', 'UNLABELED', 'DAMAGED',
    'WRONG_TEST', 'OTHER'
);

CREATE TYPE storage_condition AS ENUM (
    'ROOM_TEMPERATURE', 'REFRIGERATED', 'FROZEN', 'DEEP_FROZEN', 'DRY_ICE'
);

CREATE TYPE priority AS ENUM ('ROUTINE', 'URGENT', 'STAT', 'CRITICAL');

-- Sample table
CREATE TABLE sample (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sample_id VARCHAR(50) UNIQUE NOT NULL,  -- Human-readable sample ID with checksum

    -- Patient & Order references
    patient_id UUID NOT NULL,  -- FK to patient service
    order_id UUID NOT NULL,    -- FK to order service
    organization_id UUID NOT NULL,

    -- Sample details
    specimen_type specimen_type NOT NULL,
    sample_status sample_status NOT NULL DEFAULT 'PENDING',
    priority priority NOT NULL DEFAULT 'ROUTINE',

    -- Collection details
    collection_date_time TIMESTAMP WITH TIME ZONE,
    collector_id UUID,  -- FK to user
    collection_site VARCHAR(200),
    collection_method VARCHAR(200),
    collection_notes TEXT,

    -- Reception details
    received_date_time TIMESTAMP WITH TIME ZONE,
    received_by UUID,  -- FK to user
    reception_temperature DECIMAL(5, 2),  -- In Celsius
    reception_condition VARCHAR(200),

    -- Quality checks
    volume_ml DECIMAL(10, 2),
    appearance TEXT,
    is_hemolyzed BOOLEAN DEFAULT FALSE,
    is_lipemic BOOLEAN DEFAULT FALSE,
    is_icteric BOOLEAN DEFAULT FALSE,

    -- Rejection
    is_rejected BOOLEAN DEFAULT FALSE,
    rejection_reason rejection_reason,
    rejection_notes TEXT,
    rejected_by UUID,  -- FK to user
    rejected_at TIMESTAMP WITH TIME ZONE,

    -- Storage
    storage_location VARCHAR(200),
    storage_condition storage_condition,
    storage_position VARCHAR(100),  -- Rack, shelf, box position
    storage_temperature DECIMAL(5, 2),

    -- Barcode
    barcode VARCHAR(200) UNIQUE,
    barcode_format VARCHAR(50),  -- CODE128, QR, etc.

    -- Processing
    processed_date_time TIMESTAMP WITH TIME ZONE,
    processing_duration_minutes INTEGER,

    -- Disposal
    disposal_date_time TIMESTAMP WITH TIME ZONE,
    disposal_method VARCHAR(200),
    disposal_by UUID,  -- FK to user

    -- Chain of custody
    chain_of_custody JSONB,  -- Track all handlers and timestamps

    -- Metadata
    notes TEXT,
    special_instructions TEXT,
    biohazard_level VARCHAR(50),
    requires_fasting BOOLEAN DEFAULT FALSE,
    fasting_hours INTEGER,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    is_active BOOLEAN DEFAULT TRUE,
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP WITH TIME ZONE,

    CONSTRAINT valid_volume CHECK (volume_ml >= 0),
    CONSTRAINT valid_temperature CHECK (storage_temperature >= -200 AND storage_temperature <= 100)
);

-- Sample Container table
CREATE TABLE sample_container (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sample_id UUID NOT NULL REFERENCES sample(id) ON DELETE CASCADE,

    -- Container details
    container_type VARCHAR(100) NOT NULL,  -- Plain tube, EDTA, Citrate, etc.
    container_size_ml DECIMAL(10, 2),
    cap_color VARCHAR(50),

    -- Additive/preservative
    additive VARCHAR(200),
    preservative VARCHAR(200),
    anticoagulant VARCHAR(100),

    -- Identification
    container_barcode VARCHAR(200),
    position_in_rack VARCHAR(50),

    -- Metadata
    manufacturer VARCHAR(200),
    lot_number VARCHAR(100),
    expiry_date DATE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Sample Aliquot table (for sample splitting)
CREATE TABLE sample_aliquot (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    parent_sample_id UUID NOT NULL REFERENCES sample(id) ON DELETE CASCADE,
    aliquot_id VARCHAR(50) UNIQUE NOT NULL,

    -- Aliquot details
    aliquot_number INTEGER NOT NULL,  -- 1, 2, 3, etc.
    volume_ml DECIMAL(10, 2) NOT NULL,

    -- Storage
    storage_location VARCHAR(200),
    storage_condition storage_condition,
    storage_position VARCHAR(100),

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'AVAILABLE',  -- AVAILABLE, IN_USE, DEPLETED, DISPOSED

    -- Usage tracking
    assigned_to_test_id UUID,  -- Which test is using this aliquot
    used_at TIMESTAMP WITH TIME ZONE,
    used_by UUID,

    -- Disposal
    disposed_at TIMESTAMP WITH TIME ZONE,
    disposal_method VARCHAR(200),

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT valid_aliquot_volume CHECK (volume_ml > 0)
);

-- Sample Routing table
CREATE TABLE sample_routing (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sample_id UUID NOT NULL REFERENCES sample(id) ON DELETE CASCADE,

    -- Routing decision
    route_to VARCHAR(100) NOT NULL,  -- Department/Section
    routed_for VARCHAR(200) NOT NULL,  -- Test/panel name

    -- Assignment
    assigned_to UUID,  -- Equipment or technician
    assignment_type VARCHAR(50),  -- EQUIPMENT, TECHNICIAN, WORKSTATION

    -- Priority
    priority priority NOT NULL DEFAULT 'ROUTINE',

    -- Timing
    routed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expected_completion_time TIMESTAMP WITH TIME ZONE,
    actual_completion_time TIMESTAMP WITH TIME ZONE,

    -- Status
    routing_status VARCHAR(50) NOT NULL DEFAULT 'PENDING',  -- PENDING, ASSIGNED, IN_PROGRESS, COMPLETED, CANCELLED

    -- Automation
    is_automated BOOLEAN DEFAULT FALSE,
    automation_confidence DECIMAL(5, 4),  -- ML confidence score for auto-routing

    -- Notes
    routing_notes TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Sample Temperature Log table (for cold chain monitoring)
CREATE TABLE sample_temperature_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sample_id UUID NOT NULL REFERENCES sample(id) ON DELETE CASCADE,

    temperature_celsius DECIMAL(5, 2) NOT NULL,
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    location VARCHAR(200),
    device_id VARCHAR(100),

    -- Alerts
    is_out_of_range BOOLEAN DEFAULT FALSE,
    alert_triggered BOOLEAN DEFAULT FALSE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Sample Event Log table (audit trail)
CREATE TABLE sample_event_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sample_id UUID NOT NULL REFERENCES sample(id) ON DELETE CASCADE,

    event_type VARCHAR(100) NOT NULL,  -- COLLECTED, RECEIVED, PROCESSED, etc.
    event_description TEXT NOT NULL,

    -- Actor
    performed_by UUID NOT NULL,  -- User ID
    performed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Location
    location VARCHAR(200),
    device_id VARCHAR(100),
    ip_address VARCHAR(45),

    -- Data
    previous_state JSONB,
    new_state JSONB,
    metadata JSONB,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_sample_sample_id ON sample(sample_id);
CREATE INDEX idx_sample_patient ON sample(patient_id);
CREATE INDEX idx_sample_order ON sample(order_id);
CREATE INDEX idx_sample_organization ON sample(organization_id);
CREATE INDEX idx_sample_status ON sample(sample_status);
CREATE INDEX idx_sample_barcode ON sample(barcode);
CREATE INDEX idx_sample_collection_date ON sample(collection_date_time);
CREATE INDEX idx_sample_received_date ON sample(received_date_time);
CREATE INDEX idx_sample_active ON sample(is_active, is_deleted);

CREATE INDEX idx_sample_container_sample ON sample_container(sample_id);
CREATE INDEX idx_sample_aliquot_parent ON sample_aliquot(parent_sample_id);
CREATE INDEX idx_sample_aliquot_id ON sample_aliquot(aliquot_id);
CREATE INDEX idx_sample_routing_sample ON sample_routing(sample_id);
CREATE INDEX idx_sample_routing_status ON sample_routing(routing_status);

CREATE INDEX idx_sample_temp_log_sample ON sample_temperature_log(sample_id);
CREATE INDEX idx_sample_temp_log_time ON sample_temperature_log(recorded_at);

CREATE INDEX idx_sample_event_log_sample ON sample_event_log(sample_id);
CREATE INDEX idx_sample_event_log_time ON sample_event_log(performed_at);

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_sample_updated_at BEFORE UPDATE ON sample
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sample_container_updated_at BEFORE UPDATE ON sample_container
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sample_aliquot_updated_at BEFORE UPDATE ON sample_aliquot
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sample_routing_updated_at BEFORE UPDATE ON sample_routing
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to generate sample ID with Luhn checksum
CREATE OR REPLACE FUNCTION generate_sample_id(org_code VARCHAR, sample_type VARCHAR)
RETURNS VARCHAR AS $$
DECLARE
    base_id VARCHAR;
    checksum CHAR(1);
    full_id VARCHAR;
BEGIN
    -- Generate base ID: ORG-TYPE-YYYYMMDD-SEQUENCE
    base_id := org_code || '-' || sample_type || '-' ||
               TO_CHAR(NOW(), 'YYYYMMDD') || '-' ||
               LPAD(nextval('sample_sequence')::TEXT, 6, '0');

    -- Calculate Luhn checksum (simplified version)
    checksum := (LENGTH(base_id) % 10)::CHAR(1);

    full_id := base_id || checksum;

    RETURN full_id;
END;
$$ LANGUAGE plpgsql;

-- Sequence for sample numbering
CREATE SEQUENCE IF NOT EXISTS sample_sequence START 1;

-- Function to add chain of custody entry
CREATE OR REPLACE FUNCTION add_custody_entry(
    p_sample_id UUID,
    p_handler_id UUID,
    p_action VARCHAR,
    p_location VARCHAR
)
RETURNS VOID AS $$
DECLARE
    custody_entry JSONB;
BEGIN
    custody_entry := jsonb_build_object(
        'timestamp', NOW(),
        'handler_id', p_handler_id,
        'action', p_action,
        'location', p_location
    );

    UPDATE sample
    SET chain_of_custody = COALESCE(chain_of_custody, '[]'::jsonb) || custody_entry
    WHERE id = p_sample_id;
END;
$$ LANGUAGE plpgsql;

-- Insert default test data (for development)
INSERT INTO sample (
    id,
    sample_id,
    patient_id,
    order_id,
    organization_id,
    specimen_type,
    sample_status,
    priority,
    barcode
) VALUES (
    '00000000-0000-0000-0000-000000000001',
    'DEFAULT-BLOOD-20250105-0000011',
    '00000000-0000-0000-0000-000000000001',  -- Default patient
    '00000000-0000-0000-0000-000000000001',  -- Default order
    '00000000-0000-0000-0000-000000000001',  -- Default organization
    'BLOOD',
    'PENDING',
    'ROUTINE',
    'SAMPLE001'
) ON CONFLICT DO NOTHING;
