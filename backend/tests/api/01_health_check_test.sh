#!/bin/bash

# LIS Modern Backend - Health Check Test
# Tests all 14 service health endpoints

source "$(dirname "$0")/test_config.sh"

print_header "Health Check Test - All 14 Services"
echo ""

# Test all service health endpoints
check_health "User Service" "$USER_SERVICE_URL"
check_health "Patient Service" "$PATIENT_SERVICE_URL"
check_health "Organization Service" "$ORGANIZATION_SERVICE_URL"
check_health "Sample Service" "$SAMPLE_SERVICE_URL"
check_health "Order Service" "$ORDER_SERVICE_URL"
check_health "Result Service" "$RESULT_SERVICE_URL"
check_health "Equipment Service" "$EQUIPMENT_SERVICE_URL"
check_health "Inventory Service" "$INVENTORY_SERVICE_URL"
check_health "QC Service" "$QC_SERVICE_URL"
check_health "Billing Service" "$BILLING_SERVICE_URL"
check_health "Notification Service" "$NOTIFICATION_SERVICE_URL"
check_health "Analytics Service" "$ANALYTICS_SERVICE_URL"
check_health "Report Service" "$REPORT_SERVICE_URL"
check_health "Compliance Service" "$COMPLIANCE_SERVICE_URL"

print_test_summary
