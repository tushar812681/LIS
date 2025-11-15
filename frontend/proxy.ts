import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

// Define public routes that don't require authentication
const publicRoutes = [
  '/',
  '/login',
  '/register',
  '/reset-password',
  '/terms',
  '/privacy',
  '/about',
  '/contact',
  '/blog',
  '/careers',
  '/features',
  '/technology',
  '/pricing',
  '/demo',
  '/security',
];

// Define routes that should redirect to dashboard if already authenticated
const authRoutes = ['/login', '/register', '/reset-password'];

// Define role-based route access
const roleBasedRoutes: Record<string, string[]> = {
  '/dashboard/admin': ['ADMIN', 'SUPER_ADMIN'],
  '/dashboard/lab': ['LAB_TECHNICIAN', 'LAB_MANAGER', 'ADMIN', 'SUPER_ADMIN'],
  '/dashboard/doctor': ['DOCTOR', 'ADMIN', 'SUPER_ADMIN'],
  '/dashboard/patient': ['PATIENT'],
  '/settings/organization': ['ADMIN', 'SUPER_ADMIN'],
  '/settings/users': ['ADMIN', 'SUPER_ADMIN'],
  '/settings/equipment': ['LAB_MANAGER', 'ADMIN', 'SUPER_ADMIN'],
};

/**
 * Proxy function to protect routes and enforce authentication
 */
export default function proxy(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // Get token from cookies
  const token = request.cookies.get('auth_token')?.value;

  // Check if route is public
  const isPublicRoute = publicRoutes.some((route) => {
    if (route === '/') return pathname === '/';
    return pathname.startsWith(route);
  });

  // Check if route is an auth route
  const isAuthRoute = authRoutes.some((route) => pathname.startsWith(route));

  // If accessing auth route with valid token, redirect to dashboard
  if (isAuthRoute && token) {
    return NextResponse.redirect(new URL('/dashboard', request.url));
  }

  // Allow access to public routes
  if (isPublicRoute) {
    return NextResponse.next();
  }

  // Protect all other routes
  if (!token) {
    // Store the original URL to redirect back after login
    const loginUrl = new URL('/login', request.url);
    loginUrl.searchParams.set('callbackUrl', pathname);
    return NextResponse.redirect(loginUrl);
  }

  // Decode token to check role (in production, verify JWT signature)
  try {
    const tokenPayload = JSON.parse(
      Buffer.from(token.split('.')[1], 'base64').toString()
    );

    const userRole = tokenPayload.role;

    // Check role-based access
    for (const [route, allowedRoles] of Object.entries(roleBasedRoutes)) {
      if (pathname.startsWith(route)) {
        if (!allowedRoles.includes(userRole)) {
          // Redirect to appropriate dashboard based on role
          const dashboardUrl = getRoleDashboard(userRole);
          return NextResponse.redirect(new URL(dashboardUrl, request.url));
        }
      }
    }

    // Check token expiration
    const expirationTime = tokenPayload.exp * 1000; // Convert to milliseconds
    if (Date.now() >= expirationTime) {
      // Token expired, redirect to login
      const response = NextResponse.redirect(new URL('/login', request.url));
      response.cookies.delete('auth_token');
      return response;
    }
  } catch (error) {
    // Invalid token, redirect to login
    console.error('Token validation error:', error);
    const response = NextResponse.redirect(new URL('/login', request.url));
    response.cookies.delete('auth_token');
    return response;
  }

  return NextResponse.next();
}

/**
 * Get the appropriate dashboard URL based on user role
 */
function getRoleDashboard(role: string): string {
  switch (role) {
    case 'ADMIN':
    case 'SUPER_ADMIN':
      return '/dashboard/admin';
    case 'LAB_TECHNICIAN':
    case 'LAB_MANAGER':
      return '/dashboard/lab';
    case 'DOCTOR':
      return '/dashboard/doctor';
    case 'PATIENT':
      return '/dashboard/patient';
    default:
      return '/dashboard';
  }
}

// Configure which routes to run middleware on
export const config = {
  matcher: [
    /*
     * Match all request paths except:
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico (favicon file)
     * - public folder files
     */
    '/((?!_next/static|_next/image|favicon.ico|.*\\.(?:svg|png|jpg|jpeg|gif|webp)$).*)',
  ],
};
