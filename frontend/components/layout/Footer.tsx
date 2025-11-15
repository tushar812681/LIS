'use client';

import Link from "next/link";
import Image from "next/image";
import { Shield } from "lucide-react";

export function Footer() {
  return (
    <footer className="border-t bg-white">
      <div className="container-wide py-12">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8 mb-8">
          <div className="space-y-4">
            <Image
              src="/logo.svg"
              alt="Floe-LIS Logo"
              width={160}
              height={53}
              className="h-8 w-auto"
            />
            <p className="text-sm text-slate-600">
              Modern cloud-native Laboratory Information System for Indian healthcare
            </p>
          </div>

          <div>
            <h3 className="font-semibold mb-4">Product</h3>
            <ul className="space-y-2 text-sm text-slate-600">
              <li><Link href="/features" className="hover:text-primary transition-colors">Features</Link></li>
              <li><Link href="/technology" className="hover:text-primary transition-colors">Technology</Link></li>
              <li><Link href="/pricing" className="hover:text-primary transition-colors">Pricing</Link></li>
              <li><Link href="/demo" className="hover:text-primary transition-colors">Demo</Link></li>
            </ul>
          </div>

          <div>
            <h3 className="font-semibold mb-4">Company</h3>
            <ul className="space-y-2 text-sm text-slate-600">
              <li><Link href="/about" className="hover:text-primary transition-colors">About</Link></li>
              <li><Link href="/blog" className="hover:text-primary transition-colors">Blog</Link></li>
              <li><Link href="/careers" className="hover:text-primary transition-colors">Careers</Link></li>
              <li><Link href="/contact" className="hover:text-primary transition-colors">Contact</Link></li>
            </ul>
          </div>

          <div>
            <h3 className="font-semibold mb-4">Legal</h3>
            <ul className="space-y-2 text-sm text-slate-600">
              <li><Link href="/privacy" className="hover:text-primary transition-colors">Privacy Policy</Link></li>
              <li><Link href="/terms" className="hover:text-primary transition-colors">Terms of Service</Link></li>
              <li><Link href="/security" className="hover:text-primary transition-colors">Security</Link></li>
            </ul>
          </div>
        </div>

        <div className="border-t pt-8 flex flex-col md:flex-row justify-between items-center gap-4">
          <p className="text-sm text-slate-600">
            © 2025 Floe-LIS. All rights reserved.
          </p>
          <div className="flex items-center gap-4 text-sm text-slate-600">
            <span className="flex items-center gap-2">
              <Shield className="w-4 h-4" />
              NABL Compliant
            </span>
            <span className="flex items-center gap-2">
              <Shield className="w-4 h-4" />
              ISO 15189:2022
            </span>
            <span className="flex items-center gap-2">
              <Shield className="w-4 h-4" />
              ABDM Integrated
            </span>
          </div>
        </div>
      </div>
    </footer>
  );
}
