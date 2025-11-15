# LIS Modern - Frontend

Modern Laboratory Information System (LIS/LIMS) - Next.js Frontend Application

## Overview

This is the frontend application for LIS Modern, a comprehensive Laboratory Information System built with Next.js 16, React 19, TypeScript, and Tailwind CSS v4. The system provides role-based dashboards, real-time updates, and comprehensive laboratory management features.

## Tech Stack

- **Framework**: Next.js 16.0.1 with Turbopack
- **UI Library**: React 19
- **Language**: TypeScript 5
- **Styling**: Tailwind CSS v4 with new @theme syntax
- **State Management**: Zustand
- **Forms**: React Hook Form + Zod validation
- **GraphQL**: Apollo Client
- **Real-time**: Socket.IO for WebSocket connections
- **Components**: shadcn/ui component library

## Features

### 1. Authentication System
- Login with JWT authentication
- User registration with organization creation
- Password reset flow
- Role-based access control (RBAC)
- Protected routes with middleware

### 2. Role-Based Dashboards
- **Admin Dashboard**: KPIs, patient stats, revenue tracking
- **Lab Technician Dashboard**: Sample queue, equipment status, QC actions
- **Doctor Dashboard**: Patient reports, critical value alerts
- **Patient Dashboard**: Test reports, health tips, appointments

### 3. Patient Management
- 5-step patient registration form
- Personal information
- Contact details
- Medical history with emergency contacts
- Insurance information
- Multi-step validation

### 4. Sample Tracking
- Real-time sample status updates via WebSocket
- Status indicators (Collected, Received, Processing, Completed, Rejected)
- Priority labels (Routine, Urgent, STAT)
- Timeline visualization
- Advanced filtering

### 5. Result Entry & Verification
- Result entry forms with validation
- Out-of-range detection
- Critical value alerts (Panic, Critical, Warning, Normal)
- Auto-verification with confidence scores
- Real-time feedback

### 6. Reporting Module
- Report generation with status tracking
- Digital signature support
- Multi-channel delivery (Email, WhatsApp, SMS, Print)
- Download tracking
- Bulk operations

### 7. Real-Time Notifications
- Toast notification system
- WebSocket event handlers for:
  - Sample status changes
  - Critical value detection
  - Result verification
  - Report generation
  - Equipment maintenance alerts
  - Inventory low stock warnings
  - QC failure detection
  - General notifications

### 8. Additional Features
- Dark mode support
- Collapsible sidebar navigation
- Search functionality
- Responsive design
- Permission-based UI rendering

## Getting Started

### Prerequisites

- Node.js 18+ and npm/yarn/pnpm
- Backend services running (12 microservices on ports 8081-8092)
- PostgreSQL databases configured

### Installation

1. Clone the repository:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Configure environment variables:
```bash
cp .env.local.example .env.local
```

Edit `.env.local` with your configuration:
```env
# API Configuration
NEXT_PUBLIC_GRAPHQL_URL=http://localhost:8001/graphql
NEXT_PUBLIC_WS_URL=http://localhost:9000

# App Configuration
NEXT_PUBLIC_APP_NAME=LIS/LIMS System
NEXT_PUBLIC_APP_URL=http://localhost:3000

# Feature Flags
NEXT_PUBLIC_ENABLE_ABDM=true
NEXT_PUBLIC_ENABLE_WHATSAPP=true
NEXT_PUBLIC_ENABLE_PAYMENT=true
```

4. Run the development server:
```bash
npm run dev
```

5. Open [http://localhost:3000](http://localhost:3000) in your browser.

### Build for Production

```bash
npm run build
npm run start
```

## Project Structure

```
frontend/
├── app/                          # Next.js App Router
│   ├── (auth)/                   # Auth layout group
│   │   ├── login/                # Login page
│   │   ├── register/             # Registration page
│   │   └── reset-password/       # Password reset
│   ├── dashboard/                # Dashboard routes
│   │   ├── admin/                # Admin dashboard
│   │   ├── lab/                  # Lab technician dashboard
│   │   ├── doctor/               # Doctor dashboard
│   │   ├── patient/              # Patient dashboard
│   │   ├── patients/             # Patient management
│   │   │   ├── page.tsx          # Patient list
│   │   │   └── register/         # Patient registration form
│   │   ├── orders/               # Order management
│   │   ├── samples/              # Sample tracking
│   │   ├── results/              # Result management
│   │   │   └── [id]/enter/       # Result entry form
│   │   ├── reports/              # Report module
│   │   ├── qc/                   # Quality control
│   │   ├── equipment/            # Equipment management
│   │   ├── inventory/            # Inventory management
│   │   ├── billing/              # Billing & invoices
│   │   ├── organization/         # Organization settings
│   │   ├── users/                # User management
│   │   ├── settings/             # User settings
│   │   └── notifications/        # Notification center
│   ├── layout.tsx                # Root layout
│   └── page.tsx                  # Home page
├── components/                    # React components
│   ├── dashboard/                # Dashboard components
│   │   ├── header.tsx            # Global header
│   │   └── sidebar.tsx           # Sidebar navigation
│   ├── ui/                       # shadcn/ui components
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── input.tsx
│   │   ├── toast.tsx
│   │   └── ...
│   ├── providers.tsx             # Global providers wrapper
│   └── notification-provider.tsx # WebSocket notification handler
├── lib/                          # Utility libraries
│   ├── auth.ts                   # Authentication utilities
│   ├── apollo-client.ts          # Apollo Client configuration
│   ├── websocket.ts              # WebSocket client
│   ├── store.ts                  # Zustand stores
│   └── utils.ts                  # Helper functions
├── proxy.ts                      # Route protection middleware
├── tailwind.config.ts            # Tailwind CSS configuration
├── next.config.ts                # Next.js configuration
├── tsconfig.json                 # TypeScript configuration
└── package.json                  # Dependencies

```

## Key Architecture Decisions

### 1. Client/Server Components
- Auth pages use `'use client'` directive
- Forms require client-side rendering
- Added mounting checks to prevent prerendering errors

### 2. GraphQL Integration
- Apollo Client for GraphQL queries/mutations
- TypeScript interfaces for type-safe API responses
- Separate imports for client hooks (`@apollo/client/react`)

### 3. Real-Time Updates
- Socket.IO WebSocket client
- Event handlers in NotificationProvider
- Automatic UI updates on events

### 4. Form Validation
- React Hook Form for form state management
- Zod for schema validation
- Multi-step forms with step-by-step validation

### 5. State Management
- Zustand for global state (Auth, Notifications, Sidebar)
- Local state for component-specific data
- Persistent auth state in localStorage

### 6. Route Protection
- Middleware (proxy.ts) for JWT validation
- Role-based route access
- Permission checking utilities

## API Integration

### GraphQL Endpoints
- **Main API**: http://localhost:8001/graphql
- **Patient Service**: http://localhost:8081/graphql
- **Sample Service**: http://localhost:8082/graphql
- **Order Service**: http://localhost:8083/graphql
- **Test Service**: http://localhost:8084/graphql
- **Result Service**: http://localhost:8085/graphql
- **Report Service**: http://localhost:8086/graphql
- **QC Service**: http://localhost:8087/graphql
- **Equipment Service**: http://localhost:8088/graphql
- **Inventory Service**: http://localhost:8089/graphql
- **Billing Service**: http://localhost:8090/graphql
- **User Service**: http://localhost:8091/graphql
- **Organization Service**: http://localhost:8092/graphql

### WebSocket Events
- `sample.status_changed`
- `result.critical_value_detected`
- `result.verified`
- `report.generated`
- `equipment.maintenance_due`
- `inventory.low_stock`
- `qc.failure_detected`
- `notification.created`

## User Roles & Permissions

### Roles
- `SUPER_ADMIN`: Full system access
- `ADMIN`: Organization administration
- `LAB_MANAGER`: Lab operations management
- `LAB_TECHNICIAN`: Sample and result processing
- `DOCTOR`: View patient reports
- `PATIENT`: View own reports
- `BILLING_STAFF`: Invoice management

### Permissions
Each role has specific permissions enforced at:
- Route level (middleware)
- Component level (conditional rendering)
- API level (GraphQL resolvers)

## Development Guidelines

### Code Style
- Use TypeScript strict mode
- Follow component naming conventions
- Prefer functional components with hooks
- Use Tailwind CSS for styling

### Component Structure
```tsx
'use client';

import { useState, useEffect } from 'react';
import { Component } from '@/components/ui/component';

interface Props {
  // Props interface
}

export default function PageName({ }: Props) {
  // State and hooks

  // Effects

  // Handlers

  // Render
  return (
    <div>
      {/* JSX */}
    </div>
  );
}
```

### Form Validation
```tsx
const schema = z.object({
  field: z.string().min(1, 'Field is required'),
});

type FormValues = z.infer<typeof schema>;

const form = useForm<FormValues>({
  resolver: zodResolver(schema),
  defaultValues: { field: '' },
});
```

## Testing

### Run Tests
```bash
npm run test        # Run all tests
npm run test:watch  # Watch mode
npm run test:coverage  # Coverage report
```

## Troubleshooting

### Common Issues

1. **Build errors with fonts**
   - Removed Geist fonts to avoid Turbopack issues
   - Using system fonts instead

2. **Prerendering errors**
   - Added `export const dynamic = 'force-dynamic'` to auth pages
   - Added mounting checks for client-side only components

3. **TypeScript errors**
   - Ensure all GraphQL responses have TypeScript interfaces
   - Use proper mutation typing: `useMutation<ResponseType>`

4. **Apollo Client errors**
   - Import hooks from `@apollo/client/react`
   - Import gql from `@apollo/client`

## Performance Optimization

- Turbopack for faster compilation
- Static page generation where possible
- Code splitting for routes
- Lazy loading for heavy components
- WebSocket connection pooling

## Security Features

- JWT authentication with HTTP-only cookies
- CSRF protection
- XSS prevention with React sanitization
- Role-based access control
- Permission validation at multiple layers
- Secure WebSocket connections

## Browser Support

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)

## Contributing

1. Follow the existing code style
2. Write TypeScript with strict types
3. Add tests for new features
4. Update documentation
5. Follow commit message conventions

## License

Proprietary - All rights reserved

## Support

For issues or questions, please contact the development team.
