use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_graphql::{Enum, InputObject, SimpleObject};
use rust_decimal::Decimal;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "invoice_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    Draft,
    Pending,
    PartiallyPaid,
    Paid,
    Overdue,
    Cancelled,
    Refunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Pending,
    Success,
    Failed,
    Refunded,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "payment_method", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethod {
    Cash,
    Card,
    Upi,
    NetBanking,
    Cheque,
    Insurance,
    Credit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "insurance_claim_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    PartiallyApproved,
    Rejected,
    Settled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Invoice,
    Payment,
    Refund,
    CreditNote,
    DebitNote,
    Adjustment,
}

// ============================================================================
// Invoice Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Invoice {
    // Identity
    pub id: Uuid,
    pub invoice_number: String,

    // Organization
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,

    // Patient
    pub patient_id: Uuid,
    pub patient_name: Option<String>,

    // Order Reference
    pub order_id: Uuid,

    // Invoice Details
    pub invoice_date: NaiveDate,
    pub due_date: Option<NaiveDate>,

    // Amounts
    pub subtotal_amount: Decimal,
    pub discount_amount: Option<Decimal>,
    pub discount_percentage: Option<Decimal>,
    pub taxable_amount: Decimal,

    // Tax Breakdown
    pub cgst_amount: Option<Decimal>,
    pub sgst_amount: Option<Decimal>,
    pub igst_amount: Option<Decimal>,
    pub total_tax_amount: Option<Decimal>,

    // Final Amount
    pub total_amount: Decimal,
    pub paid_amount: Option<Decimal>,
    pub outstanding_amount: Option<Decimal>,

    // Status
    pub invoice_status: InvoiceStatus,

    // Insurance
    pub is_insurance_claim: Option<bool>,
    pub insurance_company_id: Option<Uuid>,
    pub insurance_claim_id: Option<Uuid>,
    pub insurance_covered_amount: Option<Decimal>,
    pub patient_payable_amount: Option<Decimal>,

    // Payment Terms
    pub payment_terms: Option<String>,
    pub credit_period_days: Option<i32>,

    // Notes
    pub notes: Option<String>,
    pub terms_and_conditions: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub is_deleted: Option<bool>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

impl Invoice {
    pub fn is_paid(&self) -> bool {
        self.invoice_status == InvoiceStatus::Paid
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            due_date < chrono::Local::now().date_naive() && !self.is_paid()
        } else {
            false
        }
    }

    pub fn days_overdue(&self) -> Option<i64> {
        if let Some(due_date) = self.due_date {
            if self.is_overdue() {
                let today = chrono::Local::now().date_naive();
                return Some((today - due_date).num_days());
            }
        }
        None
    }

    pub fn payment_percentage(&self) -> Decimal {
        if self.total_amount > Decimal::ZERO {
            let paid = self.paid_amount.unwrap_or(Decimal::ZERO);
            (paid / self.total_amount) * Decimal::from(100)
        } else {
            Decimal::ZERO
        }
    }
}

// ============================================================================
// Invoice Item Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct InvoiceItem {
    pub id: Uuid,
    pub invoice_id: Uuid,

    // Item Details
    pub item_type: String,
    pub item_id: Option<Uuid>,
    pub item_code: Option<String>,
    pub item_name: String,
    pub description: Option<String>,

    // Quantity & Rate
    pub quantity: Option<i32>,
    pub unit_price: Decimal,

    // Discount
    pub discount_amount: Option<Decimal>,
    pub discount_percentage: Option<Decimal>,

    // Tax
    pub tax_percentage: Option<Decimal>,
    pub tax_amount: Option<Decimal>,

    // Amounts
    pub subtotal_amount: Decimal,
    pub total_amount: Decimal,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
}

// ============================================================================
// Payment Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Payment {
    // Identity
    pub id: Uuid,
    pub payment_number: String,

    // Organization
    pub organization_id: Uuid,

    // Invoice Reference
    pub invoice_id: Uuid,

    // Patient
    pub patient_id: Uuid,

    // Payment Details
    pub payment_date: NaiveDate,
    pub payment_time: NaiveTime,
    pub payment_method: PaymentMethod,

    // Amount
    pub payment_amount: Decimal,

    // Payment Method Specific Details
    pub card_last_4_digits: Option<String>,
    pub card_type: Option<String>,
    pub upi_transaction_id: Option<String>,
    pub transaction_reference: Option<String>,
    pub bank_name: Option<String>,
    pub cheque_number: Option<String>,
    pub cheque_date: Option<NaiveDate>,

    // Status
    pub payment_status: PaymentStatus,

    // Reconciliation
    #[graphql(skip)]
    pub is_reconciled: Option<bool>,
    pub reconciled_at: Option<NaiveDateTime>,
    pub reconciled_by: Option<Uuid>,

    // Gateway Details
    pub gateway_name: Option<String>,
    pub gateway_transaction_id: Option<String>,
    pub gateway_response: Option<serde_json::Value>,

    // Notes
    pub notes: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub received_by: Option<Uuid>,
}

impl Payment {
    pub fn is_success(&self) -> bool {
        self.payment_status == PaymentStatus::Success
    }

    pub fn is_reconciled(&self) -> bool {
        self.is_reconciled.unwrap_or(false)
    }
}

// ============================================================================
// Insurance Company Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct InsuranceCompany {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Company Details
    pub company_name: String,
    pub company_code: String,

    // Contact Information
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,

    // TPA Details
    pub tpa_name: Option<String>,
    pub tpa_id: Option<String>,

    // Payment Terms
    pub credit_period_days: Option<i32>,
    pub payment_terms: Option<String>,

    // Discount Agreement
    pub discount_percentage: Option<Decimal>,

    // Status
    pub is_active: Option<bool>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

// ============================================================================
// Insurance Claim Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct InsuranceClaim {
    pub id: Uuid,
    pub claim_number: String,

    // Organization
    pub organization_id: Uuid,

    // Insurance Company
    pub insurance_company_id: Uuid,

    // Patient
    pub patient_id: Uuid,
    pub patient_name: Option<String>,

    // Policy Details
    pub policy_number: String,
    pub policy_holder_name: Option<String>,
    pub sum_insured: Option<Decimal>,

    // Claim Details
    pub claim_date: NaiveDate,
    pub claim_amount: Decimal,
    pub approved_amount: Option<Decimal>,
    pub settled_amount: Option<Decimal>,
    pub rejected_amount: Option<Decimal>,

    // Status
    pub claim_status: InsuranceClaimStatus,

    // Submission
    pub submitted_date: Option<NaiveDate>,
    pub submitted_by: Option<Uuid>,

    // Approval
    pub approval_date: Option<NaiveDate>,
    pub approval_reference: Option<String>,

    // Settlement
    pub settlement_date: Option<NaiveDate>,
    pub settlement_reference: Option<String>,

    // Rejection
    pub rejection_reason: Option<String>,

    // Documents
    pub documents: Option<serde_json::Value>,

    // Notes
    pub notes: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl InsuranceClaim {
    pub fn is_settled(&self) -> bool {
        self.claim_status == InsuranceClaimStatus::Settled
    }

    pub fn is_approved(&self) -> bool {
        matches!(
            self.claim_status,
            InsuranceClaimStatus::Approved | InsuranceClaimStatus::PartiallyApproved
        )
    }
}

// ============================================================================
// Credit Note Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct CreditNote {
    pub id: Uuid,
    pub credit_note_number: String,

    // Organization
    pub organization_id: Uuid,

    // Invoice Reference
    pub invoice_id: Uuid,

    // Patient
    pub patient_id: Uuid,

    // Credit Note Details
    pub credit_date: NaiveDate,
    pub credit_amount: Decimal,
    pub reason: String,

    // Status
    pub is_applied: Option<bool>,
    pub applied_date: Option<NaiveDate>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

// ============================================================================
// Discount Scheme Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct DiscountScheme {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Scheme Details
    pub scheme_name: String,
    pub scheme_code: String,
    pub description: Option<String>,

    // Discount Configuration
    pub discount_percentage: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
    pub is_percentage: Option<bool>,

    // Applicability
    pub applicable_to: Option<String>,
    pub applicable_items: Option<serde_json::Value>,

    // Patient Category
    pub patient_categories: Option<serde_json::Value>,

    // Validity
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,

    // Limits
    pub max_discount_amount: Option<Decimal>,
    pub usage_limit: Option<i32>,
    pub usage_count: Option<i32>,

    // Status
    pub is_active: Option<bool>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

impl DiscountScheme {
    pub fn is_valid(&self) -> bool {
        let today = chrono::Local::now().date_naive();

        if let (Some(from), Some(to)) = (self.valid_from, self.valid_to) {
            today >= from && today <= to
        } else {
            true
        }
    }

    pub fn has_usage_limit(&self) -> bool {
        self.usage_limit.is_some()
    }

    pub fn is_usage_exceeded(&self) -> bool {
        if let (Some(limit), Some(count)) = (self.usage_limit, self.usage_count) {
            count >= limit
        } else {
            false
        }
    }
}

// ============================================================================
// Transaction Ledger Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct TransactionLedger {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Transaction Details
    pub transaction_date: NaiveDate,
    pub transaction_time: NaiveTime,
    pub transaction_type: TransactionType,
    pub transaction_number: String,

    // Patient
    pub patient_id: Uuid,

    // Reference
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,

    // Amounts
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub balance_amount: Option<Decimal>,

    // Description
    pub description: Option<String>,

    // Metadata
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
}

// ============================================================================
// Input DTOs
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct CreateInvoiceInput {
    pub organization_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub patient_id: Uuid,
    pub patient_name: Option<String>,
    pub order_id: Uuid,

    pub invoice_date: NaiveDate,
    pub due_date: Option<NaiveDate>,

    pub items: Vec<InvoiceItemInput>,

    pub discount_percentage: Option<Decimal>,
    pub is_insurance_claim: Option<bool>,
    pub insurance_company_id: Option<Uuid>,

    pub notes: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct InvoiceItemInput {
    pub item_type: String,
    pub item_id: Option<Uuid>,
    pub item_code: Option<String>,
    pub item_name: String,
    pub description: Option<String>,
    pub quantity: Option<i32>,
    pub unit_price: Decimal,
    pub discount_percentage: Option<Decimal>,
    pub tax_percentage: Option<Decimal>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreatePaymentInput {
    pub invoice_id: Uuid,
    pub payment_date: NaiveDate,
    pub payment_time: NaiveTime,
    pub payment_method: PaymentMethod,
    pub payment_amount: Decimal,

    pub card_last_4_digits: Option<String>,
    pub card_type: Option<String>,
    pub upi_transaction_id: Option<String>,
    pub transaction_reference: Option<String>,
    pub bank_name: Option<String>,
    pub cheque_number: Option<String>,
    pub cheque_date: Option<NaiveDate>,

    pub notes: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateInsuranceClaimInput {
    pub organization_id: Uuid,
    pub insurance_company_id: Uuid,
    pub patient_id: Uuid,
    pub patient_name: Option<String>,

    pub policy_number: String,
    pub policy_holder_name: Option<String>,
    pub sum_insured: Option<Decimal>,

    pub claim_date: NaiveDate,
    pub claim_amount: Decimal,

    pub notes: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateClaimStatusInput {
    pub id: Uuid,
    pub claim_status: InsuranceClaimStatus,
    pub approved_amount: Option<Decimal>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateCreditNoteInput {
    pub invoice_id: Uuid,
    pub credit_date: NaiveDate,
    pub credit_amount: Decimal,
    pub reason: String,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateInsuranceCompanyInput {
    pub organization_id: Uuid,
    pub company_name: String,
    pub company_code: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub credit_period_days: Option<i32>,
    pub discount_percentage: Option<Decimal>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateDiscountSchemeInput {
    pub organization_id: Uuid,
    pub scheme_name: String,
    pub scheme_code: String,
    pub description: Option<String>,
    pub discount_percentage: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
    pub is_percentage: Option<bool>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
}

// ============================================================================
// Query Filters
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct InvoiceFilter {
    pub organization_id: Uuid,
    pub invoice_status: Option<InvoiceStatus>,
    pub patient_id: Option<Uuid>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub is_overdue: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct PaymentFilter {
    pub organization_id: Uuid,
    pub payment_method: Option<PaymentMethod>,
    pub payment_status: Option<PaymentStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub is_reconciled: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct ClaimFilter {
    pub organization_id: Uuid,
    pub insurance_company_id: Option<Uuid>,
    pub claim_status: Option<InsuranceClaimStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}
