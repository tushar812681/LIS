'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import {
  TestTube2,
  ClipboardCheck,
  Beaker,
  FileText,
  Users,
  Activity,
  TrendingUp,
  AlertCircle,
  CheckCircle2,
  Clock,
  Package,
  UserPlus,
  FileBarChart,
  Calendar,
} from 'lucide-react';
import Link from 'next/link';
import { useAuthStore } from '@/lib/store';

/**
 * Dashboard Overview Page - Main LIS Dashboard
 */
export default function DashboardPage() {
  const user = useAuthStore((state) => state.user);
  const userRole = user?.roles?.[0] || 'USER';

  // Mock data - in production, fetch from GraphQL
  const metrics = {
    pendingSamples: 24,
    pendingResults: 18,
    todayOrders: 42,
    criticalResults: 3,
    activePatients: 156,
    equipmentStatus: 12,
    lowInventory: 5,
    pendingQC: 7,
  };

  const quickActions = [
    {
      title: 'Register Patient',
      description: 'Add new patient to system',
      icon: UserPlus,
      href: '/dashboard/patients/new',
      color: 'blue',
      permission: 'CREATE_PATIENT',
    },
    {
      title: 'Create Order',
      description: 'Place new test order',
      icon: ClipboardCheck,
      href: '/dashboard/orders/new',
      color: 'green',
      permission: 'CREATE_ORDER',
    },
    {
      title: 'Enter Results',
      description: 'Input test results',
      icon: Beaker,
      href: '/dashboard/results',
      color: 'purple',
      permission: 'CREATE_RESULT',
    },
    {
      title: 'Generate Report',
      description: 'Create lab report',
      icon: FileBarChart,
      href: '/dashboard/reports',
      color: 'orange',
      permission: 'VIEW_REPORT',
    },
  ];

  const recentActivity = [
    { id: 1, type: 'sample', message: 'Sample #S-2024-001 received', time: '5 min ago' },
    { id: 2, type: 'result', message: 'Results entered for Order #O-2024-042', time: '12 min ago' },
    { id: 3, type: 'patient', message: 'New patient registered: John Doe', time: '23 min ago' },
    { id: 4, type: 'qc', message: 'QC check completed for Analyzer-01', time: '1 hour ago' },
    { id: 5, type: 'alert', message: 'Low inventory alert: Reagent XYZ', time: '2 hours ago' },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight text-gray-900 dark:text-white">
          Dashboard Overview
        </h1>
        <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
          Welcome back, {user?.name || 'User'}! Here's what's happening in your laboratory today.
        </p>
      </div>

      {/* Key Metrics */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card className="border-l-4 border-l-blue-500">
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Pending Samples</CardTitle>
            <TestTube2 className="h-4 w-4 text-blue-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{metrics.pendingSamples}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Awaiting processing
            </p>
          </CardContent>
        </Card>

        <Card className="border-l-4 border-l-yellow-500">
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Pending Results</CardTitle>
            <Clock className="h-4 w-4 text-yellow-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{metrics.pendingResults}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Pending entry/review
            </p>
          </CardContent>
        </Card>

        <Card className="border-l-4 border-l-green-500">
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Today's Orders</CardTitle>
            <ClipboardCheck className="h-4 w-4 text-green-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{metrics.todayOrders}</div>
            <p className="text-xs text-green-600 dark:text-green-400">
              <TrendingUp className="mr-1 inline h-3 w-3" />
              +12% from yesterday
            </p>
          </CardContent>
        </Card>

        <Card className="border-l-4 border-l-red-500">
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Critical Results</CardTitle>
            <AlertCircle className="h-4 w-4 text-red-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">{metrics.criticalResults}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Require immediate attention
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Additional Metrics Row */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Active Patients</CardTitle>
            <Users className="h-4 w-4 text-purple-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{metrics.activePatients}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">This month</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Equipment Status</CardTitle>
            <Activity className="h-4 w-4 text-green-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{metrics.equipmentStatus}/14</div>
            <p className="text-xs text-green-600 dark:text-green-400">
              <CheckCircle2 className="mr-1 inline h-3 w-3" />
              All operational
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Low Inventory</CardTitle>
            <Package className="h-4 w-4 text-orange-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-orange-600">{metrics.lowInventory}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Items need restock</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Pending QC</CardTitle>
            <CheckCircle2 className="h-4 w-4 text-blue-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{metrics.pendingQC}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Quality checks due</p>
          </CardContent>
        </Card>
      </div>

      {/* Quick Actions & Recent Activity */}
      <div className="grid gap-6 lg:grid-cols-3">
        {/* Quick Actions */}
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle>Quick Actions</CardTitle>
            <CardDescription>Frequently used tasks and workflows</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid gap-4 sm:grid-cols-2">
              {quickActions.map((action) => {
                const Icon = action.icon;
                return (
                  <Link key={action.href} href={action.href}>
                    <Card className="transition-all hover:shadow-md cursor-pointer border-l-4" style={{ borderLeftColor: `var(--${action.color}-500)` }}>
                      <CardContent className="flex items-start gap-4 p-4">
                        <div className={`rounded-lg bg-${action.color}-50 p-3 dark:bg-${action.color}-950`}>
                          <Icon className={`h-6 w-6 text-${action.color}-600 dark:text-${action.color}-400`} />
                        </div>
                        <div className="flex-1">
                          <h3 className="font-semibold text-gray-900 dark:text-white">
                            {action.title}
                          </h3>
                          <p className="text-sm text-gray-600 dark:text-gray-400">
                            {action.description}
                          </p>
                        </div>
                      </CardContent>
                    </Card>
                  </Link>
                );
              })}
            </div>
          </CardContent>
        </Card>

        {/* Recent Activity */}
        <Card>
          <CardHeader>
            <CardTitle>Recent Activity</CardTitle>
            <CardDescription>Latest system updates</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {recentActivity.map((activity) => (
                <div key={activity.id} className="flex items-start gap-3">
                  <div className="rounded-full bg-blue-50 p-2 dark:bg-blue-950">
                    <Activity className="h-4 w-4 text-blue-600 dark:text-blue-400" />
                  </div>
                  <div className="flex-1 space-y-1">
                    <p className="text-sm text-gray-900 dark:text-white">
                      {activity.message}
                    </p>
                    <p className="text-xs text-gray-600 dark:text-gray-400">
                      {activity.time}
                    </p>
                  </div>
                </div>
              ))}
            </div>
            <Button variant="outline" className="mt-4 w-full" asChild>
              <Link href="/dashboard/notifications">View All Activity</Link>
            </Button>
          </CardContent>
        </Card>
      </div>

      {/* Calendar & Scheduled Tasks */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Calendar className="h-5 w-5" />
            Today's Schedule
          </CardTitle>
          <CardDescription>Upcoming tasks and appointments</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            <div className="flex items-center justify-between rounded-lg border border-gray-200 p-3 dark:border-gray-800">
              <div className="flex items-center gap-3">
                <div className="rounded bg-blue-50 px-3 py-1 dark:bg-blue-950">
                  <p className="text-sm font-semibold text-blue-600 dark:text-blue-400">09:00</p>
                </div>
                <div>
                  <p className="font-medium text-gray-900 dark:text-white">Daily QC Check - Hematology</p>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Equipment calibration and verification</p>
                </div>
              </div>
              <CheckCircle2 className="h-5 w-5 text-green-600" />
            </div>

            <div className="flex items-center justify-between rounded-lg border border-gray-200 p-3 dark:border-gray-800">
              <div className="flex items-center gap-3">
                <div className="rounded bg-orange-50 px-3 py-1 dark:bg-orange-950">
                  <p className="text-sm font-semibold text-orange-600 dark:text-orange-400">14:00</p>
                </div>
                <div>
                  <p className="font-medium text-gray-900 dark:text-white">Equipment Maintenance - Analyzer 03</p>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Scheduled preventive maintenance</p>
                </div>
              </div>
              <Clock className="h-5 w-5 text-orange-600" />
            </div>

            <div className="flex items-center justify-between rounded-lg border border-gray-200 p-3 dark:border-gray-800">
              <div className="flex items-center gap-3">
                <div className="rounded bg-purple-50 px-3 py-1 dark:bg-purple-950">
                  <p className="text-sm font-semibold text-purple-600 dark:text-purple-400">16:30</p>
                </div>
                <div>
                  <p className="font-medium text-gray-900 dark:text-white">Inventory Review Meeting</p>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Monthly stock and supplies review</p>
                </div>
              </div>
              <Clock className="h-5 w-5 text-purple-600" />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
