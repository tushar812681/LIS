'use client';

import { useMemo } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { FileText, Users, AlertCircle, TrendingUp, Download, Eye, Loader2, Search } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useReports, useResults } from '@/lib/hooks';
import { format, formatDistanceToNow } from 'date-fns';
import Link from 'next/link';

export const dynamic = 'force-dynamic';

interface ReportCardProps {
  id: string;
  patientName: string;
  mrn: string;
  testName: string;
  reportDate: string;
  status: string;
  isCritical?: boolean;
  criticalValues?: string[];
}

function ReportCard({ id, patientName, mrn, testName, reportDate, status, isCritical, criticalValues }: ReportCardProps) {
  const statusConfig: Record<string, { color: string; label: string }> = {
    DRAFT: { color: 'bg-gray-100 text-gray-800 dark:bg-gray-950 dark:text-gray-300', label: 'Draft' },
    APPROVED: { color: 'bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300', label: 'Approved' },
    DELIVERED: { color: 'bg-blue-100 text-blue-800 dark:bg-blue-950 dark:text-blue-300', label: 'Delivered' },
    PENDING: { color: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-950 dark:text-yellow-300', label: 'Pending' },
  };

  const config = statusConfig[status] || statusConfig.PENDING;
  const displayStatus = isCritical ? 'Critical' : config.label;
  const displayColor = isCritical ? 'bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300' : config.color;

  return (
    <Link
      href={`/dashboard/reports/${id}`}
      className="rounded-lg border border-gray-200 p-4 transition-all hover:border-blue-500 hover:shadow-md dark:border-gray-800 dark:hover:border-blue-500 block"
    >
      <div className="mb-3 flex items-start justify-between">
        <div>
          <p className="font-medium text-gray-900 dark:text-white">{patientName}</p>
          <p className="text-sm text-gray-600 dark:text-gray-400">MRN: {mrn}</p>
        </div>
        <span className={`rounded-full px-3 py-1 text-xs font-medium ${displayColor}`}>
          {displayStatus}
        </span>
      </div>

      <div className="mb-3 space-y-1">
        <p className="text-sm font-medium text-gray-900 dark:text-white">{testName}</p>
        <p className="text-xs text-gray-500 dark:text-gray-500">Report Date: {reportDate}</p>
      </div>

      {isCritical && criticalValues && criticalValues.length > 0 && (
        <div className="mb-3 rounded-md bg-red-50 p-2 dark:bg-red-950/50">
          <div className="flex items-center gap-2">
            <AlertCircle className="h-4 w-4 text-red-600 dark:text-red-400" />
            <p className="text-xs font-medium text-red-800 dark:text-red-300">Critical Values</p>
          </div>
          <ul className="ml-6 mt-1 list-disc text-xs text-red-700 dark:text-red-400">
            {criticalValues.map((value, idx) => (
              <li key={idx}>{value}</li>
            ))}
          </ul>
        </div>
      )}

      <div className="flex gap-2">
        <Button size="sm" variant="outline" className="flex-1" onClick={(e) => e.preventDefault()}>
          <Eye className="mr-2 h-4 w-4" />
          View
        </Button>
        <Button size="sm" className="flex-1" onClick={(e) => e.preventDefault()}>
          <Download className="mr-2 h-4 w-4" />
          Download
        </Button>
      </div>
    </Link>
  );
}

export default function DoctorDashboard() {
  // Fetch reports and results from GraphQL API
  const { reports, loading: reportsLoading, error: reportsError, refetch: refetchReports } = useReports({
    page: 1,
    limit: 20,
    sort: { generatedAt: 'DESC' },
  });

  const { results, loading: resultsLoading } = useResults({
    page: 1,
    limit: 20,
    sort: { resultDate: 'DESC' },
  });

  // Calculate stats from real data
  const stats = useMemo(() => {
    const today = new Date();
    today.setHours(0, 0, 0, 0);

    const reportsToday = reports.filter((r: any) => {
      if (!r.generatedAt) return false;
      const reportDate = new Date(r.generatedAt);
      reportDate.setHours(0, 0, 0, 0);
      return reportDate.getTime() === today.getTime();
    }).length;

    const criticalReports = results.filter((r: any) => r.isCritical).length;
    const pendingReports = results.filter((r: any) => r.status === 'PENDING' || r.status === 'ENTERED').length;

    // Get unique patients from reports
    const uniquePatients = new Set(reports.map((r: any) => r.patient?.id).filter(Boolean));

    return {
      totalPatients: uniquePatients.size,
      reportsToday,
      criticalReports,
      pendingReports,
    };
  }, [reports, results]);

  // Transform reports for display
  const displayReports: ReportCardProps[] = useMemo(() => {
    return reports.slice(0, 8).map((report: any) => ({
      id: report.id,
      patientName: report.patient ? `${report.patient.firstName} ${report.patient.lastName}` : 'Unknown Patient',
      mrn: report.patient?.patientId || 'N/A',
      testName: report.reportType || 'Lab Report',
      reportDate: report.generatedAt
        ? formatDistanceToNow(new Date(report.generatedAt), { addSuffix: true })
        : 'N/A',
      status: report.status,
      isCritical: report.results?.some((r: any) => r.isCritical) || false,
      criticalValues: report.results
        ?.filter((r: any) => r.isCritical)
        .flatMap((r: any) =>
          r.parameters
            ?.filter((p: any) => p.isCritical)
            .map((p: any) => `${p.parameterName}: ${p.value} ${p.unit || ''}`)
        )
        .slice(0, 3) || [],
    }));
  }, [reports]);

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Doctor Dashboard
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Your patients&apos; test reports and results
        </p>
      </div>

      {/* Error State */}
      {reportsError && (
        <Card className="border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950/50">
          <CardContent className="flex items-center gap-2 pt-6 text-red-800 dark:text-red-300">
            <AlertCircle className="h-5 w-5" />
            <p>Failed to load reports: {reportsError.message}</p>
            <Button onClick={() => refetchReports()} variant="outline" size="sm" className="ml-auto">
              Retry
            </Button>
          </CardContent>
        </Card>
      )}

      {/* Stats Grid */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Patients</CardTitle>
            <Users className="h-4 w-4 text-blue-600 dark:text-blue-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.totalPatients}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Under your care
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Reports Today</CardTitle>
            <FileText className="h-4 w-4 text-green-600 dark:text-green-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">
              {stats.reportsToday}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Available for review
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Critical Reports</CardTitle>
            <AlertCircle className="h-4 w-4 text-red-600 dark:text-red-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600 dark:text-red-400">
              {stats.criticalReports}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Requires immediate attention
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pending Results</CardTitle>
            <TrendingUp className="h-4 w-4 text-yellow-600 dark:text-yellow-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
              {stats.pendingReports}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              In processing
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Critical Reports Alert */}
      {stats.criticalReports > 0 && (
        <div className="rounded-lg border-2 border-red-200 bg-red-50 p-4 dark:border-red-900 dark:bg-red-950/50">
          <div className="flex items-center gap-3">
            <AlertCircle className="h-6 w-6 text-red-600 dark:text-red-400" />
            <div>
              <p className="font-medium text-red-900 dark:text-red-300">
                {stats.criticalReports} Critical Report{stats.criticalReports > 1 ? 's' : ''} Available
              </p>
              <p className="text-sm text-red-700 dark:text-red-400">
                These reports contain critical values that require immediate clinical attention
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Recent Reports */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Recent Patient Reports</CardTitle>
              <CardDescription>Latest test results for your patients</CardDescription>
            </div>
            <Link href="/dashboard/reports">
              <Button variant="outline" size="sm">
                View All
              </Button>
            </Link>
          </div>
        </CardHeader>
        <CardContent>
          {reportsLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="h-8 w-8 animate-spin text-gray-400" />
              <p className="ml-2 text-gray-600 dark:text-gray-400">Loading reports...</p>
            </div>
          ) : displayReports.length === 0 ? (
            <div className="py-12 text-center">
              <FileText className="mx-auto h-12 w-12 text-gray-400" />
              <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
                No reports available
              </h3>
              <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
                Reports will appear here when lab results are ready
              </p>
            </div>
          ) : (
            <div className="grid gap-4 md:grid-cols-2">
              {displayReports.map((report) => (
                <ReportCard key={report.id} {...report} />
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Patient Search */}
      <Card>
        <CardHeader>
          <CardTitle>Quick Patient Search</CardTitle>
          <CardDescription>Find patient reports by MRN or name</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex gap-2">
            <Input
              type="search"
              placeholder="Enter patient MRN or name..."
              className="flex-1"
            />
            <Button>
              <Search className="mr-2 h-4 w-4" />
              Search
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
