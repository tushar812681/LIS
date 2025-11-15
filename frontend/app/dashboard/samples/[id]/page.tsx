'use client';

import * as React from 'react';
import { useParams, useRouter } from 'next/navigation';
import { format } from 'date-fns';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { FormModal } from '@/components/ui/form-modal';
import { Input } from '@/components/ui/input';
import { SampleStatusBadge } from '@/components/ui/status-badge';
import { SkeletonCard } from '@/components/ui/skeleton';
import {
  useSample,
  useReceiveSample,
  useRejectSample,
  useUpdateSampleLocation,
} from '@/lib/hooks';
import { useNotificationStore } from '@/lib/store';
import {
  ArrowLeft,
  TestTube,
  User,
  MapPin,
  Calendar,
  Clock,


  AlertTriangle,
  CheckCircle,
  XCircle,
  Edit,
  Printer,
  QrCode,
  History,
} from 'lucide-react';

export const dynamic = 'force-dynamic';

interface SampleType {
  id: string;
  sampleId: string;
  sampleType: string;
  containerType?: string;
  volume?: string;
  status: string;
  priority?: string;
  collectionDate: string;
  collectionTime: string;
  receivedDate?: string;
  receivedTime?: string;
  location?: string;
  storageConditions?: string;
  createdAt: string;
  updatedAt: string;
  patient: {
    id: string;
    patientId: string;
    firstName: string;
    lastName: string;
    dateOfBirth: string;
    gender: string;
  };
  order?: {
    id: string;
    orderId: string;
    tests?: Array<{
      id: string;
      testName: string;
      testCode: string;
    }>;
  };
  collectedBy?: {
    name: string;
  };
  receivedBy?: {
    name: string;
  };
  rejectionInfo?: {
    reason: string;
    notes?: string;
    rejectedBy: {
      name: string;
    };
    rejectedAt: string;
  };
  chainOfCustody?: Array<{
    action: string;
    timestamp: string;
    user: {
      name: string;
    };
    location?: string;
    notes?: string;
  }>;
}

export default function SampleDetailPage() {
  const params = useParams();
  const router = useRouter();
  const sampleId = params.id as string;

  const [mounted, setMounted] = React.useState(false);
  const [receiveModalOpen, setReceiveModalOpen] = React.useState(false);
  const [rejectModalOpen, setRejectModalOpen] = React.useState(false);
  const [locationModalOpen, setLocationModalOpen] = React.useState(false);
  const [rejectReason, setRejectReason] = React.useState('');
  const [rejectNotes, setRejectNotes] = React.useState('');
  const [newLocation, setNewLocation] = React.useState('');

  const { sample, loading, error, refetch } = useSample(sampleId);
  const { receiveSample, loading: receiving } = useReceiveSample();
  const { rejectSample, loading: rejecting } = useRejectSample();
  const { updateSampleLocation, loading: updatingLocation } = useUpdateSampleLocation();
  const addNotification = useNotificationStore((state) => state.addNotification);

  React.useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) return null;

  const handleReceive = async () => {
    try {
      await receiveSample({
        variables: {
          id: sampleId,
          input: {
            receivedDate: new Date().toISOString().split('T')[0],
            receivedTime: new Date().toTimeString().slice(0, 5),
          },
        },
      });

      addNotification({
        type: 'success',
        title: 'Sample Received',
        message: 'Sample has been received successfully',
      });
      setReceiveModalOpen(false);
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Receive Failed',
        message: error instanceof Error ? error.message : 'Failed to receive sample',
      });
    }
  };

  const handleReject = async () => {
    if (!rejectReason.trim()) {
      addNotification({
        type: 'error',
        title: 'Rejection Failed',
        message: 'Please provide a reason for rejection',
      });
      return;
    }

    try {
      await rejectSample({
        variables: {
          id: sampleId,
          reason: rejectReason,
          notes: rejectNotes || null,
        },
      });

      addNotification({
        type: 'success',
        title: 'Sample Rejected',
        message: 'Sample has been rejected',
      });
      setRejectModalOpen(false);
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Rejection Failed',
        message: error instanceof Error ? error.message : 'Failed to reject sample',
      });
    }
  };

  const handleUpdateLocation = async () => {
    if (!newLocation.trim()) {
      addNotification({
        type: 'error',
        title: 'Update Failed',
        message: 'Please provide a location',
      });
      return;
    }

    try {
      await updateSampleLocation({
        variables: {
          id: sampleId,
          location: newLocation,
        },
      });

      addNotification({
        type: 'success',
        title: 'Location Updated',
        message: 'Sample location has been updated',
      });
      setLocationModalOpen(false);
      refetch();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Update Failed',
        message: error instanceof Error ? error.message : 'Failed to update location',
      });
    }
  };

  if (loading) {
    return (
      <div className="space-y-6">
        <SkeletonCard />
        <SkeletonCard />
        <SkeletonCard />
      </div>
    );
  }

  if (error || !sample) {
    return (
      <div className="flex flex-col items-center justify-center py-12">
        <AlertTriangle className="h-12 w-12 text-red-500" />
        <h2 className="mt-4 text-lg font-semibold">Sample not found</h2>
        <p className="mt-2 text-sm text-gray-600">
          The sample you&apos;re looking for doesn&apos;t exist or has been deleted.
        </p>
        <Button onClick={() => router.push('/dashboard/samples')} className="mt-4">
          Back to Samples
        </Button>
      </div>
    );
  }

  // Type assertion after null check
  const typedSample = sample as SampleType;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => router.push('/dashboard/samples')}
          >
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back
          </Button>
          <div>
            <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-100">
              Sample {typedSample.sampleId}
            </h1>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Collected on {format(new Date(typedSample.collectionDate), 'MMM dd, yyyy')}
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <Button variant="outline">
            <QrCode className="mr-2 h-4 w-4" />
            Print Label
          </Button>
          {typedSample.status === 'COLLECTED' && (
            <Button onClick={() => setReceiveModalOpen(true)}>
              <CheckCircle className="mr-2 h-4 w-4" />
              Receive Sample
            </Button>
          )}
          {typedSample.status !== 'REJECTED' && typedSample.status !== 'COMPLETED' && (
            <Button variant="destructive" onClick={() => setRejectModalOpen(true)}>
              <XCircle className="mr-2 h-4 w-4" />
              Reject
            </Button>
          )}
        </div>
      </div>

      {/* Status Card */}
      <Card className="p-6">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm text-gray-600 dark:text-gray-400">Sample Status</p>
            <SampleStatusBadge status={typedSample.status} />
          </div>
          {typedSample.priority && (
            <div className="text-right">
              <p className="text-sm text-gray-600 dark:text-gray-400">Priority</p>
              <p className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                {typedSample.priority}
              </p>
            </div>
          )}
        </div>
      </Card>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Main Content */}
        <div className="space-y-6 lg:col-span-2">
          {/* Patient Information */}
          <Card className="p-6">
            <h2 className="mb-4 text-lg font-semibold">Patient Information</h2>
            <div className="flex items-start gap-4">
              <div className="flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                <User className="h-6 w-6 text-blue-600 dark:text-blue-300" />
              </div>
              <div className="flex-1">
                <p className="font-medium text-gray-900 dark:text-gray-100">
                  {typedSample.patient.firstName} {typedSample.patient.lastName}
                </p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Patient ID: {typedSample.patient.patientId}
                </p>
                <div className="mt-2 grid gap-2 text-sm md:grid-cols-2">
                  <div>
                    <span className="text-gray-600 dark:text-gray-400">DOB: </span>
                    <span className="font-medium">
                      {format(new Date(typedSample.patient.dateOfBirth), 'MMM dd, yyyy')}
                    </span>
                  </div>
                  <div>
                    <span className="text-gray-600 dark:text-gray-400">Gender: </span>
                    <span className="font-medium">{typedSample.patient.gender}</span>
                  </div>
                </div>
              </div>
              <Button
                variant="outline"
                size="sm"
                onClick={() => router.push(`/dashboard/patients/${typedSample.patient.id}`)}
              >
                View Profile
              </Button>
            </div>
          </Card>

          {/* Order & Test Information */}
          {typedSample.order && (
            <Card className="p-6">
              <h2 className="mb-4 text-lg font-semibold">Order & Test Information</h2>
              <div className="space-y-4">
                <div className="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-800">
                  <div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">Order ID</p>
                    <p className="font-medium text-gray-900 dark:text-gray-100">
                      {typedSample.order!.orderId}
                    </p>
                  </div>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => router.push(`/dashboard/orders/${typedSample.order!.id}`)}
                  >
                    View Order
                  </Button>
                </div>

                {typedSample.order!.tests && typedSample.order!.tests.length > 0 && (
                  <div>
                    <p className="mb-2 text-sm font-medium text-gray-600 dark:text-gray-400">
                      Associated Tests
                    </p>
                    <div className="space-y-2">
                      {typedSample.order!.tests.map((test: { id: string; testName: string; testCode: string }) => (
                        <div
                          key={test.id}
                          className="flex items-center gap-3 rounded-lg border border-gray-200 p-3 dark:border-gray-800"
                        >
                          <TestTube className="h-5 w-5 text-gray-400" />
                          <div>
                            <p className="font-medium text-gray-900 dark:text-gray-100">
                              {test.testName}
                            </p>
                            <p className="text-sm text-gray-600 dark:text-gray-400">
                              {test.testCode}
                            </p>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            </Card>
          )}

          {/* Sample Details */}
          <Card className="p-6">
            <h2 className="mb-4 text-lg font-semibold">Sample Details</h2>
            <div className="grid gap-4 md:grid-cols-2">
              <div>
                <p className="text-sm text-gray-600 dark:text-gray-400">Sample Type</p>
                <p className="font-medium text-gray-900 dark:text-gray-100">
                  {typedSample.sampleType}
                </p>
              </div>

              {typedSample.containerType && (
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Container Type</p>
                  <p className="font-medium text-gray-900 dark:text-gray-100">
                    {typedSample.containerType}
                  </p>
                </div>
              )}

              {typedSample.volume && (
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Volume</p>
                  <p className="font-medium text-gray-900 dark:text-gray-100">
                    {typedSample.volume}
                  </p>
                </div>
              )}

              <div>
                <p className="text-sm text-gray-600 dark:text-gray-400">Collection Date/Time</p>
                <p className="font-medium text-gray-900 dark:text-gray-100">
                  {format(new Date(typedSample.collectionDate), 'MMM dd, yyyy')} at{' '}
                  {typedSample.collectionTime}
                </p>
              </div>

              {typedSample.receivedDate && (
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Received Date/Time</p>
                  <p className="font-medium text-gray-900 dark:text-gray-100">
                    {format(new Date(typedSample.receivedDate), 'MMM dd, yyyy')} at{' '}
                    {typedSample.receivedTime}
                  </p>
                </div>
              )}

              {typedSample.location && (
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Current Location</p>
                  <div className="flex items-center gap-2">
                    <p className="font-medium text-gray-900 dark:text-gray-100">
                      {typedSample.location}
                    </p>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => {
                        setNewLocation(typedSample.location!);
                        setLocationModalOpen(true);
                      }}
                    >
                      <Edit className="h-3 w-3" />
                    </Button>
                  </div>
                </div>
              )}

              {typedSample.storageConditions && (
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Storage Conditions</p>
                  <p className="font-medium text-gray-900 dark:text-gray-100">
                    {typedSample.storageConditions}
                  </p>
                </div>
              )}
            </div>
          </Card>

          {/* Collection Information */}
          {(typedSample.collectedBy || typedSample.receivedBy) && (
            <Card className="p-6">
              <h2 className="mb-4 text-lg font-semibold">Collection Information</h2>
              <div className="grid gap-4 md:grid-cols-2">
                {typedSample.collectedBy && (
                  <div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">Collected By</p>
                    <p className="font-medium text-gray-900 dark:text-gray-100">
                      {typedSample.collectedBy.name}
                    </p>
                  </div>
                )}

                {typedSample.receivedBy && (
                  <div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">Received By</p>
                    <p className="font-medium text-gray-900 dark:text-gray-100">
                      {typedSample.receivedBy.name}
                    </p>
                  </div>
                )}
              </div>
            </Card>
          )}

          {/* Rejection Info */}
          {typedSample.rejectionInfo && (
            <Card className="border-red-200 p-6 dark:border-red-900">
              <h2 className="mb-4 flex items-center gap-2 text-lg font-semibold text-red-600 dark:text-red-400">
                <XCircle className="h-5 w-5" />
                Rejection Information
              </h2>
              <div className="space-y-3">
                <div>
                  <p className="text-sm text-gray-600 dark:text-gray-400">Reason</p>
                  <p className="font-medium text-gray-900 dark:text-gray-100">
                    {typedSample.rejectionInfo.reason}
                  </p>
                </div>
                {typedSample.rejectionInfo.notes && (
                  <div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">Notes</p>
                    <p className="text-gray-900 dark:text-gray-100">
                      {typedSample.rejectionInfo.notes}
                    </p>
                  </div>
                )}
                <div className="grid gap-4 md:grid-cols-2">
                  <div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">Rejected By</p>
                    <p className="font-medium text-gray-900 dark:text-gray-100">
                      {typedSample.rejectionInfo.rejectedBy.name}
                    </p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">Rejected At</p>
                    <p className="font-medium text-gray-900 dark:text-gray-100">
                      {format(new Date(typedSample.rejectionInfo.rejectedAt), 'MMM dd, yyyy HH:mm')}
                    </p>
                  </div>
                </div>
              </div>
            </Card>
          )}

          {/* Chain of Custody */}
          {typedSample.chainOfCustody && typedSample.chainOfCustody.length > 0 && (
            <Card className="p-6">
              <h2 className="mb-4 flex items-center gap-2 text-lg font-semibold">
                <History className="h-5 w-5" />
                Chain of Custody
              </h2>
              <div className="space-y-4">
                {typedSample.chainOfCustody!.map((custody: { action: string; timestamp: string; user: { name: string }; location?: string; notes?: string }, index: number) => (
                  <div key={index} className="flex gap-4">
                    <div className="flex flex-col items-center">
                      <div className="flex h-8 w-8 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900">
                        <div className="h-2 w-2 rounded-full bg-blue-600 dark:bg-blue-300" />
                      </div>
                      {index < typedSample.chainOfCustody!.length - 1 && (
                        <div className="h-full w-px bg-gray-200 dark:bg-gray-700" />
                      )}
                    </div>
                    <div className="flex-1 pb-4">
                      <p className="font-medium text-gray-900 dark:text-gray-100">
                        {custody.action}
                      </p>
                      <p className="text-sm text-gray-600 dark:text-gray-400">
                        {format(new Date(custody.timestamp), 'MMM dd, yyyy HH:mm')}
                      </p>
                      <div className="mt-1 text-sm">
                        <span className="text-gray-600 dark:text-gray-400">By: </span>
                        <span className="font-medium text-gray-900 dark:text-gray-100">
                          {custody.user.name}
                        </span>
                      </div>
                      {custody.location && (
                        <div className="mt-1 text-sm">
                          <span className="text-gray-600 dark:text-gray-400">Location: </span>
                          <span className="font-medium text-gray-900 dark:text-gray-100">
                            {custody.location}
                          </span>
                        </div>
                      )}
                      {custody.notes && (
                        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                          {custody.notes}
                        </p>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </Card>
          )}
        </div>

        {/* Sidebar */}
        <div className="space-y-6">
          {/* Quick Info */}
          <Card className="p-6">
            <h3 className="mb-4 font-semibold">Quick Info</h3>
            <div className="space-y-3 text-sm">
              <div className="flex items-center gap-2">
                <Calendar className="h-4 w-4 text-gray-400" />
                <div>
                  <p className="text-gray-600 dark:text-gray-400">Created</p>
                  <p className="font-medium">
                    {format(new Date(typedSample.createdAt), 'MMM dd, yyyy')}
                  </p>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <Clock className="h-4 w-4 text-gray-400" />
                <div>
                  <p className="text-gray-600 dark:text-gray-400">Last Updated</p>
                  <p className="font-medium">
                    {format(new Date(typedSample.updatedAt), 'MMM dd, yyyy HH:mm')}
                  </p>
                </div>
              </div>

              {typedSample.location && (
                <div className="flex items-center gap-2">
                  <MapPin className="h-4 w-4 text-gray-400" />
                  <div>
                    <p className="text-gray-600 dark:text-gray-400">Location</p>
                    <p className="font-medium">{typedSample.location}</p>
                  </div>
                </div>
              )}
            </div>
          </Card>

          {/* Quick Actions */}
          <Card className="p-6">
            <h3 className="mb-4 font-semibold">Quick Actions</h3>
            <div className="space-y-2">
              <Button
                variant="outline"
                className="w-full justify-start"
                onClick={() => setLocationModalOpen(true)}
              >
                <MapPin className="mr-2 h-4 w-4" />
                Update Location
              </Button>
              <Button variant="outline" className="w-full justify-start">
                <Printer className="mr-2 h-4 w-4" />
                Print Label
              </Button>
            </div>
          </Card>
        </div>
      </div>

      {/* Receive Modal */}
      <FormModal
        open={receiveModalOpen}
        onClose={() => setReceiveModalOpen(false)}
        title="Receive Sample"
        description="Confirm sample receipt at the laboratory"
        onSubmit={(e) => {
          e.preventDefault();
          handleReceive();
        }}
        submitLabel="Receive Sample"
        isSubmitting={receiving}
      >
        <p className="text-sm text-gray-600 dark:text-gray-400">
          You are about to receive sample <strong>{typedSample.sampleId}</strong>. This action will
          update the sample status and record the receipt time.
        </p>
      </FormModal>

      {/* Reject Modal */}
      <FormModal
        open={rejectModalOpen}
        onClose={() => setRejectModalOpen(false)}
        title="Reject Sample"
        description="Provide a reason for rejecting this sample"
        onSubmit={(e) => {
          e.preventDefault();
          handleReject();
        }}
        submitLabel="Reject Sample"
        isSubmitting={rejecting}
      >
        <div className="space-y-4">
          <div>
            <label className="mb-2 block text-sm font-medium">Rejection Reason *</label>
            <select
              value={rejectReason}
              onChange={(e) => setRejectReason(e.target.value)}
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
            >
              <option value="">Select reason</option>
              <option value="Hemolyzed">Hemolyzed</option>
              <option value="Clotted">Clotted</option>
              <option value="Insufficient Quantity">Insufficient Quantity</option>
              <option value="Incorrect Container">Incorrect Container</option>
              <option value="Unlabeled">Unlabeled</option>
              <option value="Mislabeled">Mislabeled</option>
              <option value="Damaged">Damaged</option>
              <option value="Expired">Expired</option>
              <option value="Other">Other</option>
            </select>
          </div>
          <div>
            <label className="mb-2 block text-sm font-medium">Additional Notes</label>
            <textarea
              value={rejectNotes}
              onChange={(e) => setRejectNotes(e.target.value)}
              rows={3}
              className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
              placeholder="Any additional details about the rejection..."
            />
          </div>
        </div>
      </FormModal>

      {/* Location Update Modal */}
      <FormModal
        open={locationModalOpen}
        onClose={() => setLocationModalOpen(false)}
        title="Update Location"
        description="Update the current storage location of this sample"
        onSubmit={(e) => {
          e.preventDefault();
          handleUpdateLocation();
        }}
        submitLabel="Update Location"
        isSubmitting={updatingLocation}
      >
        <div>
          <label className="mb-2 block text-sm font-medium">New Location *</label>
          <Input
            value={newLocation}
            onChange={(e) => setNewLocation(e.target.value)}
            placeholder="e.g., Refrigerator A - Shelf 2"
          />
        </div>
      </FormModal>
    </div>
  );
}
