use crate::domain::*;
use crate::repository::*;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug)]
pub enum InventoryError {
    NotFound(String),
    ValidationError(String),
    InsufficientStock(String),
    InvalidOperation(String),
    DatabaseError(String),
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::InsufficientStock(msg) => write!(f, "Insufficient stock: {}", msg),
            Self::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for InventoryError {}

impl From<Error> for InventoryError {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound(msg) => InventoryError::NotFound(msg),
            Error::Database(msg) => InventoryError::DatabaseError(msg),
            Error::InvalidInput(msg) => InventoryError::ValidationError(msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, InventoryError>;

#[derive(Clone)]
pub struct InventoryService {
    vendor_repo: VendorRepository,
    item_repo: InventoryItemRepository,
    batch_repo: StockBatchRepository,
    movement_repo: StockMovementRepository,
    po_repo: PurchaseOrderRepository,
    alert_repo: StockAlertRepository,
}

impl InventoryService {
    pub fn new(
        vendor_repo: VendorRepository,
        item_repo: InventoryItemRepository,
        batch_repo: StockBatchRepository,
        movement_repo: StockMovementRepository,
        po_repo: PurchaseOrderRepository,
        alert_repo: StockAlertRepository,
    ) -> Self {
        Self {
            vendor_repo,
            item_repo,
            batch_repo,
            movement_repo,
            po_repo,
            alert_repo,
        }
    }

    // ============================================================================
    // Vendor Operations
    // ============================================================================

    pub async fn create_vendor(&self, input: CreateVendorInput, created_by: Uuid) -> Result<Vendor> {
        if input.vendor_name.is_empty() {
            return Err(InventoryError::ValidationError("Vendor name is required".to_string()));
        }

        if input.vendor_code.is_empty() {
            return Err(InventoryError::ValidationError("Vendor code is required".to_string()));
        }

        let vendor = self.vendor_repo.create(input, created_by).await?;
        Ok(vendor)
    }

    pub async fn get_vendor(&self, vendor_id: Uuid) -> Result<Vendor> {
        let vendor = self.vendor_repo.get_by_id(vendor_id).await?;
        Ok(vendor)
    }

    pub async fn list_vendors(&self, filter: Option<VendorFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<Vendor>> {
        let vendors = self.vendor_repo.list(filter, page, page_size).await?;
        Ok(vendors)
    }

    // ============================================================================
    // Inventory Item Operations
    // ============================================================================

    pub async fn create_item(&self, input: CreateInventoryItemInput, created_by: Uuid) -> Result<InventoryItem> {
        if input.item_name.is_empty() {
            return Err(InventoryError::ValidationError("Item name is required".to_string()));
        }

        if input.item_code.is_empty() {
            return Err(InventoryError::ValidationError("Item code is required".to_string()));
        }

        if input.unit_of_measure.is_empty() {
            return Err(InventoryError::ValidationError("Unit of measure is required".to_string()));
        }

        let item = self.item_repo.create(input, created_by).await?;
        Ok(item)
    }

    pub async fn get_item(&self, item_id: Uuid) -> Result<InventoryItem> {
        let item = self.item_repo.get_by_id(item_id).await?;
        Ok(item)
    }

    pub async fn list_items(&self, filter: Option<InventoryItemFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<InventoryItem>> {
        let items = self.item_repo.list(filter, page, page_size).await?;
        Ok(items)
    }

    pub async fn get_low_stock_items(&self, organization_id: Uuid) -> Result<Vec<InventoryItem>> {
        let items = self.item_repo.get_low_stock_items(organization_id).await?;
        Ok(items)
    }

    // ============================================================================
    // Stock Batch Operations
    // ============================================================================

    pub async fn create_batch(&self, input: CreateStockBatchInput, organization_id: Uuid, created_by: Uuid) -> Result<StockBatch> {
        if input.batch_number.is_empty() {
            return Err(InventoryError::ValidationError("Batch number is required".to_string()));
        }

        // Validate item exists
        let _item = self.item_repo.get_by_id(input.item_id).await?;

        let batch = self.batch_repo.create(input, organization_id, created_by).await?;
        Ok(batch)
    }

    pub async fn get_batch(&self, batch_id: Uuid) -> Result<StockBatch> {
        let batch = self.batch_repo.get_by_id(batch_id).await?;
        Ok(batch)
    }

    pub async fn get_item_batches(&self, item_id: Uuid) -> Result<Vec<StockBatch>> {
        let batches = self.batch_repo.get_by_item(item_id).await?;
        Ok(batches)
    }

    pub async fn get_expiring_batches(&self, organization_id: Uuid, days: i32) -> Result<Vec<StockBatch>> {
        let batches = self.batch_repo.get_expiring_batches(organization_id, days).await?;
        Ok(batches)
    }

    // ============================================================================
    // Stock Movement Operations
    // ============================================================================

    pub async fn record_movement(&self, input: RecordStockMovementInput, organization_id: Uuid, performed_by: Uuid) -> Result<StockMovement> {
        // Validate item exists
        let item = self.item_repo.get_by_id(input.item_id).await?;

        let quantity = std::str::FromStr::from_str(&input.quantity)
            .map_err(|_| InventoryError::ValidationError("Invalid quantity".to_string()))?;

        // For consumption/wastage/expired, check sufficient stock
        if matches!(input.movement_type, MovementType::Consumption | MovementType::Wastage | MovementType::Expired | MovementType::Return) {
            let current_stock = item.current_stock.unwrap_or(Decimal::ZERO);
            if current_stock < quantity {
                return Err(InventoryError::InsufficientStock(
                    format!("Insufficient stock for {}. Available: {}, Required: {}", item.item_name, current_stock, quantity)
                ));
            }
        }

        // If batch_id is provided, validate batch has sufficient quantity
        if let Some(batch_id) = input.batch_id {
            if matches!(input.movement_type, MovementType::Consumption | MovementType::Wastage | MovementType::Expired) {
                let batch = self.batch_repo.get_by_id(batch_id).await?;
                if batch.current_quantity < quantity {
                    return Err(InventoryError::InsufficientStock(
                        format!("Insufficient quantity in batch {}. Available: {}, Required: {}", batch.batch_number, batch.current_quantity, quantity)
                    ));
                }
            }
        }

        let movement = self.movement_repo.create(input, organization_id, performed_by).await?;
        Ok(movement)
    }

    pub async fn list_movements(&self, filter: Option<StockMovementFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<StockMovement>> {
        let movements = self.movement_repo.list(filter, page, page_size).await?;
        Ok(movements)
    }

    // ============================================================================
    // Purchase Order Operations
    // ============================================================================

    pub async fn create_purchase_order(&self, input: CreatePurchaseOrderInput, created_by: Uuid) -> Result<PurchaseOrder> {
        // Validate vendor exists
        let _vendor = self.vendor_repo.get_by_id(input.vendor_id).await?;

        // Validate items exist
        for item in &input.items {
            let _inventory_item = self.item_repo.get_by_id(item.item_id).await?;
        }

        if input.items.is_empty() {
            return Err(InventoryError::ValidationError("Purchase order must have at least one item".to_string()));
        }

        let po = self.po_repo.create(input, created_by).await?;
        Ok(po)
    }

    pub async fn get_purchase_order(&self, po_id: Uuid) -> Result<PurchaseOrder> {
        let po = self.po_repo.get_by_id(po_id).await?;
        Ok(po)
    }

    pub async fn get_po_items(&self, po_id: Uuid) -> Result<Vec<PurchaseOrderItem>> {
        let items = self.po_repo.get_po_items(po_id).await?;
        Ok(items)
    }

    pub async fn list_purchase_orders(&self, filter: Option<PurchaseOrderFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<PurchaseOrder>> {
        let pos = self.po_repo.list(filter, page, page_size).await?;
        Ok(pos)
    }

    pub async fn approve_purchase_order(&self, po_id: Uuid, approved_by: Uuid) -> Result<PurchaseOrder> {
        let po = self.po_repo.get_by_id(po_id).await?;

        if po.po_status != Some(PoStatus::Draft) && po.po_status != Some(PoStatus::Submitted) {
            return Err(InventoryError::InvalidOperation(
                format!("Cannot approve PO with status {:?}", po.po_status)
            ));
        }

        let updated_po = self.po_repo.update_status(po_id, PoStatus::Approved).await?;
        Ok(updated_po)
    }

    pub async fn receive_purchase_order(&self, input: ReceivePurchaseOrderInput, organization_id: Uuid, received_by: Uuid) -> Result<PurchaseOrder> {
        let po = self.po_repo.get_by_id(input.po_id).await?;

        if po.po_status != Some(PoStatus::Approved) && po.po_status != Some(PoStatus::Ordered) && po.po_status != Some(PoStatus::PartiallyReceived) {
            return Err(InventoryError::InvalidOperation(
                format!("Cannot receive PO with status {:?}", po.po_status)
            ));
        }

        // Get PO items to validate
        let po_items = self.po_repo.get_po_items(input.po_id).await?;

        for receive_item in &input.items {
            // Find matching PO item
            let po_item = po_items.iter()
                .find(|item| item.id == receive_item.po_item_id)
                .ok_or_else(|| InventoryError::NotFound(format!("PO item {} not found", receive_item.po_item_id)))?;

            let qty_received: Decimal = std::str::FromStr::from_str(&receive_item.quantity_received)
                .map_err(|_| InventoryError::ValidationError("Invalid quantity_received".to_string()))?;

            // Validate quantity
            let pending = po_item.quantity_pending.unwrap_or(Decimal::ZERO);
            if qty_received > pending {
                return Err(InventoryError::ValidationError(
                    format!("Received quantity {} exceeds pending quantity {}", qty_received, pending)
                ));
            }

            // Create stock batch
            let batch_input = CreateStockBatchInput {
                item_id: po_item.item_id,
                batch_number: receive_item.batch_number.clone(),
                lot_number: receive_item.lot_number.clone(),
                manufacture_date: None,
                expiry_date: receive_item.expiry_date.clone(),
                received_quantity: receive_item.quantity_received.clone(),
                unit_cost: Some(po_item.unit_price.to_string()),
            };

            let batch = self.batch_repo.create(batch_input, organization_id, received_by).await?;

            // Record stock receipt movement
            let movement_input = RecordStockMovementInput {
                item_id: po_item.item_id,
                batch_id: Some(batch.id),
                movement_type: MovementType::Receipt,
                quantity: receive_item.quantity_received.clone(),
                unit_cost: Some(po_item.unit_price.to_string()),
                reference_type: Some("PURCHASE_ORDER".to_string()),
                reference_id: Some(po.id),
                reason: Some("PO Receipt".to_string()),
                remarks: Some(format!("Received from PO {}", po.po_number)),
            };

            self.movement_repo.create(movement_input, organization_id, received_by).await?;
        }

        // Update PO status
        let updated_po = self.po_repo.update_status(input.po_id, PoStatus::Received).await?;
        Ok(updated_po)
    }

    // ============================================================================
    // Stock Alert Operations
    // ============================================================================

    pub async fn list_alerts(&self, filter: Option<StockAlertFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<StockAlert>> {
        let alerts = self.alert_repo.list(filter, page, page_size).await?;
        Ok(alerts)
    }

    pub async fn resolve_alert(&self, alert_id: Uuid, resolved_by: Uuid, notes: Option<String>) -> Result<StockAlert> {
        let alert = self.alert_repo.resolve(alert_id, resolved_by, notes).await?;
        Ok(alert)
    }

    // ============================================================================
    // Utility Operations
    // ============================================================================

    pub async fn get_stock_value(&self, organization_id: Uuid) -> Result<Decimal> {
        // In production, this would calculate total value of all stock
        // For now, return placeholder
        Ok(Decimal::ZERO)
    }

    pub async fn get_consumption_report(&self, item_id: Uuid, days: i32) -> Result<Decimal> {
        // In production, this would calculate consumption over specified days
        // For now, return placeholder
        Ok(Decimal::ZERO)
    }
}
