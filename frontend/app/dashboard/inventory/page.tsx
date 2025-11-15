'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { DataTable } from '@/components/ui/data-table';
import { FormModal } from '@/components/ui/form-modal';
import { Modal, ModalHeader, ModalTitle, ModalBody, ModalFooter } from '@/components/ui/modal';
import { useConfirmDialog } from '@/components/ui/alert-dialog';
import { useNotificationStore } from '@/lib/store';
import { useInventory, useCreateInventoryItem, useRecordTransaction, useDeleteInventoryItem } from '@/lib/hooks';
import { SkeletonTable } from '@/components/ui/skeleton';
import {
  Plus,
  Package,
  AlertTriangle,
  TrendingDown,
  Calendar
} from 'lucide-react';
import { ColumnDef } from '@tanstack/react-table';
import { DataTableColumnHeader, DataTableRowActions } from '@/components/ui/data-table';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { Input } from '@/components/ui/input';

export const dynamic = 'force-dynamic';

interface InventoryItem {
  id: string;
  name: string;
  itemCode: string;
  category: string;
  manufacturer?: string;
  currentStock: number;
  unit: string;
  reorderPoint: number;
  reorderQuantity: number;
  unitPrice: number;
  expiryDate?: string;
  location: string;
  batchNumber?: string;
}

const inventorySchema = z.object({
  name: z.string().min(1, 'Item name is required'),
  itemCode: z.string().min(1, 'Item code is required'),
  category: z.string().min(1, 'Category is required'),
  manufacturer: z.string().optional(),
  currentStock: z.string().min(1, 'Current stock is required'),
  unit: z.string().min(1, 'Unit is required'),
  reorderPoint: z.string().min(1, 'Reorder point is required'),
  reorderQuantity: z.string().min(1, 'Reorder quantity is required'),
  unitPrice: z.string().min(1, 'Unit price is required'),
  expiryDate: z.string().optional(),
  location: z.string().min(1, 'Location is required'),
  batchNumber: z.string().optional(),
});

const transactionSchema = z.object({
  itemId: z.string().min(1),
  transactionType: z.enum(['IN', 'OUT', 'ADJUSTMENT']),
  quantity: z.string().min(1, 'Quantity is required'),
  reason: z.string().min(1, 'Reason is required'),
  reference: z.string().optional(),
});

type InventoryFormValues = z.infer<typeof inventorySchema>;
type TransactionFormValues = z.infer<typeof transactionSchema>;

export default function InventoryPage() {
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showTransactionModal, setShowTransactionModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [selectedItem, setSelectedItem] = useState<InventoryItem | null>(null);
  const addNotification = useNotificationStore((state) => state.addNotification);

  // GraphQL hooks
  const { inventory, loading, error, refetch } = useInventory({});
  const { createInventoryItem, loading: creating } = useCreateInventoryItem();
  const { recordTransaction, loading: recording } = useRecordTransaction();
  const { deleteInventoryItem } = useDeleteInventoryItem();
  const { confirm } = useConfirmDialog();

  // Form hooks - must be called before any early returns
  const form = useForm<InventoryFormValues>({
    resolver: zodResolver(inventorySchema),
    defaultValues: {
      name: '',
      itemCode: '',
      category: '',
      manufacturer: '',
      currentStock: '',
      unit: '',
      reorderPoint: '',
      reorderQuantity: '',
      unitPrice: '',
      expiryDate: '',
      location: '',
      batchNumber: '',
    },
  });

  const transactionForm = useForm<TransactionFormValues>({
    resolver: zodResolver(transactionSchema),
    defaultValues: {
      itemId: '',
      transactionType: 'IN',
      quantity: '',
      reason: '',
      reference: '',
    },
  });

  const handleCreateItem = async (values: InventoryFormValues) => {
    try {
      await createInventoryItem({
        variables: {
          input: {
            name: values.name,
            itemCode: values.itemCode,
            category: values.category,
            manufacturer: values.manufacturer,
            currentStock: parseInt(values.currentStock),
            unit: values.unit,
            reorderPoint: parseInt(values.reorderPoint),
            reorderQuantity: parseInt(values.reorderQuantity),
            unitPrice: parseFloat(values.unitPrice),
            expiryDate: values.expiryDate,
            location: values.location,
            batchNumber: values.batchNumber,
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Item Added',
        message: 'Inventory item has been added successfully',
      });

      setShowCreateModal(false);
      form.reset();
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to add item',
      });
    }
  };

  const handleRecordTransaction = async (values: TransactionFormValues) => {
    try {
      await recordTransaction({
        variables: {
          itemId: values.itemId,
          input: {
            transactionType: values.transactionType,
            quantity: parseInt(values.quantity),
            reason: values.reason,
            reference: values.reference,
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Transaction Recorded',
        message: `Stock ${values.transactionType === 'IN' ? 'added' : values.transactionType === 'OUT' ? 'removed' : 'adjusted'} successfully`,
      });

      setShowTransactionModal(false);
      transactionForm.reset();
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to record transaction',
      });
    }
  };

  const handleDelete = async (id: string) => {
    confirm({
      title: 'Delete Item',
      description: 'Are you sure you want to delete this inventory item? This action cannot be undone.',
      variant: 'danger',
      onConfirm: async () => {
        try {
          await deleteInventoryItem({ variables: { id } });
          addNotification({
            type: 'success',
            title: 'Item Deleted',
            message: 'Inventory item has been deleted',
          });
          refetch();
        } catch (error) {
          addNotification({
            type: 'error',
            title: 'Error',
            message: error instanceof Error ? error.message : 'Failed to delete item',
          });
        }
      },
    });
  };

  const handleViewDetails = (item: InventoryItem) => {
    setSelectedItem(item);
    setShowDetailModal(true);
  };

  const handleOpenTransactionModal = (item: InventoryItem) => {
    setSelectedItem(item);
    transactionForm.setValue('itemId', item.id);
    setShowTransactionModal(true);
  };

  const getStockStatus = (item: InventoryItem) => {
    if (item.currentStock === 0) {
      return { status: 'OUT_OF_STOCK', label: 'Out of Stock', color: 'text-red-600', bg: 'bg-red-50 border-red-200' };
    } else if (item.currentStock <= item.reorderPoint) {
      return { status: 'LOW_STOCK', label: 'Low Stock', color: 'text-orange-600', bg: 'bg-orange-50 border-orange-200' };
    } else {
      return { status: 'IN_STOCK', label: 'In Stock', color: 'text-green-600', bg: 'bg-green-50 border-green-200' };
    }
  };

  const columns: ColumnDef<InventoryItem>[] = [
    {
      accessorKey: 'name',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Item Name" />,
      cell: ({ row }) => (
        <div>
          <p className="font-medium">{row.original.name}</p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {row.original.itemCode}
          </p>
        </div>
      ),
    },
    {
      accessorKey: 'category',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Category" />,
    },
    {
      accessorKey: 'manufacturer',
      header: 'Manufacturer',
      cell: ({ row }) => row.original.manufacturer || <span className="text-gray-400">N/A</span>,
    },
    {
      accessorKey: 'currentStock',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Stock Level" />,
      cell: ({ row }) => {
        const status = getStockStatus(row.original);
        return (
          <div>
            <p className={`font-medium ${status.color}`}>
              {row.original.currentStock} {row.original.unit}
            </p>
            <p className="text-xs text-gray-500">
              Reorder at: {row.original.reorderPoint}
            </p>
          </div>
        );
      },
    },
    {
      accessorKey: 'status',
      header: 'Status',
      cell: ({ row }) => {
        const status = getStockStatus(row.original);
        return (
          <div className={`flex items-center gap-1 rounded-full border px-2 py-1 ${status.bg}`}>
            <span className={`text-xs font-medium ${status.color}`}>{status.label}</span>
          </div>
        );
      },
    },
    {
      accessorKey: 'unitPrice',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Unit Price" />,
      cell: ({ row }) => (
        <span className="font-medium">${row.original.unitPrice.toFixed(2)}</span>
      ),
    },
    {
      accessorKey: 'expiryDate',
      header: 'Expiry Date',
      cell: ({ row }) => {
        const expiryDate = row.original.expiryDate;
        if (!expiryDate) return <span className="text-gray-400">N/A</span>;

        const date = new Date(expiryDate);
        const today = new Date();
        const daysUntil = Math.ceil((date.getTime() - today.getTime()) / (1000 * 60 * 60 * 24));

        const color = daysUntil < 30 ? 'text-red-600' : daysUntil < 90 ? 'text-orange-600' : 'text-gray-600';

        return (
          <div className={`text-sm ${color}`}>
            {date.toLocaleDateString()}
            {daysUntil > 0 && daysUntil < 90 && (
              <p className="text-xs">({daysUntil} days)</p>
            )}
          </div>
        );
      },
    },
    {
      accessorKey: 'location',
      header: 'Location',
      cell: ({ row }) => (
        <span className="text-sm">{row.original.location}</span>
      ),
    },
    {
      id: 'actions',
      cell: ({ row }) => (
        <DataTableRowActions
          row={row}
          actions={[
            {
              label: 'View Details',
              onClick: () => handleViewDetails(row.original),
            },
            {
              label: 'Stock In',
              onClick: () => {
                handleOpenTransactionModal(row.original);
                transactionForm.setValue('transactionType', 'IN');
              },
            },
            {
              label: 'Stock Out',
              onClick: () => {
                handleOpenTransactionModal(row.original);
                transactionForm.setValue('transactionType', 'OUT');
              },
            },
            {
              label: 'Adjust Stock',
              onClick: () => {
                handleOpenTransactionModal(row.original);
                transactionForm.setValue('transactionType', 'ADJUSTMENT');
              },
            },
            {
              label: 'Delete',
              onClick: () => handleDelete(row.original.id),
              variant: 'destructive',
            },
          ]}
        />
      ),
    },
  ];

  // Calculate summary stats
  const typedInventory = (inventory as InventoryItem[]) || [];
  const totalItems = typedInventory.length;
  const lowStock = typedInventory.filter(item => item.currentStock > 0 && item.currentStock <= item.reorderPoint).length;
  const outOfStock = typedInventory.filter(item => item.currentStock === 0).length;
  const expiringSoon = typedInventory.filter(item => {
    if (!item.expiryDate) return false;
    const daysUntil = Math.ceil((new Date(item.expiryDate).getTime() - new Date().getTime()) / (1000 * 60 * 60 * 24));
    return daysUntil > 0 && daysUntil < 90;
  }).length;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Inventory Management
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            Track reagents, consumables, and laboratory supplies
          </p>
        </div>
        <Button onClick={() => setShowCreateModal(true)}>
          <Plus className="mr-2 h-4 w-4" />
          Add Item
        </Button>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-6 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Items</CardTitle>
            <Package className="h-4 w-4 text-gray-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalItems}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Inventory items</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Low Stock</CardTitle>
            <AlertTriangle className="h-4 w-4 text-orange-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-orange-600">{lowStock}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Need reordering</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Out of Stock</CardTitle>
            <TrendingDown className="h-4 w-4 text-red-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">{outOfStock}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Urgent action needed</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Expiring Soon</CardTitle>
            <Calendar className="h-4 w-4 text-yellow-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-yellow-600">{expiringSoon}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Within 90 days</p>
          </CardContent>
        </Card>
      </div>

      {/* Inventory Table */}
      <Card>
        <CardHeader>
          <CardTitle>Inventory Items</CardTitle>
          <CardDescription>
            View and manage laboratory inventory with stock transactions
          </CardDescription>
        </CardHeader>
        <CardContent>
          {loading ? (
            <SkeletonTable />
          ) : error ? (
            <div className="p-4 text-center text-red-600">
              Error loading inventory: {error.message}
            </div>
          ) : (
            <DataTable
              columns={columns}
              data={typedInventory}
              searchKey="name"
              searchPlaceholder="Search by item name..."
            />
          )}
        </CardContent>
      </Card>

      {/* Create Item Modal */}
      <FormModal
        open={showCreateModal}
        onClose={() => {
          setShowCreateModal(false);
          form.reset();
        }}
        title="Add Inventory Item"
        description="Register new inventory item"
        onSubmit={form.handleSubmit(handleCreateItem)}
        submitLabel="Add Item"
        isLoading={creating}
      >
        <Form {...form}>
          <div className="space-y-4">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Item Name *</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., Sodium Chloride Solution" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="grid gap-4 md:grid-cols-2">
              <FormField
                control={form.control}
                name="itemCode"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Item Code *</FormLabel>
                    <FormControl>
                      <Input placeholder="e.g., REG-001" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="category"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Category *</FormLabel>
                    <FormControl>
                      <select
                        {...field}
                        className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                      >
                        <option value="">Select category...</option>
                        <option value="REAGENT">Reagent</option>
                        <option value="CONSUMABLE">Consumable</option>
                        <option value="CALIBRATOR">Calibrator</option>
                        <option value="CONTROL">Control</option>
                        <option value="SUPPLY">General Supply</option>
                        <option value="OTHER">Other</option>
                      </select>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="manufacturer"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Manufacturer</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., Roche Diagnostics" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="grid gap-4 md:grid-cols-3">
              <FormField
                control={form.control}
                name="currentStock"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Current Stock *</FormLabel>
                    <FormControl>
                      <Input type="number" min="0" placeholder="0" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="unit"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Unit *</FormLabel>
                    <FormControl>
                      <select
                        {...field}
                        className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                      >
                        <option value="">Select unit...</option>
                        <option value="mL">mL</option>
                        <option value="L">L</option>
                        <option value="g">g</option>
                        <option value="kg">kg</option>
                        <option value="pcs">pieces</option>
                        <option value="box">box</option>
                        <option value="bottle">bottle</option>
                      </select>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="unitPrice"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Unit Price *</FormLabel>
                    <FormControl>
                      <Input type="number" step="0.01" min="0" placeholder="0.00" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <div className="grid gap-4 md:grid-cols-2">
              <FormField
                control={form.control}
                name="reorderPoint"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Reorder Point *</FormLabel>
                    <FormControl>
                      <Input type="number" min="0" placeholder="Minimum stock level" {...field} />
                    </FormControl>
                    <FormDescription>Alert when stock falls below this level</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="reorderQuantity"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Reorder Quantity *</FormLabel>
                    <FormControl>
                      <Input type="number" min="0" placeholder="Quantity to order" {...field} />
                    </FormControl>
                    <FormDescription>Suggested reorder quantity</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <div className="grid gap-4 md:grid-cols-2">
              <FormField
                control={form.control}
                name="expiryDate"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Expiry Date</FormLabel>
                    <FormControl>
                      <Input type="date" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="batchNumber"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Batch Number</FormLabel>
                    <FormControl>
                      <Input placeholder="e.g., BATCH2024-001" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="location"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Storage Location *</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., Refrigerator 2A, Shelf 3" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </Form>
      </FormModal>

      {/* Record Transaction Modal */}
      <FormModal
        open={showTransactionModal}
        onClose={() => {
          setShowTransactionModal(false);
          transactionForm.reset();
        }}
        title="Record Stock Transaction"
        description={`Update stock for ${selectedItem?.name}`}
        onSubmit={transactionForm.handleSubmit(handleRecordTransaction)}
        submitLabel="Record Transaction"
        isLoading={recording}
      >
        <Form {...transactionForm}>
          <div className="space-y-4">
            <FormField
              control={transactionForm.control}
              name="transactionType"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Transaction Type *</FormLabel>
                  <FormControl>
                    <select
                      {...field}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    >
                      <option value="IN">Stock In (Increase)</option>
                      <option value="OUT">Stock Out (Decrease)</option>
                      <option value="ADJUSTMENT">Adjustment</option>
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={transactionForm.control}
              name="quantity"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Quantity *</FormLabel>
                  <FormControl>
                    <Input type="number" min="1" placeholder="0" {...field} />
                  </FormControl>
                  <FormDescription>
                    Current stock: {selectedItem?.currentStock} {selectedItem?.unit}
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={transactionForm.control}
              name="reason"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Reason *</FormLabel>
                  <FormControl>
                    <textarea
                      {...field}
                      rows={3}
                      placeholder="Reason for this transaction..."
                      className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={transactionForm.control}
              name="reference"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Reference</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., PO-2024-001, INV-123" {...field} />
                  </FormControl>
                  <FormDescription>
                    Optional: Purchase order or invoice number
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </Form>
      </FormModal>

      {/* Item Detail Modal */}
      <Modal open={showDetailModal} onClose={() => setShowDetailModal(false)}>
        <ModalHeader>
          <ModalTitle>Item Details</ModalTitle>
        </ModalHeader>
        <ModalBody>
          {selectedItem && (
            <div className="space-y-4">
              <div>
                <h3 className="mb-3 font-semibold">Basic Information</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Name:</span>
                    <span className="font-medium">{selectedItem.name}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Item Code:</span>
                    <span className="font-medium">{selectedItem.itemCode}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Category:</span>
                    <span className="font-medium">{selectedItem.category}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Manufacturer:</span>
                    <span className="font-medium">{selectedItem.manufacturer || 'N/A'}</span>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Stock Information</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Current Stock:</span>
                    <span className="font-medium">
                      {selectedItem.currentStock} {selectedItem.unit}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Reorder Point:</span>
                    <span className="font-medium">
                      {selectedItem.reorderPoint} {selectedItem.unit}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Reorder Quantity:</span>
                    <span className="font-medium">
                      {selectedItem.reorderQuantity} {selectedItem.unit}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Status:</span>
                    {(() => {
                      const status = getStockStatus(selectedItem);
                      return (
                        <div className={`flex items-center gap-1 rounded-full border px-2 py-1 ${status.bg}`}>
                          <span className={`text-xs font-medium ${status.color}`}>{status.label}</span>
                        </div>
                      );
                    })()}
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Pricing & Location</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Unit Price:</span>
                    <span className="font-medium">${selectedItem.unitPrice.toFixed(2)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Total Value:</span>
                    <span className="font-medium">
                      ${(selectedItem.currentStock * selectedItem.unitPrice).toFixed(2)}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Location:</span>
                    <span className="font-medium">{selectedItem.location}</span>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Batch & Expiry</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Batch Number:</span>
                    <span className="font-medium">{selectedItem.batchNumber || 'N/A'}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Expiry Date:</span>
                    <span className="font-medium">
                      {selectedItem.expiryDate
                        ? new Date(selectedItem.expiryDate).toLocaleDateString()
                        : 'N/A'}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          )}
        </ModalBody>
        <ModalFooter>
          <Button variant="outline" onClick={() => setShowDetailModal(false)}>
            Close
          </Button>
          {selectedItem && (
            <Button onClick={() => {
              setShowDetailModal(false);
              handleOpenTransactionModal(selectedItem);
            }}>
              Record Transaction
            </Button>
          )}
        </ModalFooter>
      </Modal>
    </div>
  );
}
