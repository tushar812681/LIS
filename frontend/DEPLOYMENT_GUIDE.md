# LIS Modern - Deployment Guide

**Version:** 1.0.0
**Last Updated:** January 2025

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Backend Requirements](#backend-requirements)
4. [External Services Configuration](#external-services-configuration)
5. [Deployment Options](#deployment-options)
6. [Post-Deployment](#post-deployment)
7. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### System Requirements
- **Node.js:** v20.x or higher
- **npm:** v10.x or higher
- **Memory:** Minimum 2GB RAM
- **Storage:** Minimum 500MB free space

### Development Tools
```bash
# Verify installations
node --version  # Should be v20.x+
npm --version   # Should be v10.x+
```

---

## Environment Setup

### 1. Clone and Install

```bash
cd /Users/macbookpro/Documents/LIS_Modern/frontend
npm install
```

### 2. Environment Variables

Copy the example environment file:
```bash
cp .env.example .env.local
```

Edit `.env.local` with your configuration:

```env
# GraphQL Backend
NEXT_PUBLIC_GRAPHQL_URL=http://localhost:8080/graphql
NEXT_PUBLIC_GRAPHQL_WS_URL=ws://localhost:8080/graphql

# Application
NEXT_PUBLIC_APP_NAME=Laboratory Information System
NEXT_PUBLIC_LAB_NAME=Your Lab Name
NEXT_PUBLIC_APP_URL=http://localhost:3000

# ABDM (Optional - for India's health stack)
NEXT_PUBLIC_ABDM_CLIENT_ID=your_client_id
NEXT_PUBLIC_ABDM_CLIENT_SECRET=your_secret
NEXT_PUBLIC_ABDM_BASE_URL=https://dev.abdm.gov.in
NEXT_PUBLIC_ABDM_ENV=sandbox

# Razorpay Payment Gateway
NEXT_PUBLIC_RAZORPAY_KEY_ID=rzp_test_xxx
NEXT_PUBLIC_RAZORPAY_KEY_SECRET=your_secret
NEXT_PUBLIC_RAZORPAY_ENV=test

# Email Service
NEXT_PUBLIC_EMAIL_PROVIDER=smtp
NEXT_PUBLIC_EMAIL_FROM_NAME=Lab Name
NEXT_PUBLIC_EMAIL_FROM_EMAIL=noreply@lab.com
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your_email@gmail.com
SMTP_PASS=your_app_password

# SMS Service
NEXT_PUBLIC_SMS_PROVIDER=msg91
MSG91_AUTH_KEY=your_auth_key
MSG91_SENDER_ID=LABLIS
MSG91_ROUTE=4

# WhatsApp Business
NEXT_PUBLIC_WHATSAPP_PROVIDER=twilio
TWILIO_WHATSAPP_ACCOUNT_SID=ACxxx
TWILIO_WHATSAPP_AUTH_TOKEN=your_token
TWILIO_WHATSAPP_FROM_NUMBER=whatsapp:+14155238886
```

### 3. Verify Build

```bash
npm run build
```

Expected output:
```
✓ Compiled successfully
✓ Linting and checking validity of types
✓ Collecting page data
✓ Generating static pages
```

---

## Backend Requirements

### GraphQL API Server

The frontend expects a GraphQL API at the configured URL.

**Required GraphQL Operations:**

**Queries:**
- `patients(page, pageSize, search, status)`
- `patient(id)`
- `orders(page, pageSize, status)`
- `order(id)`
- `samples(page, pageSize, status)`
- `sample(id)`
- `results(page, pageSize, status)`
- `result(id)`
- `reports(page, pageSize, status)`
- `qcRuns(testId)`
- `equipment(page, pageSize, status)`
- `inventory(page, pageSize, category)`
- `invoices(page, pageSize, paymentStatus)`

**Mutations:**
- `registerPatient(input)`
- `createOrder(input)`
- `collectSample(input)`
- `enterResult(input)`
- `verifyResult(id)`
- `approveResult(id)`
- `generateReport(input)`
- `createQCRun(input)`
- `scheduleMaintenance(input)`
- `recordTransaction(input)`
- `createInvoice(input)`
- `recordPayment(input)`

### WebSocket Server (Optional)

For real-time updates, set up a WebSocket server at port 9000:

```javascript
// Required events to emit:
- sample.collected
- sample.received
- sample.processing
- sample.completed
- result.entered
- result.verified
- result.critical
- report.generated
```

---

## External Services Configuration

### 1. ABDM Integration (India Only)

**Sign Up:**
1. Visit https://abdm.gov.in/
2. Register for sandbox access
3. Get Client ID and Secret

**Configuration:**
```env
NEXT_PUBLIC_ABDM_CLIENT_ID=your_client_id
NEXT_PUBLIC_ABDM_CLIENT_SECRET=your_secret
NEXT_PUBLIC_ABDM_ENV=sandbox  # or production
```

### 2. Razorpay Payment Gateway

**Sign Up:**
1. Visit https://razorpay.com/
2. Create account
3. Get API keys from Dashboard

**Test Mode:**
```env
NEXT_PUBLIC_RAZORPAY_KEY_ID=rzp_test_xxx
NEXT_PUBLIC_RAZORPAY_KEY_SECRET=test_secret
NEXT_PUBLIC_RAZORPAY_ENV=test
```

**Production:**
```env
NEXT_PUBLIC_RAZORPAY_KEY_ID=rzp_live_xxx
NEXT_PUBLIC_RAZORPAY_KEY_SECRET=live_secret
NEXT_PUBLIC_RAZORPAY_ENV=live
```

### 3. Email Service

**Option A: SMTP (Gmail)**
```env
NEXT_PUBLIC_EMAIL_PROVIDER=smtp
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your_email@gmail.com
SMTP_PASS=your_app_password  # Not regular password!
```

**Gmail App Password:** https://myaccount.google.com/apppasswords

**Option B: SendGrid**
```env
NEXT_PUBLIC_EMAIL_PROVIDER=sendgrid
SENDGRID_API_KEY=SG.xxx
```

**Option C: AWS SES**
```env
NEXT_PUBLIC_EMAIL_PROVIDER=ses
AWS_SES_REGION=us-east-1
AWS_SES_ACCESS_KEY_ID=xxx
AWS_SES_SECRET_ACCESS_KEY=xxx
```

### 4. SMS Service

**Option A: MSG91 (India)**
```env
NEXT_PUBLIC_SMS_PROVIDER=msg91
MSG91_AUTH_KEY=your_key
MSG91_SENDER_ID=LABLIS  # 6 characters
MSG91_ROUTE=4  # Transactional
```

**Option B: Twilio**
```env
NEXT_PUBLIC_SMS_PROVIDER=twilio
TWILIO_ACCOUNT_SID=ACxxx
TWILIO_AUTH_TOKEN=xxx
TWILIO_FROM_NUMBER=+1234567890
```

### 5. WhatsApp Business API

**Option A: Twilio**
```env
NEXT_PUBLIC_WHATSAPP_PROVIDER=twilio
TWILIO_WHATSAPP_ACCOUNT_SID=ACxxx
TWILIO_WHATSAPP_AUTH_TOKEN=xxx
TWILIO_WHATSAPP_FROM_NUMBER=whatsapp:+14155238886
```

**Option B: Meta Business**
```env
NEXT_PUBLIC_WHATSAPP_PROVIDER=meta
META_WHATSAPP_PHONE_NUMBER_ID=xxx
META_WHATSAPP_ACCESS_TOKEN=xxx
META_WHATSAPP_BUSINESS_ACCOUNT_ID=xxx
```

---

## Deployment Options

### Option 1: Vercel (Recommended for Quick Deploy)

#### Prerequisites
- Vercel account
- GitHub repository

#### Steps
1. **Push to GitHub**
```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/yourusername/lis-modern.git
git push -u origin main
```

2. **Deploy to Vercel**
```bash
npm i -g vercel
vercel login
vercel
```

3. **Configure Environment Variables**
- Go to Vercel Dashboard → Project → Settings → Environment Variables
- Add all variables from `.env.local`

4. **Deploy**
```bash
vercel --prod
```

#### Advantages
- Zero configuration
- Automatic HTTPS
- Global CDN
- Automatic scaling

---

### Option 2: AWS EC2

#### Prerequisites
- AWS account
- EC2 instance (t3.medium or higher)
- Ubuntu 22.04 LTS

#### Steps

1. **Connect to EC2**
```bash
ssh -i your-key.pem ubuntu@your-ec2-ip
```

2. **Install Node.js**
```bash
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
```

3. **Install PM2**
```bash
sudo npm install -g pm2
```

4. **Clone and Setup**
```bash
git clone https://github.com/yourusername/lis-modern.git
cd lis-modern/frontend
npm install
```

5. **Configure Environment**
```bash
nano .env.local
# Paste your environment variables
```

6. **Build**
```bash
npm run build
```

7. **Start with PM2**
```bash
pm2 start npm --name "lis-frontend" -- start
pm2 save
pm2 startup
```

8. **Configure Nginx**
```bash
sudo apt install nginx
sudo nano /etc/nginx/sites-available/lis
```

```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

```bash
sudo ln -s /etc/nginx/sites-available/lis /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

9. **SSL with Let's Encrypt**
```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

---

### Option 3: Docker

#### Dockerfile
Create `Dockerfile`:
```dockerfile
FROM node:20-alpine AS base

# Dependencies
FROM base AS deps
WORKDIR /app
COPY package*.json ./
RUN npm ci

# Builder
FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN npm run build

# Runner
FROM base AS runner
WORKDIR /app

ENV NODE_ENV production

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 3000

ENV PORT 3000

CMD ["node", "server.js"]
```

#### Build and Run
```bash
docker build -t lis-frontend .
docker run -p 3000:3000 --env-file .env.local lis-frontend
```

#### Docker Compose
Create `docker-compose.yml`:
```yaml
version: '3.8'

services:
  frontend:
    build: .
    ports:
      - "3000:3000"
    env_file:
      - .env.local
    restart: unless-stopped
    depends_on:
      - backend

  backend:
    image: lis-backend:latest
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://...
    restart: unless-stopped
```

```bash
docker-compose up -d
```

---

## Post-Deployment

### 1. Health Check

Visit these URLs:
- `http://your-domain.com` - Homepage should load
- `http://your-domain.com/login` - Login page
- `http://your-domain.com/dashboard` - Should redirect to login

### 2. Test External Services

#### ABDM
```bash
curl -X POST http://your-domain.com/api/abdm/test
```

#### Payment
```bash
curl -X POST http://your-domain.com/api/payment/test
```

#### Email
```bash
curl -X POST http://your-domain.com/api/email/test \
  -H "Content-Type: application/json" \
  -d '{"to":"test@example.com"}'
```

### 3. Monitoring Setup

#### PM2 Monitoring
```bash
pm2 monit
pm2 logs lis-frontend
```

#### Set Up Error Tracking (Sentry)
```bash
npm install @sentry/nextjs
npx @sentry/wizard -i nextjs
```

### 4. Performance Monitoring

Add to `.env.local`:
```env
NEXT_PUBLIC_GA_ID=G-XXXXXXXXXX
SENTRY_DSN=https://xxx@xxx.ingest.sentry.io/xxx
```

---

## Troubleshooting

### Build Errors

**Issue:** TypeScript errors during build
```bash
# Solution: Check types
npm run build 2>&1 | grep error
```

**Issue:** Out of memory
```bash
# Solution: Increase Node memory
NODE_OPTIONS='--max-old-space-size=4096' npm run build
```

### Runtime Errors

**Issue:** GraphQL connection failed
- Check `NEXT_PUBLIC_GRAPHQL_URL` in `.env.local`
- Verify backend is running
- Check network/firewall rules

**Issue:** External services not working
- Verify API keys in `.env.local`
- Check service provider dashboards for errors
- Review `/api/*` route implementations

### Performance Issues

**Issue:** Slow page loads
```bash
# Solution: Enable caching
# Add to next.config.ts
module.exports = {
  compress: true,
  poweredByHeader: false,
}
```

**Issue:** High memory usage
```bash
# Solution: Monitor with PM2
pm2 monit
pm2 restart lis-frontend --update-env
```

---

## Security Checklist

### Pre-Production
- [ ] All `.env` files in `.gitignore`
- [ ] HTTPS enabled (SSL certificate)
- [ ] CORS configured properly
- [ ] Rate limiting enabled
- [ ] Input validation on all forms
- [ ] SQL injection prevention (parameterized queries)
- [ ] XSS prevention (React default + CSP headers)
- [ ] CSRF tokens implemented
- [ ] Secure headers configured
- [ ] Authentication tokens in HTTP-only cookies
- [ ] Regular dependency updates scheduled

### CSP Headers (Add to `next.config.ts`)
```javascript
const securityHeaders = [
  {
    key: 'X-DNS-Prefetch-Control',
    value: 'on'
  },
  {
    key: 'X-XSS-Protection',
    value: '1; mode=block'
  },
  {
    key: 'X-Frame-Options',
    value: 'SAMEORIGIN'
  },
  {
    key: 'X-Content-Type-Options',
    value: 'nosniff'
  },
  {
    key: 'Referrer-Policy',
    value: 'origin-when-cross-origin'
  }
]

module.exports = {
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: securityHeaders,
      },
    ]
  },
}
```

---

## Backup Strategy

### Database Backups
```bash
# Daily automated backups
0 2 * * * /usr/local/bin/backup-db.sh
```

### Application Backups
```bash
# Weekly code snapshots
tar -czf lis-frontend-$(date +%Y%m%d).tar.gz /var/www/lis-modern/
```

---

## Support and Maintenance

### Regular Tasks
- **Daily:** Monitor error logs
- **Weekly:** Review performance metrics
- **Monthly:** Update dependencies
- **Quarterly:** Security audit

### Update Dependencies
```bash
npm outdated
npm update
npm audit fix
```

### Logs
```bash
# PM2 logs
pm2 logs lis-frontend --lines 100

# Nginx logs
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log
```

---

## Quick Reference

### Common Commands
```bash
# Development
npm run dev

# Build
npm run build

# Production
npm run start

# Lint
npm run lint

# Type check
npx tsc --noEmit
```

### Ports
- Frontend: 3000
- GraphQL API: 8080
- WebSocket: 9000
- Backend Services: 8081-8092

---

**For additional support, refer to:**
- [PROJECT_STATUS.md](./PROJECT_STATUS.md) - Implementation status
- [lib/services/README.md](./lib/services/README.md) - External services guide
- [README.md](./README.md) - Architecture documentation
