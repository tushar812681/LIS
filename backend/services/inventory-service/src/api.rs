use async_graphql::{Context, Object, Result as GqlResult, ID};
use crate::domain::*;
use crate::service::{InventoryService, InventoryError};
use uuid::Uuid;
use std::str::FromStr;

use async_graphql::ErrorExtensions;

impl ErrorExtensions for InventoryError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ============================================================================
    // Vendor Queries
    // ============================================================================

    async fn vendor(&self, ctx: &Context<'_>, id: ID) -> GqlResult<Vendor> {
        let service = ctx.data::<InventoryService>()?;
        let vendor_id = Uuid::from_str(&id)?;
        let vendor = service.get_vendor(vendor_id).await?;
        Ok(vendor)
    }

    async fn vendors(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<ID>,
        is_active: Option<bool>,
        is_preferred: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<Vendor>> {
        let service = ctx.data::<InventoryService>()?;

        let filter = if organization_id.is_some() || is_active.is_some() || is_preferred.is_some() {
            Some(VendorFilter {
                organization_id: organization_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                is_active,
                is_preferred,
            })
        } else {
            None
        };

        let vendors = service.list_vendors(filter, page, page_size).await?;
        Ok(vendors)
    }

    // ============================================================================
    // Inventory Item Queries
    // ============================================================================

    async fn inventory_item(&self, ctx: &Context<'_>, id: ID) -> GqlResult<InventoryItem> {
        let service = ctx.data::<InventoryService>()?;
        let item_id = Uuid::from_str(&id)?;
        let item = service.get_item(item_id).await?;
        Ok(item)
    }

    async fn inventory_items(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<ID>,
        item_category: Option<ItemCategory>,
        is_low_stock: Option<bool>,
        is_critical: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<InventoryItem>> {
        let service = ctx.data::<InventoryService>()?;

        let filter = if organization_id.is_some() || item_category.is_some() || is_low_stock.is_some() || is_critical.is_some() {
            Some(InventoryItemFilter {
                organization_id: organization_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                item_category,
                is_low_stock,
                is_critical,
            })
        } else {
            None
        };

        let items = service.list_items(filter, page, page_size).await?;
        Ok(items)
    }

    async fn low_stock_items(&self, ctx: &Context<'_>, organization_id: ID) -> GqlResult<Vec<InventoryItem>> {
        let service = ctx.data::<InventoryService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let items = service.get_low_stock_items(org_id).await?;
        Ok(items)
    }

    // ============================================================================
    // Stock Batch Queries
    // ============================================================================

    async fn stock_batch(&self, ctx: &Context<'_>, id: ID) -> GqlResult<StockBatch> {
        let service = ctx.data::<InventoryService>()?;
        let batch_id = Uuid::from_str(&id)?;
        let batch = service.get_batch(batch_id).await?;
        Ok(batch)
    }

    async fn item_batches(&self, ctx: &Context<'_>, item_id: ID) -> GqlResult<Vec<StockBatch>> {
        let service = ctx.data::<InventoryService>()?;
        let item_uuid = Uuid::from_str(&item_id)?;
        let batches = service.get_item_batches(item_uuid).await?;
        Ok(batches)
    }

    async fn expiring_batches(
        &self,
        ctx: &Context<'_>,
        organization_id: ID,
        days: i32,
    ) -> GqlResult<Vec<StockBatch>> {
        let service = ctx.data::<InventoryService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let batches = service.get_expiring_batches(org_id, days).await?;
        Ok(batches)
    }

    // ============================================================================
    // Stock Movement Queries
    // ============================================================================

    async fn stock_movements(
        &self,
        ctx: &Context<'_>,
        item_id: Option<ID>,
        movement_type: Option<MovementType>,
        from_date: Option<String>,
        to_date: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<StockMovement>> {
        let service = ctx.data::<InventoryService>()?;

        let filter = if item_id.is_some() || movement_type.is_some() || from_date.is_some() || to_date.is_some() {
            Some(StockMovementFilter {
                item_id: item_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                movement_type,
                from_date: from_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
                to_date: to_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            })
        } else {
            None
        };

        let movements = service.list_movements(filter, page, page_size).await?;
        Ok(movements)
    }

    // ============================================================================
    // Purchase Order Queries
    // ============================================================================

    async fn purchase_order(&self, ctx: &Context<'_>, id: ID) -> GqlResult<PurchaseOrder> {
        let service = ctx.data::<InventoryService>()?;
        let po_id = Uuid::from_str(&id)?;
        let po = service.get_purchase_order(po_id).await?;
        Ok(po)
    }

    async fn purchase_order_items(&self, ctx: &Context<'_>, po_id: ID) -> GqlResult<Vec<PurchaseOrderItem>> {
        let service = ctx.data::<InventoryService>()?;
        let po_uuid = Uuid::from_str(&po_id)?;
        let items = service.get_po_items(po_uuid).await?;
        Ok(items)
    }

    async fn purchase_orders(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<ID>,
        vendor_id: Option<ID>,
        po_status: Option<PoStatus>,
        from_date: Option<String>,
        to_date: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<PurchaseOrder>> {
        let service = ctx.data::<InventoryService>()?;

        let filter = if organization_id.is_some() || vendor_id.is_some() || po_status.is_some() || from_date.is_some() || to_date.is_some() {
            Some(PurchaseOrderFilter {
                organization_id: organization_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                vendor_id: vendor_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                po_status,
                from_date: from_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
                to_date: to_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            })
        } else {
            None
        };

        let pos = service.list_purchase_orders(filter, page, page_size).await?;
        Ok(pos)
    }

    // ============================================================================
    // Stock Alert Queries
    // ============================================================================

    async fn stock_alerts(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<ID>,
        item_id: Option<ID>,
        alert_severity: Option<AlertSeverity>,
        is_resolved: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<StockAlert>> {
        let service = ctx.data::<InventoryService>()?;

        let filter = if organization_id.is_some() || item_id.is_some() || alert_severity.is_some() || is_resolved.is_some() {
            Some(StockAlertFilter {
                organization_id: organization_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                item_id: item_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                alert_severity,
                is_resolved,
            })
        } else {
            None
        };

        let alerts = service.list_alerts(filter, page, page_size).await?;
        Ok(alerts)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // ============================================================================
    // Vendor Mutations
    // ============================================================================

    async fn create_vendor(
        &self,
        ctx: &Context<'_>,
        input: CreateVendorInput,
        created_by: ID,
    ) -> GqlResult<Vendor> {
        let service = ctx.data::<InventoryService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let vendor = service.create_vendor(input, creator_id).await?;
        Ok(vendor)
    }

    // ============================================================================
    // Inventory Item Mutations
    // ============================================================================

    async fn create_inventory_item(
        &self,
        ctx: &Context<'_>,
        input: CreateInventoryItemInput,
        created_by: ID,
    ) -> GqlResult<InventoryItem> {
        let service = ctx.data::<InventoryService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let item = service.create_item(input, creator_id).await?;
        Ok(item)
    }

    // ============================================================================
    // Stock Batch Mutations
    // ============================================================================

    async fn create_stock_batch(
        &self,
        ctx: &Context<'_>,
        input: CreateStockBatchInput,
        organization_id: ID,
        created_by: ID,
    ) -> GqlResult<StockBatch> {
        let service = ctx.data::<InventoryService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let creator_id = Uuid::from_str(&created_by)?;
        let batch = service.create_batch(input, org_id, creator_id).await?;
        Ok(batch)
    }

    // ============================================================================
    // Stock Movement Mutations
    // ============================================================================

    async fn record_stock_movement(
        &self,
        ctx: &Context<'_>,
        input: RecordStockMovementInput,
        organization_id: ID,
        performed_by: ID,
    ) -> GqlResult<StockMovement> {
        let service = ctx.data::<InventoryService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let performer_id = Uuid::from_str(&performed_by)?;
        let movement = service.record_movement(input, org_id, performer_id).await?;
        Ok(movement)
    }

    // ============================================================================
    // Purchase Order Mutations
    // ============================================================================

    async fn create_purchase_order(
        &self,
        ctx: &Context<'_>,
        input: CreatePurchaseOrderInput,
        created_by: ID,
    ) -> GqlResult<PurchaseOrder> {
        let service = ctx.data::<InventoryService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let po = service.create_purchase_order(input, creator_id).await?;
        Ok(po)
    }

    async fn approve_purchase_order(
        &self,
        ctx: &Context<'_>,
        po_id: ID,
        approved_by: ID,
    ) -> GqlResult<PurchaseOrder> {
        let service = ctx.data::<InventoryService>()?;
        let po_uuid = Uuid::from_str(&po_id)?;
        let approver_id = Uuid::from_str(&approved_by)?;
        let po = service.approve_purchase_order(po_uuid, approver_id).await?;
        Ok(po)
    }

    async fn receive_purchase_order(
        &self,
        ctx: &Context<'_>,
        input: ReceivePurchaseOrderInput,
        organization_id: ID,
        received_by: ID,
    ) -> GqlResult<PurchaseOrder> {
        let service = ctx.data::<InventoryService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let receiver_id = Uuid::from_str(&received_by)?;
        let po = service.receive_purchase_order(input, org_id, receiver_id).await?;
        Ok(po)
    }

    // ============================================================================
    // Stock Alert Mutations
    // ============================================================================

    async fn resolve_stock_alert(
        &self,
        ctx: &Context<'_>,
        alert_id: ID,
        resolved_by: ID,
        notes: Option<String>,
    ) -> GqlResult<StockAlert> {
        let service = ctx.data::<InventoryService>()?;
        let alert_uuid = Uuid::from_str(&alert_id)?;
        let resolver_id = Uuid::from_str(&resolved_by)?;
        let alert = service.resolve_alert(alert_uuid, resolver_id, notes).await?;
        Ok(alert)
    }
}
