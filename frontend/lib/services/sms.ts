/**
 * SMS Service Integration
 *
 * Supports multiple SMS providers:
 * - Twilio
 * - MSG91 (popular in India)
 * - AWS SNS
 *
 * For sending:
 * - OTP verification
 * - Appointment reminders
 * - Report ready notifications
 * - Payment confirmations
 */

export interface SMSConfig {
  provider: 'twilio' | 'msg91' | 'sns';
  // Twilio Config
  twilio?: {
    accountSid: string;
    authToken: string;
    fromNumber: string;
  };
  // MSG91 Config
  msg91?: {
    authKey: string;
    senderId: string;
    route: '1' | '4'; // 1: Promotional, 4: Transactional
    templateId?: string;
  };
  // AWS SNS Config
  sns?: {
    region: string;
    accessKeyId: string;
    secretAccessKey: string;
  };
}

export interface SMSMessage {
  to: string; // Phone number with country code (e.g., +919876543210)
  message: string;
  templateId?: string;
  variables?: Record<string, string>;
}

export interface SMSResponse {
  success: boolean;
  messageId?: string;
  error?: string;
}

// Predefined SMS templates
const SMS_TEMPLATES: Record<string, string> = {
  OTP_VERIFICATION: 'Your OTP for verification is {{otp}}. Valid for {{validity}} minutes. Do not share with anyone. - {{labName}}',
  APPOINTMENT_REMINDER: 'Reminder: You have an appointment on {{date}} at {{time}} at {{location}}. - {{labName}}',
  REPORT_READY: 'Dear {{patientName}}, your lab report for {{testName}} is ready. Order #{{orderNumber}}. Download from portal. - {{labName}}',
  PAYMENT_SUCCESS: 'Payment of Rs.{{amount}} received for Invoice #{{invoiceNumber}}. Thank you! - {{labName}}',
  SAMPLE_COLLECTED: 'Your sample for {{testName}} has been collected. Sample ID: {{sampleId}}. - {{labName}}',
  CRITICAL_RESULT: 'URGENT: Critical result detected in {{testName}}. Please contact your doctor immediately. - {{labName}}',
  ORDER_CREATED: 'Order #{{orderNumber}} created successfully. Please arrive at {{location}} for sample collection. - {{labName}}',
  REGISTRATION_SUCCESS: 'Welcome {{patientName}}! Your patient ID is {{patientId}}. You can now access our services. - {{labName}}',
};

class SMSService {
  private config: SMSConfig;

  constructor(config?: SMSConfig) {
    this.config = config || {
      provider: (process.env.NEXT_PUBLIC_SMS_PROVIDER as 'twilio' | 'msg91' | 'sns') || 'msg91',
    };
  }

  /**
   * Format phone number to E.164 format
   */
  private formatPhoneNumber(phone: string): string {
    // Remove all non-digit characters
    let cleaned = phone.replace(/\D/g, '');

    // If doesn't start with country code, add +91 for India
    if (!cleaned.startsWith('91') && cleaned.length === 10) {
      cleaned = '91' + cleaned;
    }

    return '+' + cleaned;
  }

  /**
   * Send SMS using configured provider
   */
  async sendSMS(message: SMSMessage): Promise<SMSResponse> {
    try {
      const formattedPhone = this.formatPhoneNumber(message.to);

      const response = await fetch('/api/sms/send', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          ...message,
          to: formattedPhone,
          provider: this.config.provider,
        }),
      });

      if (!response.ok) {
        const error = await response.json();
        return {
          success: false,
          error: error.message || 'Failed to send SMS',
        };
      }

      const data = await response.json();
      return {
        success: true,
        messageId: data.messageId,
      };
    } catch (error) {
      console.error('SMS sending error:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  /**
   * Send SMS using template
   */
  async sendTemplateSMS(
    templateName: keyof typeof SMS_TEMPLATES,
    to: string,
    variables: Record<string, string>
  ): Promise<SMSResponse> {
    const template = SMS_TEMPLATES[templateName];

    if (!template) {
      return {
        success: false,
        error: `Template ${templateName} not found`,
      };
    }

    // Replace variables in message
    let message = template;
    Object.entries(variables).forEach(([key, value]) => {
      const regex = new RegExp(`{{${key}}}`, 'g');
      message = message.replace(regex, value);
    });

    return this.sendSMS({ to, message });
  }

  /**
   * Send OTP
   */
  async sendOTP(
    phone: string,
    otp: string,
    validity: number = 10
  ): Promise<SMSResponse> {
    return this.sendTemplateSMS('OTP_VERIFICATION', phone, {
      otp,
      validity: validity.toString(),
      labName: process.env.NEXT_PUBLIC_LAB_NAME || 'Lab',
    });
  }

  /**
   * Send appointment reminder
   */
  async sendAppointmentReminder(
    phone: string,
    date: string,
    time: string,
    location: string
  ): Promise<SMSResponse> {
    return this.sendTemplateSMS('APPOINTMENT_REMINDER', phone, {
      date,
      time,
      location,
      labName: process.env.NEXT_PUBLIC_LAB_NAME || 'Lab',
    });
  }

  /**
   * Send report ready notification
   */
  async sendReportReadyNotification(
    phone: string,
    patientName: string,
    testName: string,
    orderNumber: string
  ): Promise<SMSResponse> {
    return this.sendTemplateSMS('REPORT_READY', phone, {
      patientName,
      testName,
      orderNumber,
      labName: process.env.NEXT_PUBLIC_LAB_NAME || 'Lab',
    });
  }

  /**
   * Send payment success notification
   */
  async sendPaymentSuccessNotification(
    phone: string,
    amount: string,
    invoiceNumber: string
  ): Promise<SMSResponse> {
    return this.sendTemplateSMS('PAYMENT_SUCCESS', phone, {
      amount,
      invoiceNumber,
      labName: process.env.NEXT_PUBLIC_LAB_NAME || 'Lab',
    });
  }

  /**
   * Send sample collected notification
   */
  async sendSampleCollectedNotification(
    phone: string,
    testName: string,
    sampleId: string
  ): Promise<SMSResponse> {
    return this.sendTemplateSMS('SAMPLE_COLLECTED', phone, {
      testName,
      sampleId,
      labName: process.env.NEXT_PUBLIC_LAB_NAME || 'Lab',
    });
  }

  /**
   * Send critical result alert
   */
  async sendCriticalResultAlert(
    phone: string,
    testName: string
  ): Promise<SMSResponse> {
    return this.sendTemplateSMS('CRITICAL_RESULT', phone, {
      testName,
      labName: process.env.NEXT_PUBLIC_LAB_NAME || 'Lab',
    });
  }

  /**
   * Send order created notification
   */
  async sendOrderCreatedNotification(
    phone: string,
    orderNumber: string,
    location: string
  ): Promise<SMSResponse> {
    return this.sendTemplateSMS('ORDER_CREATED', phone, {
      orderNumber,
      location,
      labName: process.env.NEXT_PUBLIC_LAB_NAME || 'Lab',
    });
  }

  /**
   * Send registration success notification
   */
  async sendRegistrationSuccess(
    phone: string,
    patientName: string,
    patientId: string
  ): Promise<SMSResponse> {
    return this.sendTemplateSMS('REGISTRATION_SUCCESS', phone, {
      patientName,
      patientId,
      labName: process.env.NEXT_PUBLIC_LAB_NAME || 'Lab',
    });
  }

  /**
   * Send bulk SMS
   */
  async sendBulkSMS(
    messages: SMSMessage[]
  ): Promise<{ success: number; failed: number; results: SMSResponse[] }> {
    const results = await Promise.all(
      messages.map((msg) => this.sendSMS(msg))
    );

    const success = results.filter((r) => r.success).length;
    const failed = results.length - success;

    return { success, failed, results };
  }

  /**
   * Verify phone number
   */
  async verifyPhoneNumber(phone: string): Promise<boolean> {
    try {
      const formattedPhone = this.formatPhoneNumber(phone);

      // Basic validation
      if (formattedPhone.length < 12 || formattedPhone.length > 15) {
        return false;
      }

      const response = await fetch('/api/sms/verify-number', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ phone: formattedPhone }),
      });

      return response.ok;
    } catch (error) {
      console.error('Phone verification error:', error);
      return false;
    }
  }

  /**
   * Get SMS delivery status
   */
  async getDeliveryStatus(messageId: string): Promise<{
    status: 'queued' | 'sent' | 'delivered' | 'failed';
    error?: string;
  }> {
    try {
      const response = await fetch(`/api/sms/status/${messageId}`, {
        method: 'GET',
      });

      if (!response.ok) {
        return { status: 'failed', error: 'Failed to fetch status' };
      }

      return await response.json();
    } catch (error) {
      console.error('Status fetch error:', error);
      return { status: 'failed', error: 'Unknown error' };
    }
  }

  /**
   * Check SMS balance/credits
   */
  async checkBalance(): Promise<{ balance: number; currency?: string }> {
    try {
      const response = await fetch('/api/sms/balance', {
        method: 'GET',
      });

      if (!response.ok) {
        return { balance: 0 };
      }

      return await response.json();
    } catch (error) {
      console.error('Balance check error:', error);
      return { balance: 0 };
    }
  }

  /**
   * Verify SMS configuration
   */
  async verifyConfiguration(): Promise<boolean> {
    try {
      const response = await fetch('/api/sms/verify-config', {
        method: 'GET',
      });

      return response.ok;
    } catch (error) {
      console.error('SMS configuration verification error:', error);
      return false;
    }
  }
}

// Singleton instance
export const smsService = new SMSService();

// Export templates and class
export { SMS_TEMPLATES, SMSService };
