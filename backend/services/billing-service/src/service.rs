use crate::domain::*;
use crate::repository::*;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{Local, NaiveDate};
use std::str::FromStr;
use common::pagination::PaginationParams;

#[derive(Debug)]
pub enum BillingError {
    NotFound(String),
    ValidationError(String),
    InvoiceAlreadyPaid,
    InvoiceAlreadyCancelled,
    PaymentExceedsOutstanding,
    InvalidDiscountScheme,
    InsuranceClaimAlreadySubmitted,
    CreditNoteExceedsInvoice,
    DatabaseError(String),
}

impl std::fmt::Display for BillingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::InvoiceAlreadyPaid => write!(f, "Invoice is already paid"),
            Self::InvoiceAlreadyCancelled => write!(f, "Invoice is already cancelled"),
            Self::PaymentExceedsOutstanding => write!(f, "Payment amount exceeds outstanding amount"),
            Self::InvalidDiscountScheme => write!(f, "Discount scheme is not valid or has expired"),
            Self::InsuranceClaimAlreadySubmitted => write!(f, "Insurance claim has already been submitted"),
            Self::CreditNoteExceedsInvoice => write!(f, "Credit note amount exceeds invoice amount"),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for BillingError {}

impl async_graphql::ErrorExtensions for BillingError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self))
    }
}

impl From<common::error::Error> for BillingError {
    fn from(err: common::error::Error) -> Self {
        match err {
            common::error::Error::NotFound(msg) => BillingError::NotFound(msg),
            common::error::Error::Database(e) => BillingError::DatabaseError(e.to_string()),
            _ => BillingError::DatabaseError(err.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, BillingError>;

#[derive(Clone)]
pub struct BillingService {
    invoice_repo: InvoiceRepository,
    payment_repo: PaymentRepository,
    insurance_company_repo: InsuranceCompanyRepository,
    insurance_claim_repo: InsuranceClaimRepository,
    credit_note_repo: CreditNoteRepository,
    discount_scheme_repo: DiscountSchemeRepository,
}

impl BillingService {
    pub fn new(
        invoice_repo: InvoiceRepository,
        payment_repo: PaymentRepository,
        insurance_company_repo: InsuranceCompanyRepository,
        insurance_claim_repo: InsuranceClaimRepository,
        credit_note_repo: CreditNoteRepository,
        discount_scheme_repo: DiscountSchemeRepository,
    ) -> Self {
        Self {
            invoice_repo,
            payment_repo,
            insurance_company_repo,
            insurance_claim_repo,
            credit_note_repo,
            discount_scheme_repo,
        }
    }

    // ============================================================================
    // Invoice Operations
    // ============================================================================

    /// Create a new invoice with validation
    pub async fn create_invoice(
        &self,
        input: CreateInvoiceInput,
        created_by: Uuid,
    ) -> Result<Invoice> {
        // Validate items
        if input.items.is_empty() {
            return Err(BillingError::ValidationError(
                "Invoice must have at least one item".to_string()
            ));
        }

        for item in &input.items {
            if item.unit_price <= Decimal::ZERO {
                return Err(BillingError::ValidationError(
                    "Item unit price must be greater than zero".to_string()
                ));
            }
            if let Some(qty) = item.quantity {
                if qty <= 0 {
                    return Err(BillingError::ValidationError(
                        "Item quantity must be greater than zero".to_string()
                    ));
                }
            }
        }

        // Create invoice
        let invoice = self.invoice_repo.create(input, created_by).await?;

        Ok(invoice)
    }

    /// Get invoice by ID
    pub async fn get_invoice(&self, invoice_id: Uuid) -> Result<Invoice> {
        let invoice = self.invoice_repo.find_by_id(invoice_id).await?
            .ok_or_else(|| BillingError::NotFound("Invoice not found".to_string()))?;
        Ok(invoice)
    }

    /// Get invoice by invoice number
    pub async fn get_invoice_by_number(&self, invoice_number: &str) -> Result<Invoice> {
        let invoice = self.invoice_repo.find_by_invoice_number(invoice_number).await?
            .ok_or_else(|| BillingError::NotFound("Invoice not found".to_string()))?;
        Ok(invoice)
    }

    /// List invoices with pagination
    pub async fn list_invoices(
        &self,
        filter: InvoiceFilter,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<common::pagination::Paginated<Invoice>> {
        let pagination = PaginationParams {
            page: page.unwrap_or(1),
            page_size: page_size.unwrap_or(20),
        };
        let invoices = self.invoice_repo.list(filter, pagination).await?;
        Ok(invoices)
    }

    /// Get invoices for a patient
    pub async fn get_patient_invoices(&self, patient_id: Uuid) -> Result<Vec<Invoice>> {
        let invoices = self.invoice_repo.find_by_patient(patient_id).await?;
        Ok(invoices)
    }

    /// Get invoices for an order
    pub async fn get_order_invoices(&self, order_id: Uuid) -> Result<Vec<Invoice>> {
        // find_by_order returns Option<Invoice>, so we convert it to Vec
        let invoice = self.invoice_repo.find_by_order(order_id).await?;
        let invoices = invoice.into_iter().collect();
        Ok(invoices)
    }

    /// Cancel an invoice
    pub async fn cancel_invoice(
        &self,
        invoice_id: Uuid,
        cancellation_reason: Option<String>,
        cancelled_by: Uuid,
    ) -> Result<Invoice> {
        let invoice = self.invoice_repo.find_by_id(invoice_id).await?
            .ok_or_else(|| BillingError::NotFound("Invoice not found".to_string()))?;

        // Validate invoice can be cancelled
        if invoice.invoice_status == InvoiceStatus::Cancelled {
            return Err(BillingError::InvoiceAlreadyCancelled);
        }

        if invoice.invoice_status == InvoiceStatus::Paid {
            return Err(BillingError::ValidationError(
                "Cannot cancel a paid invoice. Please issue a credit note instead.".to_string()
            ));
        }

        let paid_amount = invoice.paid_amount.unwrap_or(Decimal::ZERO);
        if paid_amount > Decimal::ZERO {
            return Err(BillingError::ValidationError(
                "Cannot cancel a partially paid invoice. Please issue a credit note instead.".to_string()
            ));
        }

        // Cancel invoice (repository only takes id)
        let cancelled_invoice = self.invoice_repo.cancel(invoice_id).await?;

        Ok(cancelled_invoice)
    }

    /// Get invoice items
    pub async fn get_invoice_items(&self, invoice_id: Uuid) -> Result<Vec<InvoiceItem>> {
        let items = self.invoice_repo.get_invoice_items(invoice_id).await?;
        Ok(items)
    }

    /// Update invoice status based on current state
    pub async fn update_invoice_status(&self, invoice_id: Uuid) -> Result<Invoice> {
        let invoice = self.invoice_repo.find_by_id(invoice_id).await?
            .ok_or_else(|| BillingError::NotFound("Invoice not found".to_string()))?;

        // Check if overdue
        if invoice.is_overdue() && invoice.invoice_status != InvoiceStatus::Overdue {
            // Update to overdue status
            let updated = self.invoice_repo.update_status(
                invoice_id,
                InvoiceStatus::Overdue,
            ).await?;
            return Ok(updated);
        }

        Ok(invoice)
    }

    // ============================================================================
    // Payment Operations
    // ============================================================================

    /// Record a payment for an invoice
    pub async fn record_payment(
        &self,
        input: CreatePaymentInput,
        created_by: Uuid,
    ) -> Result<Payment> {
        // Get invoice and validate
        let invoice = self.invoice_repo.find_by_id(input.invoice_id).await?
            .ok_or_else(|| BillingError::NotFound("Invoice not found".to_string()))?;

        if invoice.invoice_status == InvoiceStatus::Cancelled {
            return Err(BillingError::ValidationError(
                "Cannot record payment for a cancelled invoice".to_string()
            ));
        }

        if invoice.invoice_status == InvoiceStatus::Paid {
            return Err(BillingError::InvoiceAlreadyPaid);
        }

        // Validate payment amount
        let outstanding = invoice.outstanding_amount.unwrap_or(Decimal::ZERO);
        if input.payment_amount > outstanding {
            return Err(BillingError::PaymentExceedsOutstanding);
        }

        if input.payment_amount <= Decimal::ZERO {
            return Err(BillingError::ValidationError(
                "Payment amount must be greater than zero".to_string()
            ));
        }

        // Validate payment method specific fields
        match input.payment_method {
            PaymentMethod::Card => {
                if input.card_last_4_digits.is_none() {
                    return Err(BillingError::ValidationError(
                        "Card last 4 digits required for card payments".to_string()
                    ));
                }
            },
            PaymentMethod::Upi => {
                if input.upi_transaction_id.is_none() {
                    return Err(BillingError::ValidationError(
                        "UPI transaction ID required for UPI payments".to_string()
                    ));
                }
            },
            PaymentMethod::Cheque => {
                if input.cheque_number.is_none() {
                    return Err(BillingError::ValidationError(
                        "Cheque number required for cheque payments".to_string()
                    ));
                }
            },
            _ => {}
        }

        // Create payment (repository needs organization_id, patient_id, and created_by)
        let payment = self.payment_repo.create(
            input,
            invoice.organization_id,
            invoice.patient_id,
            created_by
        ).await?;

        Ok(payment)
    }

    /// Get payment by ID
    pub async fn get_payment(&self, payment_id: Uuid) -> Result<Payment> {
        let payment = self.payment_repo.find_by_id(payment_id).await?
            .ok_or_else(|| BillingError::NotFound("Payment not found".to_string()))?;
        Ok(payment)
    }

    /// List payments with pagination
    pub async fn list_payments(
        &self,
        filter: PaymentFilter,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<common::pagination::Paginated<Payment>> {
        let pagination = PaginationParams {
            page: page.unwrap_or(1),
            page_size: page_size.unwrap_or(20),
        };
        let payments = self.payment_repo.list(filter, pagination).await?;
        Ok(payments)
    }

    /// Get payments for an invoice
    pub async fn get_invoice_payments(&self, invoice_id: Uuid) -> Result<Vec<Payment>> {
        let payments = self.payment_repo.list_by_invoice(invoice_id).await?;
        Ok(payments)
    }

    /// Reconcile a payment
    pub async fn reconcile_payment(
        &self,
        payment_id: Uuid,
        reconciled_by: Uuid,
    ) -> Result<Payment> {
        let payment = self.payment_repo.find_by_id(payment_id).await?
            .ok_or_else(|| BillingError::NotFound("Payment not found".to_string()))?;

        if payment.is_reconciled.unwrap_or(false) {
            return Err(BillingError::ValidationError(
                "Payment is already reconciled".to_string()
            ));
        }

        if payment.payment_status != PaymentStatus::Success {
            return Err(BillingError::ValidationError(
                "Can only reconcile successful payments".to_string()
            ));
        }

        let reconciled_payment = self.payment_repo.reconcile(payment_id, reconciled_by).await?;

        Ok(reconciled_payment)
    }

    // ============================================================================
    // Insurance Company Operations
    // ============================================================================

    /// Create insurance company
    pub async fn create_insurance_company(
        &self,
        input: CreateInsuranceCompanyInput,
        created_by: Uuid,
    ) -> Result<InsuranceCompany> {
        // Validate inputs
        if input.company_name.is_empty() {
            return Err(BillingError::ValidationError(
                "Company name is required".to_string()
            ));
        }

        if input.company_code.is_empty() {
            return Err(BillingError::ValidationError(
                "Company code is required".to_string()
            ));
        }

        let company = self.insurance_company_repo.create(input, created_by).await?;
        Ok(company)
    }

    /// Get insurance company by ID
    pub async fn get_insurance_company(&self, company_id: Uuid) -> Result<InsuranceCompany> {
        let company = self.insurance_company_repo.find_by_id(company_id).await?
            .ok_or_else(|| BillingError::NotFound("Insurance company not found".to_string()))?;
        Ok(company)
    }

    /// List all active insurance companies
    pub async fn list_insurance_companies(&self, organization_id: Uuid) -> Result<Vec<InsuranceCompany>> {
        let companies = self.insurance_company_repo.list_by_organization(organization_id).await?;
        Ok(companies)
    }

    // ============================================================================
    // Insurance Claim Operations
    // ============================================================================

    /// Create an insurance claim
    pub async fn create_insurance_claim(
        &self,
        input: CreateInsuranceClaimInput,
        created_by: Uuid,
    ) -> Result<InsuranceClaim> {
        // Validate insurance company exists
        let _company = self.insurance_company_repo.find_by_id(input.insurance_company_id).await?;

        // Validate claim amount
        if input.claim_amount <= Decimal::ZERO {
            return Err(BillingError::ValidationError(
                "Claim amount must be greater than zero".to_string()
            ));
        }

        // Validate policy details
        if input.policy_number.is_empty() {
            return Err(BillingError::ValidationError(
                "Policy number is required".to_string()
            ));
        }

        // Create claim
        let claim = self.insurance_claim_repo.create(input, created_by).await?;

        Ok(claim)
    }

    /// Get insurance claim by ID
    pub async fn get_insurance_claim(&self, claim_id: Uuid) -> Result<InsuranceClaim> {
        let claim = self.insurance_claim_repo.find_by_id(claim_id).await?
            .ok_or_else(|| BillingError::NotFound("Insurance claim not found".to_string()))?;
        Ok(claim)
    }

    /// List insurance claims with pagination
    pub async fn list_insurance_claims(
        &self,
        filter: ClaimFilter,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<common::pagination::Paginated<InsuranceClaim>> {
        let pagination = PaginationParams {
            page: page.unwrap_or(1),
            page_size: page_size.unwrap_or(20),
        };
        let claims = self.insurance_claim_repo.list(filter, pagination).await?;
        Ok(claims)
    }

    /// Update insurance claim status
    pub async fn update_claim_status(
        &self,
        claim_id: Uuid,
        new_status: InsuranceClaimStatus,
        approved_amount: Option<Decimal>,
        rejection_reason: Option<String>,
        updated_by: Uuid,
    ) -> Result<InsuranceClaim> {
        let claim = self.insurance_claim_repo.find_by_id(claim_id).await?
            .ok_or_else(|| BillingError::NotFound("Insurance claim not found".to_string()))?;

        // Validate status transition
        match (&claim.claim_status, &new_status) {
            (InsuranceClaimStatus::Draft, InsuranceClaimStatus::Submitted) => {
                // Valid: Draft -> Submitted
            },
            (InsuranceClaimStatus::Submitted, InsuranceClaimStatus::UnderReview) => {
                // Valid: Submitted -> UnderReview
            },
            (InsuranceClaimStatus::UnderReview, InsuranceClaimStatus::Approved) => {
                // Valid: UnderReview -> Approved
                if approved_amount.is_none() {
                    return Err(BillingError::ValidationError(
                        "Approved amount required when approving claim".to_string()
                    ));
                }
                if approved_amount.unwrap() > claim.claim_amount {
                    return Err(BillingError::ValidationError(
                        "Approved amount cannot exceed claimed amount".to_string()
                    ));
                }
            },
            (InsuranceClaimStatus::UnderReview, InsuranceClaimStatus::PartiallyApproved) => {
                // Valid: UnderReview -> PartiallyApproved
                if approved_amount.is_none() {
                    return Err(BillingError::ValidationError(
                        "Approved amount required for partial approval".to_string()
                    ));
                }
                let approved = approved_amount.unwrap();
                if approved <= Decimal::ZERO || approved >= claim.claim_amount {
                    return Err(BillingError::ValidationError(
                        "Partially approved amount must be between 0 and claimed amount".to_string()
                    ));
                }
            },
            (InsuranceClaimStatus::UnderReview, InsuranceClaimStatus::Rejected) => {
                // Valid: UnderReview -> Rejected
                if rejection_reason.is_none() {
                    return Err(BillingError::ValidationError(
                        "Rejection reason required when rejecting claim".to_string()
                    ));
                }
            },
            (InsuranceClaimStatus::Approved | InsuranceClaimStatus::PartiallyApproved,
             InsuranceClaimStatus::Settled) => {
                // Valid: Approved/PartiallyApproved -> Settled
            },
            _ => {
                return Err(BillingError::ValidationError(
                    format!("Invalid status transition from {:?} to {:?}",
                            claim.claim_status, new_status)
                ));
            }
        }

        // Update claim status
        let input = UpdateClaimStatusInput {
            id: claim_id,
            claim_status: new_status,
            approved_amount,
            rejection_reason,
        };
        let updated_claim = self.insurance_claim_repo.update_status(input, updated_by).await?;

        Ok(updated_claim)
    }

    // ============================================================================
    // Credit Note Operations
    // ============================================================================

    /// Create a credit note for an invoice
    pub async fn create_credit_note(
        &self,
        input: CreateCreditNoteInput,
        created_by: Uuid,
    ) -> Result<CreditNote> {
        // Get and validate invoice
        let invoice = self.invoice_repo.find_by_id(input.invoice_id).await?
            .ok_or_else(|| BillingError::NotFound("Invoice not found".to_string()))?;

        if invoice.invoice_status == InvoiceStatus::Cancelled {
            return Err(BillingError::ValidationError(
                "Cannot create credit note for cancelled invoice".to_string()
            ));
        }

        // Validate credit amount
        if input.credit_amount <= Decimal::ZERO {
            return Err(BillingError::ValidationError(
                "Credit amount must be greater than zero".to_string()
            ));
        }

        if input.credit_amount > invoice.total_amount {
            return Err(BillingError::CreditNoteExceedsInvoice);
        }

        // Validate reason
        if input.reason.is_empty() {
            return Err(BillingError::ValidationError(
                "Credit note reason is required".to_string()
            ));
        }

        // Create credit note (repository needs organization_id, patient_id, and created_by)
        let credit_note = self.credit_note_repo.create(
            input,
            invoice.organization_id,
            invoice.patient_id,
            created_by
        ).await?;

        Ok(credit_note)
    }

    /// Get credit note by ID
    pub async fn get_credit_note(&self, credit_note_id: Uuid) -> Result<CreditNote> {
        let credit_note = self.credit_note_repo.find_by_id(credit_note_id).await?
            .ok_or_else(|| BillingError::NotFound("Credit note not found".to_string()))?;
        Ok(credit_note)
    }

    /// List credit notes for an invoice
    pub async fn get_invoice_credit_notes(&self, invoice_id: Uuid) -> Result<Vec<CreditNote>> {
        let credit_notes = self.credit_note_repo.list_by_invoice(invoice_id).await?;
        Ok(credit_notes)
    }

    // ============================================================================
    // Discount Scheme Operations
    // ============================================================================

    /// Create a discount scheme
    pub async fn create_discount_scheme(
        &self,
        input: CreateDiscountSchemeInput,
        created_by: Uuid,
    ) -> Result<DiscountScheme> {
        // Validate inputs
        if input.scheme_name.is_empty() {
            return Err(BillingError::ValidationError(
                "Scheme name is required".to_string()
            ));
        }

        // Validate discount value based on type
        if input.is_percentage.unwrap_or(true) {
            if let Some(discount_pct) = input.discount_percentage {
                if discount_pct <= Decimal::ZERO {
                    return Err(BillingError::ValidationError(
                        "Discount percentage must be greater than zero".to_string()
                    ));
                }
                if discount_pct > Decimal::from(100) {
                    return Err(BillingError::ValidationError(
                        "Percentage discount cannot exceed 100%".to_string()
                    ));
                }
            } else {
                return Err(BillingError::ValidationError(
                    "Discount percentage is required for percentage-based discounts".to_string()
                ));
            }
        } else {
            if let Some(discount_amt) = input.discount_amount {
                if discount_amt <= Decimal::ZERO {
                    return Err(BillingError::ValidationError(
                        "Discount amount must be greater than zero".to_string()
                    ));
                }
            } else {
                return Err(BillingError::ValidationError(
                    "Discount amount is required for fixed-amount discounts".to_string()
                ));
            }
        }

        // Validate date range
        if let (Some(from), Some(to)) = (input.valid_from, input.valid_to) {
            if from > to {
                return Err(BillingError::ValidationError(
                    "Valid from date must be before valid to date".to_string()
                ));
            }
        }

        let scheme = self.discount_scheme_repo.create(input, created_by).await?;
        Ok(scheme)
    }

    /// Get discount scheme by ID
    pub async fn get_discount_scheme(&self, scheme_id: Uuid) -> Result<DiscountScheme> {
        let scheme = self.discount_scheme_repo.find_by_id(scheme_id).await?
            .ok_or_else(|| BillingError::NotFound("Discount scheme not found".to_string()))?;
        Ok(scheme)
    }

    /// List all active discount schemes
    pub async fn list_active_discount_schemes(&self, organization_id: Uuid) -> Result<Vec<DiscountScheme>> {
        let schemes = self.discount_scheme_repo.list_by_organization(organization_id).await?;
        Ok(schemes)
    }

    /// Get applicable discount schemes for a patient category
    pub async fn get_applicable_discount_schemes(
        &self,
        organization_id: Uuid,
        patient_category: &str,
    ) -> Result<Vec<DiscountScheme>> {
        let all_schemes = self.discount_scheme_repo.list_by_organization(organization_id).await?;

        let today = Local::now().date_naive();
        let applicable: Vec<DiscountScheme> = all_schemes.into_iter()
            .filter(|scheme| {
                // Check date validity
                let date_valid = {
                    let after_start = scheme.valid_from
                        .map(|from| today >= from)
                        .unwrap_or(true);
                    let before_end = scheme.valid_to
                        .map(|to| today <= to)
                        .unwrap_or(true);
                    after_start && before_end
                };

                // Check usage limit
                let usage_valid = scheme.usage_limit
                    .map(|limit| scheme.usage_count.unwrap_or(0) < limit)
                    .unwrap_or(true);

                // Check patient category if specified
                let category_valid = if let Some(categories) = &scheme.patient_categories {
                    categories.as_array()
                        .map(|arr| arr.iter().any(|c|
                            c.as_str().map(|s| s == patient_category).unwrap_or(false)
                        ))
                        .unwrap_or(false)
                } else {
                    true // No category restriction
                };

                date_valid && usage_valid && category_valid
            })
            .collect();

        Ok(applicable)
    }
}
