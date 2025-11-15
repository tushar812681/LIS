-- Analytics Service Database Schema
-- Stores dashboard configs, custom reports, and cached metrics

-- Custom dashboard configurations
CREATE TABLE IF NOT EXISTS dashboard_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    user_id UUID,
    role VARCHAR(50) NOT NULL,

    dashboard_name VARCHAR(200) NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    layout_config JSONB, -- Widget positions and sizes

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

CREATE INDEX idx_dashboard_config_org_role ON dashboard_config(organization_id, role);
CREATE INDEX idx_dashboard_config_user ON dashboard_config(user_id);

-- Dashboard widgets
CREATE TABLE IF NOT EXISTS dashboard_widget (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dashboard_id UUID NOT NULL REFERENCES dashboard_config(id) ON DELETE CASCADE,

    widget_type VARCHAR(50) NOT NULL, -- KPI, CHART, TABLE, ALERT_LIST
    title VARCHAR(200) NOT NULL,
    position JSONB NOT NULL, -- {x, y, w, h}

    data_source VARCHAR(100) NOT NULL, -- SAMPLES_TODAY, TAT_COMPLIANCE, REVENUE
    refresh_interval_seconds INTEGER DEFAULT 300,

    config JSONB, -- Widget-specific configuration

    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dashboard_widget_dashboard ON dashboard_widget(dashboard_id);

-- Custom saved reports
CREATE TABLE IF NOT EXISTS saved_report (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    created_by UUID NOT NULL,

    report_name VARCHAR(200) NOT NULL,
    report_type VARCHAR(50) NOT NULL, -- TAT_ANALYSIS, REVENUE, SAMPLE_VOLUME, CUSTOM
    description TEXT,

    -- Report configuration
    date_range_type VARCHAR(50), -- TODAY, THIS_WEEK, THIS_MONTH, CUSTOM, LAST_N_DAYS
    custom_date_from DATE,
    custom_date_to DATE,

    filters JSONB, -- Dynamic filters
    group_by VARCHAR(100), -- DEPARTMENT, TEST, PRIORITY, DAY, WEEK
    aggregation VARCHAR(50), -- COUNT, SUM, AVG, MIN, MAX

    -- Scheduling
    is_scheduled BOOLEAN DEFAULT FALSE,
    schedule_frequency VARCHAR(50), -- DAILY, WEEKLY, MONTHLY
    schedule_time TIME,
    recipients JSONB, -- Array of user IDs or emails

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_generated_at TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

CREATE INDEX idx_saved_report_org ON saved_report(organization_id);
CREATE INDEX idx_saved_report_creator ON saved_report(created_by);
CREATE INDEX idx_saved_report_scheduled ON saved_report(organization_id) WHERE is_scheduled = TRUE;

-- Cached metrics (for performance)
CREATE TABLE IF NOT EXISTS cached_metric (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    metric_key VARCHAR(100) NOT NULL,
    metric_date DATE NOT NULL,

    metric_value DECIMAL(15,2) NOT NULL,
    metric_metadata JSONB, -- Additional context

    calculated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,

    UNIQUE(organization_id, metric_key, metric_date)
);

CREATE INDEX idx_cached_metric_lookup ON cached_metric(organization_id, metric_key, metric_date);
CREATE INDEX idx_cached_metric_expiry ON cached_metric(expires_at);

-- KPI definitions
CREATE TABLE IF NOT EXISTS kpi_definition (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,

    kpi_name VARCHAR(100) NOT NULL,
    kpi_code VARCHAR(50) NOT NULL,
    description TEXT,
    category VARCHAR(50), -- OPERATIONAL, FINANCIAL, QUALITY, EFFICIENCY

    calculation_method VARCHAR(50) NOT NULL, -- SQL_QUERY, FORMULA, AGGREGATION
    sql_query TEXT,
    formula TEXT,

    unit VARCHAR(20), -- PERCENTAGE, COUNT, CURRENCY, HOURS, DAYS
    target_value DECIMAL(15,2),
    warning_threshold DECIMAL(15,2),
    critical_threshold DECIMAL(15,2),

    is_higher_better BOOLEAN DEFAULT TRUE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE,

    UNIQUE(organization_id, kpi_code)
);

CREATE INDEX idx_kpi_definition_org ON kpi_definition(organization_id);

-- KPI values (historical tracking)
CREATE TABLE IF NOT EXISTS kpi_value (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kpi_id UUID NOT NULL REFERENCES kpi_definition(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL,

    value_date DATE NOT NULL,
    calculated_value DECIMAL(15,2) NOT NULL,

    status VARCHAR(20), -- ON_TARGET, WARNING, CRITICAL
    metadata JSONB,

    calculated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE(kpi_id, value_date)
);

CREATE INDEX idx_kpi_value_kpi_date ON kpi_value(kpi_id, value_date DESC);
CREATE INDEX idx_kpi_value_org ON kpi_value(organization_id, value_date);

-- Report execution log
CREATE TABLE IF NOT EXISTS report_execution_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID REFERENCES saved_report(id) ON DELETE SET NULL,
    organization_id UUID NOT NULL,

    executed_by UUID,
    execution_type VARCHAR(50), -- MANUAL, SCHEDULED

    parameters JSONB,

    status VARCHAR(20) NOT NULL, -- SUCCESS, FAILED, PARTIAL
    row_count INTEGER,
    execution_time_ms INTEGER,
    error_message TEXT,

    report_output_url VARCHAR(500), -- S3/MinIO URL

    executed_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_report_execution_log_report ON report_execution_log(report_id);
CREATE INDEX idx_report_execution_log_org_date ON report_execution_log(organization_id, executed_at DESC);

-- Alert definitions for KPIs
CREATE TABLE IF NOT EXISTS kpi_alert (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kpi_id UUID NOT NULL REFERENCES kpi_definition(id) ON DELETE CASCADE,

    alert_name VARCHAR(200) NOT NULL,
    condition VARCHAR(50) NOT NULL, -- EXCEEDS, BELOW, EQUALS, CHANGE_PERCENTAGE
    threshold_value DECIMAL(15,2) NOT NULL,

    alert_recipients JSONB NOT NULL, -- Array of user IDs
    alert_channels JSONB, -- [EMAIL, SMS, IN_APP]

    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_kpi_alert_kpi ON kpi_alert(kpi_id) WHERE is_active = TRUE;

-- Auto-update trigger for dashboard_config
CREATE OR REPLACE FUNCTION update_dashboard_config_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_dashboard_config_updated_at_trigger
BEFORE UPDATE ON dashboard_config
FOR EACH ROW EXECUTE FUNCTION update_dashboard_config_updated_at();

-- Auto-update trigger for saved_report
CREATE OR REPLACE FUNCTION update_saved_report_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_saved_report_updated_at_trigger
BEFORE UPDATE ON saved_report
FOR EACH ROW EXECUTE FUNCTION update_saved_report_updated_at();

-- Function to clean up expired cached metrics
CREATE OR REPLACE FUNCTION cleanup_expired_metrics()
RETURNS void AS $$
BEGIN
    DELETE FROM cached_metric WHERE expires_at < NOW();
END;
$$ LANGUAGE plpgsql;

-- Insert default KPI definitions
INSERT INTO kpi_definition (organization_id, kpi_name, kpi_code, description, category, calculation_method, unit, target_value, is_active)
VALUES
    ('00000000-0000-0000-0000-000000000000'::UUID, 'TAT Compliance Rate', 'TAT_COMPLIANCE', 'Percentage of orders completed within promised TAT', 'QUALITY', 'SQL_QUERY', 'PERCENTAGE', 95.0, TRUE),
    ('00000000-0000-0000-0000-000000000000'::UUID, 'Daily Sample Volume', 'SAMPLE_VOLUME', 'Total number of samples processed per day', 'OPERATIONAL', 'SQL_QUERY', 'COUNT', NULL, TRUE),
    ('00000000-0000-0000-0000-000000000000'::UUID, 'Average TAT Hours', 'AVG_TAT', 'Average turnaround time in hours', 'EFFICIENCY', 'SQL_QUERY', 'HOURS', 24.0, FALSE),
    ('00000000-0000-0000-0000-000000000000'::UUID, 'Revenue Per Day', 'DAILY_REVENUE', 'Total revenue generated per day', 'FINANCIAL', 'SQL_QUERY', 'CURRENCY', NULL, TRUE),
    ('00000000-0000-0000-0000-000000000000'::UUID, 'Sample Rejection Rate', 'REJECTION_RATE', 'Percentage of samples rejected', 'QUALITY', 'SQL_QUERY', 'PERCENTAGE', 5.0, FALSE),
    ('00000000-0000-0000-0000-000000000000'::UUID, 'Equipment Utilization', 'EQUIPMENT_UTIL', 'Percentage of equipment capacity used', 'OPERATIONAL', 'SQL_QUERY', 'PERCENTAGE', 75.0, TRUE)
ON CONFLICT (organization_id, kpi_code) DO NOTHING;

COMMENT ON TABLE dashboard_config IS 'User and role-specific dashboard configurations';
COMMENT ON TABLE saved_report IS 'Custom saved reports with scheduling';
COMMENT ON TABLE cached_metric IS 'Performance cache for frequently accessed metrics';
COMMENT ON TABLE kpi_definition IS 'KPI definitions with calculation methods';
COMMENT ON TABLE kpi_value IS 'Historical KPI values for trending';
