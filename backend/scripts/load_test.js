// LIS Modern Backend - Comprehensive Load Testing with k6
// Run: k6 run scripts/load_test.js

import http from 'k6/http';
import { check, group, sleep } from 'k6';
import { Rate, Trend, Counter } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');
const responseTime = new Trend('response_time');
const requestCounter = new Counter('requests');

// Test configuration
export const options = {
  stages: [
    { duration: '30s', target: 50 },   // Ramp up to 50 users
    { duration: '1m', target: 100 },   // Ramp up to 100 users
    { duration: '2m', target: 200 },   // Ramp up to 200 users
    { duration: '2m', target: 200 },   // Stay at 200 users
    { duration: '1m', target: 500 },   // Spike to 500 users
    { duration: '1m', target: 500 },   // Stay at 500 users
    { duration: '30s', target: 0 },    // Ramp down to 0
  ],
  thresholds: {
    'http_req_duration': ['p(95)<500', 'p(99)<1000'], // 95% < 500ms, 99% < 1s
    'http_req_failed': ['rate<0.01'],                 // Error rate < 1%
    'errors': ['rate<0.05'],                          // Custom error rate < 5%
  },
};

// Service endpoints
const BASE_URL = __ENV.BASE_URL || 'http://localhost:8001';
const SERVICES = {
  patient: `${BASE_URL}:8001`,
  organization: `${BASE_URL}:8002`,
  sample: `${BASE_URL}:8003`,
  order: `${BASE_URL}:8004`,
  result: `${BASE_URL}:8005`,
  equipment: `${BASE_URL}:8006`,
  inventory: `${BASE_URL}:8007`,
  qc: `${BASE_URL}:8008`,
  billing: `${BASE_URL}:8009`,
  user: `${BASE_URL}:8010`,
  notification: `${BASE_URL}:8011`,
  analytics: `${BASE_URL}:8012`,
  report: `${BASE_URL}:8013`,
  compliance: `${BASE_URL}:8014`,
};

// GraphQL queries
const QUERIES = {
  patients: {
    query: `query {
      patients(organizationId: "00000000-0000-0000-0000-000000000001", page: 1, pageSize: 20) {
        patientId
        firstName
        lastName
        mrn
      }
    }`,
  },
  samples: {
    query: `query {
      samples(organizationId: "00000000-0000-0000-0000-000000000001", page: 1, pageSize: 20) {
        sampleId
        sampleNumber
        sampleType
        sampleStatus
      }
    }`,
  },
  orders: {
    query: `query {
      orders(organizationId: "00000000-0000-0000-0000-000000000001", page: 1, pageSize: 20) {
        orderId
        orderNumber
        orderStatus
      }
    }`,
  },
};

// Helper function to make GraphQL request
function graphqlRequest(url, query) {
  const payload = JSON.stringify(query);
  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
    timeout: '30s',
  };

  return http.post(`${url}/graphql`, payload, params);
}

// Test scenarios
export default function () {
  requestCounter.add(1);

  // Test 1: Health checks for all services
  group('Health Checks', () => {
    Object.entries(SERVICES).forEach(([name, url]) => {
      const res = http.get(`${url}/health`, { timeout: '5s' });

      const success = check(res, {
        [`${name} health: status 200`]: (r) => r.status === 200,
        [`${name} health: response time <100ms`]: (r) => r.timings.duration < 100,
        [`${name} health: has status field`]: (r) => {
          try {
            return JSON.parse(r.body).status === 'healthy';
          } catch (e) {
            return false;
          }
        },
      });

      responseTime.add(res.timings.duration);
      errorRate.add(!success);
    });
  });

  sleep(1);

  // Test 2: GraphQL queries
  group('GraphQL Queries', () => {
    // Patient queries
    const patientRes = graphqlRequest(SERVICES.patient, QUERIES.patients);
    check(patientRes, {
      'patients query: status 200': (r) => r.status === 200,
      'patients query: has data': (r) => {
        try {
          const body = JSON.parse(r.body);
          return body.data !== undefined;
        } catch (e) {
          return false;
        }
      },
      'patients query: response time <500ms': (r) => r.timings.duration < 500,
    });

    responseTime.add(patientRes.timings.duration);
    errorRate.add(patientRes.status !== 200);

    // Sample queries
    const sampleRes = graphqlRequest(SERVICES.sample, QUERIES.samples);
    check(sampleRes, {
      'samples query: status 200': (r) => r.status === 200,
      'samples query: response time <500ms': (r) => r.timings.duration < 500,
    });

    responseTime.add(sampleRes.timings.duration);
    errorRate.add(sampleRes.status !== 200);

    // Order queries
    const orderRes = graphqlRequest(SERVICES.order, QUERIES.orders);
    check(orderRes, {
      'orders query: status 200': (r) => r.status === 200,
      'orders query: response time <500ms': (r) => r.timings.duration < 500,
    });

    responseTime.add(orderRes.timings.duration);
    errorRate.add(orderRes.status !== 200);
  });

  sleep(2);

  // Test 3: Concurrent requests simulation
  group('Concurrent Operations', () => {
    const requests = [
      ['GET', `${SERVICES.patient}/health`],
      ['GET', `${SERVICES.sample}/health`],
      ['GET', `${SERVICES.order}/health`],
      ['GET', `${SERVICES.result}/health`],
    ];

    const responses = http.batch(requests);
    responses.forEach((res, i) => {
      check(res, {
        [`concurrent request ${i}: status 200`]: (r) => r.status === 200,
      });
      responseTime.add(res.timings.duration);
    });
  });

  sleep(1);
}

// Teardown function
export function teardown(data) {
  console.log('Load test completed');
  console.log(`Total requests: ${requestCounter.count}`);
}
