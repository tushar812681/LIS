-- Initialize all LIS databases
-- This script runs when PostgreSQL container starts for the first time

-- Core Workflow Databases
CREATE DATABASE lis_patient;
CREATE DATABASE lis_sample;
CREATE DATABASE lis_order;
CREATE DATABASE lis_result;

-- Infrastructure Databases
CREATE DATABASE lis_user;
CREATE DATABASE lis_organization;
CREATE DATABASE lis_equipment;

-- Compliance & Operations Databases
CREATE DATABASE lis_qc;
CREATE DATABASE lis_billing;
CREATE DATABASE lis_report;

-- Support Services Databases
CREATE DATABASE lis_inventory;
CREATE DATABASE lis_notification;
CREATE DATABASE lis_analytics;
CREATE DATABASE lis_compliance;

-- New Production-Critical Service Databases
CREATE DATABASE lis_sync;
CREATE DATABASE lis_file;
CREATE DATABASE lis_integration;
CREATE DATABASE lis_abdm;

-- Grant permissions
GRANT ALL PRIVILEGES ON DATABASE lis_patient TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_sample TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_order TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_result TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_user TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_organization TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_equipment TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_qc TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_billing TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_report TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_inventory TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_notification TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_analytics TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_compliance TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_sync TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_file TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_integration TO postgres;
GRANT ALL PRIVILEGES ON DATABASE lis_abdm TO postgres;
