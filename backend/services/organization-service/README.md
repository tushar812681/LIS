# Organization Service

Multi-tenancy and organization management service for the Laboratory Information System (LIS). Manages organizations, branches, departments, accreditations, and subscription plans.

## Features

### Core Capabilities
- **Organization Management**: Create, update, and manage laboratory organizations
- **Multi-tenant Architecture**: Complete isolation between organizations
- **Branch Management**: Support for multi-branch organizations
- **Department Management**: Organize lab operations by department (Biochemistry, Hematology, etc.)
- **Accreditation Tracking**: Manage NABL, CAP, ISO certifications
- **Subscription Management**: Flexible subscription plans with usage limits
- **Organization Settings**: Key-value settings storage per organization
- **Audit Logging**: Complete audit trail for all changes

### Subscription Plans
- **Free**: Limited features for trial users
- **Basic**: Small single-lab operations
- **Professional**: Multi-branch diagnostics centers
- **Enterprise**: Large hospital labs with advanced features
- **Custom**: Tailored plans for specific requirements

### Organization Types
- Single Lab
- Multi-branch Lab
- Hospital Lab
- Diagnostic Center
- Reference Lab
- Collection Center

### Accreditation Types
- NABL (National Accreditation Board for Testing and Calibration Laboratories)
- CAP (College of American Pathologists)
- ISO 15189 (Medical Laboratories Quality Management)
- ISO 9001 (Quality Management Systems)
- JCI (Joint Commission International)
- NABH (National Accreditation Board for Hospitals)

## Architecture

### Clean Architecture Layers
1. **Domain** (`domain.rs`): Business entities and value objects
2. **Repository** (`repository.rs`): Database access layer
3. **Service** (`service.rs`): Business logic and validation
4. **API** (`api.rs`): GraphQL API layer

## Database Schema

### Tables
- `organization`: Main organization entity
- `organization_branch`: Branch/location management
- `accreditation`: Certification tracking
- `organization_setting`: Flexible settings storage
- `department`: Department management
- `working_hours_template`: Business hours configuration
- `organization_audit_log`: Audit trail

### Custom Functions
- `generate_org_code()`: Auto-generate organization codes with Luhn checksum
- `reset_monthly_test_counter()`: Reset usage counters monthly
- `check_subscription_validity()`: Auto-expire expired subscriptions
- `log_organization_changes()`: Audit trigger for tracking changes

## GraphQL API

### Queries

#### Organizations
```graphql
# Get organization by ID
organization(id: UUID!): Organization

# Get organization by code
organizationByCode(orgCode: String!): Organization

# List organizations with filtering
organizations(
  filter: OrganizationFilter
  page: Int
  pageSize: Int
): OrganizationPaginated
```

#### Branches
```graphql
# Get branch by ID
branch(id: UUID!): OrganizationBranch

# List branches for organization
branches(
  organizationId: UUID!
  filter: BranchFilter
): [OrganizationBranch!]!
```

#### Accreditations
```graphql
# Get accreditation by ID
accreditation(id: UUID!): Accreditation

# List accreditations for organization
accreditations(organizationId: UUID!): [Accreditation!]!
```

#### Departments
```graphql
# Get department by ID
department(id: UUID!): Department

# List departments for organization
departments(
  organizationId: UUID!
  filter: DepartmentFilter
): [Department!]!
```

#### Settings
```graphql
# Get specific setting
organizationSetting(
  organizationId: UUID!
  category: String!
  key: String!
): OrganizationSetting

# List settings for organization
organizationSettings(
  organizationId: UUID!
  category: String
): [OrganizationSetting!]!
```

### Mutations

#### Organization Operations
```graphql
# Create organization
createOrganization(input: CreateOrganizationInput!): Organization!

# Update organization
updateOrganization(input: UpdateOrganizationInput!): Organization!

# Update organization status
updateOrganizationStatus(input: UpdateOrganizationStatusInput!): Organization!

# Update subscription
updateSubscription(input: UpdateSubscriptionInput!): Organization!

# Delete organization (soft delete)
deleteOrganization(id: UUID!): Boolean!
```

#### Branch Operations
```graphql
# Create branch
createBranch(input: CreateBranchInput!): OrganizationBranch!

# Update branch
updateBranch(input: UpdateBranchInput!): OrganizationBranch!

# Deactivate branch
deactivateBranch(id: UUID!): Boolean!
```

#### Accreditation Operations
```graphql
# Add accreditation
addAccreditation(input: AddAccreditationInput!): Accreditation!

# Update accreditation
updateAccreditation(input: UpdateAccreditationInput!): Accreditation!

# Deactivate accreditation
deactivateAccreditation(id: UUID!): Boolean!
```

#### Department Operations
```graphql
# Create department
createDepartment(input: CreateDepartmentInput!): Department!

# Update department
updateDepartment(input: UpdateDepartmentInput!): Department!

# Deactivate department
deactivateDepartment(id: UUID!): Boolean!
```

#### Settings Operations
```graphql
# Update setting (upsert)
updateOrganizationSetting(input: UpdateOrganizationSettingInput!): OrganizationSetting!

# Delete setting
deleteOrganizationSetting(
  organizationId: UUID!
  category: String!
  key: String!
): Boolean!
```

#### Usage Tracking
```graphql
# Increment test counter (called by other services)
incrementTestCounter(organizationId: UUID!): Boolean!
```

## Business Logic

### Organization Creation
- Auto-generates organization code with Luhn checksum (e.g., ORG-0000011)
- Starts with 30-day trial by default
- Validates email uniqueness
- Sets default subscription limits

### Subscription Management
- Enforces user limits per plan
- Enforces branch limits per plan
- Tracks monthly test usage
- Auto-expires subscriptions based on end date

### Branch Management
- Validates branch limits based on subscription
- Prevents deletion of main branch
- Tracks branch-level capacity

### Department Management
- Supports organization-level and branch-level departments
- Validates branch belongs to organization
- Standard departments: Biochemistry, Hematology, Microbiology, Pathology

### Accreditation Tracking
- Validates date ranges
- Tracks expiry dates
- Supports multiple concurrent accreditations
- 90-day renewal reminders

## Setup

### Prerequisites
- Rust 1.75+
- PostgreSQL 16+

### Installation
```bash
# Navigate to service directory
cd backend/services/organization-service

# Copy environment variables
cp .env.example .env

# Edit .env with your database credentials
vim .env

# Run migrations (done automatically on startup)
# sqlx migrate run

# Build the service
cargo build --release

# Run the service
cargo run
```

### Database Setup
```bash
# Create database
createdb lis_organization

# Migrations run automatically on service startup
```

## Development

### Running Locally
```bash
# Run with hot reload
cargo watch -x run

# Run tests
cargo test

# Check code
cargo clippy
```

### GraphiQL Playground
Access the interactive GraphQL playground at:
```
http://localhost:8086/graphql
```

### Health Checks
```bash
# Health check
curl http://localhost:8086/health

# Readiness check (includes database connectivity)
curl http://localhost:8086/ready
```

## Example Usage

### Create Organization
```graphql
mutation {
  createOrganization(input: {
    organizationName: "Apollo Diagnostics"
    legalName: "Apollo Diagnostics Private Limited"
    organizationType: DIAGNOSTIC_CENTER
    email: "info@apollodiagnostics.com"
    phone: "+91-9876543210"
    city: "Mumbai"
    state: "Maharashtra"
    country: "India"
    subscriptionPlan: PROFESSIONAL
  }) {
    id
    orgCode
    organizationName
    organizationStatus
    subscriptionPlan
  }
}
```

### Create Branch
```graphql
mutation {
  createBranch(input: {
    organizationId: "550e8400-e29b-41d4-a716-446655440000"
    branchCode: "MUM-01"
    branchName: "Mumbai Main Branch"
    isMainBranch: true
    city: "Mumbai"
    sampleProcessingCapacity: 500
  }) {
    id
    branchCode
    branchName
    isMainBranch
  }
}
```

### Add Accreditation
```graphql
mutation {
  addAccreditation(input: {
    organizationId: "550e8400-e29b-41d4-a716-446655440000"
    accreditationType: NABL
    accreditationNumber: "TC-1234"
    issuingAuthority: "NABL India"
    issueDate: "2024-01-01"
    expiryDate: "2026-12-31"
    scopeOfAccreditation: "Clinical Biochemistry, Hematology"
  }) {
    id
    accreditationType
    accreditationNumber
    expiryDate
    isActive
  }
}
```

## Configuration

### Environment Variables
- `DATABASE_URL`: PostgreSQL connection string
- `DATABASE_MAX_CONNECTIONS`: Max database connections (default: 32)
- `HOST`: Server host (default: 0.0.0.0)
- `PORT`: Server port (default: 8086)
- `ENABLE_CACHING`: Enable Redis caching (default: false)
- `ENABLE_EVENTS`: Enable Kafka events (default: false)

## Integration

### With Other Services
- **User Service**: Validates user limits per organization
- **Order Service**: Checks subscription validity before order creation
- **Result Service**: Validates test limits per month
- **Billing Service**: Subscription billing and invoicing

### Events Published
- `organization.created`
- `organization.updated`
- `organization.status_changed`
- `subscription.updated`
- `subscription.expired`
- `branch.created`
- `accreditation.added`
- `accreditation.expiring` (90 days before expiry)

## Performance

### Database Indexes
- Organization code (unique, B-tree)
- Organization email (unique)
- Organization status + subscription plan (composite)
- Full-text search on organization name
- Branch organization_id (foreign key)
- Department organization_id (foreign key)

### Query Optimization
- Paginated organization listing
- Filtered branch and department queries
- Soft deletes for audit trail

## Security

### Data Isolation
- All queries scoped by organization_id
- Soft deletes prevent data loss
- Audit logging for compliance

### Validation
- Email format validation
- Subscription limit enforcement
- Date range validation
- Business rule validation

## Monitoring

### Metrics
- Active organizations count
- Organizations by status
- Organizations by subscription plan
- Expiring subscriptions (30 days)
- Monthly test usage by organization

### Logging
- Organization creation/updates
- Subscription changes
- Branch operations
- Accreditation management

## License

Copyright © 2025 LIS Modern. All rights reserved.
