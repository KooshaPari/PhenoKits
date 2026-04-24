import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

/**
 * Merges Tailwind CSS classes with clsx and tailwind-merge.
 * Use this for all className concatenation to avoid conflicts.
 *
 * @example
 * cn('px-2', 'py-1', condition && 'bg-blue-500')
 * cn('btn', { 'btn-primary': isPrimary, 'btn-large': isLarge })
 *
 * @param inputs - Class values to merge
 * @returns Merged class string
 */
export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs));
}
