use async_graphql::{Context, Object, Result, SimpleObject, InputObject, Enum, ID};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::*;
use crate::service::OrderService;
use common::types::{OrderStatus, Priority};

// ============================================================================
// GraphQL Types
// ============================================================================

#[derive(SimpleObject)]
pub struct TestCatalogGQL {
    pub id: ID,
    pub test_code: String,
    pub test_name: String,
    pub short_name: Option<String>,
    pub category_id: Option<ID>,
    pub department: Option<String>,
    pub specimen_type: String,
    pub specimen_volume_ml: Option<f64>,
    pub minimum_volume_ml: Option<f64>,
    pub specimen_container: Option<String>,
    pub test_method: Option<String>,
    pub result_type: String,
    pub unit_of_measurement: Option<String>,
    pub reference_range_text: Option<String>,
    pub standard_tat_hours: Option<i32>,
    pub urgent_tat_hours: Option<i32>,
    pub stat_tat_hours: Option<i32>,
    pub clinical_significance: Option<String>,
    pub requires_fasting: bool,
    pub fasting_hours: Option<i32>,
    pub special_instructions: Option<String>,
    pub base_price: Option<String>,
    pub urgent_price_multiplier: Option<String>,
    pub is_outsourced: bool,
    pub external_lab_name: Option<String>,
    pub is_active: bool,
    pub is_available: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TestCatalog> for TestCatalogGQL {
    fn from(test: TestCatalog) -> Self {
        Self {
            id: test.id.to_string().into(),
            test_code: test.test_code,
            test_name: test.test_name,
            short_name: test.short_name,
            category_id: test.category_id.map(|id| id.to_string().into()),
            department: test.department,
            specimen_type: test.specimen_type,
            specimen_volume_ml: test.specimen_volume_ml,
            minimum_volume_ml: test.minimum_volume_ml,
            specimen_container: test.specimen_container,
            test_method: test.test_method,
            result_type: test.result_type,
            unit_of_measurement: test.unit_of_measurement,
            reference_range_text: test.reference_range_text,
            standard_tat_hours: test.standard_tat_hours,
            urgent_tat_hours: test.urgent_tat_hours,
            stat_tat_hours: test.stat_tat_hours,
            clinical_significance: test.clinical_significance,
            requires_fasting: test.requires_fasting,
            fasting_hours: test.fasting_hours,
            special_instructions: test.special_instructions,
            base_price: test.base_price.map(|p| p.to_string()),
            urgent_price_multiplier: test.urgent_price_multiplier.map(|p| p.to_string()),
            is_outsourced: test.is_outsourced,
            external_lab_name: test.external_lab_name,
            is_active: test.is_active,
            is_available: test.is_available,
            created_at: test.created_at.to_rfc3339(),
            updated_at: test.updated_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject)]
pub struct TestPanelGQL {
    pub id: ID,
    pub panel_code: String,
    pub panel_name: String,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<ID>,
    pub panel_price: Option<String>,
    pub discount_percentage: Option<String>,
    pub is_popular: bool,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TestPanel> for TestPanelGQL {
    fn from(panel: TestPanel) -> Self {
        Self {
            id: panel.id.to_string().into(),
            panel_code: panel.panel_code,
            panel_name: panel.panel_name,
            short_name: panel.short_name,
            description: panel.description,
            category_id: panel.category_id.map(|id| id.to_string().into()),
            panel_price: panel.panel_price.map(|p| p.to_string()),
            discount_percentage: panel.discount_percentage.map(|p| p.to_string()),
            is_popular: panel.is_popular,
            is_active: panel.is_active,
            created_at: panel.created_at.to_rfc3339(),
            updated_at: panel.updated_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject)]
pub struct TestOrderGQL {
    pub id: ID,
    pub order_number: String,
    pub patient_id: ID,
    pub organization_id: ID,
    pub order_status: OrderStatusEnum,
    pub order_source: String,
    pub priority: PriorityEnum,
    pub referring_doctor_id: Option<ID>,
    pub referring_doctor_name: Option<String>,
    pub clinical_notes: Option<String>,
    pub order_date: String,
    pub confirmed_at: Option<String>,
    pub expected_completion_date: Option<String>,
    pub actual_completion_date: Option<String>,
    pub collection_date_time: Option<String>,
    pub collection_location: Option<String>,
    pub home_collection_requested: bool,
    pub total_amount: String,
    pub discount_amount: String,
    pub tax_amount: String,
    pub final_amount: String,
    pub payment_status: String,
    pub payment_method: Option<String>,
    pub advance_paid: String,
    pub insurance_company: Option<String>,
    pub insurance_policy_number: Option<String>,
    pub report_delivery_method: Option<String>,
    pub report_delivery_email: Option<String>,
    pub report_delivery_phone: Option<String>,
    pub notes: Option<String>,
    pub special_instructions: Option<String>,
    pub is_cancelled: bool,
    pub cancelled_at: Option<String>,
    pub cancellation_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: ID,
    pub updated_by: Option<ID>,
}

impl From<TestOrder> for TestOrderGQL {
    fn from(order: TestOrder) -> Self {
        Self {
            id: order.id.to_string().into(),
            order_number: order.order_number,
            patient_id: order.patient_id.to_string().into(),
            organization_id: order.organization_id.to_string().into(),
            order_status: order.order_status.into(),
            order_source: order.order_source,
            priority: order.priority.into(),
            referring_doctor_id: order.referring_doctor_id.map(|id| id.to_string().into()),
            referring_doctor_name: order.referring_doctor_name,
            clinical_notes: order.clinical_notes,
            order_date: order.order_date.to_rfc3339(),
            confirmed_at: order.confirmed_at.map(|dt| dt.to_rfc3339()),
            expected_completion_date: order.expected_completion_date.map(|dt| dt.to_rfc3339()),
            actual_completion_date: order.actual_completion_date.map(|dt| dt.to_rfc3339()),
            collection_date_time: order.collection_date_time.map(|dt| dt.to_rfc3339()),
            collection_location: order.collection_location,
            home_collection_requested: order.home_collection_requested,
            total_amount: order.total_amount.to_string(),
            discount_amount: order.discount_amount.to_string(),
            tax_amount: order.tax_amount.to_string(),
            final_amount: order.final_amount.to_string(),
            payment_status: order.payment_status,
            payment_method: order.payment_method,
            advance_paid: order.advance_paid.to_string(),
            insurance_company: order.insurance_company,
            insurance_policy_number: order.insurance_policy_number,
            report_delivery_method: order.report_delivery_method,
            report_delivery_email: order.report_delivery_email,
            report_delivery_phone: order.report_delivery_phone,
            notes: order.notes,
            special_instructions: order.special_instructions,
            is_cancelled: order.is_cancelled,
            cancelled_at: order.cancelled_at.map(|dt| dt.to_rfc3339()),
            cancellation_reason: order.cancellation_reason,
            created_at: order.created_at.to_rfc3339(),
            updated_at: order.updated_at.to_rfc3339(),
            created_by: order.created_by.to_string().into(),
            updated_by: order.updated_by.map(|id| id.to_string().into()),
        }
    }
}

#[derive(SimpleObject)]
pub struct TestOrderItemGQL {
    pub id: ID,
    pub order_id: ID,
    pub test_id: Option<ID>,
    pub panel_id: Option<ID>,
    pub test_name: String,
    pub test_code: String,
    pub sample_id: Option<ID>,
    pub specimen_type: Option<String>,
    pub item_status: String,
    pub unit_price: String,
    pub quantity: i32,
    pub discount_amount: String,
    pub tax_amount: String,
    pub total_price: String,
    pub result_id: Option<ID>,
    pub result_status: String,
    pub expected_completion: Option<String>,
    pub actual_completion: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TestOrderItem> for TestOrderItemGQL {
    fn from(item: TestOrderItem) -> Self {
        Self {
            id: item.id.to_string().into(),
            order_id: item.order_id.to_string().into(),
            test_id: item.test_id.map(|id| id.to_string().into()),
            panel_id: item.panel_id.map(|id| id.to_string().into()),
            test_name: item.test_name,
            test_code: item.test_code,
            sample_id: item.sample_id.map(|id| id.to_string().into()),
            specimen_type: item.specimen_type,
            item_status: item.item_status,
            unit_price: item.unit_price.to_string(),
            quantity: item.quantity,
            discount_amount: item.discount_amount.to_string(),
            tax_amount: item.tax_amount.to_string(),
            total_price: item.total_price.to_string(),
            result_id: item.result_id.map(|id| id.to_string().into()),
            result_status: item.result_status,
            expected_completion: item.expected_completion.map(|dt| dt.to_rfc3339()),
            actual_completion: item.actual_completion.map(|dt| dt.to_rfc3339()),
            notes: item.notes,
            created_at: item.created_at.to_rfc3339(),
            updated_at: item.updated_at.to_rfc3339(),
        }
    }
}

// ============================================================================
// Enums
// ============================================================================

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum OrderStatusEnum {
    PendingPayment,
    Confirmed,
    SampleCollected,
    InProgress,
    PartiallyCompleted,
    Completed,
    OnHold,
    Cancelled,
}

impl From<OrderStatus> for OrderStatusEnum {
    fn from(status: OrderStatus) -> Self {
        match status {
            OrderStatus::PendingPayment => OrderStatusEnum::PendingPayment,
            OrderStatus::Confirmed => OrderStatusEnum::Confirmed,
            OrderStatus::SampleCollected => OrderStatusEnum::SampleCollected,
            OrderStatus::InProgress => OrderStatusEnum::InProgress,
            OrderStatus::PartiallyCompleted => OrderStatusEnum::PartiallyCompleted,
            OrderStatus::Completed => OrderStatusEnum::Completed,
            OrderStatus::OnHold => OrderStatusEnum::OnHold,
            OrderStatus::Cancelled => OrderStatusEnum::Cancelled,
        }
    }
}

impl From<OrderStatusEnum> for OrderStatus {
    fn from(status: OrderStatusEnum) -> Self {
        match status {
            OrderStatusEnum::PendingPayment => OrderStatus::PendingPayment,
            OrderStatusEnum::Confirmed => OrderStatus::Confirmed,
            OrderStatusEnum::SampleCollected => OrderStatus::SampleCollected,
            OrderStatusEnum::InProgress => OrderStatus::InProgress,
            OrderStatusEnum::PartiallyCompleted => OrderStatus::PartiallyCompleted,
            OrderStatusEnum::Completed => OrderStatus::Completed,
            OrderStatusEnum::OnHold => OrderStatus::OnHold,
            OrderStatusEnum::Cancelled => OrderStatus::Cancelled,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PriorityEnum {
    Routine,
    Urgent,
    Stat,
}

impl From<Priority> for PriorityEnum {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::Routine => PriorityEnum::Routine,
            Priority::Urgent => PriorityEnum::Urgent,
            Priority::Stat => PriorityEnum::Stat,
        }
    }
}

impl From<PriorityEnum> for Priority {
    fn from(priority: PriorityEnum) -> Self {
        match priority {
            PriorityEnum::Routine => Priority::Routine,
            PriorityEnum::Urgent => Priority::Urgent,
            PriorityEnum::Stat => Priority::Stat,
        }
    }
}

// ============================================================================
// Input Types
// ============================================================================

#[derive(InputObject)]
pub struct CreateOrderInputGQL {
    pub patient_id: ID,
    pub order_source: String,
    pub priority: PriorityEnum,
    pub referring_doctor_name: Option<String>,
    pub clinical_notes: Option<String>,
    pub home_collection_requested: bool,
    pub collection_date_time: Option<String>,
    pub report_delivery_method: Option<String>,
    pub report_delivery_email: Option<String>,
    pub report_delivery_phone: Option<String>,
}

impl TryFrom<CreateOrderInputGQL> for CreateOrderInput {
    type Error = String;

    fn try_from(input: CreateOrderInputGQL) -> std::result::Result<Self, Self::Error> {
        let patient_id = Uuid::parse_str(&input.patient_id)
            .map_err(|e| format!("Invalid patient_id: {}", e))?;

        let collection_date_time = if let Some(dt_str) = input.collection_date_time {
            Some(DateTime::parse_from_rfc3339(&dt_str)
                .map_err(|e| format!("Invalid collection_date_time: {}", e))?
                .with_timezone(&Utc))
        } else {
            None
        };

        Ok(CreateOrderInput {
            patient_id,
            order_source: input.order_source,
            priority: input.priority.into(),
            referring_doctor_name: input.referring_doctor_name,
            clinical_notes: input.clinical_notes,
            home_collection_requested: input.home_collection_requested,
            collection_date_time,
            report_delivery_method: input.report_delivery_method,
            report_delivery_email: input.report_delivery_email,
            report_delivery_phone: input.report_delivery_phone,
        })
    }
}

#[derive(InputObject)]
pub struct AddTestToOrderInputGQL {
    pub order_id: ID,
    pub test_id: Option<ID>,
    pub panel_id: Option<ID>,
    pub quantity: i32,
}

impl TryFrom<AddTestToOrderInputGQL> for AddTestToOrderInput {
    type Error = String;

    fn try_from(input: AddTestToOrderInputGQL) -> std::result::Result<Self, Self::Error> {
        let order_id = Uuid::parse_str(&input.order_id)
            .map_err(|e| format!("Invalid order_id: {}", e))?;

        let test_id = if let Some(id) = input.test_id {
            Some(Uuid::parse_str(&id).map_err(|e| format!("Invalid test_id: {}", e))?)
        } else {
            None
        };

        let panel_id = if let Some(id) = input.panel_id {
            Some(Uuid::parse_str(&id).map_err(|e| format!("Invalid panel_id: {}", e))?)
        } else {
            None
        };

        Ok(AddTestToOrderInput {
            order_id,
            test_id,
            panel_id,
            quantity: input.quantity,
        })
    }
}

#[derive(InputObject)]
pub struct ConfirmOrderInputGQL {
    pub order_id: ID,
    pub payment_method: Option<String>,
    pub advance_paid: Option<String>,
}

impl TryFrom<ConfirmOrderInputGQL> for ConfirmOrderInput {
    type Error = String;

    fn try_from(input: ConfirmOrderInputGQL) -> std::result::Result<Self, Self::Error> {
        let order_id = Uuid::parse_str(&input.order_id)
            .map_err(|e| format!("Invalid order_id: {}", e))?;

        let advance_paid = if let Some(amount_str) = input.advance_paid {
            Some(amount_str.parse::<rust_decimal::Decimal>()
                .map_err(|e| format!("Invalid advance_paid amount: {}", e))?)
        } else {
            None
        };

        Ok(ConfirmOrderInput {
            order_id,
            payment_method: input.payment_method,
            advance_paid,
        })
    }
}

#[derive(InputObject)]
pub struct CancelOrderInputGQL {
    pub order_id: ID,
    pub cancellation_reason: String,
}

impl TryFrom<CancelOrderInputGQL> for CancelOrderInput {
    type Error = String;

    fn try_from(input: CancelOrderInputGQL) -> std::result::Result<Self, Self::Error> {
        let order_id = Uuid::parse_str(&input.order_id)
            .map_err(|e| format!("Invalid order_id: {}", e))?;

        Ok(CancelOrderInput {
            order_id,
            cancellation_reason: input.cancellation_reason,
        })
    }
}

#[derive(InputObject)]
pub struct UpdateOrderStatusInputGQL {
    pub order_id: ID,
    pub new_status: OrderStatusEnum,
    pub notes: Option<String>,
}

impl TryFrom<UpdateOrderStatusInputGQL> for UpdateOrderStatusInput {
    type Error = String;

    fn try_from(input: UpdateOrderStatusInputGQL) -> std::result::Result<Self, Self::Error> {
        let order_id = Uuid::parse_str(&input.order_id)
            .map_err(|e| format!("Invalid order_id: {}", e))?;

        Ok(UpdateOrderStatusInput {
            order_id,
            new_status: input.new_status.into(),
            notes: input.notes,
        })
    }
}

// ============================================================================
// GraphQL Query Root
// ============================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get test by ID
    async fn test(&self, ctx: &Context<'_>, id: ID) -> Result<Option<TestCatalogGQL>> {
        let service = ctx.data::<OrderService>()?;
        let test_id = Uuid::parse_str(&id)?;

        match service.get_test_by_id(test_id).await {
            Ok(test) => Ok(Some(test.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get test by code
    async fn test_by_code(&self, ctx: &Context<'_>, code: String) -> Result<Option<TestCatalogGQL>> {
        let service = ctx.data::<OrderService>()?;

        match service.get_test_by_code(&code).await {
            Ok(test) => Ok(Some(test.into())),
            Err(_) => Ok(None),
        }
    }

    /// Search tests
    async fn search_tests(
        &self,
        ctx: &Context<'_>,
        category_id: Option<ID>,
        department: Option<String>,
        search_query: Option<String>,
        limit: Option<i32>,
    ) -> Result<Vec<TestCatalogGQL>> {
        let service = ctx.data::<OrderService>()?;

        let cat_id = if let Some(id) = category_id {
            Some(Uuid::parse_str(&id)?)
        } else {
            None
        };

        let filter = TestCatalogFilter {
            category_id: cat_id,
            department,
            is_active: Some(true),
            search_query,
        };

        let tests = service.search_tests(filter, limit.unwrap_or(50) as i64).await?;
        Ok(tests.into_iter().map(|t| t.into()).collect())
    }

    /// Get all active tests
    async fn all_active_tests(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<TestCatalogGQL>> {
        let service = ctx.data::<OrderService>()?;
        let tests = service.get_all_active_tests(limit.unwrap_or(100) as i64).await?;
        Ok(tests.into_iter().map(|t| t.into()).collect())
    }

    /// Get panel by ID
    async fn panel(&self, ctx: &Context<'_>, id: ID) -> Result<Option<TestPanelGQL>> {
        let service = ctx.data::<OrderService>()?;
        let panel_id = Uuid::parse_str(&id)?;

        match service.get_panel_by_id(panel_id).await {
            Ok(panel) => Ok(Some(panel.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get panel tests
    async fn panel_tests(&self, ctx: &Context<'_>, panel_id: ID) -> Result<Vec<TestCatalogGQL>> {
        let service = ctx.data::<OrderService>()?;
        let id = Uuid::parse_str(&panel_id)?;
        let tests = service.get_panel_tests(id).await?;
        Ok(tests.into_iter().map(|t| t.into()).collect())
    }

    /// Get popular panels
    async fn popular_panels(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<TestPanelGQL>> {
        let service = ctx.data::<OrderService>()?;
        let panels = service.get_popular_panels(limit.unwrap_or(10) as i64).await?;
        Ok(panels.into_iter().map(|p| p.into()).collect())
    }

    /// Get order by ID
    async fn order(&self, ctx: &Context<'_>, id: ID) -> Result<Option<TestOrderGQL>> {
        let service = ctx.data::<OrderService>()?;
        let order_id = Uuid::parse_str(&id)?;

        match service.get_order(order_id).await {
            Ok(order) => Ok(Some(order.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get order by order number
    async fn order_by_number(&self, ctx: &Context<'_>, order_number: String) -> Result<Option<TestOrderGQL>> {
        let service = ctx.data::<OrderService>()?;

        match service.get_order_by_number(&order_number).await {
            Ok(order) => Ok(Some(order.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get orders by patient
    async fn orders_by_patient(&self, ctx: &Context<'_>, patient_id: ID, limit: Option<i32>) -> Result<Vec<TestOrderGQL>> {
        let service = ctx.data::<OrderService>()?;
        let id = Uuid::parse_str(&patient_id)?;
        let orders = service.get_orders_by_patient(id, limit.unwrap_or(50) as i64).await?;
        Ok(orders.into_iter().map(|o| o.into()).collect())
    }

    /// Get order items
    async fn order_items(&self, ctx: &Context<'_>, order_id: ID) -> Result<Vec<TestOrderItemGQL>> {
        let service = ctx.data::<OrderService>()?;
        let id = Uuid::parse_str(&order_id)?;
        let items = service.get_order_items(id).await?;
        Ok(items.into_iter().map(|i| i.into()).collect())
    }
}

// ============================================================================
// GraphQL Mutation Root
// ============================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create new order
    async fn create_order(&self, ctx: &Context<'_>, input: CreateOrderInputGQL) -> Result<TestOrderGQL> {
        let service = ctx.data::<OrderService>()?;

        // TODO: Get org_id and user_id from auth context
        let org_id = Uuid::nil();
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let order = service.create_order(domain_input, org_id, user_id).await?;
        Ok(order.into())
    }

    /// Add test or panel to order
    async fn add_test_to_order(&self, ctx: &Context<'_>, input: AddTestToOrderInputGQL) -> Result<TestOrderGQL> {
        let service = ctx.data::<OrderService>()?;

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let order = service.add_test_to_order(domain_input).await?;
        Ok(order.into())
    }

    /// Remove item from order
    async fn remove_item_from_order(&self, ctx: &Context<'_>, order_id: ID, item_id: ID) -> Result<TestOrderGQL> {
        let service = ctx.data::<OrderService>()?;
        let oid = Uuid::parse_str(&order_id)?;
        let iid = Uuid::parse_str(&item_id)?;

        let order = service.remove_item_from_order(oid, iid).await?;
        Ok(order.into())
    }

    /// Confirm order
    async fn confirm_order(&self, ctx: &Context<'_>, input: ConfirmOrderInputGQL) -> Result<TestOrderGQL> {
        let service = ctx.data::<OrderService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let order = service.confirm_order(domain_input, user_id).await?;
        Ok(order.into())
    }

    /// Cancel order
    async fn cancel_order(&self, ctx: &Context<'_>, input: CancelOrderInputGQL) -> Result<TestOrderGQL> {
        let service = ctx.data::<OrderService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let order = service.cancel_order(domain_input, user_id).await?;
        Ok(order.into())
    }

    /// Update order status
    async fn update_order_status(&self, ctx: &Context<'_>, input: UpdateOrderStatusInputGQL) -> Result<TestOrderGQL> {
        let service = ctx.data::<OrderService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let order = service.update_order_status(domain_input, user_id).await?;
        Ok(order.into())
    }
}
