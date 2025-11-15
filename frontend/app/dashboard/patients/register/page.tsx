'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { useNotificationStore } from '@/lib/store';
import { useCreatePatient } from '@/lib/hooks/usePatients';
import { ChevronLeft, ChevronRight, Check, Loader2 } from 'lucide-react';

// Schema for each step
const step1Schema = z.object({
  firstName: z.string().min(2, 'First name must be at least 2 characters'),
  middleName: z.string().optional(),
  lastName: z.string().min(2, 'Last name must be at least 2 characters'),
  dateOfBirth: z.string().min(1, 'Date of birth is required'),
  gender: z.enum(['MALE', 'FEMALE', 'OTHER'], { message: 'Please select a gender' }),
  bloodGroup: z.enum(['A+', 'A-', 'B+', 'B-', 'AB+', 'AB-', 'O+', 'O-', 'UNKNOWN'], { message: 'Please select blood group' }).optional(),
});

const step2Schema = z.object({
  phone: z.string().regex(/^\+?[1-9]\d{1,14}$/, 'Please enter a valid phone number'),
  email: z.string().email('Please enter a valid email').optional().or(z.literal('')),
  address: z.string().min(5, 'Address must be at least 5 characters'),
  city: z.string().min(2, 'City is required'),
  state: z.string().min(2, 'State is required'),
  postalCode: z.string().regex(/^\d{6}$/, 'Please enter a valid 6-digit postal code'),
  country: z.string().min(1, 'Country is required'),
});

const step3Schema = z.object({
  emergencyContactName: z.string().min(2, 'Emergency contact name is required'),
  emergencyContactPhone: z.string().regex(/^\+?[1-9]\d{1,14}$/, 'Please enter a valid phone number'),
  emergencyContactRelation: z.string().min(2, 'Relationship is required'),
  allergies: z.string().optional(),
  chronicConditions: z.string().optional(),
  currentMedications: z.string().optional(),
});

const step4Schema = z.object({
  hasInsurance: z.boolean(),
  insuranceProvider: z.string().optional(),
  insurancePolicyNumber: z.string().optional(),
  insuranceValidUntil: z.string().optional(),
});

const fullSchema = step1Schema.merge(step2Schema).merge(step3Schema).merge(step4Schema);

type FormValues = z.infer<typeof fullSchema>;

const steps = [
  { id: 1, title: 'Personal Information', description: 'Basic details' },
  { id: 2, title: 'Contact Information', description: 'Address and contact' },
  { id: 3, title: 'Medical History', description: 'Emergency contact & health' },
  { id: 4, title: 'Insurance', description: 'Insurance details' },
  { id: 5, title: 'Review', description: 'Review and submit' },
];

export default function PatientRegistrationPage() {
  const router = useRouter();
  const [currentStep, setCurrentStep] = useState(1);
  const [mounted, setMounted] = useState(false);
  const addNotification = useNotificationStore((state) => state.addNotification);

  useEffect(() => {
    setMounted(true);
  }, []);

  const { createPatient, loading } = useCreatePatient();

  const form = useForm<FormValues>({
    resolver: zodResolver(fullSchema),
    defaultValues: {
      firstName: '',
      middleName: '',
      lastName: '',
      dateOfBirth: '',
      gender: undefined,
      bloodGroup: 'UNKNOWN',
      phone: '',
      email: '',
      address: '',
      city: '',
      state: '',
      postalCode: '',
      country: 'India',
      emergencyContactName: '',
      emergencyContactPhone: '',
      emergencyContactRelation: '',
      allergies: '',
      chronicConditions: '',
      currentMedications: '',
      hasInsurance: false,
      insuranceProvider: '',
      insurancePolicyNumber: '',
      insuranceValidUntil: '',
    },
    mode: 'onChange',
  });

  const validateStep = async (step: number): Promise<boolean> => {
    let fields: (keyof FormValues)[] = [];

    switch (step) {
      case 1:
        fields = ['firstName', 'lastName', 'dateOfBirth', 'gender'];
        break;
      case 2:
        fields = ['phone', 'address', 'city', 'state', 'postalCode', 'country'];
        break;
      case 3:
        fields = ['emergencyContactName', 'emergencyContactPhone', 'emergencyContactRelation'];
        break;
      case 4:
        if (form.getValues('hasInsurance')) {
          fields = ['insuranceProvider', 'insurancePolicyNumber'];
        }
        break;
    }

    const result = await form.trigger(fields);
    return result;
  };

  const handleNext = async () => {
    const isValid = await validateStep(currentStep);
    if (isValid) {
      setCurrentStep((prev) => Math.min(prev + 1, steps.length));
    }
  };

  const handlePrevious = () => {
    setCurrentStep((prev) => Math.max(prev - 1, 1));
  };

  const onSubmit = async (values: FormValues) => {
    try {
      // Transform form data to match GraphQL input
      const input = {
        firstName: values.firstName,
        middleName: values.middleName || null,
        lastName: values.lastName,
        dateOfBirth: values.dateOfBirth,
        gender: values.gender,
        phone: values.phone,
        email: values.email || null,
        address: {
          street: values.address,
          city: values.city,
          state: values.state,
          postalCode: values.postalCode,
          country: values.country,
        },
        emergencyContact: {
          name: values.emergencyContactName,
          relationship: values.emergencyContactRelation,
          phone: values.emergencyContactPhone,
        },
        medicalHistory: {
          allergies: values.allergies || null,
          medications: values.currentMedications || null,
          conditions: values.chronicConditions || null,
          notes: null,
        },
        insuranceInfo: values.hasInsurance
          ? {
              provider: values.insuranceProvider,
              policyNumber: values.insurancePolicyNumber,
              groupNumber: null,
              validUntil: values.insuranceValidUntil,
            }
          : null,
      };

      const result = await createPatient({
        variables: { input },
      });

      if (result.data && (result.data as { createPatient?: unknown }).createPatient) {
        const patient = (result.data as { createPatient: { patientId: string } }).createPatient;
        addNotification({
          type: 'success',
          title: 'Patient Registered',
          message: `Patient ${patient.patientId} registered successfully!`,
        });
        router.push('/dashboard/patients');
      }
    } catch (err) {
      addNotification({
        type: 'error',
        title: 'Registration Failed',
        message: err instanceof Error ? err.message : 'Failed to register patient',
      });
    }
  };

  if (!mounted) {
    return null;
  }

  return (
    <div className="mx-auto max-w-4xl space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Patient Registration
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Complete all steps to register a new patient
        </p>
      </div>

      {/* Progress Steps */}
      <div className="flex items-center justify-between">
        {steps.map((step, index) => (
          <div key={step.id} className="flex flex-1 items-center">
            <div className="flex flex-col items-center">
              <div
                className={`flex h-10 w-10 items-center justify-center rounded-full border-2 transition-colors ${
                  currentStep > step.id
                    ? 'border-green-600 bg-green-600 text-white'
                    : currentStep === step.id
                    ? 'border-blue-600 bg-blue-600 text-white'
                    : 'border-gray-300 bg-white text-gray-400 dark:border-gray-700 dark:bg-gray-800'
                }`}
              >
                {currentStep > step.id ? (
                  <Check className="h-5 w-5" />
                ) : (
                  <span>{step.id}</span>
                )}
              </div>
              <div className="mt-2 text-center">
                <p className="text-xs font-medium text-gray-900 dark:text-white">
                  {step.title}
                </p>
                <p className="text-xs text-gray-500 dark:text-gray-500">
                  {step.description}
                </p>
              </div>
            </div>
            {index < steps.length - 1 && (
              <div
                className={`h-0.5 flex-1 ${
                  currentStep > step.id
                    ? 'bg-green-600'
                    : 'bg-gray-300 dark:bg-gray-700'
                }`}
              />
            )}
          </div>
        ))}
      </div>

      {/* Form */}
      <Card>
        <CardHeader>
          <CardTitle>{steps[currentStep - 1].title}</CardTitle>
          <CardDescription>{steps[currentStep - 1].description}</CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
              {/* Step 1: Personal Information */}
              {currentStep === 1 && (
                <div className="space-y-4">
                  <div className="grid gap-4 md:grid-cols-3">
                    <FormField
                      control={form.control}
                      name="firstName"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>First Name *</FormLabel>
                          <FormControl>
                            <Input placeholder="John" {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="middleName"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Middle Name</FormLabel>
                          <FormControl>
                            <Input placeholder="M." {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="lastName"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Last Name *</FormLabel>
                          <FormControl>
                            <Input placeholder="Doe" {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>

                  <div className="grid gap-4 md:grid-cols-3">
                    <FormField
                      control={form.control}
                      name="dateOfBirth"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Date of Birth *</FormLabel>
                          <FormControl>
                            <Input type="date" {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="gender"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Gender *</FormLabel>
                          <FormControl>
                            <select
                              {...field}
                              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                            >
                              <option value="">Select gender</option>
                              <option value="MALE">Male</option>
                              <option value="FEMALE">Female</option>
                              <option value="OTHER">Other</option>
                            </select>
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="bloodGroup"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Blood Group</FormLabel>
                          <FormControl>
                            <select
                              {...field}
                              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                            >
                              <option value="UNKNOWN">Unknown</option>
                              <option value="A+">A+</option>
                              <option value="A-">A-</option>
                              <option value="B+">B+</option>
                              <option value="B-">B-</option>
                              <option value="AB+">AB+</option>
                              <option value="AB-">AB-</option>
                              <option value="O+">O+</option>
                              <option value="O-">O-</option>
                            </select>
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>
                </div>
              )}

              {/* Step 2: Contact Information */}
              {currentStep === 2 && (
                <div className="space-y-4">
                  <div className="grid gap-4 md:grid-cols-2">
                    <FormField
                      control={form.control}
                      name="phone"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Phone Number *</FormLabel>
                          <FormControl>
                            <Input placeholder="+91 9876543210" {...field} />
                          </FormControl>
                          <FormDescription>Include country code</FormDescription>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="email"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Email</FormLabel>
                          <FormControl>
                            <Input type="email" placeholder="john.doe@example.com" {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>

                  <FormField
                    control={form.control}
                    name="address"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Address *</FormLabel>
                        <FormControl>
                          <Input placeholder="Street address" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  <div className="grid gap-4 md:grid-cols-4">
                    <FormField
                      control={form.control}
                      name="city"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>City *</FormLabel>
                          <FormControl>
                            <Input placeholder="Mumbai" {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="state"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>State *</FormLabel>
                          <FormControl>
                            <Input placeholder="Maharashtra" {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="postalCode"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Postal Code *</FormLabel>
                          <FormControl>
                            <Input placeholder="400001" {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="country"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Country</FormLabel>
                          <FormControl>
                            <Input {...field} disabled />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>
                </div>
              )}

              {/* Step 3: Medical History */}
              {currentStep === 3 && (
                <div className="space-y-4">
                  <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                    <h3 className="mb-4 font-medium text-gray-900 dark:text-white">
                      Emergency Contact
                    </h3>
                    <div className="grid gap-4 md:grid-cols-3">
                      <FormField
                        control={form.control}
                        name="emergencyContactName"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Name *</FormLabel>
                            <FormControl>
                              <Input placeholder="Jane Doe" {...field} />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="emergencyContactPhone"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Phone *</FormLabel>
                            <FormControl>
                              <Input placeholder="+91 9876543210" {...field} />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="emergencyContactRelation"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Relationship *</FormLabel>
                            <FormControl>
                              <Input placeholder="Spouse" {...field} />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>
                  </div>

                  <div className="rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                    <h3 className="mb-4 font-medium text-gray-900 dark:text-white">
                      Medical Information
                    </h3>
                    <div className="space-y-4">
                      <FormField
                        control={form.control}
                        name="allergies"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Known Allergies</FormLabel>
                            <FormControl>
                              <Input placeholder="Penicillin, Peanuts..." {...field} />
                            </FormControl>
                            <FormDescription>
                              List any known allergies (optional)
                            </FormDescription>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="chronicConditions"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Chronic Conditions</FormLabel>
                            <FormControl>
                              <Input placeholder="Diabetes, Hypertension..." {...field} />
                            </FormControl>
                            <FormDescription>
                              List any chronic conditions (optional)
                            </FormDescription>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="currentMedications"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Current Medications</FormLabel>
                            <FormControl>
                              <Input placeholder="Metformin, Aspirin..." {...field} />
                            </FormControl>
                            <FormDescription>
                              List current medications (optional)
                            </FormDescription>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>
                  </div>
                </div>
              )}

              {/* Step 4: Insurance */}
              {currentStep === 4 && (
                <div className="space-y-4">
                  <FormField
                    control={form.control}
                    name="hasInsurance"
                    render={({ field }) => (
                      <FormItem className="flex flex-row items-center space-x-3 space-y-0 rounded-md border border-gray-200 p-4 dark:border-gray-800">
                        <FormControl>
                          <input
                            type="checkbox"
                            checked={field.value}
                            onChange={field.onChange}
                            className="h-4 w-4 rounded"
                          />
                        </FormControl>
                        <div className="space-y-1 leading-none">
                          <FormLabel>Patient has health insurance</FormLabel>
                          <FormDescription>
                            Check if patient has insurance coverage
                          </FormDescription>
                        </div>
                      </FormItem>
                    )}
                  />

                  {form.watch('hasInsurance') && (
                    <div className="space-y-4 rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                      <div className="grid gap-4 md:grid-cols-2">
                        <FormField
                          control={form.control}
                          name="insuranceProvider"
                          render={({ field }) => (
                            <FormItem>
                              <FormLabel>Insurance Provider</FormLabel>
                              <FormControl>
                                <Input placeholder="Star Health" {...field} />
                              </FormControl>
                              <FormMessage />
                            </FormItem>
                          )}
                        />
                        <FormField
                          control={form.control}
                          name="insurancePolicyNumber"
                          render={({ field }) => (
                            <FormItem>
                              <FormLabel>Policy Number</FormLabel>
                              <FormControl>
                                <Input placeholder="POL123456" {...field} />
                              </FormControl>
                              <FormMessage />
                            </FormItem>
                          )}
                        />
                      </div>
                      <FormField
                        control={form.control}
                        name="insuranceValidUntil"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Valid Until</FormLabel>
                            <FormControl>
                              <Input type="date" {...field} />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>
                  )}
                </div>
              )}

              {/* Step 5: Review */}
              {currentStep === 5 && (
                <div className="space-y-4">
                  <div className="rounded-lg border border-gray-200 bg-gray-50 p-4 dark:border-gray-800 dark:bg-gray-800">
                    <h3 className="mb-4 font-medium text-gray-900 dark:text-white">
                      Review Information
                    </h3>
                    <div className="grid gap-4 md:grid-cols-2">
                      <div>
                        <p className="text-sm text-gray-600 dark:text-gray-400">Full Name</p>
                        <p className="font-medium text-gray-900 dark:text-white">
                          {form.getValues('firstName')} {form.getValues('middleName')}{' '}
                          {form.getValues('lastName')}
                        </p>
                      </div>
                      <div>
                        <p className="text-sm text-gray-600 dark:text-gray-400">Date of Birth</p>
                        <p className="font-medium text-gray-900 dark:text-white">
                          {form.getValues('dateOfBirth')}
                        </p>
                      </div>
                      <div>
                        <p className="text-sm text-gray-600 dark:text-gray-400">Gender</p>
                        <p className="font-medium text-gray-900 dark:text-white">
                          {form.getValues('gender')}
                        </p>
                      </div>
                      <div>
                        <p className="text-sm text-gray-600 dark:text-gray-400">Phone</p>
                        <p className="font-medium text-gray-900 dark:text-white">
                          {form.getValues('phone')}
                        </p>
                      </div>
                      <div className="md:col-span-2">
                        <p className="text-sm text-gray-600 dark:text-gray-400">Address</p>
                        <p className="font-medium text-gray-900 dark:text-white">
                          {form.getValues('address')}, {form.getValues('city')},{' '}
                          {form.getValues('state')} - {form.getValues('postalCode')}
                        </p>
                      </div>
                    </div>
                  </div>

                  <div className="rounded-lg border-2 border-dashed border-blue-200 bg-blue-50 p-4 dark:border-blue-900 dark:bg-blue-950/50">
                    <p className="text-sm text-blue-900 dark:text-blue-300">
                      Please review all information carefully before submitting. Click &quot;Submit&quot; to
                      register the patient.
                    </p>
                  </div>
                </div>
              )}

              {/* Navigation Buttons */}
              <div className="flex justify-between pt-4">
                <Button
                  type="button"
                  variant="outline"
                  onClick={handlePrevious}
                  disabled={currentStep === 1 || loading}
                >
                  <ChevronLeft className="mr-2 h-4 w-4" />
                  Previous
                </Button>

                {currentStep < steps.length ? (
                  <Button type="button" onClick={handleNext} disabled={loading}>
                    Next
                    <ChevronRight className="ml-2 h-4 w-4" />
                  </Button>
                ) : (
                  <Button type="submit" disabled={loading}>
                    {loading ? (
                      <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                        Registering...
                      </>
                    ) : (
                      'Submit'
                    )}
                  </Button>
                )}
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
