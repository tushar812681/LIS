'use client';

import * as React from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { DataTable, DataTableColumnHeader } from '@/components/ui/data-table';
import { SearchBar } from '@/components/ui/search-bar';
import { StatusBadge } from '@/components/ui/status-badge';
import { SkeletonTable } from '@/components/ui/skeleton';
import { usePatients } from '@/lib/hooks/usePatients';
import { Plus, Filter, Download, User, Eye } from 'lucide-react';
import { format } from 'date-fns';
import type { ColumnDef } from '@tanstack/react-table';

export const dynamic = 'force-dynamic';

interface Patient {
  id: string;
  patientId: string;
  firstName: string;
  lastName: string;
  dateOfBirth: string;
  gender: string;
  email?: string;
  phone?: string;
  status: string;
  createdAt: string;
}

export default function PatientsPage() {
  const router = useRouter();
  const [mounted, setMounted] = React.useState(false);
  const [searchQuery, setSearchQuery] = React.useState('');
  const [page, setPage] = React.useState(1);
  const [limit] = React.useState(20);

  const { patients, pagination, loading, error, refetch } = usePatients({
    page,
    limit,
    search: searchQuery,
  });

  const typedPatients = (patients as Patient[]) || [];

  React.useEffect(() => {
    setMounted(true);
  }, []);

  React.useEffect(() => {
    if (searchQuery) {
      setPage(1); // Reset to first page when searching
    }
  }, [searchQuery]);

  if (!mounted) return null;

  const columns: ColumnDef<Patient>[] = [
    {
      accessorKey: 'patientId',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Patient ID" />,
      cell: ({ row }) => (
        <div className="font-medium text-gray-900 dark:text-gray-100">
          {row.getValue('patientId')}
        </div>
      ),
    },
    {
      accessorKey: 'firstName',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Name" />,
      cell: ({ row }) => {
        const firstName = row.getValue('firstName') as string;
        const lastName = row.original.lastName;
        return (
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
              <User className="h-5 w-5 text-blue-600 dark:text-blue-300" />
            </div>
            <div>
              <div className="font-medium text-gray-900 dark:text-gray-100">
                {firstName} {lastName}
              </div>
              <div className="text-sm text-gray-600 dark:text-gray-400">
                {row.original.email || 'No email'}
              </div>
            </div>
          </div>
        );
      },
    },
    {
      accessorKey: 'dateOfBirth',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Date of Birth" />,
      cell: ({ row }) => {
        const dob = row.getValue('dateOfBirth') as string;
        return (
          <div className="text-sm text-gray-900 dark:text-gray-100">
            {format(new Date(dob), 'MMM dd, yyyy')}
          </div>
        );
      },
    },
    {
      accessorKey: 'gender',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Gender" />,
      cell: ({ row }) => (
        <div className="text-sm text-gray-900 dark:text-gray-100">
          {row.getValue('gender')}
        </div>
      ),
    },
    {
      accessorKey: 'phone',
      header: 'Contact',
      cell: ({ row }) => (
        <div className="text-sm">
          <div className="text-gray-900 dark:text-gray-100">
            {row.getValue('phone') || 'N/A'}
          </div>
        </div>
      ),
    },
    {
      accessorKey: 'status',
      header: 'Status',
      cell: ({ row }) => {
        const status = row.getValue('status') as string;
        return (
          <StatusBadge variant={status === 'ACTIVE' ? 'success' : 'neutral'}>
            {status}
          </StatusBadge>
        );
      },
    },
    {
      accessorKey: 'createdAt',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Registered" />,
      cell: ({ row }) => {
        const date = row.getValue('createdAt') as string;
        return (
          <div className="text-sm text-gray-600 dark:text-gray-400">
            {format(new Date(date), 'MMM dd, yyyy')}
          </div>
        );
      },
    },
    {
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        const patient = row.original;
        return (
          <div className="flex gap-2">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => router.push(`/dashboard/patients/${patient.id}`)}
            >
              <Eye className="h-4 w-4" />
            </Button>
          </div>
        );
      },
    },
  ];

  const handleRowClick = (patient: Patient) => {
    router.push(`/dashboard/patients/${patient.id}`);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Patients</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Manage patient records and registrations
          </p>
        </div>
        <Link href="/dashboard/patients/register">
          <Button>
            <Plus className="mr-2 h-4 w-4" />
            Register Patient
          </Button>
        </Link>
      </div>

      {/* Search and Actions */}
      <Card className="p-6">
        <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
          <div className="flex-1 md:max-w-md">
            <SearchBar
              value={searchQuery}
              onChange={setSearchQuery}
              placeholder="Search by name, ID, phone, or email..."
              debounceMs={500}
            />
          </div>
          <div className="flex gap-2">
            <Button variant="outline">
              <Filter className="mr-2 h-4 w-4" />
              Filters
            </Button>
            <Button variant="outline">
              <Download className="mr-2 h-4 w-4" />
              Export
            </Button>
          </div>
        </div>
      </Card>

      {/* Patient Table */}
      <Card className="p-6">
        {loading ? (
          <SkeletonTable rows={10} columns={7} />
        ) : error ? (
          <div className="flex flex-col items-center justify-center py-12">
            <div className="text-red-500">Error loading patients</div>
            <Button onClick={() => refetch()} className="mt-4" variant="outline">
              Try Again
            </Button>
          </div>
        ) : typedPatients.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12">
            <User className="h-12 w-12 text-gray-400" />
            <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
              No patients found
            </h3>
            <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
              {searchQuery
                ? 'Try adjusting your search criteria'
                : 'Get started by registering a new patient'}
            </p>
            {!searchQuery && (
              <Link href="/dashboard/patients/register">
                <Button className="mt-4">
                  <Plus className="mr-2 h-4 w-4" />
                  Register Patient
                </Button>
              </Link>
            )}
          </div>
        ) : (
          <DataTable
            columns={columns}
            data={typedPatients}
            onRowClick={handleRowClick}
          />
        )}
      </Card>

      {/* Pagination Info */}
      {pagination && pagination.total > 0 && (
        <div className="flex items-center justify-between text-sm text-gray-600 dark:text-gray-400">
          <div>
            Showing {(page - 1) * limit + 1} to {Math.min(page * limit, pagination.total)} of{' '}
            {pagination.total} patients
          </div>
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setPage((p) => Math.max(1, p - 1))}
              disabled={page === 1}
            >
              Previous
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setPage((p) => Math.min(pagination.totalPages, p + 1))}
              disabled={page === pagination.totalPages}
            >
              Next
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
