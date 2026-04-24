/**
 * Sentry configuration for Dino (DINOForge mod platform)
 * 
 * Traces to: FR-DINO-SENTRY-001
 * 
 * Error tracking for Diplomacy is Not an Option mod platform
 */

import { init } from '@sentry/nextjs';

const SENTRY_DSN = process.env.NEXT_PUBLIC_SENTRY_DSN;

if (SENTRY_DSN && process.env.NODE_ENV === 'production') {
  init({
    dsn: SENTRY_DSN,
    environment: process.env.SENTRY_ENVIRONMENT || 'production',
    release: process.env.DINO_RELEASE || process.env.VERCEL_GIT_COMMIT_SHA,
    
    // Error sampling
    sampleRate: 1.0,
    
    // Performance traces
    tracesSampleRate: 0.1,
    
    beforeSend(event) {
      // Filter out mod-loading errors that aren't our fault
      if (event.exception?.values?.some(e => 
        e.value?.includes('mod failed to load') || 
        e.value?.includes('incompatible mod version')
      )) {
        event.level = 'warning';
      }
      return event;
    },
  });
}

export const captureModError = (error: Error, modId: string, modVersion: string) => {
  if (typeof window !== 'undefined' && (window as any).Sentry) {
    (window as any).Sentry.withScope((scope: any) => {
      scope.setTag('mod.id', modId);
      scope.setTag('mod.version', modVersion);
      scope.setLevel('error');
      (window as any).Sentry.captureException(error);
    });
  }
};

export { SENTRY_DSN };
