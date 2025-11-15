'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Building2, Save, MapPin, Phone, Mail, Globe } from 'lucide-react';

export const dynamic = 'force-dynamic';

export default function OrganizationPage() {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Organization</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Manage organization settings and configuration
          </p>
        </div>
        <Button>
          <Save className="mr-2 h-4 w-4" />
          Save Changes
        </Button>
      </div>

      {/* Organization Details */}
      <Card>
        <CardHeader>
          <CardTitle>Organization Details</CardTitle>
          <CardDescription>Basic information about your organization</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2">
            <div>
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                Organization Name
              </label>
              <Input placeholder="Lab Name" defaultValue="City Medical Laboratory" />
            </div>
            <div>
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                Registration Number
              </label>
              <Input placeholder="REG12345" defaultValue="REG2025001" />
            </div>
            <div>
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300 flex items-center gap-1">
                <Phone className="h-4 w-4" />
                Phone Number
              </label>
              <Input placeholder="+91 1234567890" defaultValue="+91 9876543210" />
            </div>
            <div>
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300 flex items-center gap-1">
                <Mail className="h-4 w-4" />
                Email
              </label>
              <Input type="email" placeholder="info@lab.com" defaultValue="info@citylab.com" />
            </div>
            <div className="md:col-span-2">
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300 flex items-center gap-1">
                <MapPin className="h-4 w-4" />
                Address
              </label>
              <Input placeholder="Street Address" defaultValue="123 Main Street, City" />
            </div>
            <div>
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300 flex items-center gap-1">
                <Globe className="h-4 w-4" />
                Website
              </label>
              <Input placeholder="https://lab.com" defaultValue="https://citylab.com" />
            </div>
            <div>
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                License Number
              </label>
              <Input placeholder="LIC12345" defaultValue="LIC2025001" />
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Additional Settings */}
      <div className="grid gap-6 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Branding</CardTitle>
            <CardDescription>Customize your organization branding</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="text-center py-8 text-gray-500">
              <Building2 className="h-10 w-10 mx-auto mb-3 text-gray-400" />
              <p className="text-sm">Logo and branding customization</p>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Integrations</CardTitle>
            <CardDescription>Connect external services and APIs</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="text-center py-8 text-gray-500">
              <Globe className="h-10 w-10 mx-auto mb-3 text-gray-400" />
              <p className="text-sm">ABDM, payment gateways, and more</p>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
