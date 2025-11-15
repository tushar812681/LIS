/**
 * WhatsApp Business API Integration Service
 *
 * Supports:
 * - Twilio WhatsApp Business API
 * - Meta WhatsApp Business API
 *
 * For sending:
 * - Appointment confirmations
 * - Report ready notifications
 * - Payment receipts
 * - Interactive messages
 * - Document sharing (PDFs)
 */

export interface WhatsAppConfig {
  provider: 'twilio' | 'meta';
  // Twilio Config
  twilio?: {
    accountSid: string;
    authToken: string;
    fromNumber: string; // WhatsApp enabled number (e.g., whatsapp:+14155238886)
  };
  // Meta WhatsApp Business Config
  meta?: {
    phoneNumberId: string;
    accessToken: string;
    businessAccountId: string;
  };
}

export interface WhatsAppMessage {
  to: string; // Phone number with country code (e.g., +919876543210)
  type: 'text' | 'template' | 'media' | 'interactive';
  content: string | WhatsAppTemplate | WhatsAppMedia | WhatsAppInteractive;
}

export interface WhatsAppTemplate {
  name: string;
  language: string;
  components?: WhatsAppTemplateComponent[];
}

export interface WhatsAppTemplateComponent {
  type: 'header' | 'body' | 'button';
  parameters: Array<{
    type: 'text' | 'image' | 'document';
    text?: string;
    image?: { link: string };
    document?: { link: string; filename: string };
  }>;
}

export interface WhatsAppMedia {
  type: 'image' | 'document' | 'video';
  url: string;
  caption?: string;
  filename?: string;
}

export interface WhatsAppInteractive {
  type: 'button' | 'list';
  body: string;
  buttons?: Array<{
    id: string;
    title: string;
  }>;
  listItems?: Array<{
    id: string;
    title: string;
    description?: string;
  }>;
}

export interface WhatsAppResponse {
  success: boolean;
  messageId?: string;
  error?: string;
}

// Predefined WhatsApp message templates
const WHATSAPP_TEMPLATES = {
  REPORT_READY: {
    name: 'report_ready',
    language: 'en',
    body: 'Dear {{1}}, your lab report for {{2}} is now ready. Order #{{3}}. You can download it from our patient portal.',
  },
  APPOINTMENT_CONFIRMATION: {
    name: 'appointment_confirmation',
    language: 'en',
    body: 'Hi {{1}}, your appointment is confirmed for {{2}} at {{3}}. Location: {{4}}. See you soon!',
  },
  PAYMENT_RECEIPT: {
    name: 'payment_receipt',
    language: 'en',
    body: 'Thank you {{1}}! We have received your payment of ₹{{2}} for Invoice #{{3}}. Receipt has been sent to your email.',
  },
  SAMPLE_COLLECTED: {
    name: 'sample_collected',
    language: 'en',
    body: 'Hi {{1}}, your sample for {{2}} has been collected. Sample ID: {{3}}. We will notify you when results are ready.',
  },
  CRITICAL_RESULT: {
    name: 'critical_result',
    language: 'en',
    body: '⚠️ URGENT: Dear {{1}}, a critical abnormal value has been detected in your {{2}} test. Please contact your doctor immediately.',
  },
};

class WhatsAppService {
  private config: WhatsAppConfig;

  constructor(config?: WhatsAppConfig) {
    this.config = config || {
      provider: (process.env.NEXT_PUBLIC_WHATSAPP_PROVIDER as 'twilio' | 'meta') || 'twilio',
    };
  }

  /**
   * Format phone number for WhatsApp
   */
  private formatWhatsAppNumber(phone: string): string {
    // Remove all non-digit characters
    let cleaned = phone.replace(/\D/g, '');

    // If doesn't start with country code, add +91 for India
    if (!cleaned.startsWith('91') && cleaned.length === 10) {
      cleaned = '91' + cleaned;
    }

    return '+' + cleaned;
  }

  /**
   * Send WhatsApp message
   */
  async sendMessage(message: WhatsAppMessage): Promise<WhatsAppResponse> {
    try {
      const formattedPhone = this.formatWhatsAppNumber(message.to);

      const response = await fetch('/api/whatsapp/send', {
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
          error: error.message || 'Failed to send WhatsApp message',
        };
      }

      const data = await response.json();
      return {
        success: true,
        messageId: data.messageId,
      };
    } catch (error) {
      console.error('WhatsApp sending error:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  /**
   * Send text message
   */
  async sendTextMessage(to: string, text: string): Promise<WhatsAppResponse> {
    return this.sendMessage({
      to,
      type: 'text',
      content: text,
    });
  }

  /**
   * Send template message
   */
  async sendTemplateMessage(
    to: string,
    templateName: keyof typeof WHATSAPP_TEMPLATES,
    parameters: string[]
  ): Promise<WhatsAppResponse> {
    const template = WHATSAPP_TEMPLATES[templateName];

    if (!template) {
      return {
        success: false,
        error: `Template ${templateName} not found`,
      };
    }

    // Replace parameters in template
    let message = template.body;
    parameters.forEach((param, index) => {
      const placeholder = `{{${index + 1}}}`;
      message = message.replace(placeholder, param);
    });

    return this.sendTextMessage(to, message);
  }

  /**
   * Send report ready notification
   */
  async sendReportReadyNotification(
    phone: string,
    patientName: string,
    testName: string,
    orderNumber: string,
    reportUrl: string
  ): Promise<WhatsAppResponse> {
    // Send template message
    const templateResponse = await this.sendTemplateMessage(
      phone,
      'REPORT_READY',
      [patientName, testName, orderNumber]
    );

    // If template succeeds, send document
    if (templateResponse.success && reportUrl) {
      await this.sendDocument(phone, reportUrl, `Report_${orderNumber}.pdf`);
    }

    return templateResponse;
  }

  /**
   * Send appointment confirmation
   */
  async sendAppointmentConfirmation(
    phone: string,
    patientName: string,
    date: string,
    time: string,
    location: string
  ): Promise<WhatsAppResponse> {
    return this.sendTemplateMessage(phone, 'APPOINTMENT_CONFIRMATION', [
      patientName,
      date,
      time,
      location,
    ]);
  }

  /**
   * Send payment receipt
   */
  async sendPaymentReceipt(
    phone: string,
    patientName: string,
    amount: string,
    invoiceNumber: string,
    receiptUrl?: string
  ): Promise<WhatsAppResponse> {
    const templateResponse = await this.sendTemplateMessage(phone, 'PAYMENT_RECEIPT', [
      patientName,
      amount,
      invoiceNumber,
    ]);

    // If receipt URL provided, send as document
    if (templateResponse.success && receiptUrl) {
      await this.sendDocument(phone, receiptUrl, `Receipt_${invoiceNumber}.pdf`);
    }

    return templateResponse;
  }

  /**
   * Send sample collected notification
   */
  async sendSampleCollectedNotification(
    phone: string,
    patientName: string,
    testName: string,
    sampleId: string
  ): Promise<WhatsAppResponse> {
    return this.sendTemplateMessage(phone, 'SAMPLE_COLLECTED', [
      patientName,
      testName,
      sampleId,
    ]);
  }

  /**
   * Send critical result alert
   */
  async sendCriticalResultAlert(
    phone: string,
    patientName: string,
    testName: string
  ): Promise<WhatsAppResponse> {
    return this.sendTemplateMessage(phone, 'CRITICAL_RESULT', [patientName, testName]);
  }

  /**
   * Send document (PDF)
   */
  async sendDocument(
    to: string,
    documentUrl: string,
    filename: string,
    caption?: string
  ): Promise<WhatsAppResponse> {
    const media: WhatsAppMedia = {
      type: 'document',
      url: documentUrl,
      filename,
      caption,
    };

    return this.sendMessage({
      to,
      type: 'media',
      content: media,
    });
  }

  /**
   * Send image
   */
  async sendImage(
    to: string,
    imageUrl: string,
    caption?: string
  ): Promise<WhatsAppResponse> {
    const media: WhatsAppMedia = {
      type: 'image',
      url: imageUrl,
      caption,
    };

    return this.sendMessage({
      to,
      type: 'media',
      content: media,
    });
  }

  /**
   * Send interactive button message
   */
  async sendButtonMessage(
    to: string,
    body: string,
    buttons: Array<{ id: string; title: string }>
  ): Promise<WhatsAppResponse> {
    const interactive: WhatsAppInteractive = {
      type: 'button',
      body,
      buttons,
    };

    return this.sendMessage({
      to,
      type: 'interactive',
      content: interactive,
    });
  }

  /**
   * Send interactive list message
   */
  async sendListMessage(
    to: string,
    body: string,
    listItems: Array<{ id: string; title: string; description?: string }>
  ): Promise<WhatsAppResponse> {
    const interactive: WhatsAppInteractive = {
      type: 'list',
      body,
      listItems,
    };

    return this.sendMessage({
      to,
      type: 'interactive',
      content: interactive,
    });
  }

  /**
   * Send appointment options (interactive)
   */
  async sendAppointmentOptions(
    phone: string,
    patientName: string
  ): Promise<WhatsAppResponse> {
    return this.sendButtonMessage(
      phone,
      `Hi ${patientName}! How would you like to proceed with your appointment?`,
      [
        { id: 'confirm', title: 'Confirm Appointment' },
        { id: 'reschedule', title: 'Reschedule' },
        { id: 'cancel', title: 'Cancel' },
      ]
    );
  }

  /**
   * Send test catalog (interactive list)
   */
  async sendTestCatalog(phone: string): Promise<WhatsAppResponse> {
    return this.sendListMessage(
      phone,
      'Browse our available tests:',
      [
        {
          id: 'cbc',
          title: 'Complete Blood Count',
          description: '₹500 - Results in 24 hours',
        },
        {
          id: 'lipid',
          title: 'Lipid Profile',
          description: '₹800 - Results in 48 hours',
        },
        {
          id: 'glucose',
          title: 'Blood Glucose',
          description: '₹300 - Results in 12 hours',
        },
        {
          id: 'thyroid',
          title: 'Thyroid Function',
          description: '₹1200 - Results in 48 hours',
        },
      ]
    );
  }

  /**
   * Get message status
   */
  async getMessageStatus(messageId: string): Promise<{
    status: 'sent' | 'delivered' | 'read' | 'failed';
    timestamp?: string;
  }> {
    try {
      const response = await fetch(`/api/whatsapp/status/${messageId}`, {
        method: 'GET',
      });

      if (!response.ok) {
        return { status: 'failed' };
      }

      return await response.json();
    } catch (error) {
      console.error('Status fetch error:', error);
      return { status: 'failed' };
    }
  }

  /**
   * Handle incoming webhook
   */
  async handleWebhook(payload: unknown): Promise<{
    type: 'message' | 'status';
    data: unknown;
  }> {
    try {
      const response = await fetch('/api/whatsapp/webhook', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error('Webhook processing failed');
      }

      return await response.json();
    } catch (error) {
      console.error('Webhook handling error:', error);
      throw error;
    }
  }

  /**
   * Verify webhook signature (for security)
   */
  async verifyWebhookSignature(
    payload: string,
    signature: string
  ): Promise<boolean> {
    try {
      const response = await fetch('/api/whatsapp/verify-signature', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ payload, signature }),
      });

      return response.ok;
    } catch (error) {
      console.error('Signature verification error:', error);
      return false;
    }
  }

  /**
   * Check if phone number has WhatsApp
   */
  async checkWhatsAppNumber(phone: string): Promise<boolean> {
    try {
      const formattedPhone = this.formatWhatsAppNumber(phone);

      const response = await fetch('/api/whatsapp/check-number', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ phone: formattedPhone }),
      });

      if (!response.ok) {
        return false;
      }

      const data = await response.json();
      return data.hasWhatsApp === true;
    } catch (error) {
      console.error('WhatsApp number check error:', error);
      return false;
    }
  }

  /**
   * Verify WhatsApp configuration
   */
  async verifyConfiguration(): Promise<boolean> {
    try {
      const response = await fetch('/api/whatsapp/verify-config', {
        method: 'GET',
      });

      return response.ok;
    } catch (error) {
      console.error('WhatsApp configuration verification error:', error);
      return false;
    }
  }
}

// Singleton instance
export const whatsappService = new WhatsAppService();

// Export templates and class
export { WHATSAPP_TEMPLATES, WhatsAppService };
