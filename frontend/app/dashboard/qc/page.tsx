'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { DataTable } from '@/components/ui/data-table';
import { FormModal } from '@/components/ui/form-modal';
import { useConfirmDialog } from '@/components/ui/alert-dialog';
import { useNotificationStore } from '@/lib/store';
import { useQCRuns, useCreateQCRun, useDeleteQCRun } from '@/lib/hooks';
import { SkeletonTable } from '@/components/ui/skeleton';
import {
  Plus,
  AlertTriangle,
  CheckCircle2,
  TrendingUp,
  Activity
} from 'lucide-react';
import { ColumnDef } from '@tanstack/react-table';
import { DataTableColumnHeader, DataTableRowActions } from '@/components/ui/data-table';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';

export const dynamic = 'force-dynamic';

interface QCRun {
  id: string;
  testId: string;
  test: {
    name: string;
    code: string;
  };
  lotNumber: string;
  level: string;
  meanValue: number;
  standardDeviation: number;
  measuredValue: number;
  zScore: number;
  qcStatus: 'PASSED' | 'WARNING' | 'FAILED';
  performedAt: string;
  performedBy: {
    firstName: string;
    lastName: string;
  };
}

const qcRunSchema = z.object({
  testId: z.string().min(1, 'Test is required'),
  lotNumber: z.string().min(1, 'Lot number is required'),
  level: z.enum(['LEVEL_1', 'LEVEL_2', 'LEVEL_3']),
  measuredValue: z.string().min(1, 'Measured value is required'),
  meanValue: z.string().min(1, 'Mean value is required'),
  standardDeviation: z.string().min(1, 'Standard deviation is required'),
});

type QCRunFormValues = z.infer<typeof qcRunSchema>;

export default function QCPage() {
  const router = useRouter();
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [selectedTest] = useState<string | null>(null);
  const addNotification = useNotificationStore((state) => state.addNotification);

  // GraphQL hooks
  const { qcRuns, loading, error, refetch } = useQCRuns({
    testId: selectedTest || undefined,
  });
  const { createQCRun, loading: creating } = useCreateQCRun();
  const { deleteQCRun } = useDeleteQCRun();
  const { confirm } = useConfirmDialog();

  // Form hooks - must be called before any early returns
  const form = useForm<QCRunFormValues>({
    resolver: zodResolver(qcRunSchema),
    defaultValues: {
      testId: '',
      lotNumber: '',
      level: 'LEVEL_1',
      measuredValue: '',
      meanValue: '',
      standardDeviation: '',
    },
  });

  const handleCreateQCRun = async (values: QCRunFormValues) => {
    try {
      await createQCRun({
        variables: {
          input: {
            testId: values.testId,
            lotNumber: values.lotNumber,
            level: values.level,
            measuredValue: parseFloat(values.measuredValue),
            meanValue: parseFloat(values.meanValue),
            standardDeviation: parseFloat(values.standardDeviation),
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'QC Run Created',
        message: 'Quality control run has been recorded successfully',
      });

      setShowCreateModal(false);
      form.reset();
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to create QC run',
      });
    }
  };

  const handleDelete = async (id: string) => {
    confirm({
      title: 'Delete QC Run',
      description: 'Are you sure you want to delete this QC run? This action cannot be undone.',
      variant: 'danger',
      onConfirm: async () => {
        try {
          await deleteQCRun({ variables: { id } });
          addNotification({
            type: 'success',
            title: 'QC Run Deleted',
            message: 'Quality control run has been deleted',
          });
          refetch();
        } catch (error) {
          addNotification({
            type: 'error',
            title: 'Error',
            message: error instanceof Error ? error.message : 'Failed to delete QC run',
          });
        }
      },
    });
  };

  const getQCStatusBadge = (status: string, zScore: number) => {
    if (status === 'PASSED') {
      return (
        <div className="flex items-center gap-1 text-green-600">
          <CheckCircle2 className="h-4 w-4" />
          <span className="text-sm font-medium">Passed</span>
        </div>
      );
    } else if (status === 'WARNING') {
      return (
        <div className="flex items-center gap-1 text-orange-600">
          <AlertTriangle className="h-4 w-4" />
          <span className="text-sm font-medium">Warning ({zScore.toFixed(2)} SD)</span>
        </div>
      );
    } else {
      return (
        <div className="flex items-center gap-1 text-red-600">
          <AlertTriangle className="h-4 w-4" />
          <span className="text-sm font-medium">Failed ({zScore.toFixed(2)} SD)</span>
        </div>
      );
    }
  };

  // Type the qcRuns array
  const typedQCRuns = (qcRuns as QCRun[]) || [];

  const columns: ColumnDef<QCRun>[] = [
    {
      accessorKey: 'performedAt',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Date/Time" />,
      cell: ({ row }) => (
        <div className="text-sm">
          {new Date(row.original.performedAt).toLocaleString()}
        </div>
      ),
    },
    {
      accessorKey: 'test.name',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Test" />,
      cell: ({ row }) => (
        <div>
          <p className="font-medium">{row.original.test.name}</p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {row.original.test.code}
          </p>
        </div>
      ),
    },
    {
      accessorKey: 'lotNumber',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Lot #" />,
      cell: ({ row }) => (
        <span className="font-mono text-sm">{row.original.lotNumber}</span>
      ),
    },
    {
      accessorKey: 'level',
      header: 'Level',
      cell: ({ row }) => (
        <span className="text-sm">
          {row.original.level.replace('LEVEL_', 'L')}
        </span>
      ),
    },
    {
      accessorKey: 'measuredValue',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Measured" />,
      cell: ({ row }) => (
        <span className="font-medium">{row.original.measuredValue.toFixed(2)}</span>
      ),
    },
    {
      accessorKey: 'meanValue',
      header: 'Mean',
      cell: ({ row }) => (
        <span className="text-sm">{row.original.meanValue.toFixed(2)}</span>
      ),
    },
    {
      accessorKey: 'zScore',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Z-Score" />,
      cell: ({ row }) => {
        const zScore = row.original.zScore;
        const color =
          Math.abs(zScore) <= 2
            ? 'text-green-600'
            : Math.abs(zScore) <= 3
            ? 'text-orange-600'
            : 'text-red-600';
        return <span className={`font-medium ${color}`}>{zScore.toFixed(2)}</span>;
      },
    },
    {
      accessorKey: 'qcStatus',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Status" />,
      cell: ({ row }) => getQCStatusBadge(row.original.qcStatus, row.original.zScore),
    },
    {
      accessorKey: 'performedBy',
      header: 'Performed By',
      cell: ({ row }) => (
        <span className="text-sm">
          {row.original.performedBy.firstName} {row.original.performedBy.lastName}
        </span>
      ),
    },
    {
      id: 'actions',
      cell: ({ row }) => (
        <DataTableRowActions
          row={row}
          actions={[
            {
              label: 'View Chart',
              onClick: () => router.push(`/dashboard/qc/${row.original.testId}/chart`),
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
  const totalRuns = typedQCRuns.length;
  const passedRuns = typedQCRuns.filter(r => r.qcStatus === 'PASSED').length;
  const failedRuns = typedQCRuns.filter(r => r.qcStatus === 'FAILED').length;
  const warningRuns = typedQCRuns.filter(r => r.qcStatus === 'WARNING').length;
  const passRate = totalRuns > 0 ? ((passedRuns / totalRuns) * 100).toFixed(1) : '0';

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Quality Control
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            Monitor and manage laboratory quality control runs
          </p>
        </div>
        <Button onClick={() => setShowCreateModal(true)}>
          <Plus className="mr-2 h-4 w-4" />
          Record QC Run
        </Button>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-6 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Runs</CardTitle>
            <Activity className="h-4 w-4 text-gray-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalRuns}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Last 30 days</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pass Rate</CardTitle>
            <TrendingUp className="h-4 w-4 text-green-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600">{passRate}%</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              {passedRuns} of {totalRuns} runs
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Warnings</CardTitle>
            <AlertTriangle className="h-4 w-4 text-orange-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-orange-600">{warningRuns}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">2-3 SD from mean</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Failures</CardTitle>
            <AlertTriangle className="h-4 w-4 text-red-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">{failedRuns}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">&gt;3 SD from mean</p>
          </CardContent>
        </Card>
      </div>

      {/* QC Runs Table */}
      <Card>
        <CardHeader>
          <CardTitle>QC Run History</CardTitle>
          <CardDescription>
            Recent quality control runs with statistical analysis
          </CardDescription>
        </CardHeader>
        <CardContent>
          {loading ? (
            <SkeletonTable />
          ) : error ? (
            <div className="p-4 text-center text-red-600">
              Error loading QC runs: {error.message}
            </div>
          ) : (
            <DataTable
              columns={columns}
              data={typedQCRuns}
              searchKey="test.name"
              searchPlaceholder="Search by test name..."
            />
          )}
        </CardContent>
      </Card>

      {/* Create QC Run Modal */}
      <FormModal
        open={showCreateModal}
        onClose={() => {
          setShowCreateModal(false);
          form.reset();
        }}
        title="Record QC Run"
        description="Enter the quality control measurement details"
        onSubmit={form.handleSubmit(handleCreateQCRun)}
        submitLabel="Record QC Run"
        isLoading={creating}
      >
        <Form {...form}>
          <div className="space-y-4">
            <FormField
              control={form.control}
              name="testId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Test *</FormLabel>
                  <FormControl>
                    <select
                      {...field}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    >
                      <option value="">Select test...</option>
                      <option value="test-001">Hemoglobin (HGB)</option>
                      <option value="test-002">White Blood Cell Count (WBC)</option>
                      <option value="test-003">Glucose (GLU)</option>
                      <option value="test-004">Creatinine (CREAT)</option>
                      <option value="test-005">Sodium (Na)</option>
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="lotNumber"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Lot Number *</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., LOT2024-001" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="level"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>QC Level *</FormLabel>
                  <FormControl>
                    <select
                      {...field}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    >
                      <option value="LEVEL_1">Level 1 (Low)</option>
                      <option value="LEVEL_2">Level 2 (Normal)</option>
                      <option value="LEVEL_3">Level 3 (High)</option>
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="grid gap-4 md:grid-cols-3">
              <FormField
                control={form.control}
                name="measuredValue"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Measured Value *</FormLabel>
                    <FormControl>
                      <Input type="number" step="0.01" placeholder="0.00" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="meanValue"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Expected Mean *</FormLabel>
                    <FormControl>
                      <Input type="number" step="0.01" placeholder="0.00" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="standardDeviation"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Std Deviation *</FormLabel>
                    <FormControl>
                      <Input type="number" step="0.01" placeholder="0.00" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
          </div>
        </Form>
      </FormModal>
    </div>
  );
}
