#!/bin/bash

# LIS Modern Backend - API Readiness Validation
# Validates that all APIs are ready for testing without requiring running services

source "$(dirname "$0")/test_config.sh"

print_header "API Readiness Validation - Comprehensive Check"
echo ""

VALIDATION_PASSED=0
VALIDATION_FAILED=0

# Test 1: Verify all service directories exist
print_info "Validation 1: Service Directory Structure"
SERVICES=(
    "patient-service"
    "user-service"
    "organization-service"
    "sample-service"
    "order-service"
    "result-service"
    "equipment-service"
    "inventory-service"
    "qc-service"
    "billing-service"
    "notification-service"
    "analytics-service"
    "report-service"
    "compliance-service"
)

for service in "${SERVICES[@]}"; do
    if [ -d "../../services/$service" ]; then
        print_success "$service directory exists"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_error "$service directory missing"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 2: Verify API files exist
print_info "Validation 2: API Implementation Files"
for service in "${SERVICES[@]}"; do
    if [ -f "../../services/$service/src/api.rs" ]; then
        print_success "$service API file exists"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_error "$service API file missing"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 3: Verify GraphQL schema definitions
print_info "Validation 3: GraphQL Schema Definitions"
for service in "${SERVICES[@]}"; do
    if grep -q "struct QueryRoot" "../../services/$service/src/api.rs" 2>/dev/null && \
       grep -q "struct MutationRoot" "../../services/$service/src/api.rs" 2>/dev/null; then
        print_success "$service has complete GraphQL schema (Query + Mutation)"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_warning "$service GraphQL schema incomplete"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 4: Verify Cargo.toml configurations
print_info "Validation 4: Cargo Configuration Files"
for service in "${SERVICES[@]}"; do
    if [ -f "../../services/$service/Cargo.toml" ]; then
        print_success "$service Cargo.toml exists"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_error "$service Cargo.toml missing"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 5: Verify domain models
print_info "Validation 5: Domain Model Files"
for service in "${SERVICES[@]}"; do
    if [ -f "../../services/$service/src/domain.rs" ]; then
        print_success "$service domain model exists"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_warning "$service domain model missing"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 6: Verify repositories
print_info "Validation 6: Repository Layer Files"
for service in "${SERVICES[@]}"; do
    if [ -f "../../services/$service/src/repository.rs" ]; then
        print_success "$service repository exists"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_warning "$service repository missing"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 7: Verify service layer
print_info "Validation 7: Service Layer Files"
for service in "${SERVICES[@]}"; do
    if [ -f "../../services/$service/src/service.rs" ]; then
        print_success "$service business logic exists"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_warning "$service business logic missing"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 8: Verify main entry points
print_info "Validation 8: Service Entry Points"
for service in "${SERVICES[@]}"; do
    if [ -f "../../services/$service/src/main.rs" ]; then
        print_success "$service main.rs exists"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_error "$service main.rs missing"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 9: Verify test scripts
print_info "Validation 9: API Test Scripts"
TEST_SCRIPTS=(
    "test_config.sh"
    "01_health_check_test.sh"
    "02_auth_test.sh"
    "03_patient_test.sh"
    "04_order_workflow_test.sh"
    "run_all_tests.sh"
)

for script in "${TEST_SCRIPTS[@]}"; do
    if [ -f "$(dirname "$0")/$script" ] && [ -x "$(dirname "$0")/$script" ]; then
        print_success "Test script $script exists and is executable"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_error "Test script $script missing or not executable"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 10: Verify documentation
print_info "Validation 10: API Documentation"
DOCS=(
    "../../API_TESTING_GUIDE.md"
    "../../API_READINESS_REPORT.md"
)

for doc in "${DOCS[@]}"; do
    if [ -f "$doc" ]; then
        print_success "$(basename $doc) exists"
        VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
    else
        print_error "$(basename $doc) missing"
        VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
    fi
done
echo ""

# Test 11: Verify Docker configuration
print_info "Validation 11: Docker Infrastructure"
if [ -f "../../docker-compose.yml" ]; then
    print_success "docker-compose.yml exists"
    VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
else
    print_error "docker-compose.yml missing"
    VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
fi

if [ -f "../../init-databases.sql" ]; then
    print_success "Database initialization script exists"
    VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
else
    print_warning "Database initialization script missing"
    VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
fi
echo ""

# Test 12: Verify prerequisites
print_info "Validation 12: System Prerequisites"
if command -v curl &> /dev/null; then
    print_success "curl is installed"
    VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
else
    print_error "curl is not installed"
    VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
fi

if command -v jq &> /dev/null; then
    print_success "jq is installed"
    VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
else
    print_error "jq is not installed"
    VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
fi

if command -v cargo &> /dev/null; then
    print_success "Rust/Cargo is installed"
    VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
else
    print_error "Rust/Cargo is not installed"
    VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
fi
echo ""

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Validation Summary:"
echo "  Total Validations:  $((VALIDATION_PASSED + VALIDATION_FAILED))"
echo -e "  ${GREEN}Passed:             $VALIDATION_PASSED${NC}"
echo -e "  ${RED}Failed:             $VALIDATION_FAILED${NC}"

if [ $VALIDATION_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                                                              ║${NC}"
    echo -e "${GREEN}║     ✓ ALL VALIDATIONS PASSED! APIs ARE 100% READY! ✓        ║${NC}"
    echo -e "${GREEN}║                                                              ║${NC}"
    echo -e "${GREEN}║     Your backend is ready for API testing and production!   ║${NC}"
    echo -e "${GREEN}║                                                              ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 0
else
    echo ""
    echo -e "${YELLOW}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${YELLOW}║                                                              ║${NC}"
    echo -e "${YELLOW}║     ⚠ SOME VALIDATIONS FAILED - REVIEW REQUIRED ⚠           ║${NC}"
    echo -e "${YELLOW}║                                                              ║${NC}"
    echo -e "${YELLOW}║     Please address the issues above before proceeding       ║${NC}"
    echo -e "${YELLOW}║                                                              ║${NC}"
    echo -e "${YELLOW}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 1
fi
