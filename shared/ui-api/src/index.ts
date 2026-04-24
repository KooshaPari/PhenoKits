/**
 * Shared API Client
 *
 * This package provides centralized API client utilities for Phenotype UI projects.
 * Originally consolidated from BytePort frontend projects to eliminate duplication.
 */

export {
  ApiClient,
  type ApiClientConfig,
  type RequestConfig,
  type ApiResponse,
} from "./client.js";

export {
  ApiError,
  NetworkError,
  TimeoutError,
  ValidationError,
  AuthenticationError,
  isApiError,
  createErrorFromResponse,
} from "./errors.js";

export {
  loggingInterceptor,
  authInterceptor,
  retryInterceptor,
  rateLimitInterceptor,
  type Interceptor,
  type InterceptorContext,
} from "./interceptors.js";
