import { useState, useEffect, useCallback } from "react";

export interface DeploymentStatus {
  id: string;
  status: "pending" | "building" | "deploying" | "running" | "failed" | "stopped";
  progress: number;
  message?: string;
  timestamp: string;
}

export interface UseDeploymentOptions {
  deploymentId: string | null;
  pollInterval?: number;
  onStatusChange?: (status: DeploymentStatus) => void;
  onComplete?: (status: DeploymentStatus) => void;
  onError?: (error: Error) => void;
}

interface UseDeploymentResult {
  status: DeploymentStatus | null;
  isLoading: boolean;
  error: Error | null;
  refresh: () => Promise<void>;
}

/**
 * Hook for tracking deployment status with polling.
 *
 * @example
 * ```tsx
 * const { status, isLoading } = useDeployment({
 *   deploymentId: "deploy-123",
 *   pollInterval: 5000,
 *   onComplete: (status) => console.log("Deployment complete:", status)
 * });
 * ```
 */
export function useDeployment(options: UseDeploymentOptions): UseDeploymentResult {
  const { deploymentId, pollInterval = 5000, onStatusChange, onComplete, onError } = options;

  const [status, setStatus] = useState<DeploymentStatus | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchStatus = useCallback(async () => {
    if (!deploymentId) return;

    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch(`/api/deployments/${deploymentId}/status`);

      if (!response.ok) {
        throw new Error(`Failed to fetch deployment status: ${response.status}`);
      }

      const data = (await response.json()) as DeploymentStatus;
      setStatus(data);
      onStatusChange?.(data);

      if (["running", "failed", "stopped"].includes(data.status)) {
        onComplete?.(data);
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err));
      setError(error);
      onError?.(error);
    } finally {
      setIsLoading(false);
    }
  }, [deploymentId, onStatusChange, onComplete, onError]);

  useEffect(() => {
    if (!deploymentId) {
      setStatus(null);
      return;
    }

    void fetchStatus();

    const intervalId = setInterval(() => {
      void fetchStatus();
    }, pollInterval);

    return () => clearInterval(intervalId);
  }, [deploymentId, pollInterval, fetchStatus]);

  return {
    status,
    isLoading,
    error,
    refresh: fetchStatus,
  };
}
