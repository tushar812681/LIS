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

CREATE TYPE invoice_status AS ENUM (
    'DRAFT',
    'PENDING',
    'PARTIALLY_PAID',
    'PAID',
    'OVERDUE',
    'CANCELLED',
    'REFUNDED'
);

CREATE TYPE payment_status AS ENUM (
    'PENDING',
    'SUCCESS',
    'FAILED',
    'REFUNDED',
    'CANCELLED'
);

CREATE TYPE payment_method AS ENUM (
    'CASH',
    'CARD',
    'UPI',
    'NET_BANKING',
    'CHEQUE',
    'INSURANCE',
    'CREDIT'
);

CREATE TYPE insurance_claim_status AS ENUM (
    'DRAFT',
    'SUBMITTED',
    'UNDER_REVIEW',
    'APPROVED',
    'PARTIALLY_APPROVED',
    'REJECTED',
    'SETTLED'
);

CREATE TYPE transaction AS ENUM (
    'INVOICE',
    'PAYMENT',
    'REFUND',
    'CREDIT_NOTE',
    'DEBIT_NOTE',
    'ADJUSTMENT'
);

-- ============================================================================
-- Invoice Table
-- ============================================================================

CREATE TABLE invoice (
    -- Identity
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    invoice_number VARCHAR(50) UNIQUE NOT NULL,

    -- Organization
    organization_id UUID NOT NULL,
    branch_id UUID,

    -- Patient
    patient_id UUID NOT NULL,
    patient_name VARCHAR(200),

    -- Order Reference
    order_id UUID NOT NULL,

    -- Invoice Details
    invoice_date DATE NOT NULL,
    due_date DATE,

    -- Amounts
    subtotal_amount DECIMAL(12, 2) NOT NULL DEFAULT 0,
    discount_amount DECIMAL(12, 2) DEFAULT 0,
    discount_percentage DECIMAL(5, 2) DEFAULT 0,
    taxable_amount DECIMAL(12, 2) NOT NULL DEFAULT 0,

    -- Tax Breakdown (for India GST)
    cgst_amount DECIMAL(12, 2) DEFAULT 0,
    sgst_amount DECIMAL(12, 2) DEFAULT 0,
    igst_amount DECIMAL(12, 2) DEFAULT 0,
    total_tax_amount DECIMAL(12, 2) DEFAULT 0,

    -- Final Amount
    total_amount DECIMAL(12, 2) NOT NULL DEFAULT 0,
    paid_amount DECIMAL(12, 2) DEFAULT 0,
    outstanding_amount DECIMAL(12, 2) DEFAULT 0,

    -- Status
    invoice_status invoice_status NOT NULL DEFAULT 'DRAFT',

    -- Insurance
    is_insurance_claim BOOLEAN DEFAULT FALSE,
    insurance_company_id UUID,
    insurance_claim_id UUID,
    insurance_covered_amount DECIMAL(12, 2) DEFAULT 0,
    patient_payable_amount DECIMAL(12, 2) DEFAULT 0,

    -- Payment Terms
    payment_terms TEXT,
    credit_period_days INTEGER DEFAULT 0,

    -- Notes
    notes TEXT,
    terms_and_conditions TEXT,

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
CREATE INDEX idx_invoice_number ON invoice(invoice_number) WHERE is_deleted = FALSE;
CREATE INDEX idx_invoice_org ON invoice(organization_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_invoice_patient ON invoice(patient_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_invoice_order ON invoice(order_id) WHERE is_deleted = FALSE;
CREATE INDEX idx_invoice_status ON invoice(invoice_status) WHERE is_deleted = FALSE;
CREATE INDEX idx_invoice_date ON invoice(invoice_date DESC) WHERE is_deleted = FALSE;
CREATE INDEX idx_invoice_due_date ON invoice(due_date) WHERE is_deleted = FALSE AND invoice_status IN ('PENDING', 'PARTIALLY_PAID');

-- ============================================================================
-- Invoice Item Table
-- ============================================================================

CREATE TABLE invoice_item (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    invoice_id UUID NOT NULL REFERENCES invoice(id) ON DELETE CASCADE,

    -- Item Details
    item_type VARCHAR(50) NOT NULL, -- TEST, PACKAGE, CONSULTATION, SAMPLE_COLLECTION
    item_id UUID, -- Reference to test/package
    item_code VARCHAR(50),
    item_name VARCHAR(200) NOT NULL,
    description TEXT,

    -- Quantity & Rate
    quantity INTEGER DEFAULT 1,
    unit_price DECIMAL(10, 2) NOT NULL,

    -- Discount
    discount_amount DECIMAL(10, 2) DEFAULT 0,
    discount_percentage DECIMAL(5, 2) DEFAULT 0,

    -- Tax
    tax_percentage DECIMAL(5, 2) DEFAULT 0,
    tax_amount DECIMAL(10, 2) DEFAULT 0,

    -- Amounts
    subtotal_amount DECIMAL(10, 2) NOT NULL,
    total_amount DECIMAL(10, 2) NOT NULL,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_invoice_item_invoice ON invoice_item(invoice_id);

-- ============================================================================
-- Payment Table
-- ============================================================================

CREATE TABLE payment (
    -- Identity
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_number VARCHAR(50) UNIQUE NOT NULL,

    -- Organization
    organization_id UUID NOT NULL,

    -- Invoice Reference
    invoice_id UUID NOT NULL REFERENCES invoice(id) ON DELETE RESTRICT,

    -- Patient
    patient_id UUID NOT NULL,

    -- Payment Details
    payment_date DATE NOT NULL,
    payment_time TIME NOT NULL,
    payment_method payment_method NOT NULL,

    -- Amount
    payment_amount DECIMAL(12, 2) NOT NULL,

    -- Payment Method Specific Details
    card_last_4_digits VARCHAR(4),
    card_type VARCHAR(20), -- VISA, MASTERCARD, RUPAY
    upi_transaction_id VARCHAR(100),
    transaction_reference VARCHAR(200),
    bank_name VARCHAR(200),
    cheque_number VARCHAR(50),
    cheque_date DATE,

    -- Status
    payment_status payment_status NOT NULL DEFAULT 'PENDING',

    -- Reconciliation
    is_reconciled BOOLEAN DEFAULT FALSE,
    reconciled_at TIMESTAMP,
    reconciled_by UUID,

    -- Gateway Details (for online payments)
    gateway_name VARCHAR(100),
    gateway_transaction_id VARCHAR(200),
    gateway_response JSONB,

    -- Notes
    notes TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    received_by UUID
);

-- Indexes
CREATE INDEX idx_payment_number ON payment(payment_number);
CREATE INDEX idx_payment_org ON payment(organization_id);
CREATE INDEX idx_payment_invoice ON payment(invoice_id);
CREATE INDEX idx_payment_patient ON payment(patient_id);
CREATE INDEX idx_payment_date ON payment(payment_date DESC);
CREATE INDEX idx_payment_status ON payment(payment_status);
CREATE INDEX idx_payment_method ON payment(payment_method);

-- ============================================================================
-- Insurance Company Table
-- ============================================================================

CREATE TABLE insurance_company (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Company Details
    company_name VARCHAR(200) NOT NULL,
    company_code VARCHAR(50) UNIQUE NOT NULL,

    -- Contact Information
    contact_person VARCHAR(200),
    email VARCHAR(255),
    phone VARCHAR(20),
    address TEXT,

    -- TPA Details
    tpa_name VARCHAR(200),
    tpa_id VARCHAR(100),

    -- Payment Terms
    credit_period_days INTEGER DEFAULT 30,
    payment_terms TEXT,

    -- Discount Agreement
    discount_percentage DECIMAL(5, 2) DEFAULT 0,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_insurance_company_org ON insurance_company(organization_id) WHERE is_active = TRUE;
CREATE INDEX idx_insurance_company_code ON insurance_company(company_code);

-- ============================================================================
-- Insurance Claim Table
-- ============================================================================

CREATE TABLE insurance_claim (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    claim_number VARCHAR(50) UNIQUE NOT NULL,

    -- Organization
    organization_id UUID NOT NULL,

    -- Insurance Company
    insurance_company_id UUID NOT NULL REFERENCES insurance_company(id),

    -- Patient
    patient_id UUID NOT NULL,
    patient_name VARCHAR(200),

    -- Policy Details
    policy_number VARCHAR(100) NOT NULL,
    policy_holder_name VARCHAR(200),
    sum_insured DECIMAL(12, 2),

    -- Claim Details
    claim_date DATE NOT NULL,
    claim_amount DECIMAL(12, 2) NOT NULL,
    approved_amount DECIMAL(12, 2) DEFAULT 0,
    settled_amount DECIMAL(12, 2) DEFAULT 0,
    rejected_amount DECIMAL(12, 2) DEFAULT 0,

    -- Status
    claim_status insurance_claim_status NOT NULL DEFAULT 'DRAFT',

    -- Submission
    submitted_date DATE,
    submitted_by UUID,

    -- Approval
    approval_date DATE,
    approval_reference VARCHAR(100),

    -- Settlement
    settlement_date DATE,
    settlement_reference VARCHAR(100),

    -- Rejection
    rejection_reason TEXT,

    -- Documents
    documents JSONB, -- Array of document URLs

    -- Notes
    notes TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

CREATE INDEX idx_claim_number ON insurance_claim(claim_number);
CREATE INDEX idx_claim_org ON insurance_claim(organization_id);
CREATE INDEX idx_claim_company ON insurance_claim(insurance_company_id);
CREATE INDEX idx_claim_patient ON insurance_claim(patient_id);
CREATE INDEX idx_claim_status ON insurance_claim(claim_status);
CREATE INDEX idx_claim_date ON insurance_claim(claim_date DESC);

-- ============================================================================
-- Credit Note Table
-- ============================================================================

CREATE TABLE credit_note (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    credit_note_number VARCHAR(50) UNIQUE NOT NULL,

    -- Organization
    organization_id UUID NOT NULL,

    -- Invoice Reference
    invoice_id UUID NOT NULL REFERENCES invoice(id),

    -- Patient
    patient_id UUID NOT NULL,

    -- Credit Note Details
    credit_date DATE NOT NULL,
    credit_amount DECIMAL(12, 2) NOT NULL,
    reason TEXT NOT NULL,

    -- Status
    is_applied BOOLEAN DEFAULT FALSE,
    applied_date DATE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_credit_note_org ON credit_note(organization_id);
CREATE INDEX idx_credit_note_invoice ON credit_note(invoice_id);
CREATE INDEX idx_credit_note_patient ON credit_note(patient_id);

-- ============================================================================
-- Discount Scheme Table
-- ============================================================================

CREATE TABLE discount_scheme (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Scheme Details
    scheme_name VARCHAR(200) NOT NULL,
    scheme_code VARCHAR(50) NOT NULL,
    description TEXT,

    -- Discount Configuration
    discount_percentage DECIMAL(5, 2) DEFAULT 0,
    discount_amount DECIMAL(10, 2) DEFAULT 0,
    is_percentage BOOLEAN DEFAULT TRUE,

    -- Applicability
    applicable_to VARCHAR(50), -- ALL, TESTS, PACKAGES, SPECIFIC_TESTS
    applicable_items JSONB, -- Array of test/package IDs

    -- Patient Category
    patient_categories JSONB, -- Array of categories (SENIOR_CITIZEN, STAFF, CORPORATE, etc.)

    -- Validity
    valid_from DATE,
    valid_to DATE,

    -- Limits
    max_discount_amount DECIMAL(10, 2),
    usage_limit INTEGER,
    usage_count INTEGER DEFAULT 0,

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_discount_scheme_org ON discount_scheme(organization_id) WHERE is_active = TRUE;
CREATE INDEX idx_discount_scheme_code ON discount_scheme(scheme_code);

-- ============================================================================
-- Transaction Ledger Table
-- ============================================================================

CREATE TABLE transaction_ledger (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Transaction Details
    transaction_date DATE NOT NULL,
    transaction_time TIME NOT NULL,
    transaction_type transaction NOT NULL,
    transaction_number VARCHAR(50) NOT NULL,

    -- Patient
    patient_id UUID NOT NULL,

    -- Reference
    reference_id UUID, -- Invoice, Payment, Credit Note ID
    reference_type VARCHAR(50),

    -- Amounts
    debit_amount DECIMAL(12, 2) DEFAULT 0,
    credit_amount DECIMAL(12, 2) DEFAULT 0,
    balance_amount DECIMAL(12, 2) DEFAULT 0,

    -- Description
    description TEXT,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    created_by UUID
);

CREATE INDEX idx_ledger_org ON transaction_ledger(organization_id);
CREATE INDEX idx_ledger_patient ON transaction_ledger(patient_id);
CREATE INDEX idx_ledger_date ON transaction_ledger(transaction_date DESC);
CREATE INDEX idx_ledger_type ON transaction_ledger(transaction_type);

-- ============================================================================
-- Functions
-- ============================================================================

-- Generate Invoice Number
CREATE OR REPLACE FUNCTION generate_invoice_number()
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
BEGIN
    sequence_num := nextval('invoice_sequence');
    base_id := 'INV-' || TO_CHAR(CURRENT_DATE, 'YYYYMM') || '-' || LPAD(sequence_num::TEXT, 5, '0');
    RETURN base_id;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS invoice_sequence START 1;

-- Generate Payment Number
CREATE OR REPLACE FUNCTION generate_payment_number()
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
BEGIN
    sequence_num := nextval('payment_sequence');
    base_id := 'PAY-' || TO_CHAR(CURRENT_DATE, 'YYYYMMDD') || '-' || LPAD(sequence_num::TEXT, 5, '0');
    RETURN base_id;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS payment_sequence START 1;

-- Generate Claim Number
CREATE OR REPLACE FUNCTION generate_claim_number()
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
BEGIN
    sequence_num := nextval('claim_sequence');
    base_id := 'CLM-' || TO_CHAR(CURRENT_DATE, 'YYYYMM') || '-' || LPAD(sequence_num::TEXT, 5, '0');
    RETURN base_id;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS claim_sequence START 1;

-- Generate Credit Note Number
CREATE OR REPLACE FUNCTION generate_credit_note_number()
RETURNS VARCHAR AS $$
DECLARE
    sequence_num BIGINT;
    base_id VARCHAR;
BEGIN
    sequence_num := nextval('credit_note_sequence');
    base_id := 'CN-' || TO_CHAR(CURRENT_DATE, 'YYYYMM') || '-' || LPAD(sequence_num::TEXT, 5, '0');
    RETURN base_id;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE IF NOT EXISTS credit_note_sequence START 1;

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_invoice_updated_at
    BEFORE UPDATE ON invoice
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_payment_updated_at
    BEFORE UPDATE ON payment
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_insurance_claim_updated_at
    BEFORE UPDATE ON insurance_claim
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Update invoice status when payment is made
CREATE OR REPLACE FUNCTION update_invoice_on_payment()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.payment_status = 'SUCCESS' THEN
        -- Update paid amount and outstanding amount
        UPDATE invoice
        SET paid_amount = paid_amount + NEW.payment_amount,
            outstanding_amount = total_amount - (paid_amount + NEW.payment_amount),
            invoice_status = CASE
                WHEN (paid_amount + NEW.payment_amount) >= total_amount THEN 'PAID'::invoice_status
                WHEN (paid_amount + NEW.payment_amount) > 0 THEN 'PARTIALLY_PAID'::invoice_status
                ELSE invoice_status
            END
        WHERE id = NEW.invoice_id;

        -- Create ledger entry
        INSERT INTO transaction_ledger (
            organization_id, transaction_date, transaction_time,
            transaction_type, transaction_number,
            patient_id, reference_id, reference_type,
            credit_amount, description
        )
        VALUES (
            NEW.organization_id, NEW.payment_date, NEW.payment_time,
            'PAYMENT', NEW.payment_number,
            NEW.patient_id, NEW.id, 'PAYMENT',
            NEW.payment_amount,
            'Payment received via ' || NEW.payment_method::TEXT
        );
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_invoice_payment_trigger
    AFTER INSERT OR UPDATE ON payment
    FOR EACH ROW
    EXECUTE FUNCTION update_invoice_on_payment();

-- Check overdue invoices
CREATE OR REPLACE FUNCTION check_overdue_invoices()
RETURNS void AS $$
BEGIN
    UPDATE invoice
    SET invoice_status = 'OVERDUE'
    WHERE due_date < CURRENT_DATE
      AND invoice_status IN ('PENDING', 'PARTIALLY_PAID')
      AND is_deleted = FALSE;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Sample Data
-- ============================================================================

-- Sample Insurance Company
INSERT INTO insurance_company (
    id, organization_id, company_name, company_code,
    contact_person, email, phone,
    credit_period_days, is_active
) VALUES (
    uuid_generate_v4(),
    (SELECT id FROM organization LIMIT 1),
    'Star Health Insurance',
    'STAR-001',
    'Claims Manager',
    'claims@starhealth.com',
    '+91-9876543210',
    45,
    TRUE
);

-- Sample Discount Scheme
INSERT INTO discount_scheme (
    id, organization_id, scheme_name, scheme_code,
    description, discount_percentage, is_percentage,
    applicable_to, valid_from, valid_to, is_active
) VALUES (
    uuid_generate_v4(),
    (SELECT id FROM organization LIMIT 1),
    'Senior Citizen Discount',
    'SENIOR-10',
    '10% discount for patients above 60 years',
    10.0,
    TRUE,
    'ALL',
    CURRENT_DATE,
    CURRENT_DATE + INTERVAL '1 year',
    TRUE
);
