#!/bin/bash

# LIS Modern Backend - Patient Service Test
# Tests patient CRUD operations

source "$(dirname "$0")/test_config.sh"

# Load auth token from previous test
if [ -f /tmp/lis_auth_token.txt ]; then
    AUTH_TOKEN=$(cat /tmp/lis_auth_token.txt)
    export AUTH_TOKEN
fi

print_header "Patient Service Test - CRUD Operations"
echo ""

# Test 1: Create Organization First
print_info "Test: Create Organization"
ORG_QUERY='
mutation CreateOrganization($input: CreateOrganizationInput!) {
  createOrganization(input: $input) {
    id
    organizationName
    organizationType
    subscriptionPlan
  }
}
'

ORG_VARS=$(cat <<EOF
{
  "input": {
    "organizationName": "Test Lab Hospital",
    "organizationType": "HOSPITAL",
    "subscriptionPlan": "ENTERPRISE",
    "contactEmail": "contact@testlab.com",
    "contactPhone": "9876543210",
    "address": "Test Address, Mumbai, Maharashtra",
    "city": "Mumbai",
    "state": "Maharashtra",
    "pincode": "400001",
    "country": "India"
  }
}
EOF
)

ORG_RESPONSE=$(graphql_query "$ORGANIZATION_SERVICE_URL" "$ORG_QUERY" "$ORG_VARS")
echo "$ORG_RESPONSE" | jq . || echo "$ORG_RESPONSE"

ORG_ID=$(echo "$ORG_RESPONSE" | jq -r '.data.createOrganization.id // empty')

if [ -n "$ORG_ID" ] && [ "$ORG_ID" != "null" ]; then
    print_success "Organization created: $ORG_ID"
    export TEST_ORG_ID="$ORG_ID"
else
    print_warning "Organization may already exist or creation failed"
    export TEST_ORG_ID="$TEST_ORG_ID"
fi
echo ""

# Test 2: Create Patient
print_info "Test: Create Patient"
CREATE_PATIENT_QUERY='
mutation CreatePatient($input: CreatePatientInput!, $organizationId: String!, $createdBy: String!) {
  createPatient(input: $input, organizationId: $organizationId, createdBy: $createdBy) {
    id
    mrnNumber
    firstName
    lastName
    fullName
    dateOfBirth
    age
    gender
    mobileNumber
    email
  }
}
'

CREATE_PATIENT_VARS=$(cat <<EOF
{
  "input": {
    "firstName": "Rajesh",
    "lastName": "Kumar",
    "dateOfBirth": "1990-05-15",
    "gender": "MALE",
    "mobileNumber": "9876543210",
    "email": "rajesh.kumar@example.com",
    "nationality": "Indian"
  },
  "organizationId": "$TEST_ORG_ID",
  "createdBy": "00000000-0000-0000-0000-000000000001"
}
EOF
)

CREATE_RESPONSE=$(graphql_query "$PATIENT_SERVICE_URL" "$CREATE_PATIENT_QUERY" "$CREATE_PATIENT_VARS")
echo "$CREATE_RESPONSE" | jq . || echo "$CREATE_RESPONSE"

PATIENT_ID=$(echo "$CREATE_RESPONSE" | jq -r '.data.createPatient.id // empty')
MRN_NUMBER=$(echo "$CREATE_RESPONSE" | jq -r '.data.createPatient.mrnNumber // empty')

if [ -n "$PATIENT_ID" ] && [ "$PATIENT_ID" != "null" ]; then
    print_success "Patient created: $PATIENT_ID (MRN: $MRN_NUMBER)"
    export TEST_PATIENT_ID="$PATIENT_ID"
    export TEST_MRN_NUMBER="$MRN_NUMBER"
else
    print_error "Failed to create patient"
fi
echo ""

# Test 3: Get Patient by ID
print_info "Test: Get Patient by ID"
GET_PATIENT_QUERY='
query GetPatient($id: String!) {
  patient(id: $id) {
    id
    mrnNumber
    fullName
    age
    gender
    mobileNumber
  }
}
'

GET_PATIENT_VARS=$(cat <<EOF
{
  "id": "$PATIENT_ID"
}
EOF
)

GET_RESPONSE=$(graphql_query "$PATIENT_SERVICE_URL" "$GET_PATIENT_QUERY" "$GET_PATIENT_VARS")
echo "$GET_RESPONSE" | jq . || echo "$GET_RESPONSE"

if echo "$GET_RESPONSE" | grep -q "mrnNumber"; then
    print_success "Patient retrieved by ID successfully"
else
    print_error "Failed to retrieve patient by ID"
fi
echo ""

# Test 4: Get Patient by MRN
print_info "Test: Get Patient by MRN"
GET_BY_MRN_QUERY='
query GetPatientByMRN($mrnNumber: String!) {
  patientByMrn(mrnNumber: $mrnNumber) {
    id
    mrnNumber
    fullName
  }
}
'

GET_BY_MRN_VARS=$(cat <<EOF
{
  "mrnNumber": "$MRN_NUMBER"
}
EOF
)

MRN_RESPONSE=$(graphql_query "$PATIENT_SERVICE_URL" "$GET_BY_MRN_QUERY" "$GET_BY_MRN_VARS")
echo "$MRN_RESPONSE" | jq . || echo "$MRN_RESPONSE"

if echo "$MRN_RESPONSE" | grep -q "$MRN_NUMBER"; then
    print_success "Patient retrieved by MRN successfully"
else
    print_error "Failed to retrieve patient by MRN"
fi
echo ""

# Test 5: Search Patients
print_info "Test: Search Patients"
SEARCH_QUERY='
query SearchPatients($query: String!, $organizationId: String!, $limit: Int) {
  searchPatients(query: $query, organizationId: $organizationId, limit: $limit) {
    id
    mrnNumber
    fullName
    mobileNumber
  }
}
'

SEARCH_VARS=$(cat <<EOF
{
  "query": "Rajesh",
  "organizationId": "$TEST_ORG_ID",
  "limit": 10
}
EOF
)

SEARCH_RESPONSE=$(graphql_query "$PATIENT_SERVICE_URL" "$SEARCH_QUERY" "$SEARCH_VARS")
echo "$SEARCH_RESPONSE" | jq . || echo "$SEARCH_RESPONSE"

if echo "$SEARCH_RESPONSE" | grep -q "Rajesh"; then
    print_success "Patient search successful"
else
    print_warning "Search returned no results"
fi
echo ""

# Test 6: Update Patient
print_info "Test: Update Patient"
UPDATE_PATIENT_QUERY='
mutation UpdatePatient($id: String!, $input: UpdatePatientInput!, $updatedBy: String!) {
  updatePatient(id: $id, input: $input, updatedBy: $updatedBy) {
    id
    email
  }
}
'

UPDATE_PATIENT_VARS=$(cat <<EOF
{
  "id": "$PATIENT_ID",
  "input": {
    "email": "rajesh.updated@example.com"
  },
  "updatedBy": "00000000-0000-0000-0000-000000000001"
}
EOF
)

UPDATE_RESPONSE=$(graphql_query "$PATIENT_SERVICE_URL" "$UPDATE_PATIENT_QUERY" "$UPDATE_PATIENT_VARS")
echo "$UPDATE_RESPONSE" | jq . || echo "$UPDATE_RESPONSE"

if echo "$UPDATE_RESPONSE" | grep -q "rajesh.updated@example.com"; then
    print_success "Patient updated successfully"
else
    print_error "Failed to update patient"
fi
echo ""

# Test 7: List Patients (Pagination)
print_info "Test: List Patients with Pagination"
LIST_QUERY='
query ListPatients($organizationId: String!, $page: Int, $pageSize: Int) {
  patients(organizationId: $organizationId, page: $page, pageSize: $pageSize) {
    id
    mrnNumber
    fullName
  }
}
'

LIST_VARS=$(cat <<EOF
{
  "organizationId": "$TEST_ORG_ID",
  "page": 1,
  "pageSize": 20
}
EOF
)

LIST_RESPONSE=$(graphql_query "$PATIENT_SERVICE_URL" "$LIST_QUERY" "$LIST_VARS")
echo "$LIST_RESPONSE" | jq . || echo "$LIST_RESPONSE"

if echo "$LIST_RESPONSE" | grep -q "mrnNumber"; then
    print_success "Patient list retrieved successfully"
else
    print_warning "No patients found in list"
fi

print_test_summary
