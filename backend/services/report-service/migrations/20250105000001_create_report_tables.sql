-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Report Service Schema
-- ============================================================================

-- Report template types
CREATE TYPE report_template_type AS ENUM (
    'PATIENT_REPORT',
    'BATCH_REPORT',
    'QC_REPORT',
    'SUMMARY_REPORT',
    'CUSTOM_REPORT'
);

-- Report formats
CREATE TYPE report_format AS ENUM (
    'PDF',
    'HTML',
    'CSV',
    'EXCEL',
    'JSON'
);

-- Report status
CREATE TYPE report_status AS ENUM (
    'PENDING',
    'GENERATING',
    'GENERATED',
    'FAILED',
    'DELIVERED',
    'ARCHIVED'
);

-- Delivery channels
CREATE TYPE delivery_channel AS ENUM (
    'EMAIL',
    'WHATSAPP',
    'SMS',
    'DOWNLOAD',
    'PRINT'
);

-- Delivery status
CREATE TYPE delivery_status AS ENUM (
    'PENDING',
    'SENT',
    'DELIVERED',
    'FAILED',
    'BOUNCED'
);

-- Signature status
CREATE TYPE signature_status AS ENUM (
    'PENDING',
    'SIGNED',
    'REJECTED',
    'EXPIRED'
);

-- ============================================================================
-- Report Template Table
-- ============================================================================

CREATE TABLE report_template (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Template identification
    template_name VARCHAR(200) NOT NULL,
    template_code VARCHAR(100) UNIQUE NOT NULL,
    template_type report_template_type NOT NULL,
    description TEXT,

    -- Template configuration
    template_content JSONB NOT NULL, -- HTML/template structure
    header_content JSONB,            -- Header configuration
    footer_content JSONB,            -- Footer configuration
    styles JSONB,                    -- CSS styles

    -- Page settings
    page_size VARCHAR(20) DEFAULT 'A4',
    page_orientation VARCHAR(20) DEFAULT 'PORTRAIT', -- PORTRAIT or LANDSCAPE
    margin_top INTEGER DEFAULT 10,    -- in mm
    margin_bottom INTEGER DEFAULT 10,
    margin_left INTEGER DEFAULT 10,
    margin_right INTEGER DEFAULT 10,

    -- Report fields configuration
    fields_config JSONB,             -- Field definitions and mappings
    sections_config JSONB,           -- Section layout configuration

    -- Branding
    show_logo BOOLEAN DEFAULT true,
    show_watermark BOOLEAN DEFAULT false,
    watermark_text VARCHAR(100),

    -- Digital signature settings
    requires_signature BOOLEAN DEFAULT false,
    signature_fields JSONB,          -- Which signatures are required

    -- Output settings
    default_format report_format DEFAULT 'PDF',
    enable_auto_delivery BOOLEAN DEFAULT false,
    auto_delivery_channels JSONB,

    -- Status and metadata
    is_active BOOLEAN DEFAULT true,
    is_default BOOLEAN DEFAULT false,
    version INTEGER DEFAULT 1,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false,

    CONSTRAINT report_template_org_code_unique UNIQUE (organization_id, template_code)
);

-- Indexes
CREATE INDEX idx_report_template_org ON report_template(organization_id) WHERE is_deleted = false;
CREATE INDEX idx_report_template_type ON report_template(template_type) WHERE is_deleted = false;
CREATE INDEX idx_report_template_active ON report_template(is_active) WHERE is_deleted = false;

-- ============================================================================
-- Generated Report Table
-- ============================================================================

CREATE TABLE generated_report (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,
    template_id UUID REFERENCES report_template(id),

    -- Report identification
    report_number VARCHAR(50) UNIQUE NOT NULL,
    report_title VARCHAR(300) NOT NULL,
    report_type report_template_type NOT NULL,

    -- Associated entities
    patient_id UUID,
    order_id UUID,
    result_id UUID,
    batch_id UUID,

    -- Report content
    report_data JSONB NOT NULL,      -- Data used to generate report
    generated_content TEXT,          -- Generated HTML/content
    report_format report_format DEFAULT 'PDF',

    -- File storage
    file_path VARCHAR(500),
    file_size_bytes BIGINT,
    file_hash VARCHAR(128),          -- SHA256 hash for integrity
    storage_location VARCHAR(100),   -- S3, local, etc.

    -- Report metadata
    report_date DATE NOT NULL,
    generated_at TIMESTAMP,
    expires_at TIMESTAMP,            -- Optional expiry for temporary reports

    -- Status
    report_status report_status NOT NULL DEFAULT 'PENDING',
    error_message TEXT,

    -- Access control
    is_confidential BOOLEAN DEFAULT true,
    access_code VARCHAR(50),         -- Optional access code for secure sharing
    download_count INTEGER DEFAULT 0,
    last_downloaded_at TIMESTAMP,

    -- Digital signatures
    requires_signature BOOLEAN DEFAULT false,
    is_signed BOOLEAN DEFAULT false,
    signed_at TIMESTAMP,
    signed_by UUID,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_generated_report_org ON generated_report(organization_id) WHERE is_deleted = false;
CREATE INDEX idx_generated_report_patient ON generated_report(patient_id) WHERE is_deleted = false;
CREATE INDEX idx_generated_report_order ON generated_report(order_id) WHERE is_deleted = false;
CREATE INDEX idx_generated_report_status ON generated_report(report_status) WHERE is_deleted = false;
CREATE INDEX idx_generated_report_date ON generated_report(report_date DESC);
CREATE INDEX idx_generated_report_number ON generated_report(report_number);

-- ============================================================================
-- Digital Signature Table
-- ============================================================================

CREATE TABLE digital_signature (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES generated_report(id),
    organization_id UUID NOT NULL,

    -- Signatory information
    signatory_id UUID NOT NULL,      -- User ID who signed
    signatory_name VARCHAR(200) NOT NULL,
    signatory_role VARCHAR(100),
    signatory_designation VARCHAR(200),
    signatory_qualification VARCHAR(300),

    -- Signature details
    signature_type VARCHAR(50) NOT NULL, -- PATHOLOGIST, CONSULTANT, RADIOLOGIST, etc.
    signature_image_path VARCHAR(500),   -- Path to signature image
    digital_certificate TEXT,            -- Optional digital certificate

    -- Signature metadata
    signature_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    signature_location VARCHAR(200),
    signature_ip_address VARCHAR(50),
    signature_device_info JSONB,

    -- Verification
    signature_hash VARCHAR(128) NOT NULL, -- Hash of signed content
    verification_code VARCHAR(100),
    signature_status signature_status DEFAULT 'SIGNED',

    -- Comments and remarks
    signature_comments TEXT,

    -- Audit fields
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_digital_signature_report ON digital_signature(report_id) WHERE is_deleted = false;
CREATE INDEX idx_digital_signature_signatory ON digital_signature(signatory_id);
CREATE INDEX idx_digital_signature_timestamp ON digital_signature(signature_timestamp DESC);

-- ============================================================================
-- Report Delivery Table
-- ============================================================================

CREATE TABLE report_delivery (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES generated_report(id),
    organization_id UUID NOT NULL,

    -- Delivery details
    delivery_channel delivery_channel NOT NULL,
    recipient_name VARCHAR(200) NOT NULL,
    recipient_contact VARCHAR(100) NOT NULL, -- Email or phone number

    -- Delivery configuration
    subject VARCHAR(300),
    message TEXT,
    attachment_url VARCHAR(500),

    -- Status tracking
    delivery_status delivery_status DEFAULT 'PENDING',
    scheduled_at TIMESTAMP,
    sent_at TIMESTAMP,
    delivered_at TIMESTAMP,

    -- Provider details
    provider_name VARCHAR(100),      -- SendGrid, Twilio, etc.
    provider_message_id VARCHAR(200),
    provider_response JSONB,

    -- Error handling
    error_message TEXT,
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_report_delivery_report ON report_delivery(report_id) WHERE is_deleted = false;
CREATE INDEX idx_report_delivery_status ON report_delivery(delivery_status);
CREATE INDEX idx_report_delivery_channel ON report_delivery(delivery_channel);
CREATE INDEX idx_report_delivery_scheduled ON report_delivery(scheduled_at) WHERE delivery_status = 'PENDING';

-- ============================================================================
-- Report Access Log Table (for tracking who accessed reports)
-- ============================================================================

CREATE TABLE report_access_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES generated_report(id),

    -- Access details
    accessed_by UUID,                -- NULL if accessed via access code
    access_code_used VARCHAR(50),
    access_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Request details
    ip_address VARCHAR(50),
    user_agent TEXT,
    access_method VARCHAR(50),       -- VIEW, DOWNLOAD, PRINT

    -- Session info
    session_id VARCHAR(100),
    duration_seconds INTEGER
);

-- Indexes
CREATE INDEX idx_report_access_log_report ON report_access_log(report_id);
CREATE INDEX idx_report_access_log_user ON report_access_log(accessed_by);
CREATE INDEX idx_report_access_log_timestamp ON report_access_log(access_timestamp DESC);

-- ============================================================================
-- Functions
-- ============================================================================

-- Generate report number (format: RPT-YYYYMMDD-NNNN)
CREATE OR REPLACE FUNCTION generate_report_number()
RETURNS VARCHAR(50) AS $$
DECLARE
    today_date VARCHAR(8);
    sequence_num INTEGER;
    report_num VARCHAR(50);
BEGIN
    today_date := TO_CHAR(CURRENT_DATE, 'YYYYMMDD');

    -- Get the next sequence number for today
    SELECT COUNT(*) + 1 INTO sequence_num
    FROM generated_report
    WHERE report_number LIKE 'RPT-' || today_date || '-%';

    report_num := 'RPT-' || today_date || '-' || LPAD(sequence_num::TEXT, 4, '0');

    RETURN report_num;
END;
$$ LANGUAGE plpgsql;

-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers for updated_at
CREATE TRIGGER update_report_template_updated_at
    BEFORE UPDATE ON report_template
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_generated_report_updated_at
    BEFORE UPDATE ON generated_report
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Increment download count on access
CREATE OR REPLACE FUNCTION increment_download_count()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.access_method = 'DOWNLOAD' THEN
        UPDATE generated_report
        SET download_count = download_count + 1,
            last_downloaded_at = NEW.access_timestamp
        WHERE id = NEW.report_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER track_report_downloads
    AFTER INSERT ON report_access_log
    FOR EACH ROW
    EXECUTE FUNCTION increment_download_count();

-- ============================================================================
-- Sample Data
-- ============================================================================

-- Sample report template for patient test report
INSERT INTO report_template (
    id,
    organization_id,
    template_name,
    template_code,
    template_type,
    description,
    template_content,
    header_content,
    footer_content,
    fields_config,
    requires_signature,
    is_default,
    created_by
) VALUES (
    uuid_generate_v4(),
    uuid_generate_v4(), -- Replace with actual org ID
    'Standard Patient Test Report',
    'STD_PATIENT_REPORT',
    'PATIENT_REPORT',
    'Standard template for patient laboratory test reports with digital signature',
    '{"sections": ["patient_info", "test_results", "interpretation", "signatures"]}'::jsonb,
    '{"logo": true, "org_name": true, "accreditation_info": true}'::jsonb,
    '{"page_numbers": true, "generated_timestamp": true, "lab_info": true}'::jsonb,
    '{"patient_fields": ["name", "age", "gender", "contact"], "test_fields": ["test_name", "result", "unit", "reference_range", "method"]}'::jsonb,
    true,
    true,
    uuid_generate_v4()
);

-- Comments
COMMENT ON TABLE report_template IS 'Stores configurable report templates';
COMMENT ON TABLE generated_report IS 'Stores generated report instances';
COMMENT ON TABLE digital_signature IS 'Stores digital signatures for reports';
COMMENT ON TABLE report_delivery IS 'Tracks report delivery via various channels';
COMMENT ON TABLE report_access_log IS 'Logs all report access for audit trail';
