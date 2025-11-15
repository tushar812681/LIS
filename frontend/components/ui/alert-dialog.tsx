'use client';

import * as React from 'react';
import { AlertTriangle, Info, CheckCircle, XCircle } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  Modal,
  ModalHeader,
  ModalTitle,
  ModalDescription,
  ModalFooter,
} from '@/components/ui/modal';

interface AlertDialogProps {
  open: boolean;
  onClose: () => void;
  title: string;
  description?: string;
  variant?: 'info' | 'warning' | 'danger' | 'success';
  onConfirm?: () => void | Promise<void>;
  confirmLabel?: string;
  cancelLabel?: string;
  isConfirming?: boolean;
  showCancel?: boolean;
}

export function AlertDialog({
  open,
  onClose,
  title,
  description,
  variant = 'info',
  onConfirm,
  confirmLabel = 'Confirm',
  cancelLabel = 'Cancel',
  isConfirming = false,
  showCancel = true,
}: AlertDialogProps) {
  const handleConfirm = async () => {
    if (onConfirm) {
      await onConfirm();
    } else {
      onClose();
    }
  };

  const icons = {
    info: <Info className="h-6 w-6 text-blue-600" />,
    warning: <AlertTriangle className="h-6 w-6 text-yellow-600" />,
    danger: <XCircle className="h-6 w-6 text-red-600" />,
    success: <CheckCircle className="h-6 w-6 text-green-600" />,
  };

  const confirmVariants = {
    info: 'default' as const,
    warning: 'default' as const,
    danger: 'destructive' as const,
    success: 'default' as const,
  };

  return (
    <Modal open={open} onClose={onClose} size="sm" closeOnOverlayClick={!isConfirming}>
      <ModalHeader>
        <div className="flex items-center gap-3">
          {icons[variant]}
          <ModalTitle>{title}</ModalTitle>
        </div>
        {description && <ModalDescription className="mt-2">{description}</ModalDescription>}
      </ModalHeader>

      <ModalFooter>
        {showCancel && (
          <Button
            type="button"
            variant="outline"
            onClick={onClose}
            disabled={isConfirming}
          >
            {cancelLabel}
          </Button>
        )}
        <Button
          type="button"
          variant={confirmVariants[variant]}
          onClick={handleConfirm}
          disabled={isConfirming}
        >
          {isConfirming ? (
            <>
              <div className="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-white border-t-transparent" />
              Processing...
            </>
          ) : (
            confirmLabel
          )}
        </Button>
      </ModalFooter>
    </Modal>
  );
}

// Confirmation Dialog Hook for easier usage
export function useConfirmDialog() {
  const [isOpen, setIsOpen] = React.useState(false);
  const [config, setConfig] = React.useState<{
    title: string;
    description?: string;
    variant?: 'info' | 'warning' | 'danger' | 'success';
    onConfirm: () => void | Promise<void>;
  } | null>(null);
  const [isConfirming, setIsConfirming] = React.useState(false);

  const confirm = React.useCallback(
    (options: {
      title: string;
      description?: string;
      variant?: 'info' | 'warning' | 'danger' | 'success';
      onConfirm: () => void | Promise<void>;
    }) => {
      setConfig(options);
      setIsOpen(true);
    },
    []
  );

  const handleConfirm = async () => {
    if (config?.onConfirm) {
      setIsConfirming(true);
      try {
        await config.onConfirm();
        setIsOpen(false);
      } catch (error) {
        console.error('Confirmation error:', error);
      } finally {
        setIsConfirming(false);
      }
    }
  };

  const handleClose = () => {
    if (!isConfirming) {
      setIsOpen(false);
      setConfig(null);
    }
  };

  const ConfirmDialog = config ? (
    <AlertDialog
      open={isOpen}
      onClose={handleClose}
      title={config.title}
      description={config.description}
      variant={config.variant}
      onConfirm={handleConfirm}
      isConfirming={isConfirming}
    />
  ) : null;

  return {
    confirm,
    ConfirmDialog,
    isOpen,
    close: handleClose,
  };
}
