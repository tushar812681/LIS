# Low-Level Design (LLD)
## Cloud-Native LIS/LIMS - Detailed Component Design

**Version**: 1.0.0
**Last Updated**: 2024-11-05
**Status**: Active

---

## Table of Contents

1. [Introduction](#introduction)
2. [Microservice Detailed Designs](#microservice-detailed-designs)
3. [Algorithms & Data Structures](#algorithms--data-structures)
4. [State Machines](#state-machines)
5. [Caching Strategy](#caching-strategy)
6. [Error Handling](#error-handling)
7. [Performance Optimization](#performance-optimization)
8. [Security Implementation](#security-implementation)

---

## 1. Introduction

This document provides detailed low-level design for all microservices, including algorithms, data structures, sequence diagrams, state machines, and implementation patterns.

### 1.1 Design Goals

- **Performance**: <100ms API response time (P95)
- **Scalability**: Support 10,000+ concurrent users
- **Reliability**: 99.9% uptime
- **Maintainability**: Clean code, SOLID principles
- **Testability**: >80% code coverage

---

## 2. Microservice Detailed Designs

### 2.1 Patient Service

#### 2.1.1 Core Data Structures

```rust
// Domain Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: Uuid,
    pub mrn_number: String,
    pub salutation: Option<Salutation>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Gender,
    pub aadhaar_number: Option<EncryptedString>,
    pub email: Option<Email>,
    pub mobile_number: PhoneNumber,
    pub blood_group: Option<BloodGroup>,
    pub marital_status: Option<MaritalStatus>,
    pub nationality: String,
    pub occupation: Option<String>,
    pub organization_id: Uuid,
    pub custom_fields: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
}

// Value Objects
#[derive(Debug, Clone)]
pub struct EncryptedString(String); // AES-256 encrypted

#[derive(Debug, Clone)]
pub struct Email(String); // Validated email

#[derive(Debug, Clone)]
pub struct PhoneNumber {
    country_code: String,
    number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BloodGroup {
    APositive,
    ANegative,
    BPositive,
    BNegative,
    OPositive,
    ONegative,
    ABPositive,
    ABNegative,
}
```

#### 2.1.2 Repository Pattern

```rust
#[async_trait]
pub trait PatientRepository: Send + Sync {
    async fn create(&self, patient: Patient) -> Result<Patient, Error>;
    async fn update(&self, id: Uuid, patient: Patient) -> Result<Patient, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Patient>, Error>;
    async fn find_by_mrn(&self, mrn: &str) -> Result<Option<Patient>, Error>;
    async fn search(&self, query: PatientSearchQuery) -> Result<Vec<Patient>, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>; // Soft delete
}

// PostgreSQL Implementation
pub struct PostgresPatientRepository {
    pool: PgPool,
    cache: Arc<RedisCache>,
}

impl PostgresPatientRepository {
    pub fn new(pool: PgPool, cache: Arc<RedisCache>) -> Self {
        Self { pool, cache }
    }

    // Cache-aside pattern
    async fn get_with_cache(&self, id: Uuid) -> Result<Option<Patient>, Error> {
        // Try cache first
        if let Some(patient) = self.cache.get::<Patient>(&format!("patient:{}", id)).await? {
            return Ok(Some(patient));
        }

        // Cache miss - fetch from DB
        let patient = sqlx::query_as!(
            Patient,
            r#"
            SELECT * FROM patient WHERE id = $1 AND is_active = true
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        // Update cache
        if let Some(ref p) = patient {
            self.cache.set(&format!("patient:{}", id), p, 3600).await?;
        }

        Ok(patient)
    }
}
```

#### 2.1.3 Service Layer

```rust
pub struct PatientService {
    repository: Arc<dyn PatientRepository>,
    event_publisher: Arc<dyn EventPublisher>,
    encryption_service: Arc<EncryptionService>,
    validator: Arc<PatientValidator>,
}

impl PatientService {
    pub async fn create_patient(
        &self,
        input: CreatePatientInput,
        created_by: Uuid,
    ) -> Result<Patient, ServiceError> {
        // 1. Validate input
        self.validator.validate_create_input(&input)?;

        // 2. Check for duplicates (by mobile or email)
        if self.check_duplicate(&input).await? {
            return Err(ServiceError::DuplicatePatient);
        }

        // 3. Generate MRN
        let mrn = self.generate_mrn().await?;

        // 4. Encrypt sensitive data
        let aadhaar = if let Some(ref a) = input.aadhaar_number {
            Some(self.encryption_service.encrypt(a).await?)
        } else {
            None
        };

        // 5. Create patient entity
        let patient = Patient {
            id: Uuid::new_v4(),
            mrn_number: mrn,
            aadhaar_number: aadhaar,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_active: true,
            ..input.into()
        };

        // 6. Save to database (transaction)
        let saved_patient = self.repository.create(patient.clone()).await?;

        // 7. Publish event
        self.event_publisher
            .publish(Event::PatientCreated {
                patient_id: saved_patient.id,
                organization_id: saved_patient.organization_id,
                timestamp: Utc::now(),
            })
            .await?;

        // 8. Index in Elasticsearch for search
        self.index_for_search(&saved_patient).await?;

        Ok(saved_patient)
    }

    // MRN Generation Algorithm
    async fn generate_mrn(&self) -> Result<String, Error> {
        // Format: ORG-YEAR-SEQUENCE
        // Example: ABC-2024-000123

        let org_code = "ABC"; // From organization
        let year = Utc::now().year();

        // Atomic increment using Redis
        let sequence = self.repository
            .increment_mrn_sequence(org_code, year)
            .await?;

        Ok(format!("{}-{}-{:06}", org_code, year, sequence))
    }
}
```

#### 2.1.4 GraphQL Resolvers

```rust
#[Object]
impl PatientQuery {
    async fn patient(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<Patient>, Error> {
        let service = ctx.data::<Arc<PatientService>>()?;
        let user = ctx.data::<CurrentUser>()?;

        // Authorization check
        if !user.has_permission("patient.read") {
            return Err(Error::Unauthorized);
        }

        service.get_patient(id).await
    }

    async fn search_patients(
        &self,
        ctx: &Context<'_>,
        query: String,
        filters: Option<PatientFilters>,
        pagination: PaginationInput,
    ) -> Result<PatientConnection, Error> {
        let service = ctx.data::<Arc<PatientService>>()?;

        service.search_patients(query, filters, pagination).await
    }
}

#[Object]
impl PatientMutation {
    async fn create_patient(
        &self,
        ctx: &Context<'_>,
        input: CreatePatientInput,
    ) -> Result<Patient, Error> {
        let service = ctx.data::<Arc<PatientService>>()?;
        let user = ctx.data::<CurrentUser>()?;

        // Authorization
        if !user.has_permission("patient.create") {
            return Err(Error::Unauthorized);
        }

        // Audit log
        ctx.data::<Arc<AuditService>>()?
            .log_action(AuditAction::PatientCreate, &input)
            .await?;

        service.create_patient(input, user.id).await
    }
}
```

---

### 2.2 Sample Service

#### 2.2.1 Sample Lifecycle State Machine

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SampleStatus {
    Collected,
    InTransit,
    Received,
    Processing,
    Completed,
    Rejected,
    Disposed,
}

pub struct SampleStateMachine;

impl SampleStateMachine {
    pub fn can_transition(from: SampleStatus, to: SampleStatus) -> bool {
        use SampleStatus::*;

        matches!(
            (from, to),
            (Collected, InTransit)
                | (Collected, Rejected)
                | (InTransit, Received)
                | (InTransit, Rejected)
                | (Received, Processing)
                | (Received, Rejected)
                | (Processing, Completed)
                | (Processing, Rejected)
                | (Completed, Disposed)
                | (Rejected, Disposed)
        )
    }

    pub fn transition(
        sample: &mut Sample,
        to: SampleStatus,
        user_id: Uuid,
    ) -> Result<(), Error> {
        if !Self::can_transition(sample.status, to) {
            return Err(Error::InvalidStateTransition {
                from: sample.status,
                to,
            });
        }

        sample.status = to;
        sample.updated_at = Utc::now();
        sample.updated_by = Some(user_id);

        Ok(())
    }
}
```

#### 2.2.2 Barcode Generation

```rust
pub struct BarcodeGenerator {
    format: BarcodeFormat,
}

#[derive(Debug, Clone)]
pub enum BarcodeFormat {
    Code128,
    QRCode,
    RFID,
}

impl BarcodeGenerator {
    pub fn generate(&self, sample: &Sample) -> Result<String, Error> {
        match self.format {
            BarcodeFormat::Code128 => self.generate_code128(sample),
            BarcodeFormat::QRCode => self.generate_qr_code(sample),
            BarcodeFormat::RFID => self.generate_rfid(sample),
        }
    }

    fn generate_code128(&self, sample: &Sample) -> Result<String, Error> {
        // Format: YYMMDD-SEQ-CHECKSUM
        // Example: 241105-001234-7

        let date = sample.collected_at.format("%y%m%d");
        let seq = format!("{:06}", self.get_sequence()?);
        let checksum = self.calculate_checksum(&format!("{}{}", date, seq));

        Ok(format!("{}-{}-{}", date, seq, checksum))
    }

    fn calculate_checksum(&self, data: &str) -> u8 {
        // Modulo 10 algorithm
        data.bytes()
            .enumerate()
            .map(|(i, b)| {
                let digit = (b - b'0') as u32;
                if i % 2 == 0 { digit } else { digit * 3 }
            })
            .sum::<u32>() % 10
    }
}
```

#### 2.2.3 Chain of Custody (Blockchain)

```rust
pub struct ChainOfCustody {
    blockchain: Arc<BlockchainClient>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustodyRecord {
    sample_id: Uuid,
    from_location: String,
    to_location: String,
    handled_by: Uuid,
    timestamp: DateTime<Utc>,
    gps_location: Option<(f64, f64)>,
    temperature: Option<f32>,
    signature: String,
}

impl ChainOfCustody {
    pub async fn record_transfer(
        &self,
        record: CustodyRecord,
    ) -> Result<String, Error> {
        // Hash the record
        let record_json = serde_json::to_string(&record)?;
        let hash = self.calculate_hash(&record_json);

        // Store on blockchain
        let block_hash = self.blockchain
            .add_block(record_json, hash)
            .await?;

        Ok(block_hash)
    }

    pub async fn verify_chain(
        &self,
        sample_id: Uuid,
    ) -> Result<Vec<CustodyRecord>, Error> {
        // Retrieve all blocks for this sample
        let blocks = self.blockchain
            .get_blocks_by_sample(sample_id)
            .await?;

        // Verify integrity
        for (i, block) in blocks.iter().enumerate() {
            if i > 0 {
                let prev_hash = &blocks[i - 1].hash;
                if block.previous_hash != *prev_hash {
                    return Err(Error::ChainIntegrityViolation);
                }
            }
        }

        Ok(blocks.into_iter().map(|b| b.data).collect())
    }

    fn calculate_hash(&self, data: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
```

---

### 2.3 Result Service with Auto-Verification

#### 2.3.1 Auto-Verification Engine

```rust
pub struct AutoVerificationEngine {
    ml_client: Arc<MLClient>,
    rule_engine: Arc<RuleEngine>,
    config: AutoVerifyConfig,
}

#[derive(Debug, Clone)]
pub struct AutoVerifyConfig {
    min_confidence_threshold: f64, // 0.85 = 85%
    enabled_for_tests: HashSet<Uuid>,
    max_delta_check_deviation: f64,
}

#[derive(Debug)]
pub struct AutoVerificationResult {
    pub can_auto_verify: bool,
    pub confidence: f64,
    pub reasons: Vec<String>,
    pub flags: Vec<VerificationFlag>,
    pub model_version: String,
}

impl AutoVerificationEngine {
    pub async fn verify_result(
        &self,
        result: &TestResult,
    ) -> Result<AutoVerificationResult, Error> {
        let mut reasons = Vec::new();
        let mut flags = Vec::new();

        // 1. Check if test is enabled for auto-verification
        if !self.config.enabled_for_tests.contains(&result.test_id) {
            return Ok(AutoVerificationResult {
                can_auto_verify: false,
                confidence: 0.0,
                reasons: vec!["Test not enabled for auto-verification".to_string()],
                flags: vec![],
                model_version: "N/A".to_string(),
            });
        }

        // 2. Rule-based validation
        let rule_result = self.rule_engine.validate(result).await?;
        if !rule_result.passed {
            reasons.extend(rule_result.failures);
            flags.push(VerificationFlag::RuleViolation);
        }

        // 3. Delta check
        let delta_result = self.check_delta(result).await?;
        if delta_result.deviation > self.config.max_delta_check_deviation {
            reasons.push(format!(
                "Delta check deviation: {:.2}%",
                delta_result.deviation * 100.0
            ));
            flags.push(VerificationFlag::DeltaCheckFailed);
        }

        // 4. Critical value check
        if self.is_critical_value(result)? {
            reasons.push("Critical value detected".to_string());
            flags.push(VerificationFlag::CriticalValue);
            return Ok(AutoVerificationResult {
                can_auto_verify: false,
                confidence: 0.0,
                reasons,
                flags,
                model_version: "N/A".to_string(),
            });
        }

        // 5. ML-based pattern recognition
        let ml_result = self.ml_client
            .predict_verification(result)
            .await?;

        let confidence = ml_result.confidence;

        // 6. Final decision
        let can_auto_verify = confidence >= self.config.min_confidence_threshold
            && flags.is_empty();

        if can_auto_verify {
            reasons.push(format!("ML confidence: {:.2}%", confidence * 100.0));
        } else {
            reasons.push(format!(
                "ML confidence too low: {:.2}% < {:.2}%",
                confidence * 100.0,
                self.config.min_confidence_threshold * 100.0
            ));
        }

        Ok(AutoVerificationResult {
            can_auto_verify,
            confidence,
            reasons,
            flags,
            model_version: ml_result.model_version,
        })
    }

    // Delta check algorithm
    async fn check_delta(&self, result: &TestResult) -> Result<DeltaCheckResult, Error> {
        // Get previous result for same patient and test
        let prev_result = self.get_previous_result(
            result.patient_id,
            result.test_id,
        ).await?;

        let Some(prev) = prev_result else {
            return Ok(DeltaCheckResult {
                has_previous: false,
                deviation: 0.0,
                passed: true,
            });
        };

        // Calculate percentage deviation
        let current_value = result.numeric_value.unwrap_or(0.0);
        let previous_value = prev.numeric_value.unwrap_or(0.0);

        let deviation = if previous_value != 0.0 {
            ((current_value - previous_value) / previous_value).abs()
        } else {
            0.0
        };

        Ok(DeltaCheckResult {
            has_previous: true,
            deviation,
            passed: deviation <= self.config.max_delta_check_deviation,
        })
    }
}
```

#### 2.3.2 ML Client Integration

```rust
pub struct MLClient {
    http_client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct MLPredictionRequest {
    test_id: Uuid,
    patient_id: Uuid,
    value: f64,
    age: u8,
    gender: String,
    previous_values: Vec<f64>,
    qc_status: String,
}

#[derive(Debug, Deserialize)]
pub struct MLPredictionResponse {
    pub confidence: f64,
    pub prediction: bool,
    pub model_version: String,
    pub feature_importance: HashMap<String, f64>,
}

impl MLClient {
    pub async fn predict_verification(
        &self,
        result: &TestResult,
    ) -> Result<MLPredictionResponse, Error> {
        let request = self.build_request(result).await?;

        let response = self.http_client
            .post(&format!("{}/predict/auto-verify", self.base_url))
            .json(&request)
            .timeout(Duration::from_millis(500)) // Fast inference
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::MLServiceError(response.status()));
        }

        Ok(response.json().await?)
    }
}
```

---

### 2.4 Equipment Integration Service

#### 2.4.1 HL7 Parser

```rust
pub struct HL7Parser;

impl HL7Parser {
    pub fn parse_oru_r01(&self, message: &str) -> Result<HL7Result, Error> {
        // HL7 ORU^R01 message structure:
        // MSH|^~\&|ANALYZER|LAB|LIS|HOSPITAL|20241105120000||ORU^R01|123|P|2.5
        // PID|1||MRN123||DOE^JOHN||19800101|M|||...
        // OBR|1||ORDER123|TEST001^Hemoglobin^L|||20241105120000
        // OBX|1|NM|HGB^Hemoglobin^L||14.5|g/dL|13.0-17.0|N|||F

        let segments: Vec<&str> = message.split('\r').collect();

        let mut result = HL7Result::default();

        for segment in segments {
            let fields: Vec<&str> = segment.split('|').collect();

            match fields.get(0) {
                Some(&"MSH") => result.header = self.parse_msh(fields)?,
                Some(&"PID") => result.patient = self.parse_pid(fields)?,
                Some(&"OBR") => result.order = self.parse_obr(fields)?,
                Some(&"OBX") => result.observations.push(self.parse_obx(fields)?),
                _ => {}
            }
        }

        Ok(result)
    }

    fn parse_obx(&self, fields: Vec<&str>) -> Result<Observation, Error> {
        Ok(Observation {
            set_id: fields.get(1).unwrap_or(&"").parse()?,
            value_type: fields.get(2).unwrap_or(&"").to_string(),
            observation_id: fields.get(3).unwrap_or(&"").to_string(),
            value: fields.get(5).unwrap_or(&"").to_string(),
            units: fields.get(6).unwrap_or(&"").to_string(),
            reference_range: fields.get(7).unwrap_or(&"").to_string(),
            abnormal_flags: fields.get(8).unwrap_or(&"").to_string(),
            observation_result_status: fields.get(11).unwrap_or(&"").to_string(),
        })
    }
}
```

#### 2.4.2 ASTM Protocol Handler

```rust
pub struct ASTMProtocolHandler {
    port: SerialPort,
}

#[derive(Debug)]
pub enum ASTMMessage {
    Header,
    Patient,
    Order,
    Result,
    Comment,
    Terminator,
}

impl ASTMProtocolHandler {
    pub async fn listen(&mut self) -> Result<Vec<ASTMResult>, Error> {
        let mut buffer = Vec::new();
        let mut results = Vec::new();

        loop {
            // Read frame
            let frame = self.read_frame().await?;

            match frame.message_type {
                ASTMMessage::Header => {
                    buffer.clear();
                    buffer.push(frame);
                }
                ASTMMessage::Terminator => {
                    buffer.push(frame);
                    // Process complete message
                    let result = self.process_message(&buffer)?;
                    results.push(result);
                    buffer.clear();
                }
                _ => buffer.push(frame),
            }
        }
    }

    async fn read_frame(&mut self) -> Result<ASTMFrame, Error> {
        // ASTM frame format:
        // <STX><FN>Message<ETX><CS><CR><LF>
        // STX = 0x02, ETX = 0x03, CR = 0x0D, LF = 0x0A

        let mut frame_buffer = Vec::new();

        // Wait for STX
        loop {
            let byte = self.read_byte().await?;
            if byte == 0x02 {
                break;
            }
        }

        // Read frame number
        let frame_num = self.read_byte().await?;

        // Read until ETX
        loop {
            let byte = self.read_byte().await?;
            if byte == 0x03 {
                break;
            }
            frame_buffer.push(byte);
        }

        // Read checksum
        let checksum = self.read_byte().await?;

        // Verify checksum
        let calculated_checksum = self.calculate_checksum(&frame_buffer);
        if checksum != calculated_checksum {
            return Err(Error::ChecksumMismatch);
        }

        // Parse message
        let message = String::from_utf8(frame_buffer)?;
        let message_type = self.determine_message_type(&message);

        Ok(ASTMFrame {
            frame_num,
            message_type,
            data: message,
        })
    }
}
```

---

## 3. Algorithms & Data Structures

### 3.1 TAT (Turnaround Time) Prediction

```rust
pub struct TATPredictor {
    ml_model: Arc<TATModel>,
    historical_data: Arc<TimeSeriesCache>,
}

impl TATPredictor {
    pub async fn predict_tat(
        &self,
        order: &Order,
    ) -> Result<TATPrediction, Error> {
        // Features for ML model
        let features = self.extract_features(order).await?;

        // Get prediction from ML model
        let prediction = self.ml_model.predict(&features).await?;

        // Adjust based on current load
        let current_load = self.get_current_load().await?;
        let adjusted_tat = self.adjust_for_load(prediction.tat_minutes, current_load);

        // Calculate due time
        let due_at = order.created_at + Duration::minutes(adjusted_tat as i64);

        Ok(TATPrediction {
            estimated_tat_minutes: adjusted_tat,
            due_at,
            confidence: prediction.confidence,
            factors: prediction.feature_importance,
        })
    }

    async fn extract_features(&self, order: &Order) -> Result<TATFeatures, Error> {
        Ok(TATFeatures {
            // Test characteristics
            test_count: order.tests.len(),
            test_complexity: self.calculate_complexity(&order.tests),
            requires_manual_processing: self.has_manual_tests(&order.tests),

            // Temporal features
            hour_of_day: order.created_at.hour(),
            day_of_week: order.created_at.weekday().number_from_monday(),
            is_weekend: order.created_at.weekday().number_from_monday() > 5,

            // Load features
            current_queue_length: self.get_queue_length().await?,
            average_processing_time: self.get_avg_processing_time().await?,
            staff_on_duty: self.get_staff_count().await?,

            // Historical features
            avg_tat_last_hour: self.get_historical_tat(Duration::hours(1)).await?,
            avg_tat_same_dow: self.get_historical_tat_dow(
                order.created_at.weekday()
            ).await?,
        })
    }
}
```

### 3.2 QC Westgard Rules Implementation

```rust
pub struct WestgardRules;

impl WestgardRules {
    /// Apply Westgard multirule QC algorithm
    pub fn apply_rules(qc_data: &[QCDataPoint]) -> Vec<RuleViolation> {
        let mut violations = Vec::new();

        if qc_data.len() < 2 {
            return violations;
        }

        let mean = calculate_mean(qc_data);
        let sd = calculate_std_dev(qc_data, mean);

        // Rule 1_2s: Warning if 1 point > 2 SD
        if let Some(violation) = self.check_1_2s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule 1_3s: Reject if 1 point > 3 SD
        if let Some(violation) = self.check_1_3s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule 2_2s: Reject if 2 consecutive points > 2 SD (same side)
        if let Some(violation) = self.check_2_2s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule R_4s: Reject if range > 4 SD
        if let Some(violation) = self.check_r_4s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule 4_1s: Reject if 4 consecutive points > 1 SD (same side)
        if let Some(violation) = self.check_4_1s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule 10_x: Reject if 10 consecutive points on same side of mean
        if let Some(violation) = self.check_10_x(qc_data, mean) {
            violations.push(violation);
        }

        violations
    }

    fn check_1_3s(
        data: &[QCDataPoint],
        mean: f64,
        sd: f64,
    ) -> Option<RuleViolation> {
        for point in data {
            let z_score = (point.value - mean).abs() / sd;
            if z_score > 3.0 {
                return Some(RuleViolation {
                    rule: "1_3s",
                    severity: Severity::Reject,
                    description: format!(
                        "Value {} is {:.2} SD from mean",
                        point.value, z_score
                    ),
                    affected_points: vec![point.clone()],
                });
            }
        }
        None
    }

    fn check_2_2s(
        data: &[QCDataPoint],
        mean: f64,
        sd: f64,
    ) -> Option<RuleViolation> {
        for window in data.windows(2) {
            let z1 = (window[0].value - mean) / sd;
            let z2 = (window[1].value - mean) / sd;

            // Both on same side and > 2 SD
            if (z1 > 2.0 && z2 > 2.0) || (z1 < -2.0 && z2 < -2.0) {
                return Some(RuleViolation {
                    rule: "2_2s",
                    severity: Severity::Reject,
                    description: "2 consecutive points > 2 SD on same side".to_string(),
                    affected_points: window.to_vec(),
                });
            }
        }
        None
    }
}
```

### 3.3 Sample Routing Algorithm

```rust
pub struct SampleRouter {
    equipment_status: Arc<EquipmentStatusCache>,
    workload_balancer: Arc<WorkloadBalancer>,
}

impl SampleRouter {
    pub async fn route_sample(
        &self,
        sample: &Sample,
        test: &Test,
    ) -> Result<EquipmentAssignment, Error> {
        // Get all equipment capable of performing this test
        let capable_equipment = self.get_capable_equipment(test).await?;

        if capable_equipment.is_empty() {
            return Err(Error::NoAvailableEquipment);
        }

        // Filter by status (online, calibrated, QC passed)
        let available = self.filter_available(&capable_equipment).await?;

        // Score each equipment
        let mut scores: Vec<(Equipment, f64)> = Vec::new();

        for equipment in available {
            let score = self.calculate_score(&equipment, sample, test).await?;
            scores.push((equipment, score));
        }

        // Sort by score (descending)
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Select best equipment
        let (best_equipment, _) = scores.first()
            .ok_or(Error::NoSuitableEquipment)?;

        Ok(EquipmentAssignment {
            equipment_id: best_equipment.id,
            sample_id: sample.id,
            test_id: test.id,
            assigned_at: Utc::now(),
            priority: sample.priority,
        })
    }

    async fn calculate_score(
        &self,
        equipment: &Equipment,
        sample: &Sample,
        test: &Test,
    ) -> Result<f64, Error> {
        let mut score = 100.0;

        // Factor 1: Current workload (lower is better)
        let workload = self.workload_balancer
            .get_current_load(equipment.id)
            .await?;
        score -= workload as f64 * 0.5;

        // Factor 2: Equipment performance (higher is better)
        let uptime = equipment.uptime_percentage;
        score += uptime * 0.3;

        // Factor 3: Last calibration (more recent is better)
        let days_since_calibration = (Utc::now() - equipment.last_calibration)
            .num_days();
        score -= days_since_calibration as f64 * 0.1;

        // Factor 4: TAT capability (faster is better)
        let avg_tat = equipment.average_tat_minutes;
        score -= avg_tat * 0.2;

        // Factor 5: Priority sample boost
        if sample.priority == 1 {
            score += 20.0; // STAT samples get priority
        }

        Ok(score.max(0.0))
    }
}
```

---

## 4. State Machines

### 4.1 Order State Machine

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    SampleCollected,
    Processing,
    PartiallyCompleted,
    Completed,
    Cancelled,
}

pub struct OrderStateMachine;

impl OrderStateMachine {
    pub fn allowed_transitions(status: OrderStatus) -> Vec<OrderStatus> {
        use OrderStatus::*;

        match status {
            Pending => vec![Confirmed, Cancelled],
            Confirmed => vec![SampleCollected, Cancelled],
            SampleCollected => vec![Processing, Cancelled],
            Processing => vec![PartiallyCompleted, Completed, Cancelled],
            PartiallyCompleted => vec![Completed, Cancelled],
            Completed => vec![], // Terminal state
            Cancelled => vec![], // Terminal state
        }
    }

    pub fn can_transition(from: OrderStatus, to: OrderStatus) -> bool {
        Self::allowed_transitions(from).contains(&to)
    }
}
```

### 4.2 Result Verification State Machine

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultStatus {
    Pending,
    Entered,
    TechnicalReview,
    PathologistReview,
    Verified,
    Approved,
    Released,
    Amended,
}

pub struct ResultStateMachine {
    config: VerificationConfig,
}

#[derive(Debug, Clone)]
pub struct VerificationConfig {
    require_technical_review: bool,
    require_pathologist_review: bool,
    require_director_approval: bool,
    allow_auto_verification: bool,
}

impl ResultStateMachine {
    pub fn next_status(
        &self,
        current: ResultStatus,
        test: &Test,
        auto_verify_result: Option<&AutoVerificationResult>,
    ) -> ResultStatus {
        use ResultStatus::*;

        match current {
            Pending => Entered,

            Entered => {
                // Check if auto-verification is possible
                if self.config.allow_auto_verification {
                    if let Some(av) = auto_verify_result {
                        if av.can_auto_verify {
                            return Verified;
                        }
                    }
                }

                // Manual verification path
                if self.config.require_technical_review {
                    TechnicalReview
                } else if self.config.require_pathologist_review {
                    PathologistReview
                } else {
                    Verified
                }
            }

            TechnicalReview => {
                if self.config.require_pathologist_review || test.requires_approval {
                    PathologistReview
                } else {
                    Verified
                }
            }

            PathologistReview => {
                if self.config.require_director_approval {
                    Approved
                } else {
                    Verified
                }
            }

            Approved | Verified => Released,

            Released => Amended, // Only if amended

            Amended => Released, // After amendment, release again
        }
    }
}
```

---

## 5. Caching Strategy

### 5.1 Multi-Level Cache

```rust
pub struct CacheManager {
    l1: Arc<MemoryCache>,      // In-process cache
    l2: Arc<RedisCache>,        // Distributed cache
    l3: Arc<DatabaseCache>,     // Database query cache
}

impl CacheManager {
    pub async fn get<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, Error> {
        // L1: Check in-memory cache (fastest)
        if let Some(value) = self.l1.get::<T>(key).await? {
            return Ok(Some(value));
        }

        // L2: Check Redis (fast)
        if let Some(value) = self.l2.get::<T>(key).await? {
            // Populate L1
            self.l1.set(key, &value, 300).await?;
            return Ok(Some(value));
        }

        // L3: Check database query cache
        if let Some(value) = self.l3.get::<T>(key).await? {
            // Populate L2 and L1
            self.l2.set(key, &value, 3600).await?;
            self.l1.set(key, &value, 300).await?;
            return Ok(Some(value));
        }

        Ok(None)
    }

    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: u64,
    ) -> Result<(), Error> {
        // Set in all levels
        self.l1.set(key, value, ttl.min(300)).await?;
        self.l2.set(key, value, ttl).await?;
        Ok(())
    }

    pub async fn invalidate(&self, key: &str) -> Result<(), Error> {
        // Invalidate all levels
        self.l1.delete(key).await?;
        self.l2.delete(key).await?;
        Ok(())
    }
}
```

### 5.2 Cache-Aside Pattern

```rust
pub async fn get_test_catalog_cached(
    cache: &CacheManager,
    db: &Database,
) -> Result<Vec<Test>, Error> {
    const CACHE_KEY: &str = "test_catalog:all";
    const TTL: u64 = 3600; // 1 hour

    // Try cache first
    if let Some(catalog) = cache.get::<Vec<Test>>(CACHE_KEY).await? {
        return Ok(catalog);
    }

    // Cache miss - fetch from database
    let catalog = db.get_test_catalog().await?;

    // Update cache
    cache.set(CACHE_KEY, &catalog, TTL).await?;

    Ok(catalog)
}
```

### 5.3 Cache Invalidation Strategy

```rust
pub struct CacheInvalidator {
    cache: Arc<CacheManager>,
    event_subscriber: Arc<EventSubscriber>,
}

impl CacheInvalidator {
    pub async fn start(&self) {
        self.event_subscriber
            .subscribe(|event| {
                match event {
                    Event::TestCatalogUpdated { test_id } => {
                        self.invalidate_test_cache(test_id).await
                    }
                    Event::PatientUpdated { patient_id } => {
                        self.invalidate_patient_cache(patient_id).await
                    }
                    Event::ResultVerified { result_id } => {
                        self.invalidate_result_cache(result_id).await
                    }
                    _ => Ok(()),
                }
            })
            .await;
    }

    async fn invalidate_test_cache(&self, test_id: Uuid) -> Result<(), Error> {
        // Invalidate specific test
        self.cache.invalidate(&format!("test:{}", test_id)).await?;

        // Invalidate test catalog
        self.cache.invalidate("test_catalog:all").await?;

        Ok(())
    }
}
```

---

## 6. Error Handling

### 6.1 Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Concurrency conflict")]
    ConcurrencyConflict,

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),

    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: String,
        to: String,
    },
}

// Convert to GraphQL errors
impl From<ServiceError> for async_graphql::Error {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::NotFound(msg) => {
                async_graphql::Error::new(msg)
                    .extend_with(|_, e| e.set("code", "NOT_FOUND"))
            }
            ServiceError::Unauthorized => {
                async_graphql::Error::new("Unauthorized")
                    .extend_with(|_, e| e.set("code", "UNAUTHORIZED"))
            }
            ServiceError::ValidationError(msg) => {
                async_graphql::Error::new(msg)
                    .extend_with(|_, e| e.set("code", "VALIDATION_ERROR"))
            }
            _ => async_graphql::Error::new(err.to_string()),
        }
    }
}
```

### 6.2 Retry Logic with Exponential Backoff

```rust
pub struct RetryPolicy {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
}

impl RetryPolicy {
    pub async fn execute<F, T, E>(&self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
        E: std::error::Error,
    {
        let mut attempt = 0;
        let mut delay = self.initial_delay;

        loop {
            attempt += 1;

            match operation().await {
                Ok(result) => return Ok(result),
                Err(err) if attempt >= self.max_attempts => return Err(err),
                Err(err) if !self.is_retryable(&err) => return Err(err),
                Err(_) => {
                    // Exponential backoff with jitter
                    let jitter = rand::random::<f64>() * 0.1 * delay.as_millis() as f64;
                    tokio::time::sleep(delay + Duration::from_millis(jitter as u64)).await;

                    delay = Duration::from_millis(
                        (delay.as_millis() as f64 * self.multiplier) as u64
                    ).min(self.max_delay);
                }
            }
        }
    }

    fn is_retryable<E: std::error::Error>(&self, error: &E) -> bool {
        // Define retryable errors
        let error_str = error.to_string().to_lowercase();
        error_str.contains("timeout")
            || error_str.contains("connection")
            || error_str.contains("temporary")
    }
}
```

---

## 7. Performance Optimization

### 7.1 Database Query Optimization

```rust
// Bad: N+1 query problem
async fn get_orders_with_tests_bad(db: &PgPool) -> Result<Vec<Order>, Error> {
    let orders = sqlx::query_as!(Order, "SELECT * FROM orders")
        .fetch_all(db)
        .await?;

    for order in &mut orders {
        // N queries for N orders
        let tests = sqlx::query_as!(Test, "SELECT * FROM order_test WHERE order_id = $1", order.id)
            .fetch_all(db)
            .await?;
        order.tests = tests;
    }

    Ok(orders)
}

// Good: Single query with JOIN
async fn get_orders_with_tests_good(db: &PgPool) -> Result<Vec<Order>, Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            o.*,
            ot.id as test_id,
            ot.test_id as test_test_id,
            ot.status as test_status
        FROM orders o
        LEFT JOIN order_test ot ON o.id = ot.order_id
        ORDER BY o.id, ot.id
        "#
    )
    .fetch_all(db)
    .await?;

    // Group by order
    let mut orders_map: HashMap<Uuid, Order> = HashMap::new();

    for row in rows {
        let order = orders_map.entry(row.id).or_insert_with(|| Order {
            id: row.id,
            // ... other fields
            tests: Vec::new(),
        });

        if let Some(test_id) = row.test_id {
            order.tests.push(OrderTest {
                id: test_id,
                // ... other fields
            });
        }
    }

    Ok(orders_map.into_values().collect())
}
```

### 7.2 Connection Pooling

```rust
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
}

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connection_timeout)
        .idle_timeout(config.idle_timeout)
        .connect(&std::env::var("DATABASE_URL")?)
        .await
}
```

### 7.3 Batch Processing

```rust
pub async fn batch_insert_results(
    db: &PgPool,
    results: Vec<TestResult>,
) -> Result<(), Error> {
    const BATCH_SIZE: usize = 100;

    for chunk in results.chunks(BATCH_SIZE) {
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO test_result (id, order_test_id, value, status, created_at) "
        );

        query_builder.push_values(chunk, |mut b, result| {
            b.push_bind(result.id)
                .push_bind(result.order_test_id)
                .push_bind(result.value)
                .push_bind(result.status)
                .push_bind(result.created_at);
        });

        query_builder.build().execute(db).await?;
    }

    Ok(())
}
```

---

## 8. Security Implementation

### 8.1 Encryption Service

```rust
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

pub struct EncryptionService {
    cipher: Aes256Gcm,
}

impl EncryptionService {
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new(key.into());
        Self { cipher }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, Error> {
        let nonce = Nonce::from_slice(b"unique nonce"); // Use random nonce in production

        let ciphertext = self.cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| Error::EncryptionError(e.to_string()))?;

        // Base64 encode for storage
        Ok(base64::encode(&ciphertext))
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String, Error> {
        let nonce = Nonce::from_slice(b"unique nonce");

        let decoded = base64::decode(ciphertext)?;

        let plaintext = self.cipher
            .decrypt(nonce, decoded.as_ref())
            .map_err(|e| Error::DecryptionError(e.to_string()))?;

        Ok(String::from_utf8(plaintext)?)
    }
}
```

### 8.2 Rate Limiting

```rust
use governor::{Quota, RateLimiter};

pub struct ApiRateLimiter {
    limiter: RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>,
}

impl ApiRateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(requests_per_minute).unwrap());
        Self {
            limiter: RateLimiter::keyed(quota),
        }
    }

    pub async fn check_rate_limit(&self, user_id: &str) -> Result<(), Error> {
        match self.limiter.check_key(&user_id.to_string()) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::RateLimitExceeded),
        }
    }
}
```

---

## Summary

This Low-Level Design document provides:

1. **Detailed Component Designs**: Complete implementation patterns for all microservices
2. **Algorithms**: TAT prediction, QC Westgard rules, sample routing, delta check
3. **Data Structures**: Domain models, value objects, repository patterns
4. **State Machines**: Order lifecycle, result verification, sample tracking
5. **Caching Strategy**: Multi-level caching, cache-aside pattern, invalidation
6. **Error Handling**: Comprehensive error types, retry logic, resilience
7. **Performance**: Query optimization, connection pooling, batch processing
8. **Security**: Encryption, rate limiting, audit trails

This design enables:
- **High Performance**: <100ms API responses through caching and optimization
- **Scalability**: Horizontal scaling of all components
- **Reliability**: Retry logic, circuit breakers, graceful degradation
- **Maintainability**: Clean code, SOLID principles, comprehensive tests
- **Security**: End-to-end encryption, rate limiting, audit trails

---

**Next Steps**:
1. Implementation of microservices in Rust
2. Unit tests for all algorithms
3. Integration tests for workflows
4. Performance testing and optimization
5. Security audits

---

**Document Status**: ✅ Approved
**Next Review Date**: 2025-02-05
**Owned By**: Engineering Team
