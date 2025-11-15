#!/bin/bash

# Comprehensive Test Script for LIS Microservices
# Version: 1.0.0
# Date: 2025-11-15

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TIMEOUT=5
BASE_URL="http://localhost"

# Service configurations
declare -A SERVICES=(
    ["API Gateway"]="8000"
    ["Patient Service"]="8081"
    ["Sample Service"]="8082"
    ["Order Service"]="8083"
    ["Result Service"]="8084"
    ["User Service"]="8085"
    ["Organization Service"]="8086"
    ["Equipment Service"]="8087"
    ["QC Service"]="8088"
    ["Billing Service"]="8089"
    ["Report Service"]="8090"
    ["Inventory Service"]="8091"
    ["Notification Service"]="8092"
    ["Analytics Service"]="8093"
    ["Compliance Service"]="8094"
    ["Sync Service"]="8095"
    ["File Service"]="8096"
    ["Integration Service"]="8097"
    ["ABDM Service"]="8098"
)

# Infrastructure services
declare -A INFRASTRUCTURE=(
    ["PostgreSQL"]="5432"
    ["Redis"]="6379"
    ["Kafka"]="9092"
    ["MinIO"]="9000"
    ["Prometheus"]="9090"
    ["Grafana"]="3001"
    ["Jaeger"]="16686"
)

# Print header
print_header() {
    echo ""
    echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
    echo ""
}

# Print test result
print_result() {
    if [ $2 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $1"
    else
        echo -e "${RED}✗${NC} $1"
    fi
}

# Test health endpoint
test_health() {
    local name=$1
    local port=$2
    local endpoint="${BASE_URL}:${port}/health"

    response=$(curl -s -o /dev/null -w "%{http_code}" --connect-timeout $TIMEOUT "$endpoint" 2>/dev/null || echo "000")

    if [ "$response" == "200" ]; then
        print_result "$name health check passed (Port: $port)" 0
        return 0
    else
        print_result "$name health check failed (Port: $port, Status: $response)" 1
        return 1
    fi
}

# Test GraphQL endpoint
test_graphql() {
    local name=$1
    local port=$2
    local endpoint="${BASE_URL}:${port}/graphql"

    response=$(curl -s -o /dev/null -w "%{http_code}" --connect-timeout $TIMEOUT \
        -X POST "$endpoint" \
        -H "Content-Type: application/json" \
        -d '{"query":"query{__typename}"}' 2>/dev/null || echo "000")

    if [ "$response" == "200" ]; then
        print_result "$name GraphQL endpoint accessible" 0
        return 0
    else
        print_result "$name GraphQL endpoint failed (Status: $response)" 1
        return 1
    fi
}

# Test TCP port
test_port() {
    local name=$1
    local port=$2

    if timeout $TIMEOUT bash -c "cat < /dev/null > /dev/tcp/localhost/$port" 2>/dev/null; then
        print_result "$name is listening on port $port" 0
        return 0
    else
        print_result "$name is NOT listening on port $port" 1
        return 1
    fi
}

# Test database connection
test_database() {
    local db_name=$1

    result=$(docker exec lis_postgres psql -U postgres -d $db_name -c "SELECT 1;" -t 2>/dev/null | grep -q "1" && echo "OK" || echo "FAIL")

    if [ "$result" == "OK" ]; then
        print_result "Database $db_name is accessible" 0
        return 0
    else
        print_result "Database $db_name is NOT accessible" 1
        return 1
    fi
}

# Main test execution
main() {
    print_header "LIS Microservices Comprehensive Test Suite"

    echo -e "${YELLOW}Testing started at: $(date)${NC}"
    echo ""

    total_tests=0
    passed_tests=0
    failed_tests=0

    # Test Infrastructure
    print_header "Testing Infrastructure Services"

    for service in "${!INFRASTRUCTURE[@]}"; do
        test_port "$service" "${INFRASTRUCTURE[$service]}"
        result=$?
        total_tests=$((total_tests + 1))
        if [ $result -eq 0 ]; then
            passed_tests=$((passed_tests + 1))
        else
            failed_tests=$((failed_tests + 1))
        fi
    done

    # Test Databases
    print_header "Testing Databases"

    databases=(
        "lis_patient" "lis_sample" "lis_order" "lis_result"
        "lis_user" "lis_organization" "lis_equipment"
        "lis_qc" "lis_billing" "lis_report"
        "lis_inventory" "lis_notification"
        "lis_analytics" "lis_compliance"
        "lis_sync" "lis_file" "lis_integration" "lis_abdm"
    )

    for db in "${databases[@]}"; do
        test_database "$db"
        result=$?
        total_tests=$((total_tests + 1))
        if [ $result -eq 0 ]; then
            passed_tests=$((passed_tests + 1))
        else
            failed_tests=$((failed_tests + 1))
        fi
    done

    # Test Microservices Health
    print_header "Testing Microservices Health Endpoints"

    for service in "${!SERVICES[@]}"; do
        test_health "$service" "${SERVICES[$service]}"
        result=$?
        total_tests=$((total_tests + 1))
        if [ $result -eq 0 ]; then
            passed_tests=$((passed_tests + 1))
        else
            failed_tests=$((failed_tests + 1))
        fi
    done

    # Test GraphQL Endpoints
    print_header "Testing GraphQL Endpoints"

    for service in "${!SERVICES[@]}"; do
        # Skip API Gateway for GraphQL test
        if [ "$service" != "API Gateway" ]; then
            test_graphql "$service" "${SERVICES[$service]}"
            result=$?
            total_tests=$((total_tests + 1))
            if [ $result -eq 0 ]; then
                passed_tests=$((passed_tests + 1))
            else
                failed_tests=$((failed_tests + 1))
            fi
        fi
    done

    # Print summary
    print_header "Test Summary"

    echo -e "Total Tests:  ${BLUE}$total_tests${NC}"
    echo -e "Passed:       ${GREEN}$passed_tests${NC}"
    echo -e "Failed:       ${RED}$failed_tests${NC}"

    success_rate=$(awk "BEGIN {printf \"%.1f\", ($passed_tests/$total_tests)*100}")
    echo -e "Success Rate: ${BLUE}${success_rate}%${NC}"

    echo ""
    echo -e "${YELLOW}Testing completed at: $(date)${NC}"
    echo ""

    if [ $failed_tests -eq 0 ]; then
        echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
        echo -e "${GREEN}║   ✓ ALL TESTS PASSED SUCCESSFULLY!   ║${NC}"
        echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
        exit 0
    else
        echo -e "${RED}╔════════════════════════════════════════╗${NC}"
        echo -e "${RED}║   ✗ SOME TESTS FAILED!                ║${NC}"
        echo -e "${RED}╚════════════════════════════════════════╝${NC}"
        exit 1
    fi
}

# Run main function
main
