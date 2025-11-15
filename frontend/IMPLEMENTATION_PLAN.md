# LIS Modern Frontend - Comprehensive Implementation Plan

**Date**: November 6, 2025
**Current Status**: 35% Complete (Structure Only)
**Target**: 100% Production-Ready Implementation

---

## 📋 Complete Feature Inventory

### Current State Analysis

#### ✅ What's Done (Structure Layer)
- [x] 26 pages created with routing
- [x] Basic UI layouts and navigation
- [x] Authentication pages (login, register, password reset)
- [x] Dashboard layouts (4 role-specific)
- [x] Sidebar navigation with RBAC
- [x] Basic form structures
- [x] WebSocket client setup
- [x] Apollo Client configuration
- [x] State management (Zustand)
- [x] Toast notification system
- [x] Dark mode support

#### ❌ What's Missing (Implementation Layer)
- [ ] GraphQL queries for all entities (0%)
- [ ] GraphQL mutations for all operations (15%)
- [ ] Detail pages for all entities (0%)
- [ ] CRUD modal dialogs (0%)
- [ ] Data tables with advanced features (0%)
- [ ] Form submission handlers (20%)
- [ ] File upload functionality (0%)
- [ ] Search and filter implementation (10%)
- [ ] Real data integration (10%)
- [ ] Workflow implementations (5%)

---

## 🎯 Implementation Strategy

### Phase 1: Foundation (Critical Infrastructure)
**Goal**: Build reusable components and GraphQL layer
**Duration**: Estimated focus time
**Priority**: HIGHEST

### Phase 2: Core Workflows (End-to-End Functionality)
**Goal**: Implement primary laboratory workflows
**Priority**: HIGH

### Phase 3: Advanced Features (Enhanced Functionality)
**Goal**: Add advanced UI features and optimizations
**Priority**: MEDIUM

### Phase 4: Integration & Polish (Production Ready)
**Goal**: Integrate external services and polish UI
**Priority**: MEDIUM

---

## 📦 Phase 1: Foundation Components (Build First)

### 1.1 Reusable UI Components
**Why First**: These will be used across all pages

- [ ] **DataTable Component** (Universal)
  - [ ] Table with TanStack React Table
  - [ ] Column sorting
  - [ ] Column filtering
  - [ ] Pagination
  - [ ] Row selection (single/multi)
  - [ ] Bulk actions toolbar
  - [ ] Column visibility toggle
  - [ ] Export to CSV/Excel
  - [ ] Loading states
  - [ ] Empty states
  - [ ] Error states
  - **Files**: `components/ui/data-table.tsx`, `components/ui/data-table-toolbar.tsx`

- [ ] **Modal Dialog Component** (Universal)
  - [ ] Base modal wrapper
  - [ ] Form modal template
  - [ ] Confirmation modal
  - [ ] Full-screen modal option
  - [ ] Responsive behavior
  - **Files**: `components/ui/modal.tsx`, `components/ui/form-modal.tsx`

- [ ] **SearchBar Component** (Universal)
  - [ ] Debounced search input
  - [ ] Search suggestions
  - [ ] Advanced filter builder
  - [ ] Saved filters
  - **Files**: `components/ui/search-bar.tsx`, `components/ui/filter-builder.tsx`

- [ ] **FileUpload Component** (Universal)
  - [ ] Drag and drop
  - [ ] Image preview
  - [ ] File validation
  - [ ] Progress indicator
  - [ ] Multiple file support
  - **Files**: `components/ui/file-upload.tsx`

- [ ] **StatusBadge Component** (Universal)
  - [ ] Color-coded status badges
  - [ ] Icon support
  - [ ] Tooltip on hover
  - **Files**: `components/ui/status-badge.tsx`

- [ ] **Skeleton Loaders** (Universal)
  - [ ] Table skeleton
  - [ ] Card skeleton
  - [ ] Form skeleton
  - [ ] List skeleton
  - **Files**: `components/ui/skeleton.tsx`

### 1.2 GraphQL Layer Setup
**Why First**: Required for all data operations

- [ ] **GraphQL Schema Definitions**
  - [ ] Patient schema types
  - [ ] Sample schema types
  - [ ] Order schema types
  - [ ] Result schema types
  - [ ] Report schema types
  - [ ] User schema types
  - [ ] Organization schema types
  - **Files**: `lib/graphql/types/*.ts`

- [ ] **GraphQL Queries** (All Entities)
  - [ ] GET_PATIENTS (list with filters)
  - [ ] GET_PATIENT_BY_ID (detail)
  - [ ] GET_SAMPLES (list with filters)
  - [ ] GET_SAMPLE_BY_ID (detail)
  - [ ] GET_ORDERS (list with filters)
  - [ ] GET_ORDER_BY_ID (detail)
  - [ ] GET_RESULTS (list with filters)
  - [ ] GET_RESULT_BY_ID (detail)
  - [ ] GET_REPORTS (list with filters)
  - [ ] GET_REPORT_BY_ID (detail)
  - [ ] GET_TESTS (catalog)
  - [ ] GET_QC_RUNS
  - [ ] GET_EQUIPMENT
  - [ ] GET_INVENTORY_ITEMS
  - [ ] GET_INVOICES
  - [ ] GET_USERS
  - **Files**: `lib/graphql/queries/*.ts`

- [ ] **GraphQL Mutations** (All Operations)
  - [ ] CREATE_PATIENT
  - [ ] UPDATE_PATIENT
  - [ ] DELETE_PATIENT
  - [ ] CREATE_ORDER
  - [ ] UPDATE_ORDER
  - [ ] CANCEL_ORDER
  - [ ] CREATE_SAMPLE
  - [ ] UPDATE_SAMPLE_STATUS
  - [ ] REJECT_SAMPLE
  - [ ] CREATE_RESULT
  - [ ] UPDATE_RESULT
  - [ ] VERIFY_RESULT
  - [ ] APPROVE_RESULT
  - [ ] GENERATE_REPORT
  - [ ] APPROVE_REPORT
  - [ ] CREATE_QC_RUN
  - [ ] CREATE_EQUIPMENT
  - [ ] UPDATE_EQUIPMENT
  - [ ] CREATE_INVENTORY_ITEM
  - [ ] ADJUST_STOCK
  - [ ] CREATE_INVOICE
  - [ ] RECORD_PAYMENT
  - [ ] CREATE_USER
  - [ ] UPDATE_USER
  - [ ] UPDATE_ORGANIZATION
  - **Files**: `lib/graphql/mutations/*.ts`

- [ ] **Custom Hooks** (Data Fetching)
  - [ ] usePatients()
  - [ ] usePatient(id)
  - [ ] useSamples()
  - [ ] useSample(id)
  - [ ] useOrders()
  - [ ] useOrder(id)
  - [ ] useResults()
  - [ ] useResult(id)
  - [ ] useReports()
  - [ ] useReport(id)
  - **Files**: `lib/hooks/*.ts`

---

## 🔄 Phase 2: Core Workflows (Implement Second)

### 2.1 Patient Management Module (Complete End-to-End)

#### A. Patient List Page Enhancement
- [x] Basic UI exists
- [ ] **Integrate GraphQL Query**
  - [ ] Replace mock data with usePatients() hook
  - [ ] Add loading state
  - [ ] Add error handling
  - [ ] Add pagination
  - [ ] Add real-time updates via WebSocket
  - **File**: `app/dashboard/patients/page.tsx`

- [ ] **Replace with DataTable Component**
  - [ ] Move to advanced table layout
  - [ ] Add sortable columns
  - [ ] Add column filters
  - [ ] Add bulk actions (export, delete)
  - [ ] Add column visibility
  - **File**: `app/dashboard/patients/page.tsx`

- [ ] **Advanced Search & Filters**
  - [ ] Search by MR, name, phone, email
  - [ ] Filter by gender, age range, blood group
  - [ ] Filter by registration date
  - [ ] Saved filters
  - **File**: `app/dashboard/patients/page.tsx`

#### B. Patient Detail Page (NEW - Critical)
- [ ] **Create Detail Page**
  - [ ] Route: `/dashboard/patients/[id]/page.tsx`
  - [ ] Fetch patient data with usePatient(id)
  - [ ] Display patient demographics
  - [ ] Display contact information
  - [ ] Display medical history
  - [ ] Display emergency contacts
  - [ ] Display insurance information
  - **File**: `app/dashboard/patients/[id]/page.tsx`

- [ ] **Patient Tabs Interface**
  - [ ] Overview tab (demographics)
  - [ ] Medical History tab
  - [ ] Test History tab (list of all orders)
  - [ ] Reports tab (list of all reports)
  - [ ] Documents tab (uploaded files)
  - [ ] Activity Log tab
  - **File**: `app/dashboard/patients/[id]/page.tsx`

- [ ] **Edit Patient Modal**
  - [ ] Edit demographics
  - [ ] Edit contact info
  - [ ] Edit medical history
  - [ ] Edit emergency contacts
  - [ ] Edit insurance
  - [ ] Use UPDATE_PATIENT mutation
  - **File**: `components/patients/edit-patient-modal.tsx`

- [ ] **Patient Actions**
  - [ ] Order tests button
  - [ ] View reports button
  - [ ] Print patient card
  - [ ] Archive patient
  - [ ] Merge patients (duplicate handling)
  - **File**: `components/patients/patient-actions.tsx`

#### C. Patient Registration Enhancement
- [x] Form exists
- [ ] **Connect to GraphQL**
  - [ ] Replace mutation with CREATE_PATIENT
  - [ ] Add proper error handling
  - [ ] Add success redirect to patient detail
  - [ ] Add draft saving (local storage)
  - **File**: `app/dashboard/patients/register/page.tsx`

- [ ] **Add Patient Search Before Create**
  - [ ] Check for duplicates by name/phone
  - [ ] Show existing patient warning
  - [ ] Option to merge or create new
  - **File**: `app/dashboard/patients/register/page.tsx`

### 2.2 Test Ordering Module (NEW - Critical)

#### A. Test Order Creation (NEW)
- [ ] **Create Order Form Page**
  - [ ] Route: `/dashboard/orders/create/page.tsx`
  - [ ] Patient selection (search existing or new)
  - [ ] Doctor/Referring physician selection
  - [ ] Test catalog with search
  - [ ] Multi-select tests with checkboxes
  - [ ] Sample type auto-selection
  - [ ] Priority selection (Routine/Urgent/STAT)
  - [ ] Clinical indication notes
  - [ ] Billing information
  - [ ] Use CREATE_ORDER mutation
  - **File**: `app/dashboard/orders/create/page.tsx`

- [ ] **Test Catalog Component**
  - [ ] Categorized test list
  - [ ] Test profiles (panels)
  - [ ] Individual tests
  - [ ] Search and filter
  - [ ] Price display
  - [ ] Quick add buttons
  - **File**: `components/orders/test-catalog.tsx`

- [ ] **Order Summary Component**
  - [ ] Selected tests list
  - [ ] Total price calculation
  - [ ] Sample requirements summary
  - [ ] Estimated turnaround time
  - **File**: `components/orders/order-summary.tsx`

#### B. Order Detail Page (NEW)
- [ ] **Create Detail Page**
  - [ ] Route: `/dashboard/orders/[id]/page.tsx`
  - [ ] Fetch order with useOrder(id)
  - [ ] Display patient info
  - [ ] Display ordered tests
  - [ ] Display samples required
  - [ ] Display order status
  - [ ] Display timeline
  - **File**: `app/dashboard/orders/[id]/page.tsx`

- [ ] **Order Actions**
  - [ ] Collect samples button
  - [ ] Modify order (if not collected)
  - [ ] Cancel order (with reason)
  - [ ] Print requisition
  - [ ] Print barcode labels
  - **File**: `components/orders/order-actions.tsx`

#### C. Order List Enhancement
- [x] Basic UI exists
- [ ] **Integrate GraphQL**
  - [ ] Replace mock data with useOrders()
  - [ ] Add real-time status updates
  - [ ] Add filters (status, priority, date)
  - [ ] Add DataTable component
  - **File**: `app/dashboard/orders/page.tsx`

### 2.3 Sample Management Module

#### A. Sample Collection Workflow (NEW)
- [ ] **Create Collection Page**
  - [ ] Route: `/dashboard/samples/collect/page.tsx`
  - [ ] Scan/Enter order number
  - [ ] Display patient and tests
  - [ ] Sample type checklist
  - [ ] Barcode generation
  - [ ] Collection date/time
  - [ ] Collector name
  - [ ] Collection notes
  - [ ] Use CREATE_SAMPLE mutation
  - **File**: `app/dashboard/samples/collect/page.tsx`

- [ ] **Barcode Component**
  - [ ] Generate barcode for samples
  - [ ] Print functionality
  - [ ] QR code option
  - **File**: `components/samples/barcode-generator.tsx`

#### B. Sample Detail Page (NEW)
- [ ] **Create Detail Page**
  - [ ] Route: `/dashboard/samples/[id]/page.tsx`
  - [ ] Fetch sample with useSample(id)
  - [ ] Display sample info
  - [ ] Display chain of custody
  - [ ] Display linked tests
  - [ ] Display current status
  - [ ] Timeline visualization
  - **File**: `app/dashboard/samples/[id]/page.tsx`

- [ ] **Sample Actions**
  - [ ] Receive sample
  - [ ] Process sample
  - [ ] Reject sample (with reason)
  - [ ] Aliquot sample
  - [ ] Dispose sample
  - [ ] Print label
  - **File**: `components/samples/sample-actions.tsx`

#### C. Sample List Enhancement
- [x] Basic UI with WebSocket exists
- [ ] **Integrate GraphQL**
  - [ ] Replace mock data with useSamples()
  - [ ] Keep WebSocket for real-time updates
  - [ ] Add DataTable component
  - [ ] Add advanced filters
  - **File**: `app/dashboard/samples/page.tsx`

- [ ] **Sample Reception Workflow**
  - [ ] Bulk sample receiving
  - [ ] Barcode scanning
  - [ ] Sample condition check
  - [ ] Rejection workflow
  - **File**: `components/samples/sample-reception.tsx`

### 2.4 Result Management Module

#### A. Result Entry Enhancement
- [x] Basic form exists
- [ ] **Integrate with GraphQL**
  - [ ] Fetch test parameters dynamically
  - [ ] Load reference ranges
  - [ ] Use CREATE_RESULT mutation
  - [ ] Auto-save drafts
  - **File**: `app/dashboard/results/[id]/enter/page.tsx`

- [ ] **Batch Result Entry**
  - [ ] Enter results for multiple tests
  - [ ] Table-based entry
  - [ ] Copy previous results
  - [ ] Quick navigation
  - **File**: `app/dashboard/results/batch-entry/page.tsx`

#### B. Result Approval Workflow (NEW - Critical)
- [ ] **Create Review Page**
  - [ ] Route: `/dashboard/results/[id]/review/page.tsx`
  - [ ] Display patient and test info
  - [ ] Display entered results
  - [ ] Show flags (high/low/critical)
  - [ ] Delta check comparison
  - [ ] Technical review checkbox
  - [ ] Medical review checkbox
  - [ ] Comments section
  - [ ] Use VERIFY_RESULT / APPROVE_RESULT mutations
  - **File**: `app/dashboard/results/[id]/review/page.tsx`

- [ ] **Result History Component**
  - [ ] Show previous results for comparison
  - [ ] Graphical trend
  - [ ] Delta check alerts
  - **File**: `components/results/result-history.tsx`

#### C. Result List Enhancement
- [x] Basic UI exists
- [ ] **Integrate GraphQL**
  - [ ] Replace mock data with useResults()
  - [ ] Add filters (status, critical values, date)
  - [ ] Add DataTable component
  - [ ] Critical value highlighting
  - **File**: `app/dashboard/results/page.tsx`

### 2.5 Report Management Module

#### A. Report Generation (NEW - Critical)
- [ ] **Create Generation Page**
  - [ ] Route: `/dashboard/reports/generate/page.tsx`
  - [ ] Select patient
  - [ ] Select completed tests
  - [ ] Select report template
  - [ ] Add interpretation/comments
  - [ ] Digital signature
  - [ ] Use GENERATE_REPORT mutation
  - **File**: `app/dashboard/reports/generate/page.tsx`

#### B. Report Preview & Print (NEW - Critical)
- [ ] **Create Preview Page**
  - [ ] Route: `/dashboard/reports/[id]/preview/page.tsx`
  - [ ] Fetch report with useReport(id)
  - [ ] Display formatted report
  - [ ] Print-friendly layout
  - [ ] PDF download
  - [ ] Email delivery
  - [ ] WhatsApp delivery
  - [ ] SMS notification
  - **File**: `app/dashboard/reports/[id]/preview/page.tsx`

- [ ] **Report Template Component**
  - [ ] Standard template
  - [ ] Letterhead with logo
  - [ ] Patient demographics
  - [ ] Test results table
  - [ ] Reference ranges
  - [ ] Interpretation section
  - [ ] Digital signature
  - [ ] Barcode/QR for verification
  - **File**: `components/reports/report-template.tsx`

#### C. Report List Enhancement
- [x] Basic UI exists
- [ ] **Integrate GraphQL**
  - [ ] Replace mock data with useReports()
  - [ ] Add filters (status, date, patient)
  - [ ] Add DataTable component
  - [ ] Add delivery status tracking
  - **File**: `app/dashboard/reports/page.tsx`

- [ ] **Report Delivery Interface**
  - [ ] Bulk email sending
  - [ ] WhatsApp integration
  - [ ] SMS notifications
  - [ ] Delivery status tracking
  - [ ] Resend options
  - **File**: `components/reports/report-delivery.tsx`

---

## 🔬 Phase 3: Advanced Features (Implement Third)

### 3.1 Quality Control Module

- [ ] **QC Run Entry Form**
  - [ ] QC material selection
  - [ ] Level selection (L1, L2, L3)
  - [ ] Parameter values entry
  - [ ] Auto-calculation of statistics
  - [ ] Pass/Fail determination
  - [ ] Westgard rules application
  - **File**: `app/dashboard/qc/run-entry/page.tsx`

- [ ] **Levey-Jennings Charts**
  - [ ] Chart visualization library integration
  - [ ] Mean and SD lines
  - [ ] Control limits (1SD, 2SD, 3SD)
  - [ ] Rule violation highlighting
  - [ ] Date range selection
  - **File**: `components/qc/levey-jennings-chart.tsx`

- [ ] **QC Material Management**
  - [ ] Add QC materials
  - [ ] Lot tracking
  - [ ] Expiry management
  - [ ] Expected values setup
  - **File**: `app/dashboard/qc/materials/page.tsx`

### 3.2 Equipment Management Module

- [ ] **Equipment Detail Page**
  - [ ] Route: `/dashboard/equipment/[id]/page.tsx`
  - [ ] Equipment specifications
  - [ ] Maintenance schedule
  - [ ] Service history
  - [ ] Calibration records
  - [ ] Utilization stats
  - **File**: `app/dashboard/equipment/[id]/page.tsx`

- [ ] **Maintenance Scheduling**
  - [ ] Calendar view
  - [ ] Preventive maintenance schedule
  - [ ] Maintenance log entry
  - [ ] Service request creation
  - **File**: `app/dashboard/equipment/maintenance/page.tsx`

- [ ] **Equipment Forms**
  - [ ] Add equipment modal
  - [ ] Edit equipment modal
  - [ ] Maintenance log modal
  - [ ] Calibration record modal
  - **Files**: `components/equipment/*.tsx`

### 3.3 Inventory Management Module

- [ ] **Inventory Item Detail**
  - [ ] Route: `/dashboard/inventory/[id]/page.tsx`
  - [ ] Item details
  - [ ] Stock levels by location
  - [ ] Transaction history
  - [ ] Reorder point info
  - **File**: `app/dashboard/inventory/[id]/page.tsx`

- [ ] **Stock Transactions**
  - [ ] Stock in (receiving)
  - [ ] Stock out (usage)
  - [ ] Stock transfer
  - [ ] Stock adjustment
  - [ ] Stock count/audit
  - **File**: `app/dashboard/inventory/transactions/page.tsx`

- [ ] **Purchase Order Management**
  - [ ] Create PO
  - [ ] Approve PO
  - [ ] Receive items
  - [ ] Track delivery
  - **File**: `app/dashboard/inventory/purchase-orders/page.tsx`

- [ ] **Expiry Management**
  - [ ] Expiring items dashboard
  - [ ] FEFO tracking (First Expired, First Out)
  - [ ] Alerts and notifications
  - **File**: `app/dashboard/inventory/expiry/page.tsx`

### 3.4 Billing Module

- [ ] **Invoice Creation**
  - [ ] Patient selection
  - [ ] Service selection
  - [ ] Pricing calculation
  - [ ] Discount application
  - [ ] Tax calculation (GST)
  - [ ] Payment terms
  - **File**: `app/dashboard/billing/create/page.tsx`

- [ ] **Invoice Detail Page**
  - [ ] Route: `/dashboard/billing/[id]/page.tsx`
  - [ ] Invoice details
  - [ ] Payment history
  - [ ] Print invoice
  - [ ] Email invoice
  - **File**: `app/dashboard/billing/[id]/page.tsx`

- [ ] **Payment Recording**
  - [ ] Payment entry modal
  - [ ] Multiple payment methods
  - [ ] Partial payments
  - [ ] Receipt generation
  - **File**: `components/billing/payment-modal.tsx`

- [ ] **Pricing Management**
  - [ ] Test price catalog
  - [ ] Package pricing
  - [ ] Discount rules
  - [ ] Insurance pricing
  - **File**: `app/dashboard/billing/pricing/page.tsx`

### 3.5 User Management Module

- [ ] **User Creation Form**
  - [ ] Add user modal
  - [ ] Personal information
  - [ ] Role assignment
  - [ ] Permission customization
  - [ ] Initial password setup
  - **File**: `components/users/create-user-modal.tsx`

- [ ] **User Detail Page**
  - [ ] Route: `/dashboard/users/[id]/page.tsx`
  - [ ] User profile
  - [ ] Assigned roles and permissions
  - [ ] Activity log
  - [ ] Session history
  - **File**: `app/dashboard/users/[id]/page.tsx`

- [ ] **Role & Permission Management**
  - [ ] Role list
  - [ ] Permission matrix
  - [ ] Custom role creation
  - [ ] Permission assignment UI
  - **File**: `app/dashboard/users/roles/page.tsx`

---

## 🎨 Phase 4: UI/UX Polish (Implement Fourth)

### 4.1 Global Features

- [ ] **Global Search**
  - [ ] Search bar in header
  - [ ] Multi-entity search (patients, orders, samples)
  - [ ] Quick navigation
  - [ ] Search history
  - **File**: `components/global-search.tsx`

- [ ] **Command Palette (Cmd+K)**
  - [ ] Quick actions
  - [ ] Navigation shortcuts
  - [ ] Search shortcuts
  - **File**: `components/command-palette.tsx`

- [ ] **Activity Feed**
  - [ ] Real-time activity stream
  - [ ] Filterable by entity type
  - [ ] Notifications integration
  - **File**: `components/activity-feed.tsx`

### 4.2 Dashboard Enhancements

- [ ] **Admin Dashboard Analytics**
  - [ ] Revenue charts
  - [ ] Test volume charts
  - [ ] Turnaround time metrics
  - [ ] User activity stats
  - **File**: `app/dashboard/admin/page.tsx`

- [ ] **Widget System**
  - [ ] Customizable dashboard widgets
  - [ ] Drag-and-drop layout
  - [ ] Widget library
  - [ ] User preferences
  - **File**: `components/dashboard/widgets/*.tsx`

### 4.3 Advanced UI Components

- [ ] **Date Range Picker**
  - [ ] Range selection
  - [ ] Presets (Today, Last 7 days, etc.)
  - [ ] Custom range
  - **File**: `components/ui/date-range-picker.tsx`

- [ ] **Advanced Select/Combobox**
  - [ ] Multi-select
  - [ ] Async search
  - [ ] Tag input
  - [ ] Grouping
  - **File**: `components/ui/combobox.tsx`

- [ ] **Rich Text Editor**
  - [ ] For clinical notes/interpretations
  - [ ] Formatting options
  - [ ] Template insertion
  - **File**: `components/ui/rich-text-editor.tsx`

### 4.4 Print & Export

- [ ] **Print Layouts**
  - [ ] Patient card
  - [ ] Sample labels
  - [ ] Work lists
  - [ ] QC reports
  - **Files**: `components/print/*.tsx`

- [ ] **Export Functionality**
  - [ ] Export to Excel
  - [ ] Export to CSV
  - [ ] Export to PDF
  - [ ] Bulk export
  - **File**: `lib/export.ts`

---

## 🔌 Phase 5: Integrations (Implement Fifth)

### 5.1 External Service Integrations

- [ ] **ABDM Integration**
  - [ ] Health ID verification
  - [ ] PHR integration
  - [ ] Health facility registry
  - **Files**: `lib/integrations/abdm/*.ts`

- [ ] **Payment Gateway**
  - [ ] Razorpay integration
  - [ ] Payment collection UI
  - [ ] Payment status tracking
  - **Files**: `lib/integrations/payment/*.ts`

- [ ] **Email Service**
  - [ ] SMTP configuration
  - [ ] Email templates
  - [ ] Bulk email sending
  - **Files**: `lib/integrations/email/*.ts`

- [ ] **SMS Gateway**
  - [ ] SMS notification sending
  - [ ] Template management
  - [ ] Delivery tracking
  - **Files**: `lib/integrations/sms/*.ts`

- [ ] **WhatsApp Business API**
  - [ ] Message sending
  - [ ] Template messages
  - [ ] Status tracking
  - **Files**: `lib/integrations/whatsapp/*.ts`

### 5.2 Instrument Interfaces

- [ ] **Instrument Connection**
  - [ ] Auto-import results
  - [ ] Mapping configuration
  - [ ] Error handling
  - **Files**: `lib/integrations/instruments/*.ts`

---

## 📊 Implementation Tracking

### File Count Estimates

| Category | Current | Target | Missing |
|----------|---------|--------|---------|
| Pages | 24 | 45+ | 21+ |
| Components | 15 | 80+ | 65+ |
| GraphQL Queries | 0 | 20+ | 20+ |
| GraphQL Mutations | 3 | 30+ | 27+ |
| Custom Hooks | 0 | 25+ | 25+ |
| Utility Functions | 5 | 30+ | 25+ |

### Total LOC Estimates

| Module | Current LOC | Target LOC | Missing LOC |
|--------|-------------|------------|-------------|
| Pages | ~5,000 | ~15,000 | ~10,000 |
| Components | ~2,000 | ~20,000 | ~18,000 |
| GraphQL | ~500 | ~5,000 | ~4,500 |
| Hooks | ~100 | ~3,000 | ~2,900 |
| Utils | ~500 | ~2,000 | ~1,500 |
| **Total** | **~8,100** | **~45,000** | **~36,900** |

---

## 🎯 Immediate Next Steps (Priority Order)

### Step 1: Foundation Components (Week 1)
1. Create DataTable component (1 day)
2. Create Modal components (1 day)
3. Create FileUpload component (0.5 day)
4. Create SearchBar component (0.5 day)
5. Create Skeleton loaders (0.5 day)
6. Set up GraphQL queries structure (0.5 day)
7. Set up GraphQL mutations structure (0.5 day)
8. Create custom hooks (1 day)

### Step 2: Patient Management (Week 2)
1. Patient Detail Page (2 days)
2. Edit Patient Modal (1 day)
3. Patient List with DataTable (1 day)
4. Patient Registration with GraphQL (1 day)

### Step 3: Test Ordering (Week 2-3)
1. Test Order Creation Page (2 days)
2. Order Detail Page (1 day)
3. Order List with DataTable (1 day)
4. Test Catalog Component (1 day)

### Step 4: Sample Management (Week 3-4)
1. Sample Collection Page (2 days)
2. Sample Detail Page (1 day)
3. Sample Reception Workflow (1 day)
4. Sample List with GraphQL (1 day)

### Step 5: Result Management (Week 4)
1. Result Entry with GraphQL (1 day)
2. Result Review/Approval Page (2 days)
3. Result List with DataTable (1 day)

### Step 6: Report Management (Week 5)
1. Report Generation Page (2 days)
2. Report Preview/Print (2 days)
3. Report Template Component (1 day)

---

## 📝 Development Guidelines

### Coding Standards
- Use TypeScript strict mode
- Follow existing component patterns
- Use custom hooks for data fetching
- Implement proper error handling
- Add loading states everywhere
- Use proper TypeScript interfaces
- Document complex logic with comments

### Testing Strategy
- Write unit tests for utilities
- Integration tests for GraphQL operations
- E2E tests for critical workflows
- Manual testing for UI/UX

### Git Workflow
- Feature branches for each module
- Descriptive commit messages
- Pull request for code review
- Squash commits before merge

---

## ✅ Definition of "Done"

For each feature to be considered complete:
- [ ] Code implementation finished
- [ ] GraphQL integration working
- [ ] Error handling implemented
- [ ] Loading states added
- [ ] Responsive design verified
- [ ] TypeScript types correct
- [ ] No console errors
- [ ] Tested with backend API
- [ ] Code reviewed (if team)
- [ ] Documentation updated

---

## 🚀 Ready to Start!

This plan provides:
1. ✅ Complete feature inventory
2. ✅ Prioritized implementation order
3. ✅ Detailed task breakdown
4. ✅ File structure guidance
5. ✅ Time estimates
6. ✅ Clear dependencies

**Recommendation**: Start with Phase 1 (Foundation Components) as they are dependencies for all other phases.

Shall I begin implementation with the DataTable component and GraphQL setup?
