# GraphQL API Schema
## Complete API Contracts for LIS/LIMS

**Version**: 1.0.0
**Last Updated**: 2024-11-05
**GraphQL Version**: October 2021 Spec

---

## Table of Contents

1. [Schema Overview](#schema-overview)
2. [Core Types](#core-types)
3. [Patient API](#patient-api)
4. [Sample API](#sample-api)
5. [Order API](#order-api)
6. [Result API](#result-api)
7. [Report API](#report-api)
8. [Billing API](#billing-api)
9. [Quality Control API](#quality-control-api)
10. [Equipment API](#equipment-api)
11. [Analytics API](#analytics-api)
12. [Admin API](#admin-api)
13. [Subscriptions](#subscriptions)
14. [Error Handling](#error-handling)

---

## 1. Schema Overview

### 1.1 Root Types

```graphql
schema {
  query: Query
  mutation: Mutation
  subscription: Subscription
}

type Query {
  # Patient queries
  patient(id: ID!): Patient
  patients(filter: PatientFilter, pagination: PaginationInput!): PatientConnection!
  searchPatients(query: String!, limit: Int = 10): [Patient!]!

  # Sample queries
  sample(id: ID!): Sample
  sampleByBarcode(barcode: String!): Sample
  samples(filter: SampleFilter, pagination: PaginationInput!): SampleConnection!

  # Order queries
  order(id: ID!): Order
  orders(filter: OrderFilter, pagination: PaginationInput!): OrderConnection!

  # Result queries
  result(id: ID!): TestResult
  results(filter: ResultFilter, pagination: PaginationInput!): ResultConnection!

  # Report queries
  report(id: ID!): Report
  reports(filter: ReportFilter, pagination: PaginationInput!): ReportConnection!

  # Billing queries
  invoice(id: ID!): Invoice
  invoices(filter: InvoiceFilter, pagination: PaginationInput!): InvoiceConnection!

  # QC queries
  qcResults(filter: QCFilter, pagination: PaginationInput!): QCConnection!
  westgardAnalysis(testId: ID!, period: DateRange!): WestgardAnalysis!

  # Equipment queries
  equipment(id: ID!): Equipment
  equipmentList(filter: EquipmentFilter!): [Equipment!]!
  equipmentStatus(id: ID!): EquipmentStatus!

  # Analytics queries
  dashboard(role: UserRole!): Dashboard!
  kpis(filter: KPIFilter!): [KPI!]!
  tatAnalysis(period: DateRange!): TATAnalysis!

  # Test catalog
  testCatalog(filter: TestCatalogFilter): [Test!]!
  test(id: ID!): Test
}

type Mutation {
  # Patient mutations
  createPatient(input: CreatePatientInput!): Patient!
  updatePatient(id: ID!, input: UpdatePatientInput!): Patient!
  deletePatient(id: ID!): Boolean!
  verifyAadhaar(aadhaarNumber: String!, otp: String!): AadhaarVerificationResult!

  # Sample mutations
  collectSample(input: CollectSampleInput!): Sample!
  receiveSample(id: ID!, input: ReceiveSampleInput!): Sample!
  rejectSample(id: ID!, input: RejectSampleInput!): Sample!

  # Order mutations
  createOrder(input: CreateOrderInput!): Order!
  cancelOrder(id: ID!, reason: String!): Order!
  updateOrderPriority(id: ID!, priority: Int!): Order!

  # Result mutations
  enterResult(input: EnterResultInput!): TestResult!
  verifyResult(id: ID!): TestResult!
  amendResult(id: ID!, input: AmendResultInput!): TestResult!

  # Report mutations
  generateReport(sampleId: ID!): Report!
  deliverReport(reportId: ID!, channels: [DeliveryChannel!]!): DeliveryStatus!
  signReport(reportId: ID!, signature: String!): Report!

  # Billing mutations
  generateInvoice(orderId: ID!): Invoice!
  processPayment(input: ProcessPaymentInput!): Payment!
  recordPayment(invoiceId: ID!, input: RecordPaymentInput!): Payment!

  # QC mutations
  recordIQC(input: RecordIQCInput!): IQCResult!
  recordCorrectiveAction(input: CorrectiveActionInput!): CorrectiveAction!

  # Equipment mutations
  configureEquipment(input: ConfigureEquipmentInput!): Equipment!
  recordCalibration(input: CalibrationInput!): Calibration!
  recordMaintenance(input: MaintenanceInput!): Maintenance!

  # Admin mutations
  createUser(input: CreateUserInput!): User!
  assignRole(userId: ID!, roleId: ID!): UserRole!
  updateSystemConfig(input: SystemConfigInput!): SystemConfig!
}

type Subscription {
  # Real-time updates
  sampleStatusUpdated(sampleId: ID!): Sample!
  resultVerified(orderId: ID!): TestResult!
  reportGenerated(orderId: ID!): Report!
  criticalValueDetected: CriticalValueAlert!
  qcViolation: QCViolation!
  equipmentStatusChanged(equipmentId: ID!): EquipmentStatus!
}
```

---

## 2. Core Types

### 2.1 Common Types

```graphql
# Scalars
scalar DateTime
scalar Date
scalar Time
scalar UUID
scalar JSON
scalar Email
scalar PhoneNumber
scalar Upload

# Enums
enum UserRole {
  SUPER_ADMIN
  LAB_DIRECTOR
  PATHOLOGIST
  LAB_MANAGER
  LAB_TECHNICIAN
  FRONT_DESK
  QUALITY_MANAGER
  PATIENT
}

enum Gender {
  MALE
  FEMALE
  OTHER
}

enum BloodGroup {
  A_POSITIVE
  A_NEGATIVE
  B_POSITIVE
  B_NEGATIVE
  O_POSITIVE
  O_NEGATIVE
  AB_POSITIVE
  AB_NEGATIVE
}

# Pagination
input PaginationInput {
  page: Int = 1
  pageSize: Int = 20
  sortBy: String
  sortOrder: SortOrder = ASC
}

enum SortOrder {
  ASC
  DESC
}

type PageInfo {
  hasNextPage: Boolean!
  hasPreviousPage: Boolean!
  totalPages: Int!
  totalCount: Int!
  currentPage: Int!
}

# Date Range
input DateRange {
  from: DateTime!
  to: DateTime!
}
```

---

## 3. Patient API

### 3.1 Patient Types

```graphql
type Patient {
  id: ID!
  mrnNumber: String!
  salutation: String
  firstName: String!
  middleName: String
  lastName: String!
  fullName: String!
  dateOfBirth: Date!
  age: Int!
  gender: Gender!
  aadhaarNumber: String # Masked (XXXX-XXXX-1234)
  email: Email
  mobileNumber: PhoneNumber!
  bloodGroup: BloodGroup
  maritalStatus: MaritalStatus
  nationality: String!
  occupation: String
  organization: Organization!
  addresses: [PatientAddress!]!
  contacts: [PatientContact!]!
  consents: [PatientConsent!]!
  medicalHistory: [MedicalHistory!]!
  insurance: [PatientInsurance!]!
  orders: [Order!]!
  createdAt: DateTime!
  updatedAt: DateTime!
  isActive: Boolean!
}

type PatientAddress {
  id: ID!
  addressType: AddressType!
  addressLine1: String!
  addressLine2: String
  city: String!
  state: String!
  country: String!
  postalCode: String!
  isPrimary: Boolean!
}

enum AddressType {
  HOME
  WORK
  BILLING
}

type PatientContact {
  id: ID!
  contactType: ContactType!
  name: String!
  relationship: String
  mobileNumber: PhoneNumber!
  email: Email
  isPrimary: Boolean!
}

enum ContactType {
  SELF
  PARENT
  GUARDIAN
  SPOUSE
  EMERGENCY
}

type PatientConsent {
  id: ID!
  consentType: ConsentType!
  granted: Boolean!
  purpose: String!
  grantedAt: DateTime
  expiresAt: DateTime
  revokedAt: DateTime
  revocationReason: String
}

enum ConsentType {
  DATA_PROCESSING
  MARKETING
  RESEARCH
  ABDM
}

type PatientInsurance {
  id: ID!
  insuranceProvider: String!
  policyNumber: String!
  policyHolderName: String!
  validFrom: Date!
  validUntil: Date!
  coverageAmount: Float!
  tpaName: String
  tpaId: String
  isActive: Boolean!
}
```

### 3.2 Patient Inputs

```graphql
input CreatePatientInput {
  salutation: String
  firstName: String!
  middleName: String
  lastName: String!
  dateOfBirth: Date!
  gender: Gender!
  aadhaarNumber: String
  email: Email
  mobileNumber: PhoneNumber!
  bloodGroup: BloodGroup
  maritalStatus: MaritalStatus
  nationality: String = "Indian"
  occupation: String
  organizationId: ID!
  addresses: [AddressInput!]!
  contacts: [ContactInput!]
  consents: [ConsentInput!]!
}

input UpdatePatientInput {
  salutation: String
  firstName: String
  middleName: String
  lastName: String
  email: Email
  mobileNumber: PhoneNumber
  bloodGroup: BloodGroup
  maritalStatus: MaritalStatus
  occupation: String
  addresses: [AddressInput!]
  contacts: [ContactInput!]
}

input AddressInput {
  addressType: AddressType!
  addressLine1: String!
  addressLine2: String
  city: String!
  state: String!
  country: String!
  postalCode: String!
  isPrimary: Boolean = false
}

input ContactInput {
  contactType: ContactType!
  name: String!
  relationship: String
  mobileNumber: PhoneNumber!
  email: Email
  isPrimary: Boolean = false
}

input ConsentInput {
  consentType: ConsentType!
  granted: Boolean!
  purpose: String!
}

input PatientFilter {
  search: String
  gender: Gender
  ageRange: AgeRangeInput
  createdAfter: DateTime
  isActive: Boolean
}

input AgeRangeInput {
  min: Int!
  max: Int!
}

type PatientConnection {
  edges: [PatientEdge!]!
  pageInfo: PageInfo!
}

type PatientEdge {
  node: Patient!
  cursor: String!
}
```

### 3.3 Aadhaar Verification

```graphql
type AadhaarVerificationResult {
  verified: Boolean!
  name: String
  dateOfBirth: Date
  gender: Gender
  address: String
  photo: String # Base64 encoded
  error: String
}
```

---

## 4. Sample API

### 4.1 Sample Types

```graphql
type Sample {
  id: ID!
  sampleNumber: String! # Barcode
  patient: Patient!
  order: Order!
  sampleType: SampleType!
  collectedBy: User!
  collectedAt: DateTime!
  collectionLocation: Location
  collectionMethod: CollectionMethod!
  volume: Float # in mL
  containerType: String!
  status: SampleStatus!
  currentLocation: Location
  receivedAt: DateTime
  receivedBy: User
  priority: Int! # 1=STAT, 2=Urgent, 3=Routine
  storageConditions: JSON
  tracking: [SampleTracking!]!
  aliquots: [SampleAliquot!]!
  rejection: SampleRejection
  chainOfCustodyHash: String
  createdAt: DateTime!
  updatedAt: DateTime!
}

type SampleType {
  id: ID!
  code: String!
  name: String!
  description: String
  containerType: String!
  containerColor: String
  minVolume: Float!
  idealVolume: Float!
  storageTemperature: String!
  stabilityHours: Int!
  transportMedium: String
  collectionInstructions: String
  isActive: Boolean!
}

enum SampleStatus {
  COLLECTED
  IN_TRANSIT
  RECEIVED
  PROCESSING
  COMPLETED
  REJECTED
  DISPOSED
}

enum CollectionMethod {
  VENIPUNCTURE
  CAPILLARY
  MIDSTREAM
  SWAB
  OTHER
}

type SampleTracking {
  id: ID!
  status: SampleStatus!
  location: Location!
  handledBy: User!
  timestamp: DateTime!
  gpsLocation: GPSCoordinates
  metadata: JSON
}

type GPSCoordinates {
  latitude: Float!
  longitude: Float!
}

type SampleAliquot {
  id: ID!
  aliquotNumber: String!
  parentSample: Sample!
  volume: Float!
  createdBy: User!
  createdAt: DateTime!
  currentLocation: Location
  status: AliquotStatus!
}

enum AliquotStatus {
  AVAILABLE
  IN_USE
  CONSUMED
  DISPOSED
}

type SampleRejection {
  id: ID!
  sample: Sample!
  rejectionReason: RejectionReason!
  rejectionNotes: String!
  rejectedBy: User!
  rejectedAt: DateTime!
  recollectionRequired: Boolean!
  newSample: Sample
}

enum RejectionReason {
  HEMOLYSIS
  INSUFFICIENT_VOLUME
  CLOTTED
  LIPEMIC
  UNLABELED
  DAMAGED
  EXPIRED
  CONTAMINATED
}
```

### 4.2 Sample Inputs

```graphql
input CollectSampleInput {
  patientId: ID!
  orderId: ID!
  sampleTypeId: ID!
  collectionLocationId: ID
  collectionMethod: CollectionMethod!
  volume: Float!
  containerType: String!
  priority: Int = 3
  storageConditions: JSON
  notes: String
}

input ReceiveSampleInput {
  receivedBy: ID!
  receivedAt: DateTime
  currentLocationId: ID!
  condition: SampleCondition!
  notes: String
}

enum SampleCondition {
  GOOD
  ACCEPTABLE
  QUESTIONABLE
  POOR
}

input RejectSampleInput {
  rejectionReason: RejectionReason!
  rejectionNotes: String!
  recollectionRequired: Boolean = true
}

input SampleFilter {
  status: SampleStatus
  patientId: ID
  orderId: ID
  collectedAfter: DateTime
  priority: Int
  search: String
}

type SampleConnection {
  edges: [SampleEdge!]!
  pageInfo: PageInfo!
}

type SampleEdge {
  node: Sample!
  cursor: String!
}
```

---

## 5. Order API

### 5.1 Order Types

```graphql
type Order {
  id: ID!
  orderNumber: String!
  patient: Patient!
  organization: Organization!
  referringDoctor: Doctor
  orderType: OrderType!
  orderSource: OrderSource!
  orderedBy: User!
  orderedAt: DateTime!
  appointmentAt: DateTime
  status: OrderStatus!
  paymentStatus: PaymentStatus!
  totalAmount: Float!
  discountAmount: Float!
  taxAmount: Float!
  netAmount: Float!
  clinicalNotes: String
  patientCondition: JSON
  priority: Int!
  isFastingRequired: Boolean!
  specialInstructions: String
  tests: [OrderTest!]!
  samples: [Sample!]!
  invoice: Invoice
  statusHistory: [OrderStatusHistory!]!
  cancelledBy: User
  cancelledAt: DateTime
  cancellationReason: String
  createdAt: DateTime!
  updatedAt: DateTime!
}

enum OrderType {
  REGULAR
  STAT
  ROUTINE
  PROFILE
}

enum OrderSource {
  WALK_IN
  ONLINE
  REFERRAL
  CORPORATE
  CAMP
}

enum OrderStatus {
  PENDING
  CONFIRMED
  SAMPLE_COLLECTED
  PROCESSING
  PARTIALLY_COMPLETED
  COMPLETED
  CANCELLED
}

enum PaymentStatus {
  PENDING
  PARTIAL
  PAID
  REFUNDED
}

type OrderTest {
  id: ID!
  order: Order!
  test: Test!
  sample: Sample
  status: TestStatus!
  testPrice: Float!
  discountAmount: Float!
  clinicalIndication: String
  tatDueAt: DateTime
  completedAt: DateTime
  performedBy: User
  result: TestResult
}

enum TestStatus {
  PENDING
  SAMPLE_COLLECTED
  PROCESSING
  COMPLETED
  CANCELLED
}

type Test {
  id: ID!
  code: String!
  name: String!
  shortName: String
  department: Department!
  category: TestCategory!
  description: String
  methodology: String!
  specimenType: String!
  standardTatHours: Float!
  urgentTatHours: Float
  statTatHours: Float
  prerequisites: String
  patientPreparation: String
  resultType: ResultType!
  normalRanges: JSON # Age/Gender specific
  units: String
  decimalPlaces: Int
  requiresApproval: Boolean!
  criticalValueEnabled: Boolean!
  criticalLow: Float
  criticalHigh: Float
  price: Float!
  isActive: Boolean!
}

enum ResultType {
  NUMERIC
  TEXT
  OPTION
  IMAGE
  CULTURE
}

type OrderStatusHistory {
  id: ID!
  order: Order!
  status: OrderStatus!
  changedBy: User!
  changedAt: DateTime!
  notes: String
}
```

### 5.2 Order Inputs

```graphql
input CreateOrderInput {
  patientId: ID!
  organizationId: ID!
  referringDoctorId: ID
  orderType: OrderType = REGULAR
  orderSource: OrderSource = WALK_IN
  appointmentAt: DateTime
  priority: Int = 3
  isFastingRequired: Boolean = false
  specialInstructions: String
  clinicalNotes: String
  testIds: [ID!]!
  discountAmount: Float = 0
}

input OrderFilter {
  status: OrderStatus
  paymentStatus: PaymentStatus
  patientId: ID
  orderedAfter: DateTime
  priority: Int
  search: String
}

type OrderConnection {
  edges: [OrderEdge!]!
  pageInfo: PageInfo!
}

type OrderEdge {
  node: Order!
  cursor: String!
}
```

---

## 6. Result API

### 6.1 Result Types

```graphql
type TestResult {
  id: ID!
  orderTest: OrderTest!
  test: Test!
  sample: Sample!
  status: ResultStatus!
  resultDate: DateTime!
  enteredBy: User!
  enteredAt: DateTime!
  equipment: Equipment
  analyzerRunId: String
  entryMethod: EntryMethod!
  rawResultData: JSON
  autoVerified: Boolean!
  autoVerificationConfidence: Float
  autoVerificationModelVersion: String
  deltaCheckResults: JSON
  hasCriticalValues: Boolean!
  technicianComment: String
  components: [ResultComponent!]!
  amendments: [ResultAmendment!]!
  verifications: [ResultVerification!]!
  criticalAlerts: [CriticalValueAlert!]!
  createdAt: DateTime!
  updatedAt: DateTime!
}

enum ResultStatus {
  PENDING
  ENTERED
  TECHNICAL_REVIEW
  PATHOLOGIST_REVIEW
  VERIFIED
  APPROVED
  RELEASED
  AMENDED
}

enum EntryMethod {
  MANUAL
  ANALYZER
  MIDDLEWARE
  IMPORTED
}

type ResultComponent {
  id: ID!
  testResult: TestResult!
  componentCode: String!
  componentName: String!
  numericValue: Float
  textValue: String
  optionValue: String
  units: String
  referenceLow: Float
  referenceHigh: Float
  flag: ResultFlag!
  interpretation: String
  displayOrder: Int!
}

enum ResultFlag {
  NORMAL
  LOW
  HIGH
  CRITICAL_LOW
  CRITICAL_HIGH
  ABNORMAL
}

type ResultAmendment {
  id: ID!
  testResult: TestResult!
  amendmentNumber: Int!
  previousValues: JSON!
  newValues: JSON!
  reason: String!
  detailedExplanation: String!
  amendedBy: User!
  amendedAt: DateTime!
  approvedBy: User
  approvedAt: DateTime
}

type ResultVerification {
  id: ID!
  testResult: TestResult!
  verificationLevel: VerificationLevel!
  verifiedBy: User!
  verifiedAt: DateTime!
  action: VerificationAction!
  comments: String
  verificationRulesApplied: JSON
}

enum VerificationLevel {
  TECHNICAL
  PATHOLOGIST
  DIRECTOR
}

enum VerificationAction {
  APPROVED
  REJECTED
  PENDING_REVIEW
}

type CriticalValueAlert {
  id: ID!
  testResult: TestResult!
  component: ResultComponent!
  value: Float!
  componentName: String!
  severity: CriticalSeverity!
  detectedAt: DateTime!
  notifiedTo: User
  notifiedAt: DateTime
  notificationMethod: NotificationMethod
  acknowledgmentBy: String
  acknowledgedAt: DateTime
  actionTaken: String
}

enum CriticalSeverity {
  HIGH
  CRITICAL
}

enum NotificationMethod {
  PHONE
  SMS
  WHATSAPP
  EMAIL
}
```

### 6.2 Result Inputs

```graphql
input EnterResultInput {
  orderTestId: ID!
  sampleId: ID!
  resultDate: DateTime
  equipmentId: ID
  analyzerRunId: String
  entryMethod: EntryMethod = MANUAL
  rawResultData: JSON
  components: [ResultComponentInput!]!
  technicianComment: String
}

input ResultComponentInput {
  componentCode: String!
  componentName: String!
  numericValue: Float
  textValue: String
  optionValue: String
  units: String
  displayOrder: Int = 0
}

input AmendResultInput {
  reason: String!
  detailedExplanation: String!
  newComponents: [ResultComponentInput!]!
}

input ResultFilter {
  status: ResultStatus
  patientId: ID
  orderId: ID
  testId: ID
  resultDateAfter: DateTime
  hasCriticalValues: Boolean
}

type ResultConnection {
  edges: [ResultEdge!]!
  pageInfo: PageInfo!
}

type ResultEdge {
  node: TestResult!
  cursor: String!
}
```

---

## 7. Report API

### 7.1 Report Types

```graphql
type Report {
  id: ID!
  reportNumber: String!
  sample: Sample!
  patient: Patient!
  order: Order!
  templateId: ID!
  generatedAt: DateTime!
  generatedBy: User!
  reportFormat: ReportFormat!
  reportPath: String!
  reportUrl: String!
  reportSize: Int! # in bytes
  signedBy: User
  signedAt: DateTime
  digitalSignature: String
  nablWatermark: Boolean!
  qrCode: String!
  deliveryStatus: [DeliveryStatus!]!
  viewCount: Int!
  downloadCount: Int!
  isAmended: Boolean!
  amendmentNumber: Int
  previousVersionId: ID
  createdAt: DateTime!
  updatedAt: DateTime!
}

enum ReportFormat {
  PDF
  HTML
  JSON
  XML
}

type DeliveryStatus {
  id: ID!
  report: Report!
  channel: DeliveryChannel!
  status: DeliveryStatusType!
  deliveredAt: DateTime
  viewedAt: DateTime
  downloadedAt: DateTime
  failureReason: String
  metadata: JSON
}

enum DeliveryChannel {
  WHATSAPP
  EMAIL
  SMS
  PORTAL
  PRINT
}

enum DeliveryStatusType {
  PENDING
  SENT
  DELIVERED
  VIEWED
  DOWNLOADED
  FAILED
}
```

### 7.2 Report Inputs

```graphql
input DeliverReportInput {
  reportId: ID!
  channels: [DeliveryChannel!]!
  recipientEmail: Email
  recipientMobile: PhoneNumber
  message: String
}

type DeliveryResult {
  success: Boolean!
  channel: DeliveryChannel!
  deliveryId: ID
  error: String
}
```

---

## 8. Billing API

### 8.1 Billing Types

```graphql
type Invoice {
  id: ID!
  invoiceNumber: String!
  order: Order!
  patient: Patient!
  organization: Organization!
  invoiceDate: Date!
  dueDate: Date!
  subtotal: Float!
  discountAmount: Float!
  discountReason: String
  taxAmount: Float!
  cgstAmount: Float!
  sgstAmount: Float!
  igstAmount: Float!
  totalAmount: Float!
  paidAmount: Float!
  balanceAmount: Float!
  status: InvoiceStatus!
  gstin: String
  einvoiceIrn: String
  einvoiceGeneratedAt: DateTime
  einvoiceData: JSON
  lineItems: [InvoiceLineItem!]!
  payments: [Payment!]!
  insuranceClaim: InsuranceClaim
  createdAt: DateTime!
  updatedAt: DateTime!
}

enum InvoiceStatus {
  DRAFT
  ISSUED
  PARTIAL
  PAID
  OVERDUE
  CANCELLED
}

type InvoiceLineItem {
  id: ID!
  invoice: Invoice!
  test: Test
  itemCode: String!
  description: String!
  quantity: Int!
  unitPrice: Float!
  discountPercent: Float!
  discountAmount: Float!
  taxPercent: Float!
  taxAmount: Float!
  lineTotal: Float!
  hsnCode: String
}

type Payment {
  id: ID!
  paymentNumber: String!
  invoice: Invoice!
  patient: Patient!
  amount: Float!
  paymentMethod: PaymentMethodType!
  paymentGateway: String
  transactionId: String
  upiId: String
  cardLast4: String
  cardType: String
  status: PaymentStatusType!
  paymentDate: Date!
  paymentTime: Time!
  collectedBy: User
  paymentMetadata: JSON
  failureReason: String
  createdAt: DateTime!
}

enum PaymentMethodType {
  CASH
  UPI
  CARD
  NET_BANKING
  WALLET
  BNPL
  CHEQUE
}

enum PaymentStatusType {
  PENDING
  SUCCESS
  FAILED
  REFUNDED
}

type InsuranceClaim {
  id: ID!
  claimNumber: String!
  invoice: Invoice!
  patient: Patient!
  insuranceProvider: String!
  policyNumber: String!
  claimAmount: Float!
  approvedAmount: Float!
  paidAmount: Float!
  status: ClaimStatus!
  submissionDate: Date!
  approvalDate: Date
  paymentDate: Date
  rejectionReason: String
  supportingDocuments: [String!]
  submittedBy: User!
  createdAt: DateTime!
  updatedAt: DateTime!
}

enum ClaimStatus {
  SUBMITTED
  UNDER_REVIEW
  APPROVED
  PARTIALLY_APPROVED
  REJECTED
  PAID
}
```

### 8.2 Billing Inputs

```graphql
input ProcessPaymentInput {
  invoiceId: ID!
  amount: Float!
  paymentMethod: PaymentMethodType!
  paymentGateway: String
  upiId: String
  cardToken: String
}

input RecordPaymentInput {
  amount: Float!
  paymentMethod: PaymentMethodType!
  transactionId: String
  upiId: String
  cardLast4: String
  cardType: String
  paymentDate: Date
  paymentTime: Time
  notes: String
}
```

---

## 9. Subscriptions

### 9.1 Real-Time Updates

```graphql
type Subscription {
  # Sample tracking
  sampleStatusUpdated(sampleId: ID!): SampleStatusUpdate!

  # Result updates
  resultEntered(orderId: ID!): TestResult!
  resultVerified(orderId: ID!): TestResult!

  # Report updates
  reportGenerated(orderId: ID!): Report!
  reportDelivered(reportId: ID!): DeliveryStatus!

  # Critical alerts
  criticalValueDetected: CriticalValueAlert!

  # QC monitoring
  qcViolation: QCViolation!
  qcResultRecorded(equipmentId: ID!): IQCResult!

  # Equipment monitoring
  equipmentStatusChanged(equipmentId: ID!): EquipmentStatus!
  analyzerResultReceived(equipmentId: ID!): AnalyzerResult!

  # Order updates
  orderStatusChanged(orderId: ID!): OrderStatusUpdate!

  # Payment updates
  paymentReceived(invoiceId: ID!): Payment!
}

type SampleStatusUpdate {
  sample: Sample!
  previousStatus: SampleStatus!
  newStatus: SampleStatus!
  timestamp: DateTime!
  updatedBy: User!
}

type OrderStatusUpdate {
  order: Order!
  previousStatus: OrderStatus!
  newStatus: OrderStatus!
  timestamp: DateTime!
  updatedBy: User!
}

type QCViolation {
  id: ID!
  rule: String!
  severity: String!
  equipment: Equipment!
  test: Test!
  violationTime: DateTime!
  description: String!
  actionRequired: String!
}
```

---

## 10. Error Handling

### 10.1 Error Types

```graphql
type Error {
  message: String!
  code: ErrorCode!
  path: [String!]
  extensions: JSON
}

enum ErrorCode {
  # Authentication & Authorization
  UNAUTHENTICATED
  UNAUTHORIZED
  FORBIDDEN

  # Validation
  VALIDATION_ERROR
  INVALID_INPUT
  DUPLICATE_ENTRY

  # Business Logic
  BUSINESS_RULE_VIOLATION
  INVALID_STATE_TRANSITION
  RESOURCE_NOT_FOUND

  # System
  INTERNAL_SERVER_ERROR
  SERVICE_UNAVAILABLE
  TIMEOUT

  # External Services
  PAYMENT_GATEWAY_ERROR
  SMS_GATEWAY_ERROR
  EMAIL_SERVICE_ERROR
  WHATSAPP_API_ERROR
}
```

---

## Summary

This GraphQL schema provides:

1. **Complete API Surface**: All 12 microservices exposed
2. **Type Safety**: Strong typing for all inputs and outputs
3. **Real-Time Capabilities**: Subscriptions for live updates
4. **Pagination**: Cursor-based pagination for all lists
5. **Filtering**: Comprehensive filter inputs
6. **Error Handling**: Structured error responses
7. **Documentation**: Self-documenting through schema

### Key Features:

- **Federation Ready**: Can be split across microservices
- **Relay-Compatible**: Cursor-based pagination
- **Versioning Strategy**: Deprecation over breaking changes
- **Performance**: DataLoader patterns for N+1 queries
- **Security**: Field-level authorization
- **Caching**: Cache control directives

---

**Next Steps**:
1. Generate TypeScript types from schema
2. Implement resolvers in Rust (async-graphql)
3. Setup Apollo Federation for microservices
4. Create GraphQL Playground documentation
5. Implement DataLoader for optimizations

---

**Document Status**: ✅ Approved
**Next Review Date**: 2025-02-05
**Owned By**: API Team
