-- Sync Service Database Schema
-- Version: 1.0.0
-- Date: 2025-11-15

-- ============================================================================
-- Enums
-- ============================================================================

CREATE TYPE entity_type AS ENUM (
    'patient',
    'sample',
    'order',
    'result',
    'invoice',
    'payment',
    'report',
    'inventory',
    'equipment',
    'qcresult',
    'notification'
);

CREATE TYPE sync_operation AS ENUM (
    'create',
    'update',
    'delete',
    'softdelete'
);

CREATE TYPE sync_status AS ENUM (
    'pending',
    'inprogress',
    'completed',
    'failed',
    'conflict',
    'skipped'
);

CREATE TYPE conflict_resolution_status AS ENUM (
    'pending',
    'clientwins',
    'serverwins',
    'manualresolution',
    'merged'
);

CREATE TYPE device_type AS ENUM (
    'web',
    'mobile',
    'tablet',
    'desktop'
);

CREATE TYPE network_status AS ENUM (
    'online',
    'offline',
    'slownetwork',
    'unknown'
);

-- ============================================================================
-- Tables
-- ============================================================================

-- Sync Devices
CREATE TABLE sync_devices (
    id VARCHAR(36) PRIMARY KEY,
    device_id VARCHAR(255) UNIQUE NOT NULL,
    device_name VARCHAR(255) NOT NULL,
    device_type device_type NOT NULL,
    user_id VARCHAR(36),
    organization_id VARCHAR(36) NOT NULL,
    last_sync_at TIMESTAMP WITH TIME ZONE,
    sync_enabled BOOLEAN NOT NULL DEFAULT true,
    offline_mode BOOLEAN NOT NULL DEFAULT false,
    network_status network_status NOT NULL DEFAULT 'online',
    sync_stats JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Sync Queue
CREATE TABLE sync_queue (
    id VARCHAR(36) PRIMARY KEY,
    device_id VARCHAR(255) NOT NULL,
    entity_type entity_type NOT NULL,
    entity_id VARCHAR(36) NOT NULL,
    operation sync_operation NOT NULL,
    data JSONB NOT NULL,
    client_timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server_timestamp TIMESTAMP WITH TIME ZONE,
    status sync_status NOT NULL DEFAULT 'pending',
    retry_count INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_device FOREIGN KEY (device_id) REFERENCES sync_devices(device_id) ON DELETE CASCADE
);

-- Sync Conflicts
CREATE TABLE sync_conflicts (
    id VARCHAR(36) PRIMARY KEY,
    device_id VARCHAR(255) NOT NULL,
    entity_type entity_type NOT NULL,
    entity_id VARCHAR(36) NOT NULL,
    client_data JSONB NOT NULL,
    server_data JSONB NOT NULL,
    client_version BIGINT NOT NULL DEFAULT 0,
    server_version BIGINT NOT NULL DEFAULT 0,
    resolution_status conflict_resolution_status NOT NULL DEFAULT 'pending',
    resolution_data JSONB,
    resolved_by VARCHAR(36),
    resolved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_device_conflict FOREIGN KEY (device_id) REFERENCES sync_devices(device_id) ON DELETE CASCADE
);

-- Sync Logs
CREATE TABLE sync_logs (
    id VARCHAR(36) PRIMARY KEY,
    device_id VARCHAR(255) NOT NULL,
    sync_session_id VARCHAR(36) NOT NULL,
    entity_type entity_type NOT NULL,
    operation sync_operation NOT NULL,
    entity_count INTEGER NOT NULL DEFAULT 0,
    success_count INTEGER NOT NULL DEFAULT 0,
    failure_count INTEGER NOT NULL DEFAULT 0,
    conflict_count INTEGER NOT NULL DEFAULT 0,
    duration_ms BIGINT NOT NULL,
    started_at TIMESTAMP WITH TIME ZONE NOT NULL,
    completed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT fk_device_log FOREIGN KEY (device_id) REFERENCES sync_devices(device_id) ON DELETE CASCADE
);

-- ============================================================================
-- Indexes
-- ============================================================================

-- Sync Devices Indexes
CREATE INDEX idx_sync_devices_organization ON sync_devices(organization_id);
CREATE INDEX idx_sync_devices_user ON sync_devices(user_id);
CREATE INDEX idx_sync_devices_last_sync ON sync_devices(last_sync_at);

-- Sync Queue Indexes
CREATE INDEX idx_sync_queue_device_status ON sync_queue(device_id, status);
CREATE INDEX idx_sync_queue_entity ON sync_queue(entity_type, entity_id);
CREATE INDEX idx_sync_queue_status ON sync_queue(status);
CREATE INDEX idx_sync_queue_created ON sync_queue(created_at);
CREATE INDEX idx_sync_queue_client_timestamp ON sync_queue(client_timestamp);

-- Sync Conflicts Indexes
CREATE INDEX idx_sync_conflicts_device ON sync_conflicts(device_id);
CREATE INDEX idx_sync_conflicts_status ON sync_conflicts(resolution_status);
CREATE INDEX idx_sync_conflicts_entity ON sync_conflicts(entity_type, entity_id);
CREATE INDEX idx_sync_conflicts_created ON sync_conflicts(created_at);

-- Sync Logs Indexes
CREATE INDEX idx_sync_logs_device ON sync_logs(device_id);
CREATE INDEX idx_sync_logs_session ON sync_logs(sync_session_id);
CREATE INDEX idx_sync_logs_started ON sync_logs(started_at);

-- ============================================================================
-- Functions and Triggers
-- ============================================================================

-- Update timestamp trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply update timestamp trigger to all tables
CREATE TRIGGER update_sync_devices_updated_at
    BEFORE UPDATE ON sync_devices
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sync_queue_updated_at
    BEFORE UPDATE ON sync_queue
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sync_conflicts_updated_at
    BEFORE UPDATE ON sync_conflicts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE sync_devices IS 'Registered devices that can perform offline sync';
COMMENT ON TABLE sync_queue IS 'Queue of operations waiting to be synced to server';
COMMENT ON TABLE sync_conflicts IS 'Conflicts detected during sync that need resolution';
COMMENT ON TABLE sync_logs IS 'Audit log of all sync operations';

COMMENT ON COLUMN sync_queue.data IS 'JSON data of the entity to be synced';
COMMENT ON COLUMN sync_queue.retry_count IS 'Number of times this operation has been retried';
COMMENT ON COLUMN sync_conflicts.client_data IS 'Version of data from client (offline changes)';
COMMENT ON COLUMN sync_conflicts.server_data IS 'Version of data from server (latest on server)';
COMMENT ON COLUMN sync_conflicts.resolution_data IS 'Final resolved data after conflict resolution';
