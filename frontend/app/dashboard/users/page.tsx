'use client';

import { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { UserCog, Plus, Search, Shield, Mail, Phone } from 'lucide-react';

export const dynamic = 'force-dynamic';

interface User {
  id: string;
  name: string;
  email: string;
  phone: string;
  role: string;
  status: 'ACTIVE' | 'INACTIVE';
  lastLogin: string;
}

const roleColors: Record<string, string> = {
  SUPER_ADMIN: 'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-300',
  ADMIN: 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300',
  LAB_MANAGER: 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300',
  LAB_TECHNICIAN: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300',
  DOCTOR: 'bg-indigo-100 text-indigo-800 dark:bg-indigo-900 dark:text-indigo-300',
  BILLING_STAFF: 'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-300',
};

const mockUsers: User[] = [
  {
    id: '1',
    name: 'Admin User',
    email: 'admin@citylab.com',
    phone: '+91 9876543210',
    role: 'ADMIN',
    status: 'ACTIVE',
    lastLogin: '2025-11-06T08:30:00Z',
  },
  {
    id: '2',
    name: 'John Technician',
    email: 'john@citylab.com',
    phone: '+91 9876543211',
    role: 'LAB_TECHNICIAN',
    status: 'ACTIVE',
    lastLogin: '2025-11-05T16:45:00Z',
  },
];

export default function UsersPage() {
  const [searchQuery, setSearchQuery] = useState('');
  const [users] = useState<User[]>(mockUsers);

  const filteredUsers = users.filter((user) => {
    const query = searchQuery.toLowerCase();
    return (
      user.name.toLowerCase().includes(query) ||
      user.email.toLowerCase().includes(query) ||
      user.role.toLowerCase().includes(query)
    );
  });

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">User Management</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Manage users, roles, and permissions
          </p>
        </div>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          Add User
        </Button>
      </div>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Total Users</CardDescription>
            <CardTitle className="text-3xl">24</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Active</CardDescription>
            <CardTitle className="text-3xl text-green-600">22</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Inactive</CardDescription>
            <CardTitle className="text-3xl text-gray-600">2</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Online Now</CardDescription>
            <CardTitle className="text-3xl text-blue-600">8</CardTitle>
          </CardHeader>
        </Card>
      </div>

      {/* Search */}
      <Card>
        <CardContent className="pt-6">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
            <Input
              placeholder="Search users by name, email, or role..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10"
            />
          </div>
        </CardContent>
      </Card>

      {/* User List */}
      <div className="space-y-4">
        {filteredUsers.map((user) => (
          <Card key={user.id} className="hover:shadow-lg transition-shadow">
            <CardHeader>
              <div className="flex items-start justify-between">
                <div className="flex items-center gap-3">
                  <div className="flex h-12 w-12 items-center justify-center rounded-full bg-gradient-to-br from-blue-600 to-purple-600">
                    <UserCog className="h-6 w-6 text-white" />
                  </div>
                  <div>
                    <CardTitle className="text-lg">{user.name}</CardTitle>
                    <CardDescription className="flex items-center gap-2 mt-1">
                      <span className={`px-2 py-1 text-xs font-medium rounded-full ${roleColors[user.role]}`}>
                        {user.role.replace('_', ' ')}
                      </span>
                      <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                        user.status === 'ACTIVE'
                          ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300'
                          : 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300'
                      }`}>
                        {user.status}
                      </span>
                    </CardDescription>
                  </div>
                </div>
              </div>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-3 gap-4 text-sm">
                <div>
                  <p className="text-gray-500 dark:text-gray-400 flex items-center gap-1">
                    <Mail className="h-4 w-4" />
                    Email
                  </p>
                  <p className="font-medium text-gray-900 dark:text-white">{user.email}</p>
                </div>
                <div>
                  <p className="text-gray-500 dark:text-gray-400 flex items-center gap-1">
                    <Phone className="h-4 w-4" />
                    Phone
                  </p>
                  <p className="font-medium text-gray-900 dark:text-white">{user.phone}</p>
                </div>
                <div>
                  <p className="text-gray-500 dark:text-gray-400">Last Login</p>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {new Date(user.lastLogin).toLocaleString()}
                  </p>
                </div>
              </div>
              <div className="flex gap-2 mt-4">
                <Button variant="outline" size="sm">
                  <Shield className="mr-2 h-4 w-4" />
                  Edit Permissions
                </Button>
                <Button variant="outline" size="sm">
                  Edit Profile
                </Button>
                <Button variant="outline" size="sm" className="text-red-600 hover:text-red-700">
                  Deactivate
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Empty State */}
      {filteredUsers.length === 0 && (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <UserCog className="h-12 w-12 text-gray-400" />
            <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
              No users found
            </h3>
            <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
              {searchQuery ? 'Try adjusting your search criteria' : 'Get started by adding a new user'}
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
