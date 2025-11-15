'use client';

import * as React from 'react';
import { Button } from '@/components/ui/button';
import {
  Modal,
  ModalHeader,
  ModalTitle,
  ModalDescription,
  ModalBody,
  ModalFooter,
} from '@/components/ui/modal';

interface FormModalProps {
  open: boolean;
  onClose: () => void;
  title: string;
  description?: string;
  children: React.ReactNode;
  onSubmit: (e: React.FormEvent) => void | Promise<void>;
  submitLabel?: string;
  cancelLabel?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  isLoading?: boolean;
  isSubmitting?: boolean;
  submitDisabled?: boolean;
  closeOnOverlayClick?: boolean;
}

export function FormModal({
  open,
  onClose,
  title,
  description,
  children,
  onSubmit,
  submitLabel = 'Submit',
  cancelLabel = 'Cancel',
  size = 'md',
  isLoading = false,
  isSubmitting = false,
  submitDisabled = false,
  closeOnOverlayClick = true,
}: FormModalProps) {
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await onSubmit(e);
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      size={size}
      closeOnOverlayClick={!isSubmitting && closeOnOverlayClick}
    >
      <form onSubmit={handleSubmit}>
        <ModalHeader>
          <ModalTitle>{title}</ModalTitle>
          {description && <ModalDescription>{description}</ModalDescription>}
        </ModalHeader>

        <ModalBody>
          {isLoading ? (
            <div className="flex items-center justify-center py-8">
              <div className="h-8 w-8 animate-spin rounded-full border-4 border-gray-300 border-t-blue-600" />
            </div>
          ) : (
            children
          )}
        </ModalBody>

        <ModalFooter>
          <Button
            type="button"
            variant="outline"
            onClick={onClose}
            disabled={isSubmitting}
          >
            {cancelLabel}
          </Button>
          <Button
            type="submit"
            disabled={isSubmitting || submitDisabled || isLoading}
          >
            {isSubmitting ? (
              <>
                <div className="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-white border-t-transparent" />
                Submitting...
              </>
            ) : (
              submitLabel
            )}
          </Button>
        </ModalFooter>
      </form>
    </Modal>
  );
}
