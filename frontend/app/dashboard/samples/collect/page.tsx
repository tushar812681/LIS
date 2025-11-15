'use client';

import * as React from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card } from '@/components/ui/card';
import { SearchBar } from '@/components/ui/search-bar';
import { SkeletonCard } from '@/components/ui/skeleton';
import { useCollectSample } from '@/lib/hooks';
import { useOrders } from '@/lib/hooks';
import { useNotificationStore } from '@/lib/store';
import {
  TestTube,
  User,

  Clock,
  Droplet,
  Package,

  CheckCircle,

  QrCode,
} from 'lucide-react';

const sampleSchema = z.object({
  orderId: z.string().min(1, 'Order is required'),
  testId: z.string().min(1, 'Test is required'),
  sampleType: z.string().min(1, 'Sample type is required'),
  containerType: z.string().min(1, 'Container type is required'),
  volume: z.string().optional(),
  collectionDate: z.string().min(1, 'Collection date is required'),
  collectionTime: z.string().min(1, 'Collection time is required'),
  collectionSite: z.string().optional(),
  collectionMethod: z.string().optional(),
  fastingStatus: z.enum(['FASTING', 'NON_FASTING', 'NOT_APPLICABLE'], {
    message: 'Fasting status is required',
  }),
  notes: z.string().optional(),
});

type SampleFormValues = z.infer<typeof sampleSchema>;

const SAMPLE_TYPES = [
  'Blood - Whole',
  'Blood - Serum',
  'Blood - Plasma',
  'Urine',
  'Stool',
  'Sputum',
  'CSF',
  'Tissue',
  'Swab',
  'Other',
];

const CONTAINER_TYPES = [
  'Red Top (No Additive)',
  'Yellow Top (SST)',
  'Purple Top (EDTA)',
  'Blue Top (Citrate)',
  'Green Top (Heparin)',
  'Gray Top (Fluoride)',
  'Urine Container',
  'Sterile Container',
  'Other',
];

function CollectSamplePageContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const orderIdParam = searchParams.get('orderId');

  const [mounted, setMounted] = React.useState(false);
  const [step, setStep] = React.useState(1); // 1: Select Order, 2: Collection Details
  const [selectedOrder, setSelectedOrder] = React.useState<{ id: string; orderId: string; patient: { firstName: string; lastName: string; patientId: string }; totalTests: number; priority: string; tests?: { id: string; testName: string; testCode: string; category: string; sampleType?: string }[] } | null>(null);
  const [selectedTest, setSelectedTest] = React.useState<{ id: string; testName: string; testCode: string; category: string; sampleType?: string } | null>(null);
  const [searchQuery, setSearchQuery] = React.useState('');

  const addNotification = useNotificationStore((state) => state.addNotification);

  const { collectSample, loading } = useCollectSample();
  const { orders, loading: loadingOrders } = useOrders({
    search: searchQuery,
    filters: { status: 'CONFIRMED' }, // Only show confirmed orders
    limit: 20,
  });

  // Type the orders array
  const typedOrders = (orders as { id: string; orderId: string; patient: { firstName: string; lastName: string; patientId: string }; totalTests: number; priority: string; tests?: { id: string; testName: string; testCode: string; category: string; sampleType?: string }[] }[]) || [];

  const form = useForm<SampleFormValues>({
    resolver: zodResolver(sampleSchema),
    defaultValues: {
      orderId: orderIdParam || '',
      testId: '',
      sampleType: '',
      containerType: '',
      volume: '',
      collectionDate: new Date().toISOString().split('T')[0],
      collectionTime: new Date().toTimeString().slice(0, 5),
      collectionSite: '',
      collectionMethod: '',
      fastingStatus: 'NOT_APPLICABLE',
      notes: '',
    },
  });

  React.useEffect(() => {
    setMounted(true);
  }, []);

  const handleSelectOrder = (order: typeof selectedOrder) => {
    setSelectedOrder(order);
    if (order) {
      form.setValue('orderId', order.id);
    }
    setStep(2);
  };

  React.useEffect(() => {
    if (orderIdParam && !selectedOrder) {
      // If order ID is provided, find and select it
      const order = typedOrders.find(o => o.id === orderIdParam);
      if (order) {
        handleSelectOrder(order);
      }
    }
  }, [orderIdParam, typedOrders, selectedOrder, handleSelectOrder]);

  if (!mounted) return null;

  const handleSelectTest = (test: typeof selectedTest) => {
    setSelectedTest(test);
    if (test) {
      form.setValue('testId', test.id);
      // Auto-populate sample type based on test
      if (test.sampleType) {
        form.setValue('sampleType', test.sampleType);
      }
    }
  };

  const handleSubmit = async (values: SampleFormValues) => {
    try {
      const result = await collectSample({
        variables: {
          input: {
            orderId: values.orderId,
            testId: values.testId,
            sampleType: values.sampleType,
            containerType: values.containerType,
            volume: values.volume || null,
            collectionDate: values.collectionDate,
            collectionTime: values.collectionTime,
            collectionSite: values.collectionSite || null,
            collectionMethod: values.collectionMethod || null,
            fastingStatus: values.fastingStatus,
            notes: values.notes || null,
          },
        },
      });

      if (result.data) {
        addNotification({
          type: 'success',
          title: 'Sample Collected',
          message: `Sample ${(result.data as { collectSample: { sampleId: string } }).collectSample.sampleId} collected successfully!`,
        });
        router.push(`/dashboard/samples/${(result.data as { collectSample: { id: string } }).collectSample.id}`);
      }
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Collection Failed',
        message: error instanceof Error ? error.message : 'Failed to collect sample',
      });
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Collect Sample</h1>
        <p className="text-gray-600 dark:text-gray-400">
          Register sample collection for laboratory testing
        </p>
      </div>

      {/* Progress Steps */}
      <div className="flex items-center justify-center gap-4">
        <div className="flex items-center">
          <div
            className={`flex h-10 w-10 items-center justify-center rounded-full border-2 ${
              step >= 1
                ? 'border-blue-600 bg-blue-600 text-white'
                : 'border-gray-300 bg-white text-gray-400'
            }`}
          >
            {step > 1 ? <CheckCircle className="h-5 w-5" /> : <span>1</span>}
          </div>
          <p className="ml-2 text-sm font-medium">Select Order</p>
        </div>
        <div className="h-0.5 w-16 bg-gray-300" />
        <div className="flex items-center">
          <div
            className={`flex h-10 w-10 items-center justify-center rounded-full border-2 ${
              step >= 2
                ? 'border-blue-600 bg-blue-600 text-white'
                : 'border-gray-300 bg-white text-gray-400'
            }`}
          >
            <span>2</span>
          </div>
          <p className="ml-2 text-sm font-medium">Collection Details</p>
        </div>
      </div>

      <form onSubmit={form.handleSubmit(handleSubmit)}>
        {/* Step 1: Select Order */}
        {step === 1 && (
          <Card className="p-6">
            <h2 className="mb-4 text-xl font-semibold">Select Order</h2>
            <div className="space-y-4">
              <SearchBar
                value={searchQuery}
                onChange={setSearchQuery}
                placeholder="Search by order ID, patient name, or patient ID..."
                isLoading={loadingOrders}
              />

              {loadingOrders ? (
                <SkeletonCard />
              ) : typedOrders.length === 0 ? (
                <div className="py-12 text-center text-gray-600 dark:text-gray-400">
                  {searchQuery
                    ? 'No orders found. Try a different search.'
                    : 'No confirmed orders available for sample collection.'}
                </div>
              ) : (
                <div className="space-y-3">
                  {typedOrders.map((order) => (
                    <div
                      key={order.id}
                      onClick={() => handleSelectOrder(order as typeof selectedOrder)}
                      className="cursor-pointer rounded-lg border border-gray-200 p-4 transition-colors hover:bg-gray-50 dark:border-gray-800 dark:hover:bg-gray-800"
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex items-start gap-3">
                          <div className="flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                            <TestTube className="h-6 w-6 text-blue-600 dark:text-blue-300" />
                          </div>
                          <div>
                            <p className="font-medium text-gray-900 dark:text-gray-100">
                              Order {order.orderId}
                            </p>
                            <p className="text-sm text-gray-600 dark:text-gray-400">
                              Patient: {order.patient.firstName} {order.patient.lastName} (
                              {order.patient.patientId})
                            </p>
                            <p className="mt-1 text-xs text-gray-500 dark:text-gray-500">
                              {order.totalTests} test(s) • Priority: {order.priority}
                            </p>
                          </div>
                        </div>
                        <Button type="button" size="sm">
                          Select
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </Card>
        )}

        {/* Step 2: Collection Details */}
        {step === 2 && (
          <div className="space-y-6">
            {/* Selected Order Info */}
            {selectedOrder && (
              <Card className="p-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="flex h-10 w-10 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                      <User className="h-5 w-5 text-blue-600 dark:text-blue-300" />
                    </div>
                    <div>
                      <p className="font-medium text-gray-900 dark:text-gray-100">
                        {selectedOrder.patient.firstName} {selectedOrder.patient.lastName}
                      </p>
                      <p className="text-sm text-gray-600 dark:text-gray-400">
                        Order {selectedOrder.orderId} • {selectedOrder.totalTests} test(s)
                      </p>
                    </div>
                  </div>
                  <Button type="button" variant="outline" size="sm" onClick={() => setStep(1)}>
                    Change Order
                  </Button>
                </div>
              </Card>
            )}

            {/* Test Selection */}
            <Card className="p-6">
              <h3 className="mb-4 text-lg font-semibold">Select Test</h3>
              <div className="space-y-2">
                {selectedOrder?.tests?.map((test) => (
                  <div
                    key={test.id}
                    onClick={() => handleSelectTest(test)}
                    className={`cursor-pointer rounded-lg border p-4 transition-colors ${
                      selectedTest?.id === test.id
                        ? 'border-blue-500 bg-blue-50 dark:border-blue-600 dark:bg-blue-950/30'
                        : 'border-gray-200 hover:bg-gray-50 dark:border-gray-800 dark:hover:bg-gray-800'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <div>
                        <p className="font-medium text-gray-900 dark:text-gray-100">
                          {test.testName}
                        </p>
                        <p className="text-sm text-gray-600 dark:text-gray-400">
                          {test.testCode} • {test.category}
                        </p>
                      </div>
                      {selectedTest?.id === test.id && (
                        <CheckCircle className="h-5 w-5 text-blue-600 dark:text-blue-400" />
                      )}
                    </div>
                  </div>
                ))}
              </div>
              {form.formState.errors.testId && (
                <p className="mt-2 text-sm text-red-600">
                  {form.formState.errors.testId.message}
                </p>
              )}
            </Card>

            {/* Collection Details Form */}
            {selectedTest && (
              <Card className="p-6">
                <h3 className="mb-4 text-lg font-semibold">Collection Details</h3>
                <div className="space-y-4">
                  {/* Sample Information */}
                  <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                    <h4 className="mb-3 flex items-center gap-2 font-medium">
                      <Droplet className="h-4 w-4" />
                      Sample Information
                    </h4>
                    <div className="grid gap-4 md:grid-cols-2">
                      <div>
                        <label className="mb-1 block text-sm font-medium">Sample Type *</label>
                        <select
                          {...form.register('sampleType')}
                          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        >
                          <option value="">Select type</option>
                          {SAMPLE_TYPES.map((type) => (
                            <option key={type} value={type}>
                              {type}
                            </option>
                          ))}
                        </select>
                        {form.formState.errors.sampleType && (
                          <p className="mt-1 text-sm text-red-600">
                            {form.formState.errors.sampleType.message}
                          </p>
                        )}
                      </div>

                      <div>
                        <label className="mb-1 block text-sm font-medium">
                          Container Type *
                        </label>
                        <select
                          {...form.register('containerType')}
                          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        >
                          <option value="">Select container</option>
                          {CONTAINER_TYPES.map((type) => (
                            <option key={type} value={type}>
                              {type}
                            </option>
                          ))}
                        </select>
                        {form.formState.errors.containerType && (
                          <p className="mt-1 text-sm text-red-600">
                            {form.formState.errors.containerType.message}
                          </p>
                        )}
                      </div>

                      <div>
                        <label className="mb-1 block text-sm font-medium">
                          Volume (optional)
                        </label>
                        <Input placeholder="e.g., 5 ml" {...form.register('volume')} />
                      </div>

                      <div>
                        <label className="mb-1 block text-sm font-medium">
                          Fasting Status *
                        </label>
                        <select
                          {...form.register('fastingStatus')}
                          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        >
                          <option value="NOT_APPLICABLE">Not Applicable</option>
                          <option value="FASTING">Fasting</option>
                          <option value="NON_FASTING">Non-Fasting</option>
                        </select>
                      </div>
                    </div>
                  </div>

                  {/* Collection Time */}
                  <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                    <h4 className="mb-3 flex items-center gap-2 font-medium">
                      <Clock className="h-4 w-4" />
                      Collection Time
                    </h4>
                    <div className="grid gap-4 md:grid-cols-2">
                      <div>
                        <label className="mb-1 block text-sm font-medium">Date *</label>
                        <Input type="date" {...form.register('collectionDate')} />
                        {form.formState.errors.collectionDate && (
                          <p className="mt-1 text-sm text-red-600">
                            {form.formState.errors.collectionDate.message}
                          </p>
                        )}
                      </div>

                      <div>
                        <label className="mb-1 block text-sm font-medium">Time *</label>
                        <Input type="time" {...form.register('collectionTime')} />
                        {form.formState.errors.collectionTime && (
                          <p className="mt-1 text-sm text-red-600">
                            {form.formState.errors.collectionTime.message}
                          </p>
                        )}
                      </div>
                    </div>
                  </div>

                  {/* Additional Details */}
                  <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                    <h4 className="mb-3 flex items-center gap-2 font-medium">
                      <Package className="h-4 w-4" />
                      Additional Details
                    </h4>
                    <div className="space-y-4">
                      <div>
                        <label className="mb-1 block text-sm font-medium">
                          Collection Site (optional)
                        </label>
                        <Input
                          placeholder="e.g., Left arm"
                          {...form.register('collectionSite')}
                        />
                      </div>

                      <div>
                        <label className="mb-1 block text-sm font-medium">
                          Collection Method (optional)
                        </label>
                        <Input
                          placeholder="e.g., Venipuncture"
                          {...form.register('collectionMethod')}
                        />
                      </div>

                      <div>
                        <label className="mb-1 block text-sm font-medium">Notes (optional)</label>
                        <textarea
                          {...form.register('notes')}
                          rows={3}
                          className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                          placeholder="Any additional notes about the collection..."
                        />
                      </div>
                    </div>
                  </div>
                </div>
              </Card>
            )}

            {/* Action Buttons */}
            <div className="flex justify-between">
              <Button type="button" variant="outline" onClick={() => setStep(1)}>
                Previous
              </Button>
              <div className="flex gap-2">
                <Button
                  type="button"
                  variant="outline"
                  disabled={!selectedTest}
                  onClick={() => {
                    // Print barcode functionality
                    addNotification({
                      type: 'info',
                      title: 'Print Barcode',
                      message: 'Barcode printing will be available soon',
                    });
                  }}
                >
                  <QrCode className="mr-2 h-4 w-4" />
                  Print Barcode
                </Button>
                <Button type="submit" disabled={loading || !selectedTest}>
                  {loading ? 'Collecting...' : 'Collect Sample'}
                </Button>
              </div>
            </div>
          </div>
        )}
      </form>
    </div>
  );
}

export default function CollectSamplePage() {
  return (
    <React.Suspense fallback={<SkeletonCard />}>
      <CollectSamplePageContent />
    </React.Suspense>
  );
}
