/**
 * Analytics integration for Planify
 * 
 * Traces to: FR-PLANIFY-ANALYTICS-001
 * 
 * Product analytics for project management platform
 */

import { initAnalytics, track, identify, EventType } from '@phenotype/analytics';

const ANALYTICS_ENDPOINT = process.env.NEXT_PUBLIC_ANALYTICS_ENDPOINT || 'https://analytics.phenotype.dev/v1/events';
const ANALYTICS_KEY = process.env.NEXT_PUBLIC_ANALYTICS_KEY || '';

export function initPlanifyAnalytics() {
  if (!ANALYTICS_KEY) return;

  initAnalytics({
    endpoint: ANALYTICS_ENDPOINT,
    apiKey: ANALYTICS_KEY,
    environment: process.env.NODE_ENV || 'development',
    version: process.env.PLANIFY_VERSION,
    debug: process.env.NODE_ENV === 'development',
  });
}

// Project management events
export function trackProjectCreated(projectId: string, projectName: string, userId: string) {
  track(EventType.FEATURE_USED, {
    feature: 'project',
    action: 'created',
    project_id: projectId,
    project_name: projectName,
    user_id: userId,
  });
}

export function trackIssueCreated(issueId: string, projectId: string, issueType: string) {
  track(EventType.FEATURE_USED, {
    feature: 'issue',
    action: 'created',
    issue_id: issueId,
    project_id: projectId,
    issue_type: issueType,
  });
}

export function trackIssueStatusChanged(issueId: string, from: string, to: string) {
  track(EventType.FEATURE_USED, {
    feature: 'issue',
    action: 'status_changed',
    issue_id: issueId,
    from_status: from,
    to_status: to,
  });
}

// Cycle/completion tracking
export function trackCycleStarted(cycleId: string, projectId: string) {
  track(EventType.WORKFLOW_STARTED, {
    workflow: 'cycle',
    cycle_id: cycleId,
    project_id: projectId,
  });
}

export function trackCycleCompleted(cycleId: string, projectId: string, duration: number) {
  track(EventType.WORKFLOW_COMPLETED, {
    workflow: 'cycle',
    cycle_id: cycleId,
    project_id: projectId,
    duration_ms: duration,
  });
}

// User collaboration
export function trackUserJoined(userId: string, projectId: string, role: string) {
  identify(userId, {
    project_id: projectId,
    role,
    joined_at: new Date().toISOString(),
  });
}
