/**
 * React Hook for Real-time NATS Event Updates via WebSocket
 *
 * Automatically subscribes to project-specific events and invalidates
 * React Query cache for real-time UI updates.
 */

import { type QueryClient, type QueryKey, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState } from "react";

import { logger } from "@/lib/logger";
import { type EventListener, type NATSEventMessage, realtimeClient } from "@/lib/websocket";
import { useAuthStore } from "@/stores/auth-store";

interface RealtimeConfig {
  projectId?: string;
  onEvent?: EventListener;
  enableToasts?: boolean;
}

interface RealtimeState {
  isConnected: boolean;
}

const hasText = (value: string | null | undefined): value is string =>
  typeof value === "string" && value.length > 0;

const invalidateQuery = (queryClient: QueryClient, queryKey: QueryKey): void => {
  const _invalidationPromise: Promise<void> = queryClient
    .invalidateQueries({ queryKey })
    .catch((error: unknown): void => {
      logger.error("Failed to invalidate realtime query:", { error, queryKey });
    });
};

const useRealtime = (config: RealtimeConfig = {}): RealtimeState => {
  const { projectId, onEvent } = config;
  const [isConnected, setIsConnected] = useState(false);
  const token = useAuthStore((state) => state.token);

  useEffect((): (() => void) | void => {
    if (!hasText(token)) {
      logger.warn("No auth token found, skipping WebSocket connection");
      return;
    }

    // Connect to WebSocket with token and optional project ID
    realtimeClient.connect(token, projectId);

    // Monitor connection status
    const checkConnection = setInterval(() => {
      setIsConnected(realtimeClient.isConnected());
    }, 1000);

    // Cleanup
    return (): void => {
      clearInterval(checkConnection);
      realtimeClient.disconnect();
    };
  }, [projectId, token]);

  // Listen for all events and call custom handler
  useEffect((): (() => void) | void => {
    if (!onEvent) {
      return;
    }

    const unsubscribe = realtimeClient.on("*", onEvent);
    return unsubscribe;
  }, [onEvent]);

  return { isConnected };
};

const useRealtimeUpdates = (projectId?: string): void => {
  const queryClient = useQueryClient();
  const token = useAuthStore((state) => state.token);

  useEffect((): (() => void) | void => {
    if (!hasText(projectId) || !hasText(token)) {
      return;
    }

    // Connect to WebSocket
    realtimeClient.connect(token, projectId);

    // Subscribe to item events
    const unsubItem = realtimeClient.on("item.created", () => {
      invalidateQuery(queryClient, ["items", projectId]);
    });

    const unsubItemUpdate = realtimeClient.on("item.updated", () => {
      invalidateQuery(queryClient, ["items", projectId]);
    });

    const unsubItemDelete = realtimeClient.on("item.deleted", () => {
      invalidateQuery(queryClient, ["items", projectId]);
    });

    // Subscribe to link events
    const unsubLink = realtimeClient.on("link.created", () => {
      invalidateQuery(queryClient, ["links", projectId]);
    });

    const unsubLinkDelete = realtimeClient.on("link.deleted", () => {
      invalidateQuery(queryClient, ["links", projectId]);
    });

    // Subscribe to spec events (from Python backend)
    const unsubSpec = realtimeClient.on("spec.created", () => {
      invalidateQuery(queryClient, ["specs", projectId]);
    });

    const unsubSpecUpdate = realtimeClient.on("spec.updated", () => {
      invalidateQuery(queryClient, ["specs", projectId]);
    });

    // Subscribe to AI analysis events
    const unsubAI = realtimeClient.on("ai.analysis.complete", () => {
      invalidateQuery(queryClient, ["analysis", projectId]);
    });

    // Subscribe to execution events
    const unsubExecution = realtimeClient.on("execution.completed", () => {
      invalidateQuery(queryClient, ["executions", projectId]);
    });

    const unsubExecutionFailed = realtimeClient.on("execution.failed", () => {
      invalidateQuery(queryClient, ["executions", projectId]);
    });

    // Subscribe to workflow events
    const unsubWorkflow = realtimeClient.on("workflow.completed", () => {
      invalidateQuery(queryClient, ["workflows", projectId]);
    });

    // Subscribe to project events
    const unsubProject = realtimeClient.on("project.updated", () => {
      invalidateQuery(queryClient, ["projects"]);
      invalidateQuery(queryClient, ["project", projectId]);
    });

    // Cleanup all subscriptions
    return (): void => {
      unsubItem();
      unsubItemUpdate();
      unsubItemDelete();
      unsubLink();
      unsubLinkDelete();
      unsubSpec();
      unsubSpecUpdate();
      unsubAI();
      unsubExecution();
      unsubExecutionFailed();
      unsubWorkflow();
      unsubProject();
      realtimeClient.disconnect();
    };
  }, [projectId, queryClient, token]);
};

const useRealtimeEvent = (
  eventType: string,
  handler: (event: NATSEventMessage["data"]) => void,
): void => {
  useEffect((): (() => void) => {
    const unsubscribe = realtimeClient.on(eventType, handler);
    return unsubscribe;
  }, [eventType, handler]);
};

export type { RealtimeConfig };
export { useRealtime, useRealtimeEvent, useRealtimeUpdates };
