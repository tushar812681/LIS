# Comprehensive Testing Guide - LIS Modern Backend

## Testing Strategy Overview

This guide covers all aspects of testing for the Laboratory Information System backend, ensuring 100% production readiness, performance, and reliability.

---

## 1. Unit Testing

### Domain Model Tests
**Location**: `libs/common/tests/domain_model_tests.rs`

**Coverage**:
- MRN generation uniqueness
- Sample ID generation
- Accession number formatting
- Error handling validation
- Pagination logic
- Type conversions

**Run Commands**:
```bash
# Run all unit tests
cargo test --lib

# Run specific test module
cargo test --lib domain_model_tests

# Run with output
cargo test --lib -- --nocapture
```

**Expected Results**:
- All ID generators should produce unique values
- Error types should match expected patterns
- Pagination calculations should be accurate
- No panics or unwraps should fail

---

## 2. Integration Testing

### Service-Level Tests
**Locations**:
- `services/patient-service/tests/integration_tests.rs`
- `services/result-service/tests/auto_verification_tests.rs`

**Coverage**:
- Database CRUD operations
- Service layer business logic
- Repository pattern validation
- Transaction handling
- Constraint enforcement
- Soft delete functionality

**Prerequisites**:
```bash
# Start PostgreSQL test database
docker run -d \
  --name lis-test-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=lis_test \
  -p 5433:5432 \
  postgres:15-alpine

# Set test database URL
export TEST_DATABASE_URL="postgresql://postgres:postgres@localhost:5433/lis_test"
```

**Run Commands**:
```bash
# Run integration tests
cargo test --test integration_tests

# Run with database cleanup
cargo test --test integration_tests -- --test-threads=1
```

**Expected Results**:
- Patient creation should succeed with valid data
- Duplicate constraints should be enforced
- Soft deletes should preserve data
- Relationships should be maintained
- Auto-verification logic should work correctly

---

## 3. Database Migration Testing

### Migration Validation
```bash
# Test all migrations
for service in patient sample order result user organization equipment qc billing report inventory notification analytics compliance
do
  echo "=== Testing $service-service migrations ==="
  cd services/${service}-service

  # Run migrations
  DATABASE_URL="postgresql://postgres:postgres@localhost:5433/lis_${service}_test" \
    sqlx migrate run

  # Verify migration status
  DATABASE_URL="postgresql://postgres:postgres@localhost:5433/lis_${service}_test" \
    sqlx migrate info

  cd ../..
done
```

**Validation Checklist**:
- [ ] All migrations run without errors
- [ ] Schema matches expected structure
- [ ] Indexes are created correctly
- [ ] Foreign keys are valid
- [ ] Enums are properly defined
- [ ] Default values are set
- [ ] Triggers are functional

**Rollback Testing**:
```bash
# Test migration rollback (if supported)
sqlx migrate revert
sqlx migrate run
```

---

## 4. GraphQL API Testing

### Endpoint Testing
**Tool**: Postman, Insomnia, or curl

**Patient Service Tests**:
```graphql
# Test 1: Create Patient
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
    organizationId: "ORG-UUID"
    createdBy: "USER-UUID"
  ) {
    id
    mrn
    fullName
  }
}

# Test 2: Query Patient
query {
  patient(id: "PATIENT-UUID") {
    id
    firstName
    lastName
    mrn
    mobile
    email
  }
}

# Test 3: Search Patients
query {
  searchPatients(
    organizationId: "ORG-UUID"
    query: "John"
    limit: 10
  ) {
    id
    fullName
    mrn
  }
}
```

**Analytics Service Tests**:
```graphql
# Test: Get Dashboard
query {
  dashboard(organizationId: "ORG-UUID", role: "LAB_DIRECTOR") {
    role
    metrics {
      name
      value
      unit
      status
    }
    charts {
      chartType
      title
    }
  }
}

# Test: TAT Analytics
query {
  tatAnalytics(organizationId: "ORG-UUID", days: 30) {
    meanTatHours
    complianceRate
    totalOrders
  }
}
```

**Compliance Service Tests**:
```graphql
# Test: Create CAPA
mutation {
  createCapa(
    input: {
      capaType: CORRECTIVE
      priority: HIGH
      title: "Sample Rejection Issue"
      description: "High rejection rate in hematology"
      dateIdentified: "2025-01-06"
      source: "INTERNAL_AUDIT"
    }
    organizationId: "ORG-UUID"
    createdBy: "USER-UUID"
  ) {
    id
    capaNumber
    capaStatus
  }
}

# Test: Get Compliance Dashboard
query {
  complianceDashboard(organizationId: "ORG-UUID") {
    openCapas
    overdueCapas
    pendingDocumentReviews
    expiredTrainings
    qualityIndicatorsCritical
  }
}
```

**Performance Benchmarks**:
- Simple queries: < 100ms
- Complex aggregations: < 500ms
- Dashboard generation: < 1s
- Batch operations: < 5s

---

## 5. Performance Testing

### Load Testing with k6

**Install k6**:
```bash
# macOS
brew install k6

# Linux
sudo apt-get install k6
```

**Load Test Script** (`load-test.js`):
```javascript
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '2m', target: 100 },  // Ramp up to 100 users
    { duration: '5m', target: 100 },  // Stay at 100 users
    { duration: '2m', target: 200 },  // Ramp to 200 users
    { duration: '5m', target: 200 },  // Stay at 200 users
    { duration: '2m', target: 0 },    // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'], // 95% of requests < 500ms
    http_req_failed: ['rate<0.01'],   // Error rate < 1%
  },
};

export default function () {
  // Test patient query
  let query = `
    query {
      patient(id: "test-uuid") {
        id
        fullName
      }
    }
  `;

  let response = http.post('http://localhost:8090/graphql', JSON.stringify({
    query: query
  }), {
    headers: { 'Content-Type': 'application/json' },
  });

  check(response, {
    'status is 200': (r) => r.status === 200,
    'response time < 500ms': (r) => r.timings.duration < 500,
  });

  sleep(1);
}
```

**Run Load Tests**:
```bash
k6 run load-test.js
```

**Performance Targets**:
- **Throughput**: 1000+ requests/second
- **Response Time P95**: < 500ms
- **Response Time P99**: < 1000ms
- **Error Rate**: < 0.1%
- **Concurrent Users**: Support 500+ concurrent users
- **Database Connections**: Pooled with max 32 per service

---

## 6. Stress Testing

### Database Connection Pool Testing
```rust
#[tokio::test]
async fn test_connection_pool_exhaustion() {
    let pool = PgPoolOptions::new()
        .max_connections(32)
        .connect("postgresql://...")
        .await
        .unwrap();

    let mut handles = vec![];

    // Spawn 100 concurrent requests
    for _ in 0..100 {
        let pool_clone = pool.clone();
        handles.push(tokio::spawn(async move {
            let _result = sqlx::query("SELECT 1")
                .fetch_one(&pool_clone)
                .await;
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Should not panic or deadlock
}
```

### Memory Leak Testing
```bash
# Run service with memory profiling
cargo run --release &
SERVICE_PID=$!

# Monitor memory usage over time
for i in {1..60}; do
  ps -o rss= -p $SERVICE_PID
  sleep 60
done

# Memory should stabilize, not continuously grow
```

---

## 7. Security Testing

### SQL Injection Prevention
```rust
#[tokio::test]
async fn test_sql_injection_prevention() {
    let malicious_input = "'; DROP TABLE patient; --";

    // This should be safely parameterized
    let result = sqlx::query("SELECT * FROM patient WHERE mobile = $1")
        .bind(malicious_input)
        .fetch_optional(&pool)
        .await;

    // Should not execute SQL injection
    assert!(result.is_ok());
}
```

### Input Validation Tests
```rust
#[test]
fn test_email_validation() {
    let invalid_emails = vec![
        "notanemail",
        "@example.com",
        "test@",
        "test @example.com",
    ];

    for email in invalid_emails {
        // Validation should fail
        assert!(validate_email(email).is_err());
    }
}

#[test]
fn test_mobile_validation() {
    let invalid_mobiles = vec![
        "123",           // Too short
        "abcdefghij",    // Not numeric
        "12345678901",   // Too long (India)
    ];

    for mobile in invalid_mobiles {
        assert!(validate_mobile(mobile).is_err());
    }
}
```

### Authorization Tests
```rust
#[tokio::test]
async fn test_cross_organization_access_prevention() {
    let org_a_patient = create_patient(org_a_id).await;

    // User from org_b should not access org_a patient
    let result = patient_service
        .get_patient(org_a_patient.id, org_b_id)
        .await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Unauthorized));
}
```

---

## 8. Compliance & Audit Testing

### Audit Trail Verification
```rust
#[tokio::test]
async fn test_audit_log_creation() {
    // Create a patient
    let patient = create_patient().await;

    // Verify audit log exists
    let audit_logs = sqlx::query_as::<_, AuditLog>(
        "SELECT * FROM audit_log WHERE entity_id = $1 AND action = 'CREATE'"
    )
    .bind(patient.id)
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(audit_logs.len(), 1);
    assert_eq!(audit_logs[0].entity_type, "PATIENT");
}
```

### NABL Compliance Tests
```rust
#[tokio::test]
async fn test_quality_indicator_tracking() {
    // Record quality indicator value
    let value = record_quality_indicator().await;

    // Verify status calculation
    assert!(value.indicator_status == IndicatorStatus::OnTarget
         || value.indicator_status == IndicatorStatus::Warning
         || value.indicator_status == IndicatorStatus::Critical);
}
```

---

## 9. Error Handling & Recovery Testing

### Database Failure Simulation
```rust
#[tokio::test]
async fn test_database_connection_failure_handling() {
    // Disconnect database
    pool.close().await;

    // Service should return proper error
    let result = patient_service.get_patient(patient_id).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Database(_)));
}
```

### Timeout Testing
```rust
#[tokio::test]
async fn test_query_timeout() {
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        long_running_query()
    ).await;

    // Should timeout if query takes too long
    if result.is_err() {
        // Handle timeout gracefully
    }
}
```

---

## 10. Continuous Integration Tests

### GitHub Actions Workflow (`.github/workflows/test.yml`)
```yaml
name: Backend Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        run: cargo test --workspace --all-features
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/lis_test

      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check
```

---

## 11. Performance Benchmarks

### Expected Performance Metrics

| Operation | Target | Critical |
|-----------|--------|----------|
| Patient Creation | < 50ms | < 100ms |
| Patient Query by ID | < 10ms | < 50ms |
| Patient Search | < 100ms | < 200ms |
| Result Creation + Auto-verification | < 100ms | < 200ms |
| Dashboard Generation | < 500ms | < 1s |
| Compliance Report | < 1s | < 2s |
| Batch Operations (100 items) | < 5s | < 10s |

### Database Query Optimization
```sql
-- Verify indexes exist
SELECT tablename, indexname, indexdef
FROM pg_indexes
WHERE schemaname = 'public'
ORDER BY tablename, indexname;

-- Check slow queries
SELECT query, calls, total_time, mean_time
FROM pg_stat_statements
ORDER BY mean_time DESC
LIMIT 10;

-- Analyze query plans
EXPLAIN ANALYZE
SELECT * FROM patient
WHERE organization_id = 'xxx' AND is_deleted = FALSE
LIMIT 20;
```

---

## 12. Monitoring & Observability

### Metrics to Track
```rust
// Prometheus metrics
use prometheus::{IntCounter, Histogram};

lazy_static! {
    static ref HTTP_REQUESTS: IntCounter =
        IntCounter::new("http_requests_total", "Total HTTP requests").unwrap();

    static ref RESPONSE_TIME: Histogram =
        Histogram::new("http_response_time_seconds", "HTTP response time").unwrap();

    static ref DB_QUERIES: IntCounter =
        IntCounter::new("db_queries_total", "Total database queries").unwrap();
}
```

### Health Check Tests
```bash
# Test health endpoints
curl http://localhost:8090/health
# Expected: {"status":"healthy","service":"patient-service","version":"0.1.0"}

curl http://localhost:8090/ready
# Expected: {"status":"ready"}
```

---

## 13. Test Execution Checklist

### Pre-Production Testing
- [ ] All unit tests pass (100% of domain logic)
- [ ] All integration tests pass
- [ ] Database migrations successful on all services
- [ ] GraphQL APIs respond correctly
- [ ] Load testing meets performance targets
- [ ] No memory leaks detected
- [ ] SQL injection tests pass
- [ ] Authorization tests pass
- [ ] Audit logging works correctly
- [ ] Error handling graceful
- [ ] Health checks functional
- [ ] Documentation complete

### Post-Deployment Testing
- [ ] Smoke tests on production
- [ ] Monitor error rates
- [ ] Monitor response times
- [ ] Monitor database connections
- [ ] Monitor memory usage
- [ ] Verify backup procedures
- [ ] Test disaster recovery

---

## 14. Test Data Generation

### Seed Test Data
```sql
-- Create test organization
INSERT INTO organization (id, organization_name, organization_code)
VALUES ('ORG-TEST-001', 'Test Laboratory', 'TEST-LAB');

-- Create test users
INSERT INTO "user" (id, organization_id, username, email, role)
VALUES
  ('USER-001', 'ORG-TEST-001', 'admin', 'admin@test.com', 'ADMIN'),
  ('USER-002', 'ORG-TEST-001', 'technician', 'tech@test.com', 'LAB_TECHNICIAN');

-- Create test patients
INSERT INTO patient (id, organization_id, mrn, first_name, last_name, date_of_birth, gender, mobile, created_by)
SELECT
  gen_random_uuid(),
  'ORG-TEST-001',
  'MRN-' || lpad(generate_series::text, 6, '0'),
  'Patient',
  'Test' || generate_series,
  '1990-01-01'::date + (random() * 10000)::int,
  CASE WHEN random() < 0.5 THEN 'MALE' ELSE 'FEMALE' END,
  '98765' || lpad(generate_series::text, 5, '0'),
  'USER-001'
FROM generate_series(1, 1000);
```

---

## 15. Troubleshooting Guide

### Common Issues

**Issue**: Tests fail with "connection refused"
```bash
# Solution: Ensure PostgreSQL is running
docker ps | grep postgres
# Restart if needed
docker start lis-test-db
```

**Issue**: Migration errors
```bash
# Solution: Reset database
sqlx database drop
sqlx database create
sqlx migrate run
```

**Issue**: Slow tests
```bash
# Solution: Run tests in parallel
cargo test -- --test-threads=4

# Or run specific tests
cargo test test_name
```

---

## Conclusion

This comprehensive testing strategy ensures:
- ✅ **Correctness**: All business logic works as expected
- ✅ **Performance**: Meets SLA requirements under load
- ✅ **Security**: Protected against common vulnerabilities
- ✅ **Reliability**: Handles errors gracefully
- ✅ **Compliance**: Meets NABL audit requirements
- ✅ **Maintainability**: Tests serve as documentation

Run the full test suite before each deployment:
```bash
./run-all-tests.sh
```
