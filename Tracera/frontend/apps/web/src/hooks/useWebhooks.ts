import type { UseMutationResult, UseQueryResult } from '@tanstack/react-query';
import { useMutation, useQuery } from '@tanstack/react-query';
import type {
  WebhookIntegration,
  WebhookLog,
  WebhookProvider,
  WebhookStats,
  WebhookStatus,
} from '@tracertm/types';
import { client } from '@/api/client';
const { getAuthHeaders } = client;
const API_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:4000';
const toNumber = (value: unknown, fallback: number): number => (value === undefined ? fallback : Number(value));
const toPrimitiveString = (value: unknown): string | undefined => {
  if (
    typeof value === 'string' ||
    typeof value === 'number' ||
    typeof value === 'boolean' ||
    typeof value === 'bigint'
  ) {
    return String(value);
  }
  return value === null ? 'null' : undefined;
};
const toString = (value: unknown, fallback: string): string => toPrimitiveString(value) ?? fallback;
const toStringOrUndefined = (value: unknown): string | undefined => toPrimitiveString(value);
const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null && !Array.isArray(value);
const toStringRecordOrUndefined = (value: unknown): Record<string, string> | undefined => {
  if (!isRecord(value)) return undefined;
  const record: Record<string, string> = {};
  for (const [key, recordValue] of Object.entries(value)) {
    const stringValue = toPrimitiveString(recordValue);
    if (stringValue === undefined) return undefined;
    record[key] = stringValue;
  }
  return record;
};
const toUnknownRecordOrUndefined = (value: unknown): Record<string, unknown> | undefined => (isRecord(value) ? value : undefined);
const toStringArrayOrUndefined = (value: unknown): string[] | undefined => {
  if (!Array.isArray(value)) return undefined;
  const strings: string[] = [];
  for (const item of value) {
    const stringValue = toPrimitiveString(item);
    if (stringValue === undefined) return undefined;
    strings.push(stringValue);
  }
  return strings;
};
const isWebhookProvider = (value: unknown): value is WebhookProvider =>
  value === 'github_actions' ||
  value === 'gitlab_ci' ||
  value === 'jenkins' ||
  value === 'azure_devops' ||
  value === 'circleci' ||
  value === 'travis_ci' ||
  value === 'custom';
const isWebhookStatus = (value: unknown): value is WebhookStatus =>
  value === 'active' || value === 'paused' || value === 'disabled';
const toRecordArrayOrUndefined = (value: unknown): Record<string, unknown>[] | undefined => {
  if (!Array.isArray(value)) return undefined;
  const records: Record<string, unknown>[] = [];
  for (const item of value) {
    if (isRecord(item)) {
      records.push(item);
    }
  }
  return records;
};
const toNumberRecordOrUndefined = (value: unknown): Record<string, number> | undefined => {
  if (!isRecord(value)) return undefined;
  const record: Record<string, number> = {};
  for (const [key, recordValue] of Object.entries(value)) {
    record[key] = Number(recordValue);
  }
  return record;
};
const toNumberOrUndefined = (value: unknown): number | undefined => (value === undefined ? undefined : Number(value));
const toBoolean = (value: unknown): boolean => value === true;
const parseJsonRecord = async (response: Response): Promise<Record<string, unknown>> => {
  const data: unknown = await response.json();
  if (!isRecord(data)) {
    throw new Error('Expected a JSON object response');
  }
  return data;
};
function transformWebhook(data: Record<string, unknown>): WebhookIntegration {
  return {
    apiKey: toString(data['api_key'], ''),
    autoCompleteRun: toBoolean(data['auto_complete_run']),
    autoCreateRun: toBoolean(data['auto_create_run']),
    callbackHeaders: toStringRecordOrUndefined(data['callback_headers']),
    callbackUrl: toString(data['callback_url'], ''),
    createdAt: toString(data['created_at'], ''),
    defaultSuiteId: toString(data['default_suite_id'], ''),
    description: toStringOrUndefined(data['description']),
    enabledEvents: toStringArrayOrUndefined(data['enabled_events']),
    eventFilters: toUnknownRecordOrUndefined(data['event_filters']),
    failedRequests: toNumber(data['failed_requests'], 0),
    id: String(data['id']),
    lastErrorMessage: toStringOrUndefined(data['last_error_message']),
    lastFailureAt: toStringOrUndefined(data['last_failure_at']),
    lastRequestAt: toString(data['last_request_at'], ''),
    lastSuccessAt: toStringOrUndefined(data['last_success_at']),
    metadata: toUnknownRecordOrUndefined(data['webhook_metadata']),
    name: String(data['name']),
    projectId: String(data['project_id']),
    provider: isWebhookProvider(data['provider']) ? data['provider'] : 'custom',
    rateLimitPerMinute: toNumber(data['rate_limit_per_minute'], 0),
    status: isWebhookStatus(data['status']) ? data['status'] : 'disabled',
    successfulRequests: toNumber(data['successful_requests'], 0),
    totalRequests: toNumber(data['total_requests'], 0),
    updatedAt: toString(data['updated_at'], ''),
    verifySignatures: toBoolean(data['verify_signatures']),
    version: toNumber(data['version'], 0),
    webhookSecret: toString(data['webhook_secret'], ''),
  };
}

function transformWebhookLog(data: Record<string, unknown>): WebhookLog {
  return {
    createdAt: String(data['created_at']),
    errorMessage: toStringOrUndefined(data['error_message']),
    eventType: String(data['event_type']),
    httpMethod: String(data['http_method']),
    id: String(data['id']),
    payloadSizeBytes: toNumberOrUndefined(data['payload_size_bytes']),
    processingTimeMs: toNumber(data['processing_time_ms'], 0),
    requestBodyPreview: toStringOrUndefined(data['request_body_preview']),
    requestHeaders: toUnknownRecordOrUndefined(data['request_headers']),
    requestId: toString(data['request_id'], ''),
    resultsSubmitted: Number(data['results_submitted'] ?? 0),
    sourceIp: toStringOrUndefined(data['source_ip']),
    statusCode: toNumber(data['status_code'], 0),
    success: toBoolean(data['success']),
    testRunId: toStringOrUndefined(data['test_run_id']),
    userAgent: toStringOrUndefined(data['user_agent']),
    webhookId: String(data['webhook_id']),
  };
}

interface WebhookFilters {
  projectId: string;
  provider?: WebhookProvider | undefined;
  status?: WebhookStatus | undefined;
  skip?: number | undefined;
  limit?: number | undefined;
}

async function fetchWebhooks(
  filters: WebhookFilters,
): Promise<{ webhooks: WebhookIntegration[]; total: number }> {
  const params = new URLSearchParams();
  params.set('project_id', filters.projectId);
  if (filters.provider !== undefined) {
    params.set('provider', filters.provider);
  }
  if (filters.status !== undefined) {
    params.set('status', filters.status);
  }
  if (filters.skip !== undefined) {
    params.set('skip', String(filters.skip));
  }
  if (filters.limit !== undefined) {
    params.set('limit', String(filters.limit));
  }

  const res = await fetch(`${API_URL}/api/v1/webhooks?${params}`, {
    headers: {
      'X-Bulk-Operation': 'true',
      ...getAuthHeaders(),
    },
  });
  if (!res.ok) {
    const errorText = await res.text();
    throw new Error(`Failed to fetch webhooks: ${res.status} ${errorText}`);
  }
  const data = await parseJsonRecord(res);
  const webhooks = toRecordArrayOrUndefined(data['webhooks']) ?? [];
  return {
    total: Number(data['total'] ?? 0),
    webhooks: webhooks.map((item) => transformWebhook(item)),
  };
}

async function fetchWebhook(id: string): Promise<WebhookIntegration> {
  const res = await fetch(`${API_URL}/api/v1/webhooks/${id}`, {
    headers: getAuthHeaders(),
  });
  if (!res.ok) {
    throw new Error('Failed to fetch webhook');
  }
  const data = await parseJsonRecord(res);
  return transformWebhook(data);
}

interface CreateWebhookData {
  projectId: string;
  name: string;
  description?: string | undefined;
  provider?: WebhookProvider | undefined;
  enabledEvents?: string[] | undefined;
  eventFilters?: Record<string, unknown> | undefined;
  callbackUrl?: string | undefined;
  callbackHeaders?: Record<string, string> | undefined;
  defaultSuiteId?: string | undefined;
  rateLimitPerMinute?: number | undefined;
  autoCreateRun?: boolean | undefined;
  autoCompleteRun?: boolean | undefined;
  verifySignatures?: boolean | undefined;
  metadata?: Record<string, unknown> | undefined;
}

async function createWebhook(data: CreateWebhookData): Promise<WebhookIntegration> {
  const res = await fetch(`${API_URL}/api/v1/webhooks`, {
    body: JSON.stringify({
      auto_complete_run: data['autoCompleteRun'] ?? true,
      auto_create_run: data['autoCreateRun'] ?? true,
      callback_headers: data['callbackHeaders'],
      callback_url: data['callbackUrl'],
      default_suite_id: data['defaultSuiteId'],
      description: data['description'],
      enabled_events: data['enabledEvents'],
      event_filters: data['eventFilters'],
      metadata: data['metadata'],
      name: data.name,
      project_id: data['projectId'],
      provider: data['provider'] ?? 'custom',
      rate_limit_per_minute: data['rateLimitPerMinute'] ?? 60,
      verify_signatures: data['verifySignatures'] ?? true,
    }),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'POST',
  });
  if (!res.ok) {
    const errorText = await res.text();
    throw new Error(`Failed to create webhook: ${res.status} ${errorText}`);
  }
  const result = await parseJsonRecord(res);
  return transformWebhook(result);
}

interface UpdateWebhookData {
  name?: string | undefined;
  description?: string | undefined;
  enabledEvents?: string[] | undefined;
  eventFilters?: Record<string, unknown> | undefined;
  callbackUrl?: string | undefined;
  callbackHeaders?: Record<string, string> | undefined;
  defaultSuiteId?: string | undefined;
  rateLimitPerMinute?: number | undefined;
  autoCreateRun?: boolean | undefined;
  autoCompleteRun?: boolean | undefined;
  verifySignatures?: boolean | undefined;
  metadata?: Record<string, unknown> | undefined;
}

const setPayload = <T>(
  payload: Record<string, unknown>,
  key: string,
  value: T | undefined,
): void => {
  if (value !== undefined) {
    payload[key] = value;
  }
};

async function updateWebhook(id: string, data: UpdateWebhookData): Promise<WebhookIntegration> {
  const payload: Record<string, unknown> = {};
  setPayload(payload, 'name', data['name']);
  setPayload(payload, 'description', data['description']);
  setPayload(payload, 'enabled_events', data['enabledEvents']);
  setPayload(payload, 'event_filters', data['eventFilters']);
  setPayload(payload, 'callback_url', data['callbackUrl']);
  setPayload(payload, 'callback_headers', data['callbackHeaders']);
  setPayload(payload, 'default_suite_id', data['defaultSuiteId']);
  setPayload(payload, 'rate_limit_per_minute', data['rateLimitPerMinute']);
  setPayload(payload, 'auto_create_run', data['autoCreateRun']);
  setPayload(payload, 'auto_complete_run', data['autoCompleteRun']);
  setPayload(payload, 'verify_signatures', data['verifySignatures']);
  setPayload(payload, 'metadata', data['metadata']);

  const res = await fetch(`${API_URL}/api/v1/webhooks/${id}`, {
    body: JSON.stringify(payload),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'PUT',
  });
  if (!res.ok) {
    const errorText = await res.text();
    throw new Error(`Failed to update webhook: ${res.status} ${errorText}`);
  }
  const result = await parseJsonRecord(res);
  return transformWebhook(result);
}

async function setWebhookStatus(
  id: string,
  status: WebhookStatus,
): Promise<{ id: string; status: string; version: number }> {
  const res = await fetch(`${API_URL}/api/v1/webhooks/${id}/status`, {
    body: JSON.stringify({ status }),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'POST',
  });
  if (!res.ok) {
    const errorText = await res.text();
    throw new Error(`Failed to update status: ${res.status} ${errorText}`);
  }
  const data = await parseJsonRecord(res);
  return {
    id: String(data['id']),
    status: String(data['status']),
    version: Number(data['version'] ?? 0),
  };
}

async function regenerateSecret(
  id: string,
): Promise<{ id: string; webhookSecret: string; version: number }> {
  const res = await fetch(`${API_URL}/api/v1/webhooks/${id}/regenerate-secret`, {
    headers: getAuthHeaders(),
    method: 'POST',
  });
  if (!res.ok) {
    const errorText = await res.text();
    throw new Error(`Failed to regenerate secret: ${res.status} ${errorText}`);
  }
  const data = await res.json();
  return {
    id: data['id'],
    version: data['version'],
    webhookSecret: data['webhook_secret'],
  };
}

async function deleteWebhook(id: string): Promise<void> {
  const res = await fetch(`${API_URL}/api/v1/webhooks/${id}`, {
    headers: getAuthHeaders(),
    method: 'DELETE',
  });
  if (!res.ok) {
    const errorText = await res.text();
    throw new Error(`Failed to delete webhook: ${res.status} ${errorText}`);
  }
}

interface WebhookLogFilters {
  webhookId: string;
  success?: boolean | undefined;
  eventType?: string | undefined;
  skip?: number | undefined;
  limit?: number | undefined;
}

async function fetchWebhookLogs(
  filters: WebhookLogFilters,
): Promise<{ logs: WebhookLog[]; total: number }> {
  const params = new URLSearchParams();
  if (filters.success !== undefined) {
    params.set('success', String(filters.success));
  }
  if (filters.eventType !== undefined && filters.eventType !== '') {
    params.set('event_type', filters.eventType);
  }
  if (filters.skip !== undefined) {
    params.set('skip', String(filters.skip));
  }
  if (filters.limit !== undefined) {
    params.set('limit', String(filters.limit));
  }

  const res = await fetch(`${API_URL}/api/v1/webhooks/${filters.webhookId}/logs?${params}`, {
    headers: getAuthHeaders(),
  });
  if (!res.ok) {
    throw new Error('Failed to fetch webhook logs');
  }
  const data = await parseJsonRecord(res);
  const logs = toRecordArrayOrUndefined(data['logs']) ?? [];
  return {
    logs: logs.map((item) => transformWebhookLog(item)),
    total: Number(data['total'] ?? 0),
  };
}

async function fetchWebhookStats(projectId: string): Promise<WebhookStats> {
  const res = await fetch(`${API_URL}/api/v1/projects/${projectId}/webhooks/stats`, {
    headers: getAuthHeaders(),
  });
  if (!res.ok) {
    throw new Error('Failed to fetch webhook stats');
  }
  const data = await parseJsonRecord(res);
  const byProvider = toNumberRecordOrUndefined(data['by_provider']) ?? {};
  const byStatus = toNumberRecordOrUndefined(data['by_status']) ?? {};
  return {
    byProvider,
    byStatus,
    failedRequests: toNumber(data['failed_requests'], 0),
    projectId: toString(data['project_id'], ''),
    successfulRequests: toNumber(data['successful_requests'], 0),
    total: toNumber(data['total'], 0),
    totalRequests: toNumber(data['total_requests'], 0),
  };
}

// ==================== HOOKS ====================

function useWebhooks(
  filters: WebhookFilters,
): UseQueryResult<{ webhooks: WebhookIntegration[]; total: number }> {
  return useQuery({
    enabled: filters.projectId !== '',
    queryFn: async () => fetchWebhooks(filters),
    queryKey: ['webhooks', JSON.stringify(filters)],
  });
}

function useWebhook(id = ''): UseQueryResult<WebhookIntegration> {
  return useQuery({
    enabled: id !== '',
    queryFn: async () => fetchWebhook(id),
    queryKey: ['webhook', id],
  });
}

function useCreateWebhook(): UseMutationResult<WebhookIntegration, Error, CreateWebhookData> {
  return useMutation({
    mutationFn: createWebhook,
  });
}

function useUpdateWebhook(): UseMutationResult<
  WebhookIntegration,
  Error,
  { id: string; data: UpdateWebhookData }
> {
  return useMutation({
    mutationFn: async ({ id, data }: { id: string; data: UpdateWebhookData }) =>
      updateWebhook(id, data),
  });
}

function useSetWebhookStatus(): UseMutationResult<
  { id: string; status: string; version: number },
  Error,
  { id: string; status: WebhookStatus }
> {
  return useMutation({
    mutationFn: async ({ id, status }: { id: string; status: WebhookStatus }) =>
      setWebhookStatus(id, status),
  });
}

function useRegenerateWebhookSecret(): UseMutationResult<
  { id: string; webhookSecret: string; version: number },
  Error,
  string
> {
  return useMutation({
    mutationFn: regenerateSecret,
  });
}

function useDeleteWebhook(): UseMutationResult<void, Error, string> {
  return useMutation({
    mutationFn: deleteWebhook,
  });
}

function useWebhookLogs(
  filters: WebhookLogFilters,
): UseQueryResult<{ logs: WebhookLog[]; total: number }> {
  return useQuery({
    enabled: filters.webhookId !== '',
    queryFn: async () => fetchWebhookLogs(filters),
    queryKey: ['webhookLogs', JSON.stringify(filters)],
  });
}

function useWebhookStats(projectId = ''): UseQueryResult<WebhookStats> {
  return useQuery({
    enabled: projectId !== '',
    queryFn: async () => fetchWebhookStats(projectId),
    queryKey: ['webhookStats', projectId],
  });
}

export type { CreateWebhookData };
export {
  useCreateWebhook,
  useDeleteWebhook,
  useRegenerateWebhookSecret,
  useSetWebhookStatus,
  useUpdateWebhook,
  useWebhook,
  useWebhookLogs,
  useWebhookStats,
  useWebhooks,
};
