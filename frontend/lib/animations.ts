/**
 * Optimized animation variants for Framer Motion
 * Fast, lightweight animations for better performance
 */

import { Variants } from 'framer-motion';

// Fade in animation - faster and lighter
export const fadeIn: Variants = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: { duration: 0.3, ease: 'easeOut' },
  },
};

// Slide up animation - reduced distance and duration
export const slideUp: Variants = {
  hidden: { opacity: 0, y: 15 },
  visible: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.4, ease: 'easeOut' },
  },
};

// Slide from left - optimized
export const slideLeft: Variants = {
  hidden: { opacity: 0, x: -20 },
  visible: {
    opacity: 1,
    x: 0,
    transition: { duration: 0.4, ease: 'easeOut' },
  },
};

// Slide from right - optimized
export const slideRight: Variants = {
  hidden: { opacity: 0, x: 20 },
  visible: {
    opacity: 1,
    x: 0,
    transition: { duration: 0.4, ease: 'easeOut' },
  },
};

// Scale in animation - minimal scale change
export const scaleIn: Variants = {
  hidden: { opacity: 0, scale: 0.98 },
  visible: {
    opacity: 1,
    scale: 1,
    transition: { duration: 0.3, ease: 'easeOut' },
  },
};

// Stagger container - faster stagger with no delay
export const staggerContainer: Variants = {
  hidden: { opacity: 1 },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: 0.05,
    },
  },
};

// Stagger item - lightweight
export const staggerItem: Variants = {
  hidden: { opacity: 0, y: 10 },
  visible: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.3 },
  },
};

// Reveal from bottom - simplified
export const revealFromBottom: Variants = {
  hidden: { opacity: 0, y: 20 },
  visible: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.4, ease: 'easeOut' },
  },
};

// Card hover animation - subtle
export const cardHover = {
  rest: { scale: 1, y: 0 },
  hover: {
    scale: 1.01,
    y: -2,
    transition: { duration: 0.2, ease: 'easeInOut' },
  },
};

// Button hover - simplified
export const buttonHover = {
  rest: { scale: 1 },
  hover: {
    scale: 1.02,
    transition: { duration: 0.2, ease: 'easeInOut' },
  },
  tap: {
    scale: 0.98,
  },
};

// Fade in with delay - faster
export const fadeInDelayed = (delay: number = 0): Variants => ({
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: { duration: 0.3, delay, ease: 'easeOut' },
  },
});

// Slide up with delay - optimized
export const slideUpDelayed = (delay: number = 0): Variants => ({
  hidden: { opacity: 0, y: 15 },
  visible: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.4, delay, ease: 'easeOut' },
  },
});

// Viewport animation config - optimized triggers
export const viewportConfig = {
  once: true,
  margin: '-50px',
  amount: 0.2,
};

// Viewport animation config for longer elements
export const viewportConfigLarge = {
  once: true,
  margin: '-30px',
  amount: 0.1,
};

// Spring animation config - snappier
export const springConfig = {
  type: 'spring',
  stiffness: 150,
  damping: 20,
};

// Number counter animation - fast
export const numberCounter = {
  initial: { opacity: 0, y: 10 },
  animate: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.3, ease: 'easeOut' },
  },
};

// No expensive infinite animations - removed floating and pulse
