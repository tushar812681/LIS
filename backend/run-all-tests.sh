#!/bin/bash

# Comprehensive Test Runner for LIS Modern Backend
# This script runs all tests and generates a test report

set -e

echo "================================================"
echo "LIS Modern Backend - Comprehensive Test Suite"
echo "================================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to print section headers
print_section() {
    echo ""
    echo "================================================"
    echo "$1"
    echo "================================================"
}

# Function to run a test and track results
run_test() {
    local test_name="$1"
    local test_command="$2"

    echo -e "${YELLOW}Running: $test_name${NC}"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if eval "$test_command"; then
        echo -e "${GREEN}✓ PASSED: $test_name${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        echo -e "${RED}✗ FAILED: $test_name${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# 1. Code Quality Checks
print_section "1. CODE QUALITY CHECKS"

run_test "Cargo Format Check" "cargo fmt --all -- --check"
run_test "Cargo Clippy" "cargo clippy --all-targets --all-features -- -D warnings"
run_test "Cargo Check" "cargo check --workspace --all-features"

# 2. Unit Tests
print_section "2. UNIT TESTS"

run_test "Common Library Tests" "cargo test --package common --lib"
run_test "Infrastructure Library Tests" "cargo test --package infrastructure --lib"

# 3. Domain Model Tests
print_section "3. DOMAIN MODEL TESTS"

for service in patient sample order result user organization equipment qc billing report inventory notification analytics compliance
do
    run_test "${service}-service Domain Tests" \
        "cargo test --package ${service}-service --lib domain"
done

# 4. Repository Tests
print_section "4. REPOSITORY TESTS"

for service in patient sample order result
do
    run_test "${service}-service Repository Tests" \
        "cargo test --package ${service}-service --lib repository"
done

# 5. Service Layer Tests
print_section "5. SERVICE LAYER TESTS"

for service in patient sample order result
do
    run_test "${service}-service Service Tests" \
        "cargo test --package ${service}-service --lib service"
done

# 6. Integration Tests
print_section "6. INTEGRATION TESTS"

if [ -n "$TEST_DATABASE_URL" ]; then
    run_test "Patient Service Integration Tests" \
        "cargo test --package patient-service --test integration_tests"

    run_test "Result Service Auto-Verification Tests" \
        "cargo test --package result-service --test auto_verification_tests"
else
    echo -e "${YELLOW}⚠ Skipping integration tests (TEST_DATABASE_URL not set)${NC}"
fi

# 7. API Tests
print_section "7. API TESTS"

for service in patient sample order result user organization equipment qc billing report inventory notification analytics compliance
do
    run_test "${service}-service API Tests" \
        "cargo test --package ${service}-service --lib api"
done

# 8. Build Tests
print_section "8. BUILD TESTS"

run_test "Debug Build" "cargo build --workspace"
run_test "Release Build" "cargo build --workspace --release"

# 9. Documentation Tests
print_section "9. DOCUMENTATION TESTS"

run_test "Doc Tests" "cargo test --doc"
run_test "Doc Generation" "cargo doc --workspace --no-deps"

# 10. Security Audit
print_section "10. SECURITY AUDIT"

if command -v cargo-audit &> /dev/null; then
    run_test "Cargo Audit" "cargo audit"
else
    echo -e "${YELLOW}⚠ cargo-audit not installed (run: cargo install cargo-audit)${NC}"
fi

# 11. Dependency Check
print_section "11. DEPENDENCY CHECK"

run_test "Cargo Outdated" "cargo outdated || true"

# Test Summary
print_section "TEST SUMMARY"

echo ""
echo "Total Tests Run: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
echo -e "${RED}Failed: $FAILED_TESTS${NC}"
echo ""

PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
echo "Pass Rate: ${PASS_RATE}%"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}================================================${NC}"
    echo -e "${GREEN}ALL TESTS PASSED! ✓${NC}"
    echo -e "${GREEN}Backend is ready for deployment!${NC}"
    echo -e "${GREEN}================================================${NC}"
    exit 0
else
    echo -e "${RED}================================================${NC}"
    echo -e "${RED}SOME TESTS FAILED! ✗${NC}"
    echo -e "${RED}Please fix the failing tests before deployment${NC}"
    echo -e "${RED}================================================${NC}"
    exit 1
fi
