'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';

interface SkeletonProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'text' | 'circular' | 'rectangular' | 'rounded';
  width?: string | number;
  height?: string | number;
  animation?: 'pulse' | 'wave' | 'none';
}

export function Skeleton({
  className,
  variant = 'rectangular',
  width,
  height,
  animation = 'pulse',
  ...props
}: SkeletonProps) {
  const variantClasses = {
    text: 'rounded',
    circular: 'rounded-full',
    rectangular: '',
    rounded: 'rounded-lg',
  };

  const animationClasses = {
    pulse: 'animate-pulse',
    wave: 'animate-shimmer',
    none: '',
  };

  const style: React.CSSProperties = {
    width: width ? (typeof width === 'number' ? `${width}px` : width) : undefined,
    height: height ? (typeof height === 'number' ? `${height}px` : height) : undefined,
  };

  return (
    <div
      className={cn(
        'bg-gray-200 dark:bg-gray-800',
        variantClasses[variant],
        animationClasses[animation],
        className
      )}
      style={style}
      {...props}
    />
  );
}

// Common skeleton patterns
export function SkeletonText({
  lines = 3,
  className,
}: {
  lines?: number;
  className?: string;
}) {
  return (
    <div className={cn('space-y-2', className)}>
      {Array.from({ length: lines }).map((_, i) => (
        <Skeleton
          key={i}
          variant="text"
          height={16}
          className={i === lines - 1 ? 'w-3/4' : 'w-full'}
        />
      ))}
    </div>
  );
}

export function SkeletonCard({ className }: { className?: string }) {
  return (
    <div className={cn('rounded-lg border border-gray-200 p-4 dark:border-gray-800', className)}>
      <div className="space-y-3">
        <Skeleton variant="text" height={24} className="w-3/4" />
        <SkeletonText lines={2} />
        <div className="flex gap-2">
          <Skeleton variant="rounded" height={32} width={80} />
          <Skeleton variant="rounded" height={32} width={80} />
        </div>
      </div>
    </div>
  );
}

export function SkeletonTable({
  rows = 5,
  columns = 4,
  className,
}: {
  rows?: number;
  columns?: number;
  className?: string;
}) {
  return (
    <div className={cn('space-y-3', className)}>
      {/* Table Header */}
      <div className="flex gap-4">
        {Array.from({ length: columns }).map((_, i) => (
          <Skeleton key={`header-${i}`} variant="text" height={20} className="flex-1" />
        ))}
      </div>
      {/* Table Rows */}
      {Array.from({ length: rows }).map((_, rowIndex) => (
        <div key={`row-${rowIndex}`} className="flex gap-4">
          {Array.from({ length: columns }).map((_, colIndex) => (
            <Skeleton
              key={`cell-${rowIndex}-${colIndex}`}
              variant="text"
              height={16}
              className="flex-1"
            />
          ))}
        </div>
      ))}
    </div>
  );
}

export function SkeletonAvatar({
  size = 40,
  className,
}: {
  size?: number;
  className?: string;
}) {
  return (
    <Skeleton
      variant="circular"
      width={size}
      height={size}
      className={className}
    />
  );
}

export function SkeletonUserCard({ className }: { className?: string }) {
  return (
    <div className={cn('flex items-center gap-4', className)}>
      <SkeletonAvatar size={48} />
      <div className="flex-1 space-y-2">
        <Skeleton variant="text" height={20} className="w-1/2" />
        <Skeleton variant="text" height={16} className="w-3/4" />
      </div>
    </div>
  );
}

export function SkeletonForm({
  fields = 4,
  className,
}: {
  fields?: number;
  className?: string;
}) {
  return (
    <div className={cn('space-y-4', className)}>
      {Array.from({ length: fields }).map((_, i) => (
        <div key={i} className="space-y-2">
          <Skeleton variant="text" height={16} width={120} />
          <Skeleton variant="rounded" height={40} className="w-full" />
        </div>
      ))}
      <div className="flex justify-end gap-2 pt-4">
        <Skeleton variant="rounded" height={40} width={100} />
        <Skeleton variant="rounded" height={40} width={100} />
      </div>
    </div>
  );
}

export function SkeletonDashboardStats({ className }: { className?: string }) {
  return (
    <div className={cn('grid gap-4 md:grid-cols-2 lg:grid-cols-4', className)}>
      {Array.from({ length: 4 }).map((_, i) => (
        <div
          key={i}
          className="rounded-lg border border-gray-200 p-6 dark:border-gray-800"
        >
          <div className="space-y-2">
            <Skeleton variant="text" height={16} width={100} />
            <Skeleton variant="text" height={32} width={80} />
            <Skeleton variant="text" height={14} width={120} />
          </div>
        </div>
      ))}
    </div>
  );
}

export function SkeletonList({
  items = 5,
  className,
}: {
  items?: number;
  className?: string;
}) {
  return (
    <div className={cn('space-y-3', className)}>
      {Array.from({ length: items }).map((_, i) => (
        <div
          key={i}
          className="flex items-center gap-4 rounded-lg border border-gray-200 p-4 dark:border-gray-800"
        >
          <SkeletonAvatar size={40} />
          <div className="flex-1 space-y-2">
            <Skeleton variant="text" height={18} className="w-3/4" />
            <Skeleton variant="text" height={14} className="w-1/2" />
          </div>
          <Skeleton variant="rounded" height={32} width={80} />
        </div>
      ))}
    </div>
  );
}
