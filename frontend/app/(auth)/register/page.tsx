'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { gql } from '@apollo/client';
import { useMutation } from '@apollo/client/react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { PhoneInput } from '@/components/ui/phone-input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { useNotificationStore } from '@/lib/store';
import Link from 'next/link';
import { AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';

export const dynamic = 'force-dynamic';

// Register organization admin - creates both organization and user atomically
const REGISTER_ORG_ADMIN_MUTATION = gql`
  mutation RegisterOrgAdmin($input: RegisterOrgAdminInputGQL!) {
    registerOrgAdmin(input: $input) {
      id
      email
      firstName
      lastName
      userType
      organizationId
    }
  }
`;

interface RegisterResponse {
  registerOrgAdmin: {
    id: string;
    email: string;
    firstName: string;
    lastName: string;
    userType: string;
    organizationId: string;
  };
}

const registerSchema = z.object({
  firstName: z.string().min(2, { message: 'First name must be at least 2 characters' }),
  lastName: z.string().min(2, { message: 'Last name must be at least 2 characters' }),
  email: z.string().email({ message: 'Please enter a valid email address' }),
  phone: z.string().regex(/^\+[1-9]\d{7,14}$/, { message: 'Please enter a valid phone number' }),
  password: z
    .string()
    .min(8, { message: 'Password must be at least 8 characters' })
    .regex(/[A-Z]/, { message: 'Password must contain at least one uppercase letter' })
    .regex(/[a-z]/, { message: 'Password must contain at least one lowercase letter' })
    .regex(/[0-9]/, { message: 'Password must contain at least one number' })
    .regex(/[^A-Za-z0-9]/, { message: 'Password must contain at least one special character' }),
  confirmPassword: z.string(),
  organizationName: z.string().min(2, { message: 'Organization name is required' }),
  acceptTerms: z.boolean().refine((val) => val === true, {
    message: 'You must accept the terms and conditions',
  }),
}).refine((data) => data.password === data.confirmPassword, {
  message: "Passwords don't match",
  path: ['confirmPassword'],
});

// Helper function to extract mobile number without country code
const extractMobileNumber = (phoneWithCountryCode: string): string => {
  // Remove the + sign
  const cleaned = phoneWithCountryCode.replace(/^\+/, '');

  // Country codes and their lengths
  const countryCodes = [
    { code: '971', length: 3 }, // UAE
    { code: '91', length: 2 },  // India
    { code: '86', length: 2 },  // China
    { code: '82', length: 2 },  // South Korea
    { code: '81', length: 2 },  // Japan
    { code: '66', length: 2 },  // Thailand
    { code: '65', length: 2 },  // Singapore
    { code: '62', length: 2 },  // Indonesia
    { code: '61', length: 2 },  // Australia
    { code: '60', length: 2 },  // Malaysia
    { code: '55', length: 2 },  // Brazil
    { code: '52', length: 2 },  // Mexico
    { code: '49', length: 2 },  // Germany
    { code: '44', length: 2 },  // UK
    { code: '39', length: 2 },  // Italy
    { code: '34', length: 2 },  // Spain
    { code: '33', length: 2 },  // France
    { code: '27', length: 2 },  // South Africa
    { code: '7', length: 1 },   // Russia
    { code: '1', length: 1 },   // US/Canada
  ];

  // Sort by code length descending to match longer codes first
  countryCodes.sort((a, b) => b.length - a.length);

  for (const { code, length } of countryCodes) {
    if (cleaned.startsWith(code)) {
      return cleaned.substring(length);
    }
  }

  // If no country code found, return as is
  return cleaned;
};

type RegisterFormValues = z.infer<typeof registerSchema>;

export default function RegisterPage() {
  const router = useRouter();
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const addNotification = useNotificationStore((state) => state.addNotification);

  const [register, { loading }] = useMutation<RegisterResponse>(REGISTER_ORG_ADMIN_MUTATION, {
    onCompleted: () => {
      setSuccess(true);
      addNotification({
        type: 'success',
        title: 'Registration Successful',
        message: 'Your account has been created. Please sign in to continue.',
      });

      // Redirect to login after 2 seconds
      setTimeout(() => {
        router.push('/login');
      }, 2000);
    },
    onError: (err) => {
      console.error('Registration error:', err);
      setError(err.message || 'Registration failed. Please try again.');
    },
  });

  const form = useForm<RegisterFormValues>({
    resolver: zodResolver(registerSchema),
    defaultValues: {
      firstName: '',
      lastName: '',
      email: '',
      phone: '+91',
      password: '',
      confirmPassword: '',
      organizationName: '',
      acceptTerms: false,
    },
  });

  const onSubmit = async (values: RegisterFormValues) => {
    setError(null);

    // Extract mobile number without country code
    const mobileNumber = extractMobileNumber(values.phone);

    // Validate mobile number length (10-15 digits)
    if (mobileNumber.length < 10 || mobileNumber.length > 15) {
      setError('Mobile number must be 10-15 digits');
      return;
    }

    // Register organization admin - creates both organization and user
    await register({
      variables: {
        input: {
          firstName: values.firstName,
          lastName: values.lastName,
          email: values.email,
          mobileNumber: mobileNumber, // Just the digits, e.g., "7302587877"
          password: values.password,
          organizationName: values.organizationName, // Organization name
          organizationPhone: values.phone, // Full phone with country code
        },
      },
    });
  };

  if (success) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-gray-50 px-4 py-12 dark:bg-gray-900">
        <Card className="w-full max-w-md">
          <CardHeader>
            <div className="mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-green-100 dark:bg-green-900">
              <CheckCircle2 className="h-6 w-6 text-green-600 dark:text-green-400" />
            </div>
            <CardTitle className="text-center">Registration Successful!</CardTitle>
            <CardDescription className="text-center">
              Your account and organization have been created successfully. Redirecting to login...
            </CardDescription>
          </CardHeader>
        </Card>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 px-4 py-12 dark:bg-gray-900">
      <Card className="w-full max-w-2xl">
        <CardHeader className="space-y-1">
          <CardTitle className="text-2xl font-bold text-center">Create an account</CardTitle>
          <CardDescription className="text-center">
            Enter your information to create your account and organization
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              {error && (
                <div className="flex items-center gap-2 rounded-lg border border-red-200 bg-red-50 p-3 text-sm text-red-800 dark:border-red-900 dark:bg-red-950 dark:text-red-300">
                  <AlertCircle className="h-4 w-4" />
                  <p>{error}</p>
                </div>
              )}

              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                <FormField
                  control={form.control}
                  name="firstName"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>First Name</FormLabel>
                      <FormControl>
                        <Input
                          placeholder="John"
                          disabled={loading}
                          {...field}
                        />
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
                      <FormLabel>Last Name</FormLabel>
                      <FormControl>
                        <Input
                          placeholder="Doe"
                          disabled={loading}
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>

              <FormField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Email</FormLabel>
                    <FormControl>
                      <Input
                        type="email"
                        placeholder="name@example.com"
                        autoComplete="email"
                        disabled={loading}
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="phone"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Phone Number</FormLabel>
                    <FormControl>
                      <PhoneInput
                        value={field.value}
                        onChange={field.onChange}
                        disabled={loading}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="organizationName"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Organization Name</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="My Laboratory"
                        disabled={loading}
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>
                      This will be the name of your laboratory or clinic
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                <FormField
                  control={form.control}
                  name="password"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Password</FormLabel>
                      <FormControl>
                        <Input
                          type="password"
                          placeholder="Create a strong password"
                          autoComplete="new-password"
                          disabled={loading}
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={form.control}
                  name="confirmPassword"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Confirm Password</FormLabel>
                      <FormControl>
                        <Input
                          type="password"
                          placeholder="Confirm your password"
                          autoComplete="new-password"
                          disabled={loading}
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>

              <FormField
                control={form.control}
                name="acceptTerms"
                render={({ field }) => (
                  <FormItem className="flex flex-row items-start space-x-3 space-y-0">
                    <FormControl>
                      <input
                        type="checkbox"
                        disabled={loading}
                        checked={field.value}
                        onChange={field.onChange}
                        className="mt-1 h-4 w-4 rounded border-gray-300"
                      />
                    </FormControl>
                    <div className="space-y-1 leading-none">
                      <FormLabel className="text-sm font-normal">
                        I agree to the{' '}
                        <Link href="/terms" className="text-primary hover:underline">
                          Terms of Service
                        </Link>{' '}
                        and{' '}
                        <Link href="/privacy" className="text-primary hover:underline">
                          Privacy Policy
                        </Link>
                      </FormLabel>
                      <FormMessage />
                    </div>
                  </FormItem>
                )}
              />

              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Creating account...
                  </>
                ) : (
                  'Create account'
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
        <CardFooter>
          <p className="text-center text-sm text-gray-600 dark:text-gray-400 w-full">
            Already have an account?{' '}
            <Link href="/login" className="font-medium text-primary hover:underline">
              Sign in
            </Link>
          </p>
        </CardFooter>
      </Card>
    </div>
  );
}
