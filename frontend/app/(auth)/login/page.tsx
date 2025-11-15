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
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { AuthService } from '@/lib/auth';
import { useAuthStore } from '@/lib/store';
import { useNotificationStore } from '@/lib/store';
import Link from 'next/link';
import { AlertCircle, Loader2 } from 'lucide-react';

export const dynamic = 'force-dynamic';

const LOGIN_MUTATION = gql`
  mutation Login($input: LoginInputGQL!) {
    login(input: $input) {
      accessToken
      refreshToken
      expiresIn
      permissions
      user {
        id
        userCode
        email
        firstName
        lastName
        userType
        organizationId
        userStatus
        emailVerified
        mobileVerified
      }
    }
  }
`;

interface LoginResponse {
  login: {
    accessToken: string;
    refreshToken: string;
    expiresIn: number;
    permissions: string[];
    user: {
      id: string;
      userCode: string;
      email: string;
      firstName: string;
      lastName: string;
      userType: string;
      organizationId: string;
      userStatus: string;
      emailVerified: boolean;
      mobileVerified: boolean;
    };
  };
}

const loginSchema = z.object({
  email: z.string().email({ message: 'Please enter a valid email address' }),
  password: z.string().min(1, { message: 'Password is required' }),
  rememberMe: z.boolean().optional(),
});

type LoginFormValues = z.infer<typeof loginSchema>;

export default function LoginPage() {
  const router = useRouter();
  const [error, setError] = useState<string | null>(null);
  const setUser = useAuthStore((state) => state.setUser);
  const addNotification = useNotificationStore((state) => state.addNotification);

  const [login, { loading }] = useMutation<LoginResponse>(LOGIN_MUTATION, {
    onCompleted: (data) => {
      // Store authentication tokens
      AuthService.setToken(data.login.accessToken);
      localStorage.setItem('refreshToken', data.login.refreshToken);

      // Map GraphQL response to User interface
      const user = {
        id: data.login.user.id,
        email: data.login.user.email,
        name: `${data.login.user.firstName} ${data.login.user.lastName}`,
        organization_id: data.login.user.organizationId,
        roles: [data.login.user.userType], // userType instead of role
        permissions: data.login.permissions,
      };

      AuthService.setUser(user);
      setUser(user);

      addNotification({
        type: 'success',
        title: 'Login Successful',
        message: `Welcome back, ${user.name}!`,
      });

      // Redirect based on userType
      const userType = data.login.user.userType;
      if (userType === 'SUPER_ADMIN' || userType === 'ORG_ADMIN') {
        router.push('/dashboard/admin');
      } else if (userType === 'TECHNICIAN' || userType === 'LAB_ASSISTANT') {
        router.push('/dashboard/lab');
      } else if (userType === 'DOCTOR') {
        router.push('/dashboard/doctor');
      } else if (userType === 'PATIENT') {
        router.push('/dashboard/patient');
      } else {
        router.push('/dashboard');
      }
    },
    onError: (err) => {
      console.error('Login error:', err);
      setError(err.message || 'Invalid email or password. Please try again.');
    },
  });

  const form = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      email: '',
      password: '',
      rememberMe: false,
    },
  });

  const onSubmit = async (values: LoginFormValues) => {
    setError(null);
    await login({
      variables: {
        input: {
          email: values.email,
          password: values.password,
          deviceId: null,
          deviceName: navigator.userAgent.includes('Mobile') ? 'Mobile Browser' : 'Desktop Browser',
        },
      },
    });
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 px-4 py-12 dark:bg-gray-900">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-1">
          <CardTitle className="text-2xl font-bold text-center">Sign in</CardTitle>
          <CardDescription className="text-center">
            Enter your email and password to access your account
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
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <div className="flex items-center justify-between">
                      <FormLabel>Password</FormLabel>
                      <Link
                        href="/reset-password"
                        className="text-sm text-primary hover:underline"
                      >
                        Forgot password?
                      </Link>
                    </div>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="Enter your password"
                        autoComplete="current-password"
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
                name="rememberMe"
                render={({ field }) => (
                  <FormItem className="flex flex-row items-center space-x-2 space-y-0">
                    <FormControl>
                      <input
                        type="checkbox"
                        disabled={loading}
                        checked={field.value}
                        onChange={field.onChange}
                        className="h-4 w-4 rounded border-gray-300"
                      />
                    </FormControl>
                    <FormLabel className="text-sm font-normal">
                      Remember me for 30 days
                    </FormLabel>
                  </FormItem>
                )}
              />

              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Signing in...
                  </>
                ) : (
                  'Sign in'
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
        <CardFooter>
          <p className="text-center text-sm text-gray-600 dark:text-gray-400 w-full">
            Don&apos;t have an account?{' '}
            <Link href="/register" className="font-medium text-primary hover:underline">
              Sign up
            </Link>
          </p>
        </CardFooter>
      </Card>
    </div>
  );
}
