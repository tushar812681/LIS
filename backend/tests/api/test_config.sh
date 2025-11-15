#!/bin/bash

# LIS Modern Backend - API Test Configuration
# Centralized configuration for all API tests

# Service URLs
export USER_SERVICE_URL="http://localhost:8085"
export PATIENT_SERVICE_URL="http://localhost:8081"
export ORGANIZATION_SERVICE_URL="http://localhost:8086"
export SAMPLE_SERVICE_URL="http://localhost:8082"
export ORDER_SERVICE_URL="http://localhost:8083"
export RESULT_SERVICE_URL="http://localhost:8084"
export EQUIPMENT_SERVICE_URL="http://localhost:8087"
export INVENTORY_SERVICE_URL="http://localhost:8091"
export QC_SERVICE_URL="http://localhost:8088"
export BILLING_SERVICE_URL="http://localhost:8089"
export NOTIFICATION_SERVICE_URL="http://localhost:8092"
export ANALYTICS_SERVICE_URL="http://localhost:8093"
export REPORT_SERVICE_URL="http://localhost:8090"
export COMPLIANCE_SERVICE_URL="http://localhost:8094"

# GraphQL endpoints
export GRAPHQL_PATH="/graphql"
export HEALTH_PATH="/health"
export METRICS_PATH="/metrics"

# Test configuration
export TEST_TIMEOUT=10
export MAX_RETRIES=3
export VERBOSE=${VERBOSE:-false}

# Colors for output
export GREEN='\033[0;32m'
export RED='\033[0;31m'
export YELLOW='\033[1;33m'
export BLUE='\033[0;34m'
export NC='\033[0m' # No Color

# Test data
export TEST_ORG_ID="00000000-0000-0000-0000-000000000001"
export TEST_USER_EMAIL="admin@test.com"
export TEST_USER_PASSWORD="Admin@123456"

# Authentication token (will be set after login)
export AUTH_TOKEN=""

# Test counters
export TESTS_RUN=0
export TESTS_PASSED=0
export TESTS_FAILED=0

# Helper functions
print_header() {
    echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║  $1${NC}"
    echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Test assertion functions
assert_equals() {
    local expected="$1"
    local actual="$2"
    local test_name="$3"

    TESTS_RUN=$((TESTS_RUN + 1))

    if [ "$expected" = "$actual" ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        print_success "$test_name"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        print_error "$test_name"
        echo "  Expected: $expected"
        echo "  Actual:   $actual"
        return 1
    fi
}

assert_not_empty() {
    local value="$1"
    local test_name="$2"

    TESTS_RUN=$((TESTS_RUN + 1))

    if [ -n "$value" ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        print_success "$test_name"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        print_error "$test_name"
        echo "  Value is empty"
        return 1
    fi
}

assert_contains() {
    local haystack="$1"
    local needle="$2"
    local test_name="$3"

    TESTS_RUN=$((TESTS_RUN + 1))

    if echo "$haystack" | grep -q "$needle"; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        print_success "$test_name"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        print_error "$test_name"
        echo "  '$needle' not found in response"
        return 1
    fi
}

# GraphQL query helper
graphql_query() {
    local url="$1"
    local query="$2"
    local variables="${3:-{}}"

    local response=$(curl -s -X POST "$url$GRAPHQL_PATH" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $AUTH_TOKEN" \
        --max-time $TEST_TIMEOUT \
        -d "{\"query\": $(echo "$query" | jq -Rs .), \"variables\": $variables}")

    echo "$response"
}

# Health check helper
check_health() {
    local service_name="$1"
    local url="$2"

    local response=$(curl -s -o /dev/null -w "%{http_code}" --max-time $TEST_TIMEOUT "$url$HEALTH_PATH")

    if [ "$response" = "200" ]; then
        print_success "$service_name is healthy"
        return 0
    else
        print_error "$service_name is not responding (HTTP $response)"
        return 1
    fi
}

# Print test summary
print_test_summary() {
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Test Summary:"
    echo "  Total Tests:  $TESTS_RUN"
    echo -e "  ${GREEN}Passed:       $TESTS_PASSED${NC}"
    echo -e "  ${RED}Failed:       $TESTS_FAILED${NC}"

    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "  ${GREEN}Result:       ALL TESTS PASSED ✓${NC}"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        return 0
    else
        echo -e "  ${RED}Result:       SOME TESTS FAILED ✗${NC}"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        return 1
    fi
}
