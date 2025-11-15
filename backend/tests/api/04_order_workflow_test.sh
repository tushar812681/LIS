#!/bin/bash

# LIS Modern Backend - Order Workflow Test
# Tests complete order creation and management workflow

source "$(dirname "$0")/test_config.sh"

# Load auth token and test data
if [ -f /tmp/lis_auth_token.txt ]; then
    AUTH_TOKEN=$(cat /tmp/lis_auth_token.txt)
    export AUTH_TOKEN
fi

print_header "Order Workflow Test - Complete Order Lifecycle"
echo ""

# Assume we have patient and org from previous tests
export TEST_PATIENT_ID="${TEST_PATIENT_ID:-00000000-0000-0000-0000-000000000001}"
export TEST_ORG_ID="${TEST_ORG_ID:-00000000-0000-0000-0000-000000000001}"

# Test 1: Create Test in Catalog
print_info "Test: Create Test in Catalog"
CREATE_TEST_QUERY='
mutation CreateTest($input: CreateTestInput!) {
  createTest(input: $input) {
    id
    testCode
    testName
    basePrice
  }
}
'

CREATE_TEST_VARS=$(cat <<EOF
{
  "input": {
    "testCode": "CBC001",
    "testName": "Complete Blood Count",
    "shortName": "CBC",
    "specimenType": "BLOOD",
    "resultType": "NUMERIC",
    "basePrice": 500.00,
    "standardTatHours": 24
  }
}
EOF
)

TEST_RESPONSE=$(graphql_query "$ORDER_SERVICE_URL" "$CREATE_TEST_QUERY" "$CREATE_TEST_VARS")
echo "$TEST_RESPONSE" | jq . || echo "$TEST_RESPONSE"

TEST_ID=$(echo "$TEST_RESPONSE" | jq -r '.data.createTest.id // empty')

if [ -n "$TEST_ID" ] && [ "$TEST_ID" != "null" ]; then
    print_success "Test catalog entry created: $TEST_ID"
    export TEST_TEST_ID="$TEST_ID"
else
    print_warning "Test may already exist"
fi
echo ""

# Test 2: Create Order
print_info "Test: Create Test Order"
CREATE_ORDER_QUERY='
mutation CreateOrder($input: CreateOrderInput!, $organizationId: String!, $createdBy: String!) {
  createOrder(input: $input, organizationId: $organizationId, createdBy: $createdBy) {
    id
    orderNumber
    orderStatus
    priority
    totalAmount
  }
}
'

CREATE_ORDER_VARS=$(cat <<EOF
{
  "input": {
    "patientId": "$TEST_PATIENT_ID",
    "orderSource": "WALK_IN",
    "priority": "ROUTINE",
    "referringDoctorName": "Dr. Sharma",
    "homeCollectionRequested": false
  },
  "organizationId": "$TEST_ORG_ID",
  "createdBy": "00000000-0000-0000-0000-000000000001"
}
EOF
)

ORDER_RESPONSE=$(graphql_query "$ORDER_SERVICE_URL" "$CREATE_ORDER_QUERY" "$CREATE_ORDER_VARS")
echo "$ORDER_RESPONSE" | jq . || echo "$ORDER_RESPONSE"

ORDER_ID=$(echo "$ORDER_RESPONSE" | jq -r '.data.createOrder.id // empty')
ORDER_NUMBER=$(echo "$ORDER_RESPONSE" | jq -r '.data.createOrder.orderNumber // empty')

if [ -n "$ORDER_ID" ] && [ "$ORDER_ID" != "null" ]; then
    print_success "Order created: $ORDER_ID (Order #: $ORDER_NUMBER)"
    export TEST_ORDER_ID="$ORDER_ID"
else
    print_error "Failed to create order"
fi
echo ""

# Test 3: Add Test to Order
if [ -n "$TEST_ID" ] && [ "$TEST_ID" != "null" ]; then
    print_info "Test: Add Test to Order"
    ADD_TEST_QUERY='
    mutation AddTestToOrder($input: AddTestToOrderInput!, $createdBy: String!) {
      addTestToOrder(input: $input, createdBy: $createdBy) {
        id
        testName
        unitPrice
      }
    }
    '

    ADD_TEST_VARS=$(cat <<EOF
{
  "input": {
    "orderId": "$ORDER_ID",
    "testId": "$TEST_ID",
    "quantity": 1
  },
  "createdBy": "00000000-0000-0000-0000-000000000001"
}
EOF
)

    ADD_TEST_RESPONSE=$(graphql_query "$ORDER_SERVICE_URL" "$ADD_TEST_QUERY" "$ADD_TEST_VARS")
    echo "$ADD_TEST_RESPONSE" | jq . || echo "$ADD_TEST_RESPONSE"

    if echo "$ADD_TEST_RESPONSE" | grep -q "testName"; then
        print_success "Test added to order successfully"
    else
        print_error "Failed to add test to order"
    fi
    echo ""
fi

# Test 4: Get Order Details
print_info "Test: Get Order Details"
GET_ORDER_QUERY='
query GetOrder($id: String!) {
  order(id: $id) {
    id
    orderNumber
    orderStatus
    priority
    totalAmount
  }
}
'

GET_ORDER_VARS=$(cat <<EOF
{
  "id": "$ORDER_ID"
}
EOF
)

GET_ORDER_RESPONSE=$(graphql_query "$ORDER_SERVICE_URL" "$GET_ORDER_QUERY" "$GET_ORDER_VARS")
echo "$GET_ORDER_RESPONSE" | jq . || echo "$GET_ORDER_RESPONSE"

if echo "$GET_ORDER_RESPONSE" | grep -q "orderNumber"; then
    print_success "Order details retrieved successfully"
else
    print_error "Failed to retrieve order details"
fi
echo ""

# Test 5: Confirm Order
print_info "Test: Confirm Order"
CONFIRM_ORDER_QUERY='
mutation ConfirmOrder($input: ConfirmOrderInput!, $confirmedBy: String!) {
  confirmOrder(input: $input, confirmedBy: $confirmedBy) {
    id
    orderStatus
  }
}
'

CONFIRM_ORDER_VARS=$(cat <<EOF
{
  "input": {
    "orderId": "$ORDER_ID"
  },
  "confirmedBy": "00000000-0000-0000-0000-000000000001"
}
EOF
)

CONFIRM_RESPONSE=$(graphql_query "$ORDER_SERVICE_URL" "$CONFIRM_ORDER_QUERY" "$CONFIRM_ORDER_VARS")
echo "$CONFIRM_RESPONSE" | jq . || echo "$CONFIRM_RESPONSE"

if echo "$CONFIRM_RESPONSE" | grep -q "orderStatus"; then
    print_success "Order confirmed successfully"
else
    print_error "Failed to confirm order"
fi
echo ""

# Test 6: Search Orders
print_info "Test: Search Orders"
SEARCH_ORDERS_QUERY='
query SearchOrders($filter: OrderFilter!, $limit: Int) {
  searchOrders(filter: $filter, limit: $limit) {
    id
    orderNumber
    orderStatus
  }
}
'

SEARCH_ORDERS_VARS=$(cat <<EOF
{
  "filter": {
    "patientId": "$TEST_PATIENT_ID"
  },
  "limit": 10
}
EOF
)

SEARCH_ORDERS_RESPONSE=$(graphql_query "$ORDER_SERVICE_URL" "$SEARCH_ORDERS_QUERY" "$SEARCH_ORDERS_VARS")
echo "$SEARCH_ORDERS_RESPONSE" | jq . || echo "$SEARCH_ORDERS_RESPONSE"

if echo "$SEARCH_ORDERS_RESPONSE" | grep -q "orderNumber"; then
    print_success "Orders search successful"
else
    print_warning "No orders found in search"
fi

print_test_summary
