'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';
import {
  CheckCircle,
  XCircle,
  AlertCircle,
  Clock,
  Loader2,
  Circle,
  MinusCircle,
} from 'lucide-react';

export type StatusVariant =
  | 'success'
  | 'error'
  | 'warning'
  | 'info'
  | 'pending'
  | 'processing'
  | 'neutral'
  | 'default';

export type StatusSize = 'sm' | 'md' | 'lg';

interface StatusBadgeProps {
  variant?: StatusVariant;
  size?: StatusSize;
  children: React.ReactNode;
  showIcon?: boolean;
  className?: string;
  pulse?: boolean;
}

const variantStyles: Record<StatusVariant, string> = {
  success: 'bg-green-100 text-green-800 border-green-200 dark:bg-green-950/30 dark:text-green-400 dark:border-green-900',
  error: 'bg-red-100 text-red-800 border-red-200 dark:bg-red-950/30 dark:text-red-400 dark:border-red-900',
  warning: 'bg-yellow-100 text-yellow-800 border-yellow-200 dark:bg-yellow-950/30 dark:text-yellow-400 dark:border-yellow-900',
  info: 'bg-blue-100 text-blue-800 border-blue-200 dark:bg-blue-950/30 dark:text-blue-400 dark:border-blue-900',
  pending: 'bg-gray-100 text-gray-800 border-gray-200 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-700',
  processing: 'bg-purple-100 text-purple-800 border-purple-200 dark:bg-purple-950/30 dark:text-purple-400 dark:border-purple-900',
  neutral: 'bg-gray-100 text-gray-600 border-gray-200 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-700',
  default: 'bg-gray-100 text-gray-800 border-gray-200 dark:bg-gray-800 dark:text-gray-100 dark:border-gray-700',
};

const sizeStyles: Record<StatusSize, string> = {
  sm: 'px-2 py-0.5 text-xs',
  md: 'px-2.5 py-1 text-sm',
  lg: 'px-3 py-1.5 text-base',
};

const iconSizes: Record<StatusSize, string> = {
  sm: 'h-3 w-3',
  md: 'h-4 w-4',
  lg: 'h-5 w-5',
};

const variantIcons: Record<StatusVariant, React.ComponentType<{ className?: string }>> = {
  success: CheckCircle,
  error: XCircle,
  warning: AlertCircle,
  info: Circle,
  pending: Clock,
  processing: Loader2,
  neutral: MinusCircle,
  default: Circle,
};

export function StatusBadge({
  variant = 'default',
  size = 'md',
  children,
  showIcon = true,
  className,
  pulse = false,
}: StatusBadgeProps) {
  const Icon = variantIcons[variant];

  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 rounded-full border font-medium',
        variantStyles[variant],
        sizeStyles[size],
        pulse && 'animate-pulse',
        className
      )}
    >
      {showIcon && (
        <Icon
          className={cn(
            iconSizes[size],
            variant === 'processing' && 'animate-spin'
          )}
        />
      )}
      {children}
    </span>
  );
}

// Preset status badges for common use cases
export const SampleStatusBadge = ({ status }: { status: string }) => {
  const statusMap: Record<string, { variant: StatusVariant; label: string }> = {
    COLLECTED: { variant: 'success', label: 'Collected' },
    RECEIVED: { variant: 'info', label: 'Received' },
    PROCESSING: { variant: 'processing', label: 'Processing' },
    COMPLETED: { variant: 'success', label: 'Completed' },
    REJECTED: { variant: 'error', label: 'Rejected' },
    PENDING: { variant: 'pending', label: 'Pending' },
  };

  const config = statusMap[status] || { variant: 'neutral' as const, label: status };
  return <StatusBadge variant={config.variant}>{config.label}</StatusBadge>;
};

export const OrderStatusBadge = ({ status }: { status: string }) => {
  const statusMap: Record<string, { variant: StatusVariant; label: string }> = {
    PENDING: { variant: 'pending', label: 'Pending' },
    CONFIRMED: { variant: 'info', label: 'Confirmed' },
    IN_PROGRESS: { variant: 'processing', label: 'In Progress' },
    COMPLETED: { variant: 'success', label: 'Completed' },
    CANCELLED: { variant: 'error', label: 'Cancelled' },
  };

  const config = statusMap[status] || { variant: 'neutral' as const, label: status };
  return <StatusBadge variant={config.variant}>{config.label}</StatusBadge>;
};

export const ResultStatusBadge = ({ status }: { status: string }) => {
  const statusMap: Record<string, { variant: StatusVariant; label: string }> = {
    PENDING: { variant: 'pending', label: 'Pending' },
    ENTERED: { variant: 'info', label: 'Entered' },
    VERIFIED: { variant: 'success', label: 'Verified' },
    APPROVED: { variant: 'success', label: 'Approved' },
    REJECTED: { variant: 'error', label: 'Rejected' },
    CRITICAL: { variant: 'error', label: 'Critical' },
  };

  const config = statusMap[status] || { variant: 'neutral' as const, label: status };
  return <StatusBadge variant={config.variant}>{config.label}</StatusBadge>;
};

export const PaymentStatusBadge = ({ status }: { status: string }) => {
  const statusMap: Record<string, { variant: StatusVariant; label: string }> = {
    PAID: { variant: 'success', label: 'Paid' },
    UNPAID: { variant: 'error', label: 'Unpaid' },
    PARTIAL: { variant: 'warning', label: 'Partial' },
    REFUNDED: { variant: 'neutral', label: 'Refunded' },
    PENDING: { variant: 'pending', label: 'Pending' },
  };

  const config = statusMap[status] || { variant: 'neutral' as const, label: status };
  return <StatusBadge variant={config.variant}>{config.label}</StatusBadge>;
};
