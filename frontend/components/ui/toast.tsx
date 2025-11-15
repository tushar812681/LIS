'use client';

import * as React from 'react';
import { X, CheckCircle2, AlertCircle, Info, AlertTriangle } from 'lucide-react';
import { cn } from '@/lib/utils';

export interface ToastProps {
  id: string;
  type: 'success' | 'error' | 'info' | 'warning';
  title: string;
  message?: string;
  duration?: number;
  onClose: (id: string) => void;
}

const iconMap = {
  success: CheckCircle2,
  error: AlertCircle,
  info: Info,
  warning: AlertTriangle,
};

const colorMap = {
  success: {
    container: 'bg-green-50 border-green-200 dark:bg-green-950 dark:border-green-900',
    icon: 'text-green-600 dark:text-green-400',
    title: 'text-green-900 dark:text-green-300',
    message: 'text-green-700 dark:text-green-400',
  },
  error: {
    container: 'bg-red-50 border-red-200 dark:bg-red-950 dark:border-red-900',
    icon: 'text-red-600 dark:text-red-400',
    title: 'text-red-900 dark:text-red-300',
    message: 'text-red-700 dark:text-red-400',
  },
  info: {
    container: 'bg-blue-50 border-blue-200 dark:bg-blue-950 dark:border-blue-900',
    icon: 'text-blue-600 dark:text-blue-400',
    title: 'text-blue-900 dark:text-blue-300',
    message: 'text-blue-700 dark:text-blue-400',
  },
  warning: {
    container: 'bg-yellow-50 border-yellow-200 dark:bg-yellow-950 dark:border-yellow-900',
    icon: 'text-yellow-600 dark:text-yellow-400',
    title: 'text-yellow-900 dark:text-yellow-300',
    message: 'text-yellow-700 dark:text-yellow-400',
  },
};

export function Toast({ id, type, title, message, duration = 5000, onClose }: ToastProps) {
  const [isExiting, setIsExiting] = React.useState(false);
  const Icon = iconMap[type];
  const colors = colorMap[type];

  const handleClose = () => {
    setIsExiting(true);
    setTimeout(() => {
      onClose(id);
    }, 300); // Match animation duration
  };

  React.useEffect(() => {
    if (duration > 0) {
      const timer = setTimeout(() => {
        handleClose();
      }, duration);

      return () => clearTimeout(timer);
    }
  }, [duration, id, onClose, handleClose]);

  return (
    <div
      className={cn(
        'pointer-events-auto flex w-full max-w-md gap-3 rounded-lg border p-4 shadow-lg transition-all duration-300',
        colors.container,
        isExiting
          ? 'translate-x-full opacity-0'
          : 'translate-x-0 opacity-100'
      )}
    >
      <Icon className={cn('h-5 w-5 flex-shrink-0', colors.icon)} />
      <div className="flex-1">
        <p className={cn('font-medium', colors.title)}>{title}</p>
        {message && (
          <p className={cn('mt-1 text-sm', colors.message)}>{message}</p>
        )}
      </div>
      <button
        onClick={handleClose}
        className={cn(
          'flex-shrink-0 rounded-md p-1 transition-colors hover:bg-black/5 dark:hover:bg-white/5',
          colors.icon
        )}
      >
        <X className="h-4 w-4" />
      </button>
    </div>
  );
}

export function ToastContainer({ children }: { children: React.ReactNode }) {
  return (
    <div className="pointer-events-none fixed bottom-0 right-0 z-50 flex flex-col gap-3 p-6">
      {children}
    </div>
  );
}
