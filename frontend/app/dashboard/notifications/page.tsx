'use client';

import { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Bell, CheckCircle2, AlertTriangle, Info, AlertCircle, Trash2, Check, Settings } from 'lucide-react';


interface NotificationItem {
  id: string;
  type: 'success' | 'error' | 'info' | 'warning';
  title: string;
  message: string;
  timestamp: string;
  read: boolean;
}

const iconMap = {
  success: CheckCircle2,
  error: AlertCircle,
  info: Info,
  warning: AlertTriangle,
};

const colorMap = {
  success: {
    bg: 'bg-green-100 dark:bg-green-950',
    icon: 'text-green-600 dark:text-green-400',
  },
  error: {
    bg: 'bg-red-100 dark:bg-red-950',
    icon: 'text-red-600 dark:text-red-400',
  },
  info: {
    bg: 'bg-blue-100 dark:bg-blue-950',
    icon: 'text-blue-600 dark:text-blue-400',
  },
  warning: {
    bg: 'bg-yellow-100 dark:bg-yellow-950',
    icon: 'text-yellow-600 dark:text-yellow-400',
  },
};

function NotificationCard({ notification }: { notification: NotificationItem }) {
  const Icon = iconMap[notification.type];
  const colors = colorMap[notification.type];

  return (
    <div
      className={`rounded-lg border p-4 transition-all hover:shadow-md ${
        notification.read
          ? 'border-gray-200 bg-white dark:border-gray-800 dark:bg-gray-900'
          : 'border-blue-200 bg-blue-50 dark:border-blue-900 dark:bg-blue-950/50'
      }`}
    >
      <div className="flex gap-3">
        <div className={`flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full ${colors.bg}`}>
          <Icon className={`h-5 w-5 ${colors.icon}`} />
        </div>
        <div className="flex-1">
          <div className="mb-1 flex items-start justify-between">
            <p className="font-medium text-gray-900 dark:text-white">{notification.title}</p>
            <span className="text-xs text-gray-500 dark:text-gray-500">{notification.timestamp}</span>
          </div>
          <p className="text-sm text-gray-600 dark:text-gray-400">{notification.message}</p>
        </div>
      </div>
    </div>
  );
}

export default function NotificationsPage() {
  const [filter, setFilter] = useState<'all' | 'unread'>('all');
  const [notificationPreferences, setNotificationPreferences] = useState({
    email: true,
    push: true,
    sms: false,
    sound: true,
  });

  // Mock notifications - in production, fetch from backend
  const [notifications] = useState<NotificationItem[]>([
    {
      id: '1',
      type: 'error',
      title: 'Critical Value Alert',
      message: 'Patient John Doe: Hemoglobin = 6.5 g/dL',
      timestamp: '2 minutes ago',
      read: false,
    },
    {
      id: '2',
      type: 'success',
      title: 'Result Verified',
      message: 'Sample SMP-2024-002 verified by Dr. Williams',
      timestamp: '15 minutes ago',
      read: false,
    },
    {
      id: '3',
      type: 'info',
      title: 'Sample Status Updated',
      message: 'Sample SMP-2024-003 is now Processing',
      timestamp: '1 hour ago',
      read: true,
    },
    {
      id: '4',
      type: 'warning',
      title: 'Equipment Maintenance Due',
      message: 'Hematology Analyzer: Calibration required',
      timestamp: '2 hours ago',
      read: true,
    },
    {
      id: '5',
      type: 'success',
      title: 'Report Generated',
      message: 'Report REP-2024-001 for Jane Smith is ready',
      timestamp: '3 hours ago',
      read: true,
    },
    {
      id: '6',
      type: 'warning',
      title: 'Low Stock Alert',
      message: 'EDTA Tubes: 15 units (min: 20)',
      timestamp: '5 hours ago',
      read: true,
    },
  ]);

  const filteredNotifications = filter === 'all'
    ? notifications
    : notifications.filter((n) => !n.read);

  const unreadCount = notifications.filter((n) => !n.read).length;

  const handleMarkAllRead = () => {
    // In production, call API to mark all as read
    console.log('Marking all as read');
  };

  const handleClearAll = () => {
    // In production, call API to clear notifications
    console.log('Clearing all notifications');
  };

  const handleTogglePreference = (key: keyof typeof notificationPreferences) => {
    setNotificationPreferences((prev) => ({
      ...prev,
      [key]: !prev[key],
    }));
    // In production, save to backend
  };

  return (
    <div className="mx-auto max-w-4xl space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Notifications
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            {unreadCount} unread notification{unreadCount !== 1 ? 's' : ''}
          </p>
        </div>
        <Button variant="outline">
          <Settings className="mr-2 h-4 w-4" />
          Settings
        </Button>
      </div>

      {/* Stats */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total</CardTitle>
            <Bell className="h-4 w-4 text-gray-600 dark:text-gray-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{notifications.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Unread</CardTitle>
            <AlertCircle className="h-4 w-4 text-blue-600 dark:text-blue-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {unreadCount}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Critical</CardTitle>
            <AlertCircle className="h-4 w-4 text-red-600 dark:text-red-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600 dark:text-red-400">
              {notifications.filter((n) => n.type === 'error').length}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Today</CardTitle>
            <Info className="h-4 w-4 text-green-600 dark:text-green-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">
              {notifications.length}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-between">
        <div className="flex gap-2">
          <Button
            variant={filter === 'all' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setFilter('all')}
          >
            All
          </Button>
          <Button
            variant={filter === 'unread' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setFilter('unread')}
          >
            Unread ({unreadCount})
          </Button>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={handleMarkAllRead}>
            <Check className="mr-2 h-4 w-4" />
            Mark All Read
          </Button>
          <Button variant="outline" size="sm" onClick={handleClearAll}>
            <Trash2 className="mr-2 h-4 w-4" />
            Clear All
          </Button>
        </div>
      </div>

      {/* Notifications List */}
      <Card>
        <CardHeader>
          <CardTitle>Recent Notifications</CardTitle>
          <CardDescription>Your latest system notifications</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {filteredNotifications.length > 0 ? (
              filteredNotifications.map((notification) => (
                <NotificationCard key={notification.id} notification={notification} />
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-12">
                <Bell className="h-12 w-12 text-gray-400" />
                <p className="mt-4 text-gray-600 dark:text-gray-400">
                  No {filter === 'unread' ? 'unread ' : ''}notifications
                </p>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Notification Preferences */}
      <Card>
        <CardHeader>
          <CardTitle>Notification Preferences</CardTitle>
          <CardDescription>Manage how you receive notifications</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-800">
              <div>
                <p className="font-medium text-gray-900 dark:text-white">Email Notifications</p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Receive notifications via email
                </p>
              </div>
              <button
                onClick={() => handleTogglePreference('email')}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  notificationPreferences.email ? 'bg-blue-600' : 'bg-gray-300 dark:bg-gray-700'
                }`}
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    notificationPreferences.email ? 'translate-x-6' : 'translate-x-1'
                  }`}
                />
              </button>
            </div>

            <div className="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-800">
              <div>
                <p className="font-medium text-gray-900 dark:text-white">Push Notifications</p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Receive browser push notifications
                </p>
              </div>
              <button
                onClick={() => handleTogglePreference('push')}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  notificationPreferences.push ? 'bg-blue-600' : 'bg-gray-300 dark:bg-gray-700'
                }`}
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    notificationPreferences.push ? 'translate-x-6' : 'translate-x-1'
                  }`}
                />
              </button>
            </div>

            <div className="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-800">
              <div>
                <p className="font-medium text-gray-900 dark:text-white">SMS Notifications</p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Receive critical alerts via SMS
                </p>
              </div>
              <button
                onClick={() => handleTogglePreference('sms')}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  notificationPreferences.sms ? 'bg-blue-600' : 'bg-gray-300 dark:bg-gray-700'
                }`}
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    notificationPreferences.sms ? 'translate-x-6' : 'translate-x-1'
                  }`}
                />
              </button>
            </div>

            <div className="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-800">
              <div>
                <p className="font-medium text-gray-900 dark:text-white">Sound Alerts</p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Play sound for critical notifications
                </p>
              </div>
              <button
                onClick={() => handleTogglePreference('sound')}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  notificationPreferences.sound ? 'bg-blue-600' : 'bg-gray-300 dark:bg-gray-700'
                }`}
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    notificationPreferences.sound ? 'translate-x-6' : 'translate-x-1'
                  }`}
                />
              </button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
