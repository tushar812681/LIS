'use client';

import { useState, useRef } from 'react';
import { useParams } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { useNotificationStore } from '@/lib/store';
import { useReport, useDeliverReport } from '@/lib/hooks';
import { SkeletonCard } from '@/components/ui/skeleton';
import {
  Printer,
  Download,
  Mail,
  Send,
  ArrowLeft,
  CheckCircle2,

  Loader2
} from 'lucide-react';
import Link from 'next/link';

export const dynamic = 'force-dynamic';

interface Report {
  id: string;
  reportNumber: string;
  createdAt: string;
  approvedAt?: string;
  deliveredAt?: string;
  deliveryMethod?: string;
  comments?: string;
  verifiedAt?: string;
  order: {
    orderNumber: string;
    createdAt: string;
    referringPhysician?: string;
    patient: {
      firstName: string;
      lastName: string;
      mrn: string;
      dateOfBirth: string;
      age: number;
      gender: string;
      email: string;
    };
  };
  organization?: {
    name: string;
    address: string;
    phone: string;
    email: string;
    licenseNumber?: string;
  };
  results: Array<{
    test: {
      name: string;
      referenceRangeMin?: number;
      referenceRangeMax?: number;
    };
    resultValue: string;
    resultUnit: string;
  }>;
  verifiedBy?: {
    firstName: string;
    lastName: string;
    title?: string;
  };
  approvedBy?: {
    firstName: string;
    lastName: string;
    title?: string;
  };
}

export default function ReportPreviewPage() {
  const params = useParams();
  const reportId = params.id as string;
  const [isPrinting, setIsPrinting] = useState(false);
  const printRef = useRef<HTMLDivElement>(null);
  const addNotification = useNotificationStore((state) => state.addNotification);

  // GraphQL hooks
  const { report, loading, error, refetch } = useReport(reportId);
  const { deliverReport, loading: delivering } = useDeliverReport();

  if (loading) {
    return (
      <div className="mx-auto max-w-5xl space-y-6">
        <SkeletonCard />
        <SkeletonCard />
        <SkeletonCard />
      </div>
    );
  }

  if (error || !report) {
    return (
      <div className="mx-auto max-w-5xl">
        <Card>
          <CardContent className="p-6">
            <p className="text-red-600">
              Error loading report: {error?.message || 'Report not found'}
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  // Type assertion after null check
  const typedReport = report as Report;

  const handlePrint = () => {
    setIsPrinting(true);
    setTimeout(() => {
      window.print();
      setIsPrinting(false);
    }, 100);
  };

  const handleDownloadPDF = async () => {
    try {
      // In production, this would call the backend PDF generation endpoint
      addNotification({
        type: 'info',
        title: 'Generating PDF',
        message: 'Your PDF report is being generated...',
      });

      // Simulate PDF download
      setTimeout(() => {
        addNotification({
          type: 'success',
          title: 'PDF Downloaded',
          message: 'Report has been downloaded successfully',
        });
      }, 2000);
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to download PDF',
      });
    }
  };

  const handleEmailReport = async () => {
    try {
      await deliverReport({
        variables: {
          reportId,
          input: {
            deliveryMethod: 'EMAIL',
            recipientEmail: typedReport.order.patient.email,
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Report Sent',
        message: `Report has been sent to ${typedReport.order.patient.email}`,
      });

      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to send report',
      });
    }
  };

  const handleDeliverToPortal = async () => {
    try {
      await deliverReport({
        variables: {
          reportId,
          input: {
            deliveryMethod: 'PORTAL',
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Report Delivered',
        message: 'Report has been delivered to patient portal',
      });

      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to deliver report',
      });
    }
  };

  return (
    <div className="mx-auto max-w-5xl space-y-6">
      {/* Header - Hidden when printing */}
      <div className="no-print flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Link href="/dashboard/reports">
            <Button variant="outline" size="icon">
              <ArrowLeft className="h-4 w-4" />
            </Button>
          </Link>
          <div>
            <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
              Report Preview
            </h1>
            <p className="text-gray-600 dark:text-gray-400">
              Order: {typedReport.order.orderNumber}
            </p>
          </div>
        </div>

        {/* Action Buttons */}
        <div className="flex gap-2">
          <Button variant="outline" onClick={handlePrint} disabled={isPrinting}>
            <Printer className="mr-2 h-4 w-4" />
            Print
          </Button>
          <Button variant="outline" onClick={handleDownloadPDF}>
            <Download className="mr-2 h-4 w-4" />
            Download PDF
          </Button>
          <Button variant="outline" onClick={handleEmailReport} disabled={delivering}>
            {delivering ? (
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            ) : (
              <Mail className="mr-2 h-4 w-4" />
            )}
            Email
          </Button>
          <Button onClick={handleDeliverToPortal} disabled={delivering}>
            {delivering ? (
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            ) : (
              <Send className="mr-2 h-4 w-4" />
            )}
            Deliver to Portal
          </Button>
        </div>
      </div>

      {/* Delivery Status */}
      {typedReport.deliveredAt && (
        <div className="no-print">
          <Card>
            <CardContent className="flex items-center gap-3 p-4">
              <CheckCircle2 className="h-5 w-5 text-green-600" />
              <div>
                <p className="font-medium text-green-900 dark:text-green-300">
                  Report Delivered
                </p>
                <p className="text-sm text-green-700 dark:text-green-400">
                  Delivered on {new Date(typedReport.deliveredAt).toLocaleString()} via{' '}
                  {typedReport.deliveryMethod}
                </p>
              </div>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Report Content - Printable */}
      <div ref={printRef} className="print-content">
        <Card className="overflow-hidden">
          <CardContent className="p-0">
            {/* Report Header */}
            <div className="border-b-2 border-gray-300 bg-white p-8 dark:bg-gray-900">
              <div className="flex items-start justify-between">
                <div>
                  <h2 className="mb-2 text-2xl font-bold text-gray-900 dark:text-white">
                    {typedReport.organization?.name || 'Laboratory Report'}
                  </h2>
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    {typedReport.organization?.address}
                  </p>
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    Phone: {typedReport.organization?.phone} | Email: {typedReport.organization?.email}
                  </p>
                  {typedReport.organization?.licenseNumber && (
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      License: {typedReport.organization?.licenseNumber}
                    </p>
                  )}
                </div>
                <div className="text-right">
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    Report ID: {typedReport.reportNumber}
                  </p>
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    Generated: {new Date(typedReport.createdAt).toLocaleString()}
                  </p>
                  {typedReport.approvedAt && (
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      Approved: {new Date(typedReport.approvedAt).toLocaleString()}
                    </p>
                  )}
                </div>
              </div>
            </div>

            {/* Patient Information */}
            <div className="border-b border-gray-200 bg-gray-50 p-6 dark:border-gray-700 dark:bg-gray-800">
              <h3 className="mb-4 font-semibold text-gray-900 dark:text-white">
                Patient Information
              </h3>
              <div className="grid gap-4 md:grid-cols-2">
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Name</p>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {typedReport.order.patient.firstName} {typedReport.order.patient.lastName}
                  </p>
                </div>
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">MRN</p>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {typedReport.order.patient.mrn}
                  </p>
                </div>
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Date of Birth</p>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {new Date(typedReport.order.patient.dateOfBirth).toLocaleDateString()}
                  </p>
                </div>
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Age / Gender</p>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {typedReport.order.patient.age} years / {typedReport.order.patient.gender}
                  </p>
                </div>
              </div>
            </div>

            {/* Order Information */}
            <div className="border-b border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
              <h3 className="mb-4 font-semibold text-gray-900 dark:text-white">
                Order Information
              </h3>
              <div className="grid gap-4 md:grid-cols-3">
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Order Number</p>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {typedReport.order.orderNumber}
                  </p>
                </div>
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Ordered Date</p>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {new Date(typedReport.order.createdAt).toLocaleDateString()}
                  </p>
                </div>
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Referring Physician</p>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {typedReport.order.referringPhysician || 'N/A'}
                  </p>
                </div>
              </div>
            </div>

            {/* Test Results */}
            <div className="p-6">
              <h3 className="mb-4 font-semibold text-gray-900 dark:text-white">
                Laboratory Results
              </h3>
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead>
                    <tr className="border-b-2 border-gray-300">
                      <th className="pb-2 text-left text-sm font-semibold text-gray-900 dark:text-white">
                        Test Name
                      </th>
                      <th className="pb-2 text-left text-sm font-semibold text-gray-900 dark:text-white">
                        Result
                      </th>
                      <th className="pb-2 text-left text-sm font-semibold text-gray-900 dark:text-white">
                        Unit
                      </th>
                      <th className="pb-2 text-left text-sm font-semibold text-gray-900 dark:text-white">
                        Reference Range
                      </th>
                      <th className="pb-2 text-left text-sm font-semibold text-gray-900 dark:text-white">
                        Flag
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    {typedReport.results.map((result: { test: { name: string; referenceRangeMin?: number; referenceRangeMax?: number }; resultValue: string; resultUnit: string }, index: number) => {
                      const value = parseFloat(result.resultValue);
                      const min = result.test.referenceRangeMin;
                      const max = result.test.referenceRangeMax;
                      let flag = '';

                      if (value && min && max) {
                        if (value < min) flag = 'L';
                        else if (value > max) flag = 'H';
                      }

                      return (
                        <tr
                          key={index}
                          className="border-b border-gray-200 dark:border-gray-700"
                        >
                          <td className="py-3 text-sm text-gray-900 dark:text-white">
                            {result.test.name}
                          </td>
                          <td className="py-3 text-sm font-medium text-gray-900 dark:text-white">
                            {result.resultValue}
                          </td>
                          <td className="py-3 text-sm text-gray-600 dark:text-gray-400">
                            {result.resultUnit}
                          </td>
                          <td className="py-3 text-sm text-gray-600 dark:text-gray-400">
                            {min && max ? `${min} - ${max}` : 'N/A'}
                          </td>
                          <td className="py-3 text-sm">
                            {flag && (
                              <span
                                className={`font-bold ${
                                  flag === 'H'
                                    ? 'text-red-600'
                                    : 'text-blue-600'
                                }`}
                              >
                                {flag}
                              </span>
                            )}
                          </td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            </div>

            {/* Additional Comments */}
            {typedReport.comments && (
              <div className="border-t border-gray-200 bg-gray-50 p-6 dark:border-gray-700 dark:bg-gray-800">
                <h3 className="mb-2 font-semibold text-gray-900 dark:text-white">
                  Clinical Comments
                </h3>
                <p className="text-sm text-gray-700 dark:text-gray-300">
                  {typedReport.comments}
                </p>
              </div>
            )}

            {/* Signatures */}
            <div className="border-t border-gray-200 p-6 dark:border-gray-700">
              <div className="grid gap-6 md:grid-cols-2">
                {typedReport.verifiedBy && (
                  <div>
                    <p className="mb-1 text-sm text-gray-600 dark:text-gray-400">
                      Verified By
                    </p>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {typedReport.verifiedBy.firstName} {typedReport.verifiedBy.lastName}
                    </p>
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      {typedReport.verifiedBy.title || 'Medical Technologist'}
                    </p>
                    {typedReport.verifiedAt && (
                      <p className="text-sm text-gray-500">
                        {new Date(typedReport.verifiedAt).toLocaleString()}
                      </p>
                    )}
                  </div>
                )}
                {typedReport.approvedBy && (
                  <div>
                    <p className="mb-1 text-sm text-gray-600 dark:text-gray-400">
                      Approved By
                    </p>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {typedReport.approvedBy.firstName} {typedReport.approvedBy.lastName}
                    </p>
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      {typedReport.approvedBy.title || 'Pathologist'}
                    </p>
                    {typedReport.approvedAt && (
                      <p className="text-sm text-gray-500">
                        {new Date(typedReport.approvedAt).toLocaleString()}
                      </p>
                    )}
                  </div>
                )}
              </div>
            </div>

            {/* Footer */}
            <div className="border-t-2 border-gray-300 bg-gray-50 p-4 text-center dark:bg-gray-800">
              <p className="text-xs text-gray-600 dark:text-gray-400">
                This is a computer-generated report and does not require a physical signature.
              </p>
              <p className="text-xs text-gray-600 dark:text-gray-400">
                Page 1 of 1 | Printed on {new Date().toLocaleDateString()}
              </p>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Print Styles */}
      <style jsx global>{`
        @media print {
          body {
            background: white !important;
          }
          .no-print {
            display: none !important;
          }
          .print-content {
            box-shadow: none !important;
          }
          @page {
            margin: 0.5in;
          }
        }
      `}</style>
    </div>
  );
}
