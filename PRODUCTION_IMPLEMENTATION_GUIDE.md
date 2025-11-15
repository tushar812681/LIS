# Production Implementation Guide

**Status**: Implementation in Progress
**Date**: 2025-11-15
**Branch**: claude/find-backend-gaps-01Uf4BKKxxxbwttgrGu8hgYo

---

## Overview

This guide tracks the implementation of critical production-ready features identified in BACKEND_GAPS_ANALYSIS.md.

**Total Features**: 150+ gaps → 60-80 weeks effort
**Approach**: Implementing MVP-critical features first (3-4 months of work)
**Progress**: Infrastructure setup complete, implementing services now

---

## Phase 1: Infrastructure & Service Deployment ✓ COMPLETED

### 1.1 Docker Compose Enhancements ✓
- [x] Added API Gateway to docker-compose
- [x] Added Analytics Service to docker-compose
- [x] Added Compliance Service to docker-compose
- [x] Added Kafka + Zookeeper for event streaming
- [x] Added MinIO for object storage
- [x] Added Prometheus + Grafana for monitoring
- [x] Added Jaeger for distributed tracing
- [x] Added Sync Service (offline-first)
- [x] Added File Service (file storage)
- [x] Added Integration Service (HL7/ASTM)
- [x] Added ABDM Service (Health ID)

**Services Total**: 18 microservices (12 existing + 4 new + infrastructure)

### 1.2 Database Initialization ✓
- [x] Added lis_analytics database
- [x] Added lis_compliance database
- [x] Added lis_sync database
- [x] Added lis_file database
- [x] Added lis_integration database
- [x] Added lis_abdm database

**Total Databases**: 18 separate PostgreSQL databases

---

## Phase 2: Critical Service Implementation (IN PROGRESS)

### 2.1 Sync Service (Offline-First Architecture) 🔄 IN PROGRESS
**Priority**: CRITICAL - Core differentiator
**Effort**: 3-4 weeks
**Status**: Creating service structure

**Features to Implement**:
- [ ] Offline data queue management (IndexedDB on client, PostgreSQL + Redis on server)
- [ ] Conflict resolution engine (last-write-wins with manual resolution UI)
- [ ] Delta sync algorithm (only sync changed data)
- [ ] Background synchronization scheduler
- [ ] Network status monitoring
- [ ] Sync status API endpoints

**Implementation Plan**:
```
services/sync-service/
├── src/
│   ├── main.rs           # HTTP server + GraphQL setup
│   ├── config.rs         # Environment configuration
│   ├── domain.rs         # SyncQueue, Conflict, SyncStatus models
│   ├── repository.rs     # Database access layer
│   ├── service.rs        # Sync logic, conflict resolution
│   ├── api.rs            # GraphQL queries and mutations
│   └── scheduler.rs      # Background sync scheduler
├── migrations/
│   └── 001_init.sql      # Database schema
├── Cargo.toml
└── Dockerfile
```

**Key API Operations**:
- `queueOperation(entity, operation, data)` → Queue offline operation
- `syncPendingOperations()` → Sync all pending operations
- `resolveConflict(conflictId, resolution)` → Manually resolve conflict
- `getSyncStatus(entityType)` → Get sync status by entity

---

### 2.2 File Service (MinIO/S3 Integration) ⏳ PENDING
**Priority**: HIGH - Required for reports, documents, images
**Effort**: 2 weeks

**Features to Implement**:
- [ ] File upload with validation (size, type limits)
- [ ] Presigned URL generation for secure downloads
- [ ] File access control (permission checking)
- [ ] File versioning
- [ ] Automatic cleanup of old files
- [ ] PDF, image, document storage

**MinIO Integration**:
```rust
// services/file-service/src/minio_client.rs
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;

pub struct MinIOClient {
    client: Client,
    bucket_name: String,
}

impl MinIOClient {
    pub async fn upload_file(&self, key: &str, data: Vec<u8>) -> Result<String>;
    pub async fn get_presigned_url(&self, key: &str, expiry_secs: u64) -> Result<String>;
    pub async fn delete_file(&self, key: &str) -> Result<()>;
}
```

**Key API Operations**:
- `uploadFile(file, metadata)` → Upload file, return file ID
- `getFile(fileId)` → Get file metadata
- `getPresignedUrl(fileId, expirySeconds)` → Get download URL
- `deleteFile(fileId)` → Delete file (soft delete)

---

### 2.3 Integration Service (HL7/ASTM) ⏳ PENDING
**Priority**: HIGH - Equipment connectivity
**Effort**: 4 weeks

**Features to Implement**:
- [ ] HL7 v2.5 message parser (ADT, ORM, ORU, QRY, DSR)
- [ ] ASTM E1381/E1394 frame parser
- [ ] Bidirectional TCP/IP communication
- [ ] Equipment adapter framework
- [ ] Message transformation engine
- [ ] Order sending to equipment (ORM^O01)
- [ ] Result receiving from equipment (ORU^R01)

**HL7 Message Structure**:
```rust
// services/integration-service/src/hl7/parser.rs
pub enum HL7Message {
    ORM(OrderMessage),      // Send test orders to equipment
    ORU(ResultMessage),     // Receive results from equipment
    ADT(PatientMessage),    // Patient demographics
    QRY(QueryMessage),      // Query patient/order
}

pub struct HL7Parser {
    pub fn parse_message(&self, raw: &str) -> Result<HL7Message>;
    pub fn serialize_message(&self, msg: HL7Message) -> Result<String>;
}
```

**Equipment Adapters**:
- [ ] Generic HL7 adapter
- [ ] Roche Cobas adapter (hematology)
- [ ] Abbott Architect adapter (chemistry)
- [ ] Beckman Coulter adapter

---

### 2.4 ABDM Service (Ayushman Bharat Health ID) ⏳ PENDING
**Priority**: HIGH - Government mandate for India
**Effort**: 3 weeks

**Features to Implement**:
- [ ] ABDM Health ID creation via Aadhaar
- [ ] Health ID linking to patient records
- [ ] Consent request/grant workflow
- [ ] FHIR R4 resource mapping (Patient, Observation, DiagnosticReport)
- [ ] Health Information Provider (HIP) interface
- [ ] Health Information Exchange

**ABDM Integration Flow**:
```
1. Patient provides Aadhaar number
2. Send OTP to patient mobile
3. Verify OTP
4. Create Health ID (14-digit)
5. Link Health ID to patient record
6. Generate consent request for data sharing
7. Patient grants consent
8. Export lab results as FHIR resources
```

**FHIR Resource Mapping**:
```rust
// services/abdm-service/src/fhir/converter.rs
pub struct FHIRConverter;

impl FHIRConverter {
    pub fn patient_to_fhir(&self, patient: Patient) -> fhir::Patient;
    pub fn result_to_observation(&self, result: Result) -> fhir::Observation;
    pub fn report_to_diagnostic_report(&self, report: Report) -> fhir::DiagnosticReport;
}
```

---

## Phase 3: Enhancement of Existing Services (IN PROGRESS)

### 3.1 Notification Service - WhatsApp Integration ⏳ NEXT
**Priority**: CRITICAL - Primary communication channel for India
**Effort**: 2 weeks

**Implementation Location**: `services/notification-service/src/`

**Files to Create**:
```
services/notification-service/src/
├── whatsapp/
│   ├── mod.rs
│   ├── client.rs         # WhatsApp Business API client
│   ├── templates.rs      # Template management
│   └── webhook.rs        # Delivery status webhooks
```

**WhatsApp Business API Client**:
```rust
// services/notification-service/src/whatsapp/client.rs
use reqwest::Client;

pub struct WhatsAppClient {
    api_url: String,
    access_token: String,
    phone_number_id: String,
    http_client: Client,
}

impl WhatsAppClient {
    pub async fn send_template_message(
        &self,
        to: &str,
        template_name: &str,
        variables: Vec<String>,
    ) -> Result<String>;  // Returns message ID

    pub async fn send_media_message(
        &self,
        to: &str,
        media_type: MediaType,  // Document, Image, Video
        media_url: &str,
        caption: Option<&str>,
    ) -> Result<String>;

    pub async fn get_delivery_status(&self, message_id: &str) -> Result<DeliveryStatus>;
}

pub enum MediaType {
    Document,  // For PDF reports
    Image,
    Video,
}

pub enum DeliveryStatus {
    Sent,
    Delivered,
    Read,
    Failed(String),
}
```

**Pre-approved Templates** (Register with Meta):
```
1. test_result_ready
   "Hello {{1}}, your test results for {{2}} are ready. Download: {{3}}"

2. critical_value_alert
   "URGENT: {{1}}, your {{2}} result is {{3}} (critical). Please consult your doctor immediately."

3. appointment_reminder
   "Reminder: Your lab appointment is on {{1}} at {{2}}. Location: {{3}}"

4. payment_confirmation
   "Payment received: ₹{{1}} for order {{2}}. Receipt: {{3}}"
```

**Webhook Handler** (for delivery status):
```rust
// services/notification-service/src/whatsapp/webhook.rs
pub async fn handle_status_webhook(payload: WhatsAppWebhook) -> Result<()> {
    match payload.entry[0].changes[0].value.statuses {
        Some(statuses) => {
            for status in statuses {
                // Update delivery status in database
                update_notification_status(status.id, status.status).await?;
            }
        }
        None => {}
    }
    Ok(())
}
```

**Integration with Report Service**:
```rust
// When report is generated, send WhatsApp notification
let pdf_url = file_service.get_presigned_url(report.file_id).await?;

whatsapp_client.send_template_message(
    &patient.mobile,
    "test_result_ready",
    vec![
        patient.name.clone(),
        report.test_name.clone(),
        pdf_url,
    ],
).await?;

// For PDF attachment
whatsapp_client.send_media_message(
    &patient.mobile,
    MediaType::Document,
    &pdf_url,
    Some("Your lab test report"),
).await?;
```

---

### 3.2 Billing Service - Payment Gateway Integration ⏳ NEXT
**Priority**: CRITICAL - Revenue collection
**Effort**: 2 weeks

**Implementation Location**: `services/billing-service/src/`

**Files to Create**:
```
services/billing-service/src/
├── payment_gateways/
│   ├── mod.rs
│   ├── razorpay.rs       # Razorpay integration (primary for India)
│   ├── stripe.rs         # Stripe integration
│   ├── payu.rs           # PayU integration
│   ├── gateway_trait.rs  # Common trait for all gateways
│   └── webhook.rs        # Payment confirmation webhooks
```

**Payment Gateway Trait**:
```rust
// services/billing-service/src/payment_gateways/gateway_trait.rs
use async_trait::async_trait;
use rust_decimal::Decimal;

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn create_payment_link(
        &self,
        amount: Decimal,
        currency: &str,
        order_id: &str,
        customer_name: &str,
        customer_email: &str,
        customer_mobile: &str,
    ) -> Result<PaymentLink>;

    async fn create_upi_qr(
        &self,
        amount: Decimal,
        order_id: &str,
    ) -> Result<Vec<u8>>;  // QR code image bytes

    async fn verify_payment(&self, payment_id: &str) -> Result<PaymentStatus>;

    async fn process_refund(
        &self,
        payment_id: &str,
        amount: Decimal,
        reason: &str,
    ) -> Result<String>;  // Returns refund ID

    async fn verify_webhook_signature(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<bool>;
}

pub struct PaymentLink {
    pub link_id: String,
    pub short_url: String,
    pub qr_code_url: Option<String>,
    pub expires_at: DateTime<Utc>,
}

pub enum PaymentStatus {
    Created,
    Authorized,
    Captured,
    Failed(String),
    Refunded,
}
```

**Razorpay Implementation** (PRIMARY for India):
```rust
// services/billing-service/src/payment_gateways/razorpay.rs
use reqwest::Client;
use base64::{Engine as _, engine::general_purpose};

pub struct RazorpayGateway {
    key_id: String,
    key_secret: String,
    http_client: Client,
    webhook_secret: String,
}

impl RazorpayGateway {
    pub fn new(key_id: String, key_secret: String, webhook_secret: String) -> Self {
        Self {
            key_id: key_id.clone(),
            key_secret: key_secret.clone(),
            http_client: Client::builder()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    let auth = general_purpose::STANDARD.encode(format!("{}:{}", key_id, key_secret));
                    headers.insert(
                        AUTHORIZATION,
                        format!("Basic {}", auth).parse().unwrap(),
                    );
                    headers
                })
                .build()
                .unwrap(),
            webhook_secret,
        }
    }

    // Implement PaymentGateway trait methods...
}

#[async_trait]
impl PaymentGateway for RazorpayGateway {
    async fn create_payment_link(
        &self,
        amount: Decimal,
        currency: &str,
        order_id: &str,
        customer_name: &str,
        customer_email: &str,
        customer_mobile: &str,
    ) -> Result<PaymentLink> {
        let amount_paise = (amount * Decimal::from(100)).to_u64().unwrap();

        let payload = json!({
            "amount": amount_paise,
            "currency": currency,
            "reference_id": order_id,
            "description": format!("Payment for Lab Order {}", order_id),
            "customer": {
                "name": customer_name,
                "email": customer_email,
                "contact": customer_mobile
            },
            "notify": {
                "sms": true,
                "email": true,
                "whatsapp": true
            },
            "reminder_enable": true,
            "callback_url": format!("{}/payment/callback", env::var("APP_URL")?),
            "callback_method": "get"
        });

        let response = self.http_client
            .post("https://api.razorpay.com/v1/payment_links")
            .json(&payload)
            .send()
            .await?
            .json::<RazorpayPaymentLinkResponse>()
            .await?;

        Ok(PaymentLink {
            link_id: response.id,
            short_url: response.short_url,
            qr_code_url: Some(response.qr_code_url),
            expires_at: DateTime::from_timestamp(response.expire_by, 0).unwrap(),
        })
    }

    async fn create_upi_qr(
        &self,
        amount: Decimal,
        order_id: &str,
    ) -> Result<Vec<u8>> {
        let amount_paise = (amount * Decimal::from(100)).to_u64().unwrap();

        let payload = json!({
            "type": "upi_qr",
            "name": format!("Order {}", order_id),
            "usage": "single_use",
            "fixed_amount": true,
            "payment_amount": amount_paise,
            "description": format!("Payment for Lab Order {}", order_id),
            "customer_id": order_id,
            "close_by": (Utc::now() + Duration::hours(24)).timestamp()
        });

        let response = self.http_client
            .post("https://api.razorpay.com/v1/payments/qr_codes")
            .json(&payload)
            .send()
            .await?
            .json::<RazorpayQRResponse>()
            .await?;

        // Download QR code image
        let qr_image = self.http_client
            .get(&response.image_url)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec();

        Ok(qr_image)
    }

    async fn verify_payment(&self, payment_id: &str) -> Result<PaymentStatus> {
        let response = self.http_client
            .get(format!("https://api.razorpay.com/v1/payments/{}", payment_id))
            .send()
            .await?
            .json::<RazorpayPaymentResponse>()
            .await?;

        Ok(match response.status.as_str() {
            "created" => PaymentStatus::Created,
            "authorized" => PaymentStatus::Authorized,
            "captured" => PaymentStatus::Captured,
            "failed" => PaymentStatus::Failed(response.error_description.unwrap_or_default()),
            "refunded" => PaymentStatus::Refunded,
            _ => PaymentStatus::Failed("Unknown status".to_string()),
        })
    }

    async fn process_refund(
        &self,
        payment_id: &str,
        amount: Decimal,
        reason: &str,
    ) -> Result<String> {
        let amount_paise = (amount * Decimal::from(100)).to_u64().unwrap();

        let payload = json!({
            "amount": amount_paise,
            "notes": {
                "reason": reason
            }
        });

        let response = self.http_client
            .post(format!("https://api.razorpay.com/v1/payments/{}/refund", payment_id))
            .json(&payload)
            .send()
            .await?
            .json::<RazorpayRefundResponse>()
            .await?;

        Ok(response.id)
    }

    async fn verify_webhook_signature(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<bool> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(self.webhook_secret.as_bytes())?;
        mac.update(payload.as_bytes());

        let expected_signature = hex::encode(mac.finalize().into_bytes());

        Ok(expected_signature == signature)
    }
}
```

**Payment Webhook Handler**:
```rust
// services/billing-service/src/payment_gateways/webhook.rs
pub async fn handle_razorpay_webhook(
    payload: String,
    signature: String,
    gateway: Arc<RazorpayGateway>,
) -> Result<()> {
    // Verify signature
    if !gateway.verify_webhook_signature(&payload, &signature).await? {
        return Err(Error::InvalidWebhookSignature);
    }

    let event: RazorpayWebhook = serde_json::from_str(&payload)?;

    match event.event.as_str() {
        "payment.captured" => {
            // Payment successful
            let payment_id = event.payload.payment.entity.id;
            let order_id = event.payload.payment.entity.notes.order_id;

            // Update invoice status
            update_invoice_payment_status(
                &order_id,
                &payment_id,
                PaymentStatus::Captured,
            ).await?;

            // Send payment confirmation via WhatsApp/Email
            send_payment_confirmation(&order_id).await?;
        }
        "payment.failed" => {
            // Payment failed
            let order_id = event.payload.payment.entity.notes.order_id;

            update_invoice_payment_status(
                &order_id,
                "",
                PaymentStatus::Failed(event.payload.payment.entity.error_description),
            ).await?;
        }
        "refund.processed" => {
            // Refund processed
            let refund_id = event.payload.refund.entity.id;
            let payment_id = event.payload.refund.entity.payment_id;

            update_refund_status(&refund_id, &payment_id).await?;
        }
        _ => {
            // Unhandled event
            tracing::warn!("Unhandled Razorpay webhook event: {}", event.event);
        }
    }

    Ok(())
}
```

**Integration in Billing Service**:
```rust
// services/billing-service/src/service.rs
pub struct BillingService {
    repository: Arc<BillingRepository>,
    razorpay: Arc<RazorpayGateway>,
    stripe: Arc<StripeGateway>,
}

impl BillingService {
    pub async fn create_payment_link(
        &self,
        invoice_id: &str,
        gateway_type: PaymentGatewayType,
    ) -> Result<PaymentLink> {
        let invoice = self.repository.get_invoice(invoice_id).await?;
        let patient = self.get_patient_details(&invoice.patient_id).await?;

        let gateway: &dyn PaymentGateway = match gateway_type {
            PaymentGatewayType::Razorpay => self.razorpay.as_ref(),
            PaymentGatewayType::Stripe => self.stripe.as_ref(),
        };

        let payment_link = gateway.create_payment_link(
            invoice.total_amount,
            "INR",
            &invoice.invoice_number,
            &patient.name,
            &patient.email,
            &patient.mobile,
        ).await?;

        // Save payment link in database
        self.repository.save_payment_link(&invoice_id, &payment_link).await?;

        Ok(payment_link)
    }

    pub async fn create_upi_qr(&self, invoice_id: &str) -> Result<Vec<u8>> {
        let invoice = self.repository.get_invoice(invoice_id).await?;

        let qr_code = self.razorpay.create_upi_qr(
            invoice.total_amount,
            &invoice.invoice_number,
        ).await?;

        // Save QR code to file service
        let file_id = upload_file_to_minio(qr_code.clone()).await?;
        self.repository.save_upi_qr(&invoice_id, &file_id).await?;

        Ok(qr_code)
    }
}
```

---

### 3.3 Billing Service - GSTN E-Invoice Integration ⏳ NEXT
**Priority**: HIGH - Legal requirement for B2B in India
**Effort**: 1-2 weeks

**Implementation Location**: `services/billing-service/src/`

**Files to Create**:
```
services/billing-service/src/
├── gstn/
│   ├── mod.rs
│   ├── client.rs         # GSTN API client
│   ├── einvoice.rs       # E-invoice generation
│   ├── irn.rs            # IRN generation
│   └── qr_code.rs        # QR code generation
```

**GSTN E-Invoice Client**:
```rust
// services/billing-service/src/gstn/client.rs
use reqwest::Client;

pub struct GSTNClient {
    base_url: String,
    username: String,
    password: String,
    gstin: String,
    http_client: Client,
    access_token: Option<String>,
}

impl GSTNClient {
    pub async fn generate_einvoice(
        &mut self,
        invoice: &Invoice,
    ) -> Result<EInvoiceResponse> {
        // Authenticate if needed
        if self.access_token.is_none() {
            self.authenticate().await?;
        }

        // Generate invoice JSON as per GSTN schema
        let einvoice_json = self.build_einvoice_json(invoice)?;

        // Generate IRN (Invoice Reference Number)
        let response = self.http_client
            .post(format!("{}/v1.03/invoice", self.base_url))
            .bearer_auth(self.access_token.as_ref().unwrap())
            .json(&einvoice_json)
            .send()
            .await?
            .json::<GSTNEInvoiceResponse>()
            .await?;

        Ok(EInvoiceResponse {
            irn: response.result.irn,
            ack_no: response.result.ack_no,
            ack_date: response.result.ack_dt,
            signed_invoice: response.result.signed_invoice,
            signed_qr_code: response.result.signed_qr_code,
        })
    }

    fn build_einvoice_json(&self, invoice: &Invoice) -> Result<serde_json::Value> {
        // Build JSON as per GSTN e-invoice schema v1.1
        Ok(json!({
            "Version": "1.1",
            "TranDtls": {
                "TaxSch": "GST",
                "SupTyp": "B2B",  // B2B, B2C, SEZWP, SEZWOP, EXPWP, EXPWOP
                "RegRev": "N",
                "IgstOnIntra": "N"
            },
            "DocDtls": {
                "Typ": "INV",  // Invoice type
                "No": invoice.invoice_number,
                "Dt": invoice.invoice_date.format("%d/%m/%Y").to_string()
            },
            "SellerDtls": {
                "Gstin": self.gstin,
                "LglNm": invoice.seller_legal_name,
                "Addr1": invoice.seller_address_line1,
                "Loc": invoice.seller_location,
                "Pin": invoice.seller_pincode,
                "Stcd": invoice.seller_state_code
            },
            "BuyerDtls": {
                "Gstin": invoice.buyer_gstin.unwrap_or_default(),
                "LglNm": invoice.buyer_name,
                "Pos": invoice.place_of_supply_state_code,
                "Addr1": invoice.buyer_address_line1,
                "Loc": invoice.buyer_location,
                "Pin": invoice.buyer_pincode,
                "Stcd": invoice.buyer_state_code
            },
            "ItemList": invoice.line_items.iter().map(|item| json!({
                "SlNo": item.line_number,
                "PrdDesc": item.description,
                "IsServc": "Y",  // Service indicator (Y for lab tests)
                "HsnCd": item.hsn_code,  // 9993 for medical services
                "Qty": item.quantity,
                "Unit": "OTH",
                "UnitPrice": item.unit_price,
                "TotAmt": item.total_amount,
                "Discount": item.discount,
                "AssAmt": item.assessable_amount,
                "GstRt": item.gst_rate,
                "IgstAmt": item.igst_amount,
                "CgstAmt": item.cgst_amount,
                "SgstAmt": item.sgst_amount,
                "TotItemVal": item.total_value
            })).collect::<Vec<_>>(),
            "ValDtls": {
                "AssVal": invoice.assessable_value,
                "CgstVal": invoice.cgst_amount,
                "SgstVal": invoice.sgst_amount,
                "IgstVal": invoice.igst_amount,
                "TotInvVal": invoice.total_invoice_value
            }
        }))
    }

    async fn authenticate(&mut self) -> Result<()> {
        let response = self.http_client
            .post(format!("{}/v1.03/authenticate", self.base_url))
            .json(&json!({
                "username": self.username,
                "password": self.password,
                "gstin": self.gstin
            }))
            .send()
            .await?
            .json::<GSTNAuthResponse>()
            .await?;

        self.access_token = Some(response.access_token);
        Ok(())
    }
}

pub struct EInvoiceResponse {
    pub irn: String,             // 64-character IRN
    pub ack_no: String,          // Acknowledgment number
    pub ack_date: DateTime<Utc>, // Acknowledgment date
    pub signed_invoice: String,   // Digitally signed invoice (Base64)
    pub signed_qr_code: String,   // QR code data (Base64)
}
```

**QR Code Generation**:
```rust
// services/billing-service/src/gstn/qr_code.rs
use qrcode::QrCode;
use image::Luma;

pub fn generate_einvoice_qr(irn: &str, einvoice_data: &EInvoiceResponse) -> Result<Vec<u8>> {
    // QR code format as per GSTN specification
    let qr_data = format!(
        "IRN:{}\nAckNo:{}\nAckDt:{}\nDocNo:{}\nDocDt:{}\nGSTIN:{}\nAmt:{}",
        einvoice_data.irn,
        einvoice_data.ack_no,
        einvoice_data.ack_date.format("%d/%m/%Y %H:%M:%S"),
        // ... other fields
    );

    let code = QrCode::new(qr_data.as_bytes())?;
    let image = code.render::<Luma<u8>>().build();

    let mut buffer = Vec::new();
    image.write_to(&mut buffer, image::ImageOutputFormat::Png)?;

    Ok(buffer)
}
```

**Integration in Billing Service**:
```rust
// services/billing-service/src/service.rs
impl BillingService {
    pub async fn generate_einvoice(&self, invoice_id: &str) -> Result<EInvoiceResponse> {
        let invoice = self.repository.get_invoice(invoice_id).await?;

        // Check if B2B transaction (GSTIN required)
        if invoice.buyer_gstin.is_none() && invoice.total_amount >= Decimal::from(50000) {
            return Err(Error::GSTINRequiredForB2B);
        }

        // Generate e-invoice via GSTN
        let mut gstn_client = GSTNClient::new(
            env::var("GSTN_API_URL")?,
            env::var("GSTN_USERNAME")?,
            env::var("GSTN_PASSWORD")?,
            env::var("GSTIN")?,
        );

        let einvoice = gstn_client.generate_einvoice(&invoice).await?;

        // Save IRN and other details
        self.repository.update_invoice_einvoice_details(
            invoice_id,
            &einvoice.irn,
            &einvoice.ack_no,
            einvoice.ack_date,
        ).await?;

        // Generate and save QR code
        let qr_code = generate_einvoice_qr(&einvoice.irn, &einvoice)?;
        let qr_file_id = upload_file_to_minio(qr_code).await?;

        self.repository.save_einvoice_qr(invoice_id, &qr_file_id).await?;

        Ok(einvoice)
    }
}
```

---

## Implementation Status Summary

### ✅ Completed
1. Docker Compose infrastructure setup (18 services)
2. Database initialization (18 databases)
3. Kafka event streaming setup
4. MinIO object storage setup
5. Observability stack (Prometheus, Grafana, Jaeger)

### 🔄 In Progress
1. Sync Service implementation (offline-first)
2. WhatsApp Business API integration
3. Razorpay payment gateway integration
4. GSTN e-invoice integration

### ⏳ Pending (High Priority)
1. File Service implementation
2. Integration Service (HL7/ASTM)
3. ABDM Service implementation
4. Stripe payment gateway
5. PayU payment gateway
6. ML auto-verification engine
7. Redis caching layer
8. Multi-language support (i18n)
9. DPDP 2023 compliance
10. Immutable audit trails
11. Database encryption

### ⏳ Pending (Medium Priority)
1. Enhanced error handling
2. Comprehensive test suite
3. CI/CD pipeline (GitHub Actions)
4. Load testing
5. Performance optimization
6. API documentation
7. Deployment documentation

---

## Next Steps

1. **Complete Sync Service** - Critical for offline-first
2. **Implement WhatsApp Integration** - Primary communication
3. **Implement Razorpay** - Primary payment gateway for India
4. **Implement GSTN** - Legal requirement
5. **Create File Service** - Required for reports and documents
6. **Create Integration Service** - Equipment connectivity
7. **Create ABDM Service** - Government mandate

---

## Testing Strategy

### Unit Tests
- Each service should have >80% code coverage
- Test business logic thoroughly
- Mock external dependencies

### Integration Tests
- Test API endpoints end-to-end
- Test database operations
- Test inter-service communication

### Load Tests
- Target: 10,000+ concurrent users
- Target: <100ms P95 response time
- Target: 10,000+ req/sec throughput

### Security Tests
- OWASP Top 10 vulnerability scanning
- Penetration testing
- Authentication/authorization testing
- Data encryption verification

---

## Deployment Checklist

### Before Production
- [ ] All critical services implemented and tested
- [ ] Database migrations verified
- [ ] Environment variables configured
- [ ] SSL/TLS certificates configured
- [ ] API rate limiting enabled
- [ ] Monitoring and alerting configured
- [ ] Backup and disaster recovery tested
- [ ] Load testing completed
- [ ] Security audit completed
- [ ] Documentation completed

### Production Deployment
- [ ] Deploy to staging environment
- [ ] Run smoke tests
- [ ] Deploy to production (blue-green)
- [ ] Monitor for errors
- [ ] Verify all integrations working
- [ ] Test critical user flows
- [ ] Enable production monitoring

---

**Status**: 30% Complete (Infrastructure ready, implementing critical services)
**Next Update**: After completing Sync, WhatsApp, and Payment Gateway services
**Estimated Completion**: 3-4 months for MVP-ready

