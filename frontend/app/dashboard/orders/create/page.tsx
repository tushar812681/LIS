'use client';

import * as React from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card } from '@/components/ui/card';
import { SearchBar } from '@/components/ui/search-bar';
import { SkeletonCard, SkeletonList } from '@/components/ui/skeleton';
import { useCreateOrder } from '@/lib/hooks';
import { useTestCatalog, useTestCategories } from '@/lib/hooks';
import { usePatientSearch } from '@/lib/hooks/usePatients';
import { useNotificationStore } from '@/lib/store';
import {
  X,
  User,
  Calendar,
  FileText,
  CheckCircle,
} from 'lucide-react';

const orderSchema = z.object({
  patientId: z.string().min(1, 'Patient is required'),
  orderDate: z.string().min(1, 'Order date is required'),
  priority: z.enum(['ROUTINE', 'URGENT', 'STAT'], { message: 'Priority is required' }),
  doctorName: z.string().min(2, 'Doctor name is required'),
  specialty: z.string().optional(),
  licenseNumber: z.string().optional(),
  clinicalDiagnosis: z.string().optional(),
  symptoms: z.string().optional(),
  clinicalNotes: z.string().optional(),
  testIds: z.array(z.string()).min(1, 'At least one test must be selected'),
});

type OrderFormValues = z.infer<typeof orderSchema>;

interface SelectedTest {
  id: string;
  testCode: string;
  testName: string;
  category: string;
  price: number;
  sampleType: string;
}

interface Patient {
  id: string;
  patientId: string;
  firstName: string;
  lastName: string;
  dateOfBirth: string;
}

interface Category {
  id: string;
  code: string;
  name: string;
}

function CreateOrderPageContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const patientIdParam = searchParams.get('patientId');

  const [mounted, setMounted] = React.useState(false);
  const [step, setStep] = React.useState(1); // 1: Patient, 2: Tests, 3: Details, 4: Review
  const [searchQuery, setSearchQuery] = React.useState('');
  const [selectedPatient, setSelectedPatient] = React.useState<Patient | null>(null);
  const [selectedTests, setSelectedTests] = React.useState<SelectedTest[]>([]);
  const [categoryFilter, setCategoryFilter] = React.useState<string>('');

  const addNotification = useNotificationStore((state) => state.addNotification);

  const { createOrder, loading } = useCreateOrder();
  const { searchPatients, results: patientResults, loading: searchingPatients } = usePatientSearch();
  const { tests, loading: loadingTests } = useTestCatalog({
    search: searchQuery,
    category: categoryFilter,
  });
  const { categories } = useTestCategories();

  const form = useForm<OrderFormValues>({
    resolver: zodResolver(orderSchema),
    defaultValues: {
      patientId: patientIdParam || '',
      orderDate: new Date().toISOString().split('T')[0],
      priority: 'ROUTINE',
      doctorName: '',
      specialty: '',
      licenseNumber: '',
      clinicalDiagnosis: '',
      symptoms: '',
      clinicalNotes: '',
      testIds: [],
    },
  });

  React.useEffect(() => {
    setMounted(true);
  }, []);

  React.useEffect(() => {
    if (patientIdParam && !selectedPatient) {
      // If patient ID is provided in URL, fetch patient details
      // This would be handled by a usePatient hook
      setStep(2);
    }
  }, [patientIdParam, selectedPatient]);

  if (!mounted) return null;

  const handlePatientSearch = async (query: string) => {
    if (query.length >= 2) {
      await searchPatients(query, 10);
    }
  };

  const handleSelectPatient = (patient: Patient) => {
    setSelectedPatient(patient);
    form.setValue('patientId', patient.id);
    setStep(2);
  };

  const handleSelectTest = (test: SelectedTest) => {
    const isSelected = selectedTests.find((t) => t.id === test.id);
    if (isSelected) {
      setSelectedTests(selectedTests.filter((t) => t.id !== test.id));
      form.setValue(
        'testIds',
        selectedTests.filter((t) => t.id !== test.id).map((t) => t.id)
      );
    } else {
      const newTests = [...selectedTests, test];
      setSelectedTests(newTests);
      form.setValue(
        'testIds',
        newTests.map((t) => t.id)
      );
    }
  };

  const handleSubmit = async (values: OrderFormValues) => {
    try {
      const result = await createOrder({
        variables: {
          input: {
            patientId: values.patientId,
            orderDate: values.orderDate,
            priority: values.priority,
            testIds: values.testIds,
            doctor: {
              name: values.doctorName,
              specialty: values.specialty || null,
              licenseNumber: values.licenseNumber || null,
            },
            clinicalInfo: {
              diagnosis: values.clinicalDiagnosis || null,
              symptoms: values.symptoms || null,
              notes: values.clinicalNotes || null,
            },
          },
        },
      });

      if (result.data && (result.data as { createOrder?: unknown }).createOrder) {
        const order = (result.data as { createOrder: { id: string; orderId: string } }).createOrder;
        addNotification({
          type: 'success',
          title: 'Order Created',
          message: `Order ${order.orderId} created successfully!`,
        });
        router.push(`/dashboard/orders/${order.id}`);
      }
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Order Creation Failed',
        message: error instanceof Error ? error.message : 'Failed to create order',
      });
    }
  };

  const totalPrice = selectedTests.reduce((sum: number, test: SelectedTest) => sum + test.price, 0);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Create Test Order</h1>
        <p className="text-gray-600 dark:text-gray-400">
          Order laboratory tests for a patient
        </p>
      </div>

      {/* Progress Steps */}
      <div className="flex items-center justify-between">
        {[
          { num: 1, label: 'Select Patient', icon: User },
          { num: 2, label: 'Choose Tests', icon: FileText },
          { num: 3, label: 'Order Details', icon: Calendar },
          { num: 4, label: 'Review & Submit', icon: CheckCircle },
        ].map((s, index) => {
          const Icon = s.icon;
          return (
            <div key={s.num} className="flex flex-1 items-center">
              <div className="flex flex-col items-center">
                <div
                  className={`flex h-10 w-10 items-center justify-center rounded-full border-2 ${
                    step > s.num
                      ? 'border-green-600 bg-green-600 text-white'
                      : step === s.num
                      ? 'border-blue-600 bg-blue-600 text-white'
                      : 'border-gray-300 bg-white text-gray-400 dark:border-gray-700 dark:bg-gray-800'
                  }`}
                >
                  <Icon className="h-5 w-5" />
                </div>
                <p className="mt-2 text-xs font-medium">{s.label}</p>
              </div>
              {index < 3 && (
                <div
                  className={`h-0.5 flex-1 ${
                    step > s.num ? 'bg-green-600' : 'bg-gray-300 dark:bg-gray-700'
                  }`}
                />
              )}
            </div>
          );
        })}
      </div>

      <form onSubmit={form.handleSubmit(handleSubmit)}>
        {/* Step 1: Select Patient */}
        {step === 1 && (
          <Card className="p-6">
            <h2 className="mb-4 text-xl font-semibold">Select Patient</h2>
            <div className="space-y-4">
              <SearchBar
                value=""
                onChange={handlePatientSearch}
                placeholder="Search patient by name, ID, or phone..."
                isLoading={searchingPatients}
              />

              {patientResults.length > 0 && (
                <div className="space-y-2">
                  {(patientResults as Patient[]).map(patient => (
                    <div
                      key={patient.id}
                      onClick={() => handleSelectPatient(patient)}
                      className="flex cursor-pointer items-center justify-between rounded-lg border border-gray-200 p-4 transition-colors hover:bg-gray-50 dark:border-gray-800 dark:hover:bg-gray-800"
                    >
                      <div className="flex items-center gap-3">
                        <div className="flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                          <User className="h-6 w-6 text-blue-600 dark:text-blue-300" />
                        </div>
                        <div>
                          <p className="font-medium text-gray-900 dark:text-gray-100">
                            {patient.firstName} {patient.lastName}
                          </p>
                          <p className="text-sm text-gray-600 dark:text-gray-400">
                            ID: {patient.patientId} • DOB: {patient.dateOfBirth}
                          </p>
                        </div>
                      </div>
                      <Button type="button" size="sm">
                        Select
                      </Button>
                    </div>
                  ))}
                </div>
              )}

              {patientResults.length === 0 && (
                <div className="py-12 text-center text-gray-600 dark:text-gray-400">
                  Search for a patient to create an order
                </div>
              )}
            </div>
          </Card>
        )}

        {/* Step 2: Choose Tests */}
        {step === 2 && (
          <div className="space-y-6">
            {/* Selected Patient Info */}
            {selectedPatient && (
              <Card className="p-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="flex h-10 w-10 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                      <User className="h-5 w-5 text-blue-600 dark:text-blue-300" />
                    </div>
                    <div>
                      <p className="font-medium text-gray-900 dark:text-gray-100">
                        {selectedPatient.firstName} {selectedPatient.lastName}
                      </p>
                      <p className="text-sm text-gray-600 dark:text-gray-400">
                        Patient ID: {selectedPatient.patientId}
                      </p>
                    </div>
                  </div>
                  <Button type="button" variant="outline" size="sm" onClick={() => setStep(1)}>
                    Change Patient
                  </Button>
                </div>
              </Card>
            )}

            {/* Selected Tests Cart */}
            {selectedTests.length > 0 && (
              <Card className="p-4">
                <div className="mb-3 flex items-center justify-between">
                  <h3 className="font-semibold text-gray-900 dark:text-gray-100">
                    Selected Tests ({selectedTests.length})
                  </h3>
                  <p className="text-lg font-bold text-blue-600">₹{totalPrice.toFixed(2)}</p>
                </div>
                <div className="space-y-2">
                  {selectedTests.map((test: SelectedTest) => (
                    <div
                      key={test.id}
                      className="flex items-center justify-between rounded-lg border border-gray-200 p-3 dark:border-gray-800"
                    >
                      <div className="flex-1">
                        <p className="font-medium text-gray-900 dark:text-gray-100">
                          {test.testName}
                        </p>
                        <p className="text-sm text-gray-600 dark:text-gray-400">
                          {test.testCode} • {test.category} • {test.sampleType}
                        </p>
                      </div>
                      <div className="flex items-center gap-3">
                        <p className="font-medium text-gray-900 dark:text-gray-100">
                          ₹{test.price}
                        </p>
                        <Button
                          type="button"
                          variant="ghost"
                          size="sm"
                          onClick={() => handleSelectTest(test)}
                        >
                          <X className="h-4 w-4" />
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              </Card>
            )}

            {/* Test Catalog */}
            <Card className="p-6">
              <div className="mb-4 space-y-4">
                <div className="flex items-center gap-4">
                  <SearchBar
                    value={searchQuery}
                    onChange={setSearchQuery}
                    placeholder="Search tests..."
                    className="flex-1"
                  />
                  <select
                    value={categoryFilter}
                    onChange={(e) => setCategoryFilter(e.target.value)}
                    className="h-10 rounded-md border border-gray-300 bg-white px-3 dark:border-gray-700 dark:bg-gray-800"
                  >
                    <option value="">All Categories</option>
                    {(categories as Category[] | undefined)?.map(cat => (
                      <option key={cat.id} value={cat.code}>
                        {cat.name}
                      </option>
                    ))}
                  </select>
                </div>
              </div>

              {loadingTests ? (
                <SkeletonList items={5} />
              ) : tests.length === 0 ? (
                <div className="py-12 text-center text-gray-600 dark:text-gray-400">
                  No tests found
                </div>
              ) : (
                <div className="space-y-2">
                  {(tests as (SelectedTest & { description?: string; turnaroundTime?: string })[] | undefined)?.map(test => {
                    const isSelected = selectedTests.find((t) => t.id === test.id);
                    return (
                      <div
                        key={test.id}
                        onClick={() => handleSelectTest(test)}
                        className={`flex cursor-pointer items-center justify-between rounded-lg border p-4 transition-colors ${
                          isSelected
                            ? 'border-blue-500 bg-blue-50 dark:border-blue-600 dark:bg-blue-950/30'
                            : 'border-gray-200 hover:bg-gray-50 dark:border-gray-800 dark:hover:bg-gray-800'
                        }`}
                      >
                        <div className="flex-1">
                          <p className="font-medium text-gray-900 dark:text-gray-100">
                            {test.testName}
                          </p>
                          <p className="text-sm text-gray-600 dark:text-gray-400">
                            {test.testCode} • {test.category} • Sample: {test.sampleType} •
                            TAT: {test.turnaroundTime}
                          </p>
                          {test.description && (
                            <p className="mt-1 text-sm text-gray-500 dark:text-gray-500">
                              {test.description}
                            </p>
                          )}
                        </div>
                        <div className="ml-4 flex items-center gap-3">
                          <p className="font-semibold text-gray-900 dark:text-gray-100">
                            ₹{test.price}
                          </p>
                          {isSelected && (
                            <CheckCircle className="h-5 w-5 text-blue-600 dark:text-blue-400" />
                          )}
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </Card>

            <div className="flex justify-between">
              <Button type="button" variant="outline" onClick={() => setStep(1)}>
                Previous
              </Button>
              <Button
                type="button"
                onClick={() => setStep(3)}
                disabled={selectedTests.length === 0}
              >
                Continue
              </Button>
            </div>
          </div>
        )}

        {/* Step 3: Order Details */}
        {step === 3 && (
          <Card className="p-6">
            <h2 className="mb-4 text-xl font-semibold">Order Details</h2>
            <div className="space-y-4">
              <div className="grid gap-4 md:grid-cols-2">
                <div>
                  <label className="mb-1 block text-sm font-medium">Order Date *</label>
                  <Input type="date" {...form.register('orderDate')} />
                  {form.formState.errors.orderDate && (
                    <p className="mt-1 text-sm text-red-600">
                      {form.formState.errors.orderDate.message}
                    </p>
                  )}
                </div>
                <div>
                  <label className="mb-1 block text-sm font-medium">Priority *</label>
                  <select
                    {...form.register('priority')}
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                  >
                    <option value="ROUTINE">Routine</option>
                    <option value="URGENT">Urgent</option>
                    <option value="STAT">STAT</option>
                  </select>
                </div>
              </div>

              <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                <h3 className="mb-3 font-medium">Ordering Physician</h3>
                <div className="space-y-4">
                  <div>
                    <label className="mb-1 block text-sm font-medium">Doctor Name *</label>
                    <Input placeholder="Dr. John Smith" {...form.register('doctorName')} />
                    {form.formState.errors.doctorName && (
                      <p className="mt-1 text-sm text-red-600">
                        {form.formState.errors.doctorName.message}
                      </p>
                    )}
                  </div>
                  <div className="grid gap-4 md:grid-cols-2">
                    <div>
                      <label className="mb-1 block text-sm font-medium">Specialty</label>
                      <Input placeholder="Cardiology" {...form.register('specialty')} />
                    </div>
                    <div>
                      <label className="mb-1 block text-sm font-medium">License Number</label>
                      <Input placeholder="MED123456" {...form.register('licenseNumber')} />
                    </div>
                  </div>
                </div>
              </div>

              <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                <h3 className="mb-3 font-medium">Clinical Information</h3>
                <div className="space-y-4">
                  <div>
                    <label className="mb-1 block text-sm font-medium">Clinical Diagnosis</label>
                    <Input
                      placeholder="Suspected infection"
                      {...form.register('clinicalDiagnosis')}
                    />
                  </div>
                  <div>
                    <label className="mb-1 block text-sm font-medium">Symptoms</label>
                    <Input placeholder="Fever, cough" {...form.register('symptoms')} />
                  </div>
                  <div>
                    <label className="mb-1 block text-sm font-medium">Clinical Notes</label>
                    <textarea
                      {...form.register('clinicalNotes')}
                      rows={3}
                      className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                      placeholder="Additional clinical information..."
                    />
                  </div>
                </div>
              </div>
            </div>

            <div className="mt-6 flex justify-between">
              <Button type="button" variant="outline" onClick={() => setStep(2)}>
                Previous
              </Button>
              <Button type="button" onClick={() => setStep(4)}>
                Review Order
              </Button>
            </div>
          </Card>
        )}

        {/* Step 4: Review & Submit */}
        {step === 4 && (
          <div className="space-y-6">
            <Card className="p-6">
              <h2 className="mb-4 text-xl font-semibold">Review Order</h2>

              <div className="space-y-6">
                {/* Patient Info */}
                <div>
                  <h3 className="mb-2 font-medium text-gray-900 dark:text-gray-100">Patient</h3>
                  <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                    <p className="font-medium">
                      {selectedPatient?.firstName} {selectedPatient?.lastName}
                    </p>
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      ID: {selectedPatient?.patientId}
                    </p>
                  </div>
                </div>

                {/* Order Details */}
                <div>
                  <h3 className="mb-2 font-medium text-gray-900 dark:text-gray-100">
                    Order Details
                  </h3>
                  <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                    <div className="grid gap-2 md:grid-cols-2">
                      <div>
                        <p className="text-sm text-gray-600 dark:text-gray-400">Order Date</p>
                        <p className="font-medium">{form.getValues('orderDate')}</p>
                      </div>
                      <div>
                        <p className="text-sm text-gray-600 dark:text-gray-400">Priority</p>
                        <p className="font-medium">{form.getValues('priority')}</p>
                      </div>
                      <div>
                        <p className="text-sm text-gray-600 dark:text-gray-400">Doctor</p>
                        <p className="font-medium">{form.getValues('doctorName')}</p>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Selected Tests */}
                <div>
                  <h3 className="mb-2 font-medium text-gray-900 dark:text-gray-100">
                    Tests ({selectedTests.length})
                  </h3>
                  <div className="space-y-2">
                    {selectedTests.map((test: SelectedTest) => (
                      <div
                        key={test.id}
                        className="flex items-center justify-between rounded-lg border border-gray-200 p-3 dark:border-gray-800"
                      >
                        <div>
                          <p className="font-medium">{test.testName}</p>
                          <p className="text-sm text-gray-600 dark:text-gray-400">
                            {test.testCode} • {test.category}
                          </p>
                        </div>
                        <p className="font-medium">₹{test.price}</p>
                      </div>
                    ))}
                  </div>
                  <div className="mt-3 flex items-center justify-between rounded-lg bg-blue-50 p-4 dark:bg-blue-950/30">
                    <p className="font-semibold text-gray-900 dark:text-gray-100">Total</p>
                    <p className="text-xl font-bold text-blue-600 dark:text-blue-400">
                      ₹{totalPrice.toFixed(2)}
                    </p>
                  </div>
                </div>
              </div>
            </Card>

            <div className="flex justify-between">
              <Button type="button" variant="outline" onClick={() => setStep(3)}>
                Previous
              </Button>
              <Button type="submit" disabled={loading}>
                {loading ? 'Creating Order...' : 'Create Order'}
              </Button>
            </div>
          </div>
        )}
      </form>
    </div>
  );
}

export default function CreateOrderPage() {
  return (
    <React.Suspense fallback={<SkeletonCard />}>
      <CreateOrderPageContent />
    </React.Suspense>
  );
}
