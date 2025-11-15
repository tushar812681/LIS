# External Services Integration

This directory contains integration modules for all external services used in the Laboratory Information System.

## Available Services

### 1. ABDM Integration (`abdm.ts`)
Integration with Ayushman Bharat Digital Mission for India's national health stack.

**Features:**
- ABHA (Ayushman Bharat Health Account) number creation
- Patient verification
- Health records sharing
- Consent management

**Usage:**
```typescript
import { abdmService } from '@/lib/services';

// Generate OTP for ABHA creation
const { txnId } = await abdmService.generateABHACreationOTP('+919876543210');

// Verify OTP and create ABHA
const abhaAddress = await abdmService.verifyOTPAndCreateABHA(txnId, '123456');

// Search existing ABHA
const patient = await abdmService.searchABHAByHealthId('user@abdm');

// Request consent for health records
const { consentId } = await abdmService.requestConsent({
  patientId: 'patient-123',
  purpose: 'CAREMGT',
  hiTypes: ['Prescription', 'DiagnosticReport'],
  dateRange: { from: '2024-01-01', to: '2024-12-31' },
  dataEraseAt: '2025-01-01',
});
```

**Environment Variables:**
```
NEXT_PUBLIC_ABDM_CLIENT_ID=your_client_id
NEXT_PUBLIC_ABDM_CLIENT_SECRET=your_client_secret
NEXT_PUBLIC_ABDM_BASE_URL=https://dev.abdm.gov.in
NEXT_PUBLIC_ABDM_ENV=sandbox
```

---

### 2. Payment Gateway (`payment.ts`)
Razorpay integration for online payments.

**Features:**
- Order creation
- Payment checkout
- Payment verification
- Payment links
- Refunds
- Webhook handling

**Usage:**
```typescript
import { paymentService } from '@/lib/services';

// Create payment order
const order = await paymentService.createOrder(
  1500, // amount in rupees
  'INV-2024-001', // receipt/invoice number
  { patientId: 'PAT001' } // optional notes
);

// Initiate payment
await paymentService.initiatePayment(
  order.orderId,
  1500,
  {
    name: 'John Doe',
    email: 'john@example.com',
    contact: '+919876543210',
  },
  'Lab Test Payment',
  (response) => {
    // Success callback
    console.log('Payment successful:', response.razorpay_payment_id);
  },
  (error) => {
    // Failure callback
    console.error('Payment failed:', error);
  }
);

// Verify payment
const verified = await paymentService.verifyPayment(
  order.orderId,
  paymentId,
  signature
);

// Create payment link
const link = await paymentService.createPaymentLink(
  2500,
  'Lab Test - CBC',
  {
    name: 'Jane Smith',
    email: 'jane@example.com',
    contact: '+919876543210',
  }
);
console.log('Payment link:', link.short_url);

// Initiate refund
const refund = await paymentService.initiateRefund(paymentId, 500); // partial refund
```

**Environment Variables:**
```
NEXT_PUBLIC_RAZORPAY_KEY_ID=rzp_test_xxx
NEXT_PUBLIC_RAZORPAY_KEY_SECRET=your_secret
NEXT_PUBLIC_RAZORPAY_WEBHOOK_SECRET=webhook_secret
NEXT_PUBLIC_RAZORPAY_ENV=test
```

---

### 3. Email Service (`email.ts`)
Multi-provider email service supporting SMTP, SendGrid, and AWS SES.

**Features:**
- Template-based emails
- HTML emails
- Attachments
- Bulk emails
- Predefined templates for common scenarios

**Predefined Templates:**
- `REPORT_READY` - Lab report notification
- `APPOINTMENT_CONFIRMATION` - Appointment confirmation
- `PAYMENT_RECEIPT` - Payment receipt
- `CRITICAL_RESULT` - Critical test result alert

**Usage:**
```typescript
import { emailService } from '@/lib/services';

// Send report ready email
await emailService.sendReportEmail(
  'patient@example.com',
  'John Doe',
  'Complete Blood Count (CBC)',
  'ORD-2024-001',
  '2024-01-15',
  'https://lab.com/reports/12345'
);

// Send custom email
await emailService.sendEmail({
  to: 'patient@example.com',
  subject: 'Test Results Ready',
  html: '<h1>Your results are ready!</h1>',
  attachments: [{
    filename: 'report.pdf',
    content: pdfBuffer,
    contentType: 'application/pdf',
  }],
});

// Send bulk emails
const results = await emailService.sendBulkEmails([
  { to: 'user1@example.com', subject: 'Test 1', html: '...' },
  { to: 'user2@example.com', subject: 'Test 2', html: '...' },
]);
console.log(`Sent: ${results.success}, Failed: ${results.failed}`);
```

**Environment Variables:**
```
NEXT_PUBLIC_EMAIL_PROVIDER=smtp
NEXT_PUBLIC_EMAIL_FROM_NAME=Lab Name
NEXT_PUBLIC_EMAIL_FROM_EMAIL=noreply@lab.com

# SMTP
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your_email@gmail.com
SMTP_PASS=your_password
```

---

### 4. SMS Service (`sms.ts`)
Multi-provider SMS service supporting Twilio, MSG91, and AWS SNS.

**Features:**
- Template-based SMS
- OTP sending
- Bulk SMS
- Delivery status tracking
- Balance checking

**Predefined Templates:**
- `OTP_VERIFICATION` - OTP for verification
- `APPOINTMENT_REMINDER` - Appointment reminder
- `REPORT_READY` - Report ready notification
- `PAYMENT_SUCCESS` - Payment confirmation
- `SAMPLE_COLLECTED` - Sample collection confirmation
- `CRITICAL_RESULT` - Critical result alert
- `ORDER_CREATED` - Order creation confirmation
- `REGISTRATION_SUCCESS` - Patient registration

**Usage:**
```typescript
import { smsService } from '@/lib/services';

// Send OTP
await smsService.sendOTP('+919876543210', '123456', 10); // 10 minutes validity

// Send appointment reminder
await smsService.sendAppointmentReminder(
  '+919876543210',
  '15th Jan 2024',
  '10:00 AM',
  'Central Lab, Mumbai'
);

// Send report notification
await smsService.sendReportReadyNotification(
  '+919876543210',
  'John Doe',
  'CBC',
  'ORD-2024-001'
);

// Send custom SMS
await smsService.sendSMS({
  to: '+919876543210',
  message: 'Your custom message here',
});

// Check delivery status
const status = await smsService.getDeliveryStatus(messageId);

// Check SMS balance
const balance = await smsService.checkBalance();
console.log(`Remaining credits: ${balance.balance}`);
```

**Environment Variables:**
```
NEXT_PUBLIC_SMS_PROVIDER=msg91

# MSG91 (popular in India)
MSG91_AUTH_KEY=your_auth_key
MSG91_SENDER_ID=LABLIS
MSG91_ROUTE=4
```

---

### 5. WhatsApp Service (`whatsapp.ts`)
WhatsApp Business API integration for rich messaging.

**Features:**
- Text messages
- Template messages
- Document sharing (PDF reports)
- Image sharing
- Interactive buttons
- Interactive lists
- Message status tracking

**Predefined Templates:**
- `REPORT_READY` - Lab report notification
- `APPOINTMENT_CONFIRMATION` - Appointment confirmation
- `PAYMENT_RECEIPT` - Payment receipt
- `SAMPLE_COLLECTED` - Sample collection confirmation
- `CRITICAL_RESULT` - Critical result alert

**Usage:**
```typescript
import { whatsappService } from '@/lib/services';

// Send report ready notification with PDF
await whatsappService.sendReportReadyNotification(
  '+919876543210',
  'John Doe',
  'CBC',
  'ORD-2024-001',
  'https://lab.com/reports/12345.pdf'
);

// Send appointment confirmation
await whatsappService.sendAppointmentConfirmation(
  '+919876543210',
  'John Doe',
  '15th Jan 2024',
  '10:00 AM',
  'Central Lab, Mumbai'
);

// Send document (PDF report)
await whatsappService.sendDocument(
  '+919876543210',
  'https://lab.com/report.pdf',
  'Lab_Report.pdf',
  'Your lab report is attached'
);

// Send interactive buttons
await whatsappService.sendButtonMessage(
  '+919876543210',
  'Confirm your appointment?',
  [
    { id: 'yes', title: 'Yes, Confirm' },
    { id: 'no', title: 'Reschedule' },
  ]
);

// Send interactive list
await whatsappService.sendTestCatalog('+919876543210');

// Check message status
const status = await whatsappService.getMessageStatus(messageId);

// Check if number has WhatsApp
const hasWhatsApp = await whatsappService.checkWhatsAppNumber('+919876543210');
```

**Environment Variables:**
```
NEXT_PUBLIC_WHATSAPP_PROVIDER=twilio

# Twilio
TWILIO_WHATSAPP_ACCOUNT_SID=ACxxx
TWILIO_WHATSAPP_AUTH_TOKEN=your_token
TWILIO_WHATSAPP_FROM_NUMBER=whatsapp:+14155238886
```

---

## Multi-Channel Notifications

Send notifications across multiple channels simultaneously:

```typescript
import { sendMultiChannelNotification } from '@/lib/services';

const results = await sendMultiChannelNotification({
  email: {
    to: 'patient@example.com',
    subject: 'Your Report is Ready',
    html: '<h1>Report Ready!</h1>',
  },
  sms: {
    to: '+919876543210',
    message: 'Your report is ready. Download from portal.',
  },
  whatsapp: {
    to: '+919876543210',
    message: 'Your lab report is ready!',
  },
});

console.log('Email sent:', results.email);
console.log('SMS sent:', results.sms);
console.log('WhatsApp sent:', results.whatsapp);
```

## Service Health Check

Check if all services are properly configured:

```typescript
import { checkServicesHealth } from '@/lib/services';

const health = await checkServicesHealth();
console.log('ABDM:', health.abdm ? '✓' : '✗');
console.log('Payment:', health.payment ? '✓' : '✗');
console.log('Email:', health.email ? '✓' : '✗');
console.log('SMS:', health.sms ? '✓' : '✗');
console.log('WhatsApp:', health.whatsapp ? '✓' : '✗');
```

## Best Practices

### 1. Error Handling
Always wrap service calls in try-catch blocks:

```typescript
try {
  const result = await emailService.sendEmail({ ... });
  if (result) {
    console.log('Email sent successfully');
  }
} catch (error) {
  console.error('Failed to send email:', error);
  // Implement fallback or retry logic
}
```

### 2. Rate Limiting
Implement rate limiting for SMS and WhatsApp to avoid costs:

```typescript
// Implement debouncing or throttling
import { debounce } from 'lodash';

const sendNotification = debounce(async (phone, message) => {
  await smsService.sendSMS({ to: phone, message });
}, 1000); // Max 1 SMS per second
```

### 3. Template Customization
Customize templates based on your lab's branding:

```typescript
// Modify templates in the service files
// or create custom templates for your specific needs
```

### 4. Cost Optimization
- Use email for detailed notifications (cheapest)
- Use SMS for critical alerts (moderate cost)
- Use WhatsApp for rich media (cost-effective for engagement)

### 5. Compliance
- Obtain user consent for SMS/WhatsApp
- Provide opt-out mechanisms
- Follow GDPR/privacy regulations
- Maintain audit logs

## API Routes

These services require backend API routes to handle sensitive operations. Create the following routes in `/app/api/`:

- `/api/payment/create-order`
- `/api/payment/verify`
- `/api/payment/webhook`
- `/api/email/send`
- `/api/sms/send`
- `/api/whatsapp/send`

Refer to backend documentation for implementation details.

## Testing

### Development Mode
Use sandbox/test environments:
- ABDM: Use sandbox mode
- Razorpay: Use test keys (rzp_test_xxx)
- Twilio: Use test credentials
- MSG91: Use test route

### Production Mode
- Switch to production credentials
- Enable webhook endpoints
- Set up monitoring and alerts
- Implement proper error tracking

## Support

For issues or questions:
1. Check service provider documentation
2. Review environment variables
3. Check API route implementations
4. Enable debug mode for detailed logs

## License

All integrations follow their respective service provider terms and conditions.
