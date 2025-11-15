'use client';

import { useState, useEffect, useMemo } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  TestTube2,
  Search,

  QrCode,
  Clock,
  CheckCircle2,
  AlertTriangle,
  XCircle,
  Loader2,
  Eye,
} from 'lucide-react';
import { getWebSocketClient } from '@/lib/websocket-client';
import { useNotificationStore } from '@/lib/store';

interface Sample {
  id: string;
  sampleNumber: string;
  patientMrn: string;
  patientName: string;
  testName: string;
  sampleType: string;
  sampleStatus: 'COLLECTED' | 'RECEIVED' | 'PROCESSING' | 'COMPLETED' | 'REJECTED';
  collectedAt: string;
  collectedBy: string;
  priority: 'ROUTINE' | 'URGENT' | 'STAT';
  lastUpdated: string;
}

// Sample statuses with colors and icons
const statusConfig = {
  COLLECTED: {
    label: 'Collected',
    color: 'bg-blue-100 text-blue-800 dark:bg-blue-950 dark:text-blue-300',
    icon: Clock,
  },
  RECEIVED: {
    label: 'Received',
    color: 'bg-purple-100 text-purple-800 dark:bg-purple-950 dark:text-purple-300',
    icon: CheckCircle2,
  },
  PROCESSING: {
    label: 'Processing',
    color: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-950 dark:text-yellow-300',
    icon: Loader2,
  },
  COMPLETED: {
    label: 'Completed',
    color: 'bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300',
    icon: CheckCircle2,
  },
  REJECTED: {
    label: 'Rejected',
    color: 'bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300',
    icon: XCircle,
  },
};

const priorityConfig = {
  ROUTINE: {
    label: 'Routine',
    color: 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300',
  },
  URGENT: {
    label: 'Urgent',
    color: 'bg-orange-100 text-orange-800 dark:bg-orange-950 dark:text-orange-300',
  },
  STAT: {
    label: 'STAT',
    color: 'bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300',
  },
};

function SampleCard({ sample }: { sample: Sample }) {
  const status = statusConfig[sample.sampleStatus];
  const priority = priorityConfig[sample.priority];
  const StatusIcon = status.icon;

  return (
    <div className="rounded-lg border border-gray-200 p-4 transition-all hover:border-blue-500 hover:shadow-md dark:border-gray-800 dark:hover:border-blue-500">
      <div className="mb-3 flex items-start justify-between">
        <div className="flex items-center gap-3">
          <div className="flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-950">
            <TestTube2 className="h-6 w-6 text-blue-600 dark:text-blue-400" />
          </div>
          <div>
            <p className="font-medium text-gray-900 dark:text-white">{sample.sampleNumber}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {sample.patientName} ({sample.patientMrn})
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <span className={`rounded-full px-2 py-1 text-xs font-medium ${priority.color}`}>
            {priority.label}
          </span>
          <span className={`flex items-center gap-1 rounded-full px-2 py-1 text-xs font-medium ${status.color}`}>
            <StatusIcon className="h-3 w-3" />
            {status.label}
          </span>
        </div>
      </div>

      <div className="mb-3 grid gap-2 text-sm">
        <div className="flex justify-between">
          <span className="text-gray-600 dark:text-gray-400">Test:</span>
          <span className="font-medium text-gray-900 dark:text-white">{sample.testName}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-600 dark:text-gray-400">Sample Type:</span>
          <span className="font-medium text-gray-900 dark:text-white">{sample.sampleType}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-600 dark:text-gray-400">Collected:</span>
          <span className="font-medium text-gray-900 dark:text-white">{sample.collectedAt}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-600 dark:text-gray-400">Collected By:</span>
          <span className="font-medium text-gray-900 dark:text-white">{sample.collectedBy}</span>
        </div>
      </div>

      {/* Timeline */}
      <div className="mb-3 border-t border-gray-200 pt-3 dark:border-gray-800">
        <p className="mb-2 text-xs font-medium text-gray-600 dark:text-gray-400">Sample Timeline</p>
        <div className="flex items-center gap-2">
          {['COLLECTED', 'RECEIVED', 'PROCESSING', 'COMPLETED'].map((step, index) => (
            <div key={step} className="flex flex-1 items-center">
              <div
                className={`flex h-2 w-2 rounded-full ${
                  ['COLLECTED', 'RECEIVED', 'PROCESSING', 'COMPLETED'].indexOf(sample.sampleStatus) >= index
                    ? 'bg-blue-600'
                    : 'bg-gray-300 dark:bg-gray-700'
                }`}
              />
              {index < 3 && (
                <div
                  className={`h-0.5 flex-1 ${
                    ['COLLECTED', 'RECEIVED', 'PROCESSING', 'COMPLETED'].indexOf(sample.sampleStatus) > index
                      ? 'bg-blue-600'
                      : 'bg-gray-300 dark:bg-gray-700'
                  }`}
                />
              )}
            </div>
          ))}
        </div>
      </div>

      <div className="flex gap-2">
        <Button size="sm" variant="outline" className="flex-1">
          <Eye className="mr-2 h-4 w-4" />
          View Details
        </Button>
        {sample.sampleStatus === 'COLLECTED' && (
          <Button size="sm" className="flex-1">
            Mark Received
          </Button>
        )}
        {sample.sampleStatus === 'RECEIVED' && (
          <Button size="sm" className="flex-1">
            Start Processing
          </Button>
        )}
      </div>
    </div>
  );
}

const mockSamples: Sample[] = [
  {
    id: '1',
    sampleNumber: 'SMP-2024-001',
    patientMrn: 'PAT-10001',
    patientName: 'John Doe',
    testName: 'Complete Blood Count',
    sampleType: 'Blood (EDTA)',
    sampleStatus: 'PROCESSING',
    collectedAt: 'Today, 10:30 AM',
    collectedBy: 'Dr. Smith',
    priority: 'URGENT',
    lastUpdated: '2 min ago',
  },
  {
    id: '2',
    sampleNumber: 'SMP-2024-002',
    patientMrn: 'PAT-10002',
    patientName: 'Jane Smith',
    testName: 'Lipid Profile',
    sampleType: 'Serum',
    sampleStatus: 'RECEIVED',
    collectedAt: 'Today, 09:15 AM',
    collectedBy: 'Dr. Johnson',
    priority: 'ROUTINE',
    lastUpdated: '15 min ago',
  },
  {
    id: '3',
    sampleNumber: 'SMP-2024-003',
    patientMrn: 'PAT-10003',
    patientName: 'Robert Johnson',
    testName: 'Liver Function Test',
    sampleType: 'Serum',
    sampleStatus: 'COMPLETED',
    collectedAt: 'Today, 08:00 AM',
    collectedBy: 'Dr. Williams',
    priority: 'ROUTINE',
    lastUpdated: '1 hr ago',
  },
  {
    id: '4',
    sampleNumber: 'SMP-2024-004',
    patientMrn: 'PAT-10004',
    patientName: 'Emily Davis',
    testName: 'HbA1c',
    sampleType: 'Blood (EDTA)',
    sampleStatus: 'COLLECTED',
    collectedAt: 'Today, 11:00 AM',
    collectedBy: 'Nurse Brown',
    priority: 'STAT',
    lastUpdated: 'Just now',
  },
];

export default function SamplesPage() {
  const [samples, setSamples] = useState<Sample[]>(mockSamples);
  const [searchQuery, setSearchQuery] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('ALL');
  const [priorityFilter, setPriorityFilter] = useState<string>('ALL');
  const addNotification = useNotificationStore((state) => state.addNotification);

  // WebSocket real-time updates
  useEffect(() => {
    const ws = getWebSocketClient();

    // Subscribe to sample status changes
    const unsubscribe = ws.on('sample.status_changed', (event) => {
      const { sample_id, new_status } = (event as { payload: { sample_id: string; new_status: Sample['sampleStatus'] } }).payload;

      // Update sample status in real-time
      setSamples((prev) =>
        prev.map((sample) =>
          sample.id === sample_id
            ? { ...sample, sampleStatus: new_status, lastUpdated: 'Just now' }
            : sample
        )
      );

      // Show notification
      addNotification({
        type: 'info',
        title: 'Sample Updated',
        message: `Sample status changed to ${new_status}`,
      });
    });

    // Subscribe to new samples
    const unsubscribeNew = ws.on('sample.created', (event) => {
      const newSample = (event as { payload: Sample }).payload;
      setSamples((prev) => [newSample, ...prev]);

      addNotification({
        type: 'success',
        title: 'New Sample',
        message: `Sample ${newSample.sampleNumber} has been collected`,
      });
    });

    return () => {
      unsubscribe();
      unsubscribeNew();
    };
  }, [addNotification]);

  // Filter samples - derived state using useMemo
  const filteredSamples = useMemo(() => {
    let filtered = samples;

    // Search filter
    if (searchQuery) {
      filtered = filtered.filter(
        (sample) =>
          sample.sampleNumber.toLowerCase().includes(searchQuery.toLowerCase()) ||
          sample.patientName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          sample.patientMrn.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }

    // Status filter
    if (statusFilter !== 'ALL') {
      filtered = filtered.filter((sample) => sample.sampleStatus === statusFilter);
    }

    // Priority filter
    if (priorityFilter !== 'ALL') {
      filtered = filtered.filter((sample) => sample.priority === priorityFilter);
    }

    return filtered;
  }, [samples, searchQuery, statusFilter, priorityFilter]);

  // Stats
  const stats = {
    total: samples.length,
    collected: samples.filter((s) => s.sampleStatus === 'COLLECTED').length,
    processing: samples.filter((s) => s.sampleStatus === 'PROCESSING').length,
    completed: samples.filter((s) => s.sampleStatus === 'COMPLETED').length,
    urgent: samples.filter((s) => s.priority === 'URGENT' || s.priority === 'STAT').length,
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Sample Tracking
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            Real-time sample status monitoring
          </p>
        </div>
        <Button>
          <QrCode className="mr-2 h-4 w-4" />
          Scan Barcode
        </Button>
      </div>

      {/* Stats */}
      <div className="grid gap-4 md:grid-cols-5">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Samples</CardTitle>
            <TestTube2 className="h-4 w-4 text-gray-600 dark:text-gray-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.total}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Collected</CardTitle>
            <Clock className="h-4 w-4 text-blue-600 dark:text-blue-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {stats.collected}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Processing</CardTitle>
            <Loader2 className="h-4 w-4 text-yellow-600 dark:text-yellow-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
              {stats.processing}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Completed</CardTitle>
            <CheckCircle2 className="h-4 w-4 text-green-600 dark:text-green-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">
              {stats.completed}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Urgent</CardTitle>
            <AlertTriangle className="h-4 w-4 text-red-600 dark:text-red-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600 dark:text-red-400">
              {stats.urgent}
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
            {/* Search */}
            <div className="relative flex-1 min-w-[200px]">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
              <Input
                placeholder="Search by sample number, patient name, or MRN..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-10"
              />
            </div>

            {/* Status Filter */}
            <select
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
              className="flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
            >
              <option value="ALL">All Statuses</option>
              <option value="COLLECTED">Collected</option>
              <option value="RECEIVED">Received</option>
              <option value="PROCESSING">Processing</option>
              <option value="COMPLETED">Completed</option>
              <option value="REJECTED">Rejected</option>
            </select>

            {/* Priority Filter */}
            <select
              value={priorityFilter}
              onChange={(e) => setPriorityFilter(e.target.value)}
              className="flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
            >
              <option value="ALL">All Priorities</option>
              <option value="ROUTINE">Routine</option>
              <option value="URGENT">Urgent</option>
              <option value="STAT">STAT</option>
            </select>
          </div>
        </CardContent>
      </Card>

      {/* Samples Grid */}
      <div className="grid gap-4 md:grid-cols-2">
        {filteredSamples.map((sample) => (
          <SampleCard key={sample.id} sample={sample} />
        ))}
      </div>

      {filteredSamples.length === 0 && (
        <Card>
          <CardContent className="flex h-64 flex-col items-center justify-center">
            <TestTube2 className="h-12 w-12 text-gray-400" />
            <p className="mt-4 text-gray-600 dark:text-gray-400">
              No samples found matching your filters
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
