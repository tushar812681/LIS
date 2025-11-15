use sqlx::PgPool;
use uuid::Uuid;
use common::error::{Error, Result};
use common::pagination::{Paginated, PaginationParams};
use crate::domain::*;
use rust_decimal::Decimal;

// ============================================================================
// Invoice Repository
// ============================================================================

#[derive(Clone)]
pub struct InvoiceRepository {
    pool: PgPool,
}

impl InvoiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateInvoiceInput, created_by: Uuid) -> Result<Invoice> {
        let id = Uuid::new_v4();

        // Generate invoice number
        let invoice_number: (String,) = sqlx::query_as("SELECT generate_invoice_number()")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        // Calculate totals
        let mut subtotal = Decimal::ZERO;
        for item in &input.items {
            let qty = Decimal::from(item.quantity.unwrap_or(1));
            subtotal += item.unit_price * qty;
        }

        let discount_amount = if let Some(discount_pct) = input.discount_percentage {
            subtotal * discount_pct / Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        let taxable_amount = subtotal - discount_amount;

        // Calculate GST (assuming 18% total - 9% CGST + 9% SGST for intra-state)
        let tax_rate = Decimal::from_str_exact("18").unwrap();
        let total_tax = taxable_amount * tax_rate / Decimal::from(100);
        let cgst = total_tax / Decimal::from(2);
        let sgst = total_tax / Decimal::from(2);

        let total_amount = taxable_amount + total_tax;

        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            INSERT INTO invoice (
                id, invoice_number, organization_id, branch_id,
                patient_id, patient_name, order_id,
                invoice_date, due_date,
                subtotal_amount, discount_amount, discount_percentage,
                taxable_amount, cgst_amount, sgst_amount,
                total_tax_amount, total_amount, outstanding_amount,
                invoice_status, is_insurance_claim, insurance_company_id,
                notes, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(invoice_number.0)
        .bind(input.organization_id)
        .bind(input.branch_id)
        .bind(input.patient_id)
        .bind(&input.patient_name)
        .bind(input.order_id)
        .bind(input.invoice_date)
        .bind(input.due_date)
        .bind(subtotal)
        .bind(discount_amount)
        .bind(input.discount_percentage)
        .bind(taxable_amount)
        .bind(cgst)
        .bind(sgst)
        .bind(total_tax)
        .bind(total_amount)
        .bind(total_amount) // Outstanding = Total initially
        .bind(InvoiceStatus::Pending)
        .bind(input.is_insurance_claim.unwrap_or(false))
        .bind(input.insurance_company_id)
        .bind(&input.notes)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        // Create invoice items
        for item_input in &input.items {
            self.create_invoice_item(invoice.id, item_input).await?;
        }

        Ok(invoice)
    }

    async fn create_invoice_item(&self, invoice_id: Uuid, input: &InvoiceItemInput) -> Result<InvoiceItem> {
        let id = Uuid::new_v4();
        let qty = Decimal::from(input.quantity.unwrap_or(1));

        let subtotal = input.unit_price * qty;

        let discount_amount = if let Some(disc_pct) = input.discount_percentage {
            subtotal * disc_pct / Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        let after_discount = subtotal - discount_amount;

        let tax_amount = if let Some(tax_pct) = input.tax_percentage {
            after_discount * tax_pct / Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        let total_amount = after_discount + tax_amount;

        let item = sqlx::query_as::<_, InvoiceItem>(
            r#"
            INSERT INTO invoice_item (
                id, invoice_id, item_type, item_id, item_code, item_name,
                description, quantity, unit_price,
                discount_amount, discount_percentage,
                tax_percentage, tax_amount,
                subtotal_amount, total_amount
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(invoice_id)
        .bind(&input.item_type)
        .bind(input.item_id)
        .bind(&input.item_code)
        .bind(&input.item_name)
        .bind(&input.description)
        .bind(input.quantity.unwrap_or(1))
        .bind(input.unit_price)
        .bind(discount_amount)
        .bind(input.discount_percentage)
        .bind(input.tax_percentage)
        .bind(tax_amount)
        .bind(subtotal)
        .bind(total_amount)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(item)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Invoice>> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoice WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(invoice)
    }

    pub async fn find_by_invoice_number(&self, invoice_number: &str) -> Result<Option<Invoice>> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoice WHERE invoice_number = $1 AND is_deleted = FALSE"
        )
        .bind(invoice_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(invoice)
    }

    pub async fn find_by_order(&self, order_id: Uuid) -> Result<Option<Invoice>> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoice WHERE order_id = $1 AND is_deleted = FALSE ORDER BY created_at DESC LIMIT 1"
        )
        .bind(order_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(invoice)
    }

    pub async fn find_by_patient(&self, patient_id: Uuid) -> Result<Vec<Invoice>> {
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoice WHERE patient_id = $1 AND is_deleted = FALSE ORDER BY invoice_date DESC"
        )
        .bind(patient_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(invoices)
    }

    pub async fn list(
        &self,
        filter: InvoiceFilter,
        pagination: PaginationParams,
    ) -> Result<Paginated<Invoice>> {
        let mut query = String::from(
            "FROM invoice WHERE organization_id = $1 AND is_deleted = FALSE"
        );
        let mut bindings = vec![];

        if let Some(status) = filter.invoice_status {
            bindings.push(format!("invoice_status = '{:?}'", status));
        }
        if let Some(patient_id) = filter.patient_id {
            bindings.push(format!("patient_id = '{}'", patient_id));
        }
        if let Some(from_date) = filter.from_date {
            bindings.push(format!("invoice_date >= '{}'", from_date));
        }
        if let Some(to_date) = filter.to_date {
            bindings.push(format!("invoice_date <= '{}'", to_date));
        }
        if let Some(is_overdue) = filter.is_overdue {
            if is_overdue {
                bindings.push("invoice_status = 'OVERDUE'".to_string());
            }
        }

        if !bindings.is_empty() {
            let where_clause = format!(" AND {}", bindings.join(" AND "));
            query.push_str(&where_clause);
        }

        // Get total count
        let count_query = format!("SELECT COUNT(*) {}", query);
        let total: (i64,) = sqlx::query_as(&count_query)
            .bind(filter.organization_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        // Get paginated results
        let select_query = format!(
            "SELECT * {} ORDER BY invoice_date DESC LIMIT $2 OFFSET $3",
            query
        );

        let invoices = sqlx::query_as::<_, Invoice>(&select_query)
            .bind(filter.organization_id)
            .bind(pagination.page_size as i64)
            .bind(((pagination.page - 1) * pagination.page_size) as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(Paginated::new(
            invoices,
            &pagination,
            total.0 as u64,
        ))
    }

    pub async fn get_invoice_items(&self, invoice_id: Uuid) -> Result<Vec<InvoiceItem>> {
        let items = sqlx::query_as::<_, InvoiceItem>(
            "SELECT * FROM invoice_item WHERE invoice_id = $1 ORDER BY created_at"
        )
        .bind(invoice_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(items)
    }

    pub async fn cancel(&self, id: Uuid) -> Result<Invoice> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "UPDATE invoice SET invoice_status = $2 WHERE id = $1 AND is_deleted = FALSE RETURNING *"
        )
        .bind(id)
        .bind(InvoiceStatus::Cancelled)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(invoice)
    }

    pub async fn update_status(&self, id: Uuid, status: InvoiceStatus) -> Result<Invoice> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "UPDATE invoice SET invoice_status = $2 WHERE id = $1 AND is_deleted = FALSE RETURNING *"
        )
        .bind(id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(invoice)
    }
}

// ============================================================================
// Payment Repository
// ============================================================================

#[derive(Clone)]
pub struct PaymentRepository {
    pool: PgPool,
}

impl PaymentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreatePaymentInput, organization_id: Uuid, patient_id: Uuid, created_by: Uuid) -> Result<Payment> {
        let id = Uuid::new_v4();

        // Generate payment number
        let payment_number: (String,) = sqlx::query_as("SELECT generate_payment_number()")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        let payment = sqlx::query_as::<_, Payment>(
            r#"
            INSERT INTO payment (
                id, payment_number, organization_id, invoice_id, patient_id,
                payment_date, payment_time, payment_method, payment_amount,
                card_last_4_digits, card_type, upi_transaction_id,
                transaction_reference, bank_name, cheque_number, cheque_date,
                payment_status, notes, created_by, received_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(payment_number.0)
        .bind(organization_id)
        .bind(input.invoice_id)
        .bind(patient_id)
        .bind(input.payment_date)
        .bind(input.payment_time)
        .bind(input.payment_method)
        .bind(input.payment_amount)
        .bind(&input.card_last_4_digits)
        .bind(&input.card_type)
        .bind(&input.upi_transaction_id)
        .bind(&input.transaction_reference)
        .bind(&input.bank_name)
        .bind(&input.cheque_number)
        .bind(input.cheque_date)
        .bind(PaymentStatus::Success) // Default to success for manual entry
        .bind(&input.notes)
        .bind(created_by)
        .bind(created_by) // received_by same as created_by
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(payment)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Payment>> {
        let payment = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payment WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(payment)
    }

    pub async fn list_by_invoice(&self, invoice_id: Uuid) -> Result<Vec<Payment>> {
        let payments = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payment WHERE invoice_id = $1 ORDER BY payment_date DESC, payment_time DESC"
        )
        .bind(invoice_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(payments)
    }

    pub async fn list(
        &self,
        filter: PaymentFilter,
        pagination: PaginationParams,
    ) -> Result<Paginated<Payment>> {
        let mut query = String::from(
            "FROM payment WHERE organization_id = $1"
        );
        let mut bindings = vec![];

        if let Some(method) = filter.payment_method {
            bindings.push(format!("payment_method = '{:?}'", method));
        }
        if let Some(status) = filter.payment_status {
            bindings.push(format!("payment_status = '{:?}'", status));
        }
        if let Some(from_date) = filter.from_date {
            bindings.push(format!("payment_date >= '{}'", from_date));
        }
        if let Some(to_date) = filter.to_date {
            bindings.push(format!("payment_date <= '{}'", to_date));
        }
        if let Some(is_reconciled) = filter.is_reconciled {
            bindings.push(format!("is_reconciled = {}", is_reconciled));
        }

        if !bindings.is_empty() {
            let where_clause = format!(" AND {}", bindings.join(" AND "));
            query.push_str(&where_clause);
        }

        // Get total count
        let count_query = format!("SELECT COUNT(*) {}", query);
        let total: (i64,) = sqlx::query_as(&count_query)
            .bind(filter.organization_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        // Get paginated results
        let select_query = format!(
            "SELECT * {} ORDER BY payment_date DESC, payment_time DESC LIMIT $2 OFFSET $3",
            query
        );

        let payments = sqlx::query_as::<_, Payment>(&select_query)
            .bind(filter.organization_id)
            .bind(pagination.page_size as i64)
            .bind(((pagination.page - 1) * pagination.page_size) as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(Paginated::new(
            payments,
            &pagination,
            total.0 as u64,
        ))
    }

    pub async fn reconcile(&self, id: Uuid, reconciled_by: Uuid) -> Result<Payment> {
        let payment = sqlx::query_as::<_, Payment>(
            r#"
            UPDATE payment
            SET is_reconciled = TRUE,
                reconciled_at = NOW(),
                reconciled_by = $2
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(reconciled_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(payment)
    }
}

// ============================================================================
// Insurance Company Repository
// ============================================================================

#[derive(Clone)]
pub struct InsuranceCompanyRepository {
    pool: PgPool,
}

impl InsuranceCompanyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateInsuranceCompanyInput, created_by: Uuid) -> Result<InsuranceCompany> {
        let id = Uuid::new_v4();

        let company = sqlx::query_as::<_, InsuranceCompany>(
            r#"
            INSERT INTO insurance_company (
                id, organization_id, company_name, company_code,
                contact_person, email, phone,
                credit_period_days, discount_percentage,
                is_active, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.organization_id)
        .bind(&input.company_name)
        .bind(&input.company_code)
        .bind(&input.contact_person)
        .bind(&input.email)
        .bind(&input.phone)
        .bind(input.credit_period_days.unwrap_or(30))
        .bind(input.discount_percentage)
        .bind(true)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(company)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<InsuranceCompany>> {
        let company = sqlx::query_as::<_, InsuranceCompany>(
            "SELECT * FROM insurance_company WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(company)
    }

    pub async fn list_by_organization(&self, organization_id: Uuid) -> Result<Vec<InsuranceCompany>> {
        let companies = sqlx::query_as::<_, InsuranceCompany>(
            "SELECT * FROM insurance_company WHERE organization_id = $1 AND is_active = TRUE ORDER BY company_name"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(companies)
    }
}

// ============================================================================
// Insurance Claim Repository
// ============================================================================

#[derive(Clone)]
pub struct InsuranceClaimRepository {
    pool: PgPool,
}

impl InsuranceClaimRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateInsuranceClaimInput, created_by: Uuid) -> Result<InsuranceClaim> {
        let id = Uuid::new_v4();

        // Generate claim number
        let claim_number: (String,) = sqlx::query_as("SELECT generate_claim_number()")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        let claim = sqlx::query_as::<_, InsuranceClaim>(
            r#"
            INSERT INTO insurance_claim (
                id, claim_number, organization_id, insurance_company_id,
                patient_id, patient_name, policy_number, policy_holder_name,
                sum_insured, claim_date, claim_amount, claim_status,
                notes, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(claim_number.0)
        .bind(input.organization_id)
        .bind(input.insurance_company_id)
        .bind(input.patient_id)
        .bind(&input.patient_name)
        .bind(&input.policy_number)
        .bind(&input.policy_holder_name)
        .bind(input.sum_insured)
        .bind(input.claim_date)
        .bind(input.claim_amount)
        .bind(InsuranceClaimStatus::Draft)
        .bind(&input.notes)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(claim)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<InsuranceClaim>> {
        let claim = sqlx::query_as::<_, InsuranceClaim>(
            "SELECT * FROM insurance_claim WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(claim)
    }

    pub async fn update_status(&self, input: UpdateClaimStatusInput, updated_by: Uuid) -> Result<InsuranceClaim> {
        let claim = sqlx::query_as::<_, InsuranceClaim>(
            r#"
            UPDATE insurance_claim
            SET claim_status = $2,
                approved_amount = COALESCE($3, approved_amount),
                rejection_reason = COALESCE($4, rejection_reason),
                updated_by = $5
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.claim_status)
        .bind(input.approved_amount)
        .bind(&input.rejection_reason)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(claim)
    }

    pub async fn list(
        &self,
        filter: ClaimFilter,
        pagination: PaginationParams,
    ) -> Result<Paginated<InsuranceClaim>> {
        let mut query = String::from(
            "FROM insurance_claim WHERE organization_id = $1"
        );
        let mut bindings = vec![];

        if let Some(company_id) = filter.insurance_company_id {
            bindings.push(format!("insurance_company_id = '{}'", company_id));
        }
        if let Some(status) = filter.claim_status {
            bindings.push(format!("claim_status = '{:?}'", status));
        }
        if let Some(from_date) = filter.from_date {
            bindings.push(format!("claim_date >= '{}'", from_date));
        }
        if let Some(to_date) = filter.to_date {
            bindings.push(format!("claim_date <= '{}'", to_date));
        }

        if !bindings.is_empty() {
            let where_clause = format!(" AND {}", bindings.join(" AND "));
            query.push_str(&where_clause);
        }

        // Get total count
        let count_query = format!("SELECT COUNT(*) {}", query);
        let total: (i64,) = sqlx::query_as(&count_query)
            .bind(filter.organization_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        // Get paginated results
        let select_query = format!(
            "SELECT * {} ORDER BY claim_date DESC LIMIT $2 OFFSET $3",
            query
        );

        let claims = sqlx::query_as::<_, InsuranceClaim>(&select_query)
            .bind(filter.organization_id)
            .bind(pagination.page_size as i64)
            .bind(((pagination.page - 1) * pagination.page_size) as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(Paginated::new(
            claims,
            &pagination,
            total.0 as u64,
        ))
    }
}

// ============================================================================
// Credit Note Repository
// ============================================================================

#[derive(Clone)]
pub struct CreditNoteRepository {
    pool: PgPool,
}

impl CreditNoteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateCreditNoteInput, organization_id: Uuid, patient_id: Uuid, created_by: Uuid) -> Result<CreditNote> {
        let id = Uuid::new_v4();

        // Generate credit note number
        let credit_note_number: (String,) = sqlx::query_as("SELECT generate_credit_note_number()")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e))?;

        let credit_note = sqlx::query_as::<_, CreditNote>(
            r#"
            INSERT INTO credit_note (
                id, credit_note_number, organization_id, invoice_id, patient_id,
                credit_date, credit_amount, reason, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(credit_note_number.0)
        .bind(organization_id)
        .bind(input.invoice_id)
        .bind(patient_id)
        .bind(input.credit_date)
        .bind(input.credit_amount)
        .bind(&input.reason)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(credit_note)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<CreditNote>> {
        let credit_note = sqlx::query_as::<_, CreditNote>(
            "SELECT * FROM credit_note WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(credit_note)
    }

    pub async fn list_by_invoice(&self, invoice_id: Uuid) -> Result<Vec<CreditNote>> {
        let credit_notes = sqlx::query_as::<_, CreditNote>(
            "SELECT * FROM credit_note WHERE invoice_id = $1 ORDER BY credit_date DESC"
        )
        .bind(invoice_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(credit_notes)
    }
}

// ============================================================================
// Discount Scheme Repository
// ============================================================================

#[derive(Clone)]
pub struct DiscountSchemeRepository {
    pool: PgPool,
}

impl DiscountSchemeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateDiscountSchemeInput, created_by: Uuid) -> Result<DiscountScheme> {
        let id = Uuid::new_v4();

        let scheme = sqlx::query_as::<_, DiscountScheme>(
            r#"
            INSERT INTO discount_scheme (
                id, organization_id, scheme_name, scheme_code, description,
                discount_percentage, discount_amount, is_percentage,
                valid_from, valid_to, is_active, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.organization_id)
        .bind(&input.scheme_name)
        .bind(&input.scheme_code)
        .bind(&input.description)
        .bind(input.discount_percentage)
        .bind(input.discount_amount)
        .bind(input.is_percentage.unwrap_or(true))
        .bind(input.valid_from)
        .bind(input.valid_to)
        .bind(true)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(scheme)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<DiscountScheme>> {
        let scheme = sqlx::query_as::<_, DiscountScheme>(
            "SELECT * FROM discount_scheme WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(scheme)
    }

    pub async fn find_by_code(&self, organization_id: Uuid, scheme_code: &str) -> Result<Option<DiscountScheme>> {
        let scheme = sqlx::query_as::<_, DiscountScheme>(
            "SELECT * FROM discount_scheme WHERE organization_id = $1 AND scheme_code = $2 AND is_active = TRUE"
        )
        .bind(organization_id)
        .bind(scheme_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(scheme)
    }

    pub async fn list_by_organization(&self, organization_id: Uuid) -> Result<Vec<DiscountScheme>> {
        let schemes = sqlx::query_as::<_, DiscountScheme>(
            "SELECT * FROM discount_scheme WHERE organization_id = $1 AND is_active = TRUE ORDER BY scheme_name"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(schemes)
    }
}
