'use client';

import { useState, useEffect } from 'react';
import { useRouter, useParams } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { useNotificationStore } from '@/lib/store';
import { useResult, useEnterResult } from '@/lib/hooks';
import { SkeletonCard } from '@/components/ui/skeleton';
import { ArrowLeft, Save, AlertTriangle, CheckCircle2, Info, Loader2 } from 'lucide-react';
import Link from 'next/link';

export const dynamic = 'force-dynamic';

interface Result {
  resultValue?: string;
  resultMethod?: string;
  instrumentId?: string;
  remarks?: string;
  performedBy?: string;
  test?: {
    name: string;
    code: string;
    defaultUnit: string;
    referenceRangeMin?: number;
    referenceRangeMax?: number;
    criticalLow?: number;
    criticalHigh?: number;
    panicLow?: number;
    panicHigh?: number;
  };
  sample?: {
    sampleNumber: string;
    order?: {
      patient?: {
        firstName: string;
        lastName: string;
        mrn: string;
        age: number;
        gender: string;
      };
    };
  };
}

const resultSchema = z.object({
  resultValue: z.string().min(1, 'Result value is required'),
  resultUnit: z.string().min(1, 'Unit is required'),
  resultMethod: z.string().optional(),
  instrumentId: z.string().optional(),
  remarks: z.string().optional(),
  performedBy: z.string().optional(),
});

type ResultFormValues = z.infer<typeof resultSchema>;

export default function ResultEntryPage() {
  const router = useRouter();
  const params = useParams();
  const resultId = params.id as string;
  const [isOutOfRange, setIsOutOfRange] = useState(false);
  const [criticalityLevel, setCriticalityLevel] = useState<'NORMAL' | 'WARNING' | 'CRITICAL' | 'PANIC'>('NORMAL');
  const [autoVerifyResult, setAutoVerifyResult] = useState<{ passed: boolean; confidence: number } | null>(null);
  const addNotification = useNotificationStore((state) => state.addNotification);

  // GraphQL hooks
  const { result, loading: loadingResult, error: resultError } = useResult(resultId);
  const { enterResult, loading: entering } = useEnterResult();

  // Type the result for use throughout the component
  const typedResultData = result as Result | null | undefined;

  // Form hook - must be called before any early returns
  const form = useForm<ResultFormValues>({
    resolver: zodResolver(resultSchema),
    defaultValues: {
      resultValue: '',
      resultUnit: '',
      resultMethod: 'Automated',
      instrumentId: '',
      remarks: '',
      performedBy: '',
    },
  });

  // Watch result value for real-time validation
  const resultValue = form.watch('resultValue');

  // Update form defaults when result loads
  useEffect(() => {
    if (typedResultData) {
      form.reset({
        resultValue: typedResultData.resultValue || '',
        resultUnit: typedResultData.test?.defaultUnit || '',
        resultMethod: typedResultData.resultMethod || 'Automated',
        instrumentId: typedResultData.instrumentId || '',
        remarks: typedResultData.remarks || '',
        performedBy: typedResultData.performedBy || '',
      });
    }
  }, [typedResultData, form]);

  // Real-time validation
  useEffect(() => {
    if (resultValue && typedResultData?.test) {
      const numValue = parseFloat(resultValue);
      if (!isNaN(numValue)) {
        const refRangeMin = typedResultData.test.referenceRangeMin || 0;
        const refRangeMax = typedResultData.test.referenceRangeMax || 0;
        const critLow = typedResultData.test.criticalLow || 0;
        const critHigh = typedResultData.test.criticalHigh || 0;
        const panLow = typedResultData.test.panicLow || 0;
        const panHigh = typedResultData.test.panicHigh || 0;

        // Check if out of range
        const outOfRange = numValue < refRangeMin || numValue > refRangeMax;
        setIsOutOfRange(outOfRange);

        // Determine criticality
        if (numValue <= panLow || numValue >= panHigh) {
          setCriticalityLevel('PANIC');
        } else if (numValue <= critLow || numValue >= critHigh) {
          setCriticalityLevel('CRITICAL');
        } else if (outOfRange) {
          setCriticalityLevel('WARNING');
        } else {
          setCriticalityLevel('NORMAL');
        }

        // Simulate auto-verification
        const withinCritical = numValue > critLow && numValue < critHigh;
        const confidence = withinCritical ? 95 : 60;
        setAutoVerifyResult({
          passed: withinCritical && confidence >= 90,
          confidence,
        });
      }
    }
  }, [resultValue, typedResultData]);

  if (loadingResult) {
    return (
      <div className="mx-auto max-w-4xl space-y-6">
        <SkeletonCard />
        <SkeletonCard />
        <SkeletonCard />
      </div>
    );
  }

  if (resultError || !result) {
    return (
      <div className="mx-auto max-w-4xl">
        <Card>
          <CardContent className="p-6">
            <p className="text-red-600">Error loading result: {resultError?.message || 'Result not found'}</p>
          </CardContent>
        </Card>
      </div>
    );
  }

  // Type assertion after null check
  const typedResult = result as Result;

  const onSubmit = async (values: ResultFormValues) => {
    try {
      await enterResult({
        variables: {
          resultId,
          input: {
            resultValue: values.resultValue,
            resultUnit: values.resultUnit,
            resultMethod: values.resultMethod,
            instrumentId: values.instrumentId,
            remarks: values.remarks,
            resultStatus: 'ENTERED',
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Result Saved',
        message: 'Result entered successfully',
      });

      router.push('/dashboard/results');
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to save result',
      });
    }
  };

  const handleSaveAndNext = async () => {
    const isValid = await form.trigger();
    if (isValid) {
      await form.handleSubmit(onSubmit)();
      // In production, navigate to next pending result
    }
  };

  return (
    <div className="mx-auto max-w-4xl space-y-6">
      {/* Header */}
      <div className="flex items-center gap-4">
        <Link href="/dashboard/results">
          <Button variant="outline" size="icon">
            <ArrowLeft className="h-4 w-4" />
          </Button>
        </Link>
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Enter Test Result
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            {typedResult.test?.name} - {typedResult.sample?.sampleNumber}
          </p>
        </div>
      </div>

      {/* Patient Information */}
      <Card>
        <CardHeader>
          <CardTitle>Patient Information</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 md:grid-cols-4">
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Patient Name</p>
              <p className="font-medium text-gray-900 dark:text-white">
                {typedResult.sample?.order?.patient?.firstName} {typedResult.sample?.order?.patient?.lastName}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">MRN</p>
              <p className="font-medium text-gray-900 dark:text-white">
                {typedResult.sample?.order?.patient?.mrn}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Age/Gender</p>
              <p className="font-medium text-gray-900 dark:text-white">
                {typedResult.sample?.order?.patient?.age} years / {typedResult.sample?.order?.patient?.gender}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Sample Number</p>
              <p className="font-medium text-gray-900 dark:text-white">
                {typedResult.sample?.sampleNumber}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Test Information */}
      <Card>
        <CardHeader>
          <CardTitle>Test Information</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 md:grid-cols-3">
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Test Name</p>
              <p className="font-medium text-gray-900 dark:text-white">
                {typedResult.test?.name}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Test Code</p>
              <p className="font-medium text-gray-900 dark:text-white">
                {typedResult.test?.code}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Reference Range</p>
              <p className="font-medium text-gray-900 dark:text-white">
                {typedResult.test?.referenceRangeMin} - {typedResult.test?.referenceRangeMax}{' '}
                {typedResult.test?.defaultUnit}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Result Entry Form */}
      <Card>
        <CardHeader>
          <CardTitle>Enter Result</CardTitle>
          <CardDescription>Enter the test result value and additional information</CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
              {/* Result Value */}
              <div className="grid gap-4 md:grid-cols-2">
                <FormField
                  control={form.control}
                  name="resultValue"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Result Value *</FormLabel>
                      <FormControl>
                        <Input
                          type="number"
                          step="0.01"
                          placeholder="Enter result"
                          {...field}
                          className={
                            isOutOfRange
                              ? 'border-red-500 focus:border-red-500'
                              : ''
                          }
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={form.control}
                  name="resultUnit"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Unit *</FormLabel>
                      <FormControl>
                        <Input {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>

              {/* Real-time Validation Feedback */}
              {resultValue && (
                <div className="space-y-3">
                  {/* Range Check */}
                  {isOutOfRange ? (
                    <div className="flex items-center gap-2 rounded-lg border-2 border-orange-200 bg-orange-50 p-3 dark:border-orange-900 dark:bg-orange-950/50">
                      <AlertTriangle className="h-5 w-5 text-orange-600 dark:text-orange-400" />
                      <div>
                        <p className="font-medium text-orange-900 dark:text-orange-300">
                          Out of Reference Range
                        </p>
                        <p className="text-sm text-orange-700 dark:text-orange-400">
                          Expected: {typedResult.test?.referenceRangeMin} -{' '}
                          {typedResult.test?.referenceRangeMax} {typedResult.test?.defaultUnit}
                        </p>
                      </div>
                    </div>
                  ) : (
                    <div className="flex items-center gap-2 rounded-lg border-2 border-green-200 bg-green-50 p-3 dark:border-green-900 dark:bg-green-950/50">
                      <CheckCircle2 className="h-5 w-5 text-green-600 dark:text-green-400" />
                      <p className="font-medium text-green-900 dark:text-green-300">
                        Within Reference Range
                      </p>
                    </div>
                  )}

                  {/* Criticality Alert */}
                  {(criticalityLevel === 'CRITICAL' || criticalityLevel === 'PANIC') && (
                    <div className="flex items-center gap-2 rounded-lg border-2 border-red-200 bg-red-50 p-3 dark:border-red-900 dark:bg-red-950/50">
                      <AlertTriangle className="h-5 w-5 text-red-600 dark:text-red-400" />
                      <div>
                        <p className="font-medium text-red-900 dark:text-red-300">
                          {criticalityLevel === 'PANIC' ? 'Panic Value' : 'Critical Value'}
                        </p>
                        <p className="text-sm text-red-700 dark:text-red-400">
                          Requires immediate physician notification
                        </p>
                      </div>
                    </div>
                  )}

                  {/* Auto-Verification Status */}
                  {autoVerifyResult && (
                    <div
                      className={`flex items-center gap-2 rounded-lg border-2 p-3 ${
                        autoVerifyResult.passed
                          ? 'border-green-200 bg-green-50 dark:border-green-900 dark:bg-green-950/50'
                          : 'border-yellow-200 bg-yellow-50 dark:border-yellow-900 dark:bg-yellow-950/50'
                      }`}
                    >
                      <Info
                        className={`h-5 w-5 ${
                          autoVerifyResult.passed
                            ? 'text-green-600 dark:text-green-400'
                            : 'text-yellow-600 dark:text-yellow-400'
                        }`}
                      />
                      <div>
                        <p
                          className={`font-medium ${
                            autoVerifyResult.passed
                              ? 'text-green-900 dark:text-green-300'
                              : 'text-yellow-900 dark:text-yellow-300'
                          }`}
                        >
                          Auto-Verification:{' '}
                          {autoVerifyResult.passed ? 'Will Pass' : 'Requires Manual Review'}
                        </p>
                        <p
                          className={`text-sm ${
                            autoVerifyResult.passed
                              ? 'text-green-700 dark:text-green-400'
                              : 'text-yellow-700 dark:text-yellow-400'
                          }`}
                        >
                          Confidence Score: {autoVerifyResult.confidence}%
                        </p>
                      </div>
                    </div>
                  )}
                </div>
              )}

              {/* Additional Information */}
              <div className="grid gap-4 md:grid-cols-2">
                <FormField
                  control={form.control}
                  name="resultMethod"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Method</FormLabel>
                      <FormControl>
                        <select
                          {...field}
                          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        >
                          <option value="Automated">Automated</option>
                          <option value="Manual">Manual</option>
                          <option value="Semi-Automated">Semi-Automated</option>
                        </select>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={form.control}
                  name="instrumentId"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Instrument</FormLabel>
                      <FormControl>
                        <select
                          {...field}
                          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        >
                          <option value="">Select instrument</option>
                          <option value="inst-001">Hematology Analyzer - HA-100</option>
                          <option value="inst-002">Chemistry Analyzer - CA-200</option>
                          <option value="inst-003">Immunoassay Analyzer - IA-300</option>
                        </select>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>

              <FormField
                control={form.control}
                name="remarks"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Remarks / Comments</FormLabel>
                    <FormControl>
                      <textarea
                        {...field}
                        rows={3}
                        placeholder="Add any observations or notes..."
                        className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                      />
                    </FormControl>
                    <FormDescription>
                      Optional comments about the result or testing process
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* Actions */}
              <div className="flex gap-3">
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => router.back()}
                  disabled={entering}
                >
                  Cancel
                </Button>
                <Button type="submit" disabled={entering}>
                  {entering ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      Saving...
                    </>
                  ) : (
                    <>
                      <Save className="mr-2 h-4 w-4" />
                      Save Result
                    </>
                  )}
                </Button>
                <Button
                  type="button"
                  onClick={handleSaveAndNext}
                  disabled={entering}
                  variant="secondary"
                >
                  Save & Next
                </Button>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>

      {/* Quick Reference */}
      <Card>
        <CardHeader>
          <CardTitle>Critical Value Thresholds</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-2 text-sm">
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Panic Low:</span>
              <span className="font-medium text-red-600 dark:text-red-400">
                &lt; {typedResult.test?.panicLow} {typedResult.test?.defaultUnit}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Critical Low:</span>
              <span className="font-medium text-orange-600 dark:text-orange-400">
                &lt; {typedResult.test?.criticalLow} {typedResult.test?.defaultUnit}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Normal Range:</span>
              <span className="font-medium text-green-600 dark:text-green-400">
                {typedResult.test?.referenceRangeMin} - {typedResult.test?.referenceRangeMax}{' '}
                {typedResult.test?.defaultUnit}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Critical High:</span>
              <span className="font-medium text-orange-600 dark:text-orange-400">
                &gt; {typedResult.test?.criticalHigh} {typedResult.test?.defaultUnit}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Panic High:</span>
              <span className="font-medium text-red-600 dark:text-red-400">
                &gt; {typedResult.test?.panicHigh} {typedResult.test?.defaultUnit}
              </span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
