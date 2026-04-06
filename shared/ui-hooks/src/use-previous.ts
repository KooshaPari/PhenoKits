import { useRef, useEffect } from "react";

/**
 * Hook for tracking the previous value of a state or prop.
 *
 * @example
 * ```tsx
 * const previousCount = usePrevious(count);
 *
 * useEffect(() => {
 *   if (count > (previousCount ?? 0)) {
 *     console.log("Count increased!");
 *   }
 * }, [count, previousCount]);
 * ```
 */
export function usePrevious<T>(value: T): T | undefined {
  const ref = useRef<T | undefined>(undefined);

  useEffect(() => {
    ref.current = value;
  }, [value]);

  return ref.current;
}
