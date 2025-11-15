# Entity-Relationship Diagrams
## Cloud-Native LIS/LIMS Database Schema

**Version**: 1.0.0
**Last Updated**: 2024-11-05
**Database**: PostgreSQL 16+ (Primary), MongoDB 7+ (Secondary)

---

## Table of Contents

1. [Overview](#overview)
2. [Database Strategy](#database-strategy)
3. [Core Domain ER Diagrams](#core-domain-er-diagrams)
4. [Data Relationships](#data-relationships)
5. [Indexing Strategy](#indexing-strategy)
6. [Partitioning Strategy](#partitioning-strategy)
7. [Data Retention](#data-retention)

---

## 1. Overview

### 1.1 Database Distribution

| Domain | Database | Rationale |
|--------|----------|-----------|
| **Patients** | PostgreSQL | ACID transactions, relational integrity |
| **Samples** | PostgreSQL | Strong consistency, referential integrity |
| **Orders** | PostgreSQL | Transactional data, complex queries |
| **Results** | PostgreSQL | Critical data, ACID compliance |
| **Billing** | PostgreSQL | Financial data, audit requirements |
| **Equipment** | PostgreSQL | Structured data, relationships |
| **Quality Control** | PostgreSQL | Statistical analysis, trends |
| **Inventory** | PostgreSQL | Stock management, transactions |
| **Reports** | MongoDB | Flexible templates, document storage |
| **Configuration** | MongoDB | Dynamic schemas, versioning |
| **Compliance Documents** | MongoDB | Unstructured documents |
| **Audit Logs** | MongoDB + Blockchain | Immutable logs, append-only |

---

## 2. Database Strategy

### 2.1 ACID Compliance

All PostgreSQL tables follow ACID principles:
- **Atomicity**: Transactions complete fully or not at all
- **Consistency**: Data integrity constraints enforced
- **Isolation**: Concurrent transactions don't interfere
- **Durability**: Committed data persists

### 2.2 Normalization

- **Core tables**: 3NF (Third Normal Form)
- **Performance-critical**: Denormalized for read optimization
- **Audit tables**: Append-only, no updates/deletes

---

## 3. Core Domain ER Diagrams

### 3.1 Patient Domain

```mermaid
erDiagram
    PATIENT ||--o{ PATIENT_ADDRESS : has
    PATIENT ||--o{ PATIENT_CONTACT : has
    PATIENT ||--o{ PATIENT_CONSENT : has
    PATIENT ||--o{ PATIENT_MEDICAL_HISTORY : has
    PATIENT ||--o{ PATIENT_INSURANCE : has
    PATIENT ||--|| ORGANIZATION : "belongs_to"

    PATIENT {
        uuid id PK
        string mrn_number UK "Medical Record Number"
        string salutation
        string first_name
        string middle_name
        string last_name
        date date_of_birth
        enum gender "MALE|FEMALE|OTHER"
        string aadhaar_number UK "Encrypted"
        string email UK
        string mobile_number UK
        enum blood_group
        enum marital_status
        string nationality
        string occupation
        uuid organization_id FK
        jsonb custom_fields
        timestamp created_at
        timestamp updated_at
        boolean is_active
        uuid created_by
        uuid updated_by
    }

    PATIENT_ADDRESS {
        uuid id PK
        uuid patient_id FK
        enum address_type "HOME|WORK|BILLING"
        string address_line1
        string address_line2
        string city
        string state
        string country
        string postal_code
        point location "Lat/Long for geospatial queries"
        boolean is_primary
        timestamp created_at
    }

    PATIENT_CONTACT {
        uuid id PK
        uuid patient_id FK
        enum contact_type "SELF|PARENT|GUARDIAN|SPOUSE|EMERGENCY"
        string name
        string relationship
        string mobile_number
        string email
        boolean is_primary
        timestamp created_at
    }

    PATIENT_CONSENT {
        uuid id PK
        uuid patient_id FK
        enum consent_type "DATA_PROCESSING|MARKETING|RESEARCH|ABDM"
        boolean granted
        text purpose
        timestamp granted_at
        timestamp expires_at
        string ip_address
        jsonb metadata
        timestamp revoked_at
        string revocation_reason
    }

    PATIENT_MEDICAL_HISTORY {
        uuid id PK
        uuid patient_id FK
        enum category "ALLERGY|MEDICATION|DIAGNOSIS|SURGERY|FAMILY_HISTORY"
        string title
        text description
        date diagnosed_date
        jsonb details
        timestamp created_at
        uuid created_by
    }

    PATIENT_INSURANCE {
        uuid id PK
        uuid patient_id FK
        string insurance_provider
        string policy_number UK
        string policy_holder_name
        date valid_from
        date valid_until
        decimal coverage_amount
        string tpa_name
        string tpa_id
        jsonb additional_details
        boolean is_active
    }

    ORGANIZATION {
        uuid id PK
        string code UK
        string name
        enum org_type "LAB|HOSPITAL|CLINIC|DIAGNOSTIC_CENTER"
        uuid parent_org_id FK
        jsonb settings
    }
```

### 3.2 Sample Domain

```mermaid
erDiagram
    SAMPLE ||--|| PATIENT : "belongs_to"
    SAMPLE ||--|| ORDERS : "for_order"
    SAMPLE ||--o{ SAMPLE_TRACKING : has
    SAMPLE ||--o{ SAMPLE_ALIQUOT : has
    SAMPLE ||--|| SAMPLE_TYPE : of_type
    SAMPLE ||--o{ SAMPLE_REJECTION : may_have

    SAMPLE {
        uuid id PK
        string sample_number UK "Barcode/RFID"
        uuid patient_id FK
        uuid order_id FK
        uuid sample_type_id FK
        uuid collected_by FK "User ID"
        timestamp collected_at
        uuid collection_location_id FK
        enum collection_method "VENIPUNCTURE|CAPILLARY|MIDSTREAM|SWAB"
        decimal volume_ml
        string container_type
        jsonb collection_details
        enum status "COLLECTED|RECEIVED|PROCESSING|COMPLETED|REJECTED|DISPOSED"
        uuid current_location_id FK
        timestamp received_at
        uuid received_by FK
        integer priority "1=STAT, 2=Urgent, 3=Routine"
        jsonb storage_conditions
        timestamp created_at
        timestamp updated_at
        string chain_of_custody_hash "Blockchain hash"
    }

    SAMPLE_TYPE {
        uuid id PK
        string code UK
        string name
        string description
        string container_type
        string container_color
        decimal min_volume_ml
        decimal ideal_volume_ml
        string storage_temperature
        integer stability_hours
        string transport_medium
        jsonb collection_instructions
        boolean is_active
    }

    SAMPLE_TRACKING {
        uuid id PK
        uuid sample_id FK
        enum status "COLLECTED|IN_TRANSIT|RECEIVED|PROCESSING|STORED|DISPOSED"
        uuid location_id FK
        string location_name
        uuid handled_by FK
        timestamp timestamp
        jsonb metadata
        point gps_location
    }

    SAMPLE_ALIQUOT {
        uuid id PK
        uuid parent_sample_id FK
        string aliquot_number UK
        decimal volume_ml
        uuid created_by FK
        timestamp created_at
        uuid current_location_id FK
        enum status "AVAILABLE|IN_USE|CONSUMED|DISPOSED"
    }

    SAMPLE_REJECTION {
        uuid id PK
        uuid sample_id FK
        enum rejection_reason "HEMOLYSIS|INSUFFICIENT_VOLUME|CLOTTED|LIPEMIC|UNLABELED|DAMAGED"
        text rejection_notes
        uuid rejected_by FK
        timestamp rejected_at
        boolean recollection_required
        uuid new_sample_id FK "If recollected"
    }

    ORDERS {
        uuid id PK
    }

    PATIENT {
        uuid id PK
    }
```

### 3.3 Order Domain

```mermaid
erDiagram
    ORDERS ||--|| PATIENT : for
    ORDERS ||--o{ ORDER_TEST : contains
    ORDER_TEST ||--|| TEST : references
    ORDERS ||--o{ ORDER_STATUS_HISTORY : has
    TEST ||--|| TEST_CATEGORY : belongs_to
    TEST ||--o{ TEST_PROFILE : may_be_in
    TEST_PROFILE ||--o{ TEST_PROFILE_TESTS : contains
    TEST ||--o{ TEST_PRICE : has
    TEST ||--|| DEPARTMENT : belongs_to

    ORDERS {
        uuid id PK
        string order_number UK
        uuid patient_id FK
        uuid organization_id FK
        uuid referring_doctor_id FK
        enum order_type "REGULAR|STAT|ROUTINE|PROFILE"
        enum order_source "WALK_IN|ONLINE|REFERRAL|CORPORATE|CAMP"
        uuid ordered_by FK "User ID"
        timestamp ordered_at
        timestamp appointment_at
        enum status "PENDING|CONFIRMED|SAMPLE_COLLECTED|PROCESSING|COMPLETED|CANCELLED"
        enum payment_status "PENDING|PARTIAL|PAID|REFUNDED"
        decimal total_amount
        decimal discount_amount
        decimal tax_amount
        decimal net_amount
        string clinical_notes
        jsonb patient_condition
        integer priority
        boolean is_fasting_required
        string special_instructions
        uuid cancelled_by FK
        timestamp cancelled_at
        string cancellation_reason
        timestamp created_at
        timestamp updated_at
    }

    ORDER_TEST {
        uuid id PK
        uuid order_id FK
        uuid test_id FK
        uuid sample_id FK "May be null until collected"
        enum status "PENDING|SAMPLE_COLLECTED|PROCESSING|COMPLETED|CANCELLED"
        decimal test_price
        decimal discount_amount
        string clinical_indication
        timestamp tat_due_at "TAT deadline"
        timestamp completed_at
        uuid performed_by FK
        jsonb test_parameters
    }

    TEST {
        uuid id PK
        string code UK
        string name
        string short_name
        uuid department_id FK
        uuid category_id FK
        string description
        string methodology
        string specimen_type
        decimal standard_tat_hours
        decimal urgent_tat_hours
        decimal stat_tat_hours
        string prerequisites
        string patient_preparation
        enum result_type "NUMERIC|TEXT|OPTION|IMAGE|CULTURE"
        jsonb normal_ranges "Age/Gender specific"
        string units
        integer decimal_places
        boolean requires_approval
        boolean critical_value_enabled
        decimal critical_low
        decimal critical_high
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }

    TEST_CATEGORY {
        uuid id PK
        string code UK
        string name
        uuid parent_category_id FK
        string description
        boolean is_active
    }

    TEST_PROFILE {
        uuid id PK
        string code UK
        string name
        string description
        decimal price
        boolean is_active
    }

    TEST_PROFILE_TESTS {
        uuid id PK
        uuid profile_id FK
        uuid test_id FK
        integer display_order
    }

    TEST_PRICE {
        uuid id PK
        uuid test_id FK
        uuid organization_id FK
        enum price_type "REGULAR|CORPORATE|INSURANCE|CAMP"
        string customer_category
        decimal price
        date valid_from
        date valid_until
        boolean is_active
    }

    DEPARTMENT {
        uuid id PK
        string code UK
        string name
        string description
        uuid head_of_department FK
        boolean is_active
    }
```

### 3.4 Result Domain

```mermaid
erDiagram
    TEST_RESULT ||--|| ORDER_TEST : for
    TEST_RESULT ||--|| TEST : of_type
    TEST_RESULT ||--o{ RESULT_COMPONENT : has
    TEST_RESULT ||--o{ RESULT_AMENDMENT : may_have
    TEST_RESULT ||--o{ RESULT_VERIFICATION : has
    TEST_RESULT ||--o{ CRITICAL_VALUE_ALERT : may_have
    TEST_RESULT ||--|| EQUIPMENT : may_be_from

    TEST_RESULT {
        uuid id PK
        uuid order_test_id FK
        uuid test_id FK
        uuid sample_id FK
        enum status "PENDING|ENTERED|VERIFIED|APPROVED|RELEASED|AMENDED"
        timestamp result_date
        uuid entered_by FK
        timestamp entered_at
        uuid equipment_id FK "If analyzer result"
        string analyzer_run_id
        enum entry_method "MANUAL|ANALYZER|MIDDLEWARE|IMPORTED"
        jsonb raw_result_data "From analyzer"
        boolean auto_verified
        decimal auto_verification_confidence
        string auto_verification_model_version
        jsonb delta_check_results
        boolean has_critical_values
        string technician_comment
        timestamp created_at
        timestamp updated_at
    }

    RESULT_COMPONENT {
        uuid id PK
        uuid test_result_id FK
        string component_code
        string component_name
        decimal numeric_value
        string text_value
        string option_value
        string units
        decimal reference_low
        decimal reference_high
        enum flag "NORMAL|LOW|HIGH|CRITICAL_LOW|CRITICAL_HIGH|ABNORMAL"
        jsonb interpretation
        integer display_order
    }

    RESULT_AMENDMENT {
        uuid id PK
        uuid test_result_id FK
        integer amendment_number
        jsonb previous_values
        jsonb new_values
        string reason
        string detailed_explanation
        uuid amended_by FK
        timestamp amended_at
        uuid approved_by FK
        timestamp approved_at
    }

    RESULT_VERIFICATION {
        uuid id PK
        uuid test_result_id FK
        enum verification_level "TECHNICAL|PATHOLOGIST|DIRECTOR"
        uuid verified_by FK
        timestamp verified_at
        enum action "APPROVED|REJECTED|PENDING_REVIEW"
        string comments
        jsonb verification_rules_applied
    }

    CRITICAL_VALUE_ALERT {
        uuid id PK
        uuid test_result_id FK
        uuid component_id FK
        decimal value
        string component_name
        enum severity "HIGH|CRITICAL"
        timestamp detected_at
        uuid notified_to FK
        timestamp notified_at
        enum notification_method "PHONE|SMS|WHATSAPP|EMAIL"
        string acknowledgment_by
        timestamp acknowledged_at
        string action_taken
    }

    ORDER_TEST {
        uuid id PK
    }

    TEST {
        uuid id PK
    }

    EQUIPMENT {
        uuid id PK
    }
```

### 3.5 Equipment Integration Domain

```mermaid
erDiagram
    EQUIPMENT ||--|| EQUIPMENT_TYPE : of_type
    EQUIPMENT ||--o{ EQUIPMENT_QC : has
    EQUIPMENT ||--o{ EQUIPMENT_CALIBRATION : has
    EQUIPMENT ||--o{ EQUIPMENT_MAINTENANCE : has
    EQUIPMENT ||--o{ EQUIPMENT_TEST_MAPPING : supports
    EQUIPMENT_TEST_MAPPING ||--|| TEST : for
    EQUIPMENT ||--o{ ANALYZER_RESULT_QUEUE : receives

    EQUIPMENT {
        uuid id PK
        string serial_number UK
        string name
        uuid equipment_type_id FK
        string manufacturer
        string model
        string software_version
        string firmware_version
        uuid location_id FK
        enum status "ONLINE|OFFLINE|MAINTENANCE|CALIBRATION|QC_FAILED"
        enum communication_protocol "HL7|ASTM|FILE|API"
        string host_address
        integer port
        jsonb connection_settings
        timestamp last_online_at
        timestamp installation_date
        date warranty_expiry_date
        string service_contract_number
        uuid managed_by FK
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }

    EQUIPMENT_TYPE {
        uuid id PK
        string code UK
        string name
        string category "CHEMISTRY|HEMATOLOGY|IMMUNOLOGY|MICROBIOLOGY"
        string manufacturer
        jsonb supported_protocols
        jsonb default_settings
    }

    EQUIPMENT_QC {
        uuid id PK
        uuid equipment_id FK
        string lot_number
        string level "LEVEL_1|LEVEL_2|LEVEL_3"
        date qc_date
        time qc_time
        uuid test_id FK
        decimal target_value
        decimal obtained_value
        decimal standard_deviation
        decimal cv_percentage
        string westgard_rules_applied
        boolean passed
        string failure_reason
        uuid performed_by FK
        uuid reviewed_by FK
        jsonb qc_rules_results
        timestamp created_at
    }

    EQUIPMENT_CALIBRATION {
        uuid id PK
        uuid equipment_id FK
        uuid test_id FK
        date calibration_date
        time calibration_time
        string calibrator_lot
        date calibrator_expiry
        decimal calibration_factor
        jsonb calibration_curve
        enum status "PASS|FAIL"
        string failure_reason
        uuid performed_by FK
        uuid approved_by FK
        date valid_until
        timestamp created_at
    }

    EQUIPMENT_MAINTENANCE {
        uuid id PK
        uuid equipment_id FK
        enum maintenance_type "PREVENTIVE|CORRECTIVE|CALIBRATION|VALIDATION"
        date scheduled_date
        date completed_date
        string description
        text work_performed
        decimal cost
        string vendor_name
        string engineer_name
        jsonb parts_replaced
        enum status "SCHEDULED|IN_PROGRESS|COMPLETED|CANCELLED"
        uuid scheduled_by FK
        uuid performed_by FK
        timestamp created_at
    }

    EQUIPMENT_TEST_MAPPING {
        uuid id PK
        uuid equipment_id FK
        uuid test_id FK
        string equipment_test_code "Analyzer's test code"
        jsonb result_mapping "Map analyzer fields to LIS"
        boolean is_active
    }

    ANALYZER_RESULT_QUEUE {
        uuid id PK
        uuid equipment_id FK
        string sample_number
        uuid order_test_id FK
        enum status "PENDING|PROCESSING|COMPLETED|ERROR"
        jsonb raw_data "HL7/ASTM message"
        jsonb parsed_results
        timestamp received_at
        timestamp processed_at
        text error_message
    }

    TEST {
        uuid id PK
    }
```

### 3.6 Quality Control Domain

```mermaid
erDiagram
    QC_LOT ||--o{ IQC_RESULT : has
    IQC_RESULT ||--|| TEST : for
    IQC_RESULT ||--|| EQUIPMENT : from
    EQC_PROGRAM ||--o{ EQC_SAMPLE : has
    EQC_SAMPLE ||--o{ EQC_RESULT : has
    QC_RULE ||--o{ QC_RULE_VIOLATION : triggers

    QC_LOT {
        uuid id PK
        string lot_number UK
        string manufacturer
        string product_name
        string level "LEVEL_1|LEVEL_2|LEVEL_3"
        date received_date
        date opened_date
        date expiry_date
        jsonb target_values "Test-specific targets"
        jsonb acceptable_ranges
        uuid opened_by FK
        boolean is_active
        timestamp created_at
    }

    IQC_RESULT {
        uuid id PK
        uuid qc_lot_id FK
        uuid test_id FK
        uuid equipment_id FK
        date qc_date
        time qc_time
        shift_code shift "MORNING|EVENING|NIGHT"
        decimal measured_value
        decimal target_value
        decimal sd
        decimal cv_percent
        boolean in_control
        uuid performed_by FK
        uuid reviewed_by FK
        timestamp created_at
    }

    QC_RULE {
        uuid id PK
        string rule_code "1_2s|1_3s|2_2s|R_4s|4_1s|10_x"
        string name
        string description
        jsonb rule_logic
        enum severity "WARNING|REJECT"
        boolean is_active
    }

    QC_RULE_VIOLATION {
        uuid id PK
        uuid iqc_result_id FK
        uuid qc_rule_id FK
        string rule_code
        string violation_description
        enum action_taken "CONTINUE|STOP|RECALIBRATE|MAINTENANCE"
        text corrective_action
        uuid reviewed_by FK
        timestamp reviewed_at
        timestamp created_at
    }

    EQC_PROGRAM {
        uuid id PK
        string program_code UK
        string provider_name
        string program_name
        enum frequency "MONTHLY|QUARTERLY|ANNUAL"
        date enrollment_date
        boolean is_active
    }

    EQC_SAMPLE {
        uuid id PK
        uuid eqc_program_id FK
        string sample_code UK
        date dispatch_date
        date due_date
        enum status "PENDING|IN_PROGRESS|SUBMITTED|GRADED"
        timestamp created_at
    }

    EQC_RESULT {
        uuid id PK
        uuid eqc_sample_id FK
        uuid test_id FK
        decimal reported_value
        decimal peer_mean
        decimal peer_sd
        decimal z_score
        enum performance "ACCEPTABLE|UNACCEPTABLE"
        string grade
        uuid submitted_by FK
        timestamp submitted_at
        timestamp graded_at
    }

    TEST {
        uuid id PK
    }

    EQUIPMENT {
        uuid id PK
    }
```

### 3.7 Billing Domain

```mermaid
erDiagram
    INVOICE ||--|| PATIENT : for
    INVOICE ||--|| ORDERS : for_order
    INVOICE ||--o{ INVOICE_LINE_ITEM : contains
    INVOICE ||--o{ PAYMENT : receives
    INVOICE ||--o{ INSURANCE_CLAIM : may_have
    PAYMENT ||--|| PAYMENT_METHOD : uses

    INVOICE {
        uuid id PK
        string invoice_number UK
        uuid order_id FK
        uuid patient_id FK
        uuid organization_id FK
        date invoice_date
        date due_date
        decimal subtotal
        decimal discount_amount
        string discount_reason
        decimal tax_amount
        decimal cgst_amount
        decimal sgst_amount
        decimal igst_amount
        decimal total_amount
        decimal paid_amount
        decimal balance_amount
        enum status "DRAFT|ISSUED|PARTIAL|PAID|OVERDUE|CANCELLED"
        string gstin "GST Number"
        string einvoice_irn "E-Invoice IRN"
        timestamp einvoice_generated_at
        jsonb einvoice_data
        string billing_address
        string shipping_address
        uuid created_by FK
        timestamp created_at
        timestamp updated_at
    }

    INVOICE_LINE_ITEM {
        uuid id PK
        uuid invoice_id FK
        uuid test_id FK "May be null for misc items"
        string item_code
        string description
        integer quantity
        decimal unit_price
        decimal discount_percent
        decimal discount_amount
        decimal tax_percent
        decimal tax_amount
        decimal line_total
        string hsn_code "For GST"
    }

    PAYMENT {
        uuid id PK
        string payment_number UK
        uuid invoice_id FK
        uuid patient_id FK
        decimal amount
        enum payment_method "CASH|UPI|CARD|NET_BANKING|WALLET|BNPL|CHEQUE"
        string payment_gateway "RAZORPAY|STRIPE|PAYU"
        string transaction_id "Gateway transaction ID"
        string upi_id
        string card_last4
        string card_type "VISA|MASTERCARD|RUPAY|AMEX"
        enum status "PENDING|SUCCESS|FAILED|REFUNDED"
        date payment_date
        time payment_time
        uuid collected_by FK
        jsonb payment_metadata
        string failure_reason
        timestamp created_at
    }

    PAYMENT_METHOD {
        uuid id PK
        string code UK
        string name
        boolean is_online
        string provider
        jsonb configuration
        boolean is_active
    }

    INSURANCE_CLAIM {
        uuid id PK
        string claim_number UK
        uuid invoice_id FK
        uuid patient_id FK
        uuid insurance_provider_id FK
        string policy_number
        decimal claim_amount
        decimal approved_amount
        decimal paid_amount
        enum status "SUBMITTED|UNDER_REVIEW|APPROVED|PARTIALLY_APPROVED|REJECTED|PAID"
        date submission_date
        date approval_date
        date payment_date
        text rejection_reason
        jsonb supporting_documents
        uuid submitted_by FK
        timestamp created_at
        timestamp updated_at
    }

    PATIENT {
        uuid id PK
    }

    ORDERS {
        uuid id PK
    }
```

### 3.8 Inventory Domain

```mermaid
erDiagram
    INVENTORY_ITEM ||--|| ITEM_CATEGORY : belongs_to
    INVENTORY_ITEM ||--o{ STOCK_TRANSACTION : has
    INVENTORY_ITEM ||--o{ STOCK_LEVEL : has
    INVENTORY_ITEM ||--|| VENDOR : supplied_by
    STOCK_TRANSACTION ||--|| PURCHASE_ORDER : may_belong_to
    PURCHASE_ORDER ||--|| VENDOR : from

    INVENTORY_ITEM {
        uuid id PK
        string item_code UK
        string name
        string description
        uuid category_id FK
        uuid vendor_id FK
        string manufacturer
        enum item_type "REAGENT|CONSUMABLE|EQUIPMENT|CONTROL|CALIBRATOR"
        string unit_of_measure "ML|TESTS|PIECES|VIALS"
        decimal unit_price
        integer reorder_level
        integer reorder_quantity
        string storage_conditions
        boolean requires_cold_chain
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }

    ITEM_CATEGORY {
        uuid id PK
        string code UK
        string name
        uuid parent_category_id FK
        string description
    }

    STOCK_TRANSACTION {
        uuid id PK
        uuid item_id FK
        enum transaction_type "RECEIPT|CONSUMPTION|ADJUSTMENT|RETURN|DISPOSAL"
        decimal quantity
        string unit_of_measure
        string batch_number
        date expiry_date
        uuid location_id FK
        uuid purchase_order_id FK "If receipt"
        uuid consumed_by_test FK "If consumption"
        uuid performed_by FK
        timestamp transaction_date
        string reference_number
        text notes
        timestamp created_at
    }

    STOCK_LEVEL {
        uuid id PK
        uuid item_id FK
        uuid location_id FK
        string batch_number
        date expiry_date
        decimal available_quantity
        decimal reserved_quantity
        decimal allocated_quantity
        timestamp last_updated
    }

    PURCHASE_ORDER {
        uuid id PK
        string po_number UK
        uuid vendor_id FK
        date order_date
        date expected_delivery_date
        date actual_delivery_date
        enum status "DRAFT|SUBMITTED|APPROVED|RECEIVED|PARTIALLY_RECEIVED|CLOSED|CANCELLED"
        decimal total_amount
        uuid created_by FK
        uuid approved_by FK
        timestamp created_at
        timestamp updated_at
    }

    VENDOR {
        uuid id PK
        string code UK
        string name
        string contact_person
        string email
        string phone
        string address
        string gstin
        enum vendor_type "MANUFACTURER|DISTRIBUTOR|SERVICE_PROVIDER"
        enum payment_terms "IMMEDIATE|NET_30|NET_60|NET_90"
        boolean is_active
    }
```

### 3.9 Compliance & Audit Domain

```mermaid
erDiagram
    AUDIT_LOG {
        uuid id PK
        timestamp timestamp
        uuid user_id FK
        string username
        string ip_address
        string user_agent
        enum action "CREATE|READ|UPDATE|DELETE|LOGIN|LOGOUT|EXPORT"
        string entity_type "Patient|Sample|Result|etc"
        uuid entity_id
        jsonb before_state
        jsonb after_state
        string reason "For amendments, deletions"
        string session_id
        jsonb metadata
    }

    DOCUMENT {
        uuid id PK
        string document_number UK
        string title
        enum document_type "SOP|POLICY|FORM|MANUAL|CERTIFICATE|REPORT"
        string version
        enum status "DRAFT|REVIEW|APPROVED|OBSOLETE"
        uuid author_id FK
        uuid reviewer_id FK
        uuid approver_id FK
        date effective_date
        date review_due_date
        date obsolete_date
        string file_path
        string file_hash
        jsonb change_history
        timestamp created_at
        timestamp updated_at
    }

    CAPA {
        uuid id PK
        string capa_number UK
        enum type "CORRECTIVE|PREVENTIVE"
        enum source "INTERNAL_AUDIT|EXTERNAL_AUDIT|INCIDENT|COMPLAINT|QC_FAILURE"
        string title
        text problem_description
        text root_cause_analysis
        text action_plan
        date identified_date
        date target_completion_date
        date actual_completion_date
        uuid initiated_by FK
        uuid assigned_to FK
        enum status "OPEN|IN_PROGRESS|COMPLETED|VERIFIED|CLOSED"
        uuid verified_by FK
        text verification_comments
        timestamp created_at
        timestamp updated_at
    }

    TRAINING_RECORD {
        uuid id PK
        uuid user_id FK
        string training_topic
        string trainer_name
        date training_date
        decimal duration_hours
        enum training_type "INDUCTION|REFRESH|ON_JOB|EXTERNAL|CERTIFICATION"
        enum status "SCHEDULED|COMPLETED|CANCELLED"
        decimal score
        boolean passed
        date valid_until
        string certificate_number
        uuid conducted_by FK
        timestamp created_at
    }
```

### 3.10 User & Security Domain

```mermaid
erDiagram
    USER ||--o{ USER_ROLE : has
    USER_ROLE ||--|| ROLE : references
    ROLE ||--o{ ROLE_PERMISSION : has
    ROLE_PERMISSION ||--|| PERMISSION : references
    USER ||--o{ USER_SESSION : has
    USER ||--o{ USER_MFA_DEVICE : has

    USER {
        uuid id PK
        string username UK
        string email UK
        string password_hash
        string first_name
        string last_name
        string mobile_number
        uuid organization_id FK
        uuid department_id FK
        string designation
        string signature_path
        string license_number "For pathologists"
        boolean is_active
        boolean is_verified
        boolean require_password_change
        timestamp last_login_at
        string last_login_ip
        integer failed_login_attempts
        timestamp locked_until
        timestamp created_at
        timestamp updated_at
    }

    ROLE {
        uuid id PK
        string code UK
        string name
        string description
        integer hierarchy_level
        boolean is_system_role
        boolean is_active
    }

    USER_ROLE {
        uuid id PK
        uuid user_id FK
        uuid role_id FK
        uuid organization_id FK
        date valid_from
        date valid_until
        uuid assigned_by FK
        timestamp created_at
    }

    PERMISSION {
        uuid id PK
        string code UK "e.g., patient.create, result.verify"
        string resource
        string action
        string description
    }

    ROLE_PERMISSION {
        uuid id PK
        uuid role_id FK
        uuid permission_id FK
    }

    USER_SESSION {
        uuid id PK
        uuid user_id FK
        string session_token UK
        string refresh_token
        string ip_address
        string user_agent
        timestamp created_at
        timestamp expires_at
        timestamp last_active_at
        boolean is_active
    }

    USER_MFA_DEVICE {
        uuid id PK
        uuid user_id FK
        enum device_type "TOTP|SMS|EMAIL"
        string device_identifier
        string secret_key "Encrypted"
        boolean is_verified
        boolean is_primary
        timestamp verified_at
        timestamp created_at
    }
```

---

## 4. Data Relationships

### 4.1 Key Relationships

| From | To | Relationship | Cardinality |
|------|-----|--------------|-------------|
| PATIENT | ORDERS | Places orders | 1:N |
| ORDERS | ORDER_TEST | Contains tests | 1:N |
| ORDERS | SAMPLE | Requires samples | 1:N |
| SAMPLE | TEST_RESULT | Produces results | 1:N |
| TEST_RESULT | REPORT | Generates reports | N:1 |
| ORDER | INVOICE | Billed via | 1:1 |
| INVOICE | PAYMENT | Receives payments | 1:N |
| EQUIPMENT | TEST_RESULT | Produces results | 1:N |
| TEST | ORDER_TEST | Ordered as | N:N |

### 4.2 Referential Integrity

**Cascade Delete Rules:**
- `PATIENT` deletion → Soft delete (mark inactive)
- `SAMPLE` deletion → Not allowed (audit requirement)
- `TEST_RESULT` deletion → Not allowed (regulatory)
- `ORDER` cancellation → Mark as cancelled, preserve data

**Orphan Prevention:**
- Foreign keys with `ON DELETE RESTRICT`
- Application-level validation
- Database triggers for complex rules

---

## 5. Indexing Strategy

### 5.1 Primary Indexes

Every table has:
- **Primary Key**: UUID (B-tree index)
- **Unique Constraints**: Natural keys (e.g., MRN, sample number)

### 5.2 Secondary Indexes

#### Patient Indexes
```sql
CREATE INDEX idx_patient_mrn ON patient(mrn_number);
CREATE INDEX idx_patient_mobile ON patient(mobile_number);
CREATE INDEX idx_patient_email ON patient(email);
CREATE INDEX idx_patient_aadhaar ON patient(aadhaar_number); -- Encrypted
CREATE INDEX idx_patient_org ON patient(organization_id);
CREATE INDEX idx_patient_search ON patient USING gin(to_tsvector('english', first_name || ' ' || last_name));
```

#### Sample Indexes
```sql
CREATE INDEX idx_sample_number ON sample(sample_number);
CREATE INDEX idx_sample_patient ON sample(patient_id);
CREATE INDEX idx_sample_order ON sample(order_id);
CREATE INDEX idx_sample_status ON sample(status);
CREATE INDEX idx_sample_collected_at ON sample(collected_at);
CREATE INDEX idx_sample_composite ON sample(patient_id, collected_at DESC);
```

#### Result Indexes
```sql
CREATE INDEX idx_result_order_test ON test_result(order_test_id);
CREATE INDEX idx_result_sample ON test_result(sample_id);
CREATE INDEX idx_result_status ON test_result(status);
CREATE INDEX idx_result_date ON test_result(result_date);
CREATE INDEX idx_result_critical ON test_result(has_critical_values) WHERE has_critical_values = true;
```

#### Audit Log Indexes
```sql
CREATE INDEX idx_audit_user ON audit_log(user_id);
CREATE INDEX idx_audit_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_timestamp ON audit_log(timestamp DESC);
CREATE INDEX idx_audit_action ON audit_log(action);
```

### 5.3 Partial Indexes

For performance optimization:
```sql
-- Only index active patients
CREATE INDEX idx_active_patients ON patient(id) WHERE is_active = true;

-- Only index pending results
CREATE INDEX idx_pending_results ON test_result(id) WHERE status = 'PENDING';

-- Only index critical values
CREATE INDEX idx_critical_alerts ON critical_value_alert(test_result_id)
WHERE acknowledged_at IS NULL;
```

---

## 6. Partitioning Strategy

### 6.1 Time-Based Partitioning

**Audit Logs** (Monthly Partitions):
```sql
CREATE TABLE audit_log (
    id UUID PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL,
    -- other columns
) PARTITION BY RANGE (timestamp);

CREATE TABLE audit_log_2024_11 PARTITION OF audit_log
FOR VALUES FROM ('2024-11-01') TO ('2024-12-01');
```

**Test Results** (Yearly Partitions):
```sql
CREATE TABLE test_result (
    id UUID PRIMARY KEY,
    result_date TIMESTAMP NOT NULL,
    -- other columns
) PARTITION BY RANGE (result_date);

CREATE TABLE test_result_2024 PARTITION OF test_result
FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');
```

### 6.2 Hash Partitioning

**Large Tables** (e.g., Samples):
```sql
CREATE TABLE sample (
    id UUID PRIMARY KEY,
    -- other columns
) PARTITION BY HASH (id);

CREATE TABLE sample_p0 PARTITION OF sample FOR VALUES WITH (MODULUS 4, REMAINDER 0);
CREATE TABLE sample_p1 PARTITION OF sample FOR VALUES WITH (MODULUS 4, REMAINDER 1);
CREATE TABLE sample_p2 PARTITION OF sample FOR VALUES WITH (MODULUS 4, REMAINDER 2);
CREATE TABLE sample_p3 PARTITION OF sample FOR VALUES WITH (MODULUS 4, REMAINDER 3);
```

---

## 7. Data Retention

### 7.1 Retention Periods (NABL Compliance)

| Data Type | Retention Period | Storage Tier |
|-----------|-----------------|--------------|
| **Patient Demographics** | Indefinite | Hot |
| **Test Results** | 5+ years | Hot → Warm → Cold |
| **Audit Logs** | 7+ years | Hot → Warm → Archive |
| **Quality Control** | 5+ years | Warm |
| **Equipment Logs** | 5+ years | Warm |
| **Financial Records** | 7+ years | Warm → Archive |
| **Compliance Documents** | Indefinite | Archive |
| **Report PDFs** | 5+ years | Cold storage (S3) |

### 7.2 Archival Strategy

**Tiered Storage:**
```
Hot Storage (PostgreSQL) → 0-1 year
Warm Storage (Compressed PostgreSQL) → 1-5 years
Cold Storage (S3 Glacier) → 5+ years
```

**Automated Archival:**
- Monthly job to move old data
- Compressed and encrypted archives
- Indexed metadata for retrieval
- Restore process for legal/audit requests

---

## Summary

This ER diagram documentation provides:
1. **Comprehensive schema** for all 12 core modules
2. **Relational integrity** with proper foreign keys
3. **Indexing strategy** for performance
4. **Partitioning strategy** for scalability
5. **Data retention** policy (NABL compliant)
6. **Audit trail** for regulatory compliance

The schema supports:
- **High performance** through indexing and partitioning
- **Data integrity** through constraints and transactions
- **Regulatory compliance** through audit trails and retention
- **Scalability** through partitioning and archiving
- **Flexibility** through JSON fields where appropriate

---

**Next Steps:**
1. Review with domain experts
2. Create migration scripts
3. Setup development/staging databases
4. Load test with realistic data volumes
5. Document data access patterns for optimization

---

**Document Status**: ✅ Approved
**Next Review Date**: 2025-02-05
**Owned By**: Database Architecture Team
