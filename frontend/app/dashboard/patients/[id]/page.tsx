'use client';

import * as React from 'react';
import { useParams, useRouter } from 'next/navigation';
import {
  User,
  Calendar,
  Phone,
  Mail,
  MapPin,
  FileText,
  Activity,
  AlertCircle,
  Edit,
  Trash2,
  ArrowLeft,
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import { FormModal } from '@/components/ui/form-modal';
import { useConfirmDialog } from '@/components/ui/alert-dialog';
import { StatusBadge } from '@/components/ui/status-badge';
import { SkeletonCard, SkeletonTable } from '@/components/ui/skeleton';
import { Input } from '@/components/ui/input';
import { usePatient, useUpdatePatient, useDeletePatient } from '@/lib/hooks/usePatients';
import { useForm } from 'react-hook-form';
import { format } from 'date-fns';

export const dynamic = 'force-dynamic';

type TabType = 'overview' | 'medical-history' | 'test-history' | 'reports';

interface PatientType {
  patientId: string;
  firstName: string;
  lastName: string;
  middleName?: string;
  dateOfBirth: string;
  gender: string;
  phone?: string;
  alternatePhone?: string;
  email?: string;
  status: string;
  address?: {
    street: string;
    city: string;
    state: string;
    postalCode: string;
    country: string;
  };
  emergencyContact?: {
    name: string;
    relationship: string;
    phone: string;
  };
  insuranceInfo?: {
    provider: string;
    policyNumber: string;
    groupNumber?: string;
    validUntil: string;
  };
  medicalHistory?: {
    allergies?: string;
    medications?: string;
    conditions?: string;
    notes?: string;
  };
}

export default function PatientDetailPage() {
  const params = useParams();
  const router = useRouter();
  const patientId = params.id as string;

  const [mounted, setMounted] = React.useState(false);
  const [activeTab, setActiveTab] = React.useState<TabType>('overview');
  const [editModalOpen, setEditModalOpen] = React.useState(false);

  const { patient, loading, error, refetch } = usePatient(patientId);
  const { updatePatient, loading: updating } = useUpdatePatient();
  const { deletePatient } = useDeletePatient();
  const { confirm, ConfirmDialog } = useConfirmDialog();

  const form = useForm();

  const typedPatient = patient ? (patient as PatientType) : null;

  React.useEffect(() => {
    setMounted(true);
  }, []);

  React.useEffect(() => {
    if (typedPatient) {
      form.reset({
        firstName: typedPatient.firstName,
        lastName: typedPatient.lastName,
        middleName: typedPatient.middleName || '',
        email: typedPatient.email || '',
        phone: typedPatient.phone || '',
        alternatePhone: typedPatient.alternatePhone || '',
        street: typedPatient.address?.street || '',
        city: typedPatient.address?.city || '',
        state: typedPatient.address?.state || '',
        postalCode: typedPatient.address?.postalCode || '',
        country: typedPatient.address?.country || '',
      });
    }
  }, [typedPatient, form]);

  if (!mounted) return null;

  const handleEditSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const values = form.getValues();

    try {
      await updatePatient({
        variables: {
          id: patientId,
          input: {
            firstName: values.firstName,
            lastName: values.lastName,
            middleName: values.middleName,
            email: values.email,
            phone: values.phone,
            alternatePhone: values.alternatePhone,
            address: {
              street: values.street,
              city: values.city,
              state: values.state,
              postalCode: values.postalCode,
              country: values.country,
            },
          },
        },
      });
      setEditModalOpen(false);
      refetch();
    } catch (error) {
      console.error('Error updating patient:', error);
    }
  };

  const handleDelete = () => {
    confirm({
      title: 'Delete Patient',
      description: 'Are you sure you want to delete this patient? This action cannot be undone.',
      variant: 'danger',
      onConfirm: async () => {
        try {
          await deletePatient({ variables: { id: patientId } });
          router.push('/dashboard/patients');
        } catch (error) {
          console.error('Error deleting patient:', error);
        }
      },
    });
  };

  if (loading) {
    return (
      <div className="space-y-6">
        <SkeletonCard />
        <SkeletonCard />
        <SkeletonTable rows={5} columns={4} />
      </div>
    );
  }

  if (error || !typedPatient) {
    return (
      <div className="flex flex-col items-center justify-center py-12">
        <AlertCircle className="h-12 w-12 text-red-500" />
        <h2 className="mt-4 text-lg font-semibold">Patient not found</h2>
        <p className="mt-2 text-sm text-gray-600">
          The patient you&apos;re looking for doesn&apos;t exist or has been deleted.
        </p>
        <Button onClick={() => router.push('/dashboard/patients')} className="mt-4">
          Back to Patients
        </Button>
      </div>
    );
  }

  const tabs = [
    { id: 'overview' as const, label: 'Overview', icon: User },
    { id: 'medical-history' as const, label: 'Medical History', icon: FileText },
    { id: 'test-history' as const, label: 'Test History', icon: Activity },
    { id: 'reports' as const, label: 'Reports', icon: FileText },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => router.push('/dashboard/patients')}
          >
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back
          </Button>
          <div>
            <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-100">
              {typedPatient.firstName} {typedPatient.lastName}
            </h1>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Patient ID: {typedPatient.patientId}
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => setEditModalOpen(true)}>
            <Edit className="mr-2 h-4 w-4" />
            Edit
          </Button>
          <Button variant="destructive" onClick={handleDelete}>
            <Trash2 className="mr-2 h-4 w-4" />
            Delete
          </Button>
        </div>
      </div>

      {/* Patient Info Card */}
      <div className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-800 dark:bg-gray-900">
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <div className="flex items-start gap-3">
            <Calendar className="mt-0.5 h-5 w-5 text-gray-400" />
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Date of Birth</p>
              <p className="font-medium text-gray-900 dark:text-gray-100">
                {format(new Date(typedPatient.dateOfBirth), 'MMM dd, yyyy')}
              </p>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <User className="mt-0.5 h-5 w-5 text-gray-400" />
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Gender</p>
              <p className="font-medium text-gray-900 dark:text-gray-100">
                {typedPatient.gender}
              </p>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <Phone className="mt-0.5 h-5 w-5 text-gray-400" />
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Phone</p>
              <p className="font-medium text-gray-900 dark:text-gray-100">
                {typedPatient.phone || 'N/A'}
              </p>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <Mail className="mt-0.5 h-5 w-5 text-gray-400" />
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Email</p>
              <p className="font-medium text-gray-900 dark:text-gray-100">
                {typedPatient.email || 'N/A'}
              </p>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <MapPin className="mt-0.5 h-5 w-5 text-gray-400" />
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Address</p>
              <p className="font-medium text-gray-900 dark:text-gray-100">
                {typedPatient.address
                  ? `${typedPatient.address.street}, ${typedPatient.address.city}, ${typedPatient.address.state} ${typedPatient.address.postalCode}`
                  : 'N/A'}
              </p>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <Activity className="mt-0.5 h-5 w-5 text-gray-400" />
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Status</p>
              <StatusBadge variant={typedPatient.status === 'ACTIVE' ? 'success' : 'neutral'}>
                {typedPatient.status}
              </StatusBadge>
            </div>
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200 dark:border-gray-800">
        <nav className="-mb-px flex space-x-8">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex items-center gap-2 border-b-2 px-1 py-4 text-sm font-medium transition-colors ${
                  activeTab === tab.id
                    ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'border-transparent text-gray-600 hover:border-gray-300 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100'
                }`}
              >
                <Icon className="h-4 w-4" />
                {tab.label}
              </button>
            );
          })}
        </nav>
      </div>

      {/* Tab Content */}
      <div>
        {activeTab === 'overview' && <OverviewTab patient={typedPatient} />}
        {activeTab === 'medical-history' && <MedicalHistoryTab patient={typedPatient} />}
        {activeTab === 'test-history' && <TestHistoryTab />}
        {activeTab === 'reports' && <ReportsTab />}
      </div>

      {/* Edit Modal */}
      <FormModal
        open={editModalOpen}
        onClose={() => setEditModalOpen(false)}
        title="Edit Patient Information"
        description="Update patient details"
        onSubmit={handleEditSubmit}
        isSubmitting={updating}
      >
        <div className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2">
            <div>
              <label className="mb-1 block text-sm font-medium">First Name</label>
              <Input {...form.register('firstName')} />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium">Last Name</label>
              <Input {...form.register('lastName')} />
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium">Middle Name</label>
            <Input {...form.register('middleName')} />
          </div>

          <div className="grid gap-4 md:grid-cols-2">
            <div>
              <label className="mb-1 block text-sm font-medium">Email</label>
              <Input type="email" {...form.register('email')} />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium">Phone</label>
              <Input {...form.register('phone')} />
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium">Street Address</label>
            <Input {...form.register('street')} />
          </div>

          <div className="grid gap-4 md:grid-cols-3">
            <div>
              <label className="mb-1 block text-sm font-medium">City</label>
              <Input {...form.register('city')} />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium">State</label>
              <Input {...form.register('state')} />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium">Postal Code</label>
              <Input {...form.register('postalCode')} />
            </div>
          </div>
        </div>
      </FormModal>

      {ConfirmDialog}
    </div>
  );
}

// Overview Tab Component
function OverviewTab({ patient }: { patient: PatientType }) {
  return (
    <div className="space-y-6">
      {/* Emergency Contact */}
      {patient.emergencyContact && (
        <div className="rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900">
          <h3 className="mb-4 text-lg font-semibold">Emergency Contact</h3>
          <div className="grid gap-4 md:grid-cols-3">
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Name</p>
              <p className="font-medium">{patient.emergencyContact.name}</p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Relationship</p>
              <p className="font-medium">{patient.emergencyContact.relationship}</p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Phone</p>
              <p className="font-medium">{patient.emergencyContact.phone}</p>
            </div>
          </div>
        </div>
      )}

      {/* Insurance Information */}
      {patient.insuranceInfo && (
        <div className="rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900">
          <h3 className="mb-4 text-lg font-semibold">Insurance Information</h3>
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Provider</p>
              <p className="font-medium">{patient.insuranceInfo.provider}</p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Policy Number</p>
              <p className="font-medium">{patient.insuranceInfo.policyNumber}</p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Group Number</p>
              <p className="font-medium">{patient.insuranceInfo.groupNumber}</p>
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Valid Until</p>
              <p className="font-medium">
                {format(new Date(patient.insuranceInfo.validUntil), 'MMM dd, yyyy')}
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

// Medical History Tab Component
function MedicalHistoryTab({ patient }: { patient: PatientType }) {
  const medicalHistory = patient.medicalHistory || {};

  return (
    <div className="space-y-6">
      <div className="rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900">
        <h3 className="mb-4 text-lg font-semibold">Medical History</h3>

        <div className="space-y-6">
          <div>
            <h4 className="mb-2 font-medium text-gray-900 dark:text-gray-100">Allergies</h4>
            <p className="text-gray-700 dark:text-gray-300">
              {medicalHistory.allergies || 'No known allergies'}
            </p>
          </div>

          <div>
            <h4 className="mb-2 font-medium text-gray-900 dark:text-gray-100">Medications</h4>
            <p className="text-gray-700 dark:text-gray-300">
              {medicalHistory.medications || 'No current medications'}
            </p>
          </div>

          <div>
            <h4 className="mb-2 font-medium text-gray-900 dark:text-gray-100">
              Medical Conditions
            </h4>
            <p className="text-gray-700 dark:text-gray-300">
              {medicalHistory.conditions || 'No known conditions'}
            </p>
          </div>

          <div>
            <h4 className="mb-2 font-medium text-gray-900 dark:text-gray-100">Notes</h4>
            <p className="text-gray-700 dark:text-gray-300">
              {medicalHistory.notes || 'No additional notes'}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

// Test History Tab Component (Placeholder - will integrate with actual data)
function TestHistoryTab() {
  return (
    <div className="rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900">
      <h3 className="mb-4 text-lg font-semibold">Test History</h3>
      <p className="text-gray-600 dark:text-gray-400">
        Test history will be displayed here once orders are created.
      </p>
    </div>
  );
}

// Reports Tab Component (Placeholder - will integrate with actual data)
function ReportsTab() {
  return (
    <div className="rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-900">
      <h3 className="mb-4 text-lg font-semibold">Reports</h3>
      <p className="text-gray-600 dark:text-gray-400">
        Patient reports will be displayed here once available.
      </p>
    </div>
  );
}
