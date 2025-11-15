import type { Metadata } from "next";
import "./globals.css";
import { Providers } from "@/components/providers";

export const metadata: Metadata = {
  title: "Floe-LIS - Laboratory Information System",
  description: "Modern cloud-native Laboratory Information System for Indian healthcare market",
  keywords: ["LIS", "LIMS", "Laboratory", "Healthcare", "India", "NABL", "ABDM", "Floe-LIS"],
  icons: {
    icon: [
      { url: '/favicon.svg?v=2', type: 'image/svg+xml' },
    ],
    shortcut: '/favicon.svg?v=2',
    apple: '/favicon.svg?v=2',
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased">
        <Providers>
          {children}
        </Providers>
      </body>
    </html>
  );
}
