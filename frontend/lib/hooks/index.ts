'use client';

import { useQuery, useMutation, useApolloClient } from '@apollo/client/react';
import * as Queries from '@/lib/graphql/queries';
import * as Mutations from '@/lib/graphql/mutations';

// ============================================================================
// ORDERS HOOKS
// ============================================================================

export function useOrders(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_ORDERS, {
    variables,
    fetchPolicy: 'cache-and-network',
  });

  return {
    orders: (data as { orders?: { data?: unknown[] } })?.orders?.data || [],
    pagination: (data as { orders?: { pagination?: unknown } })?.orders?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useOrder(id: string) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_ORDER, {
    variables: { id },
    skip: !id,
  });

  return {
    order: (data as { order?: unknown })?.order,
    loading,
    error,
    refetch,
  };
}

export function useCreateOrder() {
  const client = useApolloClient();
  const [createOrder, { loading, error }] = useMutation(Mutations.CREATE_ORDER, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_ORDERS] });
    },
  });

  return { createOrder, loading, error };
}

export function useUpdateOrder() {
  const [updateOrder, { loading, error }] = useMutation(Mutations.UPDATE_ORDER);
  return { updateOrder, loading, error };
}

export function useCancelOrder() {
  const [cancelOrder, { loading, error }] = useMutation(Mutations.CANCEL_ORDER);
  return { cancelOrder, loading, error };
}

// ============================================================================
// SAMPLES HOOKS
// ============================================================================

export function useSamples(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_SAMPLES, {
    variables,
    fetchPolicy: 'cache-and-network',
  });

  return {
    samples: (data as { samples?: { data?: unknown[] } })?.samples?.data || [],
    pagination: (data as { samples?: { pagination?: unknown } })?.samples?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useSample(id: string) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_SAMPLE, {
    variables: { id },
    skip: !id,
  });

  return {
    sample: (data as { sample?: unknown })?.sample,
    loading,
    error,
    refetch,
  };
}

export function useCollectSample() {
  const client = useApolloClient();
  const [collectSample, { loading, error }] = useMutation(Mutations.COLLECT_SAMPLE, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_SAMPLES] });
    },
  });

  return { collectSample, loading, error };
}

export function useReceiveSample() {
  const [receiveSample, { loading, error }] = useMutation(Mutations.RECEIVE_SAMPLE);
  return { receiveSample, loading, error };
}

export function useRejectSample() {
  const [rejectSample, { loading, error }] = useMutation(Mutations.REJECT_SAMPLE);
  return { rejectSample, loading, error };
}

export function useUpdateSampleStatus() {
  const [updateSampleStatus, { loading, error }] = useMutation(Mutations.UPDATE_SAMPLE_STATUS);
  return { updateSampleStatus, loading, error };
}

export function useUpdateSampleLocation() {
  const [updateSampleLocation, { loading, error }] = useMutation(Mutations.UPDATE_SAMPLE_LOCATION);
  return { updateSampleLocation, loading, error };
}

// ============================================================================
// RESULTS HOOKS
// ============================================================================

export function useResults(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_RESULTS, {
    variables,
    fetchPolicy: 'cache-and-network',
  });

  return {
    results: (data as { results?: { data?: unknown[] } })?.results?.data || [],
    pagination: (data as { results?: { pagination?: unknown } })?.results?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useResult(id: string) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_RESULT, {
    variables: { id },
    skip: !id,
  });

  return {
    result: (data as { result?: unknown })?.result,
    loading,
    error,
    refetch,
  };
}

export function useEnterResult() {
  const client = useApolloClient();
  const [enterResult, { loading, error }] = useMutation(Mutations.ENTER_RESULT, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_RESULTS] });
    },
  });

  return { enterResult, loading, error };
}

export function useVerifyResult() {
  const [verifyResult, { loading, error }] = useMutation(Mutations.VERIFY_RESULT);
  return { verifyResult, loading, error };
}

export function useApproveResult() {
  const [approveResult, { loading, error }] = useMutation(Mutations.APPROVE_RESULT);
  return { approveResult, loading, error };
}

export function useRejectResult() {
  const [rejectResult, { loading, error }] = useMutation(Mutations.REJECT_RESULT);
  return { rejectResult, loading, error };
}

export function useBatchEnterResults() {
  const client = useApolloClient();
  const [batchEnterResults, { loading, error }] = useMutation(Mutations.BATCH_ENTER_RESULTS, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_RESULTS] });
    },
  });

  return { batchEnterResults, loading, error };
}

// ============================================================================
// REPORTS HOOKS
// ============================================================================

export function useReports(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_REPORTS, {
    variables,
    fetchPolicy: 'cache-and-network',
  });

  return {
    reports: (data as { reports?: { data?: unknown[] } })?.reports?.data || [],
    pagination: (data as { reports?: { pagination?: unknown } })?.reports?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useReport(id: string) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_REPORT, {
    variables: { id },
    skip: !id,
  });

  return {
    report: (data as { report?: unknown })?.report,
    loading,
    error,
    refetch,
  };
}

export function useGenerateReport() {
  const client = useApolloClient();
  const [generateReport, { loading, error }] = useMutation(Mutations.GENERATE_REPORT, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_REPORTS] });
    },
  });

  return { generateReport, loading, error };
}

export function useApproveReport() {
  const [approveReport, { loading, error }] = useMutation(Mutations.APPROVE_REPORT);
  return { approveReport, loading, error };
}

export function useDeliverReport() {
  const [deliverReport, { loading, error }] = useMutation(Mutations.DELIVER_REPORT);
  return { deliverReport, loading, error };
}

// ============================================================================
// TEST CATALOG HOOKS
// ============================================================================

export function useTestCatalog(variables?: { search?: string; category?: string }) {
  const { data, loading, error } = useQuery(Queries.GET_TEST_CATALOG, {
    variables,
  });

  return {
    tests: (data as { testCatalog?: unknown[] })?.testCatalog || [],
    loading,
    error,
  };
}

export function useTestCategories() {
  const { data, loading, error } = useQuery(Queries.GET_TEST_CATEGORIES);

  return {
    categories: (data as { testCategories?: unknown[] })?.testCategories || [],
    loading,
    error,
  };
}

// ============================================================================
// QC HOOKS
// ============================================================================

export function useQCRuns(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_QC_RUNS, {
    variables,
  });

  return {
    qcRuns: (data as { qcRuns?: { data?: unknown[] } })?.qcRuns?.data || [],
    pagination: (data as { qcRuns?: { pagination?: unknown } })?.qcRuns?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useQCStatistics(variables: {
  testId: string;
  materialId: string;
  dateRange?: { start: string; end: string };
}) {
  const { data, loading, error } = useQuery(Queries.GET_QC_STATISTICS, {
    variables,
    skip: !variables.testId || !variables.materialId,
  });

  return {
    statistics: (data as { qcStatistics?: unknown })?.qcStatistics,
    loading,
    error,
  };
}

export function useQCMaterials(testId?: string) {
  const { data, loading, error } = useQuery(Queries.GET_QC_MATERIALS, {
    variables: { testId },
  });

  return {
    materials: (data as { qcMaterials?: unknown[] })?.qcMaterials || [],
    loading,
    error,
  };
}

export function useEnterQCRun() {
  const client = useApolloClient();
  const [enterQCRun, { loading, error }] = useMutation(Mutations.ENTER_QC_RUN, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_QC_RUNS] });
    },
  });

  return { enterQCRun, loading, error };
}

export function useCreateQCRun() {
  const client = useApolloClient();
  const [createQCRun, { loading, error }] = useMutation(Mutations.CREATE_QC_RUN, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_QC_RUNS] });
    },
  });

  return { createQCRun, loading, error };
}

export function useDeleteQCRun() {
  const client = useApolloClient();
  const [deleteQCRun, { loading, error }] = useMutation(Mutations.DELETE_QC_RUN, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_QC_RUNS] });
    },
  });

  return { deleteQCRun, loading, error };
}

// ============================================================================
// EQUIPMENT HOOKS
// ============================================================================

export function useEquipment(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_EQUIPMENT, {
    variables,
  });

  return {
    equipment: (data as { equipment?: { data?: unknown[] } })?.equipment?.data || [],
    pagination: (data as { equipment?: { pagination?: unknown } })?.equipment?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useEquipmentDetail(id: string) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_EQUIPMENT_DETAIL, {
    variables: { id },
    skip: !id,
  });

  return {
    equipment: (data as { equipment?: unknown })?.equipment,
    loading,
    error,
    refetch,
  };
}

export function useCreateEquipment() {
  const client = useApolloClient();
  const [createEquipment, { loading, error }] = useMutation(Mutations.CREATE_EQUIPMENT, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_EQUIPMENT] });
    },
  });

  return { createEquipment, loading, error };
}

export function useUpdateEquipment() {
  const [updateEquipment, { loading, error }] = useMutation(Mutations.UPDATE_EQUIPMENT);
  return { updateEquipment, loading, error };
}

export function useLogMaintenance() {
  const [logMaintenance, { loading, error }] = useMutation(Mutations.LOG_MAINTENANCE);
  return { logMaintenance, loading, error };
}

export function useLogCalibration() {
  const [logCalibration, { loading, error }] = useMutation(Mutations.LOG_CALIBRATION);
  return { logCalibration, loading, error };
}

export function useScheduleMaintenance() {
  const client = useApolloClient();
  const [scheduleMaintenance, { loading, error }] = useMutation(Mutations.SCHEDULE_MAINTENANCE, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_EQUIPMENT] });
    },
  });

  return { scheduleMaintenance, loading, error };
}

export function useUpdateEquipmentStatus() {
  const [updateEquipmentStatus, { loading, error }] = useMutation(Mutations.UPDATE_EQUIPMENT_STATUS);
  return { updateEquipmentStatus, loading, error };
}

export function useDeleteEquipment() {
  const client = useApolloClient();
  const [deleteEquipment, { loading, error }] = useMutation(Mutations.DELETE_EQUIPMENT, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_EQUIPMENT] });
    },
  });

  return { deleteEquipment, loading, error };
}

// ============================================================================
// INVENTORY HOOKS
// ============================================================================

export function useInventory(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_INVENTORY, {
    variables,
  });

  return {
    inventory: (data as { inventory?: { data?: unknown[] } })?.inventory?.data || [],
    pagination: (data as { inventory?: { pagination?: unknown } })?.inventory?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useInventoryItem(id: string) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_INVENTORY_ITEM, {
    variables: { id },
    skip: !id,
  });

  return {
    item: (data as { inventoryItem?: unknown })?.inventoryItem,
    loading,
    error,
    refetch,
  };
}

export function useStockIn() {
  const client = useApolloClient();
  const [stockIn, { loading, error }] = useMutation(Mutations.STOCK_IN, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_INVENTORY] });
    },
  });

  return { stockIn, loading, error };
}

export function useStockOut() {
  const client = useApolloClient();
  const [stockOut, { loading, error }] = useMutation(Mutations.STOCK_OUT, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_INVENTORY] });
    },
  });

  return { stockOut, loading, error };
}

export function useCreateInventoryItem() {
  const client = useApolloClient();
  const [createInventoryItem, { loading, error }] = useMutation(Mutations.CREATE_INVENTORY_ITEM, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_INVENTORY] });
    },
  });

  return { createInventoryItem, loading, error };
}

export function useUpdateInventoryItem() {
  const [updateInventoryItem, { loading, error }] = useMutation(Mutations.UPDATE_INVENTORY_ITEM);
  return { updateInventoryItem, loading, error };
}

export function useRecordTransaction() {
  const client = useApolloClient();
  const [recordTransaction, { loading, error }] = useMutation(Mutations.RECORD_TRANSACTION, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_INVENTORY] });
    },
  });

  return { recordTransaction, loading, error };
}

export function useDeleteInventoryItem() {
  const client = useApolloClient();
  const [deleteInventoryItem, { loading, error }] = useMutation(Mutations.DELETE_INVENTORY_ITEM, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_INVENTORY] });
    },
  });

  return { deleteInventoryItem, loading, error };
}

// ============================================================================
// BILLING HOOKS
// ============================================================================

export function useInvoices(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_INVOICES, {
    variables,
  });

  return {
    invoices: (data as { invoices?: { data?: unknown[] } })?.invoices?.data || [],
    pagination: (data as { invoices?: { pagination?: unknown } })?.invoices?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useInvoice(id: string) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_INVOICE, {
    variables: { id },
    skip: !id,
  });

  return {
    invoice: (data as { invoice?: unknown })?.invoice,
    loading,
    error,
    refetch,
  };
}

export function useCreateInvoice() {
  const client = useApolloClient();
  const [createInvoice, { loading, error }] = useMutation(Mutations.CREATE_INVOICE, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_INVOICES] });
    },
  });

  return { createInvoice, loading, error };
}

export function useRecordPayment() {
  const [recordPayment, { loading, error }] = useMutation(Mutations.RECORD_PAYMENT);
  return { recordPayment, loading, error };
}

export function useDeleteInvoice() {
  const client = useApolloClient();
  const [deleteInvoice, { loading, error }] = useMutation(Mutations.DELETE_INVOICE, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_INVOICES] });
    },
  });

  return { deleteInvoice, loading, error };
}

// ============================================================================
// USER HOOKS
// ============================================================================

export function useUsers(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_USERS, {
    variables,
  });

  return {
    users: (data as { users?: { data?: unknown[] } })?.users?.data || [],
    pagination: (data as { users?: { pagination?: unknown } })?.users?.pagination,
    loading,
    error,
    refetch,
  };
}

export function useUser(id: string) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_USER, {
    variables: { id },
    skip: !id,
  });

  return {
    user: (data as { user?: unknown })?.user,
    loading,
    error,
    refetch,
  };
}

export function useCreateUser() {
  const client = useApolloClient();
  const [createUser, { loading, error }] = useMutation(Mutations.CREATE_USER, {
    onCompleted: () => {
      client.refetchQueries({ include: [Queries.GET_USERS] });
    },
  });

  return { createUser, loading, error };
}

export function useUpdateUser() {
  const [updateUser, { loading, error }] = useMutation(Mutations.UPDATE_USER);
  return { updateUser, loading, error };
}

// ============================================================================
// ORGANIZATION HOOKS
// ============================================================================

export function useOrganization() {
  const { data, loading, error, refetch } = useQuery(Queries.GET_ORGANIZATION);

  return {
    organization: (data as { organization?: unknown })?.organization,
    loading,
    error,
    refetch,
  };
}

export function useUpdateOrganization() {
  const [updateOrganization, { loading, error }] = useMutation(Mutations.UPDATE_ORGANIZATION);
  return { updateOrganization, loading, error };
}

// ============================================================================
// DASHBOARD HOOKS
// ============================================================================

export function useDashboardStats(dateRange?: { start: string; end: string }) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_DASHBOARD_STATS, {
    variables: { dateRange },
  });

  return {
    stats: (data as { dashboardStats?: unknown })?.dashboardStats,
    loading,
    error,
    refetch,
  };
}

// ============================================================================
// NOTIFICATION HOOKS
// ============================================================================

export function useNotifications(variables?: Record<string, unknown>) {
  const { data, loading, error, refetch } = useQuery(Queries.GET_NOTIFICATIONS, {
    variables,
    fetchPolicy: 'cache-and-network',
  });

  return {
    notifications: (data as { notifications?: { data?: unknown[] } })?.notifications?.data || [],
    pagination: (data as { notifications?: { pagination?: unknown } })?.notifications?.pagination,
    unreadCount: (data as { notifications?: { unreadCount?: number } })?.notifications?.unreadCount || 0,
    loading,
    error,
    refetch,
  };
}

export function useMarkNotificationRead() {
  const [markNotificationRead, { loading, error }] = useMutation(
    Mutations.MARK_NOTIFICATION_READ
  );
  return { markNotificationRead, loading, error };
}

export function useMarkAllNotificationsRead() {
  const client = useApolloClient();
  const [markAllNotificationsRead, { loading, error }] = useMutation(
    Mutations.MARK_ALL_NOTIFICATIONS_READ,
    {
      onCompleted: () => {
        client.refetchQueries({ include: [Queries.GET_NOTIFICATIONS] });
      },
    }
  );
  return { markAllNotificationsRead, loading, error };
}

// ============================================================================
// SEARCH HOOKS
// ============================================================================

export function useGlobalSearch() {
  const search = async (query: string) => {
    if (!query || query.length < 2) {
      return {
        patients: [],
        orders: [],
        samples: [],
        invoices: [],
      };
    }

    // In production, this would be a GraphQL query
    // For now, return empty arrays as placeholder
    try {
      // TODO: Implement GraphQL search query
      return {
        patients: [],
        orders: [],
        samples: [],
        invoices: [],
      };
    } catch (error) {
      console.error('Search error:', error);
      return {
        patients: [],
        orders: [],
        samples: [],
        invoices: [],
      };
    }
  };

  return { search };
}
