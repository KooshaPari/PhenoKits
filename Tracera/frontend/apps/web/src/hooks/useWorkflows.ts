import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

import type { WorkflowRun, WorkflowSchedule } from '@tracertm/types';

import { client } from '@/api/client';

const { getAuthHeaders } = client;

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:4000';
const REFRESH_RUNS_INTERVAL_MS = 15_000;
const REFRESH_SCHEDULES_INTERVAL_MS = 30_000;

interface WorkflowRunsResponse {
  runs: WorkflowRun[];
  total: number;
}

interface WorkflowSchedulesResponse {
  schedules: WorkflowSchedule[];
  total: number;
}

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null;

const asStringOrUndefined = (value: unknown): string | undefined =>
  typeof value === 'string' ? value : undefined;

const asRecordOrUndefined = (value: unknown): Record<string, unknown> | undefined =>
  isRecord(value) ? value : undefined;

const parseTotal = (value: unknown): number => (typeof value === 'number' ? value : 0);

function transformRun(data: Record<string, unknown>): WorkflowRun {
  return {
    completedAt: asStringOrUndefined(data['completed_at']),
    createdAt: asStringOrUndefined(data['created_at']),
    createdByUserId: asStringOrUndefined(data['created_by_user_id']),
    errorMessage: asStringOrUndefined(data['error_message']),
    externalRunId: asStringOrUndefined(data['external_run_id']),
    graphId: asStringOrUndefined(data['graph_id']),
    id: asStringOrUndefined(data['id']) ?? '',
    payload: asRecordOrUndefined(data['payload']),
    projectId: asStringOrUndefined(data['project_id']),
    result: asRecordOrUndefined(data['result']),
    startedAt: asStringOrUndefined(data['started_at']),
    status: asStringOrUndefined(data['status']) ?? '',
    updatedAt: asStringOrUndefined(data['updated_at']),
    workflowName: asStringOrUndefined(data['workflow_name']) ?? '',
  };
}

function transformSchedule(data: Record<string, unknown>): WorkflowSchedule {
  return Object.assign({}, data, {
    additionalMetadata: asRecordOrUndefined(data['additional_metadata']),
    cronName: asStringOrUndefined(data['cron_name'] ?? data['name']),
    expression: asStringOrUndefined(data['expression'] ?? data['cron_expression']),
    id: asStringOrUndefined(data['id'] ?? data['cron_id']),
    workflowName: asStringOrUndefined(data['workflow_name']),
  });
}

const useWorkflowRuns = (
  projectId: string,
  status?: string,
  workflowName?: string,
  limit = 100,
): ReturnType<typeof useQuery<WorkflowRunsResponse>> =>
  useQuery({
    enabled: Boolean(projectId),
    queryFn: async () => {
      const params = new URLSearchParams({ limit: String(limit) });
      if (status !== undefined && status !== '') {
        params.set('status', status);
      }
      if (workflowName !== undefined && workflowName !== '') {
        params.set('workflow_name', workflowName);
      }
      const res = await fetch(`${API_URL}/api/v1/projects/${projectId}/workflows/runs?${params}`, {
        headers: { 'X-Bulk-Operation': 'true', ...getAuthHeaders() },
      });
      if (!res.ok) {
        throw new Error(`Failed to fetch workflow runs: ${res.status}`);
      }
      const data: unknown = await res.json();
      const runs = isRecord(data) && Array.isArray(data['runs']) ? data['runs'] : [];
      return {
        runs: runs.filter(isRecord).map((run) => transformRun(run)),
        total: isRecord(data) ? parseTotal(data['total']) : 0,
      };
    },
    queryKey: ['workflows', 'runs', projectId, status, workflowName, limit],
    refetchInterval: REFRESH_RUNS_INTERVAL_MS,
  });

const useWorkflowSchedules = (projectId: string): ReturnType<typeof useQuery<WorkflowSchedulesResponse>> =>
  useQuery({
    enabled: Boolean(projectId),
    queryFn: async () => {
      const res = await fetch(`${API_URL}/api/v1/projects/${projectId}/workflows/schedules`, {
        headers: { 'X-Bulk-Operation': 'true', ...getAuthHeaders() },
      });
      if (!res.ok) {
        throw new Error(`Failed to fetch workflow schedules: ${res.status}`);
      }
      const data: unknown = await res.json();
      const schedules = isRecord(data) && Array.isArray(data['schedules']) ? data['schedules'] : [];
      return {
        schedules: schedules.filter(isRecord).map((schedule) => transformSchedule(schedule)),
        total: isRecord(data) ? parseTotal(data['total']) : 0,
      };
    },
    queryKey: ['workflows', 'schedules', projectId],
    refetchInterval: REFRESH_SCHEDULES_INTERVAL_MS,
  });

const useBootstrapWorkflowSchedules = (): ReturnType<typeof useMutation<any, any, any, any>> => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (projectId: string) => {
      const res = await fetch(
        `${API_URL}/api/v1/projects/${projectId}/workflows/schedules/bootstrap`,
        {
          headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
          method: 'POST',
        },
      );
      if (!res.ok) {
        throw new Error(`Failed to bootstrap schedules: ${res.status}`);
      }
      return res.json();
    },
    onSuccess: (_data, projectId) =>
      queryClient.invalidateQueries({
        queryKey: ['workflows', 'schedules', projectId],
      }),
  });
};

const useDeleteWorkflowSchedule = (): ReturnType<typeof useMutation<any, any, any, any>> => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ projectId, cronId }: { projectId: string; cronId: string }) => {
      const res = await fetch(
        `${API_URL}/api/v1/projects/${projectId}/workflows/schedules/${cronId}`,
        { headers: getAuthHeaders(), method: 'DELETE' },
      );
      if (!res.ok) {
        throw new Error(`Failed to delete schedule: ${res.status}`);
      }
      return res.json();
    },
    onSuccess: (_data, variables) =>
      queryClient.invalidateQueries({
        queryKey: ['workflows', 'schedules', variables.projectId],
      }),
  });
};

export {
  useBootstrapWorkflowSchedules,
  useDeleteWorkflowSchedule,
  useWorkflowRuns,
  useWorkflowSchedules,
};
