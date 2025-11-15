'use client';

import * as React from 'react';
import { Plus, X, Filter } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

export type FilterOperator =
  | 'equals'
  | 'not_equals'
  | 'contains'
  | 'not_contains'
  | 'starts_with'
  | 'ends_with'
  | 'greater_than'
  | 'less_than'
  | 'greater_than_or_equal'
  | 'less_than_or_equal'
  | 'in'
  | 'not_in'
  | 'between'
  | 'is_null'
  | 'is_not_null';

export interface FilterField {
  key: string;
  label: string;
  type: 'text' | 'number' | 'date' | 'select' | 'boolean';
  operators?: FilterOperator[];
  options?: Array<{ label: string; value: string | number | boolean }>;
}

export interface FilterCondition {
  id: string;
  field: string;
  operator: FilterOperator;
  value: unknown;
}

interface FilterBuilderProps {
  fields: FilterField[];
  filters: FilterCondition[];
  onChange: (filters: FilterCondition[]) => void;
  onApply?: (filters: FilterCondition[]) => void;
  className?: string;
  maxFilters?: number;
}

const operatorLabels: Record<FilterOperator, string> = {
  equals: 'Equals',
  not_equals: 'Not equals',
  contains: 'Contains',
  not_contains: 'Does not contain',
  starts_with: 'Starts with',
  ends_with: 'Ends with',
  greater_than: 'Greater than',
  less_than: 'Less than',
  greater_than_or_equal: 'Greater than or equal',
  less_than_or_equal: 'Less than or equal',
  in: 'In',
  not_in: 'Not in',
  between: 'Between',
  is_null: 'Is empty',
  is_not_null: 'Is not empty',
};

const defaultOperatorsByType: Record<
  FilterField['type'],
  FilterOperator[]
> = {
  text: ['equals', 'not_equals', 'contains', 'not_contains', 'starts_with', 'ends_with'],
  number: ['equals', 'not_equals', 'greater_than', 'less_than', 'greater_than_or_equal', 'less_than_or_equal', 'between'],
  date: ['equals', 'not_equals', 'greater_than', 'less_than', 'between'],
  select: ['equals', 'not_equals', 'in', 'not_in'],
  boolean: ['equals'],
};

export function FilterBuilder({
  fields,
  filters,
  onChange,
  onApply,
  className,
  maxFilters = 10,
}: FilterBuilderProps) {
  const addFilter = () => {
    if (filters.length >= maxFilters) return;

    const newFilter: FilterCondition = {
      id: `filter-${Date.now()}`,
      field: fields[0]?.key || '',
      operator: 'equals',
      value: '',
    };
    onChange([...filters, newFilter]);
  };

  const removeFilter = (id: string) => {
    onChange(filters.filter((f) => f.id !== id));
  };

  const updateFilter = (id: string, updates: Partial<FilterCondition>) => {
    onChange(
      filters.map((f) =>
        f.id === id
          ? {
              ...f,
              ...updates,
              // Reset value if field changes
              ...(updates.field && updates.field !== f.field ? { value: '' } : {}),
            }
          : f
      )
    );
  };

  const clearAllFilters = () => {
    onChange([]);
  };

  const getFieldConfig = (fieldKey: string): FilterField | undefined => {
    return fields.find((f) => f.key === fieldKey);
  };

  const getAvailableOperators = (fieldKey: string): FilterOperator[] => {
    const field = getFieldConfig(fieldKey);
    if (!field) return [];
    return field.operators || defaultOperatorsByType[field.type] || [];
  };

  const renderValueInput = (filter: FilterCondition) => {
    const field = getFieldConfig(filter.field);
    if (!field) return null;

    // No value input needed for these operators
    if (filter.operator === 'is_null' || filter.operator === 'is_not_null') {
      return null;
    }

    switch (field.type) {
      case 'select':
        return (
          <select
            value={String(filter.value || '')}
            onChange={(e) => updateFilter(filter.id, { value: e.target.value })}
            className="h-9 w-full rounded-md border border-gray-300 bg-white px-3 text-sm dark:border-gray-700 dark:bg-gray-800"
          >
            <option value="">Select...</option>
            {field.options?.map((option) => (
              <option key={String(option.value)} value={String(option.value)}>
                {option.label}
              </option>
            ))}
          </select>
        );

      case 'boolean':
        return (
          <select
            value={String(filter.value)}
            onChange={(e) => updateFilter(filter.id, { value: e.target.value === 'true' })}
            className="h-9 w-full rounded-md border border-gray-300 bg-white px-3 text-sm dark:border-gray-700 dark:bg-gray-800"
          >
            <option value="">Select...</option>
            <option value="true">Yes</option>
            <option value="false">No</option>
          </select>
        );

      case 'date':
        return (
          <Input
            type="date"
            value={String(filter.value || '')}
            onChange={(e) => updateFilter(filter.id, { value: e.target.value })}
            className="h-9"
          />
        );

      case 'number':
        return (
          <Input
            type="number"
            value={String(filter.value || '')}
            onChange={(e) => updateFilter(filter.id, { value: e.target.value })}
            placeholder="Enter value"
            className="h-9"
          />
        );

      case 'text':
      default:
        return (
          <Input
            type="text"
            value={String(filter.value || '')}
            onChange={(e) => updateFilter(filter.id, { value: e.target.value })}
            placeholder="Enter value"
            className="h-9"
          />
        );
    }
  };

  return (
    <div className={cn('space-y-4', className)}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Filter className="h-4 w-4 text-gray-500" />
          <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
            Filters {filters.length > 0 && `(${filters.length})`}
          </span>
        </div>
        {filters.length > 0 && (
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onClick={clearAllFilters}
            className="h-8 text-sm"
          >
            Clear all
          </Button>
        )}
      </div>

      {/* Filter List */}
      {filters.length > 0 && (
        <div className="space-y-3">
          {filters.map((filter) => {
            const availableOperators = getAvailableOperators(filter.field);
            return (
              <div
                key={filter.id}
                className="flex items-start gap-2 rounded-lg border border-gray-200 p-3 dark:border-gray-800"
              >
                {/* Field Select */}
                <select
                  value={filter.field}
                  onChange={(e) => updateFilter(filter.id, { field: e.target.value })}
                  className="h-9 w-1/3 rounded-md border border-gray-300 bg-white px-3 text-sm dark:border-gray-700 dark:bg-gray-800"
                >
                  {fields.map((field) => (
                    <option key={field.key} value={field.key}>
                      {field.label}
                    </option>
                  ))}
                </select>

                {/* Operator Select */}
                <select
                  value={filter.operator}
                  onChange={(e) =>
                    updateFilter(filter.id, { operator: e.target.value as FilterOperator })
                  }
                  className="h-9 w-1/3 rounded-md border border-gray-300 bg-white px-3 text-sm dark:border-gray-700 dark:bg-gray-800"
                >
                  {availableOperators.map((op) => (
                    <option key={op} value={op}>
                      {operatorLabels[op]}
                    </option>
                  ))}
                </select>

                {/* Value Input */}
                <div className="flex-1">{renderValueInput(filter)}</div>

                {/* Remove Button */}
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  onClick={() => removeFilter(filter.id)}
                  className="h-9 w-9 p-0"
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            );
          })}
        </div>
      )}

      {/* Add Filter Button */}
      {filters.length < maxFilters && (
        <Button
          type="button"
          variant="outline"
          size="sm"
          onClick={addFilter}
          className="w-full"
        >
          <Plus className="mr-2 h-4 w-4" />
          Add filter
        </Button>
      )}

      {/* Apply Button */}
      {onApply && filters.length > 0 && (
        <Button type="button" onClick={() => onApply(filters)} className="w-full">
          Apply filters
        </Button>
      )}
    </div>
  );
}
