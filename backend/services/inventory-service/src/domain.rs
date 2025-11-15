use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "item_category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemCategory {
    Reagent,
    Consumable,
    Calibrator,
    Control,
    Kit,
    Chemical,
    Stationery,
    EquipmentPart,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "storage_condition", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageCondition {
    RoomTemperature,
    Refrigerated2To8,
    FrozenMinus20,
    FrozenMinus80,
    ControlledTemperature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "movement_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MovementType {
    Receipt,
    Consumption,
    Adjustment,
    Return,
    Transfer,
    Wastage,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "po_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PoStatus {
    Draft,
    Submitted,
    Approved,
    Ordered,
    PartiallyReceived,
    Received,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "alert_severity", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

// ============================================================================
// Vendor Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Vendor {
    pub id: Uuid,
    pub organization_id: Uuid,

    pub vendor_name: String,
    pub vendor_code: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,

    pub gstin: Option<String>,
    pub pan: Option<String>,
    pub payment_terms: Option<String>,
    pub credit_days: Option<i32>,

    pub rating: Option<Decimal>,
    pub total_orders: Option<i32>,
    pub on_time_delivery_rate: Option<Decimal>,

    pub is_active: Option<bool>,
    pub is_preferred: Option<bool>,

    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

// ============================================================================
// Inventory Item Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct InventoryItem {
    pub id: Uuid,
    pub organization_id: Uuid,

    pub item_name: String,
    pub item_code: String,
    pub item_category: ItemCategory,
    pub description: Option<String>,

    pub manufacturer: Option<String>,
    pub brand: Option<String>,
    pub catalog_number: Option<String>,

    pub unit_of_measure: String,
    pub pack_size: Option<Decimal>,
    pub pack_unit: Option<String>,

    pub storage_condition: Option<StorageCondition>,
    pub storage_location: Option<String>,
    pub shelf_life_days: Option<i32>,

    pub unit_cost: Option<Decimal>,
    pub last_purchase_price: Option<Decimal>,
    pub last_purchase_date: Option<NaiveDate>,

    pub current_stock: Option<Decimal>,
    pub minimum_stock_level: Decimal,
    pub reorder_point: Decimal,
    pub maximum_stock_level: Option<Decimal>,

    pub primary_vendor_id: Option<Uuid>,
    pub alternative_vendor_id: Option<Uuid>,

    pub monthly_consumption_avg: Option<Decimal>,
    pub last_used_date: Option<NaiveDate>,

    pub is_active: Option<bool>,
    pub is_critical: Option<bool>,
    pub requires_approval: Option<bool>,

    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

impl InventoryItem {
    pub fn is_low_stock(&self) -> bool {
        let current = self.current_stock.unwrap_or(Decimal::ZERO);
        current <= self.reorder_point
    }

    pub fn is_out_of_stock(&self) -> bool {
        self.current_stock.unwrap_or(Decimal::ZERO) <= Decimal::ZERO
    }

    pub fn stock_percentage(&self) -> Option<Decimal> {
        if let Some(max) = self.maximum_stock_level {
            if max > Decimal::ZERO {
                let current = self.current_stock.unwrap_or(Decimal::ZERO);
                return Some((current / max) * Decimal::from(100));
            }
        }
        None
    }
}

// ============================================================================
// Stock Batch Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct StockBatch {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub item_id: Uuid,

    pub batch_number: String,
    pub lot_number: Option<String>,
    pub serial_number: Option<String>,

    pub manufacture_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub received_date: NaiveDate,

    pub received_quantity: Decimal,
    pub current_quantity: Decimal,
    pub unit_cost: Option<Decimal>,

    pub qc_status: Option<String>,
    pub qc_performed_by: Option<Uuid>,
    pub qc_performed_at: Option<NaiveDateTime>,
    pub qc_remarks: Option<String>,

    pub is_active: Option<bool>,
    pub is_quarantined: Option<bool>,

    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub is_deleted: Option<bool>,
}

impl StockBatch {
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry_date {
            expiry < chrono::Local::now().date_naive()
        } else {
            false
        }
    }

    pub fn days_to_expiry(&self) -> Option<i64> {
        if let Some(expiry) = self.expiry_date {
            let today = chrono::Local::now().date_naive();
            Some((expiry - today).num_days())
        } else {
            None
        }
    }

    pub fn is_expiring_soon(&self, days: i64) -> bool {
        if let Some(days_left) = self.days_to_expiry() {
            days_left > 0 && days_left <= days
        } else {
            false
        }
    }
}

// ============================================================================
// Stock Movement Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct StockMovement {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub item_id: Uuid,
    pub batch_id: Option<Uuid>,

    pub movement_type: MovementType,
    pub movement_date: NaiveDateTime,
    pub quantity: Decimal,
    pub unit_cost: Option<Decimal>,

    pub balance_before: Option<Decimal>,
    pub balance_after: Option<Decimal>,

    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub department_id: Option<Uuid>,

    pub reason: Option<String>,
    pub remarks: Option<String>,

    pub performed_by: Uuid,
    pub approved_by: Option<Uuid>,
    pub created_at: NaiveDateTime,

    pub is_deleted: Option<bool>,
}

// ============================================================================
// Purchase Order Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub vendor_id: Uuid,

    pub po_number: String,
    pub po_date: NaiveDate,
    pub expected_delivery_date: Option<NaiveDate>,
    pub actual_delivery_date: Option<NaiveDate>,

    pub subtotal_amount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub shipping_charges: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
    pub total_amount: Decimal,

    pub payment_terms: Option<String>,
    pub payment_due_date: Option<NaiveDate>,

    pub delivery_address: Option<String>,
    pub shipping_method: Option<String>,
    pub tracking_number: Option<String>,

    pub po_status: Option<PoStatus>,
    pub requested_by: Uuid,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<NaiveDateTime>,

    pub special_instructions: Option<String>,
    pub internal_notes: Option<String>,

    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

// ============================================================================
// Purchase Order Item Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct PurchaseOrderItem {
    pub id: Uuid,
    pub po_id: Uuid,
    pub item_id: Uuid,

    pub item_description: Option<String>,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub tax_rate: Option<Decimal>,
    pub discount_percentage: Option<Decimal>,
    pub line_total: Decimal,

    pub quantity_received: Option<Decimal>,
    pub quantity_pending: Option<Decimal>,

    pub remarks: Option<String>,

    pub created_at: NaiveDateTime,
}

// ============================================================================
// Stock Alert Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct StockAlert {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub item_id: Uuid,

    pub alert_type: String,
    pub alert_severity: AlertSeverity,
    pub alert_message: String,

    pub is_acknowledged: Option<bool>,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<NaiveDateTime>,

    pub is_resolved: Option<bool>,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<NaiveDateTime>,
    pub resolution_notes: Option<String>,

    pub created_at: NaiveDateTime,
    pub is_deleted: Option<bool>,
}

// ============================================================================
// Input Types
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct CreateVendorInput {
    pub organization_id: Uuid,
    pub vendor_name: String,
    pub vendor_code: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub gstin: Option<String>,
    pub pan: Option<String>,
    pub payment_terms: Option<String>,
    pub credit_days: Option<i32>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateInventoryItemInput {
    pub organization_id: Uuid,
    pub item_name: String,
    pub item_code: String,
    pub item_category: ItemCategory,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub unit_of_measure: String,
    pub minimum_stock_level: String, // Decimal as string
    pub reorder_point: String,
    pub maximum_stock_level: Option<String>,
    pub storage_condition: Option<StorageCondition>,
    pub storage_location: Option<String>,
    pub primary_vendor_id: Option<Uuid>,
    pub unit_cost: Option<String>,
    pub is_critical: Option<bool>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateStockBatchInput {
    pub item_id: Uuid,
    pub batch_number: String,
    pub lot_number: Option<String>,
    pub manufacture_date: Option<String>,
    pub expiry_date: Option<String>,
    pub received_quantity: String,
    pub unit_cost: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct RecordStockMovementInput {
    pub item_id: Uuid,
    pub batch_id: Option<Uuid>,
    pub movement_type: MovementType,
    pub quantity: String,
    pub unit_cost: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub reason: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreatePurchaseOrderInput {
    pub organization_id: Uuid,
    pub vendor_id: Uuid,
    pub po_date: String,
    pub expected_delivery_date: Option<String>,
    pub payment_terms: Option<String>,
    pub delivery_address: Option<String>,
    pub shipping_charges: Option<String>,
    pub discount_amount: Option<String>,
    pub special_instructions: Option<String>,
    pub items: Vec<PurchaseOrderItemInput>,
}

#[derive(Debug, Clone, InputObject)]
pub struct PurchaseOrderItemInput {
    pub item_id: Uuid,
    pub quantity: String,
    pub unit_price: String,
    pub tax_rate: Option<String>,
    pub discount_percentage: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct ReceivePurchaseOrderInput {
    pub po_id: Uuid,
    pub received_date: String,
    pub items: Vec<ReceivePoItemInput>,
}

#[derive(Debug, Clone, InputObject)]
pub struct ReceivePoItemInput {
    pub po_item_id: Uuid,
    pub quantity_received: String,
    pub batch_number: String,
    pub lot_number: Option<String>,
    pub expiry_date: Option<String>,
}

// ============================================================================
// Filter Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct VendorFilter {
    pub organization_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub is_preferred: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct InventoryItemFilter {
    pub organization_id: Option<Uuid>,
    pub item_category: Option<ItemCategory>,
    pub is_low_stock: Option<bool>,
    pub is_critical: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct StockMovementFilter {
    pub item_id: Option<Uuid>,
    pub movement_type: Option<MovementType>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct PurchaseOrderFilter {
    pub organization_id: Option<Uuid>,
    pub vendor_id: Option<Uuid>,
    pub po_status: Option<PoStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct StockAlertFilter {
    pub organization_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub alert_severity: Option<AlertSeverity>,
    pub is_resolved: Option<bool>,
}
