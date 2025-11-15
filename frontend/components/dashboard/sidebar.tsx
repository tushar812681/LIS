'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { cn } from '@/lib/utils';
import { useSidebarStore } from '@/lib/store';
import {
  LayoutDashboard,
  Users,
  TestTube2,
  FileText,
  Microscope,
  Package,
  Bell,
  Settings,
  LogOut,
  ChevronLeft,
  ChevronRight,
  Beaker,
  ClipboardCheck,
  DollarSign,
  Building2,
  UserCog,
  Activity,
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useAuthStore } from '@/lib/store';
import { AuthService } from '@/lib/auth';
import { useRouter } from 'next/navigation';
import { useState, useEffect } from 'react';

interface NavItem {
  title: string;
  href: string;
  icon: React.ComponentType<{ className?: string }>;
  permission?: string;
  roles?: string[];
}

// Organized menu items for Laboratory Information System workflow
const navItems: NavItem[] = [
  // Core Dashboard
  {
    title: 'Dashboard',
    href: '/dashboard',
    icon: LayoutDashboard,
  },

  // Patient Management - Core workflow starts here
  {
    title: 'Patients',
    href: '/dashboard/patients',
    icon: Users,
    permission: 'VIEW_PATIENT',
  },

  // Order Management - After patient registration
  {
    title: 'Orders',
    href: '/dashboard/orders',
    icon: ClipboardCheck,
    permission: 'VIEW_ORDER',
  },

  // Sample Management - After order creation
  {
    title: 'Samples',
    href: '/dashboard/samples',
    icon: TestTube2,
    permission: 'VIEW_SAMPLE',
  },

  // Results Entry - After sample processing
  {
    title: 'Results',
    href: '/dashboard/results',
    icon: Beaker,
    permission: 'VIEW_RESULT',
  },

  // Reports Generation - Final step in workflow
  {
    title: 'Reports',
    href: '/dashboard/reports',
    icon: FileText,
    permission: 'VIEW_REPORT',
  },

  // Quality Control - Lab operations
  {
    title: 'Quality Control',
    href: '/dashboard/qc',
    icon: Microscope,
    permission: 'VIEW_QC',
    roles: ['LAB_TECHNICIAN', 'LAB_MANAGER', 'ADMIN', 'SUPER_ADMIN'],
  },

  // Equipment Management
  {
    title: 'Equipment',
    href: '/dashboard/equipment',
    icon: Activity,
    permission: 'VIEW_EQUIPMENT',
    roles: ['LAB_TECHNICIAN', 'LAB_MANAGER', 'ADMIN', 'SUPER_ADMIN'],
  },

  // Inventory Management
  {
    title: 'Inventory',
    href: '/dashboard/inventory',
    icon: Package,
    permission: 'VIEW_INVENTORY',
    roles: ['LAB_MANAGER', 'ADMIN', 'SUPER_ADMIN'],
  },

  // Billing & Finance
  {
    title: 'Billing',
    href: '/dashboard/billing',
    icon: DollarSign,
    permission: 'VIEW_INVOICE',
    roles: ['BILLING_STAFF', 'ADMIN', 'SUPER_ADMIN'],
  },

  // Organization Settings - Admin only
  {
    title: 'Organization',
    href: '/dashboard/organization',
    icon: Building2,
    permission: 'VIEW_ORGANIZATION',
    roles: ['ADMIN', 'SUPER_ADMIN'],
  },

  // User Management - Admin only
  {
    title: 'Users',
    href: '/dashboard/users',
    icon: UserCog,
    permission: 'VIEW_USER',
    roles: ['ADMIN', 'SUPER_ADMIN'],
  },

  // Notifications - Available to all
  {
    title: 'Notifications',
    href: '/dashboard/notifications',
    icon: Bell,
  },

  // Settings - Available to all
  {
    title: 'Settings',
    href: '/dashboard/settings',
    icon: Settings,
  },
];

export function Sidebar() {
  const pathname = usePathname();
  const { isCollapsed, toggle } = useSidebarStore();
  const { user, logout } = useAuthStore();
  const router = useRouter();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const handleLogout = () => {
    AuthService.logout();
    logout();
    router.push('/login');
  };

  const canAccessItem = (item: NavItem) => {
    if (!item.permission && !item.roles) return true;

    // Check role-based access
    if (item.roles && user?.roles) {
      if (!item.roles.some(role => user.roles.includes(role))) return false;
    }

    // Check permission-based access
    if (item.permission) {
      return AuthService.hasPermission(item.permission);
    }

    return true;
  };

  const filteredNavItems = navItems.filter(canAccessItem);

  return (
    <div
      className={cn(
        'fixed left-0 top-0 z-40 h-screen border-r border-gray-200 bg-white transition-all duration-300 dark:border-gray-800 dark:bg-gray-900',
        isCollapsed ? 'w-16' : 'w-64'
      )}
    >
      {/* Header */}
      <div className="flex h-16 items-center justify-between border-b border-gray-200 px-4 dark:border-gray-800">
        {!isCollapsed && (
          <div className="flex items-center gap-2">
            <div className="h-8 w-8 rounded-lg bg-gradient-to-br from-blue-600 to-purple-600" />
            <span className="text-lg font-bold text-gray-900 dark:text-white">
              LIS Modern
            </span>
          </div>
        )}
        <Button
          variant="ghost"
          size="icon"
          onClick={toggle}
          className="ml-auto"
        >
          {isCollapsed ? (
            <ChevronRight className="h-4 w-4" />
          ) : (
            <ChevronLeft className="h-4 w-4" />
          )}
        </Button>
      </div>

      {/* Navigation */}
      <nav className="flex flex-col gap-1 p-2">
        {filteredNavItems.map((item) => {
          const Icon = item.icon;
          const isActive = pathname === item.href || pathname.startsWith(item.href + '/');

          return (
            <Link
              key={item.href}
              href={item.href}
              className={cn(
                'flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors',
                isActive
                  ? 'bg-blue-50 text-blue-700 dark:bg-blue-950 dark:text-blue-300'
                  : 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800'
              )}
              title={isCollapsed ? item.title : undefined}
            >
              <Icon className="h-5 w-5 flex-shrink-0" />
              {!isCollapsed && <span>{item.title}</span>}
            </Link>
          );
        })}
      </nav>

      {/* User Profile & Logout */}
      <div className="absolute bottom-0 left-0 right-0 border-t border-gray-200 p-2 dark:border-gray-800">
        {mounted && (
          <>
            {!isCollapsed && user && (
              <div className="mb-2 rounded-lg bg-gray-50 p-3 dark:bg-gray-800">
                <p className="text-sm font-medium text-gray-900 dark:text-white">
                  {user.name}
                </p>
                <p className="text-xs text-gray-600 dark:text-gray-400">{user.email}</p>
                <p className="mt-1 text-xs font-medium text-blue-600 dark:text-blue-400">
                  {user.roles[0]}
                </p>
              </div>
            )}
            <Button
              variant="ghost"
              onClick={handleLogout}
              className={cn(
                'w-full justify-start text-red-600 hover:bg-red-50 hover:text-red-700 dark:text-red-400 dark:hover:bg-red-950',
                isCollapsed && 'justify-center'
              )}
            >
              <LogOut className="h-5 w-5" />
              {!isCollapsed && <span className="ml-3">Logout</span>}
            </Button>
          </>
        )}
      </div>
    </div>
  );
}
