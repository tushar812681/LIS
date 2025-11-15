# API Routes Implementation Guide

This document outlines all API routes that need to be implemented in the backend to support the external services integration in the LIS Modern frontend.

---

## Overview

The frontend uses 5 external service integrations that require backend API routes:
1. **Payment Gateway** (Razorpay)
2. **Email Service** (SMTP/SendGrid/SES)
3. **SMS Service** (Twilio/MSG91/SNS)
4. **WhatsApp Service** (Twilio/Meta)
5. **ABDM Integration** (handled mostly by frontend, but may need backend support)

---

## Payment API Routes

### POST /api/payment/create-order

Create a new payment order with Razorpay.

**Request:**
```json
{
  "amount": 150000,  // Amount in paise (₹1500.00)
  "currency": "INR",
  "receipt": "INV-2024-001",
  "notes": {
    "patientId": "PAT001",
    "invoiceId": "INV001"
  }
}
```

**Response:**
```json
{
  "orderId": "order_xxx",
  "amount": 150000,
  "currency": "INR",
  "receipt": "INV-2024-001"
}
```

**Implementation:**
```javascript
import Razorpay from 'razorpay';

const razorpay = new Razorpay({
  key_id: process.env.RAZORPAY_KEY_ID,
  key_secret: process.env.RAZORPAY_KEY_SECRET,
});

export async function POST(request) {
  const { amount, currency, receipt, notes } = await request.json();

  try {
    const order = await razorpay.orders.create({
      amount,
      currency,
      receipt,
      notes,
    });

    return Response.json(order);
  } catch (error) {
    return Response.json({ error: error.message }, { status: 500 });
  }
}
```

---

### POST /api/payment/verify

Verify payment signature after payment completion.

**Request:**
```json
{
  "orderId": "order_xxx",
  "paymentId": "pay_xxx",
  "signature": "signature_xxx"
}
```

**Response:**
```json
{
  "verified": true
}
```

**Implementation:**
```javascript
import crypto from 'crypto';

export async function POST(request) {
  const { orderId, paymentId, signature } = await request.json();

  const secret = process.env.RAZORPAY_KEY_SECRET;
  const body = orderId + "|" + paymentId;

  const expectedSignature = crypto
    .createHmac('sha256', secret)
    .update(body)
    .digest('hex');

  const verified = expectedSignature === signature;

  return Response.json({ verified });
}
```

---

### POST /api/payment/create-link

Create a payment link for sharing.

**Request:**
```json
{
  "amount": 250000,
  "currency": "INR",
  "description": "Lab Test - CBC",
  "customer": {
    "name": "John Doe",
    "email": "john@example.com",
    "contact": "+919876543210"
  },
  "expire_by": 1735689600,
  "notify": {
    "sms": true,
    "email": true
  },
  "reminder_enable": true
}
```

**Response:**
```json
{
  "id": "plink_xxx",
  "short_url": "https://rzp.io/i/xxx",
  "amount": 250000,
  "currency": "INR"
}
```

---

### POST /api/payment/refund

Initiate a refund.

**Request:**
```json
{
  "paymentId": "pay_xxx",
  "amount": 50000,  // Optional, omit for full refund
  "notes": {
    "reason": "Test cancelled"
  }
}
```

**Response:**
```json
{
  "id": "rfnd_xxx",
  "payment_id": "pay_xxx",
  "amount": 50000,
  "currency": "INR",
  "status": "processed"
}
```

---

### POST /api/payment/webhook

Handle Razorpay webhooks.

**Headers:**
```
X-Razorpay-Signature: signature_here
```

**Request:**
```json
{
  "event": "payment.captured",
  "payload": {
    "payment": {
      "entity": {
        "id": "pay_xxx",
        "amount": 150000,
        "status": "captured"
      }
    }
  }
}
```

**Implementation:**
```javascript
import crypto from 'crypto';

export async function POST(request) {
  const signature = request.headers.get('X-Razorpay-Signature');
  const payload = await request.text();

  const secret = process.env.RAZORPAY_WEBHOOK_SECRET;
  const expectedSignature = crypto
    .createHmac('sha256', secret)
    .update(payload)
    .digest('hex');

  if (signature !== expectedSignature) {
    return Response.json({ error: 'Invalid signature' }, { status: 401 });
  }

  const event = JSON.parse(payload);

  // Handle different events
  switch (event.event) {
    case 'payment.captured':
      // Update database
      break;
    case 'payment.failed':
      // Handle failure
      break;
  }

  return Response.json({ verified: true });
}
```

---

## Email API Routes

### POST /api/email/send

Send an email.

**Request:**
```json
{
  "to": "patient@example.com",
  "subject": "Your Lab Report is Ready",
  "html": "<h1>Report Ready</h1><p>Your report is available.</p>",
  "from": {
    "name": "Lab Name",
    "email": "noreply@lab.com"
  },
  "attachments": [
    {
      "filename": "report.pdf",
      "content": "base64_encoded_content",
      "contentType": "application/pdf"
    }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "messageId": "msg_xxx"
}
```

**Implementation (SMTP):**
```javascript
import nodemailer from 'nodemailer';

const transporter = nodemailer.createTransport({
  host: process.env.SMTP_HOST,
  port: process.env.SMTP_PORT,
  secure: false,
  auth: {
    user: process.env.SMTP_USER,
    pass: process.env.SMTP_PASS,
  },
});

export async function POST(request) {
  const { to, subject, html, from, attachments } = await request.json();

  try {
    const info = await transporter.sendMail({
      from: `"${from.name}" <${from.email}>`,
      to,
      subject,
      html,
      attachments,
    });

    return Response.json({ success: true, messageId: info.messageId });
  } catch (error) {
    return Response.json({ success: false, error: error.message }, { status: 500 });
  }
}
```

**Implementation (SendGrid):**
```javascript
import sgMail from '@sendgrid/mail';

sgMail.setApiKey(process.env.SENDGRID_API_KEY);

export async function POST(request) {
  const { to, subject, html, from } = await request.json();

  const msg = {
    to,
    from: from.email,
    subject,
    html,
  };

  try {
    await sgMail.send(msg);
    return Response.json({ success: true });
  } catch (error) {
    return Response.json({ success: false, error: error.message }, { status: 500 });
  }
}
```

---

### GET /api/email/verify

Verify email configuration.

**Response:**
```json
{
  "verified": true,
  "provider": "smtp"
}
```

---

## SMS API Routes

### POST /api/sms/send

Send an SMS.

**Request:**
```json
{
  "to": "+919876543210",
  "message": "Your OTP is 123456. Valid for 10 minutes.",
  "provider": "msg91"
}
```

**Response:**
```json
{
  "success": true,
  "messageId": "msg_xxx"
}
```

**Implementation (MSG91):**
```javascript
export async function POST(request) {
  const { to, message, provider } = await request.json();

  const url = 'https://api.msg91.com/api/v5/flow/';
  const response = await fetch(url, {
    method: 'POST',
    headers: {
      'authkey': process.env.MSG91_AUTH_KEY,
      'content-type': 'application/json',
    },
    body: JSON.stringify({
      sender: process.env.MSG91_SENDER_ID,
      route: process.env.MSG91_ROUTE,
      mobiles: to.replace('+', ''),
      sms: message,
    }),
  });

  const data = await response.json();

  return Response.json({
    success: response.ok,
    messageId: data.request_id,
  });
}
```

**Implementation (Twilio):**
```javascript
import twilio from 'twilio';

const client = twilio(
  process.env.TWILIO_ACCOUNT_SID,
  process.env.TWILIO_AUTH_TOKEN
);

export async function POST(request) {
  const { to, message } = await request.json();

  try {
    const msg = await client.messages.create({
      body: message,
      from: process.env.TWILIO_FROM_NUMBER,
      to,
    });

    return Response.json({ success: true, messageId: msg.sid });
  } catch (error) {
    return Response.json({ success: false, error: error.message }, { status: 500 });
  }
}
```

---

### GET /api/sms/status/:messageId

Get SMS delivery status.

**Response:**
```json
{
  "status": "delivered",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

---

### GET /api/sms/balance

Check SMS credits balance.

**Response:**
```json
{
  "balance": 1500,
  "currency": "INR"
}
```

---

## WhatsApp API Routes

### POST /api/whatsapp/send

Send a WhatsApp message.

**Request:**
```json
{
  "to": "+919876543210",
  "type": "text",
  "content": "Your lab report is ready!",
  "provider": "twilio"
}
```

**Response:**
```json
{
  "success": true,
  "messageId": "msg_xxx"
}
```

**Implementation (Twilio):**
```javascript
import twilio from 'twilio';

const client = twilio(
  process.env.TWILIO_WHATSAPP_ACCOUNT_SID,
  process.env.TWILIO_WHATSAPP_AUTH_TOKEN
);

export async function POST(request) {
  const { to, type, content } = await request.json();

  try {
    let messageData = {
      from: process.env.TWILIO_WHATSAPP_FROM_NUMBER,
      to: `whatsapp:${to}`,
    };

    if (type === 'text') {
      messageData.body = content;
    } else if (type === 'media') {
      messageData.mediaUrl = [content.url];
      if (content.caption) {
        messageData.body = content.caption;
      }
    }

    const msg = await client.messages.create(messageData);

    return Response.json({ success: true, messageId: msg.sid });
  } catch (error) {
    return Response.json({ success: false, error: error.message }, { status: 500 });
  }
}
```

---

### GET /api/whatsapp/status/:messageId

Get WhatsApp message status.

**Response:**
```json
{
  "status": "delivered",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

---

### POST /api/whatsapp/check-number

Check if a number has WhatsApp.

**Request:**
```json
{
  "phone": "+919876543210"
}
```

**Response:**
```json
{
  "hasWhatsApp": true
}
```

---

## Environment Variables Required

Add these to your backend `.env` file:

```env
# Razorpay
RAZORPAY_KEY_ID=rzp_test_xxx
RAZORPAY_KEY_SECRET=your_secret
RAZORPAY_WEBHOOK_SECRET=webhook_secret

# Email - SMTP
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your_email@gmail.com
SMTP_PASS=your_app_password

# Email - SendGrid (alternative)
SENDGRID_API_KEY=SG.xxx

# SMS - MSG91
MSG91_AUTH_KEY=your_key
MSG91_SENDER_ID=LABLIS
MSG91_ROUTE=4

# SMS - Twilio (alternative)
TWILIO_ACCOUNT_SID=ACxxx
TWILIO_AUTH_TOKEN=your_token
TWILIO_FROM_NUMBER=+1234567890

# WhatsApp - Twilio
TWILIO_WHATSAPP_ACCOUNT_SID=ACxxx
TWILIO_WHATSAPP_AUTH_TOKEN=your_token
TWILIO_WHATSAPP_FROM_NUMBER=whatsapp:+14155238886
```

---

## Security Considerations

### 1. Rate Limiting

Implement rate limiting on all API routes:

```javascript
import rateLimit from 'express-rate-limit';

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // Limit each IP to 100 requests per windowMs
});

app.use('/api/', limiter);
```

### 2. Authentication

Verify JWT tokens before processing:

```javascript
export async function POST(request) {
  const token = request.headers.get('Authorization')?.replace('Bearer ', '');

  if (!token) {
    return Response.json({ error: 'Unauthorized' }, { status: 401 });
  }

  // Verify JWT
  const user = await verifyToken(token);
  if (!user) {
    return Response.json({ error: 'Invalid token' }, { status: 401 });
  }

  // Process request...
}
```

### 3. Input Validation

Validate all inputs:

```javascript
import { z } from 'zod';

const emailSchema = z.object({
  to: z.string().email(),
  subject: z.string().min(1).max(200),
  html: z.string().min(1),
});

export async function POST(request) {
  const body = await request.json();

  try {
    const validated = emailSchema.parse(body);
    // Process validated data...
  } catch (error) {
    return Response.json({ error: 'Invalid input' }, { status: 400 });
  }
}
```

### 4. Error Handling

Implement proper error handling:

```javascript
export async function POST(request) {
  try {
    // Process request...
  } catch (error) {
    console.error('API Error:', error);

    // Don't expose internal errors
    return Response.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}
```

---

## Testing

### Test Payment API

```bash
curl -X POST http://localhost:3000/api/payment/create-order \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 150000,
    "currency": "INR",
    "receipt": "TEST-001"
  }'
```

### Test Email API

```bash
curl -X POST http://localhost:3000/api/email/send \
  -H "Content-Type: application/json" \
  -d '{
    "to": "test@example.com",
    "subject": "Test Email",
    "html": "<h1>Test</h1>"
  }'
```

### Test SMS API

```bash
curl -X POST http://localhost:3000/api/sms/send \
  -H "Content-Type: application/json" \
  -d '{
    "to": "+919876543210",
    "message": "Test SMS"
  }'
```

---

## Monitoring and Logs

### Log All Requests

```javascript
export async function POST(request) {
  const startTime = Date.now();

  console.log(`[${new Date().toISOString()}] ${request.method} ${request.url}`);

  try {
    // Process request...
    const response = Response.json({ success: true });

    console.log(`[${new Date().toISOString()}] Response: ${Date.now() - startTime}ms`);

    return response;
  } catch (error) {
    console.error(`[${new Date().toISOString()}] Error:`, error);
    throw error;
  }
}
```

---

## Support

For questions or issues with these API routes:
1. Check the service provider documentation
2. Review error logs
3. Test with provider's testing tools
4. Contact support@lab.com
