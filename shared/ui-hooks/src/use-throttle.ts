import { useRef, useCallback } from "react";

/**
 * Hook for throttling a function call.
 *
 * @example
 * ```tsx
 * const throttledScroll = useThrottle((scrollPos) => {
 *   updateScrollIndicator(scrollPos);
 * }, 100);
 *
 * window.addEventListener("scroll", () => {
 *   throttledScroll(window.scrollY);
 * });
 * ```
 */
export function useThrottle<T extends (...args: unknown[]) => unknown>(
  callback: T,
  limit: number
): (...args: Parameters<T>) => void {
  const lastRunRef = useRef<number>(0);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  return useCallback(
    (...args: Parameters<T>) => {
      const now = Date.now();

      if (now - lastRunRef.current >= limit) {
        lastRunRef.current = now;
        callback(...args);
      } else {
        if (timeoutRef.current) {
          clearTimeout(timeoutRef.current);
        }

        timeoutRef.current = setTimeout(() => {
          lastRunRef.current = Date.now();
          callback(...args);
        }, limit - (now - lastRunRef.current));
      }
    },
    [callback, limit]
  );
}
