'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { FileText, Calendar, Download, Eye, CheckCircle2, Clock, AlertCircle } from 'lucide-react';
import { Button } from '@/components/ui/button';

interface ReportItemProps {
  testName: string;
  reportDate: string;
  doctor: string;
  status: 'ready' | 'processing';
  reportId: string;
}

function ReportItem({ testName, reportDate, doctor, status }: ReportItemProps) {
  return (
    <div className="flex items-center justify-between rounded-lg border border-gray-200 p-4 transition-all hover:border-blue-500 hover:shadow-md dark:border-gray-800 dark:hover:border-blue-500">
      <div className="flex items-center gap-4">
        <div className={`flex h-12 w-12 items-center justify-center rounded-full ${status === 'ready' ? 'bg-green-100 dark:bg-green-950' : 'bg-yellow-100 dark:bg-yellow-950'}`}>
          {status === 'ready' ? (
            <CheckCircle2 className="h-6 w-6 text-green-600 dark:text-green-400" />
          ) : (
            <Clock className="h-6 w-6 text-yellow-600 dark:text-yellow-400" />
          )}
        </div>
        <div>
          <p className="font-medium text-gray-900 dark:text-white">{testName}</p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            Ordered by: Dr. {doctor}
          </p>
          <p className="text-xs text-gray-500 dark:text-gray-500">
            {reportDate}
          </p>
        </div>
      </div>
      <div className="flex items-center gap-2">
        {status === 'ready' ? (
          <>
            <Button size="sm" variant="outline">
              <Eye className="mr-2 h-4 w-4" />
              View
            </Button>
            <Button size="sm">
              <Download className="mr-2 h-4 w-4" />
              Download
            </Button>
          </>
        ) : (
          <span className="rounded-full bg-yellow-100 px-3 py-1 text-xs font-medium text-yellow-800 dark:bg-yellow-950 dark:text-yellow-300">
            In Progress
          </span>
        )}
      </div>
    </div>
  );
}

export default function PatientDashboard() {
  const mockReports: ReportItemProps[] = [
    {
      testName: 'Complete Blood Count (CBC)',
      reportDate: 'Jan 15, 2025',
      doctor: 'Smith',
      status: 'ready',
      reportId: 'REP-001',
    },
    {
      testName: 'Lipid Profile',
      reportDate: 'Jan 14, 2025',
      doctor: 'Johnson',
      status: 'ready',
      reportId: 'REP-002',
    },
    {
      testName: 'HbA1c',
      reportDate: 'Today',
      doctor: 'Williams',
      status: 'processing',
      reportId: 'REP-003',
    },
  ];

  const patientInfo = {
    name: 'John Doe',
    mrn: 'PAT-10001',
    age: 35,
    bloodGroup: 'O+',
    totalReports: 12,
    pendingTests: 1,
  };

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Patient Dashboard
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          View your test reports and medical information
        </p>
      </div>

      {/* Patient Info Card */}
      <Card>
        <CardHeader>
          <CardTitle>My Information</CardTitle>
          <CardDescription>Your personal and medical details</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 md:grid-cols-4">
            <div className="flex flex-col gap-1">
              <p className="text-sm text-gray-600 dark:text-gray-400">Patient Name</p>
              <p className="font-medium text-gray-900 dark:text-white">{patientInfo.name}</p>
            </div>
            <div className="flex flex-col gap-1">
              <p className="text-sm text-gray-600 dark:text-gray-400">MRN Number</p>
              <p className="font-medium text-gray-900 dark:text-white">{patientInfo.mrn}</p>
            </div>
            <div className="flex flex-col gap-1">
              <p className="text-sm text-gray-600 dark:text-gray-400">Age</p>
              <p className="font-medium text-gray-900 dark:text-white">{patientInfo.age} years</p>
            </div>
            <div className="flex flex-col gap-1">
              <p className="text-sm text-gray-600 dark:text-gray-400">Blood Group</p>
              <p className="font-medium text-gray-900 dark:text-white">{patientInfo.bloodGroup}</p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Stats */}
      <div className="grid gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Reports</CardTitle>
            <FileText className="h-4 w-4 text-blue-600 dark:text-blue-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{patientInfo.totalReports}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Available for viewing
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pending Tests</CardTitle>
            <Clock className="h-4 w-4 text-yellow-600 dark:text-yellow-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
              {patientInfo.pendingTests}
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              In processing
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Next Appointment</CardTitle>
            <Calendar className="h-4 w-4 text-green-600 dark:text-green-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">
              Jan 20
            </div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Follow-up visit
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Recent Reports */}
      <Card>
        <CardHeader>
          <CardTitle>My Test Reports</CardTitle>
          <CardDescription>Your recent laboratory test results</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {mockReports.map((report) => (
              <ReportItem key={report.reportId} {...report} />
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Health Tips */}
      <Card>
        <CardHeader>
          <CardTitle>Health Tips</CardTitle>
          <CardDescription>Personalized recommendations based on your reports</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            <div className="flex gap-3 rounded-lg bg-blue-50 p-3 dark:bg-blue-950/50">
              <AlertCircle className="h-5 w-5 flex-shrink-0 text-blue-600 dark:text-blue-400" />
              <div>
                <p className="font-medium text-blue-900 dark:text-blue-300">
                  Stay Hydrated
                </p>
                <p className="text-sm text-blue-700 dark:text-blue-400">
                  Drink at least 8 glasses of water daily to maintain optimal health
                </p>
              </div>
            </div>
            <div className="flex gap-3 rounded-lg bg-green-50 p-3 dark:bg-green-950/50">
              <CheckCircle2 className="h-5 w-5 flex-shrink-0 text-green-600 dark:text-green-400" />
              <div>
                <p className="font-medium text-green-900 dark:text-green-300">
                  Good Blood Pressure
                </p>
                <p className="text-sm text-green-700 dark:text-green-400">
                  Your recent reports show healthy blood pressure levels. Keep it up!
                </p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Help & Support */}
      <Card>
        <CardHeader>
          <CardTitle>Need Help?</CardTitle>
          <CardDescription>Contact us for any queries</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-3 sm:flex-row">
            <Button variant="outline" className="flex-1">
              Contact Support
            </Button>
            <Button variant="outline" className="flex-1">
              Book Appointment
            </Button>
            <Button variant="outline" className="flex-1">
              View Test Catalog
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
