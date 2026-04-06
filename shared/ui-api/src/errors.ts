/**
 * API Error Classes
 *
 * Standardized error types for API operations with proper TypeScript support.
 */

export class ApiError extends Error {
  public readonly status: number;
  public readonly code: string;
  public readonly details?: Record<string, unknown>;

  constructor(message: string, status: number, code: string, details?: Record<string, unknown>) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.code = code;
    this.details = details;

    // Maintains proper stack trace for where error was thrown
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, ApiError);
    }
  }
}

export class NetworkError extends ApiError {
  constructor(message = "Network error") {
    super(message, 0, "network_error");
    this.name = "NetworkError";
  }
}

export class TimeoutError extends ApiError {
  constructor(message = "Request timeout") {
    super(message, 408, "timeout_error");
    this.name = "TimeoutError";
  }
}

export class ValidationError extends ApiError {
  public readonly fieldErrors: Record<string, string[]>;

  constructor(message: string, fieldErrors: Record<string, string[]>) {
    super(message, 400, "validation_error", { fieldErrors });
    this.name = "ValidationError";
    this.fieldErrors = fieldErrors;
  }
}

export class AuthenticationError extends ApiError {
  constructor(message = "Authentication required") {
    super(message, 401, "authentication_error");
    this.name = "AuthenticationError";
  }
}

/**
 * Type guard to check if an error is an ApiError
 */
export function isApiError(error: unknown): error is ApiError {
  return error instanceof ApiError;
}

/**
 * Create an appropriate error from an HTTP response
 */
export async function createErrorFromResponse(response: Response): Promise<ApiError> {
  const status = response.status;
  let message = `HTTP error ${status}`;
  let code = "http_error";
  let details: Record<string, unknown> | undefined;

  try {
    const body = (await response.json()) as Record<string, unknown>;
    message = (body.message as string) || (body.error as string) || message;
    code = (body.code as string) || code;
    details = body;

    // Handle validation errors
    if (status === 400 && body.fieldErrors) {
      return new ValidationError(
        message,
        body.fieldErrors as Record<string, string[]>
      );
    }
  } catch {
    // If JSON parsing fails, use text if available
    try {
      const text = await response.text();
      if (text) {
        message = text;
      }
    } catch {
      // Ignore text parsing errors
    }
  }

  // Create specific error types based on status
  switch (status) {
    case 401:
      return new AuthenticationError(message);
    case 408:
      return new TimeoutError(message);
    default:
      return new ApiError(message, status, code, details);
  }
}

/**
 * Get a user-friendly error message
 */
export function getErrorMessage(error: unknown): string {
  if (error instanceof ApiError) {
    return error.message;
  }
  if (error instanceof Error) {
    return error.message;
  }
  return String(error);
}

/**
 * Check if an error is a network-related error
 */
export function isNetworkError(error: unknown): boolean {
  return error instanceof NetworkError || error instanceof TimeoutError;
}

/**
 * Check if an error is an authentication error
 */
export function isAuthenticationError(error: unknown): boolean {
  return error instanceof AuthenticationError ||
    (error instanceof ApiError && error.status === 401);
}
