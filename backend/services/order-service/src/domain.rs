use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use common::types::{Priority, OrderStatus};

// ============================================================================
// Test Catalog Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestCatalog {
    pub id: Uuid,
    pub test_code: String,
    pub test_name: String,
    pub short_name: Option<String>,

    pub category_id: Option<Uuid>,
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

    pub base_price: Option<rust_decimal::Decimal>,
    pub urgent_price_multiplier: Option<rust_decimal::Decimal>,

    pub is_outsourced: bool,
    pub external_lab_name: Option<String>,

    pub is_active: bool,
    pub is_available: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TestCatalog {
    pub fn calculate_price(&self, priority: &Priority) -> rust_decimal::Decimal {
        let base = self.base_price.unwrap_or(rust_decimal::Decimal::ZERO);
        match priority {
            Priority::Urgent => base * self.urgent_price_multiplier.unwrap_or(rust_decimal::Decimal::from(150)) / rust_decimal::Decimal::from(100),
            Priority::Stat => base * self.urgent_price_multiplier.unwrap_or(rust_decimal::Decimal::from(200)) / rust_decimal::Decimal::from(100),
            _ => base,
        }
    }
}

// ============================================================================
// Test Panel Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestPanel {
    pub id: Uuid,
    pub panel_code: String,
    pub panel_name: String,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub panel_price: Option<rust_decimal::Decimal>,
    pub discount_percentage: Option<rust_decimal::Decimal>,
    pub is_popular: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestPanelItem {
    pub id: Uuid,
    pub panel_id: Uuid,
    pub test_id: Uuid,
    pub is_mandatory: bool,
    pub display_order: i32,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// Test Order Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestOrder {
    pub id: Uuid,
    pub order_number: String,

    pub patient_id: Uuid,
    pub organization_id: Uuid,

    pub order_status: OrderStatus,
    pub order_source: String,
    pub priority: Priority,

    pub referring_doctor_id: Option<Uuid>,
    pub referring_doctor_name: Option<String>,
    pub clinical_notes: Option<String>,

    pub order_date: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub expected_completion_date: Option<DateTime<Utc>>,
    pub actual_completion_date: Option<DateTime<Utc>>,

    pub collection_date_time: Option<DateTime<Utc>>,
    pub collection_location: Option<String>,
    pub home_collection_requested: bool,

    pub total_amount: rust_decimal::Decimal,
    pub discount_amount: rust_decimal::Decimal,
    pub tax_amount: rust_decimal::Decimal,
    pub final_amount: rust_decimal::Decimal,

    pub payment_status: String,
    pub payment_method: Option<String>,
    pub advance_paid: rust_decimal::Decimal,

    pub insurance_company: Option<String>,
    pub insurance_policy_number: Option<String>,

    pub report_delivery_method: Option<String>,
    pub report_delivery_email: Option<String>,
    pub report_delivery_phone: Option<String>,

    pub notes: Option<String>,
    pub special_instructions: Option<String>,

    pub is_cancelled: bool,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
    pub is_deleted: bool,
}

impl TestOrder {
    pub fn calculate_final_amount(&mut self) {
        let discount = if self.discount_amount > rust_decimal::Decimal::ZERO {
            self.discount_amount
        } else {
            rust_decimal::Decimal::ZERO
        };

        self.final_amount = self.total_amount - discount + self.tax_amount;
    }

    pub fn is_paid(&self) -> bool {
        self.payment_status == "PAID"
    }

    pub fn is_partially_paid(&self) -> bool {
        self.advance_paid > rust_decimal::Decimal::ZERO && self.advance_paid < self.final_amount
    }

    pub fn remaining_amount(&self) -> rust_decimal::Decimal {
        self.final_amount - self.advance_paid
    }
}

// ============================================================================
// Test Order Item Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestOrderItem {
    pub id: Uuid,
    pub order_id: Uuid,

    pub test_id: Option<Uuid>,
    pub panel_id: Option<Uuid>,

    pub test_name: String,
    pub test_code: String,

    pub sample_id: Option<Uuid>,
    pub specimen_type: Option<String>,

    pub item_status: String,

    pub unit_price: rust_decimal::Decimal,
    pub quantity: i32,
    pub discount_amount: rust_decimal::Decimal,
    pub tax_amount: rust_decimal::Decimal,
    pub total_price: rust_decimal::Decimal,

    pub result_id: Option<Uuid>,
    pub result_status: String,

    pub expected_completion: Option<DateTime<Utc>>,
    pub actual_completion: Option<DateTime<Utc>>,

    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TestOrderItem {
    pub fn calculate_total(&mut self) {
        let subtotal = self.unit_price * rust_decimal::Decimal::from(self.quantity);
        self.total_price = subtotal - self.discount_amount + self.tax_amount;
    }

    pub fn is_completed(&self) -> bool {
        self.item_status == "COMPLETED"
    }
}

// ============================================================================
// Input DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderInput {
    pub patient_id: Uuid,
    pub order_source: String,
    pub priority: Priority,
    pub referring_doctor_name: Option<String>,
    pub clinical_notes: Option<String>,
    pub home_collection_requested: bool,
    pub collection_date_time: Option<DateTime<Utc>>,
    pub report_delivery_method: Option<String>,
    pub report_delivery_email: Option<String>,
    pub report_delivery_phone: Option<String>,
}

impl CreateOrderInput {
    pub fn validate(&self) -> Result<(), common::error::Error> {
        if self.home_collection_requested && self.collection_date_time.is_none() {
            return Err(common::error::Error::Validation(
                "Collection date/time required for home collection".to_string()
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTestToOrderInput {
    pub order_id: Uuid,
    pub test_id: Option<Uuid>,
    pub panel_id: Option<Uuid>,
    pub quantity: i32,
}

impl AddTestToOrderInput {
    pub fn validate(&self) -> Result<(), common::error::Error> {
        if self.test_id.is_none() && self.panel_id.is_none() {
            return Err(common::error::Error::Validation(
                "Either test_id or panel_id must be provided".to_string()
            ));
        }
        if self.test_id.is_some() && self.panel_id.is_some() {
            return Err(common::error::Error::Validation(
                "Cannot specify both test_id and panel_id".to_string()
            ));
        }
        if self.quantity < 1 {
            return Err(common::error::Error::Validation(
                "Quantity must be at least 1".to_string()
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmOrderInput {
    pub order_id: Uuid,
    pub payment_method: Option<String>,
    pub advance_paid: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderInput {
    pub order_id: Uuid,
    pub cancellation_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderStatusInput {
    pub order_id: Uuid,
    pub new_status: OrderStatus,
    pub notes: Option<String>,
}

// ============================================================================
// Query Filters
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFilter {
    pub patient_id: Option<Uuid>,
    pub order_status: Option<OrderStatus>,
    pub order_source: Option<String>,
    pub priority: Option<Priority>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub order_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCatalogFilter {
    pub category_id: Option<Uuid>,
    pub department: Option<String>,
    pub is_active: Option<bool>,
    pub search_query: Option<String>,
}
