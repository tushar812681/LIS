'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { DataTable } from '@/components/ui/data-table';
import { FormModal } from '@/components/ui/form-modal';
import { Modal, ModalHeader, ModalTitle, ModalBody, ModalFooter } from '@/components/ui/modal';
import { useConfirmDialog } from '@/components/ui/alert-dialog';
import { useNotificationStore } from '@/lib/store';
import { useEquipment, useCreateEquipment, useUpdateEquipment, useDeleteEquipment, useScheduleMaintenance } from '@/lib/hooks';
import { SkeletonTable } from '@/components/ui/skeleton';
import {
  Plus,
  Activity,
  AlertCircle,
  CheckCircle2,
  Wrench,
  Calendar,
  Settings
} from 'lucide-react';
import { ColumnDef } from '@tanstack/react-table';
import { DataTableColumnHeader, DataTableRowActions } from '@/components/ui/data-table';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { Input } from '@/components/ui/input';

export const dynamic = 'force-dynamic';

interface Equipment {
  id: string;
  name: string;
  manufacturer: string;
  model: string;
  serialNumber: string;
  equipmentType: string;
  status: 'OPERATIONAL' | 'MAINTENANCE' | 'OUT_OF_SERVICE' | 'CALIBRATION';
  location: string;
  purchaseDate: string;
  warrantyExpiry?: string;
  lastMaintenanceDate?: string;
  nextMaintenanceDate?: string;
  calibrationDue?: string;
}

const equipmentSchema = z.object({
  name: z.string().min(1, 'Equipment name is required'),
  manufacturer: z.string().min(1, 'Manufacturer is required'),
  model: z.string().min(1, 'Model is required'),
  serialNumber: z.string().min(1, 'Serial number is required'),
  equipmentType: z.string().min(1, 'Equipment type is required'),
  location: z.string().min(1, 'Location is required'),
  purchaseDate: z.string().min(1, 'Purchase date is required'),
  warrantyExpiry: z.string().optional(),
  notes: z.string().optional(),
});

const maintenanceSchema = z.object({
  equipmentId: z.string().min(1),
  maintenanceType: z.enum(['PREVENTIVE', 'CORRECTIVE', 'CALIBRATION']),
  scheduledDate: z.string().min(1, 'Scheduled date is required'),
  description: z.string().min(1, 'Description is required'),
  performedBy: z.string().optional(),
});

type EquipmentFormValues = z.infer<typeof equipmentSchema>;
type MaintenanceFormValues = z.infer<typeof maintenanceSchema>;

export default function EquipmentPage() {
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showMaintenanceModal, setShowMaintenanceModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [selectedEquipment, setSelectedEquipment] = useState<Equipment | null>(null);
  const addNotification = useNotificationStore((state) => state.addNotification);

  // GraphQL hooks
  const { equipment, loading, error, refetch } = useEquipment({});
  const { createEquipment, loading: creating } = useCreateEquipment();
  const { updateEquipment } = useUpdateEquipment();
  const { deleteEquipment } = useDeleteEquipment();
  const { scheduleMaintenance, loading: scheduling } = useScheduleMaintenance();
  const { confirm } = useConfirmDialog();

  // Form hooks - must be called before any early returns
  const form = useForm<EquipmentFormValues>({
    resolver: zodResolver(equipmentSchema),
    defaultValues: {
      name: '',
      manufacturer: '',
      model: '',
      serialNumber: '',
      equipmentType: '',
      location: '',
      purchaseDate: '',
      warrantyExpiry: '',
      notes: '',
    },
  });

  const maintenanceForm = useForm<MaintenanceFormValues>({
    resolver: zodResolver(maintenanceSchema),
    defaultValues: {
      equipmentId: '',
      maintenanceType: 'PREVENTIVE',
      scheduledDate: '',
      description: '',
      performedBy: '',
    },
  });

  const handleCreateEquipment = async (values: EquipmentFormValues) => {
    try {
      await createEquipment({
        variables: {
          input: {
            name: values.name,
            manufacturer: values.manufacturer,
            model: values.model,
            serialNumber: values.serialNumber,
            equipmentType: values.equipmentType,
            location: values.location,
            purchaseDate: values.purchaseDate,
            warrantyExpiry: values.warrantyExpiry,
            notes: values.notes,
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Equipment Added',
        message: 'Equipment has been registered successfully',
      });

      setShowCreateModal(false);
      form.reset();
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to add equipment',
      });
    }
  };

  const handleScheduleMaintenance = async (values: MaintenanceFormValues) => {
    try {
      await scheduleMaintenance({
        variables: {
          input: {
            equipmentId: values.equipmentId,
            maintenanceType: values.maintenanceType,
            scheduledDate: values.scheduledDate,
            description: values.description,
            performedBy: values.performedBy,
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Maintenance Scheduled',
        message: 'Maintenance has been scheduled successfully',
      });

      setShowMaintenanceModal(false);
      maintenanceForm.reset();
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: error instanceof Error ? error.message : 'Failed to schedule maintenance',
      });
    }
  };

  const handleUpdateStatus = async (equipmentId: string, status: string) => {
    confirm({
      title: 'Update Equipment Status',
      description: `Are you sure you want to change the equipment status to ${status}?`,
      variant: 'info',
      onConfirm: async () => {
        try {
          await updateEquipment({
            variables: {
              id: equipmentId,
              input: { status },
            },
          });
          addNotification({
            type: 'success',
            title: 'Status Updated',
            message: 'Equipment status has been updated',
          });
          refetch();
        } catch (error) {
          addNotification({
            type: 'error',
            title: 'Error',
            message: error instanceof Error ? error.message : 'Failed to update status',
          });
        }
      },
    });
  };

  const handleDelete = async (id: string) => {
    confirm({
      title: 'Delete Equipment',
      description: 'Are you sure you want to delete this equipment? This action cannot be undone.',
      variant: 'danger',
      onConfirm: async () => {
        try {
          await deleteEquipment({ variables: { id } });
          addNotification({
            type: 'success',
            title: 'Equipment Deleted',
            message: 'Equipment has been deleted',
          });
          refetch();
        } catch (error) {
          addNotification({
            type: 'error',
            title: 'Error',
            message: error instanceof Error ? error.message : 'Failed to delete equipment',
          });
        }
      },
    });
  };

  const handleViewDetails = (equipment: Equipment) => {
    setSelectedEquipment(equipment);
    setShowDetailModal(true);
  };

  const handleOpenMaintenanceModal = (equipment: Equipment) => {
    setSelectedEquipment(equipment);
    maintenanceForm.setValue('equipmentId', equipment.id);
    setShowMaintenanceModal(true);
  };

  const getStatusBadge = (status: string) => {
    const statusConfig = {
      OPERATIONAL: { icon: CheckCircle2, color: 'text-green-600', bg: 'bg-green-50 border-green-200', label: 'Operational' },
      MAINTENANCE: { icon: Wrench, color: 'text-orange-600', bg: 'bg-orange-50 border-orange-200', label: 'Maintenance' },
      OUT_OF_SERVICE: { icon: AlertCircle, color: 'text-red-600', bg: 'bg-red-50 border-red-200', label: 'Out of Service' },
      CALIBRATION: { icon: Settings, color: 'text-blue-600', bg: 'bg-blue-50 border-blue-200', label: 'Calibration' },
    };

    const config = statusConfig[status as keyof typeof statusConfig] || statusConfig.OPERATIONAL;
    const Icon = config.icon;

    return (
      <div className={`flex items-center gap-1 rounded-full border px-2 py-1 ${config.bg}`}>
        <Icon className={`h-3 w-3 ${config.color}`} />
        <span className={`text-xs font-medium ${config.color}`}>{config.label}</span>
      </div>
    );
  };

  const columns: ColumnDef<Equipment>[] = [
    {
      accessorKey: 'name',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Equipment Name" />,
      cell: ({ row }) => (
        <div>
          <p className="font-medium">{row.original.name}</p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {row.original.equipmentType}
          </p>
        </div>
      ),
    },
    {
      accessorKey: 'manufacturer',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Manufacturer" />,
      cell: ({ row }) => (
        <div>
          <p className="font-medium">{row.original.manufacturer}</p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {row.original.model}
          </p>
        </div>
      ),
    },
    {
      accessorKey: 'serialNumber',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Serial #" />,
      cell: ({ row }) => (
        <span className="font-mono text-sm">{row.original.serialNumber}</span>
      ),
    },
    {
      accessorKey: 'location',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Location" />,
    },
    {
      accessorKey: 'status',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Status" />,
      cell: ({ row }) => getStatusBadge(row.original.status),
    },
    {
      accessorKey: 'nextMaintenanceDate',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Next Maintenance" />,
      cell: ({ row }) => {
        const nextDate = row.original.nextMaintenanceDate;
        if (!nextDate) return <span className="text-sm text-gray-400">Not scheduled</span>;

        const date = new Date(nextDate);
        const today = new Date();
        const daysUntil = Math.ceil((date.getTime() - today.getTime()) / (1000 * 60 * 60 * 24));

        const color = daysUntil < 7 ? 'text-red-600' : daysUntil < 30 ? 'text-orange-600' : 'text-gray-600';

        return (
          <div className={`text-sm ${color}`}>
            {date.toLocaleDateString()}
            {daysUntil >= 0 && (
              <p className="text-xs">({daysUntil} days)</p>
            )}
          </div>
        );
      },
    },
    {
      id: 'actions',
      cell: ({ row }) => (
        <DataTableRowActions
          row={row}
          actions={[
            {
              label: 'View Details',
              onClick: () => handleViewDetails(row.original),
            },
            {
              label: 'Schedule Maintenance',
              onClick: () => handleOpenMaintenanceModal(row.original),
            },
            {
              label: 'Mark Operational',
              onClick: () => handleUpdateStatus(row.original.id, 'OPERATIONAL'),
            },
            {
              label: 'Mark Out of Service',
              onClick: () => handleUpdateStatus(row.original.id, 'OUT_OF_SERVICE'),
              variant: 'destructive',
            },
            {
              label: 'Delete',
              onClick: () => handleDelete(row.original.id),
              variant: 'destructive',
            },
          ]}
        />
      ),
    },
  ];

  // Calculate summary stats
  const typedEquipment = (equipment as Equipment[]) || [];
  const totalEquipment = typedEquipment.length;
  const operational = typedEquipment.filter(e => e.status === 'OPERATIONAL').length;
  const maintenance = typedEquipment.filter(e => e.status === 'MAINTENANCE').length;
  const outOfService = typedEquipment.filter(e => e.status === 'OUT_OF_SERVICE').length;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Equipment Management
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            Monitor and manage laboratory equipment and maintenance
          </p>
        </div>
        <Button onClick={() => setShowCreateModal(true)}>
          <Plus className="mr-2 h-4 w-4" />
          Add Equipment
        </Button>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-6 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Equipment</CardTitle>
            <Activity className="h-4 w-4 text-gray-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalEquipment}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Registered devices</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Operational</CardTitle>
            <CheckCircle2 className="h-4 w-4 text-green-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600">{operational}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              {totalEquipment > 0 ? ((operational / totalEquipment) * 100).toFixed(1) : 0}% uptime
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Under Maintenance</CardTitle>
            <Wrench className="h-4 w-4 text-orange-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-orange-600">{maintenance}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Scheduled maintenance</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Out of Service</CardTitle>
            <AlertCircle className="h-4 w-4 text-red-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">{outOfService}</div>
            <p className="text-xs text-gray-600 dark:text-gray-400">Requires attention</p>
          </CardContent>
        </Card>
      </div>

      {/* Equipment Table */}
      <Card>
        <CardHeader>
          <CardTitle>Equipment Inventory</CardTitle>
          <CardDescription>
            View and manage all laboratory equipment and maintenance schedules
          </CardDescription>
        </CardHeader>
        <CardContent>
          {loading ? (
            <SkeletonTable />
          ) : error ? (
            <div className="p-4 text-center text-red-600">
              Error loading equipment: {error.message}
            </div>
          ) : (
            <DataTable
              columns={columns}
              data={typedEquipment}
              searchKey="name"
              searchPlaceholder="Search by equipment name..."
            />
          )}
        </CardContent>
      </Card>

      {/* Create Equipment Modal */}
      <FormModal
        open={showCreateModal}
        onClose={() => {
          setShowCreateModal(false);
          form.reset();
        }}
        title="Add New Equipment"
        description="Register new laboratory equipment"
        onSubmit={form.handleSubmit(handleCreateEquipment)}
        submitLabel="Add Equipment"
        isLoading={creating}
      >
        <Form {...form}>
          <div className="space-y-4">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Equipment Name *</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., Hematology Analyzer" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="grid gap-4 md:grid-cols-2">
              <FormField
                control={form.control}
                name="manufacturer"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Manufacturer *</FormLabel>
                    <FormControl>
                      <Input placeholder="e.g., Siemens" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="model"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Model *</FormLabel>
                    <FormControl>
                      <Input placeholder="e.g., ADVIA 2120i" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <div className="grid gap-4 md:grid-cols-2">
              <FormField
                control={form.control}
                name="serialNumber"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Serial Number *</FormLabel>
                    <FormControl>
                      <Input placeholder="e.g., SN123456789" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="equipmentType"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Equipment Type *</FormLabel>
                    <FormControl>
                      <select
                        {...field}
                        className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                      >
                        <option value="">Select type...</option>
                        <option value="ANALYZER">Analyzer</option>
                        <option value="CENTRIFUGE">Centrifuge</option>
                        <option value="MICROSCOPE">Microscope</option>
                        <option value="INCUBATOR">Incubator</option>
                        <option value="REFRIGERATOR">Refrigerator</option>
                        <option value="AUTOCLAVE">Autoclave</option>
                        <option value="OTHER">Other</option>
                      </select>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="location"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Location *</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., Lab Room 2A" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="grid gap-4 md:grid-cols-2">
              <FormField
                control={form.control}
                name="purchaseDate"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Purchase Date *</FormLabel>
                    <FormControl>
                      <Input type="date" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="warrantyExpiry"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Warranty Expiry</FormLabel>
                    <FormControl>
                      <Input type="date" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="notes"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Notes</FormLabel>
                  <FormControl>
                    <textarea
                      {...field}
                      rows={3}
                      placeholder="Additional information..."
                      className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </Form>
      </FormModal>

      {/* Schedule Maintenance Modal */}
      <FormModal
        open={showMaintenanceModal}
        onClose={() => {
          setShowMaintenanceModal(false);
          maintenanceForm.reset();
        }}
        title="Schedule Maintenance"
        description={`Schedule maintenance for ${selectedEquipment?.name}`}
        onSubmit={maintenanceForm.handleSubmit(handleScheduleMaintenance)}
        submitLabel="Schedule"
        isLoading={scheduling}
      >
        <Form {...maintenanceForm}>
          <div className="space-y-4">
            <FormField
              control={maintenanceForm.control}
              name="maintenanceType"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Maintenance Type *</FormLabel>
                  <FormControl>
                    <select
                      {...field}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    >
                      <option value="PREVENTIVE">Preventive Maintenance</option>
                      <option value="CORRECTIVE">Corrective Maintenance</option>
                      <option value="CALIBRATION">Calibration</option>
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={maintenanceForm.control}
              name="scheduledDate"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Scheduled Date *</FormLabel>
                  <FormControl>
                    <Input type="date" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={maintenanceForm.control}
              name="description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description *</FormLabel>
                  <FormControl>
                    <textarea
                      {...field}
                      rows={3}
                      placeholder="Describe the maintenance work to be performed..."
                      className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={maintenanceForm.control}
              name="performedBy"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Performed By</FormLabel>
                  <FormControl>
                    <Input placeholder="Technician name or service company" {...field} />
                  </FormControl>
                  <FormDescription>
                    Optional: Name of person or company performing the maintenance
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </Form>
      </FormModal>

      {/* Equipment Detail Modal */}
      <Modal open={showDetailModal} onClose={() => setShowDetailModal(false)}>
        <ModalHeader>
          <ModalTitle>Equipment Details</ModalTitle>
        </ModalHeader>
        <ModalBody>
          {selectedEquipment && (
            <div className="space-y-4">
              <div>
                <h3 className="mb-3 font-semibold">Basic Information</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Name:</span>
                    <span className="font-medium">{selectedEquipment.name}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Type:</span>
                    <span className="font-medium">{selectedEquipment.equipmentType}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Status:</span>
                    {getStatusBadge(selectedEquipment.status)}
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Location:</span>
                    <span className="font-medium">{selectedEquipment.location}</span>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Manufacturer Details</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Manufacturer:</span>
                    <span className="font-medium">{selectedEquipment.manufacturer}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Model:</span>
                    <span className="font-medium">{selectedEquipment.model}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Serial Number:</span>
                    <span className="font-mono font-medium">{selectedEquipment.serialNumber}</span>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Purchase & Warranty</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Purchase Date:</span>
                    <span className="font-medium">
                      {new Date(selectedEquipment.purchaseDate).toLocaleDateString()}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Warranty Expiry:</span>
                    <span className="font-medium">
                      {selectedEquipment.warrantyExpiry
                        ? new Date(selectedEquipment.warrantyExpiry).toLocaleDateString()
                        : 'N/A'}
                    </span>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="mb-3 font-semibold">Maintenance Schedule</h3>
                <div className="space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-900">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Last Maintenance:</span>
                    <span className="font-medium">
                      {selectedEquipment.lastMaintenanceDate
                        ? new Date(selectedEquipment.lastMaintenanceDate).toLocaleDateString()
                        : 'Never'}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Next Maintenance:</span>
                    <span className="font-medium">
                      {selectedEquipment.nextMaintenanceDate
                        ? new Date(selectedEquipment.nextMaintenanceDate).toLocaleDateString()
                        : 'Not scheduled'}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Calibration Due:</span>
                    <span className="font-medium">
                      {selectedEquipment.calibrationDue
                        ? new Date(selectedEquipment.calibrationDue).toLocaleDateString()
                        : 'N/A'}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          )}
        </ModalBody>
        <ModalFooter>
          <Button variant="outline" onClick={() => setShowDetailModal(false)}>
            Close
          </Button>
          {selectedEquipment && (
            <Button onClick={() => {
              setShowDetailModal(false);
              handleOpenMaintenanceModal(selectedEquipment);
            }}>
              <Calendar className="mr-2 h-4 w-4" />
              Schedule Maintenance
            </Button>
          )}
        </ModalFooter>
      </Modal>
    </div>
  );
}
