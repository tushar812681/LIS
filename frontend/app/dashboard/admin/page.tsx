'use client';

import { useMemo } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Users, TestTube2, FileText, TrendingUp, TrendingDown, DollarSign, Clock, AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';
import {
  LineChart,
  Line,
  AreaChart,
  Area,
  BarChart,
  Bar,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import { useDashboardStats, useOrders, useSamples } from '@/lib/hooks';
import { format } from 'date-fns';
import Link from 'next/link';

export const dynamic = 'force-dynamic';

interface StatCardProps {
  title: string;
  value: string | number;
  description: string;
  icon: React.ComponentType<{ className?: string }>;
  trend?: {
    value: number;
    isPositive: boolean;
  };
}

function StatCard({ title, value, description, icon: Icon, trend }: StatCardProps) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">{title}</CardTitle>
        <Icon className="h-4 w-4 text-gray-600 dark:text-gray-400" />
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">{value}</div>
        <div className="flex items-center gap-2 text-xs text-gray-600 dark:text-gray-400">
          <p>{description}</p>
          {trend && (
            <span
              className={`flex items-center gap-1 ${
                trend.isPositive ? 'text-green-600' : 'text-red-600'
              }`}
            >
              {trend.isPositive ? (
                <TrendingUp className="h-3 w-3" />
              ) : (
                <TrendingDown className="h-3 w-3" />
              )}
              {trend.value}%
            </span>
          )}
        </div>
      </CardContent>
    </Card>
  );
}

// Sample trend data (last 7 days)
const sampleTrendData = [
  { date: 'Mon', samples: 45, orders: 52 },
  { date: 'Tue', samples: 52, orders: 58 },
  { date: 'Wed', samples: 48, orders: 55 },
  { date: 'Thu', samples: 61, orders: 67 },
  { date: 'Fri', samples: 55, orders: 62 },
  { date: 'Sat', samples: 38, orders: 42 },
  { date: 'Sun', samples: 28, orders: 33 },
];

// Revenue trend data
const revenueTrendData = [
  { date: '1st', revenue: 45000 },
  { date: '5th', revenue: 52000 },
  { date: '10th', revenue: 48000 },
  { date: '15th', revenue: 61000 },
  { date: '20th', revenue: 55000 },
  { date: '25th', revenue: 68000 },
  { date: '30th', revenue: 72000 },
];

// Test type distribution
const testTypeData = [
  { name: 'CBC', count: 234, color: '#3b82f6' },
  { name: 'Lipid Panel', count: 187, color: '#8b5cf6' },
  { name: 'Glucose', count: 156, color: '#10b981' },
  { name: 'Liver Function', count: 134, color: '#f59e0b' },
  { name: 'Kidney Function', count: 98, color: '#ef4444' },
  { name: 'Others', count: 145, color: '#6b7280' },
];

// Sample status distribution
const sampleStatusData = [
  { name: 'Collected', value: 145, color: '#3b82f6' },
  { name: 'Processing', value: 89, color: '#f59e0b' },
  { name: 'Completed', value: 234, color: '#10b981' },
  { name: 'Rejected', value: 12, color: '#ef4444' },
];

// Turnaround time data
const turnaroundTimeData = [
  { department: 'Hematology', avgTime: 2.3, target: 4 },
  { department: 'Chemistry', avgTime: 3.1, target: 4 },
  { department: 'Microbiology', avgTime: 5.8, target: 6 },
  { department: 'Immunology', avgTime: 4.2, target: 5 },
  { department: 'Pathology', avgTime: 6.5, target: 8 },
];

export default function AdminDashboard() {
  // Fetch real dashboard data from GraphQL API
  const { stats, loading: statsLoading, error: statsError, refetch: refetchStats } = useDashboardStats();

  // Fetch recent orders (limit 5) for pending orders section
  const { orders, loading: ordersLoading } = useOrders({
    page: 1,
    limit: 5,
    sort: { createdAt: 'DESC' },
    filters: { status: 'PENDING' },
  });

  // Fetch recent samples (limit 5) for recent samples section
  const { samples, loading: samplesLoading } = useSamples({
    page: 1,
    limit: 5,
    sort: { collectionDate: 'DESC' },
  });

  // Calculate trends from API data
  const trends = useMemo(() => {
    if (!stats?.trends) return null;
    return {
      samples: {
        value: stats.trends.orders || 0,
        isPositive: (stats.trends.orders || 0) >= 0,
      },
      reports: {
        value: Math.abs(stats.trends.orders || 0) * 0.8, // Mock calculation
        isPositive: false,
      },
      revenue: {
        value: stats.trends.revenue || 0,
        isPositive: (stats.trends.revenue || 0) >= 0,
      },
    };
  }, [stats]);

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Admin Dashboard
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Welcome back! Here&apos;s your laboratory overview.
        </p>
      </div>

      {/* Error State */}
      {statsError && (
        <Card className="border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950/50">
          <CardContent className="flex items-center gap-2 pt-6 text-red-800 dark:text-red-300">
            <AlertCircle className="h-5 w-5" />
            <p>Failed to load dashboard stats: {statsError.message}</p>
            <Button onClick={() => refetchStats()} variant="outline" size="sm" className="ml-auto">
              Retry
            </Button>
          </CardContent>
        </Card>
      )}

      {/* Loading State for Stats */}
      {statsLoading && !stats && (
        <div className="flex items-center justify-center py-12">
          <Loader2 className="h-8 w-8 animate-spin text-gray-400" />
          <p className="ml-2 text-gray-600 dark:text-gray-400">Loading dashboard...</p>
        </div>
      )}

      {/* Stats Grid */}
      {!statsLoading && stats && (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          <StatCard
            title="Total Patients"
            value={stats.totalPatients?.toLocaleString() || '0'}
            description="Registered in system"
            icon={Users}
          />
          <StatCard
            title="Active Samples"
            value={stats.pendingSamples || 0}
            description="In processing"
            icon={TestTube2}
            trend={trends?.samples}
          />
          <StatCard
            title="Pending Results"
            value={stats.pendingResults || 0}
            description="Awaiting approval"
            icon={FileText}
            trend={trends?.reports}
          />
          <StatCard
            title="Today's Revenue"
            value={`₹${((stats.todayRevenue || 0) / 1000).toFixed(1)}K`}
            description="Total earnings"
            icon={DollarSign}
            trend={trends?.revenue}
          />
        </div>
      )}

      {/* Charts Row 1 - Sample & Order Trends + Revenue Trend */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Sample & Order Trends */}
        <Card>
          <CardHeader>
            <CardTitle>Sample & Order Trends</CardTitle>
            <CardDescription>Daily volume over the past week</CardDescription>
          </CardHeader>
          <CardContent>
            <ResponsiveContainer width="100%" height={300}>
              <LineChart data={sampleTrendData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                <XAxis dataKey="date" stroke="#6b7280" />
                <YAxis stroke="#6b7280" />
                <Tooltip
                  contentStyle={{
                    backgroundColor: '#fff',
                    border: '1px solid #e5e7eb',
                    borderRadius: '8px'
                  }}
                />
                <Legend />
                <Line
                  type="monotone"
                  dataKey="samples"
                  stroke="#3b82f6"
                  strokeWidth={2}
                  name="Samples"
                  dot={{ fill: '#3b82f6', r: 4 }}
                />
                <Line
                  type="monotone"
                  dataKey="orders"
                  stroke="#8b5cf6"
                  strokeWidth={2}
                  name="Orders"
                  dot={{ fill: '#8b5cf6', r: 4 }}
                />
              </LineChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>

        {/* Revenue Trend */}
        <Card>
          <CardHeader>
            <CardTitle>Revenue Trend</CardTitle>
            <CardDescription>Monthly revenue progression</CardDescription>
          </CardHeader>
          <CardContent>
            <ResponsiveContainer width="100%" height={300}>
              <AreaChart data={revenueTrendData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                <XAxis dataKey="date" stroke="#6b7280" />
                <YAxis stroke="#6b7280" tickFormatter={(value) => `₹${(value / 1000).toFixed(0)}K`} />
                <Tooltip
                  contentStyle={{
                    backgroundColor: '#fff',
                    border: '1px solid #e5e7eb',
                    borderRadius: '8px'
                  }}
                  formatter={(value: number) => [`₹${value.toLocaleString()}`, 'Revenue']}
                />
                <Area
                  type="monotone"
                  dataKey="revenue"
                  stroke="#10b981"
                  fill="#10b981"
                  fillOpacity={0.2}
                  strokeWidth={2}
                />
              </AreaChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>
      </div>

      {/* Charts Row 2 - Test Type Distribution + Sample Status */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Test Type Distribution */}
        <Card>
          <CardHeader>
            <CardTitle>Test Type Distribution</CardTitle>
            <CardDescription>Most requested tests this month</CardDescription>
          </CardHeader>
          <CardContent>
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={testTypeData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                <XAxis dataKey="name" stroke="#6b7280" />
                <YAxis stroke="#6b7280" />
                <Tooltip
                  contentStyle={{
                    backgroundColor: '#fff',
                    border: '1px solid #e5e7eb',
                    borderRadius: '8px'
                  }}
                />
                <Bar dataKey="count" radius={[8, 8, 0, 0]}>
                  {testTypeData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>

        {/* Sample Status Distribution */}
        <Card>
          <CardHeader>
            <CardTitle>Sample Status Distribution</CardTitle>
            <CardDescription>Current sample processing status</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex items-center justify-between">
              <ResponsiveContainer width="50%" height={250}>
                <PieChart>
                  <Pie
                    data={sampleStatusData}
                    cx="50%"
                    cy="50%"
                    innerRadius={60}
                    outerRadius={90}
                    paddingAngle={2}
                    dataKey="value"
                  >
                    {sampleStatusData.map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={entry.color} />
                    ))}
                  </Pie>
                  <Tooltip />
                </PieChart>
              </ResponsiveContainer>
              <div className="flex flex-col gap-3">
                {sampleStatusData.map((item, index) => (
                  <div key={index} className="flex items-center gap-2">
                    <div
                      className="h-3 w-3 rounded-full"
                      style={{ backgroundColor: item.color }}
                    />
                    <div>
                      <p className="text-sm font-medium text-gray-900 dark:text-white">
                        {item.name}
                      </p>
                      <p className="text-xs text-gray-600 dark:text-gray-400">
                        {item.value} samples
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Turnaround Time Chart */}
      <Card>
        <CardHeader>
          <CardTitle>Department Turnaround Time</CardTitle>
          <CardDescription>Average TAT vs Target (in hours)</CardDescription>
        </CardHeader>
        <CardContent>
          <ResponsiveContainer width="100%" height={300}>
            <BarChart data={turnaroundTimeData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
              <XAxis dataKey="department" stroke="#6b7280" />
              <YAxis stroke="#6b7280" label={{ value: 'Hours', angle: -90, position: 'insideLeft' }} />
              <Tooltip
                contentStyle={{
                  backgroundColor: '#fff',
                  border: '1px solid #e5e7eb',
                  borderRadius: '8px'
                }}
              />
              <Legend />
              <Bar dataKey="avgTime" fill="#3b82f6" name="Avg Time" radius={[8, 8, 0, 0]} />
              <Bar dataKey="target" fill="#e5e7eb" name="Target" radius={[8, 8, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </CardContent>
      </Card>

      {/* Two Column Layout - Recent Activity */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Recent Samples */}
        <Card>
          <CardHeader>
            <CardTitle>Recent Samples</CardTitle>
            <CardDescription>Latest samples collected</CardDescription>
          </CardHeader>
          <CardContent>
            {samplesLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-6 w-6 animate-spin text-gray-400" />
              </div>
            ) : samples.length === 0 ? (
              <div className="py-8 text-center text-sm text-gray-500">
                No recent samples
              </div>
            ) : (
              <div className="space-y-4">
                {samples.map((sample: any) => {
                  const statusIcon = sample.status === 'COMPLETED' ? CheckCircle2 :
                                    sample.status === 'REJECTED' ? AlertCircle : Clock;
                  const statusColor = sample.status === 'COMPLETED' ? 'green' :
                                     sample.status === 'REJECTED' ? 'red' : 'yellow';
                  const StatusIcon = statusIcon;

                  return (
                    <Link
                      key={sample.id}
                      href={`/dashboard/samples/${sample.id}`}
                      className="flex items-center justify-between border-b border-gray-200 pb-3 last:border-0 last:pb-0 dark:border-gray-800 transition-colors hover:bg-gray-50 dark:hover:bg-gray-800/50 -mx-2 px-2 py-1 rounded"
                    >
                      <div className="flex items-center gap-3">
                        <div className="flex h-10 w-10 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-950">
                          <TestTube2 className="h-5 w-5 text-blue-600 dark:text-blue-400" />
                        </div>
                        <div>
                          <p className="font-medium text-gray-900 dark:text-white">
                            {sample.sampleId}
                          </p>
                          <p className="text-sm text-gray-600 dark:text-gray-400">
                            {sample.patient ? `${sample.patient.firstName} ${sample.patient.lastName}` : 'Unknown Patient'}
                          </p>
                        </div>
                      </div>
                      <div className="flex items-center gap-2">
                        <StatusIcon className={`h-4 w-4 text-${statusColor}-600`} />
                        <span className={`text-xs font-medium text-${statusColor}-600`}>
                          {sample.status}
                        </span>
                      </div>
                    </Link>
                  );
                })}
              </div>
            )}
          </CardContent>
        </Card>

        {/* Pending Orders */}
        <Card>
          <CardHeader>
            <CardTitle>Pending Orders</CardTitle>
            <CardDescription>Orders awaiting sample collection</CardDescription>
          </CardHeader>
          <CardContent>
            {ordersLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-6 w-6 animate-spin text-gray-400" />
              </div>
            ) : orders.length === 0 ? (
              <div className="py-8 text-center text-sm text-gray-500">
                No pending orders
              </div>
            ) : (
              <div className="space-y-4">
                {orders.map((order: any) => (
                  <Link
                    key={order.id}
                    href={`/dashboard/orders/${order.id}`}
                    className="flex items-center justify-between border-b border-gray-200 pb-3 last:border-0 last:pb-0 dark:border-gray-800 transition-colors hover:bg-gray-50 dark:hover:bg-gray-800/50 -mx-2 px-2 py-1 rounded"
                  >
                    <div className="flex items-center gap-3">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-purple-100 dark:bg-purple-950">
                        <FileText className="h-5 w-5 text-purple-600 dark:text-purple-400" />
                      </div>
                      <div>
                        <p className="font-medium text-gray-900 dark:text-white">
                          {order.orderId}
                        </p>
                        <p className="text-sm text-gray-600 dark:text-gray-400">
                          {order.patient ? `${order.patient.firstName} ${order.patient.lastName}` : 'Unknown Patient'}
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      <Clock className="h-4 w-4 text-gray-400" />
                      <span className="text-sm text-gray-600 dark:text-gray-400">
                        {order.createdAt ? format(new Date(order.createdAt), 'p') : 'N/A'}
                      </span>
                    </div>
                  </Link>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
