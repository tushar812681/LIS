'use client';

import { useMemo } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { TestTube2, ClipboardCheck, AlertTriangle, CheckCircle2, Clock, Beaker, Loader2, AlertCircle } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useSamples, useEquipment } from '@/lib/hooks';
import { format, formatDistanceToNow } from 'date-fns';
import Link from 'next/link';

export const dynamic = 'force-dynamic';

interface SampleCardProps {
  id: string;
  sampleNumber: string;
  patientMrn: string;
  patientName: string;
  testName: string;
  status: string;
  priority: string;
  collectedAt: string;
}

function SampleCard({ id, sampleNumber, patientMrn, patientName, testName, status, priority, collectedAt }: SampleCardProps) {
  const statusConfig: Record<string, { color: string; label: string }> = {
    COLLECTED: { color: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-950 dark:text-yellow-300', label: 'Collected' },
    RECEIVED: { color: 'bg-blue-100 text-blue-800 dark:bg-blue-950 dark:text-blue-300', label: 'Received' },
    PROCESSING: { color: 'bg-purple-100 text-purple-800 dark:bg-purple-950 dark:text-purple-300', label: 'Processing' },
    COMPLETED: { color: 'bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300', label: 'Completed' },
    REJECTED: { color: 'bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300', label: 'Rejected' },
  };

  const isUrgent = priority === 'URGENT' || priority === 'STAT';
  const config = statusConfig[status] || statusConfig.COLLECTED;

  return (
    <Link
      href={`/dashboard/samples/${id}`}
      className="flex items-center justify-between rounded-lg border border-gray-200 p-4 transition-all hover:border-blue-500 hover:shadow-md dark:border-gray-800 dark:hover:border-blue-500"
    >
      <div className="flex items-center gap-4">
        <div className={`flex h-12 w-12 items-center justify-center rounded-full ${isUrgent ? 'bg-red-100 dark:bg-red-950' : 'bg-blue-100 dark:bg-blue-950'}`}>
          <TestTube2 className={`h-6 w-6 ${isUrgent ? 'text-red-600 dark:text-red-400' : 'text-blue-600 dark:text-blue-400'}`} />
        </div>
        <div>
          <div className="flex items-center gap-2">
            <p className="font-medium text-gray-900 dark:text-white">{sampleNumber}</p>
            {isUrgent && (
              <span className="rounded-full bg-red-100 px-2 py-0.5 text-xs font-medium text-red-800 dark:bg-red-950 dark:text-red-300">
                {priority}
              </span>
            )}
          </div>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {patientName} ({patientMrn})
          </p>
          <p className="text-xs text-gray-500 dark:text-gray-500">
            Collected: {collectedAt}
          </p>
        </div>
      </div>
      <div className="flex items-center gap-3">
        <span className={`rounded-full px-3 py-1 text-xs font-medium ${config.color}`}>
          {config.label}
        </span>
        <Button size="sm" onClick={(e) => { e.preventDefault(); /* Handle process action */ }}>
          Process
        </Button>
      </div>
    </Link>
  );
}

export default function LabDashboard() {
  // Fetch samples data from GraphQL API
  const { samples, loading: samplesLoading, error: samplesError, refetch: refetchSamples } = useSamples({
    page: 1,
    limit: 20,
    sort: { collectionDate: 'DESC' },
  });

  // Fetch equipment data
  const { equipment, loading: equipmentLoading } = useEquipment({
    page: 1,
    limit: 10,
  });

  // Calculate stats from real data
  const stats = useMemo(() => {
    const pending = samples.filter((s: any) => s.status === 'COLLECTED' || s.status === 'RECEIVED').length;
    const inProgress = samples.filter((s: any) => s.status === 'PROCESSING').length;
    const completed = samples.filter((s: any) => s.status === 'COMPLETED').length;
    const urgent = samples.filter((s: any) => s.priority === 'URGENT' || s.priority === 'STAT').length;

    return {
      pendingSamples: pending,
      inProgressSamples: inProgress,
      completedToday: completed,
      urgentSamples: urgent,
    };
  }, [samples]);

  // Transform samples for display
  const displaySamples: SampleCardProps[] = useMemo(() => {
    return samples.slice(0, 10).map((sample: any) => ({
      id: sample.id,
      sampleNumber: sample.sampleId,
      patientMrn: sample.patient?.patientId || 'N/A',
      patientName: sample.patient ? `${sample.patient.firstName} ${sample.patient.lastName}` : 'Unknown',
      testName: sample.sampleType || 'N/A',
      status: sample.status,
      priority: sample.priority || 'ROUTINE',
      collectedAt: sample.collectionDate
        ? formatDistanceToNow(new Date(sample.collectionDate), { addSuffix: true })
        : 'N/A',
    }));
  }, [samples]);

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Lab Technician Dashboard
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Your sample processing workbench
        </p>
      </div>

      {/* Error State */}
      {samplesError && (
        <Card className="border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950/50">
          <CardContent className="flex items-center gap-2 pt-6 text-red-800 dark:text-red-300">
            <AlertCircle className="h-5 w-5" />
            <p>Failed to load samples: {samplesError.message}</p>
            <Button onClick={() => refetchSamples()} variant="outline" size="sm" className="ml-auto">
              Retry
            </Button>
          </CardContent>
        </Card>
      )}

      {/* Stats Grid */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pending Samples</CardTitle>
            <Clock className="h-4 w-4 text-yellow-600 dark:text-yellow-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
              {stats.pendingSamples}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Awaiting processing
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">In Progress</CardTitle>
            <Beaker className="h-4 w-4 text-blue-600 dark:text-blue-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {stats.inProgressSamples}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Currently processing
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Completed Today</CardTitle>
            <CheckCircle2 className="h-4 w-4 text-green-600 dark:text-green-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">
              {stats.completedToday}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Finished samples
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Urgent Samples</CardTitle>
            <AlertTriangle className="h-4 w-4 text-red-600 dark:text-red-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600 dark:text-red-400">
              {stats.urgentSamples}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Requires immediate attention
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Sample Queue */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Sample Processing Queue</CardTitle>
              <CardDescription>Samples waiting for processing</CardDescription>
            </div>
            <Link href="/dashboard/samples">
              <Button variant="outline" size="sm">
                View All
              </Button>
            </Link>
          </div>
        </CardHeader>
        <CardContent>
          {samplesLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="h-8 w-8 animate-spin text-gray-400" />
              <p className="ml-2 text-gray-600 dark:text-gray-400">Loading samples...</p>
            </div>
          ) : displaySamples.length === 0 ? (
            <div className="py-12 text-center">
              <TestTube2 className="mx-auto h-12 w-12 text-gray-400" />
              <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
                No samples in queue
              </h3>
              <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
                All samples have been processed
              </p>
            </div>
          ) : (
            <div className="space-y-3">
              {displaySamples.map((sample) => (
                <SampleCard key={sample.id} {...sample} />
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Quick Actions */}
      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Quick Actions</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            <Button className="w-full justify-start" variant="outline">
              <TestTube2 className="mr-2 h-4 w-4" />
              Scan Barcode
            </Button>
            <Button className="w-full justify-start" variant="outline">
              <ClipboardCheck className="mr-2 h-4 w-4" />
              Record QC Result
            </Button>
            <Button className="w-full justify-start" variant="outline">
              <Beaker className="mr-2 h-4 w-4" />
              Start Batch Processing
            </Button>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle>Equipment Status</CardTitle>
              <Link href="/dashboard/equipment">
                <Button variant="outline" size="sm">
                  View All
                </Button>
              </Link>
            </div>
          </CardHeader>
          <CardContent>
            {equipmentLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-6 w-6 animate-spin text-gray-400" />
              </div>
            ) : equipment.length === 0 ? (
              <div className="py-8 text-center text-sm text-gray-500">
                No equipment registered
              </div>
            ) : (
              <div className="space-y-3">
                {equipment.slice(0, 5).map((item: any) => {
                  const statusColor =
                    item.status === 'OPERATIONAL' ? 'bg-green-500' :
                    item.status === 'MAINTENANCE' ? 'bg-yellow-500' :
                    item.status === 'OUT_OF_SERVICE' ? 'bg-red-500' :
                    'bg-gray-500';

                  return (
                    <Link
                      key={item.id}
                      href={`/dashboard/equipment/${item.id}`}
                      className="flex items-center justify-between rounded-lg p-2 transition-colors hover:bg-gray-50 dark:hover:bg-gray-800/50"
                    >
                      <div className="flex items-center gap-2">
                        <div className={`h-3 w-3 rounded-full ${statusColor}`} />
                        <span className="text-sm text-gray-900 dark:text-white">{item.name}</span>
                      </div>
                      <span className="text-xs text-gray-600 dark:text-gray-400">
                        {item.status.replace(/_/g, ' ').toLowerCase().replace(/\b\w/g, (c: string) => c.toUpperCase())}
                      </span>
                    </Link>
                  );
                })}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
