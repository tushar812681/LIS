#!/bin/bash

# LIS Modern Backend - GraphQL Schema Validation
# Validates all GraphQL schemas for consistency and completeness

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

print_blue() { echo -e "${BLUE}➤ $1${NC}"; }
print_green() { echo -e "${GREEN}✓ $1${NC}"; }
print_red() { echo -e "${RED}✗ $1${NC}"; }

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║        LIS Modern Backend - Schema Validation               ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

SERVICES=(
    "patient-service"
    "organization-service"
    "sample-service"
    "order-service"
    "result-service"
    "equipment-service"
    "inventory-service"
    "qc-service"
    "billing-service"
    "user-service"
    "notification-service"
    "analytics-service"
    "report-service"
    "compliance-service"
)

PASSED=0
TOTAL=${#SERVICES[@]}

print_blue "Validating GraphQL Schemas..."
echo ""

for service in "${SERVICES[@]}"; do
    API_FILE="services/$service/src/api.rs"

    if [ ! -f "$API_FILE" ]; then
        print_red "$service: API file not found"
        continue
    fi

    # Check for QueryRoot
    if grep -q "struct QueryRoot" "$API_FILE"; then
        HAS_QUERY=1
    else
        HAS_QUERY=0
    fi

    # Check for MutationRoot
    if grep -q "struct MutationRoot" "$API_FILE"; then
        HAS_MUTATION=1
    else
        HAS_MUTATION=0
    fi

    # Check for Object trait implementation
    if grep -q "#\[Object\]" "$API_FILE"; then
        HAS_OBJECT=1
    else
        HAS_OBJECT=0
    fi

    # Check for async_graphql import
    if grep -q "use async_graphql" "$API_FILE"; then
        HAS_IMPORT=1
    else
        HAS_IMPORT=0
    fi

    if [ $HAS_QUERY -eq 1 ] && [ $HAS_MUTATION -eq 1 ] && [ $HAS_OBJECT -eq 1 ] && [ $HAS_IMPORT -eq 1 ]; then
        print_green "$service: Schema complete (Query + Mutation + Object)"
        ((PASSED++))
    else
        print_red "$service: Schema incomplete"
        [ $HAS_QUERY -eq 0 ] && echo "  - Missing QueryRoot"
        [ $HAS_MUTATION -eq 0 ] && echo "  - Missing MutationRoot"
        [ $HAS_OBJECT -eq 0 ] && echo "  - Missing Object annotation"
        [ $HAS_IMPORT -eq 0 ] && echo "  - Missing async_graphql import"
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Schema Validation: $PASSED/$TOTAL services passed"
echo ""

if [ $PASSED -eq $TOTAL ]; then
    print_green "✓ All GraphQL schemas valid!"
    exit 0
else
    print_red "✗ Some schemas need attention"
    exit 1
fi
