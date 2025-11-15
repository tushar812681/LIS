# Deployment Guide - LIS Modern Frontend

## Pre-Deployment Checklist

### 1. Environment Configuration
- [ ] Copy `.env.local.example` to `.env.local`
- [ ] Configure production API endpoints
- [ ] Set up WebSocket URL
- [ ] Configure feature flags
- [ ] Verify all environment variables are set

### 2. Code Quality
- [x] All TypeScript errors resolved
- [x] Build completes successfully
- [x] No TODO/FIXME comments remaining
- [x] No unused imports
- [x] Proper error handling implemented

### 3. Security
- [x] JWT authentication configured
- [x] Route protection middleware (proxy.ts) active
- [x] Role-based access control implemented
- [x] Permission checks at all levels
- [x] XSS prevention through React
- [ ] CORS configuration verified
- [ ] CSP headers configured

### 4. Performance
- [x] Code splitting enabled
- [x] Turbopack compilation
- [x] Static page generation where possible
- [ ] Image optimization configured
- [ ] CDN setup for static assets
- [ ] Gzip/Brotli compression enabled

### 5. Testing
- [ ] Unit tests passing
- [ ] Integration tests passing
- [ ] E2E tests passing
- [ ] Cross-browser testing complete
- [ ] Mobile responsiveness verified

## Environment Variables

### Development (.env.local)
```env
NEXT_PUBLIC_GRAPHQL_URL=http://localhost:8001/graphql
NEXT_PUBLIC_WS_URL=http://localhost:9000
NEXT_PUBLIC_APP_NAME=LIS/LIMS System
NEXT_PUBLIC_APP_URL=http://localhost:3000
NEXT_PUBLIC_ENABLE_ABDM=true
NEXT_PUBLIC_ENABLE_WHATSAPP=true
NEXT_PUBLIC_ENABLE_PAYMENT=true
```

### Production (.env.production)
```env
NEXT_PUBLIC_GRAPHQL_URL=https://api.yourdomain.com/graphql
NEXT_PUBLIC_WS_URL=https://ws.yourdomain.com
NEXT_PUBLIC_APP_NAME=LIS/LIMS System
NEXT_PUBLIC_APP_URL=https://yourdomain.com
NEXT_PUBLIC_ENABLE_ABDM=true
NEXT_PUBLIC_ENABLE_WHATSAPP=true
NEXT_PUBLIC_ENABLE_PAYMENT=true
```

## Build Process

### Local Build
```bash
npm run build
npm run start
```

### Docker Build
```dockerfile
FROM node:20-alpine AS base

# Install dependencies only when needed
FROM base AS deps
RUN apk add --no-cache libc6-compat
WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

# Rebuild the source code only when needed
FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

ENV NEXT_TELEMETRY_DISABLED=1

RUN npm run build

# Production image, copy all the files and run next
FROM base AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 3000

ENV PORT=3000
ENV HOSTNAME="0.0.0.0"

CMD ["node", "server.js"]
```

### Build and Run Docker
```bash
docker build -t lis-modern-frontend .
docker run -p 3000:3000 --env-file .env.production lis-modern-frontend
```

## Deployment Platforms

### Vercel
1. Connect GitHub repository
2. Configure environment variables in Vercel dashboard
3. Deploy automatically on push to main branch

```bash
vercel --prod
```

### AWS (EC2/ECS)
1. Build Docker image
2. Push to ECR
3. Deploy to ECS cluster

```bash
# Build and tag
docker build -t lis-modern-frontend .
docker tag lis-modern-frontend:latest <account-id>.dkr.ecr.region.amazonaws.com/lis-modern-frontend:latest

# Push to ECR
aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <account-id>.dkr.ecr.region.amazonaws.com
docker push <account-id>.dkr.ecr.region.amazonaws.com/lis-modern-frontend:latest
```

### Digital Ocean
1. Create App Platform app
2. Connect repository
3. Configure build settings
4. Add environment variables

### Self-Hosted (PM2)
```bash
# Build
npm run build

# Install PM2 globally
npm install -g pm2

# Start application
pm2 start npm --name "lis-frontend" -- start

# Save PM2 configuration
pm2 save

# Setup PM2 to start on boot
pm2 startup
```

## Nginx Configuration

### Reverse Proxy
```nginx
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket support
    location /socket.io/ {
        proxy_pass http://localhost:9000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
    }
}
```

### SSL with Let's Encrypt
```bash
# Install certbot
sudo apt-get install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com

# Auto-renewal
sudo certbot renew --dry-run
```

## Monitoring

### Health Check Endpoint
Create `/app/api/health/route.ts`:
```typescript
export async function GET() {
  return Response.json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    version: process.env.npm_package_version,
  });
}
```

### Logging
- Use structured logging
- Configure log levels for production
- Set up log aggregation (e.g., CloudWatch, Datadog)

### Error Tracking
- Integrate Sentry or similar service
- Configure source maps for production
- Set up alerts for critical errors

## Performance Optimization

### 1. Caching Strategy
```typescript
// next.config.ts
export default {
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          {
            key: 'Cache-Control',
            value: 'public, max-age=31536000, immutable',
          },
        ],
      },
    ];
  },
};
```

### 2. CDN Configuration
- Configure CloudFront or similar CDN
- Set up cache invalidation on deployment
- Enable compression

### 3. Database Query Optimization
- Implement GraphQL query batching
- Use DataLoader for efficient data fetching
- Enable Apollo Client cache

## Security Hardening

### 1. HTTP Security Headers
```typescript
// next.config.ts
export default {
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          { key: 'X-DNS-Prefetch-Control', value: 'on' },
          { key: 'Strict-Transport-Security', value: 'max-age=63072000; includeSubDomains; preload' },
          { key: 'X-Frame-Options', value: 'SAMEORIGIN' },
          { key: 'X-Content-Type-Options', value: 'nosniff' },
          { key: 'X-XSS-Protection', value: '1; mode=block' },
          { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
        ],
      },
    ];
  },
};
```

### 2. Content Security Policy
```typescript
const ContentSecurityPolicy = `
  default-src 'self';
  script-src 'self' 'unsafe-eval' 'unsafe-inline';
  style-src 'self' 'unsafe-inline';
  img-src 'self' data: blob: https:;
  font-src 'self' data:;
  connect-src 'self' wss: https:;
  frame-ancestors 'self';
`;
```

### 3. Rate Limiting
- Implement rate limiting at API gateway
- Use Vercel Edge Middleware for rate limiting
- Configure backend rate limits

## Backup and Recovery

### 1. Database Backups
- Automated daily backups
- Point-in-time recovery enabled
- Backup retention policy

### 2. Code Versioning
- Git tags for releases
- Semantic versioning
- Changelog maintenance

### 3. Rollback Strategy
```bash
# Rollback to previous version
vercel rollback

# Or with PM2
pm2 stop lis-frontend
pm2 delete lis-frontend
git checkout <previous-tag>
npm run build
pm2 start npm --name "lis-frontend" -- start
```

## Post-Deployment

### 1. Smoke Tests
- [ ] Login functionality works
- [ ] Dashboard loads correctly
- [ ] Forms submit successfully
- [ ] WebSocket connections establish
- [ ] Real-time updates functioning

### 2. Monitoring Setup
- [ ] Error tracking active
- [ ] Performance monitoring configured
- [ ] Uptime monitoring enabled
- [ ] Alert notifications set up

### 3. Documentation
- [ ] Update deployment documentation
- [ ] Document any configuration changes
- [ ] Update API endpoint documentation
- [ ] Share release notes with team

## Troubleshooting

### Build Failures
```bash
# Clear Next.js cache
rm -rf .next

# Clear node_modules and reinstall
rm -rf node_modules
npm install

# Rebuild
npm run build
```

### Runtime Errors
```bash
# Check logs
pm2 logs lis-frontend

# Check Docker logs
docker logs <container-id>

# Check system resources
htop
df -h
```

### WebSocket Connection Issues
- Verify WebSocket URL is correct
- Check firewall rules
- Ensure WebSocket upgrade headers are set
- Verify SSL/TLS configuration for wss://

## Scaling Considerations

### Horizontal Scaling
- Deploy multiple instances behind load balancer
- Use sticky sessions for WebSocket connections
- Configure session store (Redis)

### Vertical Scaling
- Increase instance size based on metrics
- Monitor CPU and memory usage
- Optimize bundle size

## Maintenance Windows

### Planned Maintenance
1. Schedule during low-traffic periods
2. Notify users in advance
3. Enable maintenance mode page
4. Monitor deployment closely
5. Verify functionality post-deployment

### Maintenance Mode
Create `/app/maintenance/page.tsx` and enable via environment variable.

## Support Contacts

- **DevOps Team**: devops@yourdomain.com
- **Backend Team**: backend@yourdomain.com
- **Frontend Team**: frontend@yourdomain.com
- **Emergency**: +1-XXX-XXX-XXXX

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0   | 2025-11-06 | Initial release |
