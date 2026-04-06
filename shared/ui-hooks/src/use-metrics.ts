import { useState, useEffect, useCallback } from "react";

export interface MetricData {
  timestamp: number;
  value: number;
  label?: string;
}

export interface UseMetricsOptions {
  metricName: string;
  timeRange?: "1h" | "6h" | "24h" | "7d" | "30d";
  pollInterval?: number;
  aggregation?: "avg" | "sum" | "min" | "max" | "count";
  onData?: (data: MetricData[]) => void;
  onError?: (error: Error) => void;
}

interface UseMetricsResult {
  data: MetricData[];
  isLoading: boolean;
  error: Error | null;
  refresh: () => Promise<void>;
  currentValue: number | null;
  averageValue: number | null;
}

/**
 * Hook for fetching and polling metrics data.
 *
 * @example
 * ```tsx
 * const { data, currentValue, averageValue } = useMetrics({
 *   metricName: "cpu_usage",
 *   timeRange: "24h",
 *   pollInterval: 30000
 * });
 * ```
 */
export function useMetrics(options: UseMetricsOptions): UseMetricsResult {
  const {
    metricName,
    timeRange = "24h",
    pollInterval = 30000,
    aggregation = "avg",
    onData,
    onError,
  } = options;

  const [data, setData] = useState<MetricData[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchMetrics = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const params = new URLSearchParams({
        metric: metricName,
        range: timeRange,
        aggregation,
      });

      const response = await fetch(`/api/metrics?${params.toString()}`);

      if (!response.ok) {
        throw new Error(`Failed to fetch metrics: ${response.status}`);
      }

      const result = (await response.json()) as MetricData[];
      setData(result);
      onData?.(result);
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err));
      setError(error);
      onError?.(error);
    } finally {
      setIsLoading(false);
    }
  }, [metricName, timeRange, aggregation, onData, onError]);

  useEffect(() => {
    void fetchMetrics();

    const intervalId = setInterval(() => {
      void fetchMetrics();
    }, pollInterval);

    return () => clearInterval(intervalId);
  }, [fetchMetrics, pollInterval]);

  const currentValue = data.length > 0 ? data[data.length - 1]?.value ?? null : null;

  const averageValue =
    data.length > 0
      ? data.reduce((sum, point) => sum + point.value, 0) / data.length
      : null;

  return {
    data,
    isLoading,
    error,
    refresh: fetchMetrics,
    currentValue,
    averageValue,
  };
}
