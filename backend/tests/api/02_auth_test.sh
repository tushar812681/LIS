#!/bin/bash

# LIS Modern Backend - Authentication Test
# Tests user registration, login, and token validation

source "$(dirname "$0")/test_config.sh"

print_header "Authentication & Authorization Test"
echo ""

# Test 1: Health Check
print_info "Testing User Service Health..."
check_health "User Service" "$USER_SERVICE_URL"
echo ""

# Test 2: Register User
print_info "Test: Register New User"
REGISTER_QUERY='
mutation RegisterUser($input: RegisterUserInput!) {
  registerUser(input: $input) {
    id
    email
    firstName
    lastName
    userType
    userStatus
  }
}
'

REGISTER_VARS=$(cat <<EOF
{
  "input": {
    "firstName": "Test",
    "lastName": "Admin",
    "email": "testadmin@lis.com",
    "password": "TestAdmin@123",
    "userType": "SUPER_ADMIN"
  }
}
EOF
)

REGISTER_RESPONSE=$(graphql_query "$USER_SERVICE_URL" "$REGISTER_QUERY" "$REGISTER_VARS")
echo "$REGISTER_RESPONSE" | jq . || echo "$REGISTER_RESPONSE"

if echo "$REGISTER_RESPONSE" | grep -q "email"; then
    print_success "User registration successful"
else
    print_warning "User may already exist or registration failed"
fi
echo ""

# Test 3: Login
print_info "Test: User Login"
LOGIN_QUERY='
mutation Login($input: LoginInput!) {
  login(input: $input) {
    user {
      id
      email
      firstName
      userType
    }
    accessToken
    refreshToken
    expiresIn
  }
}
'

LOGIN_VARS=$(cat <<EOF
{
  "input": {
    "email": "testadmin@lis.com",
    "password": "TestAdmin@123"
  }
}
EOF
)

LOGIN_RESPONSE=$(graphql_query "$USER_SERVICE_URL" "$LOGIN_QUERY" "$LOGIN_VARS")
echo "$LOGIN_RESPONSE" | jq . || echo "$LOGIN_RESPONSE"

# Extract token
AUTH_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.data.login.accessToken // empty')

if [ -n "$AUTH_TOKEN" ] && [ "$AUTH_TOKEN" != "null" ]; then
    print_success "Login successful - Token obtained"
    export AUTH_TOKEN
    # Save token for other tests
    echo "$AUTH_TOKEN" > /tmp/lis_auth_token.txt
else
    print_error "Login failed - No token received"
fi
echo ""

# Test 4: Get Current User (with token)
print_info "Test: Get Current User Profile"
ME_QUERY='
query {
  me {
    id
    email
    firstName
    lastName
    userType
    userStatus
  }
}
'

ME_RESPONSE=$(graphql_query "$USER_SERVICE_URL" "$ME_QUERY" "{}")
echo "$ME_RESPONSE" | jq . || echo "$ME_RESPONSE"

if echo "$ME_RESPONSE" | grep -q "email"; then
    print_success "User profile retrieved successfully"
else
    print_error "Failed to retrieve user profile"
fi
echo ""

# Test 5: List Roles
print_info "Test: List Available Roles"
ROLES_QUERY='
query {
  roles {
    id
    roleCode
    roleName
    description
  }
}
'

ROLES_RESPONSE=$(graphql_query "$USER_SERVICE_URL" "$ROLES_QUERY" "{}")
echo "$ROLES_RESPONSE" | jq . || echo "$ROLES_RESPONSE"

if echo "$ROLES_RESPONSE" | grep -q "roleCode"; then
    print_success "Roles retrieved successfully"
else
    print_warning "No roles found or query failed"
fi
echo ""

# Test 6: List Permissions
print_info "Test: List Available Permissions"
PERMISSIONS_QUERY='
query {
  permissions {
    id
    permissionCode
    permissionName
    module
    action
  }
}
'

PERMISSIONS_RESPONSE=$(graphql_query "$USER_SERVICE_URL" "$PERMISSIONS_QUERY" "{}")
echo "$PERMISSIONS_RESPONSE" | jq . || echo "$PERMISSIONS_RESPONSE"

if echo "$PERMISSIONS_RESPONSE" | grep -q "permissionCode"; then
    print_success "Permissions retrieved successfully"
else
    print_warning "No permissions found or query failed"
fi

print_test_summary
