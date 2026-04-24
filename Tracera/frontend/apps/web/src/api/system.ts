// System status API stub
import { client } from './client';
import { getMcpConfig } from './mcp-config';

const { apiClient, getAuthHeaders, API_BASE_URL } = client;

export interface SystemStatus {
  status: 'healthy' | 'degraded' | 'unhealthy';
  uptime: number;
  queuedJobs: number;
  version?: string;
  mcp?:
    | {
        baseUrl?: string | undefined;
        authMode?: string | undefined;
        requiresAuth?: boolean;
      }
    | undefined;
}

// Dashboard summary types matching the GET /api/v1/dashboard/summary endpoint
export interface DashboardProjectStats {
  completedCount: number;
  statusCounts: Record<string, number>;
  totalCount: number;
  typeCounts: Record<string, number>;
}

export interface DashboardSummary {
  perProject: Record<string, DashboardProjectStats>;
  projectCount: number;
  statusDistribution: Record<string, number>;
  totalItemCount: number;
  typeDistribution: Record<string, number>;
}

const isRecord = (value: unknown): value is Record<string, unknown> =>
  Object.prototype.toString.call(value) === '[object Object]';

const numberRecordFrom = (value: unknown): Record<string, number> => {
  if (!isRecord(value)) {
    return {};
  }

  const parsed: Record<string, number> = {};
  for (const [key, recordValue] of Object.entries(value)) {
    if (typeof recordValue === 'number') {
      parsed[key] = recordValue;
    }
  }
  return parsed;
};

const projectStatsFrom = (value: unknown): DashboardProjectStats => {
  if (!isRecord(value)) {
    return {
      completedCount: 0,
      statusCounts: {},
      totalCount: 0,
      typeCounts: {},
    };
  }

  return {
    completedCount: typeof value['completedCount'] === 'number' ? value['completedCount'] : 0,
    statusCounts: numberRecordFrom(value['statusCounts']),
    totalCount: typeof value['totalCount'] === 'number' ? value['totalCount'] : 0,
    typeCounts: numberRecordFrom(value['typeCounts']),
  };
};

const dashboardSummaryFrom = (value: unknown): DashboardSummary => {
  if (!isRecord(value)) {
    return {
      perProject: {},
      projectCount: 0,
      statusDistribution: {},
      totalItemCount: 0,
      typeDistribution: {},
    };
  }

  const perProject: Record<string, DashboardProjectStats> = {};
  if (isRecord(value['perProject'])) {
    for (const [projectId, stats] of Object.entries(value['perProject'])) {
      perProject[projectId] = projectStatsFrom(stats);
    }
  }

  return {
    perProject,
    projectCount: typeof value['projectCount'] === 'number' ? value['projectCount'] : 0,
    statusDistribution: numberRecordFrom(value['statusDistribution']),
    totalItemCount: typeof value['totalItemCount'] === 'number' ? value['totalItemCount'] : 0,
    typeDistribution: numberRecordFrom(value['typeDistribution']),
  };
};

const defaultSystemStatus = (): SystemStatus => ({
  queuedJobs: 0,
  status: 'healthy',
  uptime: 99.9,
});

export const fetchDashboardSummary = async (signal?: AbortSignal): Promise<DashboardSummary> => {
  const requestInit: RequestInit = {
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
      ...getAuthHeaders(),
    },
  };
  if (signal !== undefined) {
    requestInit.signal = signal;
  }

  const res = await fetch(`${API_BASE_URL}/api/v1/dashboard/summary`, requestInit);
  if (!res.ok) {
    throw new Error(`Dashboard summary fetch failed: ${res.status}`);
  }
  const data: unknown = await res.json();
  return dashboardSummaryFrom(data);
};

export const fetchSystemStatus = async (): Promise<SystemStatus> => {
  // Try to fetch from health endpoint, fallback to mock data
  try {
    const [response, mcpConfig] = await Promise.all([
      apiClient.GET('/api/v1/health', {}),
      getMcpConfig().catch(() => undefined),
    ]);
    const responseData: unknown = response.data;
    if (isRecord(responseData)) {
      const merged = Object.assign(defaultSystemStatus(), responseData);
      if (mcpConfig) {
        merged.mcp = {
          authMode: mcpConfig.auth_mode ?? undefined,
          baseUrl: mcpConfig.mcp_base_url ?? undefined,
          requiresAuth: mcpConfig.requires_auth ?? false,
        };
      }
      return merged;
    }
  } catch {
    // Return mock data if endpoint doesn't exist
  }
  return defaultSystemStatus();
};
