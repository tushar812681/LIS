'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { DataTable } from '@/components/ui/data-table';
import { FormModal } from '@/components/ui/form-modal';
import { Modal, ModalHeader, ModalTitle, ModalBody, ModalFooter } from '@/components/ui/modal';
import { useConfirmDialog } from '@/components/ui/alert-dialog';
import { useNotificationStore } from '@/lib/store';
import { useInvoices, useCreateInvoice, useRecordPayment, useDeleteInvoice } from '@/lib/hooks';
import { PaymentStatusBadge } from '@/components/ui/status-badge';
import { SkeletonTable } from '@/components/ui/skeleton';
import {
  Plus,
  DollarSign,
  CreditCard,
  FileText,
  Download,
  Printer
} from 'lucide-react';
import { ColumnDef } from '@tanstack/react-table';
import { DataTableColumnHeader, DataTableRowActions } from '@/components/ui/data-table';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { Input } from '@/components/ui/input';

export const dynamic = 'force-dynamic';

interface Invoice {
  id: string;
  invoiceNumber: string;
  order: {
    orderNumber: string;
    patient: {
      firstName: string;
      lastName: string;
      mrn: string;
    };
  };
  amount: number;
  discount: number;
  tax: number;
  totalAmount: number;
  paymentStatus: 'UNPAID' | 'PARTIALLY_PAID' | 'PAID' | 'OVERDUE';
  paymentMethod?: string;
  paidAmount: number;
  dueDate: string;
  issuedAt: string;
  paidAt?: string;
}

const invoiceSchema = z.object({
  orderId: z.string().min(1, 'Order is required'),
  amount: z.string().min(1, 'Amount is required'),
  discount: z.string().optional(),
  tax: z.string().optional(),
  dueDate: z.string().min(1, 'Due date is required'),
  notes: z.string().optional(),
});

const paymentSchema = z.object({
  invoiceId: z.string().min(1),
  amount: z.string().min(1, 'Payment amount is required'),
  paymentMethod: z.enum(['CASH', 'CARD', 'UPI', 'NET_BANKING', 'CHEQUE']),
  transactionId: z.string().optional(),
  notes: z.string().optional(),
});

type InvoiceFormValues = z.infer<typeof invoiceSchema>;
type PaymentFormValues = z.infer<typeof paymentSchema>;

export default function BillingPage() {
  const router = useRouter();
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showPaymentModal, setShowPaymentModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [selectedInvoice, setSelectedInvoice] = useState<Invoice | null>(null);
  const addNotification = useNotificationStore((state) => state.addNotification);

  // GraphQL hooks
  const { invoices, loading, error, refetch } = useInvoices({});
  const { createInvoice, loading: creating } = useCreateInvoice();
  const { recordPayment, loading: recording } = useRecordPayment();
  const { deleteInvoice } = useDeleteInvoice();
  const { confirm } = useConfirmDialog();

  // Form hooks - must be called before any early returns
  const form = useForm<InvoiceFormValues>({
    resolver: zodResolver(invoiceSchema),
    defaultValues: {
      orderId: '',
      amount: '',
      discount: '0',
      tax: '0',
      dueDate: '',
      notes: '',
    },
  });

  const paymentForm = useForm<PaymentFormValues>({
    resolver: zodResolver(paymentSchema),
    defaultValues: {
      invoiceId: '',
      amount: '',
      paymentMethod: 'CASH',
      transactionId: '',
      notes: '',
    },
  });

  const handleCreateInvoice = async (values: InvoiceFormValues) => {
    try {
      await createInvoice({
        variables: {
          input: {
            orderId: values.orderId,
            amount: parseFloat(values.amount),
            discount: values.discount ? parseFloat(values.discount) : 0,
            tax: values.tax ? parseFloat(values.tax) : 0,
            dueDate: values.dueDate,
            notes: values.notes,
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Invoice Created',
        message: 'Invoice has been created successfully',
      });

      setShowCreateModal(false);
      form.reset();
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to create invoice',
      });
    }
  };

  const handleRecordPayment = async (values: PaymentFormValues) => {
    try {
      await recordPayment({
        variables: {
          invoiceId: values.invoiceId,
          input: {
            amount: parseFloat(values.amount),
            paymentMethod: values.paymentMethod,
            transactionId: values.transactionId,
            notes: values.notes,
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Payment Recorded',
        message: 'Payment has been recorded successfully',
      });

      setShowPaymentModal(false);
      paymentForm.reset();
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to record payment',
      });
    }
  };

  const handleDelete = async (id: string) => {
    confirm({
      title: 'Delete Invoice',
      description: 'Are you sure you want to delete this invoice? This action cannot be undone.',
      variant: 'danger',
      onConfirm: async () => {
        try {
          await deleteInvoice({ variables: { id } });
          addNotification({
            type: 'success',
            title: 'Invoice Deleted',
            message: 'Invoice has been deleted',
          });
          refetch();
        } catch (error) {
          addNotification({
            type: 'error',
            title: 'Error',
            message: error instanceof Error ? error.message : 'Failed to delete invoice',
          });
        }
      },
    });
  };

  const handleViewDetails = (invoice: Invoice) => {
    setSelectedInvoice(invoice);
    setShowDetailModal(true);
  };

  const handleOpenPaymentModal = (invoice: Invoice) => {
    setSelectedInvoice(invoice);
    paymentForm.setValue('invoiceId', invoice.id);
    paymentForm.setValue('amount', (invoice.totalAmount - invoice.paidAmount).toString());
    setShowPaymentModal(true);
  };

  const handlePrintInvoice = (invoice: Invoice) => {
    router.push(`/dashboard/billing/${invoice.id}/print`);
  };

  const columns: ColumnDef<Invoice>[] = [
    {
      accessorKey: 'invoiceNumber',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Invoice #" />,
      cell: ({ row }) => (
        <span className="font-mono font-medium">{row.original.invoiceNumber}</span>
      ),
    },
    {
      accessorKey: 'patient',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Patient" />,
      cell: ({ row }) => {
        const patient = row.original.order.patient;
        return (
          <div>
            <p className="font-medium">
              {patient.firstName} {patient.lastName}
            </p>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {patient.mrn}
            </p>
          </div>
        );
      },
    },
    {
      accessorKey: 'order.orderNumber',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Order #" />,
      cell: ({ row }) => (
        <span className="font-mono text-sm">{row.original.order.orderNumber}</span>
      ),
    },
    {
      accessorKey: 'totalAmount',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Total Amount" />,
      cell: ({ row }) => (
        <span className="font-medium">₹{row.original.totalAmount.toLocaleString()}</span>
      ),
    },
    {
      accessorKey: 'paidAmount',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Paid Amount" />,
      cell: ({ row }) => (
        <div>
          <p className="font-medium text-green-600">
            ₹{row.original.paidAmount.toLocaleString()}
          </p>
          {row.original.paidAmount < row.original.totalAmount && (
            <p className="text-xs text-gray-500">
              Due: ₹{(row.original.totalAmount - row.original.paidAmount).toLocaleString()}
            </p>
          )}
        </div>
      ),
    },
    {
      accessorKey: 'paymentStatus',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Status" />,
      cell: ({ row }) => <PaymentStatusBadge status={row.original.paymentStatus} />,
    },
    {
      accessorKey: 'paymentMethod',
      header: 'Payment Method',
      cell: ({ row }) => (
        <span className="text-sm">
          {row.original.paymentMethod || <span className="text-gray-400">-</span>}
        </span>
      ),
    },
    {
      accessorKey: 'issuedAt',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Issued Date" />,
      cell: ({ row }) => (
        <span className="text-sm">
          {new Date(row.original.issuedAt).toLocaleDateString()}
        </span>
      ),
    },
    {
      accessorKey: 'dueDate',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Due Date" />,
      cell: ({ row }) => {
        const dueDate = new Date(row.original.dueDate);
        const today = new Date();
        const isOverdue = dueDate < today && row.original.paymentStatus !== 'PAID';

        return (
          <span className={`text-sm ${isOverdue ? 'font-medium text-red-600' : ''}`}>
            {dueDate.toLocaleDateString()}
          </span>
        );
      },
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
              label: 'Print Invoice',
              onClick: () => handlePrintInvoice(row.original),
            },
            ...(row.original.paymentStatus !== 'PAID'
              ? [
                  {
                    label: 'Record Payment',
                    onClick: () => handleOpenPaymentModal(row.original),
                  },
                ]
              : []),
            {
              label: 'Delete',
              onClick: () => handleDelete(row.original.id),
              variant: 'destructive' as const,
            },
          ]}
        />
      ),
    },
  ];

  // Calculate summary stats
  const typedInvoices = (invoices as Invoice[]) || [];
  const totalRevenue = typedInvoices.reduce((sum, inv) => sum + inv.totalAmount, 0);
  const totalPaid = typedInvoices.reduce((sum, inv) => sum + inv.paidAmount, 0);
  const totalPending = typedInvoices.filter(inv => inv.paymentStatus === 'PARTIALLY_PAID').reduce((sum, inv) => sum + (inv.totalAmount - inv.paidAmount), 0);
  const totalOutstanding = typedInvoices.filter(inv => inv.paymentStatus === 'UNPAID' || inv.paymentStatus === 'OVERDUE').reduce((sum, inv) => sum + inv.totalAmount, 0);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Billing & Invoices
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            Manage invoices, payments, and billing records
          </p>
        </div>
        <Button onClick={() => setShowCreateModal(true)}>
          <Plus className="mr-2 h-4 w-4" />
          Create Invoice
        </Button>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-6 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Revenue</CardTitle>
            <DollarSign className="h-4 w-4 text-gray-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">₹{(totalRevenue / 100000).toFixed(1)}L</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              {typedInvoices.length} invoice(s)
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Paid</CardTitle>
            <CreditCard className="h-4 w-4 text-green-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600">
              ₹{(totalPaid / 100000).toFixed(1)}L
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              {totalRevenue > 0 ? ((totalPaid / totalRevenue) * 100).toFixed(1) : 0}% collected
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Partially Paid</CardTitle>
            <FileText className="h-4 w-4 text-orange-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-orange-600">
              ₹{(totalPending / 100000).toFixed(1)}L
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Pending collection</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Outstanding</CardTitle>
            <FileText className="h-4 w-4 text-red-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">
              ₹{(totalOutstanding / 100000).toFixed(1)}L
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Unpaid invoices</p>
          </CardContent>
        </Card>
      </div>

      {/* Invoices Table */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Invoices</CardTitle>
              <CardDescription>View and manage billing records</CardDescription>
            </div>
            <Button variant="outline">
              <Download className="mr-2 h-4 w-4" />
              Export
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {loading ? (
            <SkeletonTable />
          ) : error ? (
            <div className="p-4 text-center text-red-600">
              Error loading invoices: {error.message}
            </div>
          ) : (
            <DataTable
              columns={columns}
              data={typedInvoices}
              searchKey="invoiceNumber"
              searchPlaceholder="Search by invoice number..."
            />
          )}
        </CardContent>
      </Card>

      {/* Create Invoice Modal */}
      <FormModal
        open={showCreateModal}
        onClose={() => {
          setShowCreateModal(false);
          form.reset();
        }}
        title="Create Invoice"
        description="Generate invoice for an order"
        onSubmit={form.handleSubmit(handleCreateInvoice)}
        submitLabel="Create Invoice"
        isLoading={creating}
      >
        <Form {...form}>
          <div className="space-y-4">
            <FormField
              control={form.control}
              name="orderId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Order *</FormLabel>
                  <FormControl>
                    <select
                      {...field}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    >
                      <option value="">Select order...</option>
                      <option value="order-001">ORD-2024-001 - John Doe</option>
                      <option value="order-002">ORD-2024-002 - Jane Smith</option>
                      <option value="order-003">ORD-2024-003 - Bob Johnson</option>
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="amount"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Base Amount *</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      step="0.01"
                      min="0"
                      placeholder="0.00"
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>Base amount before discount and tax</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="grid gap-4 md:grid-cols-2">
              <FormField
                control={form.control}
                name="discount"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Discount</FormLabel>
                    <FormControl>
                      <Input
                        type="number"
                        step="0.01"
                        min="0"
                        placeholder="0.00"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>Discount amount</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="tax"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Tax / GST</FormLabel>
                    <FormControl>
                      <Input
                        type="number"
                        step="0.01"
                        min="0"
                        placeholder="0.00"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>Tax amount</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="dueDate"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Due Date *</FormLabel>
                  <FormControl>
                    <Input type="date" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="notes"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Notes</FormLabel>
                  <FormControl>
                    <textarea
                      {...field}
                      rows={3}
                      placeholder="Additional notes or payment terms..."
                      className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </Form>
      </FormModal>

      {/* Record Payment Modal */}
      <FormModal
        open={showPaymentModal}
        onClose={() => {
          setShowPaymentModal(false);
          paymentForm.reset();
        }}
        title="Record Payment"
        description={`Record payment for invoice ${selectedInvoice?.invoiceNumber}`}
        onSubmit={paymentForm.handleSubmit(handleRecordPayment)}
        submitLabel="Record Payment"
        isLoading={recording}
      >
        <Form {...paymentForm}>
          <div className="space-y-4">
            <div className="rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
              <div className="flex justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Total Amount:</span>
                <span className="font-medium">₹{selectedInvoice?.totalAmount.toLocaleString()}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Paid Amount:</span>
                <span className="font-medium text-green-600">
                  ₹{selectedInvoice?.paidAmount.toLocaleString()}
                </span>
              </div>
              <div className="flex justify-between border-t border-gray-200 pt-2 dark:border-gray-700">
                <span className="text-sm font-medium">Balance Due:</span>
                <span className="font-bold text-red-600">
                  ₹{selectedInvoice ? (selectedInvoice.totalAmount - selectedInvoice.paidAmount).toLocaleString() : 0}
                </span>
              </div>
            </div>

            <FormField
              control={paymentForm.control}
              name="amount"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Payment Amount *</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      step="0.01"
                      min="0.01"
                      placeholder="0.00"
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>
                    Enter full or partial payment amount
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={paymentForm.control}
              name="paymentMethod"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Payment Method *</FormLabel>
                  <FormControl>
                    <select
                      {...field}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    >
                      <option value="CASH">Cash</option>
                      <option value="CARD">Credit/Debit Card</option>
                      <option value="UPI">UPI</option>
                      <option value="NET_BANKING">Net Banking</option>
                      <option value="CHEQUE">Cheque</option>
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={paymentForm.control}
              name="transactionId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Transaction ID / Reference</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., TXN123456789" {...field} />
                  </FormControl>
                  <FormDescription>
                    Transaction ID, cheque number, or reference
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={paymentForm.control}
              name="notes"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Notes</FormLabel>
                  <FormControl>
                    <textarea
                      {...field}
                      rows={2}
                      placeholder="Additional payment notes..."
                      className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </Form>
      </FormModal>

      {/* Invoice Detail Modal */}
      <Modal open={showDetailModal} onClose={() => setShowDetailModal(false)}>
        <ModalHeader>
          <ModalTitle>Invoice Details</ModalTitle>
        </ModalHeader>
        <ModalBody>
          {selectedInvoice && (
            <div className="space-y-4">
              <div>
                <h3 className="mb-3 font-semibold">Invoice Information</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Invoice Number:</span>
                    <span className="font-mono font-medium">{selectedInvoice.invoiceNumber}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Order Number:</span>
                    <span className="font-mono font-medium">{selectedInvoice.order.orderNumber}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Status:</span>
                    <PaymentStatusBadge status={selectedInvoice.paymentStatus} />
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Patient Information</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Name:</span>
                    <span className="font-medium">
                      {selectedInvoice.order.patient.firstName} {selectedInvoice.order.patient.lastName}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">MRN:</span>
                    <span className="font-medium">{selectedInvoice.order.patient.mrn}</span>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Amount Breakdown</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Base Amount:</span>
                    <span className="font-medium">₹{selectedInvoice.amount.toLocaleString()}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Discount:</span>
                    <span className="font-medium text-red-600">
                      -₹{selectedInvoice.discount.toLocaleString()}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Tax:</span>
                    <span className="font-medium">₹{selectedInvoice.tax.toLocaleString()}</span>
                  </div>
                  <div className="flex justify-between border-t border-gray-200 pt-2 dark:border-gray-700">
                    <span className="font-semibold">Total Amount:</span>
                    <span className="font-bold">₹{selectedInvoice.totalAmount.toLocaleString()}</span>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Payment Information</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Paid Amount:</span>
                    <span className="font-medium text-green-600">
                      ₹{selectedInvoice.paidAmount.toLocaleString()}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Balance Due:</span>
                    <span className="font-medium text-red-600">
                      ₹{(selectedInvoice.totalAmount - selectedInvoice.paidAmount).toLocaleString()}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Payment Method:</span>
                    <span className="font-medium">{selectedInvoice.paymentMethod || 'N/A'}</span>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Dates</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Issued Date:</span>
                    <span className="font-medium">
                      {new Date(selectedInvoice.issuedAt).toLocaleDateString()}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Due Date:</span>
                    <span className="font-medium">
                      {new Date(selectedInvoice.dueDate).toLocaleDateString()}
                    </span>
                  </div>
                  {selectedInvoice.paidAt && (
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Paid Date:</span>
                      <span className="font-medium">
                        {new Date(selectedInvoice.paidAt).toLocaleDateString()}
                      </span>
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}
        </ModalBody>
        <ModalFooter>
          <Button variant="outline" onClick={() => setShowDetailModal(false)}>
            Close
          </Button>
          {selectedInvoice && selectedInvoice.paymentStatus !== 'PAID' && (
            <Button
              onClick={() => {
                setShowDetailModal(false);
                handleOpenPaymentModal(selectedInvoice);
              }}
            >
              <CreditCard className="mr-2 h-4 w-4" />
              Record Payment
            </Button>
          )}
          {selectedInvoice && (
            <Button
              onClick={() => {
                setShowDetailModal(false);
                handlePrintInvoice(selectedInvoice);
              }}
            >
              <Printer className="mr-2 h-4 w-4" />
              Print Invoice
            </Button>
          )}
        </ModalFooter>
      </Modal>
    </div>
  );
}
