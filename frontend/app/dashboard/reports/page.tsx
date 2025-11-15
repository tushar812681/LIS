'use client';

import { useState, useMemo } from 'react';
import { useRouter } from 'next/navigation';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  FileText,
  Search,
  Download,
  Eye,
  Send,
  CheckCircle2,
  Clock,

  Printer,
  Mail,
  MessageSquare,
  Shield,
} from 'lucide-react';
import { useNotificationStore } from '@/lib/store';

interface Report {
  id: string;
  reportNumber: string;
  sampleNumber: string;
  patientMrn: string;
  patientName: string;
  testName: string;
  reportStatus: 'DRAFT' | 'GENERATED' | 'APPROVED' | 'DELIVERED';
  reportType: 'INDIVIDUAL' | 'CUMULATIVE' | 'INTERIM';
  generatedAt: string;
  approvedBy: string | null;
  approvedAt: string | null;
  isSigned: boolean;
  signatoryName: string | null;
  deliveryChannels: ('EMAIL' | 'WHATSAPP' | 'SMS' | 'PRINT')[];
  deliveredAt: string | null;
  downloadCount: number;
}

const statusConfig = {
  DRAFT: {
    label: 'Draft',
    color: 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300',
    icon: Clock,
  },
  GENERATED: {
    label: 'Generated',
    color: 'bg-blue-100 text-blue-800 dark:bg-blue-950 dark:text-blue-300',
    icon: FileText,
  },
  APPROVED: {
    label: 'Approved',
    color: 'bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300',
    icon: CheckCircle2,
  },
  DELIVERED: {
    label: 'Delivered',
    color: 'bg-purple-100 text-purple-800 dark:bg-purple-950 dark:text-purple-300',
    icon: Send,
  },
};

function ReportCard({ report }: { report: Report }) {
  const router = useRouter();
  const status = statusConfig[report.reportStatus];
  const StatusIcon = status.icon;
  const addNotification = useNotificationStore((state) => state.addNotification);

  const handleDownload = () => {
    addNotification({
      type: 'success',
      title: 'Download Started',
      message: `Downloading ${report.reportNumber}.pdf`,
    });
    // In production, trigger actual PDF download
  };

  const handleSend = (channel: 'email' | 'whatsapp' | 'sms') => {
    addNotification({
      type: 'info',
      title: 'Sending Report',
      message: `Sending via ${channel}...`,
    });
    // In production, call API to send report
  };

  const handlePrint = () => {
    addNotification({
      type: 'info',
      title: 'Printing',
      message: 'Preparing report for printing...',
    });
    // In production, trigger print dialog
  };

  return (
    <div className="rounded-lg border border-gray-200 p-4 transition-all hover:border-blue-500 hover:shadow-md dark:border-gray-800 dark:hover:border-blue-500">
      {/* Header */}
      <div className="mb-3 flex items-start justify-between">
        <div className="flex items-center gap-3">
          <div className="flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-950">
            <FileText className="h-6 w-6 text-blue-600 dark:text-blue-400" />
          </div>
          <div>
            <p className="font-medium text-gray-900 dark:text-white">{report.reportNumber}</p>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {report.patientName} ({report.patientMrn})
            </p>
          </div>
        </div>
        <span className={`flex items-center gap-1 rounded-full px-2 py-1 text-xs font-medium ${status.color}`}>
          <StatusIcon className="h-3 w-3" />
          {status.label}
        </span>
      </div>

      {/* Report Details */}
      <div className="mb-3 space-y-2 rounded-lg bg-gray-50 p-3 dark:bg-gray-800">
        <div className="flex justify-between text-sm">
          <span className="text-gray-600 dark:text-gray-400">Test:</span>
          <span className="font-medium text-gray-900 dark:text-white">{report.testName}</span>
        </div>
        <div className="flex justify-between text-sm">
          <span className="text-gray-600 dark:text-gray-400">Sample:</span>
          <span className="font-medium text-gray-900 dark:text-white">{report.sampleNumber}</span>
        </div>
        <div className="flex justify-between text-sm">
          <span className="text-gray-600 dark:text-gray-400">Type:</span>
          <span className="font-medium text-gray-900 dark:text-white">{report.reportType}</span>
        </div>
        <div className="flex justify-between text-sm">
          <span className="text-gray-600 dark:text-gray-400">Generated:</span>
          <span className="font-medium text-gray-900 dark:text-white">{report.generatedAt}</span>
        </div>
      </div>

      {/* Signature Status */}
      {report.isSigned && (
        <div className="mb-3 flex items-center gap-2 rounded-lg border border-green-200 bg-green-50 p-2 dark:border-green-900 dark:bg-green-950/50">
          <Shield className="h-4 w-4 text-green-600 dark:text-green-400" />
          <div>
            <p className="text-xs font-medium text-green-900 dark:text-green-300">
              Digitally Signed
            </p>
            <p className="text-xs text-green-700 dark:text-green-400">
              Signed by: {report.signatoryName}
            </p>
          </div>
        </div>
      )}

      {/* Delivery Status */}
      {report.deliveryChannels.length > 0 && (
        <div className="mb-3">
          <p className="mb-1 text-xs text-gray-600 dark:text-gray-400">Delivery Channels:</p>
          <div className="flex flex-wrap gap-1">
            {report.deliveryChannels.map((channel) => (
              <span
                key={channel}
                className="rounded-full bg-purple-100 px-2 py-0.5 text-xs font-medium text-purple-800 dark:bg-purple-950 dark:text-purple-300"
              >
                {channel}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="flex flex-wrap gap-2">
        <Button
          size="sm"
          variant="outline"
          onClick={() => router.push(`/dashboard/reports/${report.id}`)}
        >
          <Eye className="mr-2 h-4 w-4" />
          View
        </Button>
        <Button size="sm" variant="outline" onClick={handleDownload}>
          <Download className="mr-2 h-4 w-4" />
          Download
        </Button>
        <Button size="sm" variant="outline" onClick={handlePrint}>
          <Printer className="mr-2 h-4 w-4" />
          Print
        </Button>
        {report.reportStatus === 'APPROVED' && (
          <>
            <Button size="sm" variant="outline" onClick={() => handleSend('email')}>
              <Mail className="mr-2 h-4 w-4" />
              Email
            </Button>
            <Button size="sm" variant="outline" onClick={() => handleSend('whatsapp')}>
              <MessageSquare className="mr-2 h-4 w-4" />
              WhatsApp
            </Button>
          </>
        )}
      </div>

      {/* Download Count */}
      {report.downloadCount > 0 && (
        <p className="mt-2 text-xs text-gray-600 dark:text-gray-400">
          Downloaded {report.downloadCount} time{report.downloadCount > 1 ? 's' : ''}
        </p>
      )}
    </div>
  );
}

const mockReports: Report[] = [
  {
    id: '1',
    reportNumber: 'REP-2024-001',
    sampleNumber: 'SMP-2024-001',
    patientMrn: 'PAT-10001',
    patientName: 'John Doe',
    testName: 'Complete Blood Count',
    reportStatus: 'APPROVED',
    reportType: 'INDIVIDUAL',
    generatedAt: 'Today, 11:30 AM',
    approvedBy: 'Dr. Williams',
    approvedAt: 'Today, 11:45 AM',
    isSigned: true,
    signatoryName: 'Dr. Williams',
    deliveryChannels: ['EMAIL', 'WHATSAPP'],
    deliveredAt: 'Today, 11:50 AM',
    downloadCount: 2,
  },
  {
    id: '2',
    reportNumber: 'REP-2024-002',
    sampleNumber: 'SMP-2024-002',
    patientMrn: 'PAT-10002',
    patientName: 'Jane Smith',
    testName: 'Lipid Profile',
    reportStatus: 'GENERATED',
    reportType: 'INDIVIDUAL',
    generatedAt: 'Today, 10:15 AM',
    approvedBy: null,
    approvedAt: null,
    isSigned: false,
    signatoryName: null,
    deliveryChannels: [],
    deliveredAt: null,
    downloadCount: 0,
  },
  {
    id: '3',
    reportNumber: 'REP-2024-003',
    sampleNumber: 'SMP-2024-003',
    patientMrn: 'PAT-10003',
    patientName: 'Robert Johnson',
    testName: 'Liver Function Test',
    reportStatus: 'DELIVERED',
    reportType: 'INDIVIDUAL',
    generatedAt: 'Yesterday, 4:30 PM',
    approvedBy: 'Dr. Smith',
    approvedAt: 'Yesterday, 4:45 PM',
    isSigned: true,
    signatoryName: 'Dr. Smith',
    deliveryChannels: ['EMAIL', 'PRINT'],
    deliveredAt: 'Yesterday, 5:00 PM',
    downloadCount: 1,
  },
];

export default function ReportsPage() {
  const router = useRouter();
  const [reports] = useState<Report[]>(mockReports);
  const [searchQuery, setSearchQuery] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('ALL');

  // Filter reports - derived state using useMemo
  const filteredReports = useMemo(() => {
    let filtered = reports;

    if (searchQuery) {
      filtered = filtered.filter(
        (report) =>
          report.reportNumber.toLowerCase().includes(searchQuery.toLowerCase()) ||
          report.patientName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          report.patientMrn.toLowerCase().includes(searchQuery.toLowerCase()) ||
          report.sampleNumber.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }

    if (statusFilter !== 'ALL') {
      filtered = filtered.filter((report) => report.reportStatus === statusFilter);
    }

    return filtered;
  }, [reports, searchQuery, statusFilter]);

  // Stats
  const stats = {
    total: reports.length,
    draft: reports.filter((r) => r.reportStatus === 'DRAFT').length,
    generated: reports.filter((r) => r.reportStatus === 'GENERATED').length,
    approved: reports.filter((r) => r.reportStatus === 'APPROVED').length,
    delivered: reports.filter((r) => r.reportStatus === 'DELIVERED').length,
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Reports
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            Generate, manage, and deliver patient reports
          </p>
        </div>
        <Button onClick={() => router.push('/dashboard/reports/generate')}>
          <FileText className="mr-2 h-4 w-4" />
          Generate Report
        </Button>
      </div>

      {/* Stats */}
      <div className="grid gap-4 md:grid-cols-5">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Reports</CardTitle>
            <FileText className="h-4 w-4 text-gray-600 dark:text-gray-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.total}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Draft</CardTitle>
            <Clock className="h-4 w-4 text-gray-600 dark:text-gray-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-gray-600 dark:text-gray-400">
              {stats.draft}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Generated</CardTitle>
            <FileText className="h-4 w-4 text-blue-600 dark:text-blue-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {stats.generated}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Approved</CardTitle>
            <CheckCircle2 className="h-4 w-4 text-green-600 dark:text-green-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">
              {stats.approved}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Delivered</CardTitle>
            <Send className="h-4 w-4 text-purple-600 dark:text-purple-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-purple-600 dark:text-purple-400">
              {stats.delivered}
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
                placeholder="Search by report number, patient, or sample..."
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
              <option value="DRAFT">Draft</option>
              <option value="GENERATED">Generated</option>
              <option value="APPROVED">Approved</option>
              <option value="DELIVERED">Delivered</option>
            </select>
          </div>
        </CardContent>
      </Card>

      {/* Reports Grid */}
      <div className="grid gap-4 md:grid-cols-2">
        {filteredReports.map((report) => (
          <ReportCard key={report.id} report={report} />
        ))}
      </div>

      {filteredReports.length === 0 && (
        <Card>
          <CardContent className="flex h-64 flex-col items-center justify-center">
            <FileText className="h-12 w-12 text-gray-400" />
            <p className="mt-4 text-gray-600 dark:text-gray-400">
              No reports found matching your filters
            </p>
          </CardContent>
        </Card>
      )}

      {/* Quick Actions */}
      <Card>
        <CardHeader>
          <CardTitle>Quick Actions</CardTitle>
          <CardDescription>Common report operations</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid gap-3 md:grid-cols-3">
            <Button variant="outline" className="justify-start">
              <FileText className="mr-2 h-4 w-4" />
              Generate Batch Reports
            </Button>
            <Button variant="outline" className="justify-start">
              <Download className="mr-2 h-4 w-4" />
              Bulk Download
            </Button>
            <Button variant="outline" className="justify-start">
              <Mail className="mr-2 h-4 w-4" />
              Bulk Email Delivery
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
