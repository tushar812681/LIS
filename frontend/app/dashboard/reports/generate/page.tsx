'use client';

import { useState, useEffect, useRef, Suspense } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { SearchBar } from '@/components/ui/search-bar';
import { useNotificationStore } from '@/lib/store';
import { useOrders, useGenerateReport } from '@/lib/hooks';
import { SkeletonCard } from '@/components/ui/skeleton';
import {
  FileText,
  Search,
  CheckCircle2,
  ArrowRight,
  Loader2,
  FileBarChart,
  FileCheck
} from 'lucide-react';

const reportSchema = z.object({
  orderId: z.string().min(1, 'Order is required'),
  templateId: z.string().min(1, 'Report template is required'),
  reportType: z.enum(['STANDARD', 'DETAILED', 'SUMMARY']),
  includeGraphs: z.boolean(),
  includeReferenceRanges: z.boolean(),
  includeInterpretation: z.boolean(),
  additionalComments: z.string().optional(),
  deliveryMethod: z.enum(['EMAIL', 'PRINT', 'PORTAL']),
});

type ReportFormValues = z.infer<typeof reportSchema>;

interface Order {
  id: string;
  orderNumber: string;
  patient: {
    firstName: string;
    lastName: string;
    mrn: string;
    age: number;
    gender: string;
  };
  tests: Array<{
    id: string;
    test: {
      name: string;
      code: string;
    };
    results: Array<{
      resultStatus: string;
    }>;
  }>;
  orderStatus: string;
  createdAt: string;
}

const reportTemplates = [
  {
    id: 'template-001',
    name: 'Standard Laboratory Report',
    description: 'Default report template with all test results',
    icon: FileText,
    category: 'General',
  },
  {
    id: 'template-002',
    name: 'Comprehensive Report',
    description: 'Detailed report with graphs, trends, and interpretation',
    icon: FileBarChart,
    category: 'Detailed',
  },
  {
    id: 'template-003',
    name: 'Summary Report',
    description: 'Concise report with only critical findings',
    icon: FileCheck,
    category: 'Summary',
  },
  {
    id: 'template-004',
    name: 'Hematology Panel Report',
    description: 'Specialized template for hematology tests',
    icon: FileText,
    category: 'Specialized',
  },
  {
    id: 'template-005',
    name: 'Chemistry Panel Report',
    description: 'Specialized template for chemistry tests',
    icon: FileText,
    category: 'Specialized',
  },
  {
    id: 'template-006',
    name: 'Microbiology Report',
    description: 'Template for culture and sensitivity reports',
    icon: FileText,
    category: 'Specialized',
  },
];

function ReportGenerationPageContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const orderIdParam = searchParams.get('orderId');
  const processedOrderIdRef = useRef(false);
  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedOrder, setSelectedOrder] = useState<Order | null>(null);
  const [selectedTemplate, setSelectedTemplate] = useState<string>('');
  const addNotification = useNotificationStore((state) => state.addNotification);

  // GraphQL hooks
  const { orders, loading: loadingOrders } = useOrders({
    search: searchQuery,
    filters: { status: 'COMPLETED' },
    limit: 20,
  });
  const { generateReport, loading: generating } = useGenerateReport();

  // Type the orders array
  const typedOrders = (orders as Order[]) || [];

  // Form hooks - must be called before any early returns
  const form = useForm<ReportFormValues>({
    resolver: zodResolver(reportSchema),
    defaultValues: {
      orderId: selectedOrder?.id || '',
      templateId: '',
      reportType: 'STANDARD',
      includeGraphs: false,
      includeReferenceRanges: true,
      includeInterpretation: false,
      additionalComments: '',
      deliveryMethod: 'EMAIL',
    },
  });

  // Pre-select order from URL param (only once)
  // This is a legitimate use case: syncing URL search params with component state
  // The ref ensures this only runs once, preventing infinite loops
  useEffect(() => {
    if (orderIdParam && typedOrders && !processedOrderIdRef.current) {
      const order = typedOrders.find(o => o.id === orderIdParam);
      if (order) {
        // eslint-disable-next-line react-hooks/set-state-in-effect
        setSelectedOrder(order);
        setStep(2);
        processedOrderIdRef.current = true;
      }
    }
  }, [orderIdParam, typedOrders]);

  const handleOrderSelect = (order: Order) => {
    setSelectedOrder(order);
    form.setValue('orderId', order.id);
    setStep(2);
  };

  const handleTemplateSelect = (templateId: string) => {
    setSelectedTemplate(templateId);
    form.setValue('templateId', templateId);
    setStep(3);
  };

  const onSubmit = async (values: ReportFormValues) => {
    try {
      const result = await generateReport({
        variables: {
          orderId: values.orderId,
          input: {
            templateId: values.templateId,
            reportType: values.reportType,
            includeGraphs: values.includeGraphs,
            includeReferenceRanges: values.includeReferenceRanges,
            includeInterpretation: values.includeInterpretation,
            additionalComments: values.additionalComments,
            deliveryMethod: values.deliveryMethod,
          },
        },
      });

      const reportId = (result.data as { generateReport?: { id: string } })?.generateReport?.id;

      addNotification({
        type: 'success',
        title: 'Report Generated',
        message: 'Laboratory report has been generated successfully',
      });

      // Navigate to preview page
      if (reportId) {
        router.push(`/dashboard/reports/${reportId}/preview`);
      }
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to generate report',
      });
    }
  };

  const isOrderReadyForReport = (order: Order) => {
    // Check if all tests have approved results
    return order.tests.every((test) =>
      test.results.some((result) => result.resultStatus === 'APPROVED')
    );
  };

  return (
    <div className="mx-auto max-w-6xl space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Generate Laboratory Report
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Select an order and customize the report template
        </p>
      </div>

      {/* Progress Steps */}
      <div className="flex items-center justify-center gap-4">
        <div className="flex items-center gap-2">
          <div
            className={`flex h-10 w-10 items-center justify-center rounded-full ${
              step >= 1
                ? 'bg-blue-600 text-white'
                : 'bg-gray-200 text-gray-600 dark:bg-gray-700'
            }`}
          >
            {step > 1 ? <CheckCircle2 className="h-5 w-5" /> : '1'}
          </div>
          <span className={step >= 1 ? 'font-medium' : 'text-gray-600'}>
            Select Order
          </span>
        </div>
        <ArrowRight className="h-5 w-5 text-gray-400" />
        <div className="flex items-center gap-2">
          <div
            className={`flex h-10 w-10 items-center justify-center rounded-full ${
              step >= 2
                ? 'bg-blue-600 text-white'
                : 'bg-gray-200 text-gray-600 dark:bg-gray-700'
            }`}
          >
            {step > 2 ? <CheckCircle2 className="h-5 w-5" /> : '2'}
          </div>
          <span className={step >= 2 ? 'font-medium' : 'text-gray-600'}>
            Choose Template
          </span>
        </div>
        <ArrowRight className="h-5 w-5 text-gray-400" />
        <div className="flex items-center gap-2">
          <div
            className={`flex h-10 w-10 items-center justify-center rounded-full ${
              step >= 3
                ? 'bg-blue-600 text-white'
                : 'bg-gray-200 text-gray-600 dark:bg-gray-700'
            }`}
          >
            3
          </div>
          <span className={step >= 3 ? 'font-medium' : 'text-gray-600'}>
            Configure & Generate
          </span>
        </div>
      </div>

      {/* Step 1: Select Order */}
      {step === 1 && (
        <Card>
          <CardHeader>
            <CardTitle>Step 1: Select Order</CardTitle>
            <CardDescription>
              Choose an order with completed and approved results
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <SearchBar
              value={searchQuery}
              onChange={setSearchQuery}
              placeholder="Search by order number, patient name, or MRN..."
            />

            {loadingOrders ? (
              <SkeletonCard />
            ) : (
              <div className="space-y-2">
                {typedOrders.length > 0 ? (
                  typedOrders.map((order: Order) => {
                    const ready = isOrderReadyForReport(order);
                    return (
                      <div
                        key={order.id}
                        className={`cursor-pointer rounded-lg border-2 p-4 transition-colors ${
                          ready
                            ? 'border-gray-200 hover:border-blue-500 dark:border-gray-700'
                            : 'border-gray-100 bg-gray-50 dark:border-gray-800 dark:bg-gray-900'
                        }`}
                        onClick={() => ready && handleOrderSelect(order)}
                      >
                        <div className="flex items-center justify-between">
                          <div className="flex-1">
                            <div className="flex items-center gap-3">
                              <p className="font-medium text-gray-900 dark:text-white">
                                {order.orderNumber}
                              </p>
                              {ready ? (
                                <span className="flex items-center gap-1 text-sm text-green-600">
                                  <CheckCircle2 className="h-4 w-4" />
                                  Ready
                                </span>
                              ) : (
                                <span className="text-sm text-gray-500">
                                  Pending Results
                                </span>
                              )}
                            </div>
                            <p className="text-sm text-gray-600 dark:text-gray-400">
                              {order.patient.firstName} {order.patient.lastName} (
                              {order.patient.mrn})
                            </p>
                            <p className="text-sm text-gray-500">
                              {order.tests.length} test(s) •{' '}
                              {new Date(order.createdAt).toLocaleDateString()}
                            </p>
                          </div>
                          {ready && (
                            <Button size="sm">
                              Select
                              <ArrowRight className="ml-2 h-4 w-4" />
                            </Button>
                          )}
                        </div>
                      </div>
                    );
                  })
                ) : (
                  <div className="py-12 text-center text-gray-500">
                    <Search className="mx-auto mb-2 h-12 w-12 text-gray-400" />
                    <p>No completed orders found</p>
                  </div>
                )}
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Step 2: Choose Template */}
      {step === 2 && selectedOrder && (
        <Card>
          <CardHeader>
            <CardTitle>Step 2: Choose Report Template</CardTitle>
            <CardDescription>
              Selected Order: {selectedOrder.orderNumber} - {selectedOrder.patient.firstName}{' '}
              {selectedOrder.patient.lastName}
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              {reportTemplates.map((template: { id: string; name: string; description: string; icon: typeof FileText; category: string }) => {
                const Icon = template.icon;
                return (
                  <div
                    key={template.id}
                    className={`cursor-pointer rounded-lg border-2 p-4 transition-colors ${
                      selectedTemplate === template.id
                        ? 'border-blue-500 bg-blue-50 dark:border-blue-600 dark:bg-blue-950/50'
                        : 'border-gray-200 hover:border-blue-300 dark:border-gray-700'
                    }`}
                    onClick={() => handleTemplateSelect(template.id)}
                  >
                    <div className="flex items-start gap-3">
                      <Icon className="h-6 w-6 text-blue-600" />
                      <div className="flex-1">
                        <h3 className="font-medium text-gray-900 dark:text-white">
                          {template.name}
                        </h3>
                        <p className="text-sm text-gray-600 dark:text-gray-400">
                          {template.description}
                        </p>
                        <span className="mt-2 inline-block rounded-full bg-gray-100 px-2 py-1 text-xs text-gray-700 dark:bg-gray-800 dark:text-gray-300">
                          {template.category}
                        </span>
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>

            <div className="flex gap-3">
              <Button variant="outline" onClick={() => setStep(1)}>
                Back
              </Button>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Step 3: Configure & Generate */}
      {step === 3 && selectedOrder && selectedTemplate && (
        <Card>
          <CardHeader>
            <CardTitle>Step 3: Configure Report Options</CardTitle>
            <CardDescription>
              Customize the report content and delivery method
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Form {...form}>
              <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
                {/* Report Type */}
                <FormField
                  control={form.control}
                  name="reportType"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Report Type *</FormLabel>
                      <FormControl>
                        <select
                          {...field}
                          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        >
                          <option value="STANDARD">Standard Report</option>
                          <option value="DETAILED">Detailed Report</option>
                          <option value="SUMMARY">Summary Report</option>
                        </select>
                      </FormControl>
                      <FormDescription>
                        Choose the level of detail for the report
                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {/* Options */}
                <div className="space-y-4">
                  <FormField
                    control={form.control}
                    name="includeGraphs"
                    render={({ field }) => (
                      <FormItem className="flex items-center gap-2">
                        <FormControl>
                          <input
                            type="checkbox"
                            checked={field.value}
                            onChange={field.onChange}
                            className="h-4 w-4 rounded border-gray-300"
                          />
                        </FormControl>
                        <FormLabel className="!mt-0">Include Graphs & Charts</FormLabel>
                      </FormItem>
                    )}
                  />

                  <FormField
                    control={form.control}
                    name="includeReferenceRanges"
                    render={({ field }) => (
                      <FormItem className="flex items-center gap-2">
                        <FormControl>
                          <input
                            type="checkbox"
                            checked={field.value}
                            onChange={field.onChange}
                            className="h-4 w-4 rounded border-gray-300"
                          />
                        </FormControl>
                        <FormLabel className="!mt-0">
                          Include Reference Ranges
                        </FormLabel>
                      </FormItem>
                    )}
                  />

                  <FormField
                    control={form.control}
                    name="includeInterpretation"
                    render={({ field }) => (
                      <FormItem className="flex items-center gap-2">
                        <FormControl>
                          <input
                            type="checkbox"
                            checked={field.value}
                            onChange={field.onChange}
                            className="h-4 w-4 rounded border-gray-300"
                          />
                        </FormControl>
                        <FormLabel className="!mt-0">
                          Include Clinical Interpretation
                        </FormLabel>
                      </FormItem>
                    )}
                  />
                </div>

                {/* Additional Comments */}
                <FormField
                  control={form.control}
                  name="additionalComments"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Additional Comments</FormLabel>
                      <FormControl>
                        <textarea
                          {...field}
                          rows={3}
                          placeholder="Add any additional comments or notes to include in the report..."
                          className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        />
                      </FormControl>
                      <FormDescription>
                        Optional comments from the medical director or pathologist
                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {/* Delivery Method */}
                <FormField
                  control={form.control}
                  name="deliveryMethod"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Delivery Method *</FormLabel>
                      <FormControl>
                        <select
                          {...field}
                          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        >
                          <option value="EMAIL">Send via Email</option>
                          <option value="PRINT">Print Report</option>
                          <option value="PORTAL">Patient Portal</option>
                        </select>
                      </FormControl>
                      <FormDescription>
                        Choose how the report will be delivered
                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {/* Actions */}
                <div className="flex gap-3">
                  <Button type="button" variant="outline" onClick={() => setStep(2)}>
                    Back
                  </Button>
                  <Button type="submit" disabled={generating}>
                    {generating ? (
                      <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                        Generating...
                      </>
                    ) : (
                      <>
                        <FileText className="mr-2 h-4 w-4" />
                        Generate Report
                      </>
                    )}
                  </Button>
                </div>
              </form>
            </Form>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

export default function ReportGenerationPage() {
  return (
    <Suspense fallback={<SkeletonCard />}>
      <ReportGenerationPageContent />
    </Suspense>
  );
}
