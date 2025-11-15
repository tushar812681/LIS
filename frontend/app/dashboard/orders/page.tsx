'use client';

import { useState, useMemo } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { ClipboardCheck, Search, Plus, Calendar, User, TestTube2, Clock, Loader2, AlertCircle } from 'lucide-react';
import { useOrders } from '@/lib/hooks';
import { format } from 'date-fns';
import Link from 'next/link';

export const dynamic = 'force-dynamic';

const statusColors: Record<string, string> = {
  PENDING: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300',
  COLLECTED: 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300',
  PROCESSING: 'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-300',
  IN_PROGRESS: 'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-300',
  COMPLETED: 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300',
  CANCELLED: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300',
};

const priorityColors: Record<string, string> = {
  ROUTINE: 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300',
  URGENT: 'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-300',
  STAT: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300',
};

export default function OrdersPage() {
  const [searchQuery, setSearchQuery] = useState('');
  const [page, setPage] = useState(1);
  const limit = 10;

  // Fetch orders from GraphQL API
  const { orders, pagination, loading, error, refetch } = useOrders({
    page,
    limit,
    search: searchQuery || undefined,
    sort: { orderDate: 'DESC' },
  });

  // Calculate stats from real data
  const stats = useMemo(() => ({
    pending: orders.filter((o: any) => o.status === 'PENDING').length,
    processing: orders.filter((o: any) => o.status === 'IN_PROGRESS' || o.status === 'PROCESSING').length,
    completed: orders.filter((o: any) => o.status === 'COMPLETED').length,
    urgent: orders.filter((o: any) => o.priority === 'URGENT' || o.priority === 'STAT').length,
  }), [orders]);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Orders</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Manage test orders and requisitions
          </p>
        </div>
        <Link href="/dashboard/orders/create">
          <Button>
            <Plus className="mr-2 h-4 w-4" />
            New Order
          </Button>
        </Link>
      </div>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Pending Orders</CardDescription>
            <CardTitle className="text-3xl">
              {loading ? <Loader2 className="h-8 w-8 animate-spin" /> : stats.pending}
            </CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Processing</CardDescription>
            <CardTitle className="text-3xl">
              {loading ? <Loader2 className="h-8 w-8 animate-spin" /> : stats.processing}
            </CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Completed</CardDescription>
            <CardTitle className="text-3xl">
              {loading ? <Loader2 className="h-8 w-8 animate-spin" /> : stats.completed}
            </CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Urgent Orders</CardDescription>
            <CardTitle className="text-3xl text-orange-600">
              {loading ? <Loader2 className="h-8 w-8 animate-spin" /> : stats.urgent}
            </CardTitle>
          </CardHeader>
        </Card>
      </div>

      {/* Error State */}
      {error && (
        <Card className="border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950/50">
          <CardContent className="flex items-center gap-2 pt-6 text-red-800 dark:text-red-300">
            <AlertCircle className="h-5 w-5" />
            <p>Failed to load orders: {error.message}</p>
            <Button onClick={() => refetch()} variant="outline" size="sm" className="ml-auto">
              Retry
            </Button>
          </CardContent>
        </Card>
      )}

      {/* Search */}
      <Card>
        <CardContent className="pt-6">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
            <Input
              placeholder="Search by order number, patient name, or MR number..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10"
            />
          </div>
        </CardContent>
      </Card>

      {/* Loading State */}
      {loading && orders.length === 0 && (
        <div className="flex items-center justify-center py-12">
          <Loader2 className="h-8 w-8 animate-spin text-gray-400" />
          <p className="ml-2 text-gray-600 dark:text-gray-400">Loading orders...</p>
        </div>
      )}

      {/* Orders List */}
      {!loading && orders.length > 0 && (
        <div className="space-y-4">
          {orders.map((order: any) => (
            <Card key={order.id} className="hover:shadow-lg transition-shadow">
              <CardHeader>
                <div className="flex items-start justify-between">
                  <div className="flex items-center gap-3">
                    <div className="flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                      <ClipboardCheck className="h-6 w-6 text-blue-600 dark:text-blue-300" />
                    </div>
                    <div>
                      <CardTitle className="text-lg">{order.orderId}</CardTitle>
                      <CardDescription className="flex items-center gap-4 mt-1">
                        <span className="flex items-center gap-1">
                          <User className="h-3 w-3" />
                          {order.patient?.firstName} {order.patient?.lastName} ({order.patient?.patientId})
                        </span>
                      </CardDescription>
                    </div>
                  </div>
                  <div className="flex gap-2">
                    <span className={`px-2 py-1 text-xs font-medium rounded-full ${statusColors[order.status] || statusColors.PENDING}`}>
                      {order.status}
                    </span>
                    <span className={`px-2 py-1 text-xs font-medium rounded-full ${priorityColors[order.priority] || priorityColors.ROUTINE}`}>
                      {order.priority}
                    </span>
                  </div>
                </div>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-4 gap-4 text-sm">
                  <div>
                    <p className="text-gray-500 dark:text-gray-400 flex items-center gap-1">
                      <TestTube2 className="h-4 w-4" />
                      Tests
                    </p>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {order.completedTests || 0} / {order.totalTests || 0}
                    </p>
                  </div>
                  <div>
                    <p className="text-gray-500 dark:text-gray-400 flex items-center gap-1">
                      <User className="h-4 w-4" />
                      Ordered By
                    </p>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {order.doctor?.name || 'N/A'}
                    </p>
                  </div>
                  <div>
                    <p className="text-gray-500 dark:text-gray-400 flex items-center gap-1">
                      <Calendar className="h-4 w-4" />
                      Ordered
                    </p>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {order.orderDate ? format(new Date(order.orderDate), 'PPp') : 'N/A'}
                    </p>
                  </div>
                  <div>
                    <p className="text-gray-500 dark:text-gray-400 flex items-center gap-1">
                      <Clock className="h-4 w-4" />
                      Status
                    </p>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {order.status}
                    </p>
                  </div>
                </div>
                <div className="flex gap-2 mt-4">
                  <Link href={`/dashboard/orders/${order.id}`}>
                    <Button variant="outline" size="sm">
                      View Details
                    </Button>
                  </Link>
                  <Button size="sm">
                    Process Order
                  </Button>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}

      {/* Pagination */}
      {pagination && pagination.totalPages > 1 && (
        <div className="flex items-center justify-between">
          <p className="text-sm text-gray-600 dark:text-gray-400">
            Showing {((page - 1) * limit) + 1} to {Math.min(page * limit, pagination.total)} of {pagination.total} orders
          </p>
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setPage(p => Math.max(1, p - 1))}
              disabled={page === 1}
            >
              Previous
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setPage(p => Math.min(pagination.totalPages, p + 1))}
              disabled={page === pagination.totalPages}
            >
              Next
            </Button>
          </div>
        </div>
      )}

      {/* Empty State */}
      {!loading && orders.length === 0 && (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <ClipboardCheck className="h-12 w-12 text-gray-400" />
            <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
              No orders found
            </h3>
            <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
              {searchQuery
                ? 'Try adjusting your search criteria'
                : 'No orders have been placed yet'}
            </p>
            {!searchQuery && (
              <Link href="/dashboard/orders/create">
                <Button className="mt-4">
                  <Plus className="mr-2 h-4 w-4" />
                  Create Your First Order
                </Button>
              </Link>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
