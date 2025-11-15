-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Notification Service Schema
-- ============================================================================

-- Notification channels
CREATE TYPE notification_channel AS ENUM (
    'EMAIL',
    'SMS',
    'WHATSAPP',
    'PUSH',
    'IN_APP'
);

-- Notification status
CREATE TYPE notification_status AS ENUM (
    'PENDING',
    'QUEUED',
    'SENDING',
    'SENT',
    'DELIVERED',
    'READ',
    'FAILED',
    'BOUNCED'
);

-- Notification priority
CREATE TYPE notification_priority AS ENUM (
    'LOW',
    'NORMAL',
    'HIGH',
    'URGENT'
);

-- Template types
CREATE TYPE template AS ENUM (
    'APPOINTMENT_REMINDER',
    'TEST_RESULT_READY',
    'INVOICE_GENERATED',
    'PAYMENT_RECEIVED',
    'REPORT_DELIVERY',
    'QC_ALERT',
    'EQUIPMENT_MAINTENANCE',
    'STOCK_ALERT',
    'CUSTOM'
);

-- ============================================================================
-- Notification Template Table
-- ============================================================================

CREATE TABLE notification_template (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Template identification
    template_name VARCHAR(200) NOT NULL,
    template_code VARCHAR(100) UNIQUE NOT NULL,
    template_type template NOT NULL,
    description TEXT,

    -- Channel-specific content
    email_subject VARCHAR(300),
    email_body TEXT,
    email_html_body TEXT,

    sms_content TEXT,
    sms_length INTEGER,

    whatsapp_content TEXT,
    whatsapp_template_id VARCHAR(200),

    push_title VARCHAR(200),
    push_body TEXT,

    -- Template variables (JSONB)
    variables JSONB,              -- e.g., ["patient_name", "test_name", "report_url"]

    -- Settings
    supported_channels notification_channel[] NOT NULL,
    default_channel notification_channel,
    is_active BOOLEAN DEFAULT true,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false,

    CONSTRAINT notification_template_org_code_unique UNIQUE (organization_id, template_code)
);

-- Indexes
CREATE INDEX idx_notification_template_org ON notification_template(organization_id) WHERE is_deleted = false;
CREATE INDEX idx_notification_template_type ON notification_template(template_type) WHERE is_deleted = false;
CREATE INDEX idx_notification_template_active ON notification_template(is_active) WHERE is_deleted = false;

-- ============================================================================
-- Notification Table
-- ============================================================================

CREATE TABLE notification (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,
    template_id UUID REFERENCES notification_template(id),

    -- Recipient information
    recipient_id UUID,             -- User/Patient ID
    recipient_type VARCHAR(50),    -- USER, PATIENT, STAFF
    recipient_name VARCHAR(200),
    recipient_contact VARCHAR(200) NOT NULL,

    -- Notification details
    notification_channel notification_channel NOT NULL,
    notification_priority notification_priority DEFAULT 'NORMAL',

    -- Content
    subject VARCHAR(300),
    content TEXT NOT NULL,
    html_content TEXT,

    -- Template data
    template_data JSONB,           -- Data used to populate template

    -- Scheduling
    scheduled_at TIMESTAMP,
    sent_at TIMESTAMP,
    delivered_at TIMESTAMP,
    read_at TIMESTAMP,

    -- Status tracking
    notification_status notification_status DEFAULT 'PENDING',
    status_message TEXT,

    -- Provider details
    provider_name VARCHAR(100),    -- Twilio, SendGrid, WhatsApp Business API, FCM
    provider_message_id VARCHAR(200),
    provider_response JSONB,

    -- Retry mechanism
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    last_retry_at TIMESTAMP,

    -- Reference tracking
    reference_type VARCHAR(50),    -- ORDER, RESULT, INVOICE, REPORT, etc.
    reference_id UUID,

    -- Metadata
    metadata JSONB,
    tags VARCHAR(100)[],

    -- Audit fields
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_notification_org ON notification(organization_id) WHERE is_deleted = false;
CREATE INDEX idx_notification_recipient ON notification(recipient_id) WHERE is_deleted = false;
CREATE INDEX idx_notification_status ON notification(notification_status) WHERE is_deleted = false;
CREATE INDEX idx_notification_channel ON notification(notification_channel);
CREATE INDEX idx_notification_scheduled ON notification(scheduled_at) WHERE notification_status = 'PENDING';
CREATE INDEX idx_notification_pending ON notification(notification_status, created_at)
    WHERE notification_status IN ('PENDING', 'QUEUED') AND is_deleted = false;
CREATE INDEX idx_notification_reference ON notification(reference_type, reference_id) WHERE is_deleted = false;

-- ============================================================================
-- Notification Preference Table (User/Patient preferences)
-- ============================================================================

CREATE TABLE notification_preference (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,
    user_id UUID NOT NULL,

    -- Channel preferences
    email_enabled BOOLEAN DEFAULT true,
    sms_enabled BOOLEAN DEFAULT true,
    whatsapp_enabled BOOLEAN DEFAULT true,
    push_enabled BOOLEAN DEFAULT true,
    in_app_enabled BOOLEAN DEFAULT true,

    -- Contact information
    email_address VARCHAR(200),
    phone_number VARCHAR(50),
    whatsapp_number VARCHAR(50),
    push_token VARCHAR(500),

    -- Template-specific preferences (JSONB)
    template_preferences JSONB,    -- Override channel for specific templates

    -- Quiet hours
    quiet_hours_enabled BOOLEAN DEFAULT false,
    quiet_hours_start TIME,
    quiet_hours_end TIME,

    -- Frequency limits
    max_notifications_per_day INTEGER,
    max_notifications_per_hour INTEGER,

    -- Audit fields
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,

    CONSTRAINT notification_preference_org_user_unique UNIQUE (organization_id, user_id)
);

-- Indexes
CREATE INDEX idx_notification_preference_org ON notification_preference(organization_id);
CREATE INDEX idx_notification_preference_user ON notification_preference(user_id);

-- ============================================================================
-- Notification Queue Table (for batch processing)
-- ============================================================================

CREATE TABLE notification_queue (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Queue details
    queue_name VARCHAR(100) NOT NULL,
    batch_id UUID,

    -- Notification reference
    notification_id UUID REFERENCES notification(id),

    -- Priority and scheduling
    priority INTEGER DEFAULT 5,    -- 1 (highest) to 10 (lowest)
    scheduled_for TIMESTAMP NOT NULL,

    -- Processing tracking
    is_processed BOOLEAN DEFAULT false,
    processed_at TIMESTAMP,
    processing_started_at TIMESTAMP,
    processing_by VARCHAR(100),    -- Worker ID

    -- Error tracking
    error_count INTEGER DEFAULT 0,
    last_error TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_notification_queue_pending ON notification_queue(is_processed, scheduled_for, priority)
    WHERE is_processed = false;
CREATE INDEX idx_notification_queue_notification ON notification_queue(notification_id);
CREATE INDEX idx_notification_queue_batch ON notification_queue(batch_id) WHERE batch_id IS NOT NULL;

-- ============================================================================
-- Notification Log Table (detailed event tracking)
-- ============================================================================

CREATE TABLE notification_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    notification_id UUID NOT NULL REFERENCES notification(id),

    -- Event details
    event_type VARCHAR(50) NOT NULL,  -- CREATED, QUEUED, SENT, DELIVERED, OPENED, CLICKED, FAILED, BOUNCED
    event_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Event data
    event_data JSONB,
    event_message TEXT,

    -- Source
    event_source VARCHAR(100)      -- SYSTEM, PROVIDER_WEBHOOK, USER_ACTION
);

-- Indexes
CREATE INDEX idx_notification_log_notification ON notification_log(notification_id);
CREATE INDEX idx_notification_log_timestamp ON notification_log(event_timestamp DESC);
CREATE INDEX idx_notification_log_type ON notification_log(event_type);

-- ============================================================================
-- Provider Configuration Table
-- ============================================================================

CREATE TABLE provider_configuration (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Provider details
    provider_name VARCHAR(100) NOT NULL,     -- Twilio, SendGrid, WhatsApp, FCM
    provider_type notification_channel NOT NULL,

    -- Configuration (encrypted in production)
    api_key TEXT,
    api_secret TEXT,
    endpoint_url VARCHAR(500),
    from_number VARCHAR(50),
    from_email VARCHAR(200),
    configuration JSONB,              -- Additional provider-specific config

    -- Status
    is_active BOOLEAN DEFAULT true,
    is_default BOOLEAN DEFAULT false,

    -- Limits
    daily_limit INTEGER,
    monthly_limit INTEGER,
    rate_limit_per_second DECIMAL(5, 2),

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false,

    CONSTRAINT provider_config_org_provider_unique UNIQUE (organization_id, provider_name, provider_type)
);

-- Indexes
CREATE INDEX idx_provider_config_org ON provider_configuration(organization_id) WHERE is_deleted = false;
CREATE INDEX idx_provider_config_type ON provider_configuration(provider_type) WHERE is_active = true;

-- ============================================================================
-- Functions
-- ============================================================================

-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers
CREATE TRIGGER update_notification_template_updated_at
    BEFORE UPDATE ON notification_template
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notification_updated_at
    BEFORE UPDATE ON notification
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notification_preference_updated_at
    BEFORE UPDATE ON notification_preference
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_provider_config_updated_at
    BEFORE UPDATE ON provider_configuration
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Update notification status and add log entry
CREATE OR REPLACE FUNCTION log_notification_status_change()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.notification_status IS DISTINCT FROM NEW.notification_status THEN
        INSERT INTO notification_log (notification_id, event_type, event_message, event_source)
        VALUES (
            NEW.id,
            'STATUS_CHANGED',
            'Status changed from ' || COALESCE(OLD.notification_status::TEXT, 'NULL') ||
            ' to ' || NEW.notification_status::TEXT,
            'SYSTEM'
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_log_notification_status_change
    AFTER UPDATE ON notification
    FOR EACH ROW
    EXECUTE FUNCTION log_notification_status_change();

-- ============================================================================
-- Sample Data
-- ============================================================================

-- Sample notification template
INSERT INTO notification_template (
    id, organization_id, template_name, template_code, template_type,
    email_subject, email_body, sms_content,
    variables, supported_channels, default_channel, is_active, created_by
) VALUES (
    uuid_generate_v4(),
    uuid_generate_v4(),
    'Test Result Ready',
    'TEST_RESULT_READY',
    'TEST_RESULT_READY',
    'Your Lab Test Results are Ready',
    'Dear {{patient_name}},\n\nYour test results for {{test_name}} are now ready. Please log in to view your report or visit our lab.\n\nThank you,\n{{lab_name}}',
    'Dear {{patient_name}}, your {{test_name}} results are ready. Visit {{lab_name}} or check online.',
    '["patient_name", "test_name", "lab_name", "report_url"]'::jsonb,
    ARRAY['EMAIL', 'SMS', 'WHATSAPP']::notification_channel[],
    'EMAIL'::notification_channel,
    true,
    uuid_generate_v4()
);

-- Comments
COMMENT ON TABLE notification_template IS 'Reusable notification templates';
COMMENT ON TABLE notification IS 'Notification messages sent via various channels';
COMMENT ON TABLE notification_preference IS 'User preferences for notification delivery';
COMMENT ON TABLE notification_queue IS 'Queue for batch processing notifications';
COMMENT ON TABLE notification_log IS 'Detailed event log for notifications';
COMMENT ON TABLE provider_configuration IS 'Third-party provider configurations';
