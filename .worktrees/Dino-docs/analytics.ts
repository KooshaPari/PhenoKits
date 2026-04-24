/**
 * Analytics integration for Dino (DINOForge)
 * 
 * Traces to: FR-DINO-ANALYTICS-001
 * 
 * Product analytics for Diplomacy is Not an Option mod platform
 */

import { initAnalytics, track, identify, EventType } from '@phenotype/analytics';

const ANALYTICS_ENDPOINT = process.env.NEXT_PUBLIC_ANALYTICS_ENDPOINT || 'https://analytics.phenotype.dev/v1/events';
const ANALYTICS_KEY = process.env.NEXT_PUBLIC_ANALYTICS_KEY || '';

export function initDinoAnalytics() {
  if (!ANALYTICS_KEY) return;

  initAnalytics({
    endpoint: ANALYTICS_ENDPOINT,
    apiKey: ANALYTICS_KEY,
    environment: process.env.NODE_ENV || 'production',
    version: process.env.DINO_VERSION,
    debug: process.env.NODE_ENV === 'development',
  });
}

// Mod lifecycle
export function trackModPublished(
  modId: string,
  modName: string,
  authorId: string,
  category: string
) {
  identify(authorId, {
    mod_id: modId,
    mod_name: modName,
    published_at: new Date().toISOString(),
  });
  
  track(EventType.FEATURE_USED, {
    feature: 'mod',
    action: 'published',
    mod_id: modId,
    mod_name: modName,
    category,
  });
}

export function trackModDownloaded(modId: string, modName: string, version: string) {
  track(EventType.FEATURE_USED, {
    feature: 'mod',
    action: 'downloaded',
    mod_id: modId,
    mod_name: modName,
    version,
  });
}

export function trackModInstalled(modId: string, success: boolean, errorMessage?: string) {
  track(success ? EventType.OPERATION_COMPLETED : EventType.ERROR_OCCURRED, {
    operation: 'mod_installation',
    mod_id: modId,
    success,
    error_message: errorMessage,
  });
}

export function trackModEnabled(modId: string, loadTimeMs: number) {
  track(EventType.OPERATION_COMPLETED, {
    operation: 'mod_enable',
    mod_id: modId,
    load_time_ms: loadTimeMs,
  });
}

// User engagement
export function trackGameLaunched(userId: string, modCount: number) {
  track(EventType.FEATURE_USED, {
    feature: 'game_launch',
    user_id: userId,
    mod_count: modCount,
  });
}

export function trackWorkshopBrowse(category: string, filters: string[]) {
  track(EventType.PAGE_VIEW, {
    page: 'workshop',
    category,
    filters_applied: filters.length,
  });
}

// Creator analytics
export function trackModRating(modId: string, rating: number, reviewerId: string) {
  track(EventType.FEATURE_USED, {
    feature: 'mod_rating',
    mod_id: modId,
    rating,
    reviewer_id: reviewerId,
  });
}

export function trackModUpdated(modId: string, version: string, changeCount: number) {
  track(EventType.FEATURE_USED, {
    feature: 'mod',
    action: 'updated',
    mod_id: modId,
    version,
    change_count: changeCount,
  });
}
