'use client';

import { motion } from 'framer-motion';
import { cn } from '@/lib/utils';
import { HTMLAttributes } from 'react';

interface GlassCardProps extends HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
  variant?: 'default' | 'strong' | 'dark';
  hover?: boolean;
  animate?: boolean;
}

/**
 * GlassCard component with glassmorphism effect
 * Provides a frosted glass appearance with blur and transparency
 */
export function GlassCard({
  children,
  className,
  variant = 'default',
  hover = true,
  animate = true,
  ...props
}: GlassCardProps) {
  const glassVariants = {
    default: 'glass',
    strong: 'glass-strong',
    dark: 'glass-dark',
  };

  const Component = animate ? motion.div : 'div';

  const motionProps = animate
    ? {
        initial: { opacity: 0, y: 20 },
        whileInView: { opacity: 1, y: 0 },
        viewport: { once: true, margin: '-50px' },
        transition: { duration: 0.5, ease: 'easeOut' },
        ...(hover && {
          whileHover: { y: -4, transition: { duration: 0.2 } },
        }),
      }
    : {};

  return (
    <Component
      className={cn(
        glassVariants[variant],
        'rounded-2xl shadow-xl',
        hover && 'hover-lift cursor-pointer',
        className
      )}
      {...(animate && motionProps)}
      {...props}
    >
      {children}
    </Component>
  );
}
