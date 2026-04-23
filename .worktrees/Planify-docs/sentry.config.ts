/**
 * Sentry configuration for Planify
 * 
 * Traces to: FR-PLANIFY-SENTRY-001
 * 
 * Error tracking for project management platform
 */

import { init } from '@sentry/nextjs';
import { BrowserTracing } from '@sentry/tracing';

const SENTRY_DSN = process.env.NEXT_PUBLIC_SENTRY_DSN;

if (SENTRY_DSN && process.env.NODE_ENV === 'production') {
  init({
    dsn: SENTRY_DSN,
    environment: process.env.SENTRY_ENVIRONMENT || 'production',
    release: process.env.PLANIFY_RELEASE || process.env.VERCEL_GIT_COMMIT_SHA,
    
    integrations: [
      new BrowserTracing(),
    ],
    
    // Performance monitoring
    tracesSampleRate: 0.1,
    
    // Session replay for debugging user issues
    replaysSessionSampleRate: 0.01,
    replaysOnErrorSampleRate: 0.1,
    
    beforeSend(event) {
      // Sanitize user data
      if (event.user) {
        delete event.user.email;
        delete event.user.ip_address;
      }
      return event;
    },
  });
}

export const captureUserAction = (action: string, metadata?: Record<string, unknown>) => {
  if (typeof window !== 'undefined' && (window as any).Sentry) {
    (window as any).Sentry.addBreadcrumb({
      category: 'user-action',
      message: action,
      level: 'info',
      data: metadata,
    });
  }
};

export { SENTRY_DSN };
