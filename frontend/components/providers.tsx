'use client';

import { ApolloProvider } from '@apollo/client/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import apolloClient from '@/lib/apollo-client';
import { useEffect } from 'react';
import { useAuthStore } from '@/lib/store';
import { NotificationProvider } from '@/components/notification-provider';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60 * 1000, // 1 minute
      refetchOnWindowFocus: false,
    },
  },
});

export function Providers({ children }: { children: React.ReactNode }) {
  const checkAuth = useAuthStore((state) => state.checkAuth);

  useEffect(() => {
    // Check authentication on mount
    checkAuth();
  }, [checkAuth]);

  return (
    <ApolloProvider client={apolloClient}>
      <QueryClientProvider client={queryClient}>
        {children}
        <NotificationProvider />
      </QueryClientProvider>
    </ApolloProvider>
  );
}
