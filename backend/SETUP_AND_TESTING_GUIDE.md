# Setup and Testing Guide
## Complete Guide to Install Rust, Setup Environment, and Test the LIS Modern Backend

---

## Part 1: Installing Rust and Dependencies

### Step 1: Install Rust

#### On macOS/Linux:
```bash
# Install Rust using rustup (official installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts (usually just press Enter for defaults)

# Restart your terminal or run:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### On Windows:
1. Download rustup-init.exe from https://rustup.rs/
2. Run the installer
3. Follow the prompts
4. Restart your terminal

### Step 2: Install Additional Tools

```bash
# Install SQLx CLI (for database migrations)
cargo install sqlx-cli --no-default-features --features postgres

# Install cargo-watch (for auto-reload during development)
cargo install cargo-watch

# Install cargo-audit (security auditing)
cargo install cargo-audit

# Install cargo-outdated (dependency checking)
cargo install cargo-outdated

# (Optional) Install k6 for load testing
# macOS:
brew install k6

# Linux:
sudo snap install k6

# Windows:
choco install k6
```

### Step 3: Install PostgreSQL

#### On macOS:
```bash
# Using Homebrew
brew install postgresql@15
brew services start postgresql@15

# OR using Docker
docker run -d \
  --name lis-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=lis_main \
  -p 5432:5432 \
  postgres:15-alpine
```

#### On Linux:
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql

# OR using Docker (recommended)
docker run -d \
  --name lis-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=lis_main \
  -p 5432:5432 \
  postgres:15-alpine
```

#### On Windows:
```powershell
# Using Chocolatey
choco install postgresql

# OR use Docker Desktop with the same Docker command as above
```

### Step 4: Verify PostgreSQL Connection

```bash
# Test connection
psql -U postgres -h localhost -d postgres

# If prompted for password, use: postgres
# Type \q to quit
```

---

## Part 2: Project Setup

### Step 1: Navigate to Backend Directory

```bash
cd /Users/macbookpro/Documents/LIS_Modern/backend
```

### Step 2: Create Environment File

```bash
# Create .env file
cat > .env << 'EOF'
# Database Configuration
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/lis_main

# Service Ports
PATIENT_SERVICE_PORT=8090
SAMPLE_SERVICE_PORT=8091
ORDER_SERVICE_PORT=8092
RESULT_SERVICE_PORT=8093
USER_SERVICE_PORT=8094
ORGANIZATION_SERVICE_PORT=8095
EQUIPMENT_SERVICE_PORT=8096
QC_SERVICE_PORT=8097
BILLING_SERVICE_PORT=8098
REPORT_SERVICE_PORT=8099
INVENTORY_SERVICE_PORT=8100
NOTIFICATION_SERVICE_PORT=8101
ANALYTICS_SERVICE_PORT=8102
COMPLIANCE_SERVICE_PORT=8103

# Kafka Configuration (optional for local testing)
KAFKA_BROKERS=localhost:9092
ENABLE_EVENTS=false

# Logging
RUST_LOG=info
EOF
```

### Step 3: Create Databases for Each Service

```bash
# Script to create all databases
#!/bin/bash

services=(
  "patient"
  "sample"
  "order"
  "result"
  "user"
  "organization"
  "equipment"
  "qc"
  "billing"
  "report"
  "inventory"
  "notification"
  "analytics"
  "compliance"
)

for service in "${services[@]}"
do
  echo "Creating database: lis_${service}"
  psql -U postgres -h localhost -c "CREATE DATABASE lis_${service};" || echo "Database lis_${service} might already exist"
done

echo "All databases created!"
```

Save this as `create-databases.sh`, make it executable, and run:

```bash
chmod +x create-databases.sh
./create-databases.sh
```

---

## Part 3: Running Database Migrations

### Option A: Run Migrations for All Services

```bash
#!/bin/bash

services=(
  "patient"
  "sample"
  "order"
  "result"
  "user"
  "organization"
  "equipment"
  "qc"
  "billing"
  "report"
  "inventory"
  "notification"
  "analytics"
  "compliance"
)

for service in "${services[@]}"
do
  echo "======================================"
  echo "Running migrations for $service-service"
  echo "======================================"

  cd services/${service}-service

  DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_${service}" \
    sqlx migrate run

  cd ../..
done

echo "All migrations completed!"
```

Save as `run-migrations.sh`, make executable, and run:

```bash
chmod +x run-migrations.sh
./run-migrations.sh
```

### Option B: Run Migrations Individually

```bash
# For a specific service (e.g., patient-service)
cd services/patient-service

DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_patient" \
  sqlx migrate run

# Check migration status
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_patient" \
  sqlx migrate info
```

---

## Part 4: Building the Project

### Full Workspace Build

```bash
# Check for compilation errors (fast)
cargo check --workspace

# Build in debug mode (with debug symbols)
cargo build --workspace

# Build in release mode (optimized)
cargo build --workspace --release
```

**Expected Time**:
- First build: 5-10 minutes (downloads dependencies)
- Subsequent builds: 30 seconds - 2 minutes

### Build Individual Service

```bash
# Build specific service
cargo build --package patient-service

# Build in release mode
cargo build --package patient-service --release
```

---

## Part 5: Running Tests

### 5.1 Code Quality Checks

```bash
# Format check
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Fix common issues automatically
cargo clippy --all-targets --all-features --fix
```

### 5.2 Unit Tests

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run tests for specific service
cargo test --package patient-service

# Run specific test
cargo test --package result-service test_range_check_within_range

# Run tests in parallel
cargo test --workspace -- --test-threads=4
```

### 5.3 Integration Tests

First, create test databases:

```bash
# Create test database
psql -U postgres -h localhost -c "CREATE DATABASE lis_patient_test;"

# Set test database URL
export TEST_DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_patient_test"

# Run migrations on test database
cd services/patient-service
sqlx migrate run --database-url $TEST_DATABASE_URL
cd ../..

# Run integration tests
cargo test --package patient-service --test integration_tests
```

### 5.4 Run Complete Test Suite

```bash
# Make test script executable (if not already)
chmod +x run-all-tests.sh

# Run all tests
./run-all-tests.sh
```

**Expected Output**:
```
================================================
LIS Modern Backend - Comprehensive Test Suite
================================================

================================================
1. CODE QUALITY CHECKS
================================================
Running: Cargo Format Check
✓ PASSED: Cargo Format Check
...

================================================
TEST SUMMARY
================================================

Total Tests Run: XX
Passed: XX
Failed: 0

Pass Rate: 100%
================================================
ALL TESTS PASSED! ✓
Backend is ready for deployment!
================================================
```

---

## Part 6: Running Services

### 6.1 Run Single Service

```bash
# Run patient service
cd services/patient-service

DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_patient" \
HOST="0.0.0.0" \
PORT="8090" \
cargo run

# Service should start and display:
# INFO Starting Patient Service on 0.0.0.0:8090
# INFO Connected to database
# INFO Database migrations completed
# INFO GraphQL schema built
```

### 6.2 Run All Services (Multiple Terminals)

**Terminal 1 - Patient Service**:
```bash
cd services/patient-service
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_patient" PORT=8090 cargo run
```

**Terminal 2 - Analytics Service**:
```bash
cd services/analytics-service
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_analytics" PORT=8093 cargo run
```

**Terminal 3 - Compliance Service**:
```bash
cd services/compliance-service
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_compliance" PORT=8094 cargo run
```

... (repeat for other services)

### 6.3 Run All Services with tmux (Recommended)

```bash
# Install tmux
# macOS: brew install tmux
# Linux: sudo apt install tmux

# Create a script to run all services
cat > run-all-services.sh << 'EOF'
#!/bin/bash

tmux new-session -d -s lis-services

services=(
  "patient:8090"
  "sample:8091"
  "order:8092"
  "result:8093"
  "analytics:8102"
  "compliance:8103"
)

for service_port in "${services[@]}"
do
  service="${service_port%%:*}"
  port="${service_port##*:}"

  tmux new-window -t lis-services -n "$service"
  tmux send-keys -t lis-services:$service \
    "cd services/${service}-service && DATABASE_URL=postgresql://postgres:postgres@localhost:5432/lis_${service} PORT=${port} cargo run" C-m
done

tmux attach-session -t lis-services
EOF

chmod +x run-all-services.sh
./run-all-services.sh
```

**tmux Commands**:
- `Ctrl+B` then `n` - Next window
- `Ctrl+B` then `p` - Previous window
- `Ctrl+B` then `d` - Detach (services keep running)
- `tmux attach -t lis-services` - Reattach
- `Ctrl+B` then `&` - Kill window

---

## Part 7: Testing APIs

### 7.1 Health Checks

```bash
# Test all service health endpoints
for port in 8090 8091 8092 8093 8102 8103
do
  echo "Testing service on port $port..."
  curl -s http://localhost:${port}/health | jq
done
```

**Expected Response**:
```json
{
  "status": "healthy",
  "service": "patient-service",
  "version": "0.1.0"
}
```

### 7.2 GraphQL Playground

1. Open browser to: `http://localhost:8090/graphql` (patient service)
2. You'll see GraphQL Playground interface

**Test Query**:
```graphql
query {
  __schema {
    queryType {
      name
    }
    mutationType {
      name
    }
  }
}
```

### 7.3 Sample GraphQL Queries

**Create Organization** (run on port 8095):
```graphql
mutation {
  createOrganization(
    input: {
      organizationName: "Test Laboratory"
      organizationCode: "TEST-LAB"
      licenseNumber: "LIC-12345"
      contactEmail: "info@testlab.com"
      contactPhone: "9876543210"
    }
    createdBy: "admin-user-id"
  ) {
    id
    organizationName
    organizationCode
  }
}
```

**Create Patient** (run on port 8090):
```graphql
mutation {
  createPatient(
    input: {
      firstName: "John"
      lastName: "Doe"
      dateOfBirth: "1990-01-15"
      gender: MALE
      mobile: "9876543210"
      email: "john.doe@example.com"
    }
    organizationId: "YOUR-ORG-ID"
    createdBy: "YOUR-USER-ID"
  ) {
    id
    mrn
    fullName
  }
}
```

**Get Dashboard** (run on port 8102):
```graphql
query {
  dashboard(
    organizationId: "YOUR-ORG-ID"
    role: "LAB_DIRECTOR"
  ) {
    role
    metrics {
      name
      value
      unit
      status
    }
  }
}
```

---

## Part 8: Performance Testing

### 8.1 Run k6 Load Tests

```bash
# Install k6 if not already installed
# macOS: brew install k6
# Linux: sudo snap install k6

# Run performance tests
k6 run performance-test.js

# Run with custom duration
k6 run --duration 10m performance-test.js

# Run with custom VUs
k6 run --vus 50 --duration 5m performance-test.js
```

**Expected Results**:
```
execution: local
    script: performance-test.js
    output: -

  scenarios: (100.00%) 1 scenario, 200 max VUs, 12m30s max duration
           * default: Up to 200 looping VUs for 12m0s over 5 stages

  ✓ patient query status is 200
  ✓ patient query response time < 100ms
  ✓ dashboard query status is 200

  checks.........................: 100.00% ✓ 50000 ✗ 0
  http_req_duration..............: avg=45ms    p(95)=120ms  p(99)=250ms
  http_req_failed................: 0.00%   ✓ 0    ✗ 50000
  iterations.....................: 25000   ~208/s
  vus............................: 200     min=20   max=200
```

### 8.2 Stress Testing

```bash
# Gradually increase load
k6 run --vus 10 --duration 1m performance-test.js
k6 run --vus 50 --duration 2m performance-test.js
k6 run --vus 100 --duration 3m performance-test.js
k6 run --vus 200 --duration 5m performance-test.js
```

Monitor:
- CPU usage: `top` or `htop`
- Memory: `ps aux | grep cargo`
- Database: `SELECT * FROM pg_stat_activity;`

---

## Part 9: Monitoring and Debugging

### 9.1 View Logs

```bash
# Run with verbose logging
RUST_LOG=debug cargo run --package patient-service

# Run with specific module logging
RUST_LOG=patient_service=debug,sqlx=info cargo run
```

### 9.2 Database Monitoring

```sql
-- Connect to database
psql -U postgres -h localhost -d lis_patient

-- Check active connections
SELECT count(*) FROM pg_stat_activity WHERE datname = 'lis_patient';

-- Check slow queries
SELECT pid, now() - pg_stat_activity.query_start AS duration, query
FROM pg_stat_activity
WHERE state = 'active'
ORDER BY duration DESC;

-- Check table sizes
SELECT
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) AS size
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;
```

### 9.3 Performance Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Run with profiling
cargo flamegraph --package patient-service

# View flamegraph.svg in browser
```

---

## Part 10: Troubleshooting

### Common Issues

#### Issue 1: "error: linker `cc` not found"
**Solution**:
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt install build-essential

# Fedora/RHEL
sudo dnf install gcc
```

#### Issue 2: "connection to server failed"
**Solution**:
```bash
# Check PostgreSQL is running
# macOS (Homebrew)
brew services list

# Linux
sudo systemctl status postgresql

# Docker
docker ps | grep postgres

# If not running, start it
docker start lis-postgres
```

#### Issue 3: "database does not exist"
**Solution**:
```bash
# Create the database
psql -U postgres -h localhost -c "CREATE DATABASE lis_patient;"

# Run migrations
cd services/patient-service
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/lis_patient" sqlx migrate run
```

#### Issue 4: "port already in use"
**Solution**:
```bash
# Find process using port
lsof -i :8090

# Kill the process
kill -9 <PID>

# Or use different port
PORT=8190 cargo run
```

#### Issue 5: Build is very slow
**Solution**:
```bash
# Use sccache for faster rebuilds
cargo install sccache
export RUSTC_WRAPPER=sccache

# Or use mold linker (Linux)
cargo install mold
```

---

## Part 11: Deployment Preparation

### 11.1 Production Build

```bash
# Build all services in release mode
cargo build --workspace --release

# Binaries will be in target/release/
ls -lh target/release/*-service
```

### 11.2 Docker Build

Create `Dockerfile` for each service:

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

RUN cargo build --release --package patient-service

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/patient-service /usr/local/bin/

ENV DATABASE_URL=postgresql://postgres:postgres@localhost:5432/lis_patient
ENV HOST=0.0.0.0
ENV PORT=8090

EXPOSE 8090

CMD ["patient-service"]
```

Build and run:

```bash
# Build Docker image
docker build -t lis-patient-service -f services/patient-service/Dockerfile .

# Run container
docker run -d \
  --name patient-service \
  -e DATABASE_URL="postgresql://postgres:postgres@host.docker.internal:5432/lis_patient" \
  -p 8090:8090 \
  lis-patient-service
```

---

## Part 12: Success Criteria

Your setup is complete when:

- [ ] Rust installed (`rustc --version` works)
- [ ] PostgreSQL running (`psql -U postgres -h localhost` connects)
- [ ] All databases created (14 databases)
- [ ] All migrations run successfully
- [ ] `cargo check --workspace` passes with no errors
- [ ] `cargo test --workspace` passes all tests
- [ ] Services start without errors
- [ ] Health endpoints return 200 OK
- [ ] GraphQL playground is accessible
- [ ] Sample queries execute successfully
- [ ] Load tests complete successfully

---

## Quick Start Commands

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Install tools
cargo install sqlx-cli --no-default-features --features postgres

# 3. Start PostgreSQL
docker run -d --name lis-postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:15-alpine

# 4. Create databases and run migrations
cd /Users/macbookpro/Documents/LIS_Modern/backend
./create-databases.sh
./run-migrations.sh

# 5. Run tests
./run-all-tests.sh

# 6. Start services
./run-all-services.sh

# 7. Test health
curl http://localhost:8090/health

# 8. Open GraphQL Playground
open http://localhost:8090/graphql
```

---

## Support

If you encounter issues:

1. Check the troubleshooting section above
2. Review logs with `RUST_LOG=debug`
3. Verify PostgreSQL is running
4. Check CODE_ANALYSIS_REPORT.md for known issues
5. Ensure all dependencies are installed

**Success!** Your LIS Modern Backend is now ready for development and testing! 🎉
