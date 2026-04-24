/**
 * API Error Handler Utilities
 *
 * Provides error discrimination, user-friendly messages, and error metadata
 * for comprehensive error handling in the frontend.
 */

import { ApiError } from '@/api/client-errors';
import {
  isAuthError,
  isNetworkError,
  isServerError,
  isTimeoutError,
  isValidationError,
} from '@/lib/retry';

// Re-export error predicates for convenience
export { isAuthError, isNetworkError, isServerError, isTimeoutError, isValidationError };

export type ErrorType = 'network' | 'validation' | 'auth' | 'server' | 'unknown' | 'timeout';

export interface ErrorMetadata {
  type: ErrorType;
  message: string;
  userMessage: string;
  retryable: boolean;
  statusCode?: number | undefined;
  originalError: Error;
}

function isValidationErrorMap(value: unknown): value is Record<string, string[]> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    return false;
  }

  return Object.values(value).every(
    (fieldErrors) =>
      Array.isArray(fieldErrors) && fieldErrors.every((message) => typeof message === 'string'),
  );
}

/**
 * Discriminate error type from thrown error
 */
export function getErrorType(error: unknown): ErrorType {
  if (isNetworkError(error)) {
    return 'network';
  }
  if (isTimeoutError(error)) {
    return 'timeout';
  }
  if (isAuthError(error)) {
    return 'auth';
  }
  if (isValidationError(error)) {
    return 'validation';
  }
  if (isServerError(error)) {
    return 'server';
  }
  return 'unknown';
}

/**
 * Extract validation error details from API response
 * Handles common API error response formats
 */
export function extractValidationErrors(error: unknown): Record<string, string[]> | null {
  if (!(error instanceof ApiError)) {
    return null;
  }

  // Try to extract field errors from response data
  const data = error.data;
  if (typeof data !== 'object' || data === null || Array.isArray(data)) {
    return null;
  }

  // Format 1: { errors: { field: ["error1", "error2"] } }
  if ('errors' in data && isValidationErrorMap(data.errors)) {
    return data.errors;
  }

  // Format 2: { field: ["error1", "error2"] }
  // Check if object has string array values (looks like validation errors)
  if (isValidationErrorMap(data)) {
    return data;
  }

  return null;
}

/**
 * Generate user-friendly error message based on error type
 */
export function getUserFriendlyMessage(errorType: ErrorType, error?: Error): string {
  switch (errorType) {
    case 'network': {
      return 'Unable to connect to the server. Please check your internet connection and try again.';
    }
    case 'timeout': {
      return 'Request timed out. The server is taking too long to respond. Please try again.';
    }
    case 'validation': {
      return 'Please check your input and try again.';
    }
    case 'auth': {
      return 'Your session has expired. Please log in again.';
    }
    case 'server': {
      return 'Server error. Please try again later.';
    }
    case 'unknown': {
      return error?.message ?? 'An unexpected error occurred. Please try again.';
    }
    default: {
      return 'An error occurred. Please try again.';
    }
  }
}

/**
 * Build comprehensive error metadata from error object
 */
export function buildErrorMetadata(error: unknown): ErrorMetadata {
  const originalError = error instanceof Error ? error : new Error(String(error));
  const errorType = getErrorType(error);
  const userMessage = getUserFriendlyMessage(errorType, originalError);
  const statusCode = error instanceof ApiError ? error.status : undefined;

  return {
    message: originalError.message,
    originalError,
    retryable: errorType === 'network' || errorType === 'timeout' || errorType === 'server',
    statusCode,
    type: errorType,
    userMessage,
  };
}

/**
 * Format error for logging (include metadata but not sensitive data)
 */
export function formatErrorForLogging(metadata: ErrorMetadata): Record<string, unknown> {
  return {
    message: metadata.message,
    retryable: metadata.retryable,
    statusCode: metadata.statusCode,
    type: metadata.type,
    userMessage: metadata.userMessage,
  };
}

/**
 * Determine if error requires special handling (e.g., redirect to login for auth errors)
 */
export function requiresSpecialHandling(errorType: ErrorType): boolean {
  return errorType === 'auth';
}

/**
 * Format validation error message for display
 */
export function formatValidationErrorMessage(errors: Record<string, string[]>): string {
  const errorList = Object.entries(errors)
    .map(([field, messages]) => {
      const fieldName = field.replace(/_/g, ' ').toLowerCase();
      return `${fieldName}: ${messages.join(', ')}`;
    })
    .join('\n');

  return `Validation failed:\n${errorList}`;
}

/**
 * Truncate error message to reasonable length for display
 */
export function truncateErrorMessage(message: string, maxLength: number = 200): string {
  if (message.length <= maxLength) {
    return message;
  }
  return `${message.substring(0, maxLength)}...`;
}
