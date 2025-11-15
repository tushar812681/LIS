#!/bin/bash
# =============================================================================
# Production Readiness Check Script
# =============================================================================
# This script verifies that the backend is ready for production deployment
# =============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
WARNINGS=0

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║        LIS Modern - Production Readiness Verification           ║"
echo "╔══════════════════════════════════════════════════════════════════╗"
echo ""

# =============================================================================
# Helper Functions
# =============================================================================

check_pass() {
    echo -e "${GREEN}✓${NC} $1"
    ((PASSED++))
}

check_fail() {
    echo -e "${RED}✗${NC} $1"
    ((FAILED++))
}

check_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

section_header() {
    echo ""
    echo -e "${BLUE}═══ $1 ═══${NC}"
    echo ""
}

# =============================================================================
# Environment Checks
# =============================================================================

section_header "Environment Checks"

# Check if running in backend directory
if [ ! -f "Cargo.toml" ]; then
    check_fail "Not in backend directory"
    exit 1
fi
check_pass "Running in backend directory"

# Check Rust version
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    if [ "$(printf '%s\n' "1.75" "$RUST_VERSION" | sort -V | head -n1)" = "1.75" ]; then
        check_pass "Rust version $RUST_VERSION (>= 1.75)"
    else
        check_fail "Rust version $RUST_VERSION (< 1.75 required)"
    fi
else
    check_fail "Rust not installed"
fi

# Check Cargo
if command -v cargo &> /dev/null; then
    check_pass "Cargo installed"
else
    check_fail "Cargo not installed"
fi

# =============================================================================
# Configuration Checks
# =============================================================================

section_header "Configuration Checks"

# Check for .env.example
if [ -f ".env.example" ]; then
    check_pass ".env.example exists"
else
    check_fail ".env.example not found"
fi

# Check for required config files
if [ -f "rustfmt.toml" ]; then
    check_pass "rustfmt.toml exists"
else
    check_warn "rustfmt.toml not found"
fi

if [ -f ".clippy.toml" ]; then
    check_pass ".clippy.toml exists"
else
    check_warn ".clippy.toml not found"
fi

if [ -f "deny.toml" ]; then
    check_pass "deny.toml exists"
else
    check_warn "deny.toml not found"
fi

# =============================================================================
# Code Quality Checks
# =============================================================================

section_header "Code Quality Checks"

# Check formatting
echo "Checking code formatting..."
if cargo fmt --all -- --check > /dev/null 2>&1; then
    check_pass "Code is properly formatted"
else
    check_fail "Code formatting issues found (run 'cargo fmt')"
fi

# Check clippy
echo "Running Clippy linter..."
if cargo clippy --workspace --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
    check_pass "No Clippy warnings"
else
    check_fail "Clippy warnings found (run 'cargo clippy')"
fi

# =============================================================================
# Compilation Checks
# =============================================================================

section_header "Compilation Checks"

# Check workspace compilation
echo "Compiling workspace (this may take a while)..."
if cargo check --workspace --all-features > /dev/null 2>&1; then
    check_pass "Workspace compiles successfully"
else
    check_fail "Workspace compilation failed"
fi

# Check release build
echo "Checking release build..."
if cargo build --workspace --release > /dev/null 2>&1; then
    check_pass "Release build successful"
else
    check_fail "Release build failed"
fi

# =============================================================================
# Testing Checks
# =============================================================================

section_header "Testing Checks"

# Run tests
echo "Running test suite..."
if cargo test --workspace --all-features --quiet 2>&1 | grep -q "test result: ok"; then
    check_pass "All tests pass"
else
    check_fail "Some tests failed"
fi

# Check test coverage (if cargo-tarpaulin is installed)
if command -v cargo-tarpaulin &> /dev/null; then
    echo "Checking test coverage..."
    COVERAGE=$(cargo tarpaulin --workspace --all-features --timeout 60 --quiet 2>&1 | grep "%" | tail -1 | awk '{print $1}' | tr -d '%')
    if [ ! -z "$COVERAGE" ]; then
        if (( $(echo "$COVERAGE >= 60" | bc -l) )); then
            check_pass "Test coverage: ${COVERAGE}%"
        else
            check_warn "Test coverage: ${COVERAGE}% (< 60%)"
        fi
    fi
else
    check_warn "cargo-tarpaulin not installed (skipping coverage check)"
fi

# =============================================================================
# Security Checks
# =============================================================================

section_header "Security Checks"

# Check for security advisories
if command -v cargo-audit &> /dev/null; then
    echo "Running security audit..."
    if cargo audit > /dev/null 2>&1; then
        check_pass "No known security vulnerabilities"
    else
        check_fail "Security vulnerabilities found (run 'cargo audit')"
    fi
else
    check_warn "cargo-audit not installed (skipping security check)"
fi

# Check for unwrap() usage (not recommended in production)
echo "Checking for unwrap() calls..."
UNWRAP_COUNT=$(grep -r "\.unwrap()" services/ libs/ 2>/dev/null | wc -l)
if [ "$UNWRAP_COUNT" -eq 0 ]; then
    check_pass "No unwrap() calls found"
elif [ "$UNWRAP_COUNT" -lt 10 ]; then
    check_warn "$UNWRAP_COUNT unwrap() calls found"
else
    check_fail "$UNWRAP_COUNT unwrap() calls found (use proper error handling)"
fi

# Check for TODO/FIXME
echo "Checking for TODO/FIXME comments..."
TODO_COUNT=$(grep -r "TODO\|FIXME" services/ libs/ 2>/dev/null | wc -l)
if [ "$TODO_COUNT" -eq 0 ]; then
    check_pass "No TODO/FIXME comments"
else
    check_warn "$TODO_COUNT TODO/FIXME comments found"
fi

# =============================================================================
# Documentation Checks
# =============================================================================

section_header "Documentation Checks"

# Check for README
if [ -f "README.md" ]; then
    check_pass "README.md exists"
else
    check_fail "README.md not found"
fi

# Check for documentation
echo "Checking documentation..."
if cargo doc --workspace --no-deps > /dev/null 2>&1; then
    check_pass "Documentation builds successfully"
else
    check_fail "Documentation build failed"
fi

# =============================================================================
# Docker Checks
# =============================================================================

section_header "Docker Checks"

# Check for Dockerfile
if [ -f "Dockerfile.production" ]; then
    check_pass "Dockerfile.production exists"
else
    check_fail "Dockerfile.production not found"
fi

# Check for docker-compose
if [ -f "docker-compose.yml" ]; then
    check_pass "docker-compose.yml exists"
else
    check_warn "docker-compose.yml not found"
fi

# =============================================================================
# Kubernetes Checks
# =============================================================================

section_header "Kubernetes Checks"

# Check for K8s manifests
if [ -d "../infrastructure/kubernetes" ]; then
    check_pass "Kubernetes manifests directory exists"

    # Check for required files
    if [ -f "../infrastructure/kubernetes/base/namespace.yaml" ]; then
        check_pass "namespace.yaml exists"
    else
        check_warn "namespace.yaml not found"
    fi

    if [ -f "../infrastructure/kubernetes/base/configmap.yaml" ]; then
        check_pass "configmap.yaml exists"
    else
        check_warn "configmap.yaml not found"
    fi

    if [ -f "../infrastructure/kubernetes/base/secrets.yaml" ]; then
        check_pass "secrets.yaml template exists"
    else
        check_warn "secrets.yaml template not found"
    fi
else
    check_warn "Kubernetes manifests directory not found"
fi

# =============================================================================
# CI/CD Checks
# =============================================================================

section_header "CI/CD Checks"

# Check for GitHub Actions
if [ -f "../.github/workflows/backend-ci.yml" ]; then
    check_pass "GitHub Actions workflow exists"
else
    check_warn "GitHub Actions workflow not found"
fi

# Check for Makefile
if [ -f "Makefile" ]; then
    check_pass "Makefile exists"
else
    check_warn "Makefile not found"
fi

# =============================================================================
# Dependencies Checks
# =============================================================================

section_header "Dependencies Checks"

# Check for outdated dependencies
if command -v cargo-outdated &> /dev/null; then
    echo "Checking for outdated dependencies..."
    OUTDATED=$(cargo outdated --workspace --root-deps-only 2>&1 | grep -c "->")
    if [ "$OUTDATED" -eq 0 ]; then
        check_pass "All dependencies up to date"
    else
        check_warn "$OUTDATED outdated dependencies found"
    fi
else
    check_warn "cargo-outdated not installed (skipping check)"
fi

# =============================================================================
# Performance Checks
# =============================================================================

section_header "Performance Checks"

# Check binary sizes
echo "Checking binary sizes..."
if [ -d "target/release" ]; then
    TOTAL_SIZE=$(du -sh target/release 2>/dev/null | awk '{print $1}')
    check_pass "Release binaries built (total size: $TOTAL_SIZE)"
else
    check_warn "Release binaries not found (run 'cargo build --release')"
fi

# =============================================================================
# Production-Specific Checks
# =============================================================================

section_header "Production-Specific Checks"

# Check for hardcoded credentials
echo "Scanning for potential hardcoded secrets..."
SECRETS_FOUND=0
for pattern in "password" "secret" "api_key" "token" "credential"; do
    COUNT=$(grep -ri "$pattern\s*=\s*\"" services/ libs/ 2>/dev/null | grep -v "example\|test\|TODO" | wc -l)
    if [ "$COUNT" -gt 0 ]; then
        ((SECRETS_FOUND+=COUNT))
    fi
done
if [ "$SECRETS_FOUND" -eq 0 ]; then
    check_pass "No hardcoded secrets found"
else
    check_warn "$SECRETS_FOUND potential hardcoded secrets found (review manually)"
fi

# Check for proper logging
echo "Checking logging configuration..."
if grep -rq "tracing::" services/ libs/; then
    check_pass "Tracing/logging implemented"
else
    check_warn "No tracing/logging found"
fi

# Check for health endpoints
echo "Checking for health endpoints..."
HEALTH_ENDPOINTS=$(grep -r "/health" services/ 2>/dev/null | wc -l)
if [ "$HEALTH_ENDPOINTS" -gt 0 ]; then
    check_pass "Health endpoints implemented"
else
    check_warn "No health endpoints found"
fi

# =============================================================================
# Summary
# =============================================================================

echo ""
echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                     Verification Summary                        ║"
echo "╠══════════════════════════════════════════════════════════════════╣"
echo -e "║  ${GREEN}Passed:${NC}   $PASSED                                                  ║"
echo -e "║  ${YELLOW}Warnings:${NC} $WARNINGS                                                 ║"
echo -e "║  ${RED}Failed:${NC}   $FAILED                                                  ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""

# Overall result
if [ "$FAILED" -eq 0 ]; then
    echo -e "${GREEN}✓ Production Readiness: PASSED${NC}"
    echo ""
    if [ "$WARNINGS" -gt 0 ]; then
        echo -e "${YELLOW}⚠ $WARNINGS warnings found - review recommended${NC}"
    else
        echo -e "${GREEN}✓ All checks passed! System is production-ready.${NC}"
    fi
    echo ""
    exit 0
else
    echo -e "${RED}✗ Production Readiness: FAILED${NC}"
    echo ""
    echo -e "${RED}$FAILED critical issues must be fixed before production deployment.${NC}"
    echo ""
    exit 1
fi
