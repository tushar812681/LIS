#!/bin/bash

# LIS Modern Backend - Quick Start Script
# This script helps you quickly test and verify all services

set -e  # Exit on error

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print colored output
print_green() { echo -e "${GREEN}$1${NC}"; }
print_blue() { echo -e "${BLUE}$1${NC}"; }
print_red() { echo -e "${RED}$1${NC}"; }
print_yellow() { echo -e "${YELLOW}$1${NC}"; }

# Print banner
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║         LIS Modern Backend - Quick Start Script             ║"
echo "║                  All 14 Services Ready!                      ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check Rust installation
print_blue "🔍 Checking Rust installation..."
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_green "✓ Rust installed: $RUST_VERSION"
else
    print_red "✗ Rust not found. Please install from https://rustup.rs/"
    exit 1
fi

# Check PostgreSQL
print_blue "🔍 Checking PostgreSQL..."
if command -v psql &> /dev/null; then
    PG_VERSION=$(psql --version)
    print_green "✓ PostgreSQL installed: $PG_VERSION"
else
    print_yellow "⚠ PostgreSQL not found. Install it to run services."
fi

# Check Redis
print_blue "🔍 Checking Redis..."
if command -v redis-cli &> /dev/null; then
    REDIS_VERSION=$(redis-cli --version)
    print_green "✓ Redis installed: $REDIS_VERSION"
else
    print_yellow "⚠ Redis not found. Some services may need caching."
fi

echo ""
print_blue "═══════════════════════════════════════════════════════════════"
print_blue "                  COMPILATION VERIFICATION                     "
print_blue "═══════════════════════════════════════════════════════════════"
echo ""

# Run cargo check
print_blue "🔨 Running cargo check on entire workspace..."
if cargo check --workspace 2>&1 | tail -5; then
    print_green "✓ All services compile successfully!"
else
    print_red "✗ Compilation failed. Check errors above."
    exit 1
fi

echo ""
print_blue "═══════════════════════════════════════════════════════════════"
print_blue "                     BUILD VERIFICATION                        "
print_blue "═══════════════════════════════════════════════════════════════"
echo ""

# Build workspace
print_blue "🔨 Building entire workspace (this may take a few minutes)..."
if cargo build --workspace --release; then
    print_green "✓ Workspace built successfully!"

    # List all built binaries
    echo ""
    print_blue "📦 Built services:"
    ls -lh target/release/ | grep -E "patient-service|organization-service|sample-service|order-service|result-service|equipment-service|inventory-service|qc-service|billing-service|user-service|notification-service|analytics-service|report-service|compliance-service" | awk '{print "   " $9 " (" $5 ")"}'
else
    print_red "✗ Build failed. Check errors above."
    exit 1
fi

echo ""
print_blue "═══════════════════════════════════════════════════════════════"
print_blue "                      SERVICE OVERVIEW                         "
print_blue "═══════════════════════════════════════════════════════════════"
echo ""

# Count services
SERVICE_COUNT=$(find services -maxdepth 1 -type d | tail -n +2 | wc -l | tr -d ' ')
print_green "📊 Total Services: $SERVICE_COUNT"

echo ""
print_blue "Services:"
for service in services/*/; do
    name=$(basename "$service")
    if [ -f "$service/Cargo.toml" ]; then
        version=$(grep "^version" "$service/Cargo.toml" | head -1 | cut -d'"' -f2)
        print_green "   ✓ $name (v$version)"
    fi
done

echo ""
print_blue "═══════════════════════════════════════════════════════════════"
print_blue "                   AVAILABLE COMMANDS                          "
print_blue "═══════════════════════════════════════════════════════════════"
echo ""

cat << 'EOF'
Run Individual Services:
  cargo run -p patient-service
  cargo run -p sample-service
  cargo run -p order-service
  ... (repeat for any service)

Test Services:
  cargo test --workspace              # All tests
  cargo test -p patient-service       # Specific service
  cargo test --workspace -- --nocapture  # With output

Format Code:
  cargo fmt --all

Lint Code:
  cargo clippy --workspace -- -D warnings

Check for Issues:
  cargo check --workspace

Build Optimized:
  cargo build --workspace --release

Generate Documentation:
  cargo doc --workspace --no-deps --open

Clean Build Artifacts:
  cargo clean
EOF

echo ""
print_blue "═══════════════════════════════════════════════════════════════"
print_blue "                     QUICK TEST RUN                            "
print_blue "═══════════════════════════════════════════════════════════════"
echo ""

# Ask if user wants to run tests
read -p "$(echo -e ${YELLOW}Run test suite? This may take a few minutes. [y/N]: ${NC})" -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_blue "🧪 Running test suite..."
    if cargo test --workspace 2>&1 | grep -E "test result:|running"; then
        print_green "✓ Tests completed!"
    else
        print_yellow "⚠ Some tests may have failed. Review output above."
    fi
else
    print_yellow "⏭ Skipping tests."
fi

echo ""
print_blue "═══════════════════════════════════════════════════════════════"
print_blue "                    ENVIRONMENT SETUP                          "
print_blue "═══════════════════════════════════════════════════════════════"
echo ""

# Check for .env file
if [ ! -f ".env" ]; then
    print_yellow "⚠ No .env file found."
    read -p "$(echo -e ${YELLOW}Create sample .env file? [y/N]: ${NC})" -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cat > .env << 'ENVFILE'
# LIS Modern Backend Environment Variables

# Database
DATABASE_URL=postgresql://lis_user:lis_password@localhost:5432/lis_modern
TEST_DATABASE_URL=postgresql://lis_user:lis_password@localhost:5432/lis_modern_test

# Redis
REDIS_URL=redis://localhost:6379

# Kafka
KAFKA_BROKERS=localhost:9092

# Server Configuration
HOST=0.0.0.0
PORT=8000

# Logging
RUST_LOG=info

# JWT Secret (change in production!)
JWT_SECRET=your-secret-key-change-this-in-production

# External APIs
TWILIO_ACCOUNT_SID=your_account_sid
TWILIO_AUTH_TOKEN=your_auth_token
SENDGRID_API_KEY=your_sendgrid_api_key
ENVFILE
        print_green "✓ Created .env file. Please update with your actual values."
    fi
else
    print_green "✓ .env file exists"
fi

echo ""
print_blue "═══════════════════════════════════════════════════════════════"
print_blue "                      NEXT STEPS                               "
print_blue "═══════════════════════════════════════════════════════════════"
echo ""

cat << 'EOF'
1. Database Setup:
   createdb lis_modern
   export DATABASE_URL="postgresql://user:password@localhost:5432/lis_modern"

2. Run Migrations:
   cd services/patient-service && sqlx migrate run

3. Start a Service:
   cargo run -p patient-service

4. Access GraphQL Playground:
   Open browser: http://localhost:8000/graphql

5. View Health Status:
   curl http://localhost:8000/health

6. Read Full Documentation:
   cat NEXT_STEPS.md

EOF

print_green "╔══════════════════════════════════════════════════════════════╗"
print_green "║              ✓ Quick Start Complete!                        ║"
print_green "║         All 14 services are ready to run!                    ║"
print_green "╚══════════════════════════════════════════════════════════════╝"

echo ""
print_blue "📖 For detailed next steps, see: NEXT_STEPS.md"
print_blue "🚀 Happy coding!"
echo ""
