// k6 Performance Test Script for LIS Modern Backend
// Run with: k6 run performance-test.js

import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend, Counter } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');
const patientQueryDuration = new Trend('patient_query_duration');
const dashboardQueryDuration = new Trend('dashboard_query_duration');
const resultCreationDuration = new Trend('result_creation_duration');
const requestCounter = new Counter('total_requests');

// Test configuration
export const options = {
  stages: [
    { duration: '30s', target: 20 },   // Warm-up: Ramp to 20 users
    { duration: '1m', target: 50 },    // Load: Ramp to 50 users
    { duration: '3m', target: 50 },    // Sustain: Stay at 50 users
    { duration: '1m', target: 100 },   // Stress: Ramp to 100 users
    { duration: '3m', target: 100 },   // Sustain: Stay at 100 users
    { duration: '1m', target: 200 },   // Peak: Ramp to 200 users
    { duration: '2m', target: 200 },   // Sustain: Stay at 200 users
    { duration: '30s', target: 0 },    // Cool-down: Ramp down
  ],
  thresholds: {
    'http_req_duration': ['p(95)<500', 'p(99)<1000'], // 95% < 500ms, 99% < 1s
    'http_req_failed': ['rate<0.01'],                  // Error rate < 1%
    'errors': ['rate<0.01'],
    'patient_query_duration': ['p(95)<100'],           // Patient queries < 100ms
    'dashboard_query_duration': ['p(95)<1000'],        // Dashboard < 1s
    'result_creation_duration': ['p(95)<200'],         // Result creation < 200ms
  },
};

// Base URLs for services
const BASE_URLS = {
  patient: __ENV.PATIENT_SERVICE_URL || 'http://localhost:8090',
  analytics: __ENV.ANALYTICS_SERVICE_URL || 'http://localhost:8093',
  result: __ENV.RESULT_SERVICE_URL || 'http://localhost:8091',
  compliance: __ENV.COMPLIANCE_SERVICE_URL || 'http://localhost:8094',
};

// Test data
const TEST_ORG_ID = 'a1b2c3d4-e5f6-4a5b-8c9d-0e1f2a3b4c5d';
const TEST_PATIENT_ID = 'p1a2t3i4-e5n6-7t8i-9d0a-b1c2d3e4f5a6';

// Helper function to execute GraphQL query
function executeGraphQL(url, query, variables = {}) {
  const payload = JSON.stringify({
    query: query,
    variables: variables,
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  return http.post(`${url}/graphql`, payload, params);
}

// Test scenarios
export default function () {
  const scenario = Math.random();

  // 40% Patient queries
  if (scenario < 0.4) {
    testPatientQueries();
  }
  // 30% Dashboard queries
  else if (scenario < 0.7) {
    testDashboardQueries();
  }
  // 20% Result operations
  else if (scenario < 0.9) {
    testResultOperations();
  }
  // 10% Compliance operations
  else {
    testComplianceOperations();
  }

  sleep(1);
}

function testPatientQueries() {
  // Test 1: Get patient by ID
  const getPatientQuery = `
    query GetPatient($id: String!) {
      patient(id: $id) {
        id
        mrn
        firstName
        lastName
        fullName
        dateOfBirth
        gender
        mobile
        email
      }
    }
  `;

  const startTime = Date.now();
  const response = executeGraphQL(
    BASE_URLS.patient,
    getPatientQuery,
    { id: TEST_PATIENT_ID }
  );

  const duration = Date.now() - startTime;
  patientQueryDuration.add(duration);
  requestCounter.add(1);

  const success = check(response, {
    'patient query status is 200': (r) => r.status === 200,
    'patient query has data': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.data && body.data.patient;
      } catch (e) {
        return false;
      }
    },
    'patient query response time < 100ms': () => duration < 100,
  });

  errorRate.add(!success);
}

function testDashboardQueries() {
  // Test 2: Get analytics dashboard
  const dashboardQuery = `
    query GetDashboard($orgId: String!, $role: String!) {
      dashboard(organizationId: $orgId, role: $role) {
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
        alerts {
          level
          title
          message
        }
      }
    }
  `;

  const roles = ['LAB_DIRECTOR', 'PATHOLOGIST', 'LAB_TECHNICIAN', 'FRONT_DESK'];
  const randomRole = roles[Math.floor(Math.random() * roles.length)];

  const startTime = Date.now();
  const response = executeGraphQL(
    BASE_URLS.analytics,
    dashboardQuery,
    { orgId: TEST_ORG_ID, role: randomRole }
  );

  const duration = Date.now() - startTime;
  dashboardQueryDuration.add(duration);
  requestCounter.add(1);

  const success = check(response, {
    'dashboard query status is 200': (r) => r.status === 200,
    'dashboard has metrics': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.data && body.data.dashboard && body.data.dashboard.metrics;
      } catch (e) {
        return false;
      }
    },
    'dashboard query response time < 1s': () => duration < 1000,
  });

  errorRate.add(!success);
}

function testResultOperations() {
  // Test 3: Query TAT analytics
  const tatQuery = `
    query GetTATAnalytics($orgId: String!, $days: Int!) {
      tatAnalytics(organizationId: $orgId, days: $days) {
        meanTatHours
        medianTatHours
        complianceRate
        totalOrders
      }
    }
  `;

  const startTime = Date.now();
  const response = executeGraphQL(
    BASE_URLS.analytics,
    tatQuery,
    { orgId: TEST_ORG_ID, days: 30 }
  );

  const duration = Date.now() - startTime;
  resultCreationDuration.add(duration);
  requestCounter.add(1);

  const success = check(response, {
    'TAT analytics status is 200': (r) => r.status === 200,
    'TAT analytics has data': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.data && body.data.tatAnalytics;
      } catch (e) {
        return false;
      }
    },
  });

  errorRate.add(!success);
}

function testComplianceOperations() {
  // Test 4: Get compliance dashboard
  const complianceQuery = `
    query GetComplianceDashboard($orgId: String!) {
      complianceDashboard(organizationId: $orgId) {
        openCapas
        overdueCapas
        pendingDocumentReviews
        expiredTrainings
        qualityIndicatorsCritical
      }
    }
  `;

  const response = executeGraphQL(
    BASE_URLS.compliance,
    complianceQuery,
    { orgId: TEST_ORG_ID }
  );

  requestCounter.add(1);

  const success = check(response, {
    'compliance dashboard status is 200': (r) => r.status === 200,
    'compliance dashboard has data': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.data && body.data.complianceDashboard;
      } catch (e) {
        return false;
      }
    },
  });

  errorRate.add(!success);
}

// Health check test
export function setup() {
  console.log('=== Starting Performance Tests ===');
  console.log('Testing services:');
  console.log(`  - Patient Service: ${BASE_URLS.patient}`);
  console.log(`  - Analytics Service: ${BASE_URLS.analytics}`);
  console.log(`  - Result Service: ${BASE_URLS.result}`);
  console.log(`  - Compliance Service: ${BASE_URLS.compliance}`);

  // Health checks
  for (const [name, url] of Object.entries(BASE_URLS)) {
    const response = http.get(`${url}/health`);
    if (response.status !== 200) {
      console.error(`ERROR: ${name} service is not healthy!`);
    } else {
      console.log(`✓ ${name} service is healthy`);
    }
  }
}

export function teardown(data) {
  console.log('=== Performance Tests Complete ===');
}

// Handle test results
export function handleSummary(data) {
  return {
    'performance-test-summary.json': JSON.stringify(data, null, 2),
    stdout: textSummary(data, { indent: ' ', enableColors: true }),
  };
}

function textSummary(data, options) {
  const indent = options.indent || '';
  const enableColors = options.enableColors || false;

  let summary = '\n';
  summary += `${indent}Performance Test Summary\n`;
  summary += `${indent}========================\n\n`;

  // Overall metrics
  summary += `${indent}Total Requests: ${data.metrics.total_requests.values.count}\n`;
  summary += `${indent}Request Duration (p95): ${data.metrics.http_req_duration.values['p(95)']}ms\n`;
  summary += `${indent}Request Duration (p99): ${data.metrics.http_req_duration.values['p(99)']}ms\n`;
  summary += `${indent}Error Rate: ${(data.metrics.errors.values.rate * 100).toFixed(2)}%\n\n`;

  // Service-specific metrics
  summary += `${indent}Patient Query Duration (p95): ${data.metrics.patient_query_duration.values['p(95)']}ms\n`;
  summary += `${indent}Dashboard Query Duration (p95): ${data.metrics.dashboard_query_duration.values['p(95)']}ms\n`;
  summary += `${indent}Result Creation Duration (p95): ${data.metrics.result_creation_duration.values['p(95)']}ms\n\n`;

  // Pass/Fail
  const passed = data.metrics.http_req_failed.values.rate < 0.01;
  if (passed) {
    summary += enableColors ? '\x1b[32m' : '';
    summary += `${indent}✓ PERFORMANCE TESTS PASSED\n`;
    summary += enableColors ? '\x1b[0m' : '';
  } else {
    summary += enableColors ? '\x1b[31m' : '';
    summary += `${indent}✗ PERFORMANCE TESTS FAILED\n`;
    summary += enableColors ? '\x1b[0m' : '';
  }

  return summary;
}
