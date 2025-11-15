-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Inventory Service Schema
-- ============================================================================

-- Item categories
CREATE TYPE item_category AS ENUM (
    'REAGENT',
    'CONSUMABLE',
    'CALIBRATOR',
    'CONTROL',
    'KIT',
    'CHEMICAL',
    'STATIONERY',
    'EQUIPMENT_PART',
    'OTHER'
);

-- Storage conditions
CREATE TYPE storage_condition AS ENUM (
    'ROOM_TEMPERATURE',
    'REFRIGERATED_2_TO_8',
    'FROZEN_MINUS_20',
    'FROZEN_MINUS_80',
    'CONTROLLED_TEMPERATURE'
);

-- Stock movement types
CREATE TYPE movement AS ENUM (
    'RECEIPT',
    'CONSUMPTION',
    'ADJUSTMENT',
    'RETURN',
    'TRANSFER',
    'WASTAGE',
    'EXPIRED'
);

-- Purchase order status
CREATE TYPE po_status AS ENUM (
    'DRAFT',
    'SUBMITTED',
    'APPROVED',
    'ORDERED',
    'PARTIALLY_RECEIVED',
    'RECEIVED',
    'CANCELLED'
);

-- Alert severity
CREATE TYPE alert_severity AS ENUM (
    'INFO',
    'WARNING',
    'CRITICAL'
);

-- ============================================================================
-- Vendor Table
-- ============================================================================

CREATE TABLE vendor (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Vendor details
    vendor_name VARCHAR(200) NOT NULL,
    vendor_code VARCHAR(50) UNIQUE NOT NULL,
    contact_person VARCHAR(200),
    email VARCHAR(200),
    phone VARCHAR(50),
    address TEXT,

    -- Tax and payment
    gstin VARCHAR(20),
    pan VARCHAR(15),
    payment_terms VARCHAR(200),
    credit_days INTEGER DEFAULT 30,

    -- Performance metrics
    rating DECIMAL(3, 2),              -- 0.00 to 5.00
    total_orders INTEGER DEFAULT 0,
    on_time_delivery_rate DECIMAL(5, 2), -- Percentage

    -- Status
    is_active BOOLEAN DEFAULT true,
    is_preferred BOOLEAN DEFAULT false,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_vendor_org ON vendor(organization_id) WHERE is_deleted = false;
CREATE INDEX idx_vendor_active ON vendor(is_active) WHERE is_deleted = false;

-- ============================================================================
-- Inventory Item Table
-- ============================================================================

CREATE TABLE inventory_item (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,

    -- Item identification
    item_name VARCHAR(200) NOT NULL,
    item_code VARCHAR(100) UNIQUE NOT NULL,
    item_category item_category NOT NULL,
    description TEXT,

    -- Classification
    manufacturer VARCHAR(200),
    brand VARCHAR(200),
    catalog_number VARCHAR(100),

    -- Unit and packaging
    unit_of_measure VARCHAR(50) NOT NULL,      -- mL, L, pcs, vials, etc.
    pack_size DECIMAL(12, 2),
    pack_unit VARCHAR(50),

    -- Storage and handling
    storage_condition storage_condition,
    storage_location VARCHAR(100),
    shelf_life_days INTEGER,

    -- Pricing
    unit_cost DECIMAL(12, 2),
    last_purchase_price DECIMAL(12, 2),
    last_purchase_date DATE,

    -- Stock levels
    current_stock DECIMAL(12, 2) DEFAULT 0,
    minimum_stock_level DECIMAL(12, 2) NOT NULL,
    reorder_point DECIMAL(12, 2) NOT NULL,
    maximum_stock_level DECIMAL(12, 2),

    -- Vendor information
    primary_vendor_id UUID REFERENCES vendor(id),
    alternative_vendor_id UUID REFERENCES vendor(id),

    -- Usage tracking
    monthly_consumption_avg DECIMAL(12, 2),
    last_used_date DATE,

    -- Status
    is_active BOOLEAN DEFAULT true,
    is_critical BOOLEAN DEFAULT false,
    requires_approval BOOLEAN DEFAULT false,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_inventory_item_org ON inventory_item(organization_id) WHERE is_deleted = false;
CREATE INDEX idx_inventory_item_category ON inventory_item(item_category) WHERE is_deleted = false;
CREATE INDEX idx_inventory_item_low_stock ON inventory_item(current_stock, reorder_point)
    WHERE current_stock <= reorder_point AND is_deleted = false;
CREATE INDEX idx_inventory_item_vendor ON inventory_item(primary_vendor_id) WHERE is_deleted = false;

-- ============================================================================
-- Stock Batch Table (for lot/batch tracking)
-- ============================================================================

CREATE TABLE stock_batch (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,
    item_id UUID NOT NULL REFERENCES inventory_item(id),

    -- Batch identification
    batch_number VARCHAR(100) NOT NULL,
    lot_number VARCHAR(100),
    serial_number VARCHAR(100),

    -- Dates
    manufacture_date DATE,
    expiry_date DATE,
    received_date DATE NOT NULL,

    -- Quantity and pricing
    received_quantity DECIMAL(12, 2) NOT NULL,
    current_quantity DECIMAL(12, 2) NOT NULL,
    unit_cost DECIMAL(12, 2),

    -- Quality control
    qc_status VARCHAR(50) DEFAULT 'PENDING',  -- PENDING, PASSED, FAILED, QUARANTINED
    qc_performed_by UUID,
    qc_performed_at TIMESTAMP,
    qc_remarks TEXT,

    -- Status
    is_active BOOLEAN DEFAULT true,
    is_quarantined BOOLEAN DEFAULT false,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false,

    CONSTRAINT stock_batch_item_batch_unique UNIQUE (item_id, batch_number)
);

-- Indexes
CREATE INDEX idx_stock_batch_item ON stock_batch(item_id) WHERE is_deleted = false;
CREATE INDEX idx_stock_batch_expiry ON stock_batch(expiry_date) WHERE is_deleted = false AND is_active = true;
CREATE INDEX idx_stock_batch_qc ON stock_batch(qc_status) WHERE is_deleted = false;

-- ============================================================================
-- Stock Movement Table
-- ============================================================================

CREATE TABLE stock_movement (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,
    item_id UUID NOT NULL REFERENCES inventory_item(id),
    batch_id UUID REFERENCES stock_batch(id),

    -- Movement details
    movement_type movement NOT NULL,
    movement_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    quantity DECIMAL(12, 2) NOT NULL,
    unit_cost DECIMAL(12, 2),

    -- Balance tracking
    balance_before DECIMAL(12, 2),
    balance_after DECIMAL(12, 2),

    -- References
    reference_type VARCHAR(50),        -- ORDER, TEST, MAINTENANCE, ADJUSTMENT
    reference_id UUID,
    department_id UUID,

    -- Reason and remarks
    reason VARCHAR(200),
    remarks TEXT,

    -- User tracking
    performed_by UUID NOT NULL,
    approved_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_stock_movement_item ON stock_movement(item_id) WHERE is_deleted = false;
CREATE INDEX idx_stock_movement_date ON stock_movement(movement_date DESC);
CREATE INDEX idx_stock_movement_type ON stock_movement(movement_type);
CREATE INDEX idx_stock_movement_batch ON stock_movement(batch_id) WHERE is_deleted = false;

-- ============================================================================
-- Purchase Order Table
-- ============================================================================

CREATE TABLE purchase_order (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,
    vendor_id UUID NOT NULL REFERENCES vendor(id),

    -- PO identification
    po_number VARCHAR(50) UNIQUE NOT NULL,
    po_date DATE NOT NULL,
    expected_delivery_date DATE,
    actual_delivery_date DATE,

    -- Financial details
    subtotal_amount DECIMAL(12, 2) DEFAULT 0,
    tax_amount DECIMAL(12, 2) DEFAULT 0,
    shipping_charges DECIMAL(12, 2) DEFAULT 0,
    discount_amount DECIMAL(12, 2) DEFAULT 0,
    total_amount DECIMAL(12, 2) NOT NULL,

    -- Payment terms
    payment_terms VARCHAR(200),
    payment_due_date DATE,

    -- Delivery details
    delivery_address TEXT,
    shipping_method VARCHAR(100),
    tracking_number VARCHAR(100),

    -- Status and approval
    po_status po_status DEFAULT 'DRAFT',
    requested_by UUID NOT NULL,
    approved_by UUID,
    approved_at TIMESTAMP,

    -- Notes
    special_instructions TEXT,
    internal_notes TEXT,

    -- Audit fields
    created_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID,
    updated_at TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_purchase_order_org ON purchase_order(organization_id) WHERE is_deleted = false;
CREATE INDEX idx_purchase_order_vendor ON purchase_order(vendor_id) WHERE is_deleted = false;
CREATE INDEX idx_purchase_order_status ON purchase_order(po_status) WHERE is_deleted = false;
CREATE INDEX idx_purchase_order_date ON purchase_order(po_date DESC);

-- ============================================================================
-- Purchase Order Item Table
-- ============================================================================

CREATE TABLE purchase_order_item (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    po_id UUID NOT NULL REFERENCES purchase_order(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES inventory_item(id),

    -- Item details
    item_description TEXT,
    quantity DECIMAL(12, 2) NOT NULL,
    unit_price DECIMAL(12, 2) NOT NULL,
    tax_rate DECIMAL(5, 2) DEFAULT 0,
    discount_percentage DECIMAL(5, 2) DEFAULT 0,
    line_total DECIMAL(12, 2) NOT NULL,

    -- Receiving tracking
    quantity_received DECIMAL(12, 2) DEFAULT 0,
    quantity_pending DECIMAL(12, 2),

    -- Notes
    remarks TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_po_item_po ON purchase_order_item(po_id);
CREATE INDEX idx_po_item_item ON purchase_order_item(item_id);

-- ============================================================================
-- Stock Alert Table
-- ============================================================================

CREATE TABLE stock_alert (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,
    item_id UUID NOT NULL REFERENCES inventory_item(id),

    -- Alert details
    alert_type VARCHAR(50) NOT NULL,    -- LOW_STOCK, OUT_OF_STOCK, EXPIRING_SOON, EXPIRED
    alert_severity alert_severity NOT NULL,
    alert_message TEXT NOT NULL,

    -- Status
    is_acknowledged BOOLEAN DEFAULT false,
    acknowledged_by UUID,
    acknowledged_at TIMESTAMP,

    is_resolved BOOLEAN DEFAULT false,
    resolved_by UUID,
    resolved_at TIMESTAMP,
    resolution_notes TEXT,

    -- Audit fields
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false
);

-- Indexes
CREATE INDEX idx_stock_alert_item ON stock_alert(item_id) WHERE is_deleted = false;
CREATE INDEX idx_stock_alert_unresolved ON stock_alert(is_resolved, created_at DESC)
    WHERE is_resolved = false AND is_deleted = false;
CREATE INDEX idx_stock_alert_severity ON stock_alert(alert_severity) WHERE is_deleted = false;

-- ============================================================================
-- Functions
-- ============================================================================

-- Generate PO number (format: PO-YYYYMMDD-NNNN)
CREATE OR REPLACE FUNCTION generate_po_number()
RETURNS VARCHAR(50) AS $$
DECLARE
    today_date VARCHAR(8);
    sequence_num INTEGER;
    po_num VARCHAR(50);
BEGIN
    today_date := TO_CHAR(CURRENT_DATE, 'YYYYMMDD');

    SELECT COUNT(*) + 1 INTO sequence_num
    FROM purchase_order
    WHERE po_number LIKE 'PO-' || today_date || '-%';

    po_num := 'PO-' || today_date || '-' || LPAD(sequence_num::TEXT, 4, '0');

    RETURN po_num;
END;
$$ LANGUAGE plpgsql;

-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers
CREATE TRIGGER update_vendor_updated_at
    BEFORE UPDATE ON vendor
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_inventory_item_updated_at
    BEFORE UPDATE ON inventory_item
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_purchase_order_updated_at
    BEFORE UPDATE ON purchase_order
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Update stock on movement
CREATE OR REPLACE FUNCTION update_stock_on_movement()
RETURNS TRIGGER AS $$
BEGIN
    -- Update item stock
    IF NEW.movement_type IN ('RECEIPT', 'ADJUSTMENT') AND NEW.quantity > 0 THEN
        UPDATE inventory_item
        SET current_stock = current_stock + NEW.quantity
        WHERE id = NEW.item_id;
    ELSIF NEW.movement_type IN ('CONSUMPTION', 'WASTAGE', 'EXPIRED', 'RETURN') THEN
        UPDATE inventory_item
        SET current_stock = current_stock - NEW.quantity,
            last_used_date = CURRENT_DATE
        WHERE id = NEW.item_id;
    END IF;

    -- Update batch stock if batch_id is present
    IF NEW.batch_id IS NOT NULL THEN
        IF NEW.movement_type = 'RECEIPT' THEN
            UPDATE stock_batch
            SET current_quantity = current_quantity + NEW.quantity
            WHERE id = NEW.batch_id;
        ELSE
            UPDATE stock_batch
            SET current_quantity = current_quantity - NEW.quantity
            WHERE id = NEW.batch_id;
        END IF;
    END IF;

    -- Check for low stock alert
    PERFORM check_low_stock_alert(NEW.item_id);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_stock_on_movement
    AFTER INSERT ON stock_movement
    FOR EACH ROW
    EXECUTE FUNCTION update_stock_on_movement();

-- Check and create low stock alert
CREATE OR REPLACE FUNCTION check_low_stock_alert(item_uuid UUID)
RETURNS VOID AS $$
DECLARE
    item_record RECORD;
BEGIN
    SELECT * INTO item_record
    FROM inventory_item
    WHERE id = item_uuid;

    -- Create alert if stock is low
    IF item_record.current_stock <= item_record.reorder_point THEN
        INSERT INTO stock_alert (organization_id, item_id, alert_type, alert_severity, alert_message)
        SELECT
            item_record.organization_id,
            item_record.id,
            CASE
                WHEN item_record.current_stock <= 0 THEN 'OUT_OF_STOCK'
                ELSE 'LOW_STOCK'
            END,
            CASE
                WHEN item_record.current_stock <= 0 THEN 'CRITICAL'::alert_severity
                WHEN item_record.is_critical THEN 'CRITICAL'::alert_severity
                ELSE 'WARNING'::alert_severity
            END,
            CASE
                WHEN item_record.current_stock <= 0 THEN
                    'Item ' || item_record.item_name || ' is out of stock'
                ELSE
                    'Item ' || item_record.item_name || ' is running low. Current stock: ' ||
                    item_record.current_stock || ', Reorder point: ' || item_record.reorder_point
            END
        WHERE NOT EXISTS (
            SELECT 1 FROM stock_alert
            WHERE item_id = item_record.id
            AND is_resolved = false
            AND alert_type IN ('LOW_STOCK', 'OUT_OF_STOCK')
        );
    END IF;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Sample Data
-- ============================================================================

-- Sample vendor
INSERT INTO vendor (
    id, organization_id, vendor_name, vendor_code,
    contact_person, email, phone, is_active, is_preferred, created_by
) VALUES (
    uuid_generate_v4(), uuid_generate_v4(), 'MedSupply India', 'VENDOR001',
    'Rajesh Kumar', 'rajesh@medsupply.com', '+91-9876543210',
    true, true, uuid_generate_v4()
);

-- Comments
COMMENT ON TABLE vendor IS 'Stores vendor/supplier information';
COMMENT ON TABLE inventory_item IS 'Master inventory items catalog';
COMMENT ON TABLE stock_batch IS 'Tracks individual batches/lots with expiry';
COMMENT ON TABLE stock_movement IS 'Records all stock transactions';
COMMENT ON TABLE purchase_order IS 'Purchase orders to vendors';
COMMENT ON TABLE purchase_order_item IS 'Line items in purchase orders';
COMMENT ON TABLE stock_alert IS 'Automated alerts for inventory issues';
