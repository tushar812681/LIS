/**
 * Email Service Integration
 *
 * Supports multiple email providers:
 * - SMTP (Generic)
 * - SendGrid
 * - AWS SES
 *
 * For sending:
 * - Test result reports
 * - Appointment confirmations
 * - Payment receipts
 * - Notifications
 */

export interface EmailConfig {
  provider: 'smtp' | 'sendgrid' | 'ses';
  // SMTP Config
  smtp?: {
    host: string;
    port: number;
    secure: boolean;
    auth: {
      user: string;
      pass: string;
    };
  };
  // SendGrid Config
  sendgrid?: {
    apiKey: string;
  };
  // AWS SES Config
  ses?: {
    region: string;
    accessKeyId: string;
    secretAccessKey: string;
  };
  // Common
  from: {
    name: string;
    email: string;
  };
}

export interface EmailMessage {
  to: string | string[];
  subject: string;
  text?: string;
  html?: string;
  cc?: string | string[];
  bcc?: string | string[];
  replyTo?: string;
  attachments?: EmailAttachment[];
}

export interface EmailAttachment {
  filename: string;
  content: string | Buffer;
  contentType?: string;
  encoding?: string;
}

export interface EmailTemplate {
  name: string;
  subject: string;
  html: string;
  variables: string[];
}

// Predefined email templates
const EMAIL_TEMPLATES: Record<string, EmailTemplate> = {
  REPORT_READY: {
    name: 'Report Ready',
    subject: 'Your Lab Test Report is Ready',
    html: `
      <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
        <div style="background-color: #3b82f6; color: white; padding: 20px; text-align: center;">
          <h1>Lab Report Ready</h1>
        </div>
        <div style="padding: 20px; background-color: #f9fafb;">
          <p>Dear {{patientName}},</p>
          <p>Your laboratory test report for <strong>{{testName}}</strong> is now ready.</p>
          <p><strong>Order Number:</strong> {{orderNumber}}</p>
          <p><strong>Test Date:</strong> {{testDate}}</p>
          <div style="margin: 30px 0; text-align: center;">
            <a href="{{reportUrl}}" style="background-color: #3b82f6; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; display: inline-block;">
              View Report
            </a>
          </div>
          <p>You can download your report from the patient portal or click the link above.</p>
          <p>If you have any questions, please contact us.</p>
          <p>Best regards,<br>{{labName}}</p>
        </div>
        <div style="background-color: #e5e7eb; padding: 15px; text-align: center; font-size: 12px; color: #6b7280;">
          <p>This is an automated email. Please do not reply to this message.</p>
        </div>
      </div>
    `,
    variables: ['patientName', 'testName', 'orderNumber', 'testDate', 'reportUrl', 'labName'],
  },
  APPOINTMENT_CONFIRMATION: {
    name: 'Appointment Confirmation',
    subject: 'Appointment Confirmation - {{labName}}',
    html: `
      <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
        <div style="background-color: #10b981; color: white; padding: 20px; text-align: center;">
          <h1>Appointment Confirmed</h1>
        </div>
        <div style="padding: 20px; background-color: #f9fafb;">
          <p>Dear {{patientName}},</p>
          <p>Your appointment has been confirmed with the following details:</p>
          <div style="background-color: white; padding: 15px; border-radius: 5px; margin: 20px 0;">
            <p><strong>Date:</strong> {{appointmentDate}}</p>
            <p><strong>Time:</strong> {{appointmentTime}}</p>
            <p><strong>Type:</strong> {{appointmentType}}</p>
            <p><strong>Location:</strong> {{location}}</p>
          </div>
          <p><strong>Please Note:</strong></p>
          <ul>
            <li>{{instructions}}</li>
          </ul>
          <p>If you need to reschedule or cancel, please contact us at least 24 hours in advance.</p>
          <p>Best regards,<br>{{labName}}</p>
        </div>
        <div style="background-color: #e5e7eb; padding: 15px; text-align: center; font-size: 12px; color: #6b7280;">
          <p>Contact: {{contactPhone}} | {{contactEmail}}</p>
        </div>
      </div>
    `,
    variables: ['patientName', 'appointmentDate', 'appointmentTime', 'appointmentType', 'location', 'instructions', 'labName', 'contactPhone', 'contactEmail'],
  },
  PAYMENT_RECEIPT: {
    name: 'Payment Receipt',
    subject: 'Payment Receipt - Invoice {{invoiceNumber}}',
    html: `
      <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
        <div style="background-color: #8b5cf6; color: white; padding: 20px; text-align: center;">
          <h1>Payment Received</h1>
        </div>
        <div style="padding: 20px; background-color: #f9fafb;">
          <p>Dear {{patientName}},</p>
          <p>Thank you for your payment. We have received your payment with the following details:</p>
          <div style="background-color: white; padding: 15px; border-radius: 5px; margin: 20px 0;">
            <p><strong>Invoice Number:</strong> {{invoiceNumber}}</p>
            <p><strong>Amount Paid:</strong> ₹{{amount}}</p>
            <p><strong>Payment Method:</strong> {{paymentMethod}}</p>
            <p><strong>Transaction ID:</strong> {{transactionId}}</p>
            <p><strong>Date:</strong> {{paymentDate}}</p>
          </div>
          <div style="margin: 30px 0; text-align: center;">
            <a href="{{receiptUrl}}" style="background-color: #8b5cf6; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; display: inline-block;">
              Download Receipt
            </a>
          </div>
          <p>This receipt has been sent to you for your records.</p>
          <p>Best regards,<br>{{labName}}</p>
        </div>
        <div style="background-color: #e5e7eb; padding: 15px; text-align: center; font-size: 12px; color: #6b7280;">
          <p>This is an automated receipt. Please save for your records.</p>
        </div>
      </div>
    `,
    variables: ['patientName', 'invoiceNumber', 'amount', 'paymentMethod', 'transactionId', 'paymentDate', 'receiptUrl', 'labName'],
  },
  CRITICAL_RESULT: {
    name: 'Critical Result Alert',
    subject: 'URGENT: Critical Test Result - Action Required',
    html: `
      <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
        <div style="background-color: #ef4444; color: white; padding: 20px; text-align: center;">
          <h1>⚠️ CRITICAL RESULT ALERT</h1>
        </div>
        <div style="padding: 20px; background-color: #fee2e2;">
          <p>Dear {{patientName}},</p>
          <p style="color: #dc2626; font-weight: bold; font-size: 16px;">
            A critical abnormal value has been detected in your laboratory test results.
          </p>
          <div style="background-color: white; padding: 15px; border-radius: 5px; margin: 20px 0; border-left: 4px solid #ef4444;">
            <p><strong>Test Name:</strong> {{testName}}</p>
            <p><strong>Result Value:</strong> {{resultValue}} {{unit}}</p>
            <p><strong>Normal Range:</strong> {{normalRange}}</p>
            <p><strong>Report Date:</strong> {{reportDate}}</p>
          </div>
          <p style="color: #dc2626; font-weight: bold;">
            IMMEDIATE ACTION REQUIRED:
          </p>
          <p>Please contact your healthcare provider immediately or visit the nearest emergency facility.</p>
          <div style="background-color: #fef3c7; padding: 15px; border-radius: 5px; margin: 20px 0;">
            <p style="margin: 0;"><strong>Emergency Contact:</strong> {{emergencyContact}}</p>
          </div>
          <p>Best regards,<br>{{labName}}</p>
        </div>
      </div>
    `,
    variables: ['patientName', 'testName', 'resultValue', 'unit', 'normalRange', 'reportDate', 'emergencyContact', 'labName'],
  },
};

class EmailService {
  private config: EmailConfig;

  constructor(config?: EmailConfig) {
    this.config = config || {
      provider: (process.env.NEXT_PUBLIC_EMAIL_PROVIDER as 'smtp' | 'sendgrid' | 'ses') || 'smtp',
      from: {
        name: process.env.NEXT_PUBLIC_EMAIL_FROM_NAME || 'Laboratory Information System',
        email: process.env.NEXT_PUBLIC_EMAIL_FROM_EMAIL || 'noreply@lab.com',
      },
    };
  }

  /**
   * Send email using configured provider
   */
  async sendEmail(message: EmailMessage): Promise<boolean> {
    try {
      const response = await fetch('/api/email/send', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          ...message,
          from: this.config.from,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to send email');
      }

      return true;
    } catch (error) {
      console.error('Email sending error:', error);
      return false;
    }
  }

  /**
   * Send email using template
   */
  async sendTemplateEmail(
    templateName: keyof typeof EMAIL_TEMPLATES,
    to: string | string[],
    variables: Record<string, string>
  ): Promise<boolean> {
    const template = EMAIL_TEMPLATES[templateName];

    if (!template) {
      throw new Error(`Template ${templateName} not found`);
    }

    // Replace variables in subject and HTML
    let subject = template.subject;
    let html = template.html;

    Object.entries(variables).forEach(([key, value]) => {
      const regex = new RegExp(`{{${key}}}`, 'g');
      subject = subject.replace(regex, value);
      html = html.replace(regex, value);
    });

    return this.sendEmail({
      to,
      subject,
      html,
    });
  }

  /**
   * Send test report email
   */
  async sendReportEmail(
    patientEmail: string,
    patientName: string,
    testName: string,
    orderNumber: string,
    testDate: string,
    reportUrl: string
  ): Promise<boolean> {
    return this.sendTemplateEmail('REPORT_READY', patientEmail, {
      patientName,
      testName,
      orderNumber,
      testDate,
      reportUrl,
      labName: this.config.from.name,
    });
  }

  /**
   * Send appointment confirmation
   */
  async sendAppointmentConfirmation(
    patientEmail: string,
    appointmentDetails: {
      patientName: string;
      appointmentDate: string;
      appointmentTime: string;
      appointmentType: string;
      location: string;
      instructions: string;
      contactPhone: string;
      contactEmail: string;
    }
  ): Promise<boolean> {
    return this.sendTemplateEmail('APPOINTMENT_CONFIRMATION', patientEmail, {
      ...appointmentDetails,
      labName: this.config.from.name,
    });
  }

  /**
   * Send payment receipt
   */
  async sendPaymentReceipt(
    patientEmail: string,
    receiptDetails: {
      patientName: string;
      invoiceNumber: string;
      amount: string;
      paymentMethod: string;
      transactionId: string;
      paymentDate: string;
      receiptUrl: string;
    }
  ): Promise<boolean> {
    return this.sendTemplateEmail('PAYMENT_RECEIPT', patientEmail, {
      ...receiptDetails,
      labName: this.config.from.name,
    });
  }

  /**
   * Send critical result alert
   */
  async sendCriticalResultAlert(
    patientEmail: string,
    doctorEmail: string,
    alertDetails: {
      patientName: string;
      testName: string;
      resultValue: string;
      unit: string;
      normalRange: string;
      reportDate: string;
      emergencyContact: string;
    }
  ): Promise<boolean> {
    return this.sendTemplateEmail('CRITICAL_RESULT', [patientEmail, doctorEmail], {
      ...alertDetails,
      labName: this.config.from.name,
    });
  }

  /**
   * Send bulk emails
   */
  async sendBulkEmails(
    messages: EmailMessage[]
  ): Promise<{ success: number; failed: number }> {
    const results = await Promise.allSettled(
      messages.map((msg) => this.sendEmail(msg))
    );

    const success = results.filter((r) => r.status === 'fulfilled' && r.value).length;
    const failed = results.length - success;

    return { success, failed };
  }

  /**
   * Verify email configuration
   */
  async verifyConfiguration(): Promise<boolean> {
    try {
      const response = await fetch('/api/email/verify', {
        method: 'GET',
      });

      return response.ok;
    } catch (error) {
      console.error('Email configuration verification error:', error);
      return false;
    }
  }
}

// Singleton instance
export const emailService = new EmailService();

// Export templates and class
export { EMAIL_TEMPLATES, EmailService };
