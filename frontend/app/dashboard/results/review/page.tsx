'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { DataTable } from '@/components/ui/data-table';
import { ResultStatusBadge } from '@/components/ui/status-badge';
import { Modal, ModalHeader, ModalTitle, ModalBody, ModalFooter } from '@/components/ui/modal';
import { useConfirmDialog } from '@/components/ui/alert-dialog';
import { useNotificationStore } from '@/lib/store';
import { useResults, useVerifyResult, useApproveResult, useRejectResult } from '@/lib/hooks';
import { SkeletonTable } from '@/components/ui/skeleton';
import {
  CheckCircle2,
  XCircle,
  Eye,
  AlertTriangle,
  Clock,
  User,

  TrendingUp,
  TrendingDown
} from 'lucide-react';
import { ColumnDef } from '@tanstack/react-table';
import { DataTableColumnHeader } from '@/components/ui/data-table';
import Link from 'next/link';

export const dynamic = 'force-dynamic';

interface Result {
  id: string;
  sample: {
    sampleNumber: string;
    order: {
      patient: {
        firstName: string;
        lastName: string;
        mrn: string;
        age: number;
        gender: string;
      };
    };
  };
  test: {
    name: string;
    code: string;
    defaultUnit: string;
    referenceRangeMin?: number;
    referenceRangeMax?: number;
    criticalLow?: number;
    criticalHigh?: number;
  };
  resultValue: string;
  resultUnit: string;
  resultStatus: string;
  enteredAt?: string;
  enteredBy?: {
    firstName: string;
    lastName: string;
  };
  verifiedAt?: string;
  verifiedBy?: {
    firstName: string;
    lastName: string;
  };
  remarks?: string;
  criticalityLevel?: string;
}

export default function ResultReviewPage() {
  const [selectedTab, setSelectedTab] = useState<'ENTERED' | 'VERIFIED' | 'CRITICAL'>('ENTERED');
  const [selectedResult, setSelectedResult] = useState<Result | null>(null);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [rejectReason, setRejectReason] = useState('');
  const addNotification = useNotificationStore((state) => state.addNotification);

  // GraphQL hooks
  const { results, loading, error, refetch } = useResults({
    filters: {
      status: selectedTab === 'CRITICAL' ? 'ENTERED' : selectedTab,
      critical: selectedTab === 'CRITICAL' ? true : undefined
    },
  });
  const { verifyResult, loading: verifying } = useVerifyResult();
  const { approveResult, loading: approving } = useApproveResult();
  const { rejectResult, loading: rejecting} = useRejectResult();
  const { confirm } = useConfirmDialog();

  // Type the results array
  const typedResults = (results as Result[]) || [];

  const handleViewDetails = (result: Result) => {
    setSelectedResult(result);
    setShowDetailModal(true);
  };

  const handleVerify = async (resultId: string) => {
    confirm({
      title: 'Verify Result',
      description: 'Are you sure you want to verify this test result? This action confirms that the result has been technically validated.',
      variant: 'info',
      onConfirm: async () => {
        try {
          await verifyResult({ variables: { resultId } });
          addNotification({
            type: 'success',
            title: 'Result Verified',
            message: 'Test result has been successfully verified',
          });
          refetch();
          setShowDetailModal(false);
        } catch (error) {
          addNotification({
            type: 'error',
            title: 'Error',
            message: error instanceof Error ? error.message : 'Failed to verify result',
          });
        }
      },
    });
  };

  const handleApprove = async (resultId: string) => {
    confirm({
      title: 'Approve Result',
      description: 'Are you sure you want to approve this test result? This action confirms that the result is medically validated and ready for reporting.',
      variant: 'info',
      onConfirm: async () => {
        try {
          await approveResult({ variables: { resultId } });
          addNotification({
            type: 'success',
            title: 'Result Approved',
            message: 'Test result has been successfully approved',
          });
          refetch();
          setShowDetailModal(false);
        } catch (error) {
          addNotification({
            type: 'error',
            title: 'Error',
            message: error instanceof Error ? error.message : 'Failed to approve result',
          });
        }
      },
    });
  };

  const handleReject = async (resultId: string) => {
    if (!rejectReason) {
      addNotification({
        type: 'warning',
        title: 'Rejection Reason Required',
        message: 'Please provide a reason for rejection',
      });
      return;
    }

    confirm({
      title: 'Reject Result',
      description: 'Are you sure you want to reject this test result? The result will be sent back for re-entry.',
      variant: 'danger',
      onConfirm: async () => {
        try {
          await rejectResult({ variables: { resultId, reason: rejectReason } });
          addNotification({
            type: 'success',
            title: 'Result Rejected',
            message: 'Test result has been rejected',
          });
          refetch();
          setShowDetailModal(false);
          setRejectReason('');
        } catch (error) {
          addNotification({
            type: 'error',
            title: 'Error',
            message: error instanceof Error ? error.message : 'Failed to reject result',
          });
        }
      },
    });
  };

  const getResultIndicator = (result: Result) => {
    if (!result.resultValue || !result.test.referenceRangeMin || !result.test.referenceRangeMax) {
      return null;
    }

    const value = parseFloat(result.resultValue);
    const min = result.test.referenceRangeMin;
    const max = result.test.referenceRangeMax;

    if (value < min) {
      return <TrendingDown className="h-4 w-4 text-blue-600" />;
    } else if (value > max) {
      return <TrendingUp className="h-4 w-4 text-red-600" />;
    }
    return <CheckCircle2 className="h-4 w-4 text-green-600" />;
  };

  const columns: ColumnDef<Result>[] = [
    {
      accessorKey: 'sample.sampleNumber',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Sample #" />,
      cell: ({ row }) => (
        <Link
          href={`/dashboard/samples/${row.original.sample.sampleNumber}`}
          className="font-medium text-blue-600 hover:underline"
        >
          {row.original.sample.sampleNumber}
        </Link>
      ),
    },
    {
      accessorKey: 'patient',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Patient" />,
      cell: ({ row }) => {
        const patient = row.original.sample.order.patient;
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
      accessorKey: 'resultValue',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Result" />,
      cell: ({ row }) => (
        <div className="flex items-center gap-2">
          {getResultIndicator(row.original)}
          <span className="font-medium">
            {row.original.resultValue} {row.original.resultUnit}
          </span>
        </div>
      ),
    },
    {
      accessorKey: 'referenceRange',
      header: 'Reference Range',
      cell: ({ row }) => {
        const test = row.original.test;
        if (!test.referenceRangeMin || !test.referenceRangeMax) {
          return <span className="text-gray-400">N/A</span>;
        }
        return (
          <span className="text-sm text-gray-600 dark:text-gray-400">
            {test.referenceRangeMin} - {test.referenceRangeMax} {test.defaultUnit}
          </span>
        );
      },
    },
    {
      accessorKey: 'resultStatus',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Status" />,
      cell: ({ row }) => <ResultStatusBadge status={row.original.resultStatus} />,
    },
    {
      accessorKey: 'criticalityLevel',
      header: 'Priority',
      cell: ({ row }) => {
        const level = row.original.criticalityLevel;
        if (level === 'PANIC' || level === 'CRITICAL') {
          return (
            <div className="flex items-center gap-1 text-red-600">
              <AlertTriangle className="h-4 w-4" />
              <span className="text-sm font-medium">{level}</span>
            </div>
          );
        }
        return <span className="text-sm text-gray-400">Normal</span>;
      },
    },
    {
      accessorKey: 'enteredAt',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Entered At" />,
      cell: ({ row }) => (
        <div className="text-sm">
          {row.original.enteredAt
            ? new Date(row.original.enteredAt).toLocaleString()
            : 'N/A'}
        </div>
      ),
    },
    {
      id: 'actions',
      cell: ({ row }) => (
        <Button
          variant="outline"
          size="sm"
          onClick={() => handleViewDetails(row.original)}
        >
          <Eye className="mr-2 h-4 w-4" />
          Review
        </Button>
      ),
    },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Result Review & Approval
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Verify and approve test results for final reporting
        </p>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-6 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pending Verification</CardTitle>
            <Clock className="h-4 w-4 text-gray-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {typedResults.filter((r: Result) => r.resultStatus === 'ENTERED').length || 0}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Awaiting technical review
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pending Approval</CardTitle>
            <User className="h-4 w-4 text-gray-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {typedResults.filter((r: Result) => r.resultStatus === 'VERIFIED').length || 0}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Awaiting medical review
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Critical Values</CardTitle>
            <AlertTriangle className="h-4 w-4 text-red-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">
              {typedResults.filter((r: Result) =>
                r.criticalityLevel === 'PANIC' || r.criticalityLevel === 'CRITICAL'
              ).length || 0}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Requires immediate attention
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Tabs */}
      <div className="flex gap-2 border-b border-gray-200 dark:border-gray-700">
        <button
          onClick={() => setSelectedTab('ENTERED')}
          className={`px-4 py-2 font-medium ${
            selectedTab === 'ENTERED'
              ? 'border-b-2 border-blue-600 text-blue-600'
              : 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white'
          }`}
        >
          Pending Verification
          <span className="ml-2 rounded-full bg-gray-200 px-2 py-0.5 text-xs dark:bg-gray-700">
            {typedResults.filter((r: Result) => r.resultStatus === 'ENTERED').length || 0}
          </span>
        </button>
        <button
          onClick={() => setSelectedTab('VERIFIED')}
          className={`px-4 py-2 font-medium ${
            selectedTab === 'VERIFIED'
              ? 'border-b-2 border-blue-600 text-blue-600'
              : 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white'
          }`}
        >
          Pending Approval
          <span className="ml-2 rounded-full bg-gray-200 px-2 py-0.5 text-xs dark:bg-gray-700">
            {typedResults.filter((r: Result) => r.resultStatus === 'VERIFIED').length || 0}
          </span>
        </button>
        <button
          onClick={() => setSelectedTab('CRITICAL')}
          className={`px-4 py-2 font-medium ${
            selectedTab === 'CRITICAL'
              ? 'border-b-2 border-red-600 text-red-600'
              : 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white'
          }`}
        >
          Critical Values
          <span className="ml-2 rounded-full bg-red-100 px-2 py-0.5 text-xs text-red-600 dark:bg-red-950">
            {typedResults.filter((r: Result) =>
              r.criticalityLevel === 'PANIC' || r.criticalityLevel === 'CRITICAL'
            ).length || 0}
          </span>
        </button>
      </div>

      {/* Results Table */}
      <Card>
        <CardHeader>
          <CardTitle>
            {selectedTab === 'ENTERED' && 'Results Pending Verification'}
            {selectedTab === 'VERIFIED' && 'Results Pending Approval'}
            {selectedTab === 'CRITICAL' && 'Critical Value Alerts'}
          </CardTitle>
          <CardDescription>
            {selectedTab === 'ENTERED' && 'Technical review and validation of test results'}
            {selectedTab === 'VERIFIED' && 'Medical review and approval for final reporting'}
            {selectedTab === 'CRITICAL' && 'High-priority results requiring immediate attention'}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {loading ? (
            <SkeletonTable />
          ) : error ? (
            <div className="p-4 text-center text-red-600">
              Error loading results: {error.message}
            </div>
          ) : (
            <DataTable
              columns={columns}
              data={typedResults}
              searchKey="sample.sampleNumber"
              searchPlaceholder="Search by sample number..."
            />
          )}
        </CardContent>
      </Card>

      {/* Detail Modal */}
      <Modal open={showDetailModal} onClose={() => setShowDetailModal(false)}>
        <ModalHeader>
          <ModalTitle>Result Review Details</ModalTitle>
        </ModalHeader>
        <ModalBody>
          {selectedResult && (
            <div className="space-y-6">
              {/* Critical Alert */}
              {(selectedResult.criticalityLevel === 'PANIC' ||
                selectedResult.criticalityLevel === 'CRITICAL') && (
                <div className="flex items-start gap-3 rounded-lg border-2 border-red-200 bg-red-50 p-4 dark:border-red-900 dark:bg-red-950/50">
                  <AlertTriangle className="h-5 w-5 text-red-600 dark:text-red-400" />
                  <div>
                    <p className="font-medium text-red-900 dark:text-red-300">
                      {selectedResult.criticalityLevel} VALUE ALERT
                    </p>
                    <p className="text-sm text-red-700 dark:text-red-400">
                      This result requires immediate physician notification and documentation
                    </p>
                  </div>
                </div>
              )}

              {/* Patient Information */}
              <div>
                <h3 className="mb-3 font-semibold">Patient Information</h3>
                <div className="grid gap-3 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Name:</span>
                    <span className="font-medium">
                      {selectedResult.sample.order.patient.firstName}{' '}
                      {selectedResult.sample.order.patient.lastName}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">MRN:</span>
                    <span className="font-medium">
                      {selectedResult.sample.order.patient.mrn}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Age/Gender:</span>
                    <span className="font-medium">
                      {selectedResult.sample.order.patient.age} years /{' '}
                      {selectedResult.sample.order.patient.gender}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Sample #:</span>
                    <span className="font-medium">
                      {selectedResult.sample.sampleNumber}
                    </span>
                  </div>
                </div>
              </div>

              {/* Test & Result Information */}
              <div>
                <h3 className="mb-3 font-semibold">Test & Result</h3>
                <div className="space-y-3 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Test Name:</span>
                    <span className="font-medium">{selectedResult.test.name}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Test Code:</span>
                    <span className="font-medium">{selectedResult.test.code}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Result:</span>
                    <div className="flex items-center gap-2">
                      {getResultIndicator(selectedResult)}
                      <span className="text-lg font-bold">
                        {selectedResult.resultValue} {selectedResult.resultUnit}
                      </span>
                    </div>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Reference Range:</span>
                    <span className="font-medium">
                      {selectedResult.test.referenceRangeMin} -{' '}
                      {selectedResult.test.referenceRangeMax} {selectedResult.test.defaultUnit}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Status:</span>
                    <ResultStatusBadge status={selectedResult.resultStatus} />
                  </div>
                </div>
              </div>

              {/* Entry Information */}
              {selectedResult.enteredBy && (
                <div>
                  <h3 className="mb-3 font-semibold">Entry Information</h3>
                  <div className="space-y-3 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Entered By:</span>
                      <span className="font-medium">
                        {selectedResult.enteredBy.firstName} {selectedResult.enteredBy.lastName}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Entered At:</span>
                      <span className="font-medium">
                        {selectedResult.enteredAt
                          ? new Date(selectedResult.enteredAt).toLocaleString()
                          : 'N/A'}
                      </span>
                    </div>
                  </div>
                </div>
              )}

              {/* Verification Information */}
              {selectedResult.verifiedBy && (
                <div>
                  <h3 className="mb-3 font-semibold">Verification Information</h3>
                  <div className="space-y-3 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Verified By:</span>
                      <span className="font-medium">
                        {selectedResult.verifiedBy.firstName} {selectedResult.verifiedBy.lastName}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Verified At:</span>
                      <span className="font-medium">
                        {selectedResult.verifiedAt
                          ? new Date(selectedResult.verifiedAt).toLocaleString()
                          : 'N/A'}
                      </span>
                    </div>
                  </div>
                </div>
              )}

              {/* Remarks */}
              {selectedResult.remarks && (
                <div>
                  <h3 className="mb-3 font-semibold">Remarks</h3>
                  <p className="rounded-lg bg-gray-50 p-4 text-sm dark:bg-gray-900">
                    {selectedResult.remarks}
                  </p>
                </div>
              )}

              {/* Reject Reason (if rejecting) */}
              {selectedResult.resultStatus === 'ENTERED' && (
                <div>
                  <label className="mb-2 block text-sm font-medium">
                    Rejection Reason (Required if rejecting)
                  </label>
                  <textarea
                    value={rejectReason}
                    onChange={(e) => setRejectReason(e.target.value)}
                    rows={3}
                    placeholder="Provide a reason for rejection..."
                    className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                  />
                </div>
              )}
            </div>
          )}
        </ModalBody>
        <ModalFooter>
          <Button
            variant="outline"
            onClick={() => {
              setShowDetailModal(false);
              setRejectReason('');
            }}
          >
            Close
          </Button>
          {selectedResult?.resultStatus === 'ENTERED' && (
            <>
              <Button
                variant="destructive"
                onClick={() => handleReject(selectedResult.id)}
                disabled={rejecting || !rejectReason}
              >
                <XCircle className="mr-2 h-4 w-4" />
                Reject
              </Button>
              <Button
                onClick={() => handleVerify(selectedResult.id)}
                disabled={verifying}
              >
                <CheckCircle2 className="mr-2 h-4 w-4" />
                Verify Result
              </Button>
            </>
          )}
          {selectedResult?.resultStatus === 'VERIFIED' && (
            <>
              <Button
                variant="destructive"
                onClick={() => handleReject(selectedResult.id)}
                disabled={rejecting || !rejectReason}
              >
                <XCircle className="mr-2 h-4 w-4" />
                Reject
              </Button>
              <Button
                onClick={() => handleApprove(selectedResult.id)}
                disabled={approving}
              >
                <CheckCircle2 className="mr-2 h-4 w-4" />
                Approve Result
              </Button>
            </>
          )}
        </ModalFooter>
      </Modal>
    </div>
  );
}
