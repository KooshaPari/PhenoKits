import { useEffect, useRef, useState, useCallback } from "react";

export interface SSEEvent {
  id?: string;
  event?: string;
  data: string;
}

export interface UseSSEOptions {
  url: string;
  headers?: Record<string, string>;
  onMessage?: (data: unknown, event: SSEEvent) => void;
  onOpen?: () => void;
  onError?: (error: Event) => void;
  onClose?: () => void;
  reconnect?: boolean;
  reconnectInterval?: number;
  maxReconnects?: number;
  enabled?: boolean;
}

interface UseSSEReturn {
  isConnected: boolean;
  isConnecting: boolean;
  error: Error | null;
  close: () => void;
  reconnect: () => void;
}

/**
 * Hook for Server-Sent Events (SSE) with automatic reconnection.
 *
 * @example
 * ```tsx
 * useSSE({
 *   url: "/api/stream",
 *   onMessage: (data) => console.log(data),
 *   reconnect: true,
 *   reconnectInterval: 5000
 * });
 * ```
 */
export function useSSE(options: UseSSEOptions): UseSSEReturn {
  const {
    url,
    headers = {},
    onMessage,
    onOpen,
    onError,
    onClose,
    reconnect = true,
    reconnectInterval = 5000,
    maxReconnects = 10,
    enabled = true,
  } = options;

  const [isConnected, setIsConnected] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const eventSourceRef = useRef<EventSource | null>(null);
  const reconnectCountRef = useRef(0);
  const reconnectTimerRef = useRef<NodeJS.Timeout | null>(null);
  const isManualCloseRef = useRef(false);

  const close = useCallback(() => {
    isManualCloseRef.current = true;

    if (reconnectTimerRef.current) {
      clearTimeout(reconnectTimerRef.current);
      reconnectTimerRef.current = null;
    }

    if (eventSourceRef.current) {
      eventSourceRef.current.close();
      eventSourceRef.current = null;
    }

    setIsConnected(false);
    setIsConnecting(false);
  }, []);

  const connect = useCallback(() => {
    if (!enabled || eventSourceRef.current) return;

    setIsConnecting(true);
    setError(null);

    try {
      const eventSource = new EventSource(url, { withCredentials: true });
      eventSourceRef.current = eventSource;

      eventSource.addEventListener("open", () => {
        setIsConnected(true);
        setIsConnecting(false);
        reconnectCountRef.current = 0;
        onOpen?.();
      });

      eventSource.addEventListener("message", (event) => {
        try {
          const data = JSON.parse(event.data) as unknown;
          onMessage?.(data, {
            id: event.lastEventId,
            data: event.data,
          });
        } catch {
          // If JSON parsing fails, send raw data
          onMessage?.(event.data, {
            id: event.lastEventId,
            data: event.data,
          });
        }
      });

      eventSource.addEventListener("error", (event) => {
        setError(new Error("SSE connection error"));
        onError?.(event);

        if (!isManualCloseRef.current && reconnect && reconnectCountRef.current < maxReconnects) {
          reconnectCountRef.current++;
          reconnectTimerRef.current = setTimeout(() => {
            close();
            isManualCloseRef.current = false;
            connect();
          }, reconnectInterval);
        } else {
          close();
        }
      });

      eventSource.addEventListener("close", () => {
        setIsConnected(false);
        onClose?.();
      });
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsConnecting(false);
    }
  }, [url, enabled, reconnect, reconnectInterval, maxReconnects, onOpen, onMessage, onError, onClose, close]);

  useEffect(() => {
    isManualCloseRef.current = false;
    connect();

    return () => {
      close();
    };
  }, [connect, close]);

  return {
    isConnected,
    isConnecting,
    error,
    close,
    reconnect: () => {
      close();
      isManualCloseRef.current = false;
      reconnectCountRef.current = 0;
      connect();
    },
  };
}
