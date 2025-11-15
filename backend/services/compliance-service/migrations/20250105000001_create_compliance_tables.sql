-- ============================================================================
-- COMPLIANCE SERVICE DATABASE SCHEMA
-- ISO 15189:2022 NABL Compliance Management
-- ============================================================================

-- ============================================================================
-- Organization Stub Table (for foreign key reference)
-- ============================================================================

CREATE TABLE IF NOT EXISTS organization (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert default organization for testing
INSERT INTO organization (id, name, code)
VALUES ('00000000-0000-0000-0000-000000000001', 'Default Lab', 'DEFAULT')
ON CONFLICT DO NOTHING;

-- ============================================================================
-- AUDIT LOG
-- Complete audit trail for all system actions
-- ============================================================================

CREATE TABLE IF NOT EXISTS audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    user_id UUID,
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL, -- CREATE, UPDATE, DELETE, APPROVE, REJECT, VIEW
    old_value JSONB,
    new_value JSONB,
    changes JSONB,
    reason TEXT,
    ip_address VARCHAR(45),
    user_agent TEXT,
    session_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT audit_log_organization_fk FOREIGN KEY (organization_id) REFERENCES organization(id) ON DELETE CASCADE
);

CREATE INDEX idx_audit_log_organization ON audit_log(organization_id);
CREATE INDEX idx_audit_log_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_log_user ON audit_log(user_id);
CREATE INDEX idx_audit_log_created ON audit_log(created_at DESC);
CREATE INDEX idx_audit_log_action ON audit_log(action);

-- ============================================================================
-- DOCUMENT CONTROL
-- NABL-compliant document management with version control
-- ============================================================================

CREATE TYPE document AS ENUM (
    'SOP', 'POLICY', 'PROCEDURE', 'FORM', 'MANUAL',
    'WORK_INSTRUCTION', 'SPECIFICATION', 'CERTIFICATE', 'OTHER'
);

CREATE TYPE document_status AS ENUM (
    'DRAFT', 'UNDER_REVIEW', 'APPROVED', 'PUBLISHED',
    'OBSOLETE', 'ARCHIVED'
);

CREATE TABLE IF NOT EXISTS document_control (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    document_number VARCHAR(100) NOT NULL,
    document_type document NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    version VARCHAR(20) NOT NULL DEFAULT '1.0',
    revision_number INT NOT NULL DEFAULT 1,
    document_status document_status NOT NULL DEFAULT 'DRAFT',

    -- Content
    file_path TEXT,
    file_size BIGINT,
    file_mime_type VARCHAR(100),
    content_hash VARCHAR(128),

    -- Approval workflow
    author_id UUID NOT NULL,
    reviewer_id UUID,
    approver_id UUID,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    approved_at TIMESTAMP WITH TIME ZONE,
    published_at TIMESTAMP WITH TIME ZONE,
    effective_date DATE,
    expiry_date DATE,
    next_review_date DATE,

    -- Metadata
    department VARCHAR(100),
    applicable_to TEXT[], -- Array of roles/departments
    keywords TEXT[],
    related_documents UUID[],
    supersedes_document_id UUID,

    -- Tracking
    view_count INT DEFAULT 0,
    last_viewed_at TIMESTAMP WITH TIME ZONE,
    acknowledgement_required BOOLEAN DEFAULT FALSE,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE,

    CONSTRAINT doc_control_organization_fk FOREIGN KEY (organization_id) REFERENCES organization(id) ON DELETE CASCADE,
    CONSTRAINT doc_control_unique_number UNIQUE (organization_id, document_number, version)
);

CREATE INDEX idx_doc_control_organization ON document_control(organization_id);
CREATE INDEX idx_doc_control_number ON document_control(document_number);
CREATE INDEX idx_doc_control_status ON document_control(document_status);
CREATE INDEX idx_doc_control_type ON document_control(document_type);
CREATE INDEX idx_doc_control_effective ON document_control(effective_date);
CREATE INDEX idx_doc_control_review ON document_control(next_review_date);

-- ============================================================================
-- DOCUMENT ACKNOWLEDGEMENT
-- Track user acknowledgements of controlled documents
-- ============================================================================

CREATE TABLE IF NOT EXISTS document_acknowledgement (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL,
    user_id UUID NOT NULL,
    acknowledged_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    version_acknowledged VARCHAR(20) NOT NULL,
    ip_address VARCHAR(45),

    CONSTRAINT doc_ack_document_fk FOREIGN KEY (document_id) REFERENCES document_control(id) ON DELETE CASCADE,
    CONSTRAINT doc_ack_unique UNIQUE (document_id, user_id, version_acknowledged)
);

CREATE INDEX idx_doc_ack_document ON document_acknowledgement(document_id);
CREATE INDEX idx_doc_ack_user ON document_acknowledgement(user_id);

-- ============================================================================
-- CAPA (Corrective and Preventive Actions)
-- ISO 15189:2022 CAPA Management
-- ============================================================================

CREATE TYPE capa_type AS ENUM ('CORRECTIVE', 'PREVENTIVE', 'IMPROVEMENT');

CREATE TYPE capa_status AS ENUM (
    'OPEN', 'INVESTIGATION', 'ACTION_PLAN', 'IMPLEMENTATION',
    'VERIFICATION', 'CLOSED', 'CANCELLED'
);

CREATE TYPE capa_priority AS ENUM ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL');

CREATE TABLE IF NOT EXISTS capa (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    capa_number VARCHAR(50) NOT NULL,
    capa_type capa_type NOT NULL,
    priority capa_priority NOT NULL,
    capa_status capa_status NOT NULL DEFAULT 'OPEN',

    -- Problem description
    title VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    source VARCHAR(100), -- AUDIT, COMPLAINT, INCIDENT, QC_FAILURE, etc.
    source_reference VARCHAR(200),
    date_identified DATE NOT NULL,

    -- Root cause analysis
    root_cause_analysis TEXT,
    root_cause_identified_date DATE,

    -- Action plan
    corrective_action TEXT,
    preventive_action TEXT,
    action_plan TEXT,
    assigned_to UUID,
    target_completion_date DATE,
    actual_completion_date DATE,

    -- Verification
    verification_method TEXT,
    verified_by UUID,
    verification_date DATE,
    verification_result TEXT,
    effectiveness_check BOOLEAN DEFAULT FALSE,
    effectiveness_check_date DATE,

    -- Closure
    closed_by UUID,
    closed_at TIMESTAMP WITH TIME ZONE,
    closure_remarks TEXT,

    -- Attachments and related items
    related_documents UUID[],
    attachments JSONB DEFAULT '[]',

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE,

    CONSTRAINT capa_organization_fk FOREIGN KEY (organization_id) REFERENCES organization(id) ON DELETE CASCADE,
    CONSTRAINT capa_unique_number UNIQUE (organization_id, capa_number)
);

CREATE INDEX idx_capa_organization ON capa(organization_id);
CREATE INDEX idx_capa_number ON capa(capa_number);
CREATE INDEX idx_capa_status ON capa(capa_status);
CREATE INDEX idx_capa_priority ON capa(priority);
CREATE INDEX idx_capa_assigned ON capa(assigned_to);
CREATE INDEX idx_capa_target_date ON capa(target_completion_date);

-- ============================================================================
-- TRAINING RECORDS
-- Staff competency and training management
-- ============================================================================

CREATE TYPE training AS ENUM (
    'ORIENTATION', 'TECHNICAL', 'SAFETY', 'QUALITY',
    'COMPLIANCE', 'SOFTWARE', 'EQUIPMENT', 'REFRESHER'
);

CREATE TYPE training_status AS ENUM (
    'SCHEDULED', 'IN_PROGRESS', 'COMPLETED', 'FAILED', 'CANCELLED'
);

CREATE TABLE IF NOT EXISTS training_record (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    user_id UUID NOT NULL,
    training_type training NOT NULL,
    training_status training_status NOT NULL DEFAULT 'SCHEDULED',

    -- Training details
    training_title VARCHAR(500) NOT NULL,
    training_description TEXT,
    trainer_name VARCHAR(200),
    trainer_id UUID,
    training_method VARCHAR(100), -- CLASSROOM, ONLINE, ON_JOB, SELF_STUDY

    -- Scheduling
    scheduled_date DATE,
    training_start_date DATE,
    training_end_date DATE,
    duration_hours DECIMAL(5,2),

    -- Assessment
    assessment_required BOOLEAN DEFAULT FALSE,
    assessment_score DECIMAL(5,2),
    passing_score DECIMAL(5,2) DEFAULT 70.0,
    assessment_date DATE,
    assessment_result VARCHAR(50), -- PASS, FAIL, NOT_ASSESSED

    -- Certification
    certificate_issued BOOLEAN DEFAULT FALSE,
    certificate_number VARCHAR(100),
    certificate_issued_date DATE,
    certificate_expiry_date DATE,

    -- Competency
    competency_achieved BOOLEAN DEFAULT FALSE,
    competency_assessor_id UUID,
    competency_assessment_date DATE,
    competency_remarks TEXT,

    -- Documentation
    training_materials JSONB DEFAULT '[]',
    attendance_record JSONB,
    certificates JSONB DEFAULT '[]',

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT training_organization_fk FOREIGN KEY (organization_id) REFERENCES organization(id) ON DELETE CASCADE
);

CREATE INDEX idx_training_organization ON training_record(organization_id);
CREATE INDEX idx_training_user ON training_record(user_id);
CREATE INDEX idx_training_status ON training_record(training_status);
CREATE INDEX idx_training_type ON training_record(training_type);
CREATE INDEX idx_training_scheduled ON training_record(scheduled_date);
CREATE INDEX idx_training_expiry ON training_record(certificate_expiry_date);

-- ============================================================================
-- QUALITY INDICATORS
-- Quality metrics and KPI tracking for NABL compliance
-- ============================================================================

CREATE TYPE indicator_category AS ENUM (
    'PRE_ANALYTICAL', 'ANALYTICAL', 'POST_ANALYTICAL',
    'CUSTOMER_SERVICE', 'SAFETY', 'TURNAROUND_TIME'
);

CREATE TABLE IF NOT EXISTS quality_indicator (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    indicator_name VARCHAR(200) NOT NULL,
    indicator_category indicator_category NOT NULL,
    description TEXT,

    -- Measurement
    measurement_unit VARCHAR(50),
    target_value DECIMAL(10,2),
    threshold_warning DECIMAL(10,2),
    threshold_critical DECIMAL(10,2),
    calculation_method TEXT,

    -- Tracking
    measurement_frequency VARCHAR(50), -- DAILY, WEEKLY, MONTHLY, QUARTERLY
    responsible_person_id UUID,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT qi_organization_fk FOREIGN KEY (organization_id) REFERENCES organization(id) ON DELETE CASCADE
);

CREATE INDEX idx_qi_organization ON quality_indicator(organization_id);
CREATE INDEX idx_qi_category ON quality_indicator(indicator_category);
CREATE INDEX idx_qi_active ON quality_indicator(is_active);

-- ============================================================================
-- QUALITY INDICATOR VALUES
-- Historical tracking of quality indicator measurements
-- ============================================================================

CREATE TYPE indicator_status AS ENUM ('ON_TARGET', 'WARNING', 'CRITICAL');

CREATE TABLE IF NOT EXISTS quality_indicator_value (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    indicator_id UUID NOT NULL,
    organization_id UUID NOT NULL,
    measurement_date DATE NOT NULL,
    measured_value DECIMAL(10,2) NOT NULL,
    indicator_status indicator_status NOT NULL,

    -- Analysis
    analysis_notes TEXT,
    action_taken TEXT,

    -- Audit fields
    measured_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT qiv_indicator_fk FOREIGN KEY (indicator_id) REFERENCES quality_indicator(id) ON DELETE CASCADE,
    CONSTRAINT qiv_organization_fk FOREIGN KEY (organization_id) REFERENCES organization(id) ON DELETE CASCADE,
    CONSTRAINT qiv_unique_measurement UNIQUE (indicator_id, measurement_date)
);

CREATE INDEX idx_qiv_indicator ON quality_indicator_value(indicator_id);
CREATE INDEX idx_qiv_organization ON quality_indicator_value(organization_id);
CREATE INDEX idx_qiv_date ON quality_indicator_value(measurement_date DESC);
CREATE INDEX idx_qiv_status ON quality_indicator_value(indicator_status);

-- ============================================================================
-- COMPLIANCE CHECKLIST
-- Compliance assessment and audit checklists
-- ============================================================================

CREATE TYPE checklist AS ENUM (
    'INTERNAL_AUDIT', 'EXTERNAL_AUDIT', 'NABL_ASSESSMENT',
    'DAILY_CHECK', 'MONTHLY_CHECK', 'EQUIPMENT_CHECK', 'PROCESS_CHECK'
);

CREATE TABLE IF NOT EXISTS compliance_checklist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    checklist_name VARCHAR(500) NOT NULL,
    checklist_type checklist NOT NULL,
    description TEXT,

    -- Checklist items
    items JSONB NOT NULL, -- Array of checklist items with criteria

    -- Scheduling
    frequency VARCHAR(50), -- DAILY, WEEKLY, MONTHLY, QUARTERLY, ANNUAL, ADHOC
    is_active BOOLEAN DEFAULT TRUE,

    -- Assignment
    responsible_role VARCHAR(100),

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT checklist_organization_fk FOREIGN KEY (organization_id) REFERENCES organization(id) ON DELETE CASCADE
);

CREATE INDEX idx_checklist_organization ON compliance_checklist(organization_id);
CREATE INDEX idx_checklist_type ON compliance_checklist(checklist_type);
CREATE INDEX idx_checklist_active ON compliance_checklist(is_active);

-- ============================================================================
-- COMPLIANCE ASSESSMENT
-- Completed compliance assessments
-- ============================================================================

CREATE TYPE assessment_status AS ENUM ('IN_PROGRESS', 'COMPLETED', 'REVIEWED', 'APPROVED');

CREATE TABLE IF NOT EXISTS compliance_assessment (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    checklist_id UUID NOT NULL,
    organization_id UUID NOT NULL,
    assessment_date DATE NOT NULL,
    assessment_status assessment_status NOT NULL DEFAULT 'IN_PROGRESS',

    -- Scores
    total_items INT NOT NULL,
    items_passed INT DEFAULT 0,
    items_failed INT DEFAULT 0,
    items_na INT DEFAULT 0,
    compliance_score DECIMAL(5,2),

    -- Results
    assessment_results JSONB NOT NULL, -- Detailed results for each item
    findings TEXT,
    recommendations TEXT,

    -- Assessment team
    assessor_id UUID NOT NULL,
    reviewed_by UUID,
    approved_by UUID,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    approved_at TIMESTAMP WITH TIME ZONE,

    -- Follow-up
    capa_required BOOLEAN DEFAULT FALSE,
    capa_ids UUID[],

    -- Documentation
    attachments JSONB DEFAULT '[]',

    -- Audit fields
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT assessment_checklist_fk FOREIGN KEY (checklist_id) REFERENCES compliance_checklist(id) ON DELETE CASCADE,
    CONSTRAINT assessment_organization_fk FOREIGN KEY (organization_id) REFERENCES organization(id) ON DELETE CASCADE
);

CREATE INDEX idx_assessment_checklist ON compliance_assessment(checklist_id);
CREATE INDEX idx_assessment_organization ON compliance_assessment(organization_id);
CREATE INDEX idx_assessment_date ON compliance_assessment(assessment_date DESC);
CREATE INDEX idx_assessment_status ON compliance_assessment(assessment_status);

-- ============================================================================
-- SEED DATA: Default Quality Indicators for NABL Labs
-- ============================================================================

-- Note: These will be inserted by the application on first run for each organization
-- Examples of NABL-required quality indicators:
-- - Pre-analytical: Sample rejection rate, Sample identification errors
-- - Analytical: QC failure rate, Result critical values
-- - Post-analytical: TAT compliance, Report amendment rate
-- - Customer Service: Complaint resolution time, Customer satisfaction score

-- ============================================================================
-- TRIGGERS FOR UPDATED_AT
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_document_control_updated_at BEFORE UPDATE ON document_control
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_capa_updated_at BEFORE UPDATE ON capa
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_training_record_updated_at BEFORE UPDATE ON training_record
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_quality_indicator_updated_at BEFORE UPDATE ON quality_indicator
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_compliance_assessment_updated_at BEFORE UPDATE ON compliance_assessment
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
