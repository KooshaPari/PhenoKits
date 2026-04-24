import { useState, useEffect, useCallback } from "react";

export interface UseLocalStorageOptions<T> {
  key: string;
  defaultValue: T;
  serializer?: (value: T) => string;
  deserializer?: (value: string) => T;
}

/**
 * Hook for persisting state to localStorage with type safety.
 *
 * @example
 * ```tsx
 * const [theme, setTheme] = useLocalStorage({
 *   key: "theme",
 *   defaultValue: "light"
 * });
 * ```
 */
export function useLocalStorage<T>(options: UseLocalStorageOptions<T>): [T, (value: T | ((prev: T) => T)) => void] {
  const { key, defaultValue, serializer = JSON.stringify, deserializer = JSON.parse } = options;

  const [value, setValue] = useState<T>(() => {
    if (typeof window === "undefined") {
      return defaultValue;
    }

    try {
      const item = window.localStorage.getItem(key);
      if (item) {
        return deserializer(item);
      }
      return defaultValue;
    } catch {
      return defaultValue;
    }
  });

  const setStoredValue = useCallback(
    (newValue: T | ((prev: T) => T)) => {
      setValue((prev) => {
        const valueToStore = newValue instanceof Function ? newValue(prev) : newValue;

        try {
          window.localStorage.setItem(key, serializer(valueToStore));
        } catch {
          // Silent fail if localStorage is not available
        }

        return valueToStore;
      });
    },
    [key, serializer]
  );

  // Listen for storage changes from other tabs
  useEffect(() => {
    const handleStorageChange = (event: StorageEvent) => {
      if (event.key === key && event.newValue !== null) {
        try {
          setValue(deserializer(event.newValue));
        } catch {
          // Silent fail on parse error
        }
      }
    };

    window.addEventListener("storage", handleStorageChange);
    return () => window.removeEventListener("storage", handleStorageChange);
  }, [key, deserializer]);

  return [value, setStoredValue];
}
