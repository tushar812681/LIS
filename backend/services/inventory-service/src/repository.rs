use crate::domain::*;
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use std::str::FromStr;
use chrono::{NaiveDate, Local};

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    Database(String),
    InvalidInput(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// ============================================================================
// Vendor Repository
// ============================================================================

#[derive(Clone)]
pub struct VendorRepository {
    pool: PgPool,
}

impl VendorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateVendorInput, created_by: Uuid) -> Result<Vendor> {
        let vendor = sqlx::query_as::<_, Vendor>(
            r#"
            INSERT INTO vendor (
                organization_id, vendor_name, vendor_code, contact_person,
                email, phone, address, gstin, pan, payment_terms,
                credit_days, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.vendor_name)
        .bind(input.vendor_code)
        .bind(input.contact_person)
        .bind(input.email)
        .bind(input.phone)
        .bind(input.address)
        .bind(input.gstin)
        .bind(input.pan)
        .bind(input.payment_terms)
        .bind(input.credit_days)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(vendor)
    }

    pub async fn get_by_id(&self, vendor_id: Uuid) -> Result<Vendor> {
        let vendor = sqlx::query_as::<_, Vendor>(
            "SELECT * FROM vendor WHERE id = $1 AND is_deleted = false"
        )
        .bind(vendor_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Vendor with ID {} not found", vendor_id)))?;

        Ok(vendor)
    }

    pub async fn list(&self, filter: Option<VendorFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<Vendor>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM vendor WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(org_id) = f.organization_id {
                query.push_str(&format!(" AND organization_id = '{}'", org_id));
            }
            if let Some(is_active) = f.is_active {
                query.push_str(&format!(" AND is_active = {}", is_active));
            }
            if let Some(is_preferred) = f.is_preferred {
                query.push_str(&format!(" AND is_preferred = {}", is_preferred));
            }
        }

        query.push_str(&format!(" ORDER BY vendor_name LIMIT {} OFFSET {}", page_size, offset));

        let vendors = sqlx::query_as::<_, Vendor>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(vendors)
    }
}

// ============================================================================
// Inventory Item Repository
// ============================================================================

#[derive(Clone)]
pub struct InventoryItemRepository {
    pool: PgPool,
}

impl InventoryItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateInventoryItemInput, created_by: Uuid) -> Result<InventoryItem> {
        let min_stock = Decimal::from_str(&input.minimum_stock_level)
            .map_err(|e| Error::InvalidInput(format!("Invalid minimum_stock_level: {}", e)))?;
        let reorder_point = Decimal::from_str(&input.reorder_point)
            .map_err(|e| Error::InvalidInput(format!("Invalid reorder_point: {}", e)))?;
        let max_stock = input.maximum_stock_level
            .as_ref()
            .map(|s| Decimal::from_str(s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid maximum_stock_level: {}", e)))?;
        let unit_cost = input.unit_cost
            .as_ref()
            .map(|s| Decimal::from_str(s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid unit_cost: {}", e)))?;

        let item = sqlx::query_as::<_, InventoryItem>(
            r#"
            INSERT INTO inventory_item (
                organization_id, item_name, item_code, item_category, description,
                manufacturer, unit_of_measure, minimum_stock_level, reorder_point,
                maximum_stock_level, storage_condition, storage_location,
                primary_vendor_id, unit_cost, is_critical, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.item_name)
        .bind(input.item_code)
        .bind(input.item_category)
        .bind(input.description)
        .bind(input.manufacturer)
        .bind(input.unit_of_measure)
        .bind(min_stock)
        .bind(reorder_point)
        .bind(max_stock)
        .bind(input.storage_condition)
        .bind(input.storage_location)
        .bind(input.primary_vendor_id)
        .bind(unit_cost)
        .bind(input.is_critical.unwrap_or(false))
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(item)
    }

    pub async fn get_by_id(&self, item_id: Uuid) -> Result<InventoryItem> {
        let item = sqlx::query_as::<_, InventoryItem>(
            "SELECT * FROM inventory_item WHERE id = $1 AND is_deleted = false"
        )
        .bind(item_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Item with ID {} not found", item_id)))?;

        Ok(item)
    }

    pub async fn list(&self, filter: Option<InventoryItemFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<InventoryItem>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(50);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM inventory_item WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(org_id) = f.organization_id {
                query.push_str(&format!(" AND organization_id = '{}'", org_id));
            }
            if let Some(category) = f.item_category {
                let cat_str = match category {
                    ItemCategory::Reagent => "REAGENT",
                    ItemCategory::Consumable => "CONSUMABLE",
                    ItemCategory::Calibrator => "CALIBRATOR",
                    ItemCategory::Control => "CONTROL",
                    ItemCategory::Kit => "KIT",
                    ItemCategory::Chemical => "CHEMICAL",
                    ItemCategory::Stationery => "STATIONERY",
                    ItemCategory::EquipmentPart => "EQUIPMENT_PART",
                    ItemCategory::Other => "OTHER",
                };
                query.push_str(&format!(" AND item_category = '{}'", cat_str));
            }
            if let Some(is_low_stock) = f.is_low_stock {
                if is_low_stock {
                    query.push_str(" AND current_stock <= reorder_point");
                }
            }
            if let Some(is_critical) = f.is_critical {
                query.push_str(&format!(" AND is_critical = {}", is_critical));
            }
        }

        query.push_str(&format!(" ORDER BY item_name LIMIT {} OFFSET {}", page_size, offset));

        let items = sqlx::query_as::<_, InventoryItem>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(items)
    }

    pub async fn get_low_stock_items(&self, organization_id: Uuid) -> Result<Vec<InventoryItem>> {
        let items = sqlx::query_as::<_, InventoryItem>(
            "SELECT * FROM inventory_item WHERE organization_id = $1 AND current_stock <= reorder_point AND is_deleted = false ORDER BY current_stock ASC"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(items)
    }
}

// ============================================================================
// Stock Batch Repository
// ============================================================================

#[derive(Clone)]
pub struct StockBatchRepository {
    pool: PgPool,
}

impl StockBatchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateStockBatchInput, organization_id: Uuid, created_by: Uuid) -> Result<StockBatch> {
        let received_qty = Decimal::from_str(&input.received_quantity)
            .map_err(|e| Error::InvalidInput(format!("Invalid received_quantity: {}", e)))?;
        let unit_cost = input.unit_cost
            .as_ref()
            .map(|s| Decimal::from_str(s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid unit_cost: {}", e)))?;

        let manufacture_date = input.manufacture_date
            .as_ref()
            .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid manufacture_date: {}", e)))?;

        let expiry_date = input.expiry_date
            .as_ref()
            .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid expiry_date: {}", e)))?;

        let batch = sqlx::query_as::<_, StockBatch>(
            r#"
            INSERT INTO stock_batch (
                organization_id, item_id, batch_number, lot_number,
                manufacture_date, expiry_date, received_date,
                received_quantity, current_quantity, unit_cost, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(organization_id)
        .bind(input.item_id)
        .bind(input.batch_number)
        .bind(input.lot_number)
        .bind(manufacture_date)
        .bind(expiry_date)
        .bind(Local::now().date_naive())
        .bind(received_qty)
        .bind(received_qty)
        .bind(unit_cost)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(batch)
    }

    pub async fn get_by_id(&self, batch_id: Uuid) -> Result<StockBatch> {
        let batch = sqlx::query_as::<_, StockBatch>(
            "SELECT * FROM stock_batch WHERE id = $1 AND is_deleted = false"
        )
        .bind(batch_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Batch with ID {} not found", batch_id)))?;

        Ok(batch)
    }

    pub async fn get_by_item(&self, item_id: Uuid) -> Result<Vec<StockBatch>> {
        let batches = sqlx::query_as::<_, StockBatch>(
            "SELECT * FROM stock_batch WHERE item_id = $1 AND is_deleted = false AND is_active = true ORDER BY expiry_date ASC NULLS LAST"
        )
        .bind(item_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(batches)
    }

    pub async fn get_expiring_batches(&self, organization_id: Uuid, days: i32) -> Result<Vec<StockBatch>> {
        let target_date = Local::now().date_naive() + chrono::Duration::days(days as i64);

        let batches = sqlx::query_as::<_, StockBatch>(
            "SELECT * FROM stock_batch WHERE organization_id = $1 AND expiry_date <= $2 AND expiry_date > CURRENT_DATE AND is_deleted = false AND is_active = true ORDER BY expiry_date ASC"
        )
        .bind(organization_id)
        .bind(target_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(batches)
    }
}

// ============================================================================
// Stock Movement Repository
// ============================================================================

#[derive(Clone)]
pub struct StockMovementRepository {
    pool: PgPool,
}

impl StockMovementRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: RecordStockMovementInput, organization_id: Uuid, performed_by: Uuid) -> Result<StockMovement> {
        let quantity = Decimal::from_str(&input.quantity)
            .map_err(|e| Error::InvalidInput(format!("Invalid quantity: {}", e)))?;
        let unit_cost = input.unit_cost
            .as_ref()
            .map(|s| Decimal::from_str(s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid unit_cost: {}", e)))?;

        // Get current stock for balance tracking
        let current_stock: (Option<Decimal>,) = sqlx::query_as(
            "SELECT current_stock FROM inventory_item WHERE id = $1"
        )
        .bind(input.item_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        let balance_before = current_stock.0.unwrap_or(Decimal::ZERO);
        let balance_after = match input.movement_type {
            MovementType::Receipt | MovementType::Adjustment if quantity > Decimal::ZERO => balance_before + quantity,
            _ => balance_before - quantity,
        };

        let movement = sqlx::query_as::<_, StockMovement>(
            r#"
            INSERT INTO stock_movement (
                organization_id, item_id, batch_id, movement_type, quantity,
                unit_cost, balance_before, balance_after, reference_type,
                reference_id, reason, remarks, performed_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#
        )
        .bind(organization_id)
        .bind(input.item_id)
        .bind(input.batch_id)
        .bind(input.movement_type)
        .bind(quantity)
        .bind(unit_cost)
        .bind(balance_before)
        .bind(balance_after)
        .bind(input.reference_type)
        .bind(input.reference_id)
        .bind(input.reason)
        .bind(input.remarks)
        .bind(performed_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(movement)
    }

    pub async fn list(&self, filter: Option<StockMovementFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<StockMovement>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(50);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM stock_movement WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(item_id) = f.item_id {
                query.push_str(&format!(" AND item_id = '{}'", item_id));
            }
            if let Some(movement_type) = f.movement_type {
                let type_str = match movement_type {
                    MovementType::Receipt => "RECEIPT",
                    MovementType::Consumption => "CONSUMPTION",
                    MovementType::Adjustment => "ADJUSTMENT",
                    MovementType::Return => "RETURN",
                    MovementType::Transfer => "TRANSFER",
                    MovementType::Wastage => "WASTAGE",
                    MovementType::Expired => "EXPIRED",
                };
                query.push_str(&format!(" AND movement_type = '{}'", type_str));
            }
            if let Some(from_date) = f.from_date {
                query.push_str(&format!(" AND movement_date >= '{}'", from_date));
            }
            if let Some(to_date) = f.to_date {
                query.push_str(&format!(" AND movement_date <= '{}'", to_date));
            }
        }

        query.push_str(&format!(" ORDER BY movement_date DESC LIMIT {} OFFSET {}", page_size, offset));

        let movements = sqlx::query_as::<_, StockMovement>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(movements)
    }
}

// ============================================================================
// Purchase Order Repository
// ============================================================================

#[derive(Clone)]
pub struct PurchaseOrderRepository {
    pool: PgPool,
}

impl PurchaseOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreatePurchaseOrderInput, created_by: Uuid) -> Result<PurchaseOrder> {
        // Generate PO number
        let po_number: (String,) = sqlx::query_as("SELECT generate_po_number()")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let po_date = NaiveDate::parse_from_str(&input.po_date, "%Y-%m-%d")
            .map_err(|e| Error::InvalidInput(format!("Invalid po_date: {}", e)))?;

        let expected_delivery = input.expected_delivery_date
            .as_ref()
            .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid expected_delivery_date: {}", e)))?;

        let shipping_charges = input.shipping_charges
            .as_ref()
            .map(|s| Decimal::from_str(s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid shipping_charges: {}", e)))?
            .unwrap_or(Decimal::ZERO);

        let discount_amount = input.discount_amount
            .as_ref()
            .map(|s| Decimal::from_str(s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid discount_amount: {}", e)))?
            .unwrap_or(Decimal::ZERO);

        // Calculate totals from items
        let mut subtotal = Decimal::ZERO;
        let mut tax_total = Decimal::ZERO;

        for item in &input.items {
            let qty = Decimal::from_str(&item.quantity)
                .map_err(|e| Error::InvalidInput(format!("Invalid quantity: {}", e)))?;
            let price = Decimal::from_str(&item.unit_price)
                .map_err(|e| Error::InvalidInput(format!("Invalid unit_price: {}", e)))?;
            let tax_rate = item.tax_rate
                .as_ref()
                .map(|s| Decimal::from_str(s))
                .transpose()
                .map_err(|e| Error::InvalidInput(format!("Invalid tax_rate: {}", e)))?
                .unwrap_or(Decimal::ZERO);
            let discount_pct = item.discount_percentage
                .as_ref()
                .map(|s| Decimal::from_str(s))
                .transpose()
                .map_err(|e| Error::InvalidInput(format!("Invalid discount_percentage: {}", e)))?
                .unwrap_or(Decimal::ZERO);

            let line_subtotal = qty * price;
            let line_discount = line_subtotal * discount_pct / Decimal::from(100);
            let taxable_amount = line_subtotal - line_discount;
            let line_tax = taxable_amount * tax_rate / Decimal::from(100);

            subtotal += line_subtotal;
            tax_total += line_tax;
        }

        let total_amount = subtotal + tax_total + shipping_charges - discount_amount;

        // Create PO
        let po = sqlx::query_as::<_, PurchaseOrder>(
            r#"
            INSERT INTO purchase_order (
                organization_id, vendor_id, po_number, po_date, expected_delivery_date,
                subtotal_amount, tax_amount, shipping_charges, discount_amount, total_amount,
                payment_terms, delivery_address, special_instructions, requested_by, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.vendor_id)
        .bind(po_number.0)
        .bind(po_date)
        .bind(expected_delivery)
        .bind(subtotal)
        .bind(tax_total)
        .bind(shipping_charges)
        .bind(discount_amount)
        .bind(total_amount)
        .bind(input.payment_terms)
        .bind(input.delivery_address)
        .bind(input.special_instructions)
        .bind(created_by)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        // Create PO items
        for item in input.items {
            let qty = Decimal::from_str(&item.quantity)
                .map_err(|e| Error::InvalidInput(format!("Invalid quantity: {}", e)))?;
            let price = Decimal::from_str(&item.unit_price)
                .map_err(|e| Error::InvalidInput(format!("Invalid unit_price: {}", e)))?;
            let tax_rate = item.tax_rate.as_ref().map(|s| Decimal::from_str(s)).transpose()
                .map_err(|e| Error::InvalidInput(format!("Invalid tax_rate: {}", e)))?
                .unwrap_or(Decimal::ZERO);
            let discount_pct = item.discount_percentage.as_ref().map(|s| Decimal::from_str(s)).transpose()
                .map_err(|e| Error::InvalidInput(format!("Invalid discount_percentage: {}", e)))?
                .unwrap_or(Decimal::ZERO);

            let line_subtotal = qty * price;
            let line_discount = line_subtotal * discount_pct / Decimal::from(100);
            let taxable_amount = line_subtotal - line_discount;
            let line_tax = taxable_amount * tax_rate / Decimal::from(100);
            let line_total = taxable_amount + line_tax;

            sqlx::query(
                r#"
                INSERT INTO purchase_order_item (
                    po_id, item_id, quantity, unit_price, tax_rate,
                    discount_percentage, line_total, quantity_pending
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#
            )
            .bind(po.id)
            .bind(item.item_id)
            .bind(qty)
            .bind(price)
            .bind(tax_rate)
            .bind(discount_pct)
            .bind(line_total)
            .bind(qty)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        }

        Ok(po)
    }

    pub async fn get_by_id(&self, po_id: Uuid) -> Result<PurchaseOrder> {
        let po = sqlx::query_as::<_, PurchaseOrder>(
            "SELECT * FROM purchase_order WHERE id = $1 AND is_deleted = false"
        )
        .bind(po_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("PO with ID {} not found", po_id)))?;

        Ok(po)
    }

    pub async fn get_po_items(&self, po_id: Uuid) -> Result<Vec<PurchaseOrderItem>> {
        let items = sqlx::query_as::<_, PurchaseOrderItem>(
            "SELECT * FROM purchase_order_item WHERE po_id = $1 ORDER BY created_at"
        )
        .bind(po_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(items)
    }

    pub async fn list(&self, filter: Option<PurchaseOrderFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<PurchaseOrder>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM purchase_order WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(org_id) = f.organization_id {
                query.push_str(&format!(" AND organization_id = '{}'", org_id));
            }
            if let Some(vendor_id) = f.vendor_id {
                query.push_str(&format!(" AND vendor_id = '{}'", vendor_id));
            }
            if let Some(status) = f.po_status {
                let status_str = match status {
                    PoStatus::Draft => "DRAFT",
                    PoStatus::Submitted => "SUBMITTED",
                    PoStatus::Approved => "APPROVED",
                    PoStatus::Ordered => "ORDERED",
                    PoStatus::PartiallyReceived => "PARTIALLY_RECEIVED",
                    PoStatus::Received => "RECEIVED",
                    PoStatus::Cancelled => "CANCELLED",
                };
                query.push_str(&format!(" AND po_status = '{}'", status_str));
            }
            if let Some(from_date) = f.from_date {
                query.push_str(&format!(" AND po_date >= '{}'", from_date));
            }
            if let Some(to_date) = f.to_date {
                query.push_str(&format!(" AND po_date <= '{}'", to_date));
            }
        }

        query.push_str(&format!(" ORDER BY po_date DESC LIMIT {} OFFSET {}", page_size, offset));

        let pos = sqlx::query_as::<_, PurchaseOrder>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(pos)
    }

    pub async fn update_status(&self, po_id: Uuid, status: PoStatus) -> Result<PurchaseOrder> {
        let po = sqlx::query_as::<_, PurchaseOrder>(
            "UPDATE purchase_order SET po_status = $2 WHERE id = $1 AND is_deleted = false RETURNING *"
        )
        .bind(po_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(po)
    }
}

// ============================================================================
// Stock Alert Repository
// ============================================================================

#[derive(Clone)]
pub struct StockAlertRepository {
    pool: PgPool,
}

impl StockAlertRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self, filter: Option<StockAlertFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<StockAlert>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(50);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM stock_alert WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(org_id) = f.organization_id {
                query.push_str(&format!(" AND organization_id = '{}'", org_id));
            }
            if let Some(item_id) = f.item_id {
                query.push_str(&format!(" AND item_id = '{}'", item_id));
            }
            if let Some(severity) = f.alert_severity {
                let severity_str = match severity {
                    AlertSeverity::Info => "INFO",
                    AlertSeverity::Warning => "WARNING",
                    AlertSeverity::Critical => "CRITICAL",
                };
                query.push_str(&format!(" AND alert_severity = '{}'", severity_str));
            }
            if let Some(is_resolved) = f.is_resolved {
                query.push_str(&format!(" AND is_resolved = {}", is_resolved));
            }
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", page_size, offset));

        let alerts = sqlx::query_as::<_, StockAlert>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(alerts)
    }

    pub async fn resolve(&self, alert_id: Uuid, resolved_by: Uuid, notes: Option<String>) -> Result<StockAlert> {
        let alert = sqlx::query_as::<_, StockAlert>(
            r#"
            UPDATE stock_alert
            SET is_resolved = true,
                resolved_by = $2,
                resolved_at = CURRENT_TIMESTAMP,
                resolution_notes = $3
            WHERE id = $1 AND is_deleted = false
            RETURNING *
            "#
        )
        .bind(alert_id)
        .bind(resolved_by)
        .bind(notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(alert)
    }
}
