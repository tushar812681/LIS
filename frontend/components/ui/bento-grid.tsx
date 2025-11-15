'use client';

import { motion } from 'framer-motion';
import { cn } from '@/lib/utils';
import { ReactNode } from 'react';

interface BentoGridProps {
  children: ReactNode;
  className?: string;
}

interface BentoCardProps {
  children: ReactNode;
  className?: string;
  size?: 'small' | 'medium' | 'large';
  index?: number;
}

/**
 * BentoGrid container component
 * Creates a modern bento-style grid layout
 */
export function BentoGrid({ children, className }: BentoGridProps) {
  return (
    <div
      className={cn(
        'grid auto-rows-[minmax(200px,auto)] grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4',
        className
      )}
    >
      {children}
    </div>
  );
}

/**
 * BentoCard component
 * Individual card in the bento grid with glassmorphism effect
 */
export function BentoCard({
  children,
  className,
  size = 'medium',
  index = 0,
}: BentoCardProps) {
  const sizeClasses = {
    small: '',
    medium: 'md:col-span-1',
    large: 'md:col-span-2 lg:col-span-2',
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: '-50px' }}
      transition={{ duration: 0.5, delay: index * 0.1 }}
      whileHover={{ y: -4, transition: { duration: 0.2 } }}
      className={cn(
        'glass rounded-2xl p-6 shadow-xl hover-lift relative overflow-hidden group',
        sizeClasses[size],
        className
      )}
    >
      {/* Gradient overlay on hover */}
      <div className="absolute inset-0 bg-gradient-to-br from-primary/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300" />

      {/* Content */}
      <div className="relative z-10">{children}</div>
    </motion.div>
  );
}
