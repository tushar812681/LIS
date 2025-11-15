#!/bin/bash

# Test API Gateway Integration

echo "===== API Gateway Integration Tests ====="
echo ""

# Test 1: Health Check
echo "Test 1: API Gateway Health Check"
curl -s http://localhost:8000/health | jq '.'
echo ""

# Test 2: GraphQL Health Query
echo "Test 2: GraphQL Health Query"
curl -s -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ health }"}' | jq '.'
echo ""

# Test 3: Search Patients (Empty Result Expected)
echo "Test 3: Search Patients Query"
curl -s -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query SearchPatients($query: String!, $orgId: String, $limit: Int) { searchPatients(query: $query, organizationId: $orgId, limit: $limit) }",
    "variables": {
      "query": "test",
      "orgId": "d1e1c1a0-1234-5678-9abc-123456789012",
      "limit": 5
    }
  }' | jq '.'
echo ""

echo "===== Tests Complete ====="
