'use client';

import { Sidebar } from '@/components/dashboard/sidebar';
import { Header } from '@/components/dashboard/header';
import { useSidebarStore } from '@/lib/store';
import { cn } from '@/lib/utils';

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const isCollapsed = useSidebarStore((state) => state.isCollapsed);

  return (
    <div className="flex min-h-screen bg-gray-50 dark:bg-gray-950">
      {/* Sidebar */}
      <Sidebar />

      {/* Main Content */}
      <div
        className={cn(
          'flex-1 transition-all duration-300',
          isCollapsed ? 'ml-16' : 'ml-64'
        )}
      >
        {/* Header */}
        <Header />

        {/* Page Content */}
        <main className="p-6">{children}</main>
      </div>
    </div>
  );
}
