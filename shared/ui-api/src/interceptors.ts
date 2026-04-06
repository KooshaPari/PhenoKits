/**
 * API Interceptors
 *
 * Request/response interceptors for logging, authentication, retries, and rate limiting.
 */

import type { ApiError } from "./errors.js";

export interface InterceptorContext {
  url: string;
  method: string;
  headers: Record<string, string>;
  body?: unknown;
  signal?: AbortSignal;
  response?: Response;
  data?: unknown;
  error?: ApiError;
  meta?: Record<string, unknown>;
}

export type Interceptor = (context: InterceptorContext) => InterceptorContext | Promise<InterceptorContext>;

/**
 * Logging interceptor that logs requests and responses
 */
export function loggingInterceptor(options?: {
  logRequest?: boolean;
  logResponse?: boolean;
  logError?: boolean;
}): Interceptor {
  const { logRequest = true, logResponse = true, logError = true } = options ?? {};

  return async (context) => {
    const startTime = Date.now();

    if (logRequest) {
      // eslint-disable-next-line no-console
      console.log(`[API] ${context.method} ${context.url}`);
    }

    // If there's a response, this is the response phase
    if (context.response) {
      const duration = Date.now() - startTime;

      if (logResponse) {
        // eslint-disable-next-line no-console
        console.log(`[API] ${context.method} ${context.url} - ${context.response.status} (${duration}ms)`);
      }
    }

    // If there's an error, this is the error phase
    if (context.error && logError) {
      // eslint-disable-next-line no-console
      console.error(`[API] ${context.method} ${context.url} - Error:`, context.error.message);
    }

    return context;
  };
}

/**
 * Authentication interceptor that adds auth headers
 */
export function authInterceptor(options: {
  getToken: () => string | null | Promise<string | null>;
  headerName?: string;
  tokenPrefix?: string;
}): Interceptor {
  const { getToken, headerName = "Authorization", tokenPrefix = "Bearer" } = options;

  return async (context) => {
    const token = await getToken();

    if (token) {
      return {
        ...context,
        headers: {
          ...context.headers,
          [headerName]: `${tokenPrefix} ${token}`,
        },
      };
    }

    return context;
  };
}

/**
 * Retry interceptor with exponential backoff
 */
export function retryInterceptor(options?: {
  maxRetries?: number;
  retryDelay?: number;
  retryableStatuses?: number[];
}): Interceptor {
  const {
    maxRetries = 3,
    retryDelay = 1000,
    retryableStatuses = [408, 429, 500, 502, 503, 504],
  } = options ?? {};

  return async (context) => {
    // This interceptor works on errors
    if (!context.error) {
      return context;
    }

    const currentRetry = (context.meta?.retryCount as number) ?? 0;

    // Check if we should retry
    if (
      currentRetry >= maxRetries ||
      !retryableStatuses.includes(context.error.status)
    ) {
      return context;
    }

    // Exponential backoff
    const delay = retryDelay * Math.pow(2, currentRetry);
    await new Promise((resolve) => setTimeout(resolve, delay));

    // Return context with updated retry count
    return {
      ...context,
      meta: {
        ...context.meta,
        retryCount: currentRetry + 1,
      },
    };
  };
}

/**
 * Rate limit interceptor that tracks rate limit headers
 */
export function rateLimitInterceptor(options?: {
  onRateLimit?: (remaining: number, resetTime: Date) => void;
}): Interceptor {
  const { onRateLimit } = options ?? {};

  return async (context) => {
    if (!context.response) {
      return context;
    }

    const remaining = context.response.headers.get("X-RateLimit-Remaining");
    const reset = context.response.headers.get("X-RateLimit-Reset");

    if (remaining && reset) {
      const remainingCount = parseInt(remaining, 10);
      const resetTime = new Date(parseInt(reset, 10) * 1000);

      onRateLimit?.(remainingCount, resetTime);

      // Store in meta for other interceptors
      return {
        ...context,
        meta: {
          ...context.meta,
          rateLimitRemaining: remainingCount,
          rateLimitReset: resetTime,
        },
      };
    }

    return context;
  };
}

/**
 * Compose multiple interceptors into a single interceptor
 */
export function composeInterceptors(...interceptors: Interceptor[]): Interceptor {
  return async (context) => {
    let result = context;
    for (const interceptor of interceptors) {
      result = await interceptor(result);
    }
    return result;
  };
}
