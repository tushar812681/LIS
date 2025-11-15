'use client';

import * as React from 'react';
import { useParams, useRouter } from 'next/navigation';
import { format } from 'date-fns';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { FormModal } from '@/components/ui/form-modal';
import { useConfirmDialog } from '@/components/ui/alert-dialog';
import { OrderStatusBadge, SampleStatusBadge, ResultStatusBadge } from '@/components/ui/status-badge';
import { SkeletonCard } from '@/components/ui/skeleton';
import { useOrder, useCancelOrder } from '@/lib/hooks';
import { useNotificationStore } from '@/lib/store';
import {
  ArrowLeft,
  User,
  Calendar,
  Clock,
  FileText,
  TestTube,
  AlertCircle,
  CheckCircle,
  XCircle,
  Edit,
  Printer,
  Download,
} from 'lucide-react';

export const dynamic = 'force-dynamic';

interface Order {
  id: string;
  orderId: string;
  totalTests: number;
  completedTests: number;
  status: string;
  createdAt: string;
  updatedAt: string;
  orderDate: string;
  priority: string;
  patient: {
    id: string;
    patientId: string;
    firstName: string;
    lastName: string;
    dateOfBirth: string;
    gender: string;
    phone: string;
  };
  tests: Array<{
    id: string;
    testName: string;
    testCode: string;
    category: string;
    price: number;
    sample?: { sampleId: string; status: string };
    result?: { id: string; status: string };
  }>;
  billingInfo?: {
    totalAmount: number;
    paidAmount: number;
    balance: number;
  };
  clinicalInfo?: {
    diagnosis?: string;
    symptoms?: string;
    notes?: string;
  };
  doctor?: {
    name: string;
    specialty?: string;
  };
  createdBy?: {
    name: string;
  };
}

export default function OrderDetailPage() {
  const params = useParams();
  const router = useRouter();
  const orderId = params.id as string;

  const [mounted, setMounted] = React.useState(false);
  const [cancelReason, setCancelReason] = React.useState('');
  const [showCancelModal, setShowCancelModal] = React.useState(false);

  const { order, loading, error, refetch } = useOrder(orderId);
  const { cancelOrder, loading: cancelling } = useCancelOrder();
  const { ConfirmDialog } = useConfirmDialog();
  const addNotification = useNotificationStore((state) => state.addNotification);

  React.useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) return null;

  const handleCancelOrder = async () => {
    if (!cancelReason.trim()) {
      addNotification({
        type: 'error',
        title: 'Cancellation Failed',
        message: 'Please provide a reason for cancellation',
      });
      return;
    }

    try {
      await cancelOrder({
        variables: {
          id: orderId,
          reason: cancelReason,
        },
      });

      addNotification({
        type: 'success',
        title: 'Order Cancelled',
        message: 'Order has been cancelled successfully',
      });
      setShowCancelModal(false);
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Cancellation Failed',
        message: error instanceof Error ? error.message : 'Failed to cancel order',
      });
    }
  };

  if (loading) {
    return (
      <div className="space-y-6">
        <SkeletonCard />
        <SkeletonCard />
        <SkeletonCard />
      </div>
    );
  }

  if (error || !order) {
    return (
      <div className="flex flex-col items-center justify-center py-12">
        <AlertCircle className="h-12 w-12 text-red-500" />
        <h2 className="mt-4 text-lg font-semibold">Order not found</h2>
        <p className="mt-2 text-sm text-gray-600">
          The order you&apos;re looking for doesn&apos;t exist or has been deleted.
        </p>
        <Button onClick={() => router.push('/dashboard/orders')} className="mt-4">
          Back to Orders
        </Button>
      </div>
    );
  }

  const typedOrder = order as Order;

  const completionPercentage = typedOrder.totalTests > 0
    ? Math.round((typedOrder.completedTests / typedOrder.totalTests) * 100)
    : 0;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => router.push('/dashboard/orders')}
          >
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back
          </Button>
          <div>
            <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-100">
              Order {typedOrder.orderId}
            </h1>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Created on {format(new Date(typedOrder.createdAt), 'MMM dd, yyyy HH:mm')}
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <Button variant="outline">
            <Printer className="mr-2 h-4 w-4" />
            Print
          </Button>
          {typedOrder.status !== 'CANCELLED' && typedOrder.status !== 'COMPLETED' && (
            <Button
              variant="destructive"
              onClick={() => setShowCancelModal(true)}
            >
              <XCircle className="mr-2 h-4 w-4" />
              Cancel Order
            </Button>
          )}
        </div>
      </div>

      {/* Order Status & Progress */}
      <Card className="p-6">
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Status</p>
              <OrderStatusBadge status={typedOrder.status} />
            </div>
            <div className="text-right">
              <p className="text-sm text-gray-600 dark:text-gray-400">Progress</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-gray-100">
                {typedOrder.completedTests}/{typedOrder.totalTests}
              </p>
            </div>
          </div>

          {/* Progress Bar */}
          <div className="h-2 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-800">
            <div
              className="h-full bg-blue-600 transition-all duration-300"
              style={{ width: `${completionPercentage}%` }}
            />
          </div>
        </div>
      </Card>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Main Content */}
        <div className="space-y-6 lg:col-span-2">
          {/* Patient Information */}
          <Card className="p-6">
            <h2 className="mb-4 text-lg font-semibold">Patient Information</h2>
            <div className="flex items-start gap-4">
              <div className="flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                <User className="h-6 w-6 text-blue-600 dark:text-blue-300" />
              </div>
              <div className="flex-1">
                <p className="font-medium text-gray-900 dark:text-gray-100">
                  {typedOrder.patient.firstName} {typedOrder.patient.lastName}
                </p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Patient ID: {typedOrder.patient.patientId}
                </p>
                <div className="mt-2 grid gap-2 text-sm md:grid-cols-2">
                  <div>
                    <span className="text-gray-600 dark:text-gray-400">DOB: </span>
                    <span className="font-medium">
                      {format(new Date(typedOrder.patient.dateOfBirth), 'MMM dd, yyyy')}
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-gray-400">Gender: </span>
                    <span className="font-medium">{typedOrder.patient.gender}</span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-gray-400">Phone: </span>
                    <span className="font-medium">{typedOrder.patient.phone}</span>
                  </div>
                </div>
              </div>
              <Button
                variant="outline"
                size="sm"
                onClick={() => router.push(`/dashboard/patients/${typedOrder.patient.id}`)}
              >
                View Profile
              </Button>
            </div>
          </Card>

          {/* Tests */}
          <Card className="p-6">
            <h2 className="mb-4 text-lg font-semibold">Ordered Tests ({typedOrder.tests.length})</h2>
            <div className="space-y-3">
              {typedOrder.tests.map((test: { id: string; testName: string; testCode: string; category: string; price: number; sample?: { sampleId: string; status: string }; result?: { id: string; status: string } }) => (
                <div
                  key={test.id}
                  className="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-800"
                >
                  <div className="flex-1">
                    <div className="flex items-center gap-3">
                      <TestTube className="h-5 w-5 text-gray-400" />
                      <div>
                        <p className="font-medium text-gray-900 dark:text-gray-100">
                          {test.testName}
                        </p>
                        <p className="text-sm text-gray-600 dark:text-gray-400">
                          Code: {test.testCode} • Category: {test.category}
                        </p>
                      </div>
                    </div>

                    {/* Sample Status */}
                    {test.sample && (
                      <div className="ml-8 mt-2 flex items-center gap-2 text-sm">
                        <span className="text-gray-600 dark:text-gray-400">Sample:</span>
                        <SampleStatusBadge status={test.sample.status} />
                        <span className="text-gray-600 dark:text-gray-400">
                          {test.sample.sampleId}
                        </span>
                      </div>
                    )}

                    {/* Result Status */}
                    {test.result && (
                      <div className="ml-8 mt-1 flex items-center gap-2 text-sm">
                        <span className="text-gray-600 dark:text-gray-400">Result:</span>
                        <ResultStatusBadge status={test.result.status} />
                      </div>
                    )}
                  </div>

                  <div className="flex items-center gap-2">
                    <p className="font-medium text-gray-900 dark:text-gray-100">
                      ₹{test.price}
                    </p>
                    {test.result?.status === 'APPROVED' && (
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => router.push(`/dashboard/results/${test.result!.id}`)}
                      >
                        View Result
                      </Button>
                    )}
                  </div>
                </div>
              ))}
            </div>

            {/* Billing Summary */}
            {typedOrder.billingInfo && (
              <div className="mt-4 rounded-lg bg-gray-50 p-4 dark:bg-gray-800">
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600 dark:text-gray-400">Subtotal</span>
                    <span className="font-medium">₹{typedOrder.billingInfo.totalAmount}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600 dark:text-gray-400">Paid</span>
                    <span className="font-medium text-green-600">
                      -₹{typedOrder.billingInfo.paidAmount}
                    </span>
                  </div>
                  <div className="flex justify-between border-t border-gray-200 pt-2 dark:border-gray-700">
                    <span className="font-semibold text-gray-900 dark:text-gray-100">
                      Balance
                    </span>
                    <span className="text-lg font-bold text-gray-900 dark:text-gray-100">
                      ₹{typedOrder.billingInfo.balance}
                    </span>
                  </div>
                </div>
              </div>
            )}
          </Card>

          {/* Clinical Information */}
          {typedOrder.clinicalInfo && (
            <Card className="p-6">
              <h2 className="mb-4 text-lg font-semibold">Clinical Information</h2>
              <div className="space-y-3">
                {typedOrder.clinicalInfo.diagnosis && (
                  <div>
                    <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                      Diagnosis
                    </p>
                    <p className="text-gray-900 dark:text-gray-100">
                      {typedOrder.clinicalInfo.diagnosis}
                    </p>
                  </div>
                )}
                {typedOrder.clinicalInfo.symptoms && (
                  <div>
                    <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                      Symptoms
                    </p>
                    <p className="text-gray-900 dark:text-gray-100">
                      {typedOrder.clinicalInfo.symptoms}
                    </p>
                  </div>
                )}
                {typedOrder.clinicalInfo.notes && (
                  <div>
                    <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                      Notes
                    </p>
                    <p className="text-gray-900 dark:text-gray-100">
                      {typedOrder.clinicalInfo.notes}
                    </p>
                  </div>
                )}
              </div>
            </Card>
          )}
        </div>

        {/* Sidebar */}
        <div className="space-y-6">
          {/* Order Details */}
          <Card className="p-6">
            <h3 className="mb-4 font-semibold">Order Details</h3>
            <div className="space-y-3 text-sm">
              <div className="flex items-center gap-2">
                <Calendar className="h-4 w-4 text-gray-400" />
                <div>
                  <p className="text-gray-600 dark:text-gray-400">Order Date</p>
                  <p className="font-medium">
                    {format(new Date(typedOrder.orderDate), 'MMM dd, yyyy')}
                  </p>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <Clock className="h-4 w-4 text-gray-400" />
                <div>
                  <p className="text-gray-600 dark:text-gray-400">Priority</p>
                  <p className="font-medium">{typedOrder.priority}</p>
                </div>
              </div>

              {typedOrder.doctor && (
                <div className="flex items-center gap-2">
                  <User className="h-4 w-4 text-gray-400" />
                  <div>
                    <p className="text-gray-600 dark:text-gray-400">Ordering Physician</p>
                    <p className="font-medium">{typedOrder.doctor.name}</p>
                    {typedOrder.doctor.specialty && (
                      <p className="text-xs text-gray-500">{typedOrder.doctor.specialty}</p>
                    )}
                  </div>
                </div>
              )}

              {typedOrder.createdBy && (
                <div className="flex items-center gap-2">
                  <User className="h-4 w-4 text-gray-400" />
                  <div>
                    <p className="text-gray-600 dark:text-gray-400">Created By</p>
                    <p className="font-medium">{typedOrder.createdBy.name}</p>
                  </div>
                </div>
              )}
            </div>
          </Card>

          {/* Quick Actions */}
          <Card className="p-6">
            <h3 className="mb-4 font-semibold">Quick Actions</h3>
            <div className="space-y-2">
              <Button variant="outline" className="w-full justify-start">
                <FileText className="mr-2 h-4 w-4" />
                Generate Report
              </Button>
              <Button variant="outline" className="w-full justify-start">
                <Download className="mr-2 h-4 w-4" />
                Download Invoice
              </Button>
              <Button
                variant="outline"
                className="w-full justify-start"
                onClick={() => router.push(`/dashboard/orders/${orderId}/edit`)}
              >
                <Edit className="mr-2 h-4 w-4" />
                Edit Order
              </Button>
            </div>
          </Card>

          {/* Timeline */}
          <Card className="p-6">
            <h3 className="mb-4 font-semibold">Timeline</h3>
            <div className="space-y-4">
              <div className="flex gap-3">
                <div className="flex flex-col items-center">
                  <div className="flex h-8 w-8 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                    <CheckCircle className="h-4 w-4 text-blue-600 dark:text-blue-300" />
                  </div>
                  <div className="h-full w-px bg-gray-200 dark:bg-gray-700" />
                </div>
                <div className="pb-4">
                  <p className="font-medium text-gray-900 dark:text-gray-100">Order Created</p>
                  <p className="text-xs text-gray-600 dark:text-gray-400">
                    {format(new Date(typedOrder.createdAt), 'MMM dd, yyyy HH:mm')}
                  </p>
                  <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                    Order placed by {typedOrder.createdBy?.name}
                  </p>
                </div>
              </div>

              {typedOrder.updatedAt !== typedOrder.createdAt && (
                <div className="flex gap-3">
                  <div className="flex flex-col items-center">
                    <div className="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
                      <Clock className="h-4 w-4 text-gray-600 dark:text-gray-400" />
                    </div>
                  </div>
                  <div>
                    <p className="font-medium text-gray-900 dark:text-gray-100">
                      Last Updated
                    </p>
                    <p className="text-xs text-gray-600 dark:text-gray-400">
                      {format(new Date(typedOrder.updatedAt), 'MMM dd, yyyy HH:mm')}
                    </p>
                  </div>
                </div>
              )}
            </div>
          </Card>
        </div>
      </div>

      {/* Cancel Order Modal */}
      <FormModal
        open={showCancelModal}
        onClose={() => setShowCancelModal(false)}
        title="Cancel Order"
        description="Please provide a reason for cancelling this order"
        onSubmit={(e) => {
          e.preventDefault();
          handleCancelOrder();
        }}
        submitLabel="Cancel Order"
        isSubmitting={cancelling}
      >
        <div>
          <label className="mb-2 block text-sm font-medium">Reason for Cancellation *</label>
          <textarea
            value={cancelReason}
            onChange={(e) => setCancelReason(e.target.value)}
            rows={4}
            className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
            placeholder="Enter reason for cancelling this typedOrder..."
          />
        </div>
      </FormModal>

      {ConfirmDialog}
    </div>
  );
}
