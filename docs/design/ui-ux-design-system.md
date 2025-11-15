# UI/UX Design System
## LIS Modern - Complete Visual Design Language

**Version**: 1.0.0
**Last Updated**: 2024-11-05
**Design Framework**: shadcn/ui + Tailwind CSS

---

## Table of Contents

1. [Design Principles](#design-principles)
2. [Color System](#color-system)
3. [Typography](#typography)
4. [Spacing & Layout](#spacing--layout)
5. [Component Library](#component-library)
6. [Iconography](#iconography)
7. [Data Visualization](#data-visualization)
8. [Accessibility](#accessibility)
9. [Responsive Design](#responsive-design)
10. [Animation & Motion](#animation--motion)

---

## 1. Design Principles

### 1.1 Core Principles

#### **Clarity First**
- Medical data requires absolute clarity
- No ambiguity in critical information
- Clear visual hierarchy
- Scannable layouts

#### **Efficiency**
- Minimize clicks for common tasks
- Keyboard shortcuts everywhere
- Smart defaults and auto-fill
- Batch operations support

#### **Trust & Reliability**
- Professional appearance
- Consistent behavior
- Clear feedback on all actions
- No surprises

#### **Accessibility**
- WCAG 2.1 AA compliant
- Keyboard navigation
- Screen reader friendly
- High contrast options

#### **Calm & Pleasant**
- Soothing color palette
- Generous white space
- Smooth transitions
- No aggressive alerts (except critical)

---

## 2. Color System

### 2.1 Primary Palette

```typescript
// Tailwind config (tailwind.config.ts)
export const colors = {
  // Primary - Medical Blue (Trust, Calm)
  primary: {
    50: '#eff6ff',   // Very light blue
    100: '#dbeafe',
    200: '#bfdbfe',
    300: '#93c5fd',
    400: '#60a5fa',
    500: '#3b82f6',  // Main primary
    600: '#2563eb',
    700: '#1d4ed8',
    800: '#1e40af',
    900: '#1e3a8a',
    950: '#172554',
  },

  // Secondary - Lab Green (Success, Health)
  secondary: {
    50: '#f0fdf4',
    100: '#dcfce7',
    200: '#bbf7d0',
    300: '#86efac',
    400: '#4ade80',
    500: '#22c55e',  // Main secondary
    600: '#16a34a',
    700: '#15803d',
    800: '#166534',
    900: '#14532d',
    950: '#052e16',
  },

  // Accent - Healthcare Purple
  accent: {
    50: '#faf5ff',
    100: '#f3e8ff',
    200: '#e9d5ff',
    300: '#d8b4fe',
    400: '#c084fc',
    500: '#a855f7',  // Main accent
    600: '#9333ea',
    700: '#7e22ce',
    800: '#6b21a8',
    900: '#581c87',
    950: '#3b0764',
  },

  // Semantic Colors
  success: {
    light: '#86efac',
    DEFAULT: '#22c55e',
    dark: '#15803d',
  },

  warning: {
    light: '#fde68a',
    DEFAULT: '#f59e0b',
    dark: '#d97706',
  },

  error: {
    light: '#fca5a5',
    DEFAULT: '#ef4444',
    dark: '#dc2626',
  },

  critical: {
    light: '#fca5a5',
    DEFAULT: '#dc2626',
    dark: '#991b1b',
  },

  // Neutral - Gray Scale
  neutral: {
    50: '#f8fafc',
    100: '#f1f5f9',
    200: '#e2e8f0',
    300: '#cbd5e1',
    400: '#94a3b8',
    500: '#64748b',
    600: '#475569',
    700: '#334155',
    800: '#1e293b',
    900: '#0f172a',
    950: '#020617',
  },
}
```

### 2.2 Color Usage Guidelines

| Color | Usage | Examples |
|-------|-------|----------|
| **Primary Blue** | Main actions, links, focus states | Buttons, links, selected items |
| **Secondary Green** | Success states, health indicators | Completed tasks, normal results |
| **Accent Purple** | Highlights, premium features | Analytics, AI features |
| **Success Green** | Positive feedback, normal ranges | Test results in range |
| **Warning Orange** | Caution, pending actions | Borderline results, expiring items |
| **Error Red** | Errors, failures, rejections | Failed tests, rejected samples |
| **Critical Red** | Critical values, urgent alerts | Critical lab values, STAT orders |
| **Neutral Gray** | Text, borders, backgrounds | Body text, dividers, cards |

### 2.3 Status Color System

```typescript
// Status indicators
const statusColors = {
  // Sample status
  collected: 'bg-blue-100 text-blue-800 border-blue-300',
  inTransit: 'bg-amber-100 text-amber-800 border-amber-300',
  received: 'bg-green-100 text-green-800 border-green-300',
  processing: 'bg-purple-100 text-purple-800 border-purple-300',
  completed: 'bg-emerald-100 text-emerald-800 border-emerald-300',
  rejected: 'bg-red-100 text-red-800 border-red-300',

  // Order priority
  stat: 'bg-red-500 text-white',
  urgent: 'bg-orange-500 text-white',
  routine: 'bg-gray-500 text-white',

  // Payment status
  paid: 'bg-green-100 text-green-800',
  pending: 'bg-yellow-100 text-yellow-800',
  overdue: 'bg-red-100 text-red-800',

  // QC status
  inControl: 'bg-green-100 text-green-800',
  outOfControl: 'bg-red-100 text-red-800',
  warning: 'bg-orange-100 text-orange-800',
}
```

---

## 3. Typography

### 3.1 Font Families

```typescript
// Tailwind config
export const fontFamily = {
  sans: ['Inter', 'system-ui', 'sans-serif'],
  mono: ['JetBrains Mono', 'Consolas', 'monospace'],
  display: ['Cal Sans', 'Inter', 'sans-serif'],
}
```

**Fonts:**
- **Inter**: Primary UI font (clean, modern, highly legible)
- **JetBrains Mono**: Code, patient IDs, barcodes
- **Cal Sans**: Display headings (optional, for marketing)

### 3.2 Type Scale

```typescript
// Tailwind type scale
export const fontSize = {
  xs: ['0.75rem', { lineHeight: '1rem' }],      // 12px
  sm: ['0.875rem', { lineHeight: '1.25rem' }],  // 14px
  base: ['1rem', { lineHeight: '1.5rem' }],     // 16px - body text
  lg: ['1.125rem', { lineHeight: '1.75rem' }],  // 18px
  xl: ['1.25rem', { lineHeight: '1.75rem' }],   // 20px
  '2xl': ['1.5rem', { lineHeight: '2rem' }],    // 24px - section headings
  '3xl': ['1.875rem', { lineHeight: '2.25rem' }], // 30px
  '4xl': ['2.25rem', { lineHeight: '2.5rem' }],   // 36px - page titles
  '5xl': ['3rem', { lineHeight: '1' }],           // 48px
  '6xl': ['3.75rem', { lineHeight: '1' }],        // 60px
}
```

### 3.3 Font Weights

```typescript
export const fontWeight = {
  light: '300',
  normal: '400',   // Body text
  medium: '500',   // Buttons, labels
  semibold: '600', // Headings
  bold: '700',     // Important headings
  extrabold: '800',
}
```

### 3.4 Typography Usage

| Element | Size | Weight | Color |
|---------|------|--------|-------|
| **Page Title** | 2xl (24px) | Semibold | neutral-900 |
| **Section Heading** | xl (20px) | Semibold | neutral-800 |
| **Subsection** | lg (18px) | Medium | neutral-700 |
| **Body Text** | base (16px) | Normal | neutral-700 |
| **Small Text** | sm (14px) | Normal | neutral-600 |
| **Caption** | xs (12px) | Normal | neutral-500 |
| **Label** | sm (14px) | Medium | neutral-700 |
| **Button** | sm (14px) | Medium | white |
| **Table Header** | xs (12px) | Semibold | neutral-600 |
| **Table Cell** | sm (14px) | Normal | neutral-700 |
| **Code/ID** | sm (14px) | Mono | neutral-800 |

---

## 4. Spacing & Layout

### 4.1 Spacing Scale

```typescript
// Tailwind spacing (8px base unit)
export const spacing = {
  px: '1px',
  0: '0',
  0.5: '0.125rem', // 2px
  1: '0.25rem',    // 4px
  1.5: '0.375rem', // 6px
  2: '0.5rem',     // 8px - base unit
  2.5: '0.625rem', // 10px
  3: '0.75rem',    // 12px
  3.5: '0.875rem', // 14px
  4: '1rem',       // 16px
  5: '1.25rem',    // 20px
  6: '1.5rem',     // 24px
  7: '1.75rem',    // 28px
  8: '2rem',       // 32px
  9: '2.25rem',    // 36px
  10: '2.5rem',    // 40px
  12: '3rem',      // 48px
  16: '4rem',      // 64px
  20: '5rem',      // 80px
  24: '6rem',      // 96px
}
```

### 4.2 Layout Grid

```typescript
// Container widths
export const maxWidth = {
  sm: '640px',   // Mobile
  md: '768px',   // Tablet
  lg: '1024px',  // Laptop
  xl: '1280px',  // Desktop
  '2xl': '1536px', // Large desktop
  full: '100%',
}

// Common container
.container {
  max-width: 1280px;
  margin: 0 auto;
  padding: 0 1.5rem;
}
```

### 4.3 Component Spacing

| Component | Padding | Margin | Gap |
|-----------|---------|--------|-----|
| **Card** | p-6 (24px) | mb-4 (16px) | - |
| **Button** | px-4 py-2 (16px, 8px) | - | - |
| **Form Field** | p-3 (12px) | mb-4 (16px) | - |
| **Table Cell** | p-3 (12px) | - | - |
| **Modal** | p-6 (24px) | - | - |
| **Section** | py-8 (32px) | - | gap-6 (24px) |
| **Page** | p-6 (24px) | - | gap-8 (32px) |

---

## 5. Component Library

### 5.1 Button Variants

```typescript
// Using shadcn/ui Button component
<Button variant="default">Primary Action</Button>
<Button variant="secondary">Secondary Action</Button>
<Button variant="outline">Outline Button</Button>
<Button variant="ghost">Ghost Button</Button>
<Button variant="link">Link Button</Button>
<Button variant="destructive">Delete/Cancel</Button>

// Sizes
<Button size="sm">Small Button</Button>
<Button size="default">Default Button</Button>
<Button size="lg">Large Button</Button>
<Button size="icon"><Icon /></Button>

// States
<Button disabled>Disabled</Button>
<Button loading>Loading...</Button>
```

**Visual Specifications:**
```css
/* Default Button */
.btn-default {
  background: rgb(59, 130, 246); /* primary-500 */
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-default:hover {
  background: rgb(37, 99, 235); /* primary-600 */
  transform: translateY(-1px);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.btn-default:active {
  transform: translateY(0);
}
```

### 5.2 Input Fields

```typescript
// Text Input
<Input
  type="text"
  placeholder="Enter patient name"
  className="w-full"
/>

// With Label
<div className="space-y-2">
  <Label htmlFor="mrn">MRN Number</Label>
  <Input id="mrn" type="text" />
  <p className="text-sm text-neutral-500">Patient medical record number</p>
</div>

// With Error
<Input
  type="email"
  className="border-red-500"
  aria-invalid="true"
/>
<p className="text-sm text-red-600">Invalid email address</p>
```

**Visual Specifications:**
```css
.input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid rgb(203, 213, 225); /* neutral-300 */
  border-radius: 0.375rem;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.input:focus {
  outline: none;
  border-color: rgb(59, 130, 246); /* primary-500 */
  ring: 2px solid rgba(59, 130, 246, 0.2);
}

.input:disabled {
  background: rgb(241, 245, 249); /* neutral-100 */
  cursor: not-allowed;
}
```

### 5.3 Cards

```typescript
// Basic Card
<Card>
  <CardHeader>
    <CardTitle>Patient Information</CardTitle>
    <CardDescription>View and edit patient details</CardDescription>
  </CardHeader>
  <CardContent>
    {/* Content */}
  </CardContent>
  <CardFooter>
    <Button>Save Changes</Button>
  </CardFooter>
</Card>

// Stat Card
<Card className="border-l-4 border-l-primary-500">
  <CardContent className="pt-6">
    <div className="text-sm text-neutral-600">Total Samples</div>
    <div className="text-3xl font-bold text-neutral-900">1,234</div>
    <div className="text-sm text-green-600">+12% from last month</div>
  </CardContent>
</Card>
```

### 5.4 Tables

```typescript
<Table>
  <TableHeader>
    <TableRow>
      <TableHead>MRN</TableHead>
      <TableHead>Patient Name</TableHead>
      <TableHead>Test</TableHead>
      <TableHead>Status</TableHead>
      <TableHead className="text-right">Actions</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    <TableRow>
      <TableCell className="font-mono">MRN-2024-001</TableCell>
      <TableCell>John Doe</TableCell>
      <TableCell>Complete Blood Count</TableCell>
      <TableCell>
        <Badge variant="success">Completed</Badge>
      </TableCell>
      <TableCell className="text-right">
        <Button variant="ghost" size="sm">View</Button>
      </TableCell>
    </TableRow>
  </TableBody>
</Table>
```

**Styling:**
```css
.table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0;
}

.table-header {
  background: rgb(248, 250, 252); /* neutral-50 */
  border-bottom: 1px solid rgb(226, 232, 240); /* neutral-200 */
}

.table-row {
  border-bottom: 1px solid rgb(241, 245, 249); /* neutral-100 */
  transition: background 0.2s;
}

.table-row:hover {
  background: rgb(248, 250, 252); /* neutral-50 */
}

.table-cell {
  padding: 0.75rem;
  text-align: left;
}
```

### 5.5 Badges

```typescript
// Status badges
<Badge variant="default">Default</Badge>
<Badge variant="success">Completed</Badge>
<Badge variant="warning">Pending</Badge>
<Badge variant="error">Failed</Badge>
<Badge variant="critical">Critical</Badge>

// Priority badges
<Badge className="bg-red-500">STAT</Badge>
<Badge className="bg-orange-500">Urgent</Badge>
<Badge className="bg-gray-500">Routine</Badge>
```

### 5.6 Modals/Dialogs

```typescript
<Dialog>
  <DialogTrigger asChild>
    <Button>Open Dialog</Button>
  </DialogTrigger>
  <DialogContent className="sm:max-w-[425px]">
    <DialogHeader>
      <DialogTitle>Confirm Action</DialogTitle>
      <DialogDescription>
        Are you sure you want to reject this sample?
      </DialogDescription>
    </DialogHeader>
    <div className="grid gap-4 py-4">
      {/* Content */}
    </div>
    <DialogFooter>
      <Button variant="outline">Cancel</Button>
      <Button variant="destructive">Confirm</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
```

### 5.7 Alerts

```typescript
// Success Alert
<Alert variant="success">
  <CheckCircle className="h-4 w-4" />
  <AlertTitle>Success</AlertTitle>
  <AlertDescription>
    Patient record created successfully
  </AlertDescription>
</Alert>

// Critical Alert
<Alert variant="destructive">
  <AlertTriangle className="h-4 w-4" />
  <AlertTitle>Critical Value Detected</AlertTitle>
  <AlertDescription>
    Glucose: 450 mg/dL (Critical High)
  </AlertDescription>
</Alert>
```

### 5.8 Data Display Components

```typescript
// Key-Value Pairs
<dl className="grid grid-cols-2 gap-4">
  <div>
    <dt className="text-sm font-medium text-neutral-600">Patient Name</dt>
    <dd className="text-base font-semibold text-neutral-900">John Doe</dd>
  </div>
  <div>
    <dt className="text-sm font-medium text-neutral-600">Age</dt>
    <dd className="text-base font-semibold text-neutral-900">45 years</dd>
  </div>
</dl>

// Progress Bar
<div className="space-y-2">
  <div className="flex justify-between text-sm">
    <span>Processing</span>
    <span>75%</span>
  </div>
  <Progress value={75} className="h-2" />
</div>

// Skeleton Loading
<div className="space-y-3">
  <Skeleton className="h-4 w-full" />
  <Skeleton className="h-4 w-3/4" />
  <Skeleton className="h-4 w-1/2" />
</div>
```

---

## 6. Iconography

### 6.1 Icon System

**Library:** Lucide Icons (React)

```typescript
import {
  User, Users, TestTube, FileText, Calendar,
  Clock, CheckCircle, XCircle, AlertTriangle,
  TrendingUp, TrendingDown, Activity, Beaker,
  Microscope, Syringe, Stethoscope, Pill,
  DollarSign, CreditCard, FileBarChart,
  Settings, Bell, Search, Filter, Download
} from 'lucide-react'
```

### 6.2 Icon Sizes

```typescript
const iconSizes = {
  sm: 'h-4 w-4',   // 16px - inline with text
  md: 'h-5 w-5',   // 20px - buttons
  lg: 'h-6 w-6',   // 24px - headings
  xl: 'h-8 w-8',   // 32px - features
  '2xl': 'h-10 w-10', // 40px - empty states
}

// Usage
<User className="h-5 w-5" />
```

### 6.3 Icon Colors

```typescript
// By context
<CheckCircle className="h-5 w-5 text-green-600" />  // Success
<AlertTriangle className="h-5 w-5 text-red-600" />  // Error
<Clock className="h-5 w-5 text-orange-600" />       // Warning
<Info className="h-5 w-5 text-blue-600" />          // Info
```

---

## 7. Data Visualization

### 7.1 Chart Colors

```typescript
// Using Recharts / Tremor
export const chartColors = {
  primary: ['#3b82f6', '#60a5fa', '#93c5fd'],
  success: ['#22c55e', '#4ade80', '#86efac'],
  warning: ['#f59e0b', '#fbbf24', '#fcd34d'],
  error: ['#ef4444', '#f87171', '#fca5a5'],
  multi: [
    '#3b82f6', // blue
    '#22c55e', // green
    '#f59e0b', // orange
    '#a855f7', // purple
    '#06b6d4', // cyan
    '#ec4899', // pink
  ]
}
```

### 7.2 Chart Components

```typescript
// Line Chart - TAT Trends
<LineChart data={tatData}>
  <CartesianGrid strokeDasharray="3 3" />
  <XAxis dataKey="date" />
  <YAxis />
  <Tooltip />
  <Line type="monotone" dataKey="averageTAT" stroke="#3b82f6" />
</LineChart>

// Bar Chart - Test Volume
<BarChart data={volumeData}>
  <CartesianGrid strokeDasharray="3 3" />
  <XAxis dataKey="month" />
  <YAxis />
  <Tooltip />
  <Bar dataKey="tests" fill="#22c55e" />
</BarChart>

// Area Chart - QC Trends
<AreaChart data={qcData}>
  <CartesianGrid strokeDasharray="3 3" />
  <XAxis dataKey="date" />
  <YAxis />
  <Tooltip />
  <Area type="monotone" dataKey="value" stroke="#3b82f6" fill="#93c5fd" />
</AreaChart>
```

---

## 8. Accessibility

### 8.1 WCAG 2.1 AA Compliance

**Color Contrast:**
- Normal text: 4.5:1 minimum
- Large text (18px+): 3:1 minimum
- UI components: 3:1 minimum

**Keyboard Navigation:**
- All interactive elements keyboard accessible
- Visible focus indicators
- Logical tab order
- Skip links for main content

**Screen Readers:**
- Semantic HTML
- ARIA labels where needed
- Alt text for images
- Descriptive link text

### 8.2 Focus States

```css
/* Default focus ring */
*:focus-visible {
  outline: 2px solid rgb(59, 130, 246);
  outline-offset: 2px;
}

/* Interactive elements */
button:focus-visible,
input:focus-visible,
select:focus-visible {
  ring: 2px;
  ring-color: rgb(59, 130, 246);
  ring-opacity: 0.5;
}
```

### 8.3 Alternative Text

```typescript
// Images
<img src="/logo.png" alt="LIS Modern - Laboratory Information System" />

// Icons (decorative)
<Icon aria-hidden="true" />

// Icons (meaningful)
<Icon aria-label="Critical value alert" />

// Buttons with only icons
<Button aria-label="Edit patient record">
  <Edit className="h-4 w-4" />
</Button>
```

---

## 9. Responsive Design

### 9.1 Breakpoints

```typescript
export const screens = {
  sm: '640px',   // Mobile landscape, small tablets
  md: '768px',   // Tablets
  lg: '1024px',  // Laptops
  xl: '1280px',  // Desktops
  '2xl': '1536px', // Large desktops
}
```

### 9.2 Responsive Patterns

```typescript
// Mobile-first approach
<div className="
  grid
  grid-cols-1      // Mobile: 1 column
  md:grid-cols-2   // Tablet: 2 columns
  lg:grid-cols-3   // Desktop: 3 columns
  gap-4
">
  {/* Cards */}
</div>

// Responsive text
<h1 className="
  text-2xl        // Mobile: 24px
  md:text-3xl     // Tablet: 30px
  lg:text-4xl     // Desktop: 36px
  font-bold
">
  Page Title
</h1>

// Responsive padding
<div className="
  p-4             // Mobile: 16px
  md:p-6          // Tablet: 24px
  lg:p-8          // Desktop: 32px
">
  {/* Content */}
</div>
```

### 9.3 Mobile Navigation

```typescript
// Mobile: Hamburger menu
// Desktop: Full navigation

<nav className="flex items-center justify-between">
  <Logo />

  {/* Mobile menu button */}
  <Button variant="ghost" className="md:hidden">
    <Menu className="h-6 w-6" />
  </Button>

  {/* Desktop navigation */}
  <div className="hidden md:flex items-center gap-6">
    <NavLink href="/patients">Patients</NavLink>
    <NavLink href="/samples">Samples</NavLink>
    <NavLink href="/reports">Reports</NavLink>
  </div>
</nav>
```

---

## 10. Animation & Motion

### 10.1 Animation Principles

- **Purposeful**: Only animate to provide feedback or guide attention
- **Quick**: 200-300ms for most interactions
- **Smooth**: Use easing functions for natural feel
- **Respectful**: Respect `prefers-reduced-motion`

### 10.2 Transition Speeds

```typescript
export const transitionDuration = {
  fast: '150ms',     // Hover states
  base: '200ms',     // Default interactions
  slow: '300ms',     // Complex transitions
  slower: '500ms',   // Page transitions
}
```

### 10.3 Common Animations

```css
/* Fade in */
.fade-in {
  animation: fadeIn 200ms ease-in;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

/* Slide up */
.slide-up {
  animation: slideUp 300ms ease-out;
}

@keyframes slideUp {
  from {
    transform: translateY(10px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

/* Scale */
.scale-in {
  animation: scaleIn 200ms ease-out;
}

@keyframes scaleIn {
  from {
    transform: scale(0.95);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

/* Shimmer loading */
.shimmer {
  animation: shimmer 2s infinite;
  background: linear-gradient(
    90deg,
    #f1f5f9 0%,
    #e2e8f0 50%,
    #f1f5f9 100%
  );
  background-size: 200% 100%;
}

@keyframes shimmer {
  0% { background-position: -200% 0; }
  100% { background-position: 200% 0; }
}
```

### 10.4 Reduced Motion

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

---

## 11. Dark Mode (Optional)

### 11.1 Dark Mode Colors

```typescript
// Add to tailwind config
darkMode: 'class',

theme: {
  extend: {
    colors: {
      dark: {
        background: '#0f172a',     // neutral-900
        surface: '#1e293b',        // neutral-800
        border: '#334155',         // neutral-700
        text: {
          primary: '#f1f5f9',      // neutral-100
          secondary: '#cbd5e1',    // neutral-300
          muted: '#94a3b8',        // neutral-400
        }
      }
    }
  }
}
```

### 11.2 Dark Mode Usage

```typescript
// Toggle dark mode
<div className="dark">
  {/* Dark mode active */}
</div>

// Conditional classes
<div className="
  bg-white dark:bg-dark-background
  text-neutral-900 dark:text-dark-text-primary
  border-neutral-200 dark:border-dark-border
">
  Content
</div>
```

---

## 12. Layout Patterns

### 12.1 Dashboard Layout

```typescript
<div className="min-h-screen bg-neutral-50">
  {/* Sidebar */}
  <aside className="fixed inset-y-0 left-0 w-64 bg-white border-r">
    <Sidebar />
  </aside>

  {/* Main Content */}
  <div className="ml-64">
    {/* Header */}
    <header className="sticky top-0 z-10 bg-white border-b">
      <Header />
    </header>

    {/* Content */}
    <main className="p-6">
      <Outlet />
    </main>
  </div>
</div>
```

### 12.2 Form Layout

```typescript
<form className="space-y-6">
  {/* Section */}
  <div className="space-y-4">
    <h2 className="text-xl font-semibold">Patient Information</h2>

    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div className="space-y-2">
        <Label>First Name</Label>
        <Input />
      </div>
      <div className="space-y-2">
        <Label>Last Name</Label>
        <Input />
      </div>
    </div>
  </div>

  {/* Actions */}
  <div className="flex justify-end gap-3">
    <Button variant="outline">Cancel</Button>
    <Button type="submit">Save</Button>
  </div>
</form>
```

### 12.3 Data Table Layout

```typescript
<div className="space-y-4">
  {/* Filters */}
  <div className="flex items-center justify-between">
    <div className="flex items-center gap-2">
      <Input
        placeholder="Search patients..."
        className="w-80"
      />
      <Button variant="outline">
        <Filter className="h-4 w-4 mr-2" />
        Filters
      </Button>
    </div>
    <Button>
      <Plus className="h-4 w-4 mr-2" />
      New Patient
    </Button>
  </div>

  {/* Table */}
  <Card>
    <Table>
      {/* ... */}
    </Table>
  </Card>

  {/* Pagination */}
  <Pagination />
</div>
```

---

## Summary

This design system provides:

1. **Complete Color Palette**: Primary, secondary, semantic, and status colors
2. **Typography System**: Fonts, sizes, weights for all use cases
3. **Component Library**: 20+ reusable components with shadcn/ui
4. **Spacing System**: Consistent 8px grid
5. **Accessibility**: WCAG 2.1 AA compliant
6. **Responsive Design**: Mobile-first approach
7. **Animation Guidelines**: Smooth, purposeful motion
8. **Dark Mode Support**: Optional theme

### Implementation Checklist:

- [ ] Install Inter font
- [ ] Setup Tailwind CSS with config
- [ ] Install shadcn/ui components
- [ ] Install Lucide icons
- [ ] Configure color tokens
- [ ] Setup responsive breakpoints
- [ ] Implement dark mode (optional)
- [ ] Add animation utilities
- [ ] Test accessibility
- [ ] Document component variants

### Key Design Decisions:

✅ **Medical Blue as Primary**: Trust, calm, professional
✅ **Generous Spacing**: Reduce cognitive load
✅ **High Contrast**: Readability for medical data
✅ **Consistent Components**: shadcn/ui foundation
✅ **Mobile-First**: Progressive enhancement
✅ **Accessible by Default**: WCAG 2.1 AA

---

**Next Steps**:
1. Create Figma/Design mockups
2. Build component Storybook
3. Implement in Next.js
4. User testing with lab staff
5. Iterate based on feedback

---

**Document Status**: ✅ Approved
**Next Review Date**: 2025-02-05
**Owned By**: Design Team
