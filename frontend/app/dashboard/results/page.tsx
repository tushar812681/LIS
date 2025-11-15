'use client';

import { useState, useEffect, useMemo } from 'react';
import { useRouter } from 'next/navigation';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Beaker,
  Search,
  AlertTriangle,
  CheckCircle2,
  Clock,
  Eye,
  Edit,
  FileCheck,
  TrendingUp,
  TrendingDown,
} from 'lucide-react';
import { getWebSocketClient } from '@/lib/websocket-client';
import { useNotificationStore } from '@/lib/store';

interface TestResult {
  id: string;
  sampleNumber: string;
  patientMrn: string;
  patientName: string;
  testName: string;
  testParameter: string;
  resultValue: string | null;
  unit: string;
  referenceRange: string;
  resultStatus: 'PENDING' | 'ENTERED' | 'VERIFIED' | 'APPROVED';
  criticalityLevel: 'NORMAL' | 'WARNING' | 'CRITICAL' | 'PANIC';
  isOutOfRange: boolean;
  autoVerificationStatus: 'PASSED' | 'FAILED' | 'NEEDS_REVIEW' | null;
  confidenceScore: number | null;
  enteredBy: string | null;
  verifiedBy: string | null;
  enteredAt: string | null;
  verifiedAt: string | null;
}

const statusConfig = {
  PENDING: {
    label: 'Pending Entry',
    color: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-950 dark:text-yellow-300',
    icon: Clock,
  },
  ENTERED: {
    label: 'Awaiting Verification',
    color: 'bg-blue-100 text-blue-800 dark:bg-blue-950 dark:text-blue-300',
    icon: FileCheck,
  },
  VERIFIED: {
    label: 'Verified',
    color: 'bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300',
    icon: CheckCircle2,
  },
  APPROVED: {
    label: 'Approved',
    color: 'bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300',
    icon: CheckCircle2,
  },
};

const criticalityConfig = {
  NORMAL: {
    label: 'Normal',
    color: 'bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300',
  },
  WARNING: {
    label: 'Warning',
    color: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-950 dark:text-yellow-300',
  },
  CRITICAL: {
    label: 'Critical',
    color: 'bg-orange-100 text-orange-800 dark:bg-orange-950 dark:text-orange-300',
  },
  PANIC: {
    label: 'Panic Value',
    color: 'bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300',
  },
};

function ResultCard({ result }: { result: TestResult }) {
  const router = useRouter();
  const status = statusConfig[result.resultStatus];
  const criticality = criticalityConfig[result.criticalityLevel];
  const StatusIcon = status.icon;

  return (
    <div className="rounded-lg border border-gray-200 p-4 transition-all hover:border-blue-500 hover:shadow-md dark:border-gray-800 dark:hover:border-blue-500">
      {/* Header */}
      <div className="mb-3 flex items-start justify-between">
        <div className="flex items-center gap-3">
          <div className={`flex h-12 w-12 items-center justify-center rounded-full ${
            result.criticalityLevel === 'PANIC' || result.criticalityLevel === 'CRITICAL'
              ? 'bg-red-100 dark:bg-red-950'
              : 'bg-blue-100 dark:bg-blue-950'
          }`}>
            <Beaker className={`h-6 w-6 ${
              result.criticalityLevel === 'PANIC' || result.criticalityLevel === 'CRITICAL'
                ? 'text-red-600 dark:text-red-400'
                : 'text-blue-600 dark:text-blue-400'
            }`} />
          </div>
          <div>
            <p className="font-medium text-gray-900 dark:text-white">{result.sampleNumber}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {result.patientName} ({result.patientMrn})
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <span className={`flex items-center gap-1 rounded-full px-2 py-1 text-xs font-medium ${status.color}`}>
            <StatusIcon className="h-3 w-3" />
            {status.label}
          </span>
        </div>
      </div>

      {/* Test Information */}
      <div className="mb-3 space-y-2 rounded-lg bg-gray-50 p-3 dark:bg-gray-800">
        <div className="flex justify-between">
          <span className="text-sm font-medium text-gray-900 dark:text-white">{result.testName}</span>
          <span className="text-sm text-gray-600 dark:text-gray-400">{result.testParameter}</span>
        </div>
        {result.resultValue && (
          <div className="flex items-center justify-between">
            <span className="text-sm text-gray-600 dark:text-gray-400">Result:</span>
            <div className="flex items-center gap-2">
              <span className={`text-lg font-bold ${
                result.isOutOfRange
                  ? 'text-red-600 dark:text-red-400'
                  : 'text-gray-900 dark:text-white'
              }`}>
                {result.resultValue} {result.unit}
              </span>
              {result.isOutOfRange && (
                result.resultValue > result.referenceRange.split('-')[1]
                  ? <TrendingUp className="h-4 w-4 text-red-600 dark:text-red-400" />
                  : <TrendingDown className="h-4 w-4 text-red-600 dark:text-red-400" />
              )}
            </div>
          </div>
        )}
        <div className="flex justify-between">
          <span className="text-sm text-gray-600 dark:text-gray-400">Reference Range:</span>
          <span className="text-sm font-medium text-gray-900 dark:text-white">
            {result.referenceRange} {result.unit}
          </span>
        </div>
      </div>

      {/* Criticality & Auto-Verification */}
      <div className="mb-3 flex gap-2">
        <span className={`rounded-full px-2 py-1 text-xs font-medium ${criticality.color}`}>
          {criticality.label}
        </span>
        {result.autoVerificationStatus && (
          <span className={`rounded-full px-2 py-1 text-xs font-medium ${
            result.autoVerificationStatus === 'PASSED'
              ? 'bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300'
              : result.autoVerificationStatus === 'FAILED'
              ? 'bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300'
              : 'bg-yellow-100 text-yellow-800 dark:bg-yellow-950 dark:text-yellow-300'
          }`}>
            Auto-Verify: {result.autoVerificationStatus}
            {result.confidenceScore && ` (${result.confidenceScore}%)`}
          </span>
        )}
      </div>

      {/* Critical Value Alert */}
      {(result.criticalityLevel === 'CRITICAL' || result.criticalityLevel === 'PANIC') && (
        <div className="mb-3 flex items-center gap-2 rounded-lg border-2 border-red-200 bg-red-50 p-2 dark:border-red-900 dark:bg-red-950/50">
          <AlertTriangle className="h-5 w-5 text-red-600 dark:text-red-400" />
          <div>
            <p className="text-sm font-medium text-red-900 dark:text-red-300">
              Critical Value Alert
            </p>
            <p className="text-xs text-red-700 dark:text-red-400">
              Requires immediate physician notification
            </p>
          </div>
        </div>
      )}

      {/* Metadata */}
      {result.enteredBy && (
        <div className="mb-3 text-xs text-gray-600 dark:text-gray-400">
          <p>Entered by: {result.enteredBy} at {result.enteredAt}</p>
          {result.verifiedBy && (
            <p>Verified by: {result.verifiedBy} at {result.verifiedAt}</p>
          )}
        </div>
      )}

      {/* Actions */}
      <div className="flex gap-2">
        {result.resultStatus === 'PENDING' && (
          <Button
            size="sm"
            className="flex-1"
            onClick={() => router.push(`/dashboard/results/${result.id}/enter`)}
          >
            <Edit className="mr-2 h-4 w-4" />
            Enter Result
          </Button>
        )}
        {result.resultStatus === 'ENTERED' && (
          <>
            <Button
              size="sm"
              variant="outline"
              className="flex-1"
              onClick={() => router.push(`/dashboard/results/${result.id}/enter`)}
            >
              <Edit className="mr-2 h-4 w-4" />
              Edit
            </Button>
            <Button
              size="sm"
              className="flex-1"
              onClick={() => router.push(`/dashboard/results/${result.id}/verify`)}
            >
              <CheckCircle2 className="mr-2 h-4 w-4" />
              Verify
            </Button>
          </>
        )}
        {(result.resultStatus === 'VERIFIED' || result.resultStatus === 'APPROVED') && (
          <Button
            size="sm"
            variant="outline"
            className="flex-1"
            onClick={() => router.push(`/dashboard/results/${result.id}`)}
          >
            <Eye className="mr-2 h-4 w-4" />
            View Details
          </Button>
        )}
      </div>
    </div>
  );
}

const mockResults: TestResult[] = [
  {
    id: '1',
    sampleNumber: 'SMP-2024-001',
    patientMrn: 'PAT-10001',
    patientName: 'John Doe',
    testName: 'Complete Blood Count',
    testParameter: 'Hemoglobin',
    resultValue: '6.5',
    unit: 'g/dL',
    referenceRange: '12.0-16.0',
    resultStatus: 'ENTERED',
    criticalityLevel: 'PANIC',
    isOutOfRange: true,
    autoVerificationStatus: 'FAILED',
    confidenceScore: 45,
    enteredBy: 'Tech. Smith',
    verifiedBy: null,
    enteredAt: '10:30 AM',
    verifiedAt: null,
  },
  {
    id: '2',
    sampleNumber: 'SMP-2024-002',
    patientMrn: 'PAT-10002',
    patientName: 'Jane Smith',
    testName: 'Lipid Profile',
    testParameter: 'Total Cholesterol',
    resultValue: '185',
    unit: 'mg/dL',
    referenceRange: '150-200',
    resultStatus: 'VERIFIED',
    criticalityLevel: 'NORMAL',
    isOutOfRange: false,
    autoVerificationStatus: 'PASSED',
    confidenceScore: 98,
    enteredBy: 'Tech. Johnson',
    verifiedBy: 'Dr. Williams',
    enteredAt: '09:15 AM',
    verifiedAt: '09:45 AM',
  },
  {
    id: '3',
    sampleNumber: 'SMP-2024-003',
    patientMrn: 'PAT-10003',
    patientName: 'Robert Johnson',
    testName: 'Liver Function Test',
    testParameter: 'ALT',
    resultValue: null,
    unit: 'U/L',
    referenceRange: '7-56',
    resultStatus: 'PENDING',
    criticalityLevel: 'NORMAL',
    isOutOfRange: false,
    autoVerificationStatus: null,
    confidenceScore: null,
    enteredBy: null,
    verifiedBy: null,
    enteredAt: null,
    verifiedAt: null,
  },
  {
    id: '4',
    sampleNumber: 'SMP-2024-004',
    patientMrn: 'PAT-10004',
    patientName: 'Emily Davis',
    testName: 'Glucose',
    testParameter: 'Blood Glucose',
    resultValue: '220',
    unit: 'mg/dL',
    referenceRange: '70-100',
    resultStatus: 'ENTERED',
    criticalityLevel: 'WARNING',
    isOutOfRange: true,
    autoVerificationStatus: 'NEEDS_REVIEW',
    confidenceScore: 75,
    enteredBy: 'Tech. Brown',
    verifiedBy: null,
    enteredAt: '11:00 AM',
    verifiedAt: null,
  },
];

export default function ResultsPage() {
  const [results, setResults] = useState<TestResult[]>(mockResults);
  const [searchQuery, setSearchQuery] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('ALL');
  const [criticalityFilter, setCriticalityFilter] = useState<string>('ALL');
  const addNotification = useNotificationStore((state) => state.addNotification);

  // WebSocket real-time updates
  useEffect(() => {
    const ws = getWebSocketClient();

    const unsubscribe = ws.on('result.status_changed', (event) => {
      const { result_id, new_status, verified_by } = (event as { payload: { result_id: string; new_status: string; verified_by: string } }).payload;

      setResults((prev) =>
        prev.map((result) =>
          result.id === result_id
            ? { ...result, resultStatus: new_status as TestResult['resultStatus'], verifiedBy: verified_by }
            : result
        )
      );

      addNotification({
        type: 'info',
        title: 'Result Updated',
        message: `Result status changed to ${new_status}`,
      });
    });

    const unsubscribeCritical = ws.on('result.critical_value_detected', (event) => {
      const criticalResult = (event as { payload: { testParameter: string; resultValue: string; unit: string } }).payload;

      addNotification({
        type: 'error',
        title: 'Critical Value Alert',
        message: `${criticalResult.testParameter}: ${criticalResult.resultValue} ${criticalResult.unit}`,
      });
    });

    return () => {
      unsubscribe();
      unsubscribeCritical();
    };
  }, [addNotification]);

  // Filter results - derived state using useMemo
  const filteredResults = useMemo(() => {
    let filtered = results;

    if (searchQuery) {
      filtered = filtered.filter(
        (result) =>
          result.sampleNumber.toLowerCase().includes(searchQuery.toLowerCase()) ||
          result.patientName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          result.patientMrn.toLowerCase().includes(searchQuery.toLowerCase()) ||
          result.testName.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }

    if (statusFilter !== 'ALL') {
      filtered = filtered.filter((result) => result.resultStatus === statusFilter);
    }

    if (criticalityFilter !== 'ALL') {
      filtered = filtered.filter((result) => result.criticalityLevel === criticalityFilter);
    }

    return filtered;
  }, [results, searchQuery, statusFilter, criticalityFilter]);

  // Stats
  const stats = {
    total: results.length,
    pending: results.filter((r) => r.resultStatus === 'PENDING').length,
    entered: results.filter((r) => r.resultStatus === 'ENTERED').length,
    verified: results.filter((r) => r.resultStatus === 'VERIFIED').length,
    critical: results.filter((r) => r.criticalityLevel === 'CRITICAL' || r.criticalityLevel === 'PANIC').length,
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Test Results
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Result entry, verification, and management
        </p>
      </div>

      {/* Stats */}
      <div className="grid gap-4 md:grid-cols-5">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Results</CardTitle>
            <Beaker className="h-4 w-4 text-gray-600 dark:text-gray-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.total}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pending Entry</CardTitle>
            <Clock className="h-4 w-4 text-yellow-600 dark:text-yellow-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
              {stats.pending}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Awaiting Verification</CardTitle>
            <FileCheck className="h-4 w-4 text-blue-600 dark:text-blue-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {stats.entered}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Verified</CardTitle>
            <CheckCircle2 className="h-4 w-4 text-green-600 dark:text-green-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">
              {stats.verified}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Critical Values</CardTitle>
            <AlertTriangle className="h-4 w-4 text-red-600 dark:text-red-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600 dark:text-red-400">
              {stats.critical}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Filters */}
      <Card>
        <CardHeader>
          <CardTitle>Filters</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-4">
            <div className="relative flex-1 min-w-[200px]">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
              <Input
                placeholder="Search by sample, patient, or test..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-10"
              />
            </div>

            <select
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
              className="flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
            >
              <option value="ALL">All Statuses</option>
              <option value="PENDING">Pending Entry</option>
              <option value="ENTERED">Awaiting Verification</option>
              <option value="VERIFIED">Verified</option>
              <option value="APPROVED">Approved</option>
            </select>

            <select
              value={criticalityFilter}
              onChange={(e) => setCriticalityFilter(e.target.value)}
              className="flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
            >
              <option value="ALL">All Criticality Levels</option>
              <option value="NORMAL">Normal</option>
              <option value="WARNING">Warning</option>
              <option value="CRITICAL">Critical</option>
              <option value="PANIC">Panic Value</option>
            </select>
          </div>
        </CardContent>
      </Card>

      {/* Results Grid */}
      <div className="grid gap-4 md:grid-cols-2">
        {filteredResults.map((result) => (
          <ResultCard key={result.id} result={result} />
        ))}
      </div>

      {filteredResults.length === 0 && (
        <Card>
          <CardContent className="flex h-64 flex-col items-center justify-center">
            <Beaker className="h-12 w-12 text-gray-400" />
            <p className="mt-4 text-gray-600 dark:text-gray-400">
              No results found matching your filters
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
