'use client';

import Link from "next/link";
import Image from "next/image";
import { Button } from "@/components/ui/button";

export function Navbar() {
  return (
    <header className="sticky top-0 z-50 border-b glass backdrop-blur-xl">
      <div className="container mx-auto px-4 md:px-8 py-4">
        <div className="flex items-center justify-between">
          <Link href="/">
            <Image
              src="/logo.svg"
              alt="Floe-LIS Logo"
              width={200}
              height={67}
              className="h-12 w-auto"
              priority
            />
          </Link>
          <div className="flex items-center space-x-4">
            <Link href="/login">
              <Button variant="ghost" className="hidden sm:inline-flex">Login</Button>
            </Link>
            <Link href="/register">
              <Button className="hover-glow">Get Started</Button>
            </Link>
          </div>
        </div>
      </div>
    </header>
  );
}
