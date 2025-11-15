import { Metadata } from 'next';
import Image from 'next/image';

export const metadata: Metadata = {
  title: 'Authentication | Floe-LIS',
  description: 'Sign in or create an account to access the Laboratory Information System',
};

export default function AuthLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="min-h-screen bg-gradient-to-br from-cyan-50 via-white to-teal-50 dark:from-gray-900 dark:via-gray-900 dark:to-gray-800">
      {/* Logo in top left */}
      <div className="absolute left-8 top-8">
        <Image
          src="/logo.svg"
          alt="Floe-LIS Logo"
          width={160}
          height={53}
          className="h-10 w-auto"
          priority
        />
      </div>

      {/* Main content */}
      {children}

      {/* Footer */}
      <div className="fixed bottom-0 left-0 right-0 border-t border-gray-200 bg-white/80 backdrop-blur-sm dark:border-gray-800 dark:bg-gray-900/80">
        <div className="container mx-auto px-4 py-4">
          <p className="text-center text-sm text-gray-600 dark:text-gray-400">
            © 2025 Floe-LIS. All rights reserved.
          </p>
        </div>
      </div>
    </div>
  );
}
