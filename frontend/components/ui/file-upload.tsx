'use client';

import * as React from 'react';
import { Upload, X, File, FileText, Image as ImageIcon } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';

interface FileUploadProps {
  value?: File | File[] | null;
  onChange: (files: File | File[] | null) => void;
  accept?: string;
  multiple?: boolean;
  maxSize?: number; // in MB
  maxFiles?: number;
  disabled?: boolean;
  className?: string;
  showPreview?: boolean;
  placeholder?: string;
}

export function FileUpload({
  value,
  onChange,
  accept,
  multiple = false,
  maxSize = 10, // 10MB default
  maxFiles = 5,
  disabled = false,
  className,
  showPreview = true,
  placeholder = 'Click to upload or drag and drop',
}: FileUploadProps) {
  const [isDragging, setIsDragging] = React.useState(false);
  const [previews, setPreviews] = React.useState<string[]>([]);
  const [error, setError] = React.useState<string>('');
  const inputRef = React.useRef<HTMLInputElement>(null);

  const files = React.useMemo(() => {
    if (!value) return [];
    return Array.isArray(value) ? value : [value];
  }, [value]);

  // Generate previews for image files
  React.useEffect(() => {
    if (!showPreview) return;

    const newPreviews: string[] = [];
    const imageFiles = files.filter((file) => file.type.startsWith('image/'));

    imageFiles.forEach((file) => {
      const reader = new FileReader();
      reader.onloadend = () => {
        newPreviews.push(reader.result as string);
        if (newPreviews.length === imageFiles.length) {
          setPreviews(newPreviews);
        }
      };
      reader.readAsDataURL(file);
    });

    return () => {
      previews.forEach((preview) => URL.revokeObjectURL(preview));
    };
  }, [files, showPreview, previews]);

  const validateFiles = (filesToValidate: FileList | File[]): File[] | null => {
    const fileArray = Array.from(filesToValidate);

    // Check max files
    if (multiple && maxFiles && fileArray.length > maxFiles) {
      setError(`Maximum ${maxFiles} files allowed`);
      return null;
    }

    // Check file size
    const oversizedFiles = fileArray.filter(
      (file) => file.size > maxSize * 1024 * 1024
    );
    if (oversizedFiles.length > 0) {
      setError(`File size must be less than ${maxSize}MB`);
      return null;
    }

    // Check file type
    if (accept) {
      const acceptedTypes = accept.split(',').map((type) => type.trim());
      const invalidFiles = fileArray.filter((file) => {
        const fileType = file.type;
        const fileExt = `.${file.name.split('.').pop()}`;
        return !acceptedTypes.some(
          (type) =>
            type === fileType ||
            type === fileExt ||
            (type.endsWith('/*') && fileType.startsWith(type.replace('/*', '')))
        );
      });

      if (invalidFiles.length > 0) {
        setError('Invalid file type');
        return null;
      }
    }

    setError('');
    return fileArray;
  };

  const handleFiles = (fileList: FileList | File[]) => {
    const validatedFiles = validateFiles(fileList);
    if (!validatedFiles) return;

    if (multiple) {
      onChange(validatedFiles);
    } else {
      onChange(validatedFiles[0] || null);
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (!disabled) {
      setIsDragging(true);
    }
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    if (disabled) return;

    const droppedFiles = e.dataTransfer.files;
    if (droppedFiles.length > 0) {
      handleFiles(droppedFiles);
    }
  };

  const handleClick = () => {
    if (!disabled && inputRef.current) {
      inputRef.current.click();
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFiles = e.target.files;
    if (selectedFiles && selectedFiles.length > 0) {
      handleFiles(selectedFiles);
    }
  };

  const handleRemove = (index: number) => {
    if (multiple) {
      const newFiles = files.filter((_, i) => i !== index);
      onChange(newFiles.length > 0 ? newFiles : null);
    } else {
      onChange(null);
    }
  };

  const getFileIcon = (file: File) => {
    if (file.type.startsWith('image/')) return <ImageIcon className="h-8 w-8" />;
    if (file.type.startsWith('text/')) return <FileText className="h-8 w-8" />;
    return <File className="h-8 w-8" />;
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  };

  return (
    <div className={cn('space-y-4', className)}>
      {/* Upload Area */}
      <div
        className={cn(
          'relative rounded-lg border-2 border-dashed p-6 transition-colors',
          isDragging
            ? 'border-blue-500 bg-blue-50 dark:bg-blue-950/20'
            : 'border-gray-300 dark:border-gray-700',
          disabled
            ? 'cursor-not-allowed opacity-50'
            : 'cursor-pointer hover:border-gray-400 dark:hover:border-gray-600'
        )}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        onClick={handleClick}
      >
        <input
          ref={inputRef}
          type="file"
          className="hidden"
          accept={accept}
          multiple={multiple}
          onChange={handleInputChange}
          disabled={disabled}
        />

        <div className="flex flex-col items-center justify-center space-y-2 text-center">
          <Upload
            className={cn(
              'h-10 w-10',
              isDragging ? 'text-blue-500' : 'text-gray-400'
            )}
          />
          <div className="space-y-1">
            <p className="text-sm font-medium text-gray-700 dark:text-gray-300">
              {placeholder}
            </p>
            <p className="text-xs text-gray-500 dark:text-gray-400">
              {accept && `Accepted: ${accept}`}
              {maxSize && ` • Max size: ${maxSize}MB`}
              {multiple && maxFiles && ` • Max files: ${maxFiles}`}
            </p>
          </div>
        </div>
      </div>

      {/* Error Message */}
      {error && (
        <div className="rounded-md bg-red-50 p-3 text-sm text-red-600 dark:bg-red-950/20 dark:text-red-400">
          {error}
        </div>
      )}

      {/* File List */}
      {files.length > 0 && (
        <div className="space-y-2">
          {files.map((file, index) => (
            <div
              key={`${file.name}-${index}`}
              className="flex items-center justify-between rounded-lg border border-gray-200 p-3 dark:border-gray-800"
            >
              <div className="flex items-center space-x-3">
                {/* Preview or Icon */}
                {showPreview && file.type.startsWith('image/') && previews[index] ? (
                  // eslint-disable-next-line @next/next/no-img-element
                  <img
                    src={previews[index]}
                    alt={file.name}
                    className="h-12 w-12 rounded object-cover"
                  />
                ) : (
                  <div className="text-gray-400">{getFileIcon(file)}</div>
                )}

                {/* File Info */}
                <div className="min-w-0 flex-1">
                  <p className="truncate text-sm font-medium text-gray-900 dark:text-gray-100">
                    {file.name}
                  </p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    {formatFileSize(file.size)}
                  </p>
                </div>
              </div>

              {/* Remove Button */}
              {!disabled && (
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  onClick={(e) => {
                    e.stopPropagation();
                    handleRemove(index);
                  }}
                  className="h-8 w-8 p-0"
                >
                  <X className="h-4 w-4" />
                </Button>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
