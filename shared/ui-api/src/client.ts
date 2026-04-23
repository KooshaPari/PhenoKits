import {
  ApiError,
  NetworkError,
  TimeoutError,
  createErrorFromResponse,
} from "./errors.js";
import type { Interceptor, InterceptorContext } from "./interceptors.js";

export interface ApiClientConfig {
  baseURL: string;
  timeout?: number;
  headers?: Record<string, string>;
  credentials?: RequestCredentials;
}

export interface RequestConfig {
  method?: "GET" | "POST" | "PUT" | "DELETE" | "PATCH";
  headers?: Record<string, string>;
  body?: unknown;
  timeout?: number;
  signal?: AbortSignal;
}

export interface ApiResponse<T = unknown> {
  data: T;
  status: number;
  headers: Headers;
}

/**
 * Centralized API client with interceptors, error handling, and request/response transforms.
 *
 * @example
 * ```tsx
 * const client = new ApiClient({
 *   baseURL: "https://api.example.com",
 *   timeout: 30000
 * });
 *
 * const { data } = await client.get("/projects");
 * ```
 */
export class ApiClient {
  private config: Required<ApiClientConfig>;
  private interceptors: {
    request: Interceptor[];
    response: Interceptor[];
    error: Interceptor[];
  };

  constructor(config: ApiClientConfig) {
    this.config = {
      timeout: 30000,
      headers: {
        "Content-Type": "application/json",
      },
      credentials: "same-origin",
      ...config,
    };

    this.interceptors = {
      request: [],
      response: [],
      error: [],
    };
  }

  /**
   * Add a request interceptor
   */
  addRequestInterceptor(interceptor: Interceptor): () => void {
    this.interceptors.request.push(interceptor);
    return () => {
      const index = this.interceptors.request.indexOf(interceptor);
      if (index > -1) {
        this.interceptors.request.splice(index, 1);
      }
    };
  }

  /**
   * Add a response interceptor
   */
  addResponseInterceptor(interceptor: Interceptor): () => void {
    this.interceptors.response.push(interceptor);
    return () => {
      const index = this.interceptors.response.indexOf(interceptor);
      if (index > -1) {
        this.interceptors.response.splice(index, 1);
      }
    };
  }

  /**
   * Add an error interceptor
   */
  addErrorInterceptor(interceptor: Interceptor): () => void {
    this.interceptors.error.push(interceptor);
    return () => {
      const index = this.interceptors.error.indexOf(interceptor);
      if (index > -1) {
        this.interceptors.error.splice(index, 1);
      }
    };
  }

  private async runInterceptors(
    phase: "request" | "response" | "error",
    context: InterceptorContext
  ): Promise<InterceptorContext> {
    let result = context;
    for (const interceptor of this.interceptors[phase]) {
      result = await interceptor(result);
    }
    return result;
  }

  private buildUrl(path: string): string {
    if (path.startsWith("http")) {
      return path;
    }
    const baseURL = this.config.baseURL.replace(/\/$/, "");
    const cleanPath = path.startsWith("/") ? path : `/${path}`;
    return `${baseURL}${cleanPath}`;
  }

  /**
   * Make a request to the API
   */
  async request<T = unknown>(path: string, config: RequestConfig = {}): Promise<ApiResponse<T>> {
    const url = this.buildUrl(path);
    const timeout = config.timeout ?? this.config.timeout;

    // Build request context
    let context: InterceptorContext = {
      url,
      method: config.method ?? "GET",
      headers: {
        ...this.config.headers,
        ...config.headers,
      },
      body: config.body,
      signal: config.signal,
    };

    // Run request interceptors
    context = await this.runInterceptors("request", context);

    // Create abort controller for timeout
    const abortController = new AbortController();
    let timeoutId: NodeJS.Timeout | null = null;

    if (timeout > 0) {
      timeoutId = setTimeout(() => {
        abortController.abort();
      }, timeout);
    }

    // Merge signals if provided
    if (context.signal) {
      context.signal.addEventListener("abort", () => {
        abortController.abort();
      });
    }

    try {
      const response = await fetch(context.url, {
        method: context.method,
        headers: context.headers,
        body: context.body ? JSON.stringify(context.body) : undefined,
        credentials: this.config.credentials,
        signal: abortController.signal,
      });

      if (timeoutId) {
        clearTimeout(timeoutId);
      }

      // Handle non-OK responses
      if (!response.ok) {
        const error = await createErrorFromResponse(response);
        throw error;
      }

      // Parse response
      let data: T;
      const contentType = response.headers.get("content-type");
      if (contentType?.includes("application/json")) {
        data = (await response.json()) as T;
      } else {
        data = (await response.text()) as unknown as T;
      }

      // Build response context
      let responseContext: InterceptorContext = {
        ...context,
        response,
        data,
      };

      // Run response interceptors
      responseContext = await this.runInterceptors("response", responseContext);

      return {
        data: responseContext.data as T,
        status: response.status,
        headers: response.headers,
      };
    } catch (error) {
      if (timeoutId) {
        clearTimeout(timeoutId);
      }

      // Handle timeout
      if (error instanceof Error && error.name === "AbortError") {
        const timeoutError = new TimeoutError(`Request timeout after ${timeout}ms`);
        throw timeoutError;
      }

      // Handle network errors
      if (error instanceof TypeError && error.message.includes("fetch")) {
        const networkError = new NetworkError("Network error - please check your connection");
        throw networkError;
      }

      // Re-throw API errors
      if (error instanceof ApiError) {
        // Run error interceptors
        const errorContext: InterceptorContext = {
          ...context,
          error,
        };
        await this.runInterceptors("error", errorContext);
        throw error;
      }

      // Wrap unknown errors
      const wrappedError = new ApiError(
        error instanceof Error ? error.message : "Unknown error",
        0,
        "unknown_error"
      );

      // Run error interceptors
      const errorContext: InterceptorContext = {
        ...context,
        error: wrappedError,
      };
      await this.runInterceptors("error", errorContext);

      throw wrappedError;
    }
  }

  /**
   * Make a GET request
   */
  get<T = unknown>(path: string, config?: Omit<RequestConfig, "method" | "body">): Promise<ApiResponse<T>> {
    return this.request<T>(path, { ...config, method: "GET" });
  }

  /**
   * Make a POST request
   */
  post<T = unknown>(path: string, body?: unknown, config?: Omit<RequestConfig, "method">): Promise<ApiResponse<T>> {
    return this.request<T>(path, { ...config, method: "POST", body });
  }

  /**
   * Make a PUT request
   */
  put<T = unknown>(path: string, body?: unknown, config?: Omit<RequestConfig, "method">): Promise<ApiResponse<T>> {
    return this.request<T>(path, { ...config, method: "PUT", body });
  }

  /**
   * Make a PATCH request
   */
  patch<T = unknown>(path: string, body?: unknown, config?: Omit<RequestConfig, "method">): Promise<ApiResponse<T>> {
    return this.request<T>(path, { ...config, method: "PATCH", body });
  }

  /**
   * Make a DELETE request
   */
  delete<T = unknown>(path: string, config?: Omit<RequestConfig, "method" | "body">): Promise<ApiResponse<T>> {
    return this.request<T>(path, { ...config, method: "DELETE" });
  }
}

/**
 * Create a default API client instance
 */
export function createApiClient(config: ApiClientConfig): ApiClient {
  return new ApiClient(config);
}
