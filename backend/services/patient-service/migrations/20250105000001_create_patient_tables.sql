-- Create extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create custom types (no _type suffix for Rust compatibility)
CREATE TYPE gender AS ENUM ('MALE', 'FEMALE', 'OTHER', 'PREFER_NOT_TO_SAY');
CREATE TYPE language AS ENUM ('en', 'hi', 'ta', 'te', 'kn', 'bn', 'mr');
CREATE TYPE communication_channel AS ENUM ('WHATSAPP', 'SMS', 'EMAIL', 'PORTAL', 'PUSH_NOTIFICATION');
CREATE TYPE registration_source AS ENUM ('WALK_IN', 'WEB_PORTAL', 'MOBILE_APP', 'WHATSAPP', 'ABDM', 'IMPORT');

-- Organization table (referenced by patient)
CREATE TABLE IF NOT EXISTS organization (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User table (for created_by, updated_by references)
CREATE TABLE IF NOT EXISTS "user" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Patient table
CREATE TABLE patient (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mrn_number VARCHAR(50) UNIQUE NOT NULL,
    organization_id UUID NOT NULL REFERENCES organization(id),

    -- Demographics
    salutation VARCHAR(10),
    first_name VARCHAR(100) NOT NULL,
    middle_name VARCHAR(100),
    last_name VARCHAR(100),
    full_name VARCHAR(300) NOT NULL,
    date_of_birth DATE NOT NULL,
    age INTEGER NOT NULL,
    gender gender NOT NULL,
    blood_group VARCHAR(10),

    -- Identity
    aadhaar_number VARCHAR(255), -- Encrypted
    aadhaar_verified BOOLEAN DEFAULT FALSE,
    pan_number VARCHAR(10),
    passport_number VARCHAR(20),
    abdm_health_id VARCHAR(100),
    abdm_phr_address VARCHAR(100),

    -- Contact
    mobile_number VARCHAR(20) NOT NULL,
    alternate_mobile VARCHAR(20),
    email VARCHAR(255),
    preferred_language language DEFAULT 'en',
    preferred_communication communication_channel DEFAULT 'WHATSAPP',

    -- Additional
    occupation VARCHAR(100),
    marital_status VARCHAR(20),
    nationality VARCHAR(50) DEFAULT 'Indian',
    profile_photo_url TEXT,

    -- Metadata
    registration_source registration_source NOT NULL,
    registration_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_by UUID REFERENCES "user"(id),
    updated_by UUID REFERENCES "user"(id),
    is_active BOOLEAN DEFAULT TRUE,
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP WITH TIME ZONE,

    CONSTRAINT valid_age CHECK (age >= 0 AND age <= 150),
    CONSTRAINT valid_dob CHECK (date_of_birth <= CURRENT_DATE)
);

-- Patient Address table
CREATE TABLE patient_address (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
    address_type VARCHAR(20) NOT NULL, -- HOME, WORK, TEMPORARY
    is_primary BOOLEAN DEFAULT FALSE,

    address_line1 VARCHAR(500) NOT NULL,
    address_line2 VARCHAR(500),
    landmark VARCHAR(200),
    city VARCHAR(100) NOT NULL,
    district VARCHAR(100),
    state VARCHAR(100) NOT NULL,
    country VARCHAR(100) DEFAULT 'India',
    pincode VARCHAR(10) NOT NULL,

    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Patient Consent table
CREATE TABLE patient_consent (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
    consent_type VARCHAR(50) NOT NULL, -- DATA_PROCESSING, COMMUNICATION, MARKETING, DATA_SHARING, RESEARCH
    status VARCHAR(20) NOT NULL, -- GRANTED, WITHDRAWN, EXPIRED
    granted_at TIMESTAMP WITH TIME ZONE NOT NULL,
    withdrawn_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    consent_text TEXT NOT NULL,
    consent_version VARCHAR(20) NOT NULL,
    ip_address VARCHAR(45),
    device_info TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Patient Contact Person table
CREATE TABLE patient_contact_person (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,

    name VARCHAR(200) NOT NULL,
    relationship VARCHAR(50) NOT NULL,
    mobile_number VARCHAR(20) NOT NULL,
    email VARCHAR(255),
    is_emergency_contact BOOLEAN DEFAULT FALSE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Patient Insurance table
CREATE TABLE patient_insurance (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,

    insurance_company VARCHAR(200) NOT NULL,
    policy_number VARCHAR(100) NOT NULL,
    policy_holder_name VARCHAR(200) NOT NULL,
    relationship_to_patient VARCHAR(50) NOT NULL, -- SELF, SPOUSE, PARENT, etc.
    valid_from DATE NOT NULL,
    valid_to DATE NOT NULL,
    coverage_amount DECIMAL(12, 2),
    copay_percentage DECIMAL(5, 2),

    tpa_name VARCHAR(200),
    tpa_id VARCHAR(100),
    card_front_url TEXT,
    card_back_url TEXT,

    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT valid_policy_dates CHECK (valid_to >= valid_from)
);

-- Patient Medical History table
CREATE TABLE patient_medical_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,

    chronic_conditions TEXT[], -- Array of conditions
    allergies TEXT[],
    current_medications TEXT[],
    previous_surgeries TEXT[],
    family_history TEXT,

    -- Lifestyle
    smoking_status VARCHAR(20), -- NEVER, FORMER, CURRENT
    alcohol_consumption VARCHAR(20), -- NEVER, OCCASIONAL, FREQUENT

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_patient_mrn ON patient(mrn_number);
CREATE INDEX idx_patient_mobile ON patient(mobile_number);
CREATE INDEX idx_patient_organization ON patient(organization_id);
CREATE INDEX idx_patient_active ON patient(is_active, is_deleted);
CREATE INDEX idx_patient_created_at ON patient(created_at);

-- Full-text search index for patient names
CREATE INDEX idx_patient_name_fts ON patient USING gin(to_tsvector('english', full_name));

-- Indexes for addresses
CREATE INDEX idx_patient_address_patient ON patient_address(patient_id);
CREATE INDEX idx_patient_address_primary ON patient_address(patient_id, is_primary) WHERE is_primary = TRUE;
CREATE UNIQUE INDEX unique_primary_address ON patient_address(patient_id, is_primary) WHERE is_primary = TRUE;

-- Indexes for consents
CREATE INDEX idx_patient_consent_patient ON patient_consent(patient_id);
CREATE INDEX idx_patient_consent_status ON patient_consent(status);

-- Indexes for contact persons
CREATE INDEX idx_patient_contact_patient ON patient_contact_person(patient_id);

-- Indexes for insurance
CREATE INDEX idx_patient_insurance_patient ON patient_insurance(patient_id);
CREATE INDEX idx_patient_insurance_active ON patient_insurance(is_active);

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_patient_updated_at BEFORE UPDATE ON patient
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_patient_address_updated_at BEFORE UPDATE ON patient_address
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_patient_contact_updated_at BEFORE UPDATE ON patient_contact_person
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_patient_insurance_updated_at BEFORE UPDATE ON patient_insurance
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_patient_medical_history_updated_at BEFORE UPDATE ON patient_medical_history
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default organization for testing
INSERT INTO organization (id, name, code)
VALUES ('00000000-0000-0000-0000-000000000001', 'Default Lab', 'DEFAULT')
ON CONFLICT DO NOTHING;

-- Insert default user for testing
INSERT INTO "user" (id, email, name)
VALUES ('00000000-0000-0000-0000-000000000001', 'admin@lab.com', 'System Admin')
ON CONFLICT DO NOTHING;
