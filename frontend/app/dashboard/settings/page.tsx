'use client';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Save, User, Bell, Shield, Database, Palette, Globe } from 'lucide-react';

export const dynamic = 'force-dynamic';

export default function SettingsPage() {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Settings</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Manage your account and application preferences
          </p>
        </div>
        <Button>
          <Save className="mr-2 h-4 w-4" />
          Save Changes
        </Button>
      </div>

      {/* Settings Grid */}
      <div className="grid gap-6">
        {/* Profile Settings */}
        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <User className="h-5 w-5" />
              <CardTitle>Profile Settings</CardTitle>
            </div>
            <CardDescription>Manage your personal information</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Full Name
                </label>
                <Input placeholder="Your name" defaultValue="Admin User" />
              </div>
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Email
                </label>
                <Input type="email" placeholder="email@example.com" defaultValue="admin@citylab.com" />
              </div>
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Phone Number
                </label>
                <Input placeholder="+91 1234567890" defaultValue="+91 9876543210" />
              </div>
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Signature
                </label>
                <Input placeholder="Digital signature" />
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Security Settings */}
        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <Shield className="h-5 w-5" />
              <CardTitle>Security</CardTitle>
            </div>
            <CardDescription>Password and authentication settings</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Current Password
                </label>
                <Input type="password" placeholder="Enter current password" />
              </div>
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  New Password
                </label>
                <Input type="password" placeholder="Enter new password" />
              </div>
            </div>
            <div className="flex items-center gap-2">
              <input type="checkbox" className="h-4 w-4 rounded" />
              <label className="text-sm text-gray-700 dark:text-gray-300">
                Enable two-factor authentication
              </label>
            </div>
          </CardContent>
        </Card>

        {/* Notification Settings */}
        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <Bell className="h-5 w-5" />
              <CardTitle>Notifications</CardTitle>
            </div>
            <CardDescription>Configure notification preferences</CardDescription>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-900 dark:text-white">
                  Critical Value Alerts
                </p>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  Receive notifications for critical test results
                </p>
              </div>
              <input type="checkbox" className="h-4 w-4 rounded" defaultChecked />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-900 dark:text-white">
                  Sample Status Updates
                </p>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  Get notified about sample processing status
                </p>
              </div>
              <input type="checkbox" className="h-4 w-4 rounded" defaultChecked />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-900 dark:text-white">
                  Report Generation
                </p>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  Notifications when reports are ready
                </p>
              </div>
              <input type="checkbox" className="h-4 w-4 rounded" defaultChecked />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-900 dark:text-white">
                  System Alerts
                </p>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  Equipment and inventory alerts
                </p>
              </div>
              <input type="checkbox" className="h-4 w-4 rounded" defaultChecked />
            </div>
          </CardContent>
        </Card>

        {/* Additional Settings */}
        <div className="grid gap-6 md:grid-cols-3">
          <Card>
            <CardHeader>
              <div className="flex items-center gap-2">
                <Palette className="h-5 w-5" />
                <CardTitle>Appearance</CardTitle>
              </div>
              <CardDescription>Theme and display preferences</CardDescription>
            </CardHeader>
            <CardContent>
              <select className="w-full rounded-md border border-gray-300 p-2 dark:border-gray-700 dark:bg-gray-800">
                <option>System Default</option>
                <option>Light Mode</option>
                <option>Dark Mode</option>
              </select>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <div className="flex items-center gap-2">
                <Globe className="h-5 w-5" />
                <CardTitle>Language</CardTitle>
              </div>
              <CardDescription>Localization settings</CardDescription>
            </CardHeader>
            <CardContent>
              <select className="w-full rounded-md border border-gray-300 p-2 dark:border-gray-700 dark:bg-gray-800">
                <option>English</option>
                <option>Hindi</option>
                <option>Spanish</option>
              </select>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <div className="flex items-center gap-2">
                <Database className="h-5 w-5" />
                <CardTitle>Data & Privacy</CardTitle>
              </div>
              <CardDescription>Data management options</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                <Button variant="outline" size="sm" className="w-full">
                  Export Data
                </Button>
                <Button variant="outline" size="sm" className="w-full text-red-600">
                  Delete Account
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
