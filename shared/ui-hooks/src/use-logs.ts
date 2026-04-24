import { useState, useEffect, useCallback, useRef } from "react";

export interface LogEntry {
  id: string;
  timestamp: string;
  level: "debug" | "info" | "warn" | "error";
  message: string;
  source?: string;
  metadata?: Record<string, unknown>;
}

export interface UseLogsOptions {
  source?: string;
  levels?: ("debug" | "info" | "warn" | "error")[];
  maxLogs?: number;
  pollInterval?: number;
  follow?: boolean;
  onLog?: (log: LogEntry) => void;
  onError?: (error: Error) => void;
}

interface UseLogsResult {
  logs: LogEntry[];
  isLoading: boolean;
  error: Error | null;
  refresh: () => Promise<void>;
  clearLogs: () => void;
  hasMore: boolean;
  loadMore: () => Promise<void>;
}

/**
 * Hook for fetching and streaming logs with pagination.
 *
 * @example
 * ```tsx
 * const { logs, clearLogs } = useLogs({
 *   source: "api",
 *   levels: ["info", "error"],
 *   maxLogs: 1000,
 *   follow: true
 * });
 * ```
 */
export function useLogs(options: UseLogsOptions = {}): UseLogsResult {
  const {
    source,
    levels = ["info", "warn", "error"],
    maxLogs = 1000,
    pollInterval = 5000,
    follow = true,
    onLog,
    onError,
  } = options;

  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const [hasMore, setHasMore] = useState(true);

  const lastIdRef = useRef<string | null>(null);
  const abortControllerRef = useRef<AbortController | null>(null);

  const fetchLogs = useCallback(
    async (loadMore = false) => {
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }

      const abortController = new AbortController();
      abortControllerRef.current = abortController;

      setIsLoading(true);
      setError(null);

      try {
        const params = new URLSearchParams();
        if (source) params.set("source", source);
        if (levels.length > 0) params.set("levels", levels.join(","));
        if (lastIdRef.current && loadMore) params.set("before", lastIdRef.current);
        params.set("limit", String(maxLogs));

        const response = await fetch(`/api/logs?${params.toString()}`, {
          signal: abortController.signal,
        });

        if (!response.ok) {
          throw new Error(`Failed to fetch logs: ${response.status}`);
        }

        const newLogs = (await response.json()) as LogEntry[];

        if (newLogs.length < maxLogs) {
          setHasMore(false);
        }

        if (newLogs.length > 0) {
          lastIdRef.current = newLogs[newLogs.length - 1]?.id ?? null;
        }

        setLogs((prev) => {
          const combined = loadMore ? [...prev, ...newLogs] : newLogs;
          // Keep only the most recent maxLogs
          return combined.slice(-maxLogs);
        });

        // Call onLog for each new log
        newLogs.forEach((log) => onLog?.(log));
      } catch (err) {
        if (err instanceof Error && err.name === "AbortError") {
          return;
        }
        const error = err instanceof Error ? err : new Error(String(err));
        setError(error);
        onError?.(error);
      } finally {
        setIsLoading(false);
      }
    },
    [source, levels, maxLogs, onLog, onError]
  );

  const clearLogs = useCallback(() => {
    setLogs([]);
    lastIdRef.current = null;
    setHasMore(true);
  }, []);

  const loadMore = useCallback(async () => {
    if (!hasMore || isLoading) return;
    await fetchLogs(true);
  }, [fetchLogs, hasMore, isLoading]);

  useEffect(() => {
    void fetchLogs();

    let intervalId: NodeJS.Timeout | null = null;
    if (follow) {
      intervalId = setInterval(() => {
        void fetchLogs();
      }, pollInterval);
    }

    return () => {
      if (intervalId) clearInterval(intervalId);
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }
    };
  }, [fetchLogs, follow, pollInterval]);

  return {
    logs,
    isLoading,
    error,
    refresh: () => fetchLogs(),
    clearLogs,
    hasMore,
    loadMore,
  };
}
