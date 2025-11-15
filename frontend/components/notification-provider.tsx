'use client';

import { useEffect } from 'react';
import { Toast, ToastContainer } from '@/components/ui/toast';
import { useNotificationStore } from '@/lib/store';
import { getWebSocketClient } from '@/lib/websocket-client';

/**
 * Global notification provider that renders toast notifications
 * and handles real-time notifications from WebSocket events
 */
export function NotificationProvider() {
  const notifications = useNotificationStore((state) => state.notifications);
  const removeNotification = useNotificationStore((state) => state.removeNotification);
  const addNotification = useNotificationStore((state) => state.addNotification);

  // Play alert sound for critical notifications
  const playAlertSound = () => {
    // Check if user has enabled sound notifications
    const soundEnabled = localStorage.getItem('notificationSound') !== 'false';
    if (soundEnabled && typeof Audio !== 'undefined') {
      try {
        const audio = new Audio('/sounds/alert.mp3');
        audio.volume = 0.5;
        audio.play().catch((e) => console.warn('Could not play alert sound:', e));
      } catch (error) {
        console.warn('Audio playback not supported:', error);
      }
    }
  };

  // Setup WebSocket listeners for real-time notifications
  useEffect(() => {
    const ws = getWebSocketClient();

    // Generic notification event
    const unsubscribeNotification = ws.on('notification.created', (event) => {
      const { title, message, type } = (event as { payload: { title: string; message: string; type?: 'success' | 'error' | 'warning' | 'info' } }).payload;
      addNotification({
        type: type || 'info',
        title,
        message,
      });
    });

    // Sample status change notifications
    const unsubscribeSample = ws.on('sample.status_changed', (event) => {
      const { sample_number, new_status } = (event as { payload: { sample_number: string; new_status: string } }).payload;
      addNotification({
        type: 'info',
        title: 'Sample Status Updated',
        message: `Sample ${sample_number} is now ${new_status}`,
      });
    });

    // Critical value alerts
    const unsubscribeCritical = ws.on('result.critical_value_detected', (event) => {
      const { patient_name, test_parameter, result_value, unit } = (event as { payload: { patient_name: string; test_parameter: string; result_value: string; unit: string } }).payload;
      addNotification({
        type: 'error',
        title: 'Critical Value Alert',
        message: `${patient_name}: ${test_parameter} = ${result_value} ${unit}`,
      });

      // Play alert sound (if enabled)
      playAlertSound();
    });

    // Result verification notifications
    const unsubscribeResult = ws.on('result.verified', (event) => {
      const { sample_number, verified_by } = (event as { payload: { sample_number: string; verified_by: string } }).payload;
      addNotification({
        type: 'success',
        title: 'Result Verified',
        message: `Sample ${sample_number} verified by ${verified_by}`,
      });
    });

    // Report generation notifications
    const unsubscribeReport = ws.on('report.generated', (event) => {
      const { report_number, patient_name } = (event as { payload: { report_number: string; patient_name: string } }).payload;
      addNotification({
        type: 'success',
        title: 'Report Generated',
        message: `Report ${report_number} for ${patient_name} is ready`,
      });
    });

    // Equipment maintenance alerts
    const unsubscribeEquipment = ws.on('equipment.maintenance_due', (event) => {
      const { equipment_name, maintenance_type } = (event as { payload: { equipment_name: string; maintenance_type: string } }).payload;
      addNotification({
        type: 'warning',
        title: 'Equipment Maintenance Due',
        message: `${equipment_name}: ${maintenance_type} required`,
      });
    });

    // Inventory low stock alerts
    const unsubscribeInventory = ws.on('inventory.low_stock', (event) => {
      const { item_name, current_stock, minimum_stock } = (event as { payload: { item_name: string; current_stock: number; minimum_stock: number } }).payload;
      addNotification({
        type: 'warning',
        title: 'Low Stock Alert',
        message: `${item_name}: ${current_stock} units (min: ${minimum_stock})`,
      });
    });

    // QC failure alerts
    const unsubscribeQC = ws.on('qc.failure_detected', (event) => {
      const { test_name, rule_violated } = (event as { payload: { test_name: string; rule_violated: string } }).payload;
      addNotification({
        type: 'error',
        title: 'QC Failure',
        message: `${test_name}: ${rule_violated} rule violated`,
      });
    });

    return () => {
      unsubscribeNotification();
      unsubscribeSample();
      unsubscribeCritical();
      unsubscribeResult();
      unsubscribeReport();
      unsubscribeEquipment();
      unsubscribeInventory();
      unsubscribeQC();
    };
  }, [addNotification]);

  return (
    <ToastContainer>
      {notifications.map((notification) => (
        <Toast
          key={notification.id}
          id={notification.id}
          type={notification.type}
          title={notification.title}
          message={notification.message}
          duration={notification.type === 'error' ? 10000 : 5000}
          onClose={removeNotification}
        />
      ))}
    </ToastContainer>
  );
}
