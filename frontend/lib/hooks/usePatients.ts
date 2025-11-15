'use client';

import { useQuery, useMutation, useApolloClient } from '@apollo/client/react';
import {
  GET_PATIENTS,
  GET_PATIENT,
  SEARCH_PATIENTS,
} from '@/lib/graphql/queries';
import {
  CREATE_PATIENT,
  UPDATE_PATIENT,
  DELETE_PATIENT,
  MERGE_PATIENTS,
} from '@/lib/graphql/mutations';

// Types
interface Pagination {
  total: number;
  page: number;
  limit: number;
  totalPages: number;
}

// Patient list hook
export function usePatients(variables?: {
  page?: number;
  limit?: number;
  search?: string;
  filters?: Record<string, unknown>;
  sort?: Record<string, unknown>;
}) {
  const { data, loading, error, refetch, fetchMore } = useQuery(GET_PATIENTS, {
    variables,
    fetchPolicy: 'cache-and-network',
  });

  return {
    patients: (data as { patients?: { data?: unknown[] } })?.patients?.data || [],
    pagination: (data as { patients?: { pagination?: Pagination } })?.patients?.pagination,
    loading,
    error,
    refetch,
    fetchMore,
  };
}

// Single patient hook
export function usePatient(id: string) {
  const { data, loading, error, refetch } = useQuery(GET_PATIENT, {
    variables: { id },
    skip: !id,
    fetchPolicy: 'cache-and-network',
  });

  return {
    patient: (data as { patient?: unknown })?.patient,
    loading,
    error,
    refetch,
  };
}

// Patient search hook
export function usePatientSearch() {
  const [search, { data, loading }] = useMutation(SEARCH_PATIENTS);

  return {
    searchPatients: (query: string, limit?: number) =>
      search({ variables: { query, limit } }),
    results: (data as { searchPatients?: unknown[] })?.searchPatients || [],
    loading,
  };
}

// Create patient hook
export function useCreatePatient() {
  const client = useApolloClient();
  const [createPatient, { loading, error }] = useMutation(CREATE_PATIENT, {
    onCompleted: () => {
      // Refetch patients list
      client.refetchQueries({
        include: [GET_PATIENTS],
      });
    },
  });

  return {
    createPatient,
    loading,
    error,
  };
}

// Update patient hook
export function useUpdatePatient() {
  const [updatePatient, { loading, error }] = useMutation(UPDATE_PATIENT);

  return {
    updatePatient,
    loading,
    error,
  };
}

// Delete patient hook
export function useDeletePatient() {
  const client = useApolloClient();
  const [deletePatient, { loading, error }] = useMutation(DELETE_PATIENT, {
    onCompleted: () => {
      client.refetchQueries({
        include: [GET_PATIENTS],
      });
    },
  });

  return {
    deletePatient,
    loading,
    error,
  };
}

// Merge patients hook
export function useMergePatients() {
  const client = useApolloClient();
  const [mergePatients, { loading, error }] = useMutation(MERGE_PATIENTS, {
    onCompleted: () => {
      client.refetchQueries({
        include: [GET_PATIENTS],
      });
    },
  });

  return {
    mergePatients,
    loading,
    error,
  };
}
