# Modules 04-12: Comprehensive Documentation

This document provides detailed specifications for the remaining 9 core operational modules of the LIS/LIMS system.

---

# 04. Equipment Management Module

## Overview
Manages laboratory equipment integration, monitoring, calibration, and maintenance with HL7/ASTM bidirectional communication.

## Key Features

### 1. Equipment Integration (HL7/ASTM)

```rust
pub struct EquipmentIntegration {
    equipment_id: Uuid,
    protocol: IntegrationProtocol,
    connection: ConnectionType,
}

pub enum IntegrationProtocol {
    HL7v2_5,
    ASTM_E1381,
    ASTM_E1394,
    LIS2A2,
}

pub enum ConnectionType {
    Serial { port: String, baud_rate: u32 },
    TCP { host: String, port: u16 },
    FileTransfer { directory: String },
}

// HL7 Message Handler
pub async fn handle_hl7_message(
    equipment_id: Uuid,
    message: &str,
) -> Result<HL7Response, Error> {
    let parsed = parse_hl7_message(message)?;

    match parsed.message_type {
        "QRY" => handle_query_message(equipment_id, parsed).await,
        "ORU" => handle_result_message(equipment_id, parsed).await,
        "QCK" => handle_qc_message(equipment_id, parsed).await,
        _ => Err(Error::UnsupportedMessageType),
    }
}

// Process ORU (Observation Result) message
async fn handle_result_message(
    equipment_id: Uuid,
    message: ParsedHL7,
) -> Result<HL7Response, Error> {
    let sample_barcode = message.get_field("OBR.3")?;
    let sample = find_sample_by_barcode(&sample_barcode).await?;

    for observation in message.observations {
        let test_code = observation.test_code;
        let value = observation.value;
        let unit = observation.unit;
        let flag = observation.abnormal_flag;

        // Create or update result
        create_or_update_result(CreateResultInput {
            sample_id: sample.id,
            test_code,
            value,
            unit,
            flag,
            equipment_id,
            received_at: Utc::now(),
        }).await?;
    }

    Ok(HL7Response::Acknowledgment(ACK_SUCCESS))
}
```

### 2. Real-time Equipment Monitoring

```rust
pub struct EquipmentMonitor {
    redis: Arc<RedisClient>,
}

impl EquipmentMonitor {
    pub async fn track_status(
        &self,
        equipment_id: Uuid,
        status: EquipmentStatus,
    ) -> Result<(), Error> {
        let key = format!("equipment:{}:status", equipment_id);

        self.redis.set_with_expiry(&key, &status, 300).await?;

        // Check for status change
        if let Some(prev_status) = self.get_previous_status(equipment_id).await? {
            if prev_status != status {
                publish_event(Event::EquipmentStatusChanged {
                    equipment_id,
                    from: prev_status,
                    to: status,
                }).await?;

                // Alert if equipment went offline
                if status == EquipmentStatus::Offline {
                    alert_technicians(equipment_id).await?;
                }
            }
        }

        Ok(())
    }
}

pub enum EquipmentStatus {
    Online,
    Offline,
    Busy,
    Error,
    Maintenance,
    Calibrating,
    RunningQC,
}
```

### 3. Calibration Management

```rust
pub struct CalibrationScheduler {
    db: Arc<Database>,
}

impl CalibrationScheduler {
    pub async fn check_calibration_due(
        &self,
    ) -> Result<Vec<CalibrationDue>, Error> {
        let query = r#"
            SELECT
                e.id,
                e.name,
                c.last_calibration_date,
                c.calibration_interval_days,
                c.next_calibration_due
            FROM equipment e
            JOIN calibration_schedule c ON e.id = c.equipment_id
            WHERE c.next_calibration_due <= NOW() + INTERVAL '3 days'
              AND e.is_active = true
            ORDER BY c.next_calibration_due ASC
        "#;

        let results = sqlx::query_as::<_, CalibrationDue>(query)
            .fetch_all(&self.db)
            .await?;

        // Send notifications for overdue calibrations
        for cal in &results {
            if cal.next_calibration_due < Utc::now() {
                notify_calibration_overdue(cal.equipment_id).await?;
            }
        }

        Ok(results)
    }

    pub async fn record_calibration(
        &self,
        calibration: CalibrationRecord,
    ) -> Result<(), Error> {
        // Save calibration record
        save_calibration(&calibration).await?;

        // Update next calibration date
        let next_date = calibration.calibrated_at
            + Duration::days(calibration.interval_days as i64);

        update_next_calibration_date(
            calibration.equipment_id,
            next_date,
        ).await?;

        // Record in compliance log
        record_compliance_event(ComplianceEvent {
            event_type: ComplianceEventType::Calibration,
            equipment_id: Some(calibration.equipment_id),
            performed_by: calibration.performed_by,
            timestamp: calibration.calibrated_at,
            details: format!("Calibration successful. Reference: {}", calibration.reference_material),
        }).await?;

        Ok(())
    }
}
```

### 4. Preventive Maintenance

```rust
pub struct MaintenanceSchedule {
    pub equipment_id: Uuid,
    pub maintenance_type: MaintenanceType,
    pub frequency_days: i32,
    pub last_maintenance_date: DateTime<Utc>,
    pub next_maintenance_due: DateTime<Utc>,
    pub checklist: Vec<MaintenanceCheckItem>,
}

pub enum MaintenanceType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annual,
    AsNeeded,
}

pub struct MaintenanceCheckItem {
    pub item: String,
    pub category: String,        // Cleaning, Inspection, Lubrication
    pub is_critical: bool,
    pub expected_result: String,
}
```

## Data Models

```rust
pub struct Equipment {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    pub serial_number: String,

    // Integration
    pub integration_protocol: Option<IntegrationProtocol>,
    pub connection_config: Option<ConnectionConfig>,
    pub is_integrated: bool,

    // Capabilities
    pub supported_tests: Vec<Uuid>,
    pub max_capacity_per_hour: i32,
    pub auto_load_samples: bool,

    // Status
    pub status: EquipmentStatus,
    pub current_workload: i32,

    // Maintenance
    pub last_calibration_date: Option<DateTime<Utc>>,
    pub next_calibration_due: Option<DateTime<Utc>>,
    pub last_maintenance_date: Option<DateTime<Utc>>,

    // Reagents
    pub reagent_levels: HashMap<String, f64>,

    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
```

## API Reference

```graphql
type Equipment {
  id: ID!
  name: String!
  manufacturer: String!
  status: EquipmentStatus!
  currentWorkload: Int!
  supportedTests: [Test!]!
  calibrationDue: DateTime
  maintenanceDue: DateTime
  reagentLevels: [ReagentLevel!]!
}

type Mutation {
  recordCalibration(input: CalibrationInput!): Calibration!
  recordMaintenance(input: MaintenanceInput!): Maintenance!
  updateEquipmentStatus(id: ID!, status: EquipmentStatus!): Equipment!
}
```

---

# 05. Result Management Module

## Overview
Handles result entry, validation, AI-powered auto-verification, and multi-level review workflows.

## Key Features

### 1. AI-Powered Auto-Verification

```rust
pub struct AutoVerificationEngine {
    ml_client: Arc<MLClient>,
    rule_engine: Arc<RuleEngine>,
}

impl AutoVerificationEngine {
    pub async fn verify_result(
        &self,
        result: &TestResult,
    ) -> Result<AutoVerificationDecision, Error> {
        let mut checks = Vec::new();

        // 1. Reference Range Check
        let ref_range_check = self.check_reference_range(result).await?;
        checks.push(ref_range_check);
        if !ref_range_check.passed {
            return Ok(AutoVerificationDecision {
                can_auto_verify: false,
                reason: "Outside reference range".to_string(),
                requires_review: true,
            });
        }

        // 2. Critical Value Check
        if self.is_critical_value(result).await? {
            return Ok(AutoVerificationDecision {
                can_auto_verify: false,
                reason: "Critical value detected".to_string(),
                requires_pathologist_review: true,
            });
        }

        // 3. Delta Check
        let delta_check = self.perform_delta_check(result).await?;
        checks.push(delta_check);
        if !delta_check.passed {
            return Ok(AutoVerificationDecision {
                can_auto_verify: false,
                reason: format!("Delta check failed: {}% change", delta_check.percentage),
                requires_review: true,
            });
        }

        // 4. ML-Based Verification
        let ml_features = ResultVerificationFeatures {
            test_id: result.test_id,
            value: result.value.parse::<f64>().ok(),
            patient_age: result.patient_age,
            patient_gender: result.patient_gender.clone(),
            equipment_id: result.equipment_id,
            previous_results: self.get_previous_results(result).await?,
        };

        let ml_prediction = self.ml_client
            .predict_result_validity(ml_features)
            .await?;

        if ml_prediction.confidence >= 0.85 {
            Ok(AutoVerificationDecision {
                can_auto_verify: true,
                confidence: ml_prediction.confidence,
                checks_passed: checks,
            })
        } else {
            Ok(AutoVerificationDecision {
                can_auto_verify: false,
                reason: format!("Low ML confidence: {:.2}%", ml_prediction.confidence * 100.0),
                requires_review: true,
            })
        }
    }
}
```

### 2. Delta Check Algorithm

```rust
pub async fn perform_delta_check(
    &self,
    result: &TestResult,
) -> Result<DeltaCheckResult, Error> {
    // Get previous result for same test
    let previous = self.get_most_recent_result(
        result.patient_id,
        result.test_id,
    ).await?;

    if let Some(prev) = previous {
        let current_value: f64 = result.value.parse()?;
        let previous_value: f64 = prev.value.parse()?;

        let delta_percentage = ((current_value - previous_value) / previous_value).abs() * 100.0;

        let threshold = self.get_delta_threshold(result.test_id).await?;

        let passed = delta_percentage <= threshold;

        Ok(DeltaCheckResult {
            passed,
            current_value,
            previous_value,
            delta_percentage,
            threshold,
            previous_test_date: prev.tested_at,
            days_between: (result.tested_at - prev.tested_at).num_days(),
        })
    } else {
        // No previous result, pass by default
        Ok(DeltaCheckResult {
            passed: true,
            no_previous_result: true,
        })
    }
}
```

### 3. Multi-Level Review Workflow

```rust
pub enum VerificationLevel {
    AutoVerified,           // AI auto-verification
    TechnicianVerified,     // Lab technician review
    PathologistVerified,    // Pathologist review
    SeniorPathologistVerified, // Senior pathologist (critical values)
}

pub struct ResultReviewWorkflow {
    db: Arc<Database>,
}

impl ResultReviewWorkflow {
    pub async fn route_for_review(
        &self,
        result_id: Uuid,
    ) -> Result<ReviewAssignment, Error> {
        let result = get_result(result_id).await?;

        // Determine review level required
        let review_level = if result.is_critical {
            VerificationLevel::SeniorPathologistVerified
        } else if result.is_abnormal {
            VerificationLevel::PathologistVerified
        } else if result.delta_check_failed {
            VerificationLevel::TechnicianVerified
        } else {
            VerificationLevel::AutoVerified
        };

        // Find available reviewer
        let reviewer = match review_level {
            VerificationLevel::TechnicianVerified => {
                find_available_technician().await?
            }
            VerificationLevel::PathologistVerified => {
                find_available_pathologist().await?
            }
            VerificationLevel::SeniorPathologistVerified => {
                find_senior_pathologist().await?
            }
            _ => return Ok(ReviewAssignment::AutoVerified),
        };

        // Create review task
        create_review_task(ReviewTask {
            result_id,
            assigned_to: reviewer.id,
            review_level,
            priority: result.priority,
            deadline: calculate_review_deadline(&result),
        }).await?;

        Ok(ReviewAssignment {
            reviewer: reviewer.name,
            level: review_level,
        })
    }
}
```

### 4. Result Amendment

```rust
pub async fn amend_result(
    result_id: Uuid,
    amendment: ResultAmendment,
) -> Result<TestResult, Error> {
    let original_result = get_result(result_id).await?;

    // Validate permission
    if !has_permission(amendment.amended_by, "AMEND_RESULT") {
        return Err(Error::InsufficientPermissions);
    }

    // Cannot amend if report already delivered
    if original_result.report_delivered {
        return Err(Error::CannotAmendDeliveredResult);
    }

    // Create amendment record
    let amendment_record = AmendmentRecord {
        id: Uuid::new_v4(),
        result_id,
        original_value: original_result.value.clone(),
        amended_value: amendment.new_value.clone(),
        reason: amendment.reason,
        amended_by: amendment.amended_by,
        amended_at: Utc::now(),
        amendment_notes: amendment.notes,
    };

    save_amendment_record(&amendment_record).await?;

    // Update result
    let mut updated_result = original_result;
    updated_result.value = amendment.new_value;
    updated_result.is_amended = true;
    updated_result.amendment_count += 1;
    updated_result.last_amended_at = Some(Utc::now());

    save_result(&updated_result).await?;

    // Publish event
    publish_event(Event::ResultAmended {
        result_id,
        original_value: amendment_record.original_value,
        new_value: amendment_record.amended_value,
    }).await?;

    Ok(updated_result)
}
```

## Data Models

```rust
pub struct TestResult {
    pub id: Uuid,
    pub sample_id: Uuid,
    pub order_id: Uuid,
    pub patient_id: Uuid,
    pub test_id: Uuid,

    // Result Data
    pub value: String,
    pub unit: String,
    pub reference_range: String,
    pub flag: ResultFlag, // NORMAL, HIGH, LOW, CRITICAL_HIGH, CRITICAL_LOW

    // Verification
    pub verification_status: VerificationStatus,
    pub verification_method: VerificationMethod,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub auto_verification_confidence: Option<f64>,

    // Checks
    pub delta_check_performed: bool,
    pub delta_check_passed: bool,
    pub is_critical: bool,
    pub is_abnormal: bool,

    // Amendment
    pub is_amended: bool,
    pub amendment_count: i32,
    pub last_amended_at: Option<DateTime<Utc>>,

    // Source
    pub equipment_id: Option<Uuid>,
    pub entry_method: EntryMethod, // MANUAL, INTERFACE, IMPORT

    pub tested_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub enum VerificationStatus {
    Pending,
    AutoVerified,
    TechnicianVerified,
    PathologistVerified,
    Rejected,
}

pub enum VerificationMethod {
    Auto,
    Manual,
}

pub enum ResultFlag {
    Normal,
    High,
    Low,
    CriticalHigh,
    CriticalLow,
    Abnormal,
}
```

---

# 06. Quality Control Module

## Overview
Comprehensive QC management including IQC, EQC, and Westgard rules implementation.

## Key Features

### 1. Westgard Rules Implementation

```rust
pub struct WestgardRules;

impl WestgardRules {
    pub fn evaluate(qc_data: &[QCDataPoint]) -> Vec<RuleViolation> {
        let mean = calculate_mean(qc_data);
        let sd = calculate_std_dev(qc_data, mean);
        let mut violations = Vec::new();

        // Rule 1_3s: Reject if 1 point > 3 SD
        if let Some(violation) = Self::check_1_3s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule 2_2s: Reject if 2 consecutive > 2 SD (same side)
        if let Some(violation) = Self::check_2_2s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule R_4s: Reject if range of 2 consecutive > 4 SD
        if let Some(violation) = Self::check_r_4s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule 4_1s: Warning if 4 consecutive > 1 SD (same side)
        if let Some(violation) = Self::check_4_1s(qc_data, mean, sd) {
            violations.push(violation);
        }

        // Rule 10_x: Warning if 10 consecutive on same side of mean
        if let Some(violation) = Self::check_10_x(qc_data, mean) {
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
            let z_score = (point.value - mean) / sd;
            if z_score.abs() > 3.0 {
                return Some(RuleViolation {
                    rule: "1_3s".to_string(),
                    severity: Severity::Critical,
                    description: format!(
                        "Single point exceeds 3 SD (Z-score: {:.2})",
                        z_score
                    ),
                    action_required: "Reject run and investigate",
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
        for i in 1..data.len() {
            let z1 = (data[i - 1].value - mean) / sd;
            let z2 = (data[i].value - mean) / sd;

            if (z1 > 2.0 && z2 > 2.0) || (z1 < -2.0 && z2 < -2.0) {
                return Some(RuleViolation {
                    rule: "2_2s".to_string(),
                    severity: Severity::Critical,
                    description: "Two consecutive points exceed 2 SD on same side".to_string(),
                    action_required: "Reject run, check for systematic error",
                    affected_points: vec![data[i - 1].clone(), data[i].clone()],
                });
            }
        }
        None
    }

    fn check_10_x(
        data: &[QCDataPoint],
        mean: f64,
    ) -> Option<RuleViolation> {
        if data.len() < 10 {
            return None;
        }

        let last_10 = &data[data.len() - 10..];
        let all_above = last_10.iter().all(|p| p.value > mean);
        let all_below = last_10.iter().all(|p| p.value < mean);

        if all_above || all_below {
            return Some(RuleViolation {
                rule: "10_x".to_string(),
                severity: Severity::Warning,
                description: "Ten consecutive points on same side of mean".to_string(),
                action_required: "Warning - monitor for trend",
                affected_points: last_10.to_vec(),
            });
        }
        None
    }
}
```

### 2. QC Lot Management

```rust
pub struct QCLot {
    pub id: Uuid,
    pub lot_number: String,
    pub control_name: String,
    pub level: QCLevel,         // Level 1, 2, 3
    pub test_id: Uuid,

    // Statistics
    pub target_mean: f64,
    pub target_sd: f64,
    pub acceptable_range: (f64, f64),

    // Dates
    pub manufactured_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub opened_date: Option<NaiveDate>,
    pub stability_days_after_opening: i32,

    pub is_active: bool,
}

pub enum QCLevel {
    Normal,
    Abnormal,
    Pathological,
}
```

### 3. Levy-Jennings Charts

```rust
pub async fn generate_levy_jennings_chart(
    test_id: Uuid,
    period_days: i32,
) -> Result<LevyJenningsChart, Error> {
    let qc_data = get_qc_data(test_id, period_days).await?;

    let mean = calculate_mean(&qc_data);
    let sd = calculate_std_dev(&qc_data, mean);

    let chart = LevyJenningsChart {
        test_id,
        data_points: qc_data,
        mean,
        sd,
        control_limits: ControlLimits {
            plus_3sd: mean + (3.0 * sd),
            plus_2sd: mean + (2.0 * sd),
            plus_1sd: mean + sd,
            mean,
            minus_1sd: mean - sd,
            minus_2sd: mean - (2.0 * sd),
            minus_3sd: mean - (3.0 * sd),
        },
        violations: WestgardRules::evaluate(&qc_data),
    };

    Ok(chart)
}
```

---

# 07. Report Management Module

## Overview
Report generation, digital signature, and multi-channel delivery (WhatsApp, Email, SMS, Portal).

## Key Features

### 1. Template-Based Report Generation

```rust
pub struct ReportGenerator {
    template_engine: Arc<TemplateEngine>,
    pdf_generator: Arc<PDFGenerator>,
}

impl ReportGenerator {
    pub async fn generate_report(
        &self,
        order_id: Uuid,
        template_id: Uuid,
    ) -> Result<Report, Error> {
        // Fetch all data
        let order = get_order(order_id).await?;
        let patient = get_patient(order.patient_id).await?;
        let results = get_results_for_order(order_id).await?;
        let organization = get_organization(order.organization_id).await?;

        // Prepare template data
        let data = ReportData {
            organization,
            patient,
            order,
            results,
            generated_at: Utc::now(),
            report_number: generate_report_number().await?,
        };

        // Render HTML from template
        let html = self.template_engine.render(template_id, &data).await?;

        // Generate PDF
        let pdf_bytes = self.pdf_generator.generate_from_html(&html).await?;

        // Upload to S3/MinIO
        let report_url = upload_report_pdf(order_id, &pdf_bytes).await?;

        // Create report record
        let report = Report {
            id: Uuid::new_v4(),
            order_id,
            patient_id: order.patient_id,
            report_number: data.report_number,
            report_url,
            template_id,
            generated_at: Utc::now(),
            status: ReportStatus::Generated,
            signature_required: true,
            signed: false,
        };

        save_report(&report).await?;

        Ok(report)
    }
}
```

### 2. Digital Signature

```rust
pub async fn sign_report(
    report_id: Uuid,
    signer_id: Uuid,
    signature: DigitalSignature,
) -> Result<Report, Error> {
    let mut report = get_report(report_id).await?;
    let signer = get_user(signer_id).await?;

    // Validate permission
    if !signer.has_role("PATHOLOGIST") {
        return Err(Error::InsufficientPermissions);
    }

    // Verify signature
    let report_hash = calculate_report_hash(&report).await?;
    let signature_valid = verify_digital_signature(
        &report_hash,
        &signature,
        &signer.public_key,
    )?;

    if !signature_valid {
        return Err(Error::InvalidSignature);
    }

    // Apply signature
    report.signed = true;
    report.signed_by = Some(signer_id);
    report.signed_at = Some(Utc::now());
    report.signature_hash = Some(signature.hash);
    report.status = ReportStatus::Signed;

    save_report(&report).await?;

    // Trigger delivery
    deliver_report(report_id).await?;

    Ok(report)
}
```

### 3. Multi-Channel Delivery

```rust
pub struct ReportDeliveryService {
    whatsapp: Arc<WhatsAppService>,
    email: Arc<EmailService>,
    sms: Arc<SMSService>,
}

impl ReportDeliveryService {
    pub async fn deliver_report(
        &self,
        report_id: Uuid,
    ) -> Result<DeliveryStatus, Error> {
        let report = get_report(report_id).await?;
        let patient = get_patient(report.patient_id).await?;
        let order = get_order(report.order_id).await?;

        let mut delivery_statuses = Vec::new();

        // 1. WhatsApp (Primary channel in India)
        if patient.preferred_communication == CommunicationChannel::WhatsApp {
            let whatsapp_result = self.deliver_via_whatsapp(&report, &patient).await;
            delivery_statuses.push(whatsapp_result);
        }

        // 2. Email (if provided)
        if let Some(email) = &patient.email {
            let email_result = self.deliver_via_email(&report, email).await;
            delivery_statuses.push(email_result);
        }

        // 3. SMS notification
        let sms_result = self.send_sms_notification(&report, &patient).await;
        delivery_statuses.push(sms_result);

        // 4. Portal (always available)
        mark_available_on_portal(report_id).await?;

        Ok(DeliveryStatus {
            report_id,
            channels: delivery_statuses,
            delivered_at: Utc::now(),
        })
    }

    async fn deliver_via_whatsapp(
        &self,
        report: &Report,
        patient: &Patient,
    ) -> Result<ChannelDelivery, Error> {
        let message = format!(
            "🏥 *Lab Report Ready*\n\n\
            Hello {}, your test report is ready!\n\n\
            Report Number: {}\n\
            Date: {}\n\n\
            📄 View your report: {}",
            patient.first_name,
            report.report_number,
            report.generated_at.format("%d %b %Y"),
            generate_report_link(report.id)
        );

        let result = self.whatsapp
            .send_document(
                &patient.mobile_number,
                &message,
                &report.report_url,
                "Lab_Report.pdf",
            )
            .await?;

        Ok(ChannelDelivery {
            channel: DeliveryChannel::WhatsApp,
            status: DeliveryStatus::Sent,
            message_id: Some(result.message_id),
            delivered_at: Utc::now(),
        })
    }
}
```

---

# 08. Billing & Payment Module

## Overview
Complete billing, invoicing, GST compliance, and payment processing with UPI/card integration.

## Key Features

### 1. Dynamic Pricing & Discounts

```rust
pub struct PricingEngine {
    db: Arc<Database>,
}

impl PricingEngine {
    pub async fn calculate_order_price(
        &self,
        order: &Order,
    ) -> Result<PriceCalculation, Error> {
        let mut subtotal = Decimal::ZERO;

        // Calculate test prices
        for test_id in &order.test_ids {
            let test = get_test(*test_id).await?;
            let price = match order.priority {
                Priority::STAT => test.base_price + test.stat_surcharge,
                Priority::URGENT => test.base_price + test.urgent_surcharge,
                Priority::ROUTINE => test.base_price,
            };
            subtotal += price;
        }

        // Profile discounts
        for profile_id in &order.profile_ids {
            let profile = get_test_profile(*profile_id).await?;
            subtotal += profile.price; // Already discounted
        }

        // Apply discounts
        let discount = self.calculate_discount(order, subtotal).await?;

        // Calculate GST (18% on lab services in India)
        let taxable_amount = subtotal - discount;
        let gst_rate = Decimal::from_str("0.18")?;
        let gst_amount = taxable_amount * gst_rate;
        let cgst = gst_amount / Decimal::TWO;
        let sgst = gst_amount / Decimal::TWO;

        let total = taxable_amount + gst_amount;

        Ok(PriceCalculation {
            subtotal,
            discount,
            taxable_amount,
            cgst,
            sgst,
            total_gst: gst_amount,
            total,
        })
    }

    async fn calculate_discount(
        &self,
        order: &Order,
        subtotal: Decimal,
    ) -> Result<Decimal, Error> {
        // Check for applicable discounts
        let discounts = get_active_discounts(order.organization_id).await?;

        let mut best_discount = Decimal::ZERO;

        for discount in discounts {
            let amount = match discount.discount_type {
                DiscountType::Percentage => {
                    subtotal * (discount.value / Decimal::from(100))
                }
                DiscountType::FixedAmount => {
                    discount.value
                }
            };

            if amount > best_discount {
                best_discount = amount;
            }
        }

        Ok(best_discount)
    }
}
```

### 2. Payment Gateway Integration

```rust
pub struct PaymentGateway {
    razorpay: Arc<RazorpayClient>,
}

impl PaymentGateway {
    pub async fn initiate_payment(
        &self,
        invoice: &Invoice,
    ) -> Result<PaymentInitiation, Error> {
        // Create Razorpay order
        let razorpay_order = self.razorpay.create_order(CreateOrderRequest {
            amount: (invoice.total_amount * Decimal::from(100)).to_u64().unwrap(), // Paise
            currency: "INR".to_string(),
            receipt: invoice.invoice_number.clone(),
            notes: hashmap! {
                "invoice_id" => invoice.id.to_string(),
                "patient_id" => invoice.patient_id.to_string(),
            },
        }).await?;

        Ok(PaymentInitiation {
            order_id: razorpay_order.id,
            amount: invoice.total_amount,
            currency: "INR",
            payment_url: generate_payment_url(&razorpay_order.id),
        })
    }

    pub async fn verify_payment(
        &self,
        payment_id: String,
        signature: String,
    ) -> Result<PaymentVerification, Error> {
        // Verify Razorpay signature
        let is_valid = self.razorpay.verify_signature(
            &payment_id,
            &signature,
        )?;

        if !is_valid {
            return Err(Error::InvalidPaymentSignature);
        }

        // Fetch payment details
        let payment = self.razorpay.get_payment(&payment_id).await?;

        Ok(PaymentVerification {
            payment_id,
            status: payment.status,
            amount: Decimal::from(payment.amount) / Decimal::from(100),
            method: payment.method,
            verified: true,
        })
    }
}
```

### 3. E-Invoice Generation

```rust
pub async fn generate_e_invoice(
    invoice_id: Uuid,
) -> Result<EInvoice, Error> {
    let invoice = get_invoice(invoice_id).await?;
    let organization = get_organization(invoice.organization_id).await?;
    let patient = get_patient(invoice.patient_id).await?;

    // Generate IRN (Invoice Reference Number) via NIC API
    let e_invoice_request = EInvoiceRequest {
        seller: SellerDetails {
            gstin: organization.gstin,
            legal_name: organization.legal_name,
            address: organization.registered_address,
        },
        buyer: BuyerDetails {
            gstin: patient.gstin,
            legal_name: patient.full_name,
            address: patient.billing_address,
        },
        document_details: DocumentDetails {
            document_type: "INV",
            document_number: invoice.invoice_number.clone(),
            document_date: invoice.invoice_date.format("%d/%m/%Y").to_string(),
        },
        items: invoice.line_items.iter().map(|item| EInvoiceItem {
            description: item.description.clone(),
            quantity: item.quantity,
            unit_price: item.unit_price,
            total_amount: item.total_amount,
            gst_rate: 18.0,
        }).collect(),
        total_value: invoice.total_amount,
    };

    // Submit to e-invoice portal
    let response = submit_to_einvoice_portal(&e_invoice_request).await?;

    // Update invoice with IRN and QR code
    update_invoice_with_einvoice_details(
        invoice_id,
        response.irn,
        response.qr_code,
    ).await?;

    Ok(EInvoice {
        irn: response.irn,
        ack_number: response.ack_number,
        ack_date: response.ack_date,
        qr_code: response.qr_code,
    })
}
```

---

# 09. Compliance & Audit Module

## Overview
NABL ISO 15189:2022 compliance, document control, comprehensive audit trails, and CAPA management.

## Key Features

### 1. Audit Trail System

```rust
pub struct AuditLogger {
    db: Arc<Database>,
    kafka: Arc<KafkaProducer>,
}

impl AuditLogger {
    pub async fn log_event(
        &self,
        event: AuditEvent,
    ) -> Result<(), Error> {
        let log_entry = AuditLogEntry {
            id: Uuid::new_v4(),
            event_type: event.event_type,
            entity_type: event.entity_type,
            entity_id: event.entity_id,
            user_id: event.user_id,
            organization_id: event.organization_id,
            action: event.action,
            changes: event.changes,
            ip_address: event.ip_address,
            user_agent: event.user_agent,
            timestamp: Utc::now(),
        };

        // Store in PostgreSQL
        save_audit_log(&log_entry).await?;

        // Also publish to Kafka for real-time monitoring
        self.kafka.send("audit-events", &log_entry).await?;

        Ok(())
    }

    pub async fn query_audit_trail(
        &self,
        filter: AuditFilter,
    ) -> Result<Vec<AuditLogEntry>, Error> {
        let mut query = "SELECT * FROM audit_log WHERE 1=1".to_string();

        if let Some(user_id) = filter.user_id {
            query.push_str(&format!(" AND user_id = '{}'", user_id));
        }

        if let Some(entity_type) = filter.entity_type {
            query.push_str(&format!(" AND entity_type = '{}'", entity_type));
        }

        if let Some(from_date) = filter.from_date {
            query.push_str(&format!(" AND timestamp >= '{}'", from_date));
        }

        query.push_str(" ORDER BY timestamp DESC LIMIT 1000");

        let results = sqlx::query_as::<_, AuditLogEntry>(&query)
            .fetch_all(&self.db)
            .await?;

        Ok(results)
    }
}

pub enum AuditEventType {
    PatientCreated,
    PatientUpdated,
    ResultVerified,
    ResultAmended,
    ReportSigned,
    QCViolation,
    CalibrationPerformed,
    UserLogin,
    PermissionChanged,
    DataExported,
}
```

### 2. Document Control System

```rust
pub struct DocumentControlSystem {
    db: Arc<Database>,
    storage: Arc<S3Storage>,
}

impl DocumentControlSystem {
    pub async fn create_document(
        &self,
        doc: CreateDocumentInput,
    ) -> Result<Document, Error> {
        let document = Document {
            id: Uuid::new_v4(),
            document_number: generate_document_number(&doc.document_type).await?,
            title: doc.title,
            document_type: doc.document_type,
            version: "1.0".to_string(),
            status: DocumentStatus::Draft,
            created_by: doc.created_by,
            created_at: Utc::now(),
            effective_date: None,
            review_date: None,
            next_review_due: None,
            approval_workflow_id: Some(create_approval_workflow(&doc).await?),
        };

        save_document(&document).await?;

        Ok(document)
    }

    pub async fn approve_document(
        &self,
        document_id: Uuid,
        approver_id: Uuid,
    ) -> Result<Document, Error> {
        let mut document = get_document(document_id).await?;

        // Record approval
        record_approval(DocumentApproval {
            document_id,
            approver_id,
            approved_at: Utc::now(),
            comments: None,
        }).await?;

        // Check if all approvals received
        let workflow = get_approval_workflow(document.approval_workflow_id.unwrap()).await?;
        let approvals = get_approvals(document_id).await?;

        if approvals.len() >= workflow.required_approvals {
            document.status = DocumentStatus::Approved;
            document.effective_date = Some(Utc::now());
            document.next_review_due = Some(Utc::now() + Duration::days(365));
            save_document(&document).await?;
        }

        Ok(document)
    }

    pub async fn create_new_version(
        &self,
        document_id: Uuid,
        changes: String,
        updated_by: Uuid,
    ) -> Result<Document, Error> {
        let current = get_document(document_id).await?;

        // Archive current version
        archive_document_version(&current).await?;

        // Create new version
        let new_version = increment_version(&current.version);

        let new_document = Document {
            id: Uuid::new_v4(),
            document_number: current.document_number,
            title: current.title,
            document_type: current.document_type,
            version: new_version,
            status: DocumentStatus::Draft,
            created_by: updated_by,
            created_at: Utc::now(),
            change_summary: Some(changes),
            previous_version_id: Some(current.id),
            ..Default::default()
        };

        save_document(&new_document).await?;

        Ok(new_document)
    }
}
```

### 3. CAPA Management

```rust
pub struct CAPASystem {
    db: Arc<Database>,
}

impl CAPASystem {
    pub async fn create_capa(
        &self,
        capa: CreateCAPAInput,
    ) -> Result<CAPA, Error> {
        let capa_record = CAPA {
            id: Uuid::new_v4(),
            capa_number: generate_capa_number().await?,
            title: capa.title,
            description: capa.description,
            category: capa.category, // Corrective or Preventive
            source: capa.source,     // QC Violation, Audit, Incident
            root_cause: None,
            corrective_action: None,
            preventive_action: None,
            assigned_to: capa.assigned_to,
            due_date: capa.due_date,
            status: CAPAStatus::Open,
            created_by: capa.created_by,
            created_at: Utc::now(),
        };

        save_capa(&capa_record).await?;

        // Create notification
        notify_assignee(&capa_record).await?;

        Ok(capa_record)
    }

    pub async fn update_capa_status(
        &self,
        capa_id: Uuid,
        status: CAPAStatus,
        notes: String,
    ) -> Result<CAPA, Error> {
        let mut capa = get_capa(capa_id).await?;

        capa.status = status;

        match status {
            CAPAStatus::InProgress => {
                capa.started_at = Some(Utc::now());
            }
            CAPAStatus::Completed => {
                capa.completed_at = Some(Utc::now());
                capa.effectiveness_check_due = Some(Utc::now() + Duration::days(30));
            }
            CAPAStatus::Verified => {
                capa.verified_at = Some(Utc::now());
            }
            _ => {}
        }

        // Add status update to history
        add_capa_history(CAPAHistory {
            capa_id,
            status,
            notes,
            updated_at: Utc::now(),
        }).await?;

        save_capa(&capa).await?;

        Ok(capa)
    }
}
```

---

# 10. Analytics & Reporting Module

## Overview
Business intelligence dashboards, KPIs, TAT analytics, and custom report builder.

## Key Features

### 1. Role-Based Dashboards

```rust
pub async fn get_dashboard(
    user_id: Uuid,
    role: UserRole,
) -> Result<Dashboard, Error> {
    let user = get_user(user_id).await?;

    let dashboard = match role {
        UserRole::LabDirector => generate_director_dashboard(user).await?,
        UserRole::Pathologist => generate_pathologist_dashboard(user).await?,
        UserRole::LabTechnician => generate_technician_dashboard(user).await?,
        UserRole::FrontDesk => generate_frontdesk_dashboard(user).await?,
        _ => generate_basic_dashboard(user).await?,
    };

    Ok(dashboard)
}

async fn generate_director_dashboard(
    user: &User,
) -> Result<Dashboard, Error> {
    let org_id = user.organization_id;

    // Key metrics
    let today_samples = count_samples_today(org_id).await?;
    let pending_results = count_pending_results(org_id).await?;
    let tat_compliance = calculate_tat_compliance_rate(org_id).await?;
    let revenue_today = calculate_revenue_today(org_id).await?;

    // Trends
    let sample_trend = get_sample_trend_7days(org_id).await?;
    let revenue_trend = get_revenue_trend_30days(org_id).await?;

    // Equipment status
    let equipment_status = get_equipment_status_summary(org_id).await?;

    // Alerts
    let critical_alerts = get_critical_alerts(org_id).await?;

    Ok(Dashboard {
        user_role: UserRole::LabDirector,
        metrics: vec![
            Metric { name: "Today's Samples", value: today_samples.to_string(), trend: sample_trend },
            Metric { name: "Pending Results", value: pending_results.to_string(), trend: None },
            Metric { name: "TAT Compliance", value: format!("{}%", tat_compliance), trend: None },
            Metric { name: "Today's Revenue", value: format!("₹{}", revenue_today), trend: revenue_trend },
        ],
        charts: vec![
            Chart { type_: ChartType::Line, title: "Sample Volume (7 Days)", data: sample_trend },
            Chart { type_: ChartType::Bar, title: "Revenue (30 Days)", data: revenue_trend },
            Chart { type_: ChartType::Pie, title: "Equipment Status", data: equipment_status },
        ],
        alerts: critical_alerts,
    })
}
```

### 2. TAT Analytics

```rust
pub async fn analyze_tat_performance(
    org_id: Uuid,
    period: DateRange,
) -> Result<TATAnalytics, Error> {
    // Calculate TAT for completed orders
    let query = r#"
        SELECT
            o.id,
            o.created_at as ordered_at,
            o.actual_completion_at as completed_at,
            o.priority,
            EXTRACT(EPOCH FROM (o.actual_completion_at - o.created_at)) / 3600 as tat_hours,
            o.estimated_completion_at
        FROM orders o
        WHERE o.organization_id = $1
          AND o.status = 'COMPLETED'
          AND o.created_at BETWEEN $2 AND $3
    "#;

    let results = sqlx::query_as::<_, TATRecord>(query)
        .bind(org_id)
        .bind(period.start)
        .bind(period.end)
        .fetch_all(&db)
        .await?;

    // Calculate statistics
    let total_orders = results.len();
    let tat_hours: Vec<f64> = results.iter().map(|r| r.tat_hours).collect();

    let mean_tat = calculate_mean(&tat_hours);
    let median_tat = calculate_median(&tat_hours);
    let p95_tat = calculate_percentile(&tat_hours, 0.95);

    // TAT compliance
    let on_time = results.iter()
        .filter(|r| r.completed_at <= r.estimated_completion_at)
        .count();
    let compliance_rate = (on_time as f64 / total_orders as f64) * 100.0;

    // Breakdown by priority
    let stat_tat = calculate_mean_for_priority(&results, Priority::STAT);
    let urgent_tat = calculate_mean_for_priority(&results, Priority::URGENT);
    let routine_tat = calculate_mean_for_priority(&results, Priority::ROUTINE);

    Ok(TATAnalytics {
        period,
        total_orders,
        mean_tat_hours: mean_tat,
        median_tat_hours: median_tat,
        p95_tat_hours: p95_tat,
        compliance_rate,
        by_priority: vec![
            PriorityTAT { priority: Priority::STAT, mean_tat_hours: stat_tat },
            PriorityTAT { priority: Priority::URGENT, mean_tat_hours: urgent_tat },
            PriorityTAT { priority: Priority::ROUTINE, mean_tat_hours: routine_tat },
        ],
    })
}
```

### 3. Custom Report Builder

```rust
pub struct ReportBuilder {
    db: Arc<Database>,
}

impl ReportBuilder {
    pub async fn build_custom_report(
        &self,
        config: ReportConfig,
    ) -> Result<CustomReport, Error> {
        // Build dynamic SQL query based on config
        let query = self.build_query(&config)?;

        // Execute query
        let results = sqlx::query(&query)
            .fetch_all(&self.db)
            .await?;

        // Format results based on config
        let formatted = self.format_results(results, &config)?;

        Ok(CustomReport {
            title: config.title,
            generated_at: Utc::now(),
            data: formatted,
            chart_type: config.chart_type,
        })
    }

    fn build_query(&self, config: &ReportConfig) -> Result<String, Error> {
        let mut query = format!("SELECT {} FROM {}",
            config.fields.join(", "),
            config.table
        );

        // Add JOINs
        for join in &config.joins {
            query.push_str(&format!(" {} JOIN {} ON {}",
                join.join_type, join.table, join.condition));
        }

        // Add WHERE clause
        if !config.filters.is_empty() {
            query.push_str(" WHERE ");
            let filter_clauses: Vec<String> = config.filters.iter()
                .map(|f| format!("{} {} {}", f.field, f.operator, f.value))
                .collect();
            query.push_str(&filter_clauses.join(" AND "));
        }

        // Add GROUP BY
        if !config.group_by.is_empty() {
            query.push_str(&format!(" GROUP BY {}", config.group_by.join(", ")));
        }

        // Add ORDER BY
        if !config.order_by.is_empty() {
            query.push_str(&format!(" ORDER BY {}", config.order_by.join(", ")));
        }

        // Add LIMIT
        if let Some(limit) = config.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        Ok(query)
    }
}
```

---

# 11. Notification & Communication Module

## Overview
Multi-channel notification orchestration (WhatsApp, SMS, Email, Push) with delivery tracking.

## Key Features

### 1. WhatsApp Business API Integration

```rust
pub struct WhatsAppService {
    client: Arc<WhatsAppClient>,
    template_cache: Arc<RwLock<HashMap<String, WhatsAppTemplate>>>,
}

impl WhatsAppService {
    pub async fn send_template_message(
        &self,
        to: &str,
        template_name: &str,
        params: Vec<String>,
    ) -> Result<MessageResponse, Error> {
        let template = self.get_template(template_name).await?;

        let request = WhatsAppTemplateMessage {
            messaging_product: "whatsapp",
            to: format_phone_number(to),
            type_: "template",
            template: Template {
                name: template_name.to_string(),
                language: LanguageCode { code: "en" },
                components: vec![
                    TemplateComponent {
                        type_: "body",
                        parameters: params.into_iter().map(|p| Parameter {
                            type_: "text",
                            text: p,
                        }).collect(),
                    }
                ],
            },
        };

        let response = self.client
            .send_message(request)
            .await?;

        // Track delivery
        track_message_delivery(MessageDelivery {
            message_id: response.messages[0].id.clone(),
            channel: DeliveryChannel::WhatsApp,
            recipient: to.to_string(),
            status: DeliveryStatus::Sent,
            sent_at: Utc::now(),
        }).await?;

        Ok(response)
    }

    pub async fn send_report_document(
        &self,
        to: &str,
        report_url: &str,
        patient_name: &str,
    ) -> Result<MessageResponse, Error> {
        let message = format!(
            "🏥 *Lab Report Ready*\n\n\
            Hello {}, your test results are now available.\n\n\
            Please find your detailed lab report attached.",
            patient_name
        );

        let request = WhatsAppMediaMessage {
            messaging_product: "whatsapp",
            to: format_phone_number(to),
            type_: "document",
            document: MediaObject {
                link: report_url.to_string(),
                caption: Some(message),
                filename: Some("Lab_Report.pdf".to_string()),
            },
        };

        self.client.send_message(request).await
    }

    // Handle incoming WhatsApp messages (for 2-way communication)
    pub async fn handle_incoming_message(
        &self,
        message: IncomingWhatsAppMessage,
    ) -> Result<(), Error> {
        match message.type_.as_str() {
            "text" => {
                self.handle_text_message(message).await?;
            }
            "interactive" => {
                self.handle_button_click(message).await?;
            }
            _ => {
                // Unsupported message type
            }
        }

        Ok(())
    }
}
```

### 2. Notification Template Engine

```rust
pub struct NotificationTemplateEngine {
    templates: HashMap<String, NotificationTemplate>,
}

impl NotificationTemplateEngine {
    pub fn render_template(
        &self,
        template_name: &str,
        data: &HashMap<String, String>,
    ) -> Result<RenderedNotification, Error> {
        let template = self.templates.get(template_name)
            .ok_or(Error::TemplateNotFound)?;

        let title = self.interpolate(&template.title, data);
        let body = self.interpolate(&template.body, data);

        Ok(RenderedNotification {
            title,
            body,
            channel: template.channel,
            priority: template.priority,
        })
    }

    fn interpolate(&self, template: &str, data: &HashMap<String, String>) -> String {
        let mut result = template.to_string();

        for (key, value) in data {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }
}

// Predefined templates
pub fn load_default_templates() -> HashMap<String, NotificationTemplate> {
    hashmap! {
        "report_ready" => NotificationTemplate {
            name: "report_ready",
            title: "Lab Report Ready",
            body: "Hello {patient_name}, your test report is now available. Report ID: {report_number}",
            channel: DeliveryChannel::WhatsApp,
            priority: NotificationPriority::High,
        },
        "critical_value" => NotificationTemplate {
            name: "critical_value",
            title: "⚠️ Critical Lab Result",
            body: "URGENT: {patient_name} has a critical {test_name} result: {value} {unit}. Please review immediately.",
            channel: DeliveryChannel::SMS,
            priority: NotificationPriority::Critical,
        },
        "payment_reminder" => NotificationTemplate {
            name: "payment_reminder",
            title: "Payment Pending",
            body: "Hi {patient_name}, your payment of ₹{amount} for Order #{order_number} is pending. Pay now: {payment_link}",
            channel: DeliveryChannel::WhatsApp,
            priority: NotificationPriority::Normal,
        },
    }
}
```

### 3. Delivery Tracking & Retry Logic

```rust
pub struct NotificationDeliveryService {
    whatsapp: Arc<WhatsAppService>,
    sms: Arc<SMSService>,
    email: Arc<EmailService>,
    redis: Arc<RedisClient>,
}

impl NotificationDeliveryService {
    pub async fn send_notification(
        &self,
        notification: Notification,
    ) -> Result<(), Error> {
        // Try primary channel
        let result = match notification.primary_channel {
            DeliveryChannel::WhatsApp => {
                self.whatsapp.send_message(&notification).await
            }
            DeliveryChannel::SMS => {
                self.sms.send_message(&notification).await
            }
            DeliveryChannel::Email => {
                self.email.send_message(&notification).await
            }
            _ => Err(Error::UnsupportedChannel),
        };

        match result {
            Ok(_) => {
                self.mark_delivered(notification.id).await?;
                Ok(())
            }
            Err(e) => {
                // Fallback to secondary channel
                if let Some(fallback) = notification.fallback_channel {
                    self.send_via_fallback(notification, fallback).await?;
                } else {
                    // Schedule retry
                    self.schedule_retry(notification).await?;
                }
                Err(e)
            }
        }
    }

    async fn schedule_retry(
        &self,
        notification: Notification,
    ) -> Result<(), Error> {
        let retry_count = self.get_retry_count(notification.id).await?;

        if retry_count >= 3 {
            // Max retries reached, mark as failed
            self.mark_failed(notification.id).await?;
            return Ok(());
        }

        // Exponential backoff: 1min, 5min, 15min
        let delay_minutes = match retry_count {
            0 => 1,
            1 => 5,
            2 => 15,
            _ => 30,
        };

        // Schedule retry
        self.redis.set_with_expiry(
            &format!("notification:retry:{}", notification.id),
            &notification,
            delay_minutes * 60,
        ).await?;

        Ok(())
    }
}
```

---

# 12. Inventory Management Module

## Overview
Reagent, consumable, and equipment inventory tracking with automated reordering.

## Key Features

### 1. Stock Level Monitoring

```rust
pub struct InventoryMonitor {
    db: Arc<Database>,
}

impl InventoryMonitor {
    pub async fn check_stock_levels(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<StockAlert>, Error> {
        let query = r#"
            SELECT
                i.id,
                i.item_name,
                i.current_quantity,
                i.reorder_point,
                i.minimum_quantity,
                i.unit_of_measure
            FROM inventory_item i
            WHERE i.organization_id = $1
              AND i.current_quantity <= i.reorder_point
              AND i.is_active = true
            ORDER BY
                CASE
                    WHEN i.current_quantity <= i.minimum_quantity THEN 1
                    WHEN i.current_quantity <= i.reorder_point THEN 2
                    ELSE 3
                END,
                i.current_quantity ASC
        "#;

        let low_stock_items = sqlx::query_as::<_, InventoryItem>(query)
            .bind(organization_id)
            .fetch_all(&self.db)
            .await?;

        let mut alerts = Vec::new();

        for item in low_stock_items {
            let alert_level = if item.current_quantity <= item.minimum_quantity {
                AlertLevel::Critical
            } else if item.current_quantity <= item.reorder_point {
                AlertLevel::Warning
            } else {
                AlertLevel::Info
            };

            alerts.push(StockAlert {
                item_id: item.id,
                item_name: item.item_name,
                current_quantity: item.current_quantity,
                reorder_point: item.reorder_point,
                level: alert_level,
                suggested_order_quantity: calculate_order_quantity(&item).await?,
            });

            // Auto-create purchase order if critical
            if alert_level == AlertLevel::Critical {
                self.auto_create_purchase_order(&item).await?;
            }
        }

        Ok(alerts)
    }

    async fn auto_create_purchase_order(
        &self,
        item: &InventoryItem,
    ) -> Result<PurchaseOrder, Error> {
        let vendor = get_preferred_vendor(item.id).await?;
        let order_quantity = calculate_order_quantity(item).await?;

        let po = PurchaseOrder {
            id: Uuid::new_v4(),
            po_number: generate_po_number().await?,
            vendor_id: vendor.id,
            organization_id: item.organization_id,
            items: vec![PurchaseOrderItem {
                inventory_item_id: item.id,
                quantity: order_quantity,
                unit_price: item.last_purchase_price,
                total_amount: item.last_purchase_price * Decimal::from(order_quantity),
            }],
            status: POStatus::Draft,
            created_at: Utc::now(),
        };

        save_purchase_order(&po).await?;

        // Notify purchasing department
        notify_purchasing_team(&po).await?;

        Ok(po)
    }
}
```

### 2. Consumption Tracking

```rust
pub async fn record_consumption(
    consumption: StockConsumption,
) -> Result<(), Error> {
    // Deduct from current stock
    let mut item = get_inventory_item(consumption.item_id).await?;

    if item.current_quantity < consumption.quantity {
        return Err(Error::InsufficientStock);
    }

    item.current_quantity -= consumption.quantity;
    save_inventory_item(&item).await?;

    // Record transaction
    let transaction = StockTransaction {
        id: Uuid::new_v4(),
        item_id: consumption.item_id,
        transaction_type: TransactionType::Consumption,
        quantity: consumption.quantity,
        reference_type: consumption.reference_type, // Test, QC, Calibration
        reference_id: consumption.reference_id,
        performed_by: consumption.consumed_by,
        timestamp: Utc::now(),
    };

    save_stock_transaction(&transaction).await?;

    // Check if reorder needed
    if item.current_quantity <= item.reorder_point {
        create_reorder_alert(item.id).await?;
    }

    Ok(())
}
```

### 3. Expiry Management

```rust
pub async fn check_expiring_items(
    organization_id: Uuid,
    days_ahead: i32,
) -> Result<Vec<ExpiryAlert>, Error> {
    let threshold_date = Utc::now() + Duration::days(days_ahead as i64);

    let query = r#"
        SELECT
            i.id,
            i.item_name,
            i.batch_number,
            i.expiry_date,
            i.current_quantity,
            EXTRACT(DAY FROM (i.expiry_date - NOW())) as days_to_expiry
        FROM inventory_item i
        WHERE i.organization_id = $1
          AND i.expiry_date IS NOT NULL
          AND i.expiry_date <= $2
          AND i.current_quantity > 0
        ORDER BY i.expiry_date ASC
    "#;

    let expiring_items = sqlx::query_as::<_, ExpiringItem>(query)
        .bind(organization_id)
        .bind(threshold_date)
        .fetch_all(&db)
        .await?;

    let alerts: Vec<ExpiryAlert> = expiring_items.into_iter().map(|item| {
        let urgency = if item.days_to_expiry <= 7 {
            ExpiryUrgency::Critical
        } else if item.days_to_expiry <= 30 {
            ExpiryUrgency::High
        } else {
            ExpiryUrgency::Medium
        };

        ExpiryAlert {
            item_id: item.id,
            item_name: item.item_name,
            batch_number: item.batch_number,
            expiry_date: item.expiry_date,
            days_to_expiry: item.days_to_expiry,
            quantity: item.current_quantity,
            urgency,
            recommended_action: if item.days_to_expiry <= 7 {
                "Do not use. Dispose immediately."
            } else {
                "Use before ordering new stock"
            }.to_string(),
        }
    }).collect();

    Ok(alerts)
}
```

---

## Summary

This document has provided comprehensive specifications for all 9 remaining core operational modules:

- **Equipment Management**: HL7/ASTM integration, calibration, maintenance
- **Result Management**: AI auto-verification, delta checks, multi-level review
- **Quality Control**: Westgard rules, QC lot management, Levy-Jennings charts
- **Report Management**: Template-based generation, digital signature, multi-channel delivery
- **Billing & Payment**: Dynamic pricing, GST, UPI/card integration, e-invoicing
- **Compliance & Audit**: Audit trails, document control, CAPA management
- **Analytics & Reporting**: Role-based dashboards, TAT analytics, custom reports
- **Notification & Communication**: WhatsApp, SMS, email with delivery tracking
- **Inventory Management**: Stock monitoring, auto-reordering, expiry tracking

Each module integrates seamlessly with others through event-driven architecture, maintaining NABL compliance and optimizing for the Indian healthcare market.

---

**Document Version:** 1.0
**Last Updated:** 2025-11-05
**Status:** Complete
