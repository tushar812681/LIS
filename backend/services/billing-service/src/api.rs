use async_graphql::{Context, Object, Result as GqlResult, ID, ErrorExtensions};
use crate::domain::*;
use crate::service::BillingService;
use uuid::Uuid;
use std::str::FromStr;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ============================================================================
    // Invoice Queries
    // ============================================================================

    /// Get invoice by ID
    async fn invoice(&self, ctx: &Context<'_>, id: ID) -> GqlResult<Invoice> {
        let service = ctx.data::<BillingService>()?;
        let invoice_id = Uuid::from_str(&id)?;
        let invoice = service.get_invoice(invoice_id).await?;
        Ok(invoice)
    }

    /// Get invoice by invoice number
    async fn invoice_by_number(
        &self,
        ctx: &Context<'_>,
        invoice_number: String,
    ) -> GqlResult<Invoice> {
        let service = ctx.data::<BillingService>()?;
        let invoice = service.get_invoice_by_number(&invoice_number).await?;
        Ok(invoice)
    }

    /// List invoices with optional filters and pagination
    async fn invoices(
        &self,
        ctx: &Context<'_>,
        organization_id: ID,
        patient_id: Option<ID>,
        status: Option<InvoiceStatus>,
        from_date: Option<String>,
        to_date: Option<String>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> GqlResult<Vec<Invoice>> {
        let service = ctx.data::<BillingService>()?;

        let org_id = Uuid::from_str(&organization_id)?;
        let filter = InvoiceFilter {
            organization_id: org_id,
            invoice_status: status,
            patient_id: patient_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
            from_date: from_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            to_date: to_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            is_overdue: None,
        };

        let paginated = service.list_invoices(filter, page.map(|p| p as u32), page_size.map(|ps| ps as u32)).await.map_err(|e| e.extend())?;
        Ok(paginated.edges.into_iter().map(|e| e.node).collect())
    }

    /// Get all invoices for a patient
    async fn patient_invoices(&self, ctx: &Context<'_>, patient_id: ID) -> GqlResult<Vec<Invoice>> {
        let service = ctx.data::<BillingService>()?;
        let patient_uuid = Uuid::from_str(&patient_id)?;
        let invoices = service.get_patient_invoices(patient_uuid).await?;
        Ok(invoices)
    }

    /// Get all invoices for an order
    async fn order_invoices(&self, ctx: &Context<'_>, order_id: ID) -> GqlResult<Vec<Invoice>> {
        let service = ctx.data::<BillingService>()?;
        let order_uuid = Uuid::from_str(&order_id)?;
        let invoices = service.get_order_invoices(order_uuid).await?;
        Ok(invoices)
    }

    /// Get invoice items
    async fn invoice_items(&self, ctx: &Context<'_>, invoice_id: ID) -> GqlResult<Vec<InvoiceItem>> {
        let service = ctx.data::<BillingService>()?;
        let invoice_uuid = Uuid::from_str(&invoice_id)?;
        let items = service.get_invoice_items(invoice_uuid).await?;
        Ok(items)
    }

    // ============================================================================
    // Payment Queries
    // ============================================================================

    /// Get payment by ID
    async fn payment(&self, ctx: &Context<'_>, id: ID) -> GqlResult<Payment> {
        let service = ctx.data::<BillingService>()?;
        let payment_id = Uuid::from_str(&id)?;
        let payment = service.get_payment(payment_id).await?;
        Ok(payment)
    }

    /// List payments with optional filters and pagination
    async fn payments(
        &self,
        ctx: &Context<'_>,
        organization_id: ID,
        payment_method: Option<PaymentMethod>,
        payment_status: Option<PaymentStatus>,
        from_date: Option<String>,
        to_date: Option<String>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> GqlResult<Vec<Payment>> {
        let service = ctx.data::<BillingService>()?;

        let org_id = Uuid::from_str(&organization_id)?;
        let filter = PaymentFilter {
            organization_id: org_id,
            payment_method,
            payment_status,
            from_date: from_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            to_date: to_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            is_reconciled: None,
        };

        let paginated = service.list_payments(filter, page.map(|p| p as u32), page_size.map(|ps| ps as u32)).await.map_err(|e| e.extend())?;
        Ok(paginated.edges.into_iter().map(|e| e.node).collect())
    }

    /// Get all payments for an invoice
    async fn invoice_payments(&self, ctx: &Context<'_>, invoice_id: ID) -> GqlResult<Vec<Payment>> {
        let service = ctx.data::<BillingService>()?;
        let invoice_uuid = Uuid::from_str(&invoice_id)?;
        let payments = service.get_invoice_payments(invoice_uuid).await?;
        Ok(payments)
    }

    // ============================================================================
    // Insurance Company Queries
    // ============================================================================

    /// Get insurance company by ID
    async fn insurance_company(&self, ctx: &Context<'_>, id: ID) -> GqlResult<InsuranceCompany> {
        let service = ctx.data::<BillingService>()?;
        let company_id = Uuid::from_str(&id)?;
        let company = service.get_insurance_company(company_id).await?;
        Ok(company)
    }

    /// List all active insurance companies
    async fn insurance_companies(&self, ctx: &Context<'_>, organization_id: ID) -> GqlResult<Vec<InsuranceCompany>> {
        let service = ctx.data::<BillingService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let companies = service.list_insurance_companies(org_id).await?;
        Ok(companies)
    }

    // ============================================================================
    // Insurance Claim Queries
    // ============================================================================

    /// Get insurance claim by ID
    async fn insurance_claim(&self, ctx: &Context<'_>, id: ID) -> GqlResult<InsuranceClaim> {
        let service = ctx.data::<BillingService>()?;
        let claim_id = Uuid::from_str(&id)?;
        let claim = service.get_insurance_claim(claim_id).await?;
        Ok(claim)
    }

    /// List insurance claims with optional filters and pagination
    async fn insurance_claims(
        &self,
        ctx: &Context<'_>,
        organization_id: ID,
        insurance_company_id: Option<ID>,
        claim_status: Option<InsuranceClaimStatus>,
        from_date: Option<String>,
        to_date: Option<String>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> GqlResult<Vec<InsuranceClaim>> {
        let service = ctx.data::<BillingService>()?;

        let org_id = Uuid::from_str(&organization_id)?;
        let filter = ClaimFilter {
            organization_id: org_id,
            insurance_company_id: insurance_company_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
            claim_status,
            from_date: from_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            to_date: to_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
        };

        let paginated = service.list_insurance_claims(filter, page.map(|p| p as u32), page_size.map(|ps| ps as u32)).await.map_err(|e| e.extend())?;
        Ok(paginated.edges.into_iter().map(|e| e.node).collect())
    }

    // ============================================================================
    // Credit Note Queries
    // ============================================================================

    /// Get credit note by ID
    async fn credit_note(&self, ctx: &Context<'_>, id: ID) -> GqlResult<CreditNote> {
        let service = ctx.data::<BillingService>()?;
        let credit_note_id = Uuid::from_str(&id)?;
        let credit_note = service.get_credit_note(credit_note_id).await?;
        Ok(credit_note)
    }

    /// Get all credit notes for an invoice
    async fn invoice_credit_notes(&self, ctx: &Context<'_>, invoice_id: ID) -> GqlResult<Vec<CreditNote>> {
        let service = ctx.data::<BillingService>()?;
        let invoice_uuid = Uuid::from_str(&invoice_id)?;
        let credit_notes = service.get_invoice_credit_notes(invoice_uuid).await?;
        Ok(credit_notes)
    }

    // ============================================================================
    // Discount Scheme Queries
    // ============================================================================

    /// Get discount scheme by ID
    async fn discount_scheme(&self, ctx: &Context<'_>, id: ID) -> GqlResult<DiscountScheme> {
        let service = ctx.data::<BillingService>()?;
        let scheme_id = Uuid::from_str(&id)?;
        let scheme = service.get_discount_scheme(scheme_id).await?;
        Ok(scheme)
    }

    /// List all active discount schemes
    async fn discount_schemes(&self, ctx: &Context<'_>, organization_id: ID) -> GqlResult<Vec<DiscountScheme>> {
        let service = ctx.data::<BillingService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let schemes = service.list_active_discount_schemes(org_id).await?;
        Ok(schemes)
    }

    /// Get applicable discount schemes for a patient category
    async fn applicable_discount_schemes(
        &self,
        ctx: &Context<'_>,
        organization_id: ID,
        patient_category: String,
    ) -> GqlResult<Vec<DiscountScheme>> {
        let service = ctx.data::<BillingService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let schemes = service.get_applicable_discount_schemes(org_id, &patient_category).await?;
        Ok(schemes)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // ============================================================================
    // Invoice Mutations
    // ============================================================================

    /// Create a new invoice
    async fn create_invoice(
        &self,
        ctx: &Context<'_>,
        input: CreateInvoiceInput,
        created_by: ID,
    ) -> GqlResult<Invoice> {
        let service = ctx.data::<BillingService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let invoice = service.create_invoice(input, creator_id).await?;
        Ok(invoice)
    }

    /// Cancel an invoice
    async fn cancel_invoice(
        &self,
        ctx: &Context<'_>,
        invoice_id: ID,
        cancellation_reason: Option<String>,
        cancelled_by: ID,
    ) -> GqlResult<Invoice> {
        let service = ctx.data::<BillingService>()?;
        let invoice_uuid = Uuid::from_str(&invoice_id)?;
        let canceller_id = Uuid::from_str(&cancelled_by)?;
        let invoice = service.cancel_invoice(
            invoice_uuid,
            cancellation_reason,
            canceller_id,
        ).await?;
        Ok(invoice)
    }

    /// Update invoice status (checks for overdue)
    async fn update_invoice_status(
        &self,
        ctx: &Context<'_>,
        invoice_id: ID,
    ) -> GqlResult<Invoice> {
        let service = ctx.data::<BillingService>()?;
        let invoice_uuid = Uuid::from_str(&invoice_id)?;
        let invoice = service.update_invoice_status(invoice_uuid).await?;
        Ok(invoice)
    }

    // ============================================================================
    // Payment Mutations
    // ============================================================================

    /// Record a payment for an invoice
    async fn record_payment(
        &self,
        ctx: &Context<'_>,
        input: CreatePaymentInput,
        created_by: ID,
    ) -> GqlResult<Payment> {
        let service = ctx.data::<BillingService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let payment = service.record_payment(input, creator_id).await?;
        Ok(payment)
    }

    /// Reconcile a payment
    async fn reconcile_payment(
        &self,
        ctx: &Context<'_>,
        payment_id: ID,
        reconciled_by: ID,
    ) -> GqlResult<Payment> {
        let service = ctx.data::<BillingService>()?;
        let payment_uuid = Uuid::from_str(&payment_id)?;
        let reconciler_id = Uuid::from_str(&reconciled_by)?;
        let payment = service.reconcile_payment(payment_uuid, reconciler_id).await?;
        Ok(payment)
    }

    // ============================================================================
    // Insurance Company Mutations
    // ============================================================================

    /// Create an insurance company
    async fn create_insurance_company(
        &self,
        ctx: &Context<'_>,
        input: CreateInsuranceCompanyInput,
        created_by: ID,
    ) -> GqlResult<InsuranceCompany> {
        let service = ctx.data::<BillingService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let company = service.create_insurance_company(input, creator_id).await?;
        Ok(company)
    }

    // ============================================================================
    // Insurance Claim Mutations
    // ============================================================================

    /// Create an insurance claim
    async fn create_insurance_claim(
        &self,
        ctx: &Context<'_>,
        input: CreateInsuranceClaimInput,
        created_by: ID,
    ) -> GqlResult<InsuranceClaim> {
        let service = ctx.data::<BillingService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let claim = service.create_insurance_claim(input, creator_id).await?;
        Ok(claim)
    }

    /// Update insurance claim status
    async fn update_claim_status(
        &self,
        ctx: &Context<'_>,
        claim_id: ID,
        new_status: InsuranceClaimStatus,
        approved_amount: Option<String>,
        rejection_reason: Option<String>,
        updated_by: ID,
    ) -> GqlResult<InsuranceClaim> {
        let service = ctx.data::<BillingService>()?;
        let claim_uuid = Uuid::from_str(&claim_id)?;
        let updater_id = Uuid::from_str(&updated_by)?;

        let approved_decimal = if let Some(amount_str) = approved_amount {
            Some(rust_decimal::Decimal::from_str(&amount_str)?)
        } else {
            None
        };

        let claim = service.update_claim_status(
            claim_uuid,
            new_status,
            approved_decimal,
            rejection_reason,
            updater_id,
        ).await?;
        Ok(claim)
    }

    // ============================================================================
    // Credit Note Mutations
    // ============================================================================

    /// Create a credit note
    async fn create_credit_note(
        &self,
        ctx: &Context<'_>,
        input: CreateCreditNoteInput,
        created_by: ID,
    ) -> GqlResult<CreditNote> {
        let service = ctx.data::<BillingService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let credit_note = service.create_credit_note(input, creator_id).await?;
        Ok(credit_note)
    }

    // ============================================================================
    // Discount Scheme Mutations
    // ============================================================================

    /// Create a discount scheme
    async fn create_discount_scheme(
        &self,
        ctx: &Context<'_>,
        input: CreateDiscountSchemeInput,
        created_by: ID,
    ) -> GqlResult<DiscountScheme> {
        let service = ctx.data::<BillingService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let scheme = service.create_discount_scheme(input, creator_id).await?;
        Ok(scheme)
    }
}
