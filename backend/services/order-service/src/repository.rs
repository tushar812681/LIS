use sqlx::{PgPool, Row};
use uuid::Uuid;
use common::error::{Error, Result};
use common::types::OrderStatus;

use crate::domain::*;

// ============================================================================
// Test Catalog Repository
// ============================================================================

#[derive(Clone)]
pub struct TestCatalogRepository {
    pool: PgPool,
}

impl TestCatalogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<TestCatalog>> {
        let test = sqlx::query_as::<_, TestCatalog>(
            "SELECT * FROM test_catalog WHERE id = $1 AND is_active = TRUE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(test)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<TestCatalog>> {
        let test = sqlx::query_as::<_, TestCatalog>(
            "SELECT * FROM test_catalog WHERE test_code = $1 AND is_active = TRUE"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(test)
    }

    pub async fn search(&self, filter: TestCatalogFilter, limit: i64) -> Result<Vec<TestCatalog>> {
        let mut query = String::from(
            "SELECT * FROM test_catalog WHERE is_active = TRUE"
        );

        if filter.category_id.is_some() {
            query.push_str(" AND category_id = $1");
        }
        if filter.department.is_some() {
            query.push_str(" AND department = $2");
        }
        if let Some(search) = &filter.search_query {
            query.push_str(" AND (test_name ILIKE $3 OR test_code ILIKE $3)");
        }

        query.push_str(" ORDER BY test_name LIMIT $4");

        let mut sql_query = sqlx::query_as::<_, TestCatalog>(&query);

        if let Some(cat_id) = filter.category_id {
            sql_query = sql_query.bind(cat_id);
        }
        if let Some(dept) = &filter.department {
            sql_query = sql_query.bind(dept);
        }
        if let Some(search) = &filter.search_query {
            let search_pattern = format!("%{}%", search);
            sql_query = sql_query.bind(search_pattern);
        }

        sql_query = sql_query.bind(limit);

        let tests = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(tests)
    }

    pub async fn get_all_active(&self, limit: i64) -> Result<Vec<TestCatalog>> {
        let tests = sqlx::query_as::<_, TestCatalog>(
            "SELECT * FROM test_catalog WHERE is_active = TRUE AND is_available = TRUE ORDER BY test_name LIMIT $1"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(tests)
    }
}

// ============================================================================
// Test Panel Repository
// ============================================================================

#[derive(Clone)]
pub struct TestPanelRepository {
    pool: PgPool,
}

impl TestPanelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<TestPanel>> {
        let panel = sqlx::query_as::<_, TestPanel>(
            "SELECT * FROM test_panel WHERE id = $1 AND is_active = TRUE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(panel)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<TestPanel>> {
        let panel = sqlx::query_as::<_, TestPanel>(
            "SELECT * FROM test_panel WHERE panel_code = $1 AND is_active = TRUE"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(panel)
    }

    pub async fn get_panel_tests(&self, panel_id: Uuid) -> Result<Vec<Uuid>> {
        let test_ids: Vec<Uuid> = sqlx::query(
            "SELECT test_id FROM test_panel_item WHERE panel_id = $1 ORDER BY display_order"
        )
        .bind(panel_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?
        .iter()
        .map(|row| row.get::<Uuid, _>("test_id"))
        .collect();

        Ok(test_ids)
    }

    pub async fn get_popular_panels(&self, limit: i64) -> Result<Vec<TestPanel>> {
        let panels = sqlx::query_as::<_, TestPanel>(
            "SELECT * FROM test_panel WHERE is_active = TRUE AND is_popular = TRUE ORDER BY display_order LIMIT $1"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(panels)
    }
}

// ============================================================================
// Test Order Repository
// ============================================================================

#[derive(Clone)]
pub struct TestOrderRepository {
    pool: PgPool,
}

impl TestOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateOrderInput, org_id: Uuid, user_id: Uuid) -> Result<TestOrder> {
        input.validate()?;

        let order_number = self.generate_order_number(&org_id).await?;

        let order = sqlx::query_as::<_, TestOrder>(
            r#"
            INSERT INTO test_order (
                id, order_number, patient_id, organization_id,
                order_status, order_source, priority,
                referring_doctor_name, clinical_notes,
                home_collection_requested, collection_date_time,
                report_delivery_method, report_delivery_email, report_delivery_phone,
                created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&order_number)
        .bind(input.patient_id)
        .bind(org_id)
        .bind(OrderStatus::PendingPayment)
        .bind(&input.order_source)
        .bind(&input.priority)
        .bind(&input.referring_doctor_name)
        .bind(&input.clinical_notes)
        .bind(input.home_collection_requested)
        .bind(input.collection_date_time)
        .bind(&input.report_delivery_method)
        .bind(&input.report_delivery_email)
        .bind(&input.report_delivery_phone)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(order)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<TestOrder>> {
        let order = sqlx::query_as::<_, TestOrder>(
            "SELECT * FROM test_order WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(order)
    }

    pub async fn find_by_order_number(&self, order_number: &str) -> Result<Option<TestOrder>> {
        let order = sqlx::query_as::<_, TestOrder>(
            "SELECT * FROM test_order WHERE order_number = $1 AND is_deleted = FALSE"
        )
        .bind(order_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(order)
    }

    pub async fn find_by_patient(&self, patient_id: Uuid, limit: i64) -> Result<Vec<TestOrder>> {
        let orders = sqlx::query_as::<_, TestOrder>(
            r#"
            SELECT * FROM test_order
            WHERE patient_id = $1 AND is_deleted = FALSE
            ORDER BY order_date DESC
            LIMIT $2
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(orders)
    }

    pub async fn search(&self, filter: OrderFilter, org_id: Uuid, limit: i64) -> Result<Vec<TestOrder>> {
        let mut query = String::from(
            "SELECT * FROM test_order WHERE organization_id = $1 AND is_deleted = FALSE"
        );
        let mut param_count = 1;

        if filter.patient_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND patient_id = ${}", param_count));
        }
        if filter.order_status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND order_status = ${}", param_count));
        }
        if filter.priority.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND priority = ${}", param_count));
        }

        query.push_str(" ORDER BY order_date DESC LIMIT $");
        param_count += 1;
        query.push_str(&param_count.to_string());

        let mut sql_query = sqlx::query_as::<_, TestOrder>(&query).bind(org_id);

        if let Some(patient_id) = filter.patient_id {
            sql_query = sql_query.bind(patient_id);
        }
        if let Some(status) = filter.order_status {
            sql_query = sql_query.bind(status);
        }
        if let Some(priority) = filter.priority {
            sql_query = sql_query.bind(priority);
        }

        sql_query = sql_query.bind(limit);

        let orders = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(orders)
    }

    pub async fn update_status(&self, input: UpdateOrderStatusInput, user_id: Uuid) -> Result<TestOrder> {
        let order = sqlx::query_as::<_, TestOrder>(
            r#"
            UPDATE test_order
            SET order_status = $1, updated_by = $2, updated_at = NOW()
            WHERE id = $3 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&input.new_status)
        .bind(user_id)
        .bind(input.order_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(order)
    }

    pub async fn confirm_order(&self, input: ConfirmOrderInput, user_id: Uuid) -> Result<TestOrder> {
        let order = sqlx::query_as::<_, TestOrder>(
            r#"
            UPDATE test_order
            SET
                order_status = 'CONFIRMED',
                confirmed_at = NOW(),
                payment_method = $1,
                advance_paid = COALESCE($2, 0),
                updated_by = $3,
                updated_at = NOW()
            WHERE id = $4 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&input.payment_method)
        .bind(input.advance_paid)
        .bind(user_id)
        .bind(input.order_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(order)
    }

    pub async fn cancel_order(&self, input: CancelOrderInput, user_id: Uuid) -> Result<TestOrder> {
        let order = sqlx::query_as::<_, TestOrder>(
            r#"
            UPDATE test_order
            SET
                order_status = 'CANCELLED',
                is_cancelled = TRUE,
                cancelled_at = NOW(),
                cancelled_by = $1,
                cancellation_reason = $2,
                updated_by = $1,
                updated_at = NOW()
            WHERE id = $3 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(&input.cancellation_reason)
        .bind(input.order_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(order)
    }

    pub async fn update_totals(&self, order_id: Uuid) -> Result<TestOrder> {
        // Calculate total from order items
        let total: rust_decimal::Decimal = sqlx::query_scalar(
            "SELECT COALESCE(SUM(total_price), 0) FROM test_order_item WHERE order_id = $1"
        )
        .bind(order_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        let order = sqlx::query_as::<_, TestOrder>(
            r#"
            UPDATE test_order
            SET total_amount = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(total)
        .bind(order_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(order)
    }

    async fn generate_order_number(&self, org_id: &Uuid) -> Result<String> {
        let org_code = "LAB";  // Should fetch from organization service

        let row = sqlx::query("SELECT nextval('order_sequence')")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        let sequence: i64 = row.try_get(0).map_err(|e| Error::Database(sqlx::Error::Decode(Box::new(e))))?;

        let base_id = format!(
            "{}-ORD-{}-{:06}",
            org_code,
            chrono::Utc::now().format("%Y%m%d"),
            sequence
        );

        let checksum = common::utils::calculate_luhn_check_digit(&base_id);

        Ok(format!("{}{}", base_id, checksum))
    }
}

// ============================================================================
// Test Order Item Repository
// ============================================================================

#[derive(Clone)]
pub struct TestOrderItemRepository {
    pool: PgPool,
}

impl TestOrderItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_item(&self, order_id: Uuid, test_id: Uuid, test: &TestCatalog, quantity: i32) -> Result<TestOrderItem> {
        let unit_price = test.base_price.unwrap_or(rust_decimal::Decimal::ZERO);
        let total_price = unit_price * rust_decimal::Decimal::from(quantity);

        let item = sqlx::query_as::<_, TestOrderItem>(
            r#"
            INSERT INTO test_order_item (
                id, order_id, test_id, test_name, test_code,
                specimen_type, unit_price, quantity, total_price,
                item_status, result_status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'PENDING', 'PENDING')
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(order_id)
        .bind(test_id)
        .bind(&test.test_name)
        .bind(&test.test_code)
        .bind(&test.specimen_type)
        .bind(unit_price)
        .bind(quantity)
        .bind(total_price)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(item)
    }

    pub async fn find_by_order(&self, order_id: Uuid) -> Result<Vec<TestOrderItem>> {
        let items = sqlx::query_as::<_, TestOrderItem>(
            "SELECT * FROM test_order_item WHERE order_id = $1 ORDER BY created_at"
        )
        .bind(order_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(items)
    }

    pub async fn update_sample(&self, item_id: Uuid, sample_id: Uuid) -> Result<TestOrderItem> {
        let item = sqlx::query_as::<_, TestOrderItem>(
            r#"
            UPDATE test_order_item
            SET sample_id = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(sample_id)
        .bind(item_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(item)
    }

    pub async fn update_result(&self, item_id: Uuid, result_id: Uuid, status: &str) -> Result<TestOrderItem> {
        let item = sqlx::query_as::<_, TestOrderItem>(
            r#"
            UPDATE test_order_item
            SET result_id = $1, result_status = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING *
            "#
        )
        .bind(result_id)
        .bind(status)
        .bind(item_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(item)
    }

    pub async fn remove_item(&self, item_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM test_order_item WHERE id = $1")
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(())
    }
}
