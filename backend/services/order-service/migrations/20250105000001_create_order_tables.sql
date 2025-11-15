-- Create extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create custom types
CREATE TYPE order_status AS ENUM (
    'DRAFT',           -- Order created but not confirmed
    'CONFIRMED',       -- Order confirmed
    'SAMPLE_COLLECTED', -- Sample collected
    'IN_PROGRESS',     -- Tests being performed
    'COMPLETED',       -- All tests completed
    'PARTIALLY_COMPLETED', -- Some tests completed
    'CANCELLED',       -- Order cancelled
    'ON_HOLD'          -- Order temporarily held
);

CREATE TYPE order_source AS ENUM (
    'WALK_IN',         -- Patient walk-in
    'ONLINE',          -- Web portal
    'MOBILE_APP',      -- Mobile app
    'WHATSAPP',        -- WhatsApp bot
    'API',             -- External API
    'REFERRAL'         -- Doctor referral
);

CREATE TYPE specimen AS ENUM (
    'BLOOD', 'SERUM', 'PLASMA', 'URINE', 'STOOL', 'SPUTUM',
    'CSF', 'TISSUE', 'SWAB', 'BIOPSY', 'ASPIRATE', 'OTHER'
);

CREATE TYPE test_method AS ENUM (
    'MANUAL',          -- Manual testing
    'SEMI_AUTOMATED',  -- Semi-automated
    'AUTOMATED',       -- Fully automated
    'MOLECULAR',       -- Molecular/PCR
    'CULTURE',         -- Bacterial culture
    'MICROSCOPY'       -- Microscopic examination
);

CREATE TYPE result AS ENUM (
    'NUMERIC',         -- Numeric result (e.g., 12.5)
    'TEXT',            -- Text result (e.g., "Positive")
    'RANGE',           -- Range result (e.g., "10-20")
    'OPTION',          -- Multiple choice
    'DESCRIPTIVE'      -- Long text description
);

CREATE TYPE priority AS ENUM (
    'ROUTINE',         -- Normal priority
    'URGENT',          -- High priority
    'STAT'             -- Immediate priority
);

-- Test Category table
CREATE TABLE test_category (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(200) NOT NULL,
    code VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    parent_category_id UUID REFERENCES test_category(id),
    display_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Test Catalog table (Master list of all tests)
CREATE TABLE test_catalog (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    test_code VARCHAR(50) UNIQUE NOT NULL,
    test_name VARCHAR(300) NOT NULL,
    short_name VARCHAR(100),

    -- Category
    category_id UUID REFERENCES test_category(id),
    department VARCHAR(100),  -- Hematology, Biochemistry, etc.

    -- Specimen requirements
    specimen_type specimen NOT NULL,
    specimen_volume_ml DECIMAL(10, 2),
    minimum_volume_ml DECIMAL(10, 2),
    specimen_container VARCHAR(100),
    specimen_preservation VARCHAR(200),

    -- Test specifications
    test_method test_method,
    result_type result NOT NULL,
    unit_of_measurement VARCHAR(50),
    reference_range_text TEXT,

    -- TAT (Turnaround Time)
    standard_tat_hours INTEGER,
    urgent_tat_hours INTEGER,
    stat_tat_hours INTEGER,

    -- Clinical information
    clinical_significance TEXT,
    indications TEXT,
    contraindications TEXT,
    interfering_factors TEXT,

    -- Requirements
    requires_fasting BOOLEAN DEFAULT FALSE,
    fasting_hours INTEGER,
    special_instructions TEXT,

    -- Pricing
    base_price DECIMAL(10, 2),
    urgent_price_multiplier DECIMAL(5, 2) DEFAULT 1.5,
    stat_price_multiplier DECIMAL(5, 2) DEFAULT 2.0,

    -- External lab
    is_outsourced BOOLEAN DEFAULT FALSE,
    external_lab_name VARCHAR(200),
    external_lab_code VARCHAR(100),
    external_tat_hours INTEGER,

    -- Metadata
    nabl_accredited BOOLEAN DEFAULT FALSE,
    cap_accredited BOOLEAN DEFAULT FALSE,
    notes TEXT,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    is_available BOOLEAN DEFAULT TRUE,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,

    CONSTRAINT valid_volume CHECK (specimen_volume_ml >= 0),
    CONSTRAINT valid_min_volume CHECK (minimum_volume_ml >= 0),
    CONSTRAINT valid_tat CHECK (standard_tat_hours > 0)
);

-- Test Panel table (Groups of tests)
CREATE TABLE test_panel (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    panel_code VARCHAR(50) UNIQUE NOT NULL,
    panel_name VARCHAR(300) NOT NULL,
    short_name VARCHAR(100),

    description TEXT,
    category_id UUID REFERENCES test_category(id),

    -- Pricing
    panel_price DECIMAL(10, 2),
    discount_percentage DECIMAL(5, 2) DEFAULT 0,

    -- Metadata
    is_popular BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    display_order INTEGER DEFAULT 0,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

-- Test Panel Items table (Tests in a panel)
CREATE TABLE test_panel_item (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    panel_id UUID NOT NULL REFERENCES test_panel(id) ON DELETE CASCADE,
    test_id UUID NOT NULL REFERENCES test_catalog(id) ON DELETE CASCADE,
    is_mandatory BOOLEAN DEFAULT TRUE,
    display_order INTEGER DEFAULT 0,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(panel_id, test_id)
);

-- Test Order table
CREATE TABLE test_order (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_number VARCHAR(50) UNIQUE NOT NULL,

    -- Patient and Organization
    patient_id UUID NOT NULL,
    organization_id UUID NOT NULL,

    -- Order details
    order_status order_status NOT NULL DEFAULT 'DRAFT',
    order_source order_source NOT NULL,
    priority priority NOT NULL DEFAULT 'ROUTINE',

    -- Referring details
    referring_doctor_id UUID,
    referring_doctor_name VARCHAR(200),
    referring_facility VARCHAR(200),
    clinical_notes TEXT,
    icd_codes TEXT[],  -- Array of ICD-10 codes

    -- Timing
    order_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    confirmed_at TIMESTAMP WITH TIME ZONE,
    expected_completion_date TIMESTAMP WITH TIME ZONE,
    actual_completion_date TIMESTAMP WITH TIME ZONE,

    -- Sample collection
    collection_date_time TIMESTAMP WITH TIME ZONE,
    collection_location VARCHAR(200),
    home_collection_requested BOOLEAN DEFAULT FALSE,
    home_collection_address TEXT,

    -- Pricing
    total_amount DECIMAL(10, 2) NOT NULL DEFAULT 0,
    discount_amount DECIMAL(10, 2) DEFAULT 0,
    discount_percentage DECIMAL(5, 2) DEFAULT 0,
    discount_reason TEXT,
    tax_amount DECIMAL(10, 2) DEFAULT 0,
    final_amount DECIMAL(10, 2) NOT NULL DEFAULT 0,

    -- Payment
    payment_status VARCHAR(50) DEFAULT 'PENDING',  -- PENDING, PARTIAL, PAID
    payment_method VARCHAR(50),  -- CASH, CARD, UPI, INSURANCE
    advance_paid DECIMAL(10, 2) DEFAULT 0,

    -- Insurance
    insurance_company VARCHAR(200),
    insurance_policy_number VARCHAR(100),
    insurance_approval_number VARCHAR(100),
    insurance_amount_approved DECIMAL(10, 2),

    -- Delivery
    report_delivery_method VARCHAR(50),  -- EMAIL, WHATSAPP, PRINT, PORTAL
    report_delivery_email VARCHAR(255),
    report_delivery_phone VARCHAR(20),

    -- Metadata
    notes TEXT,
    special_instructions TEXT,

    -- Cancellation
    is_cancelled BOOLEAN DEFAULT FALSE,
    cancelled_at TIMESTAMP WITH TIME ZONE,
    cancelled_by UUID,
    cancellation_reason TEXT,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_by UUID NOT NULL,
    updated_by UUID,
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP WITH TIME ZONE,

    CONSTRAINT valid_amounts CHECK (total_amount >= 0 AND final_amount >= 0)
);

-- Test Order Item table (Individual tests in an order)
CREATE TABLE test_order_item (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES test_order(id) ON DELETE CASCADE,

    -- Test details
    test_id UUID REFERENCES test_catalog(id),
    panel_id UUID REFERENCES test_panel(id),

    -- Either test_id OR panel_id must be set
    test_name VARCHAR(300) NOT NULL,
    test_code VARCHAR(50) NOT NULL,

    -- Sample
    sample_id UUID,  -- Links to sample service specimen specimen_type,

    -- Status
    item_status VARCHAR(50) NOT NULL DEFAULT 'PENDING',

    -- Pricing
    unit_price DECIMAL(10, 2) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    discount_amount DECIMAL(10, 2) DEFAULT 0,
    tax_amount DECIMAL(10, 2) DEFAULT 0,
    total_price DECIMAL(10, 2) NOT NULL,

    -- Result
    result_id UUID,  -- Links to result service
    result_status VARCHAR(50) DEFAULT 'PENDING',

    -- TAT
    expected_completion TIMESTAMP WITH TIME ZONE,
    actual_completion TIMESTAMP WITH TIME ZONE,

    -- Metadata
    notes TEXT,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT valid_test_or_panel CHECK (
        (test_id IS NOT NULL AND panel_id IS NULL) OR
        (test_id IS NULL AND panel_id IS NOT NULL)
    ),
    CONSTRAINT valid_quantity CHECK (quantity > 0),
    CONSTRAINT valid_price CHECK (unit_price >= 0 AND total_price >= 0)
);

-- Test Price table (Dynamic pricing)
CREATE TABLE test_price (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    test_id UUID NOT NULL REFERENCES test_catalog(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL,

    -- Pricing
    base_price DECIMAL(10, 2) NOT NULL,
    urgent_price DECIMAL(10, 2),
    stat_price DECIMAL(10, 2),

    -- Discounts
    discount_percentage DECIMAL(5, 2) DEFAULT 0,

    -- Insurance
    insurance_company VARCHAR(200),
    insurance_price DECIMAL(10, 2),

    -- Validity
    valid_from DATE NOT NULL,
    valid_to DATE,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,

    CONSTRAINT valid_price_dates CHECK (valid_to IS NULL OR valid_to >= valid_from)
);

-- Order Status History table (Audit trail)
CREATE TABLE order_status_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES test_order(id) ON DELETE CASCADE,

    previous_status order_status,
    new_status order_status NOT NULL,

    changed_by UUID NOT NULL,
    changed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    notes TEXT,
    metadata JSONB
);

-- Indexes for performance
CREATE INDEX idx_test_catalog_code ON test_catalog(test_code);
CREATE INDEX idx_test_catalog_name ON test_catalog(test_name);
CREATE INDEX idx_test_catalog_category ON test_catalog(category_id);
CREATE INDEX idx_test_catalog_department ON test_catalog(department);
CREATE INDEX idx_test_catalog_active ON test_catalog(is_active, is_available);

CREATE INDEX idx_test_panel_code ON test_panel(panel_code);
CREATE INDEX idx_test_panel_name ON test_panel(panel_name);
CREATE INDEX idx_test_panel_active ON test_panel(is_active);

CREATE INDEX idx_test_order_number ON test_order(order_number);
CREATE INDEX idx_test_order_patient ON test_order(patient_id);
CREATE INDEX idx_test_order_organization ON test_order(organization_id);
CREATE INDEX idx_test_order_status ON test_order(order_status);
CREATE INDEX idx_test_order_date ON test_order(order_date);
CREATE INDEX idx_test_order_created ON test_order(created_at);

CREATE INDEX idx_test_order_item_order ON test_order_item(order_id);
CREATE INDEX idx_test_order_item_test ON test_order_item(test_id);
CREATE INDEX idx_test_order_item_sample ON test_order_item(sample_id);
CREATE INDEX idx_test_order_item_result ON test_order_item(result_id);

CREATE INDEX idx_test_price_test ON test_price(test_id);
CREATE INDEX idx_test_price_org ON test_price(organization_id);
CREATE INDEX idx_test_price_active ON test_price(is_active);

CREATE INDEX idx_order_history_order ON order_status_history(order_id);

-- Full-text search on test names
CREATE INDEX idx_test_catalog_name_fts ON test_catalog USING gin(to_tsvector('english', test_name));
CREATE INDEX idx_test_panel_name_fts ON test_panel USING gin(to_tsvector('english', panel_name));

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_test_catalog_updated_at BEFORE UPDATE ON test_catalog
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_test_panel_updated_at BEFORE UPDATE ON test_panel
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_test_order_updated_at BEFORE UPDATE ON test_order
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_test_order_item_updated_at BEFORE UPDATE ON test_order_item
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_test_price_updated_at BEFORE UPDATE ON test_price
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to generate order number with checksum
CREATE OR REPLACE FUNCTION generate_order_number(org_code VARCHAR)
RETURNS VARCHAR AS $$
DECLARE
    base_id VARCHAR;
    checksum CHAR(1);
    full_id VARCHAR;
BEGIN
    -- Generate base ID: ORG-ORD-YYYYMMDD-SEQUENCE
    base_id := org_code || '-ORD-' ||
               TO_CHAR(NOW(), 'YYYYMMDD') || '-' ||
               LPAD(nextval('order_sequence')::TEXT, 6, '0');

    -- Calculate Luhn checksum
    checksum := (LENGTH(base_id) % 10)::CHAR(1);

    full_id := base_id || checksum;

    RETURN full_id;
END;
$$ LANGUAGE plpgsql;

-- Sequence for order numbering
CREATE SEQUENCE IF NOT EXISTS order_sequence START 1;

-- Function to calculate order total
CREATE OR REPLACE FUNCTION calculate_order_total(p_order_id UUID)
RETURNS DECIMAL AS $$
DECLARE
    total DECIMAL(10, 2);
BEGIN
    SELECT COALESCE(SUM(total_price), 0)
    INTO total
    FROM test_order_item
    WHERE order_id = p_order_id;

    RETURN total;
END;
$$ LANGUAGE plpgsql;

-- Trigger to update order status history
CREATE OR REPLACE FUNCTION log_order_status_change()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.order_status IS DISTINCT FROM NEW.order_status THEN
        INSERT INTO order_status_history (order_id, previous_status, new_status, changed_by)
        VALUES (NEW.id, OLD.order_status, NEW.order_status, NEW.updated_by);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER log_order_status_change_trigger
AFTER UPDATE ON test_order
FOR EACH ROW
WHEN (OLD.order_status IS DISTINCT FROM NEW.order_status)
EXECUTE FUNCTION log_order_status_change();

-- Insert default test categories
INSERT INTO test_category (id, name, code, description) VALUES
    ('00000000-0000-0000-0000-000000000001', 'Hematology', 'HEMA', 'Blood tests'),
    ('00000000-0000-0000-0000-000000000002', 'Biochemistry', 'BIOC', 'Chemical analysis'),
    ('00000000-0000-0000-0000-000000000003', 'Microbiology', 'MICR', 'Infection tests'),
    ('00000000-0000-0000-0000-000000000004', 'Immunology', 'IMMU', 'Immune system tests'),
    ('00000000-0000-0000-0000-000000000005', 'Clinical Pathology', 'CLPA', 'Clinical tests')
ON CONFLICT DO NOTHING;

-- Insert sample tests
INSERT INTO test_catalog (id, test_code, test_name, short_name, category_id, specimen_type, result_type, base_price, standard_tat_hours, is_active) VALUES
    ('00000000-0000-0000-0000-000000000010', 'CBC', 'Complete Blood Count', 'CBC', '00000000-0000-0000-0000-000000000001', 'BLOOD', 'NUMERIC', 250.00, 4, TRUE),
    ('00000000-0000-0000-0000-000000000011', 'HB', 'Hemoglobin', 'Hb', '00000000-0000-0000-0000-000000000001', 'BLOOD', 'NUMERIC', 100.00, 2, TRUE),
    ('00000000-0000-0000-0000-000000000012', 'BSL', 'Blood Sugar Level', 'BSL', '00000000-0000-0000-0000-000000000002', 'SERUM', 'NUMERIC', 80.00, 2, TRUE),
    ('00000000-0000-0000-0000-000000000013', 'LFT', 'Liver Function Test', 'LFT', '00000000-0000-0000-0000-000000000002', 'SERUM', 'NUMERIC', 600.00, 24, TRUE),
    ('00000000-0000-0000-0000-000000000014', 'KFT', 'Kidney Function Test', 'KFT', '00000000-0000-0000-0000-000000000002', 'SERUM', 'NUMERIC', 550.00, 24, TRUE)
ON CONFLICT DO NOTHING;

-- Insert sample panel
INSERT INTO test_panel (id, panel_code, panel_name, short_name, panel_price, is_active) VALUES
    ('00000000-0000-0000-0000-000000000020', 'HEALTH-CHK', 'Basic Health Checkup', 'Health Checkup', 1200.00, TRUE)
ON CONFLICT DO NOTHING;

-- Link tests to panel
INSERT INTO test_panel_item (panel_id, test_id, display_order) VALUES
    ('00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000010', 1),
    ('00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000012', 2),
    ('00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000013', 3)
ON CONFLICT DO NOTHING;
