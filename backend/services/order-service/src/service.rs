use uuid::Uuid;
use common::error::{Error, Result};
use common::types::{OrderStatus, Priority};

use crate::domain::*;
use crate::repository::*;

// ============================================================================
// Order Service
// ============================================================================

#[derive(Clone)]
pub struct OrderService {
    test_catalog_repo: TestCatalogRepository,
    test_panel_repo: TestPanelRepository,
    order_repo: TestOrderRepository,
    order_item_repo: TestOrderItemRepository,
}

impl OrderService {
    pub fn new(
        test_catalog_repo: TestCatalogRepository,
        test_panel_repo: TestPanelRepository,
        order_repo: TestOrderRepository,
        order_item_repo: TestOrderItemRepository,
    ) -> Self {
        Self {
            test_catalog_repo,
            test_panel_repo,
            order_repo,
            order_item_repo,
        }
    }

    // ========================================================================
    // Test Catalog Operations
    // ========================================================================

    pub async fn get_test_by_id(&self, id: Uuid) -> Result<TestCatalog> {
        self.test_catalog_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Test not found: {}", id)))
    }

    pub async fn get_test_by_code(&self, code: &str) -> Result<TestCatalog> {
        self.test_catalog_repo
            .find_by_code(code)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Test not found: {}", code)))
    }

    pub async fn search_tests(&self, filter: TestCatalogFilter, limit: i64) -> Result<Vec<TestCatalog>> {
        self.test_catalog_repo.search(filter, limit).await
    }

    pub async fn get_all_active_tests(&self, limit: i64) -> Result<Vec<TestCatalog>> {
        self.test_catalog_repo.get_all_active(limit).await
    }

    // ========================================================================
    // Test Panel Operations
    // ========================================================================

    pub async fn get_panel_by_id(&self, id: Uuid) -> Result<TestPanel> {
        self.test_panel_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Panel not found: {}", id)))
    }

    pub async fn get_panel_by_code(&self, code: &str) -> Result<TestPanel> {
        self.test_panel_repo
            .find_by_code(code)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Panel not found: {}", code)))
    }

    pub async fn get_panel_tests(&self, panel_id: Uuid) -> Result<Vec<TestCatalog>> {
        let test_ids = self.test_panel_repo.get_panel_tests(panel_id).await?;

        let mut tests = Vec::new();
        for test_id in test_ids {
            if let Some(test) = self.test_catalog_repo.find_by_id(test_id).await? {
                tests.push(test);
            }
        }

        Ok(tests)
    }

    pub async fn get_popular_panels(&self, limit: i64) -> Result<Vec<TestPanel>> {
        self.test_panel_repo.get_popular_panels(limit).await
    }

    // ========================================================================
    // Order Operations
    // ========================================================================

    pub async fn create_order(&self, input: CreateOrderInput, org_id: Uuid, user_id: Uuid) -> Result<TestOrder> {
        input.validate()?;

        let order = self.order_repo.create(input, org_id, user_id).await?;

        // TODO: Publish ORDER_CREATED event
        // TODO: Cache order

        tracing::info!("Order created: {}", order.order_number);
        Ok(order)
    }

    pub async fn get_order(&self, id: Uuid) -> Result<TestOrder> {
        self.order_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Order not found: {}", id)))
    }

    pub async fn get_order_by_number(&self, order_number: &str) -> Result<TestOrder> {
        self.order_repo
            .find_by_order_number(order_number)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Order not found: {}", order_number)))
    }

    pub async fn get_orders_by_patient(&self, patient_id: Uuid, limit: i64) -> Result<Vec<TestOrder>> {
        self.order_repo.find_by_patient(patient_id, limit).await
    }

    pub async fn search_orders(&self, filter: OrderFilter, org_id: Uuid, limit: i64) -> Result<Vec<TestOrder>> {
        self.order_repo.search(filter, org_id, limit).await
    }

    pub async fn get_order_items(&self, order_id: Uuid) -> Result<Vec<TestOrderItem>> {
        self.order_item_repo.find_by_order(order_id).await
    }

    // ========================================================================
    // Order Item Operations
    // ========================================================================

    pub async fn add_test_to_order(&self, input: AddTestToOrderInput) -> Result<TestOrder> {
        input.validate()?;

        // Verify order exists and is in DRAFT status
        let order = self.get_order(input.order_id).await?;

        if order.order_status != OrderStatus::PendingPayment {
            return Err(Error::Validation(
                "Can only add tests to orders in DRAFT status".to_string()
            ));
        }

        if let Some(test_id) = input.test_id {
            // Adding individual test
            let test = self.get_test_by_id(test_id).await?;

            // Add test to order
            self.order_item_repo.add_item(
                input.order_id,
                test_id,
                &test,
                input.quantity
            ).await?;

            tracing::info!("Added test {} to order {}", test.test_code, order.order_number);

        } else if let Some(panel_id) = input.panel_id {
            // Adding panel (all tests in panel)
            let panel = self.get_panel_by_id(panel_id).await?;
            let tests = self.get_panel_tests(panel_id).await?;

            for test in tests {
                self.order_item_repo.add_item(
                    input.order_id,
                    test.id,
                    &test,
                    input.quantity
                ).await?;
            }

            tracing::info!("Added panel {} to order {}", panel.panel_code, order.order_number);
        }

        // Update order totals
        let order = self.order_repo.update_totals(input.order_id).await?;

        // TODO: Publish ORDER_UPDATED event
        // TODO: Invalidate cache

        Ok(order)
    }

    pub async fn remove_item_from_order(&self, order_id: Uuid, item_id: Uuid) -> Result<TestOrder> {
        // Verify order exists and is in DRAFT status
        let order = self.get_order(order_id).await?;

        if order.order_status != OrderStatus::PendingPayment {
            return Err(Error::Validation(
                "Can only remove items from orders in DRAFT status".to_string()
            ));
        }

        self.order_item_repo.remove_item(item_id).await?;

        // Update order totals
        let order = self.order_repo.update_totals(order_id).await?;

        // TODO: Publish ORDER_UPDATED event
        // TODO: Invalidate cache

        tracing::info!("Removed item from order {}", order.order_number);
        Ok(order)
    }

    // ========================================================================
    // Order Workflow Operations
    // ========================================================================

    pub async fn confirm_order(&self, input: ConfirmOrderInput, user_id: Uuid) -> Result<TestOrder> {
        // Verify order exists
        let order = self.get_order(input.order_id).await?;

        // Business rules validation
        if order.order_status != OrderStatus::PendingPayment {
            return Err(Error::Validation(
                "Can only confirm orders in DRAFT status".to_string()
            ));
        }

        // Verify order has at least one item
        let items = self.get_order_items(input.order_id).await?;
        if items.is_empty() {
            return Err(Error::Validation(
                "Cannot confirm order with no items".to_string()
            ));
        }

        // Confirm order
        let order = self.order_repo.confirm_order(input, user_id).await?;

        // TODO: Publish ORDER_CONFIRMED event
        // TODO: Trigger sample creation for each order item
        // TODO: Send confirmation notifications
        // TODO: Invalidate cache

        tracing::info!("Order confirmed: {}", order.order_number);
        Ok(order)
    }

    pub async fn cancel_order(&self, input: CancelOrderInput, user_id: Uuid) -> Result<TestOrder> {
        // Verify order exists
        let order = self.get_order(input.order_id).await?;

        // Business rules validation
        if order.is_cancelled {
            return Err(Error::Validation(
                "Order is already cancelled".to_string()
            ));
        }

        if order.order_status == OrderStatus::Completed {
            return Err(Error::Validation(
                "Cannot cancel completed orders".to_string()
            ));
        }

        // Cancel order
        let order = self.order_repo.cancel_order(input, user_id).await?;

        // TODO: Publish ORDER_CANCELLED event
        // TODO: Cancel associated samples
        // TODO: Process refunds if applicable
        // TODO: Send cancellation notifications
        // TODO: Invalidate cache

        tracing::info!("Order cancelled: {}", order.order_number);
        Ok(order)
    }

    pub async fn update_order_status(&self, input: UpdateOrderStatusInput, user_id: Uuid) -> Result<TestOrder> {
        // Verify order exists
        let order = self.get_order(input.order_id).await?;

        // Validate status transition
        self.validate_status_transition(&order.order_status, &input.new_status)?;

        // Update status
        let order = self.order_repo.update_status(input, user_id).await?;

        // TODO: Publish ORDER_STATUS_CHANGED event
        // TODO: Send status update notifications
        // TODO: Invalidate cache

        tracing::info!("Order status updated: {} -> {:?}", order.order_number, order.order_status);
        Ok(order)
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    fn validate_status_transition(&self, current: &OrderStatus, new: &OrderStatus) -> Result<()> {
        let valid = match (current, new) {
            // Draft can move to Confirmed or Cancelled
            (OrderStatus::PendingPayment, OrderStatus::Confirmed) => true,
            (OrderStatus::PendingPayment, OrderStatus::Cancelled) => true,

            // Confirmed can move to SampleCollected or Cancelled
            (OrderStatus::Confirmed, OrderStatus::SampleCollected) => true,
            (OrderStatus::Confirmed, OrderStatus::Cancelled) => true,

            // SampleCollected can move to InProgress or OnHold
            (OrderStatus::SampleCollected, OrderStatus::InProgress) => true,
            (OrderStatus::SampleCollected, OrderStatus::OnHold) => true,

            // InProgress can move to Completed, PartiallyCompleted, or OnHold
            (OrderStatus::InProgress, OrderStatus::Completed) => true,
            (OrderStatus::InProgress, OrderStatus::PartiallyCompleted) => true,
            (OrderStatus::InProgress, OrderStatus::OnHold) => true,

            // OnHold can resume to InProgress or be Cancelled
            (OrderStatus::OnHold, OrderStatus::InProgress) => true,
            (OrderStatus::OnHold, OrderStatus::Cancelled) => true,

            // PartiallyCompleted can move to Completed
            (OrderStatus::PartiallyCompleted, OrderStatus::Completed) => true,

            // Same status is allowed (no-op)
            (current, new) if current == new => true,

            _ => false,
        };

        if !valid {
            return Err(Error::Validation(
                format!("Invalid status transition from {:?} to {:?}", current, new)
            ));
        }

        Ok(())
    }

    pub async fn calculate_order_pricing(&self, order_id: Uuid) -> Result<PricingSummary> {
        let order = self.get_order(order_id).await?;
        let items = self.get_order_items(order_id).await?;

        let subtotal: rust_decimal::Decimal = items.iter()
            .map(|item| item.total_price)
            .sum();

        let discount = order.discount_amount;
        let tax = order.tax_amount;
        let total = subtotal - discount + tax;
        let remaining = total - order.advance_paid;

        Ok(PricingSummary {
            subtotal,
            discount,
            tax,
            total,
            advance_paid: order.advance_paid,
            remaining,
        })
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct PricingSummary {
    pub subtotal: rust_decimal::Decimal,
    pub discount: rust_decimal::Decimal,
    pub tax: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub advance_paid: rust_decimal::Decimal,
    pub remaining: rust_decimal::Decimal,
}
