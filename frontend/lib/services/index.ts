/**
 * External Services Integration Index
 *
 * Central export for all external service integrations:
 * - ABDM (Ayushman Bharat Digital Mission)
 * - Payment Gateway (Razorpay)
 * - Email (SMTP/SendGrid/SES)
 * - SMS (Twilio/MSG91/SNS)
 * - WhatsApp (Twilio/Meta Business API)
 */

import { abdmService } from './abdm';
import { paymentService } from './payment';
import { emailService } from './email';
import { smsService } from './sms';
import { whatsappService } from './whatsapp';

// ABDM Integration
export {
  abdmService,
  ABDMService,
  type ABDMConfig,
  type ABHAAddress,
  type ABHACard,
  type ConsentRequest,
  type HealthRecord,
} from './abdm';

// Payment Gateway Integration
export {
  paymentService,
  PaymentService,
  type PaymentConfig,
  type PaymentOrder,
  type PaymentOptions,
  type PaymentResponse,
  type PaymentLink,
  type RefundRequest,
  type RefundResponse,
} from './payment';

// Email Service Integration
export {
  emailService,
  EmailService,
  EMAIL_TEMPLATES,
  type EmailConfig,
  type EmailMessage,
  type EmailAttachment,
  type EmailTemplate,
} from './email';

// SMS Service Integration
export {
  smsService,
  SMSService,
  SMS_TEMPLATES,
  type SMSConfig,
  type SMSMessage,
  type SMSResponse,
} from './sms';

// WhatsApp Service Integration
export {
  whatsappService,
  WhatsAppService,
  WHATSAPP_TEMPLATES,
  type WhatsAppConfig,
  type WhatsAppMessage,
  type WhatsAppTemplate,
  type WhatsAppMedia,
  type WhatsAppInteractive,
  type WhatsAppResponse,
} from './whatsapp';

/**
 * Service Health Check
 * Verifies all external service configurations
 */
export async function checkServicesHealth(): Promise<{
  abdm: boolean;
  payment: boolean;
  email: boolean;
  sms: boolean;
  whatsapp: boolean;
}> {
  const [abdm, payment, email, sms, whatsapp] = await Promise.all([
    abdmService.verifyConfiguration(),
    paymentService.checkPaymentStatus('test').then(() => true).catch(() => false),
    emailService.verifyConfiguration(),
    smsService.verifyConfiguration(),
    whatsappService.verifyConfiguration(),
  ]);

  return { abdm, payment, email, sms, whatsapp };
}

/**
 * Unified notification sender
 * Sends notifications through multiple channels (Email + SMS + WhatsApp)
 */
export async function sendMultiChannelNotification(
  channels: {
    email?: { to: string; subject: string; html: string };
    sms?: { to: string; message: string };
    whatsapp?: { to: string; message: string };
  }
): Promise<{
  email?: boolean;
  sms?: boolean;
  whatsapp?: boolean;
}> {
  const results: Record<string, boolean> = {};

  if (channels.email) {
    results.email = await emailService.sendEmail(channels.email);
  }

  if (channels.sms) {
    const smsResult = await smsService.sendSMS(channels.sms);
    results.sms = smsResult.success;
  }

  if (channels.whatsapp) {
    const whatsappResult = await whatsappService.sendTextMessage(
      channels.whatsapp.to,
      channels.whatsapp.message
    );
    results.whatsapp = whatsappResult.success;
  }

  return results;
}
