import * as reactQuery from '@tanstack/react-query';

import type * as TracerTypes from '@tracertm/types';

import { API_URL, getAuthHeaders } from '@/hooks/integrationsApi';

interface GitHubReposResponse {
  repos: TracerTypes.GitHubRepo[];
  page: number;
  perPage: number;
}

interface GitHubIssuesResponse {
  issues: TracerTypes.GitHubIssue[];
  page: number;
  perPage: number;
}

interface GitHubProjectsResponse {
  projects: TracerTypes.GitHubProject[];
}

interface LinearTeamsResponse {
  teams: TracerTypes.LinearTeam[];
}

interface LinearIssuesResponse {
  issues: TracerTypes.LinearIssue[];
}

interface LinearProjectsResponse {
  projects: TracerTypes.LinearProject[];
}

type JsonRecord = Record<string, unknown>;

const ZERO = Number('0');

function isRecord(value: unknown): value is JsonRecord {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function toRecord(value: unknown): JsonRecord {
  return isRecord(value) ? value : {};
}

function toRecordArray(value: unknown): JsonRecord[] {
  return Array.isArray(value) ? value.filter(isRecord) : [];
}

function toBoolean(value: unknown): boolean {
  return value === true;
}

function toNumber(value: unknown): number {
  return typeof value === 'number' ? value : ZERO;
}

function toString(value: unknown): string {
  return typeof value === 'string' ? value : '';
}

function toOptionalString(value: unknown): string | undefined {
  return typeof value === 'string' ? value : undefined;
}

function parseGithubRepo(value: unknown): TracerTypes.GitHubRepo {
  const repo = toRecord(value);
  const owner = toRecord(repo['owner']);
  return {
    id: toNumber(repo['id']),
    name: toString(repo['name']),
    fullName: toString(repo['full_name']),
    description: toOptionalString(repo['description']),
    htmlUrl: toString(repo['html_url']),
    private: toBoolean(repo['private']),
    owner: {
      avatarUrl: toString(owner['avatar_url']),
      login: toString(owner['login']),
    },
    defaultBranch: toString(repo['default_branch']),
    updatedAt: toOptionalString(repo['updated_at']),
  };
}

function parseGithubIssue(value: unknown): TracerTypes.GitHubIssue {
  const issue = toRecord(value);
  const user = toRecord(issue['user']);
  return {
    id: toNumber(issue['id']),
    number: toNumber(issue['number']),
    title: toString(issue['title']),
    state: toString(issue['state']),
    htmlUrl: toString(issue['html_url']),
    body: toOptionalString(issue['body']),
    user: {
      avatarUrl: toString(user['avatar_url']),
      login: toString(user['login']),
    },
    labels: toRecordList(issue['labels']),
    assignees: toRecordList(issue['assignees']),
    createdAt: toString(issue['created_at']),
    updatedAt: toString(issue['updated_at']),
  };
}

function parseGithubProject(value: unknown): TracerTypes.GitHubProject {
  const project = toRecord(value);
  return {
    id: toString(project['id']),
    title: toString(project['title']),
    description: toOptionalString(project['description']),
    url: toString(project['url']),
    closed: toBoolean(project['closed']),
    public: toBoolean(project['public']),
    createdAt: toOptionalString(project['created_at']),
    updatedAt: toOptionalString(project['updated_at']),
  };
}

function parseLinearTeam(value: unknown): TracerTypes.LinearTeam {
  const team = toRecord(value);
  return {
    id: toString(team['id']),
    name: toString(team['name']),
    key: toString(team['key']),
    description: toOptionalString(team['description']),
    icon: toOptionalString(team['icon']),
    color: toOptionalString(team['color']),
  };
}

function parseLinearIssue(value: unknown): TracerTypes.LinearIssue {
  const issue = toRecord(value);
  return {
    id: toString(issue['id']),
    identifier: toString(issue['identifier']),
    title: toString(issue['title']),
    description: toOptionalString(issue['description']),
    state: toOptionalString(issue['state']),
    priority: toNumber(issue['priority']),
    url: toString(issue['url']),
    assignee: toOptionalString(issue['assignee']),
    labels: toRecordList(issue['labels']),
    createdAt: toOptionalString(issue['created_at']),
    updatedAt: toOptionalString(issue['updated_at']),
  };
}

function parseLinearProject(value: unknown): TracerTypes.LinearProject {
  const project = toRecord(value);
  return {
    id: toString(project['id']),
    name: toString(project['name']),
    description: toOptionalString(project['description']),
    state: toOptionalString(project['state']),
    progress: toNumber(project['progress']),
    url: toString(project['url']),
    startDate: toOptionalString(project['start_date']),
    targetDate: toOptionalString(project['target_date']),
  };
}

function parseGitHubReposResponse(raw: unknown): GitHubReposResponse {
  const payload = toRecord(raw);
  return {
    page: toNumber(payload['page']),
    perPage: toNumber(payload['per_page']),
    repos: toRecordArray(payload['repos']).map(parseGithubRepo),
  };
}

function parseGitHubIssuesResponse(raw: unknown): GitHubIssuesResponse {
  const payload = toRecord(raw);
  return {
    issues: toRecordArray(payload['issues']).map(parseGithubIssue),
    page: toNumber(payload['page']),
    perPage: toNumber(payload['per_page']),
  };
}

function parseGitHubProjectsResponse(raw: unknown): GitHubProjectsResponse {
  const payload = toRecord(raw);
  return {
    projects: toRecordArray(payload['projects']).map(parseGithubProject),
  };
}

function parseLinearTeamsResponse(raw: unknown): LinearTeamsResponse {
  const payload = toRecord(raw);
  return {
    teams: toRecordArray(payload['teams']).map(parseLinearTeam),
  };
}

function parseLinearIssuesResponse(raw: unknown): LinearIssuesResponse {
  const payload = toRecord(raw);
  return {
    issues: toRecordArray(payload['issues']).map(parseLinearIssue),
  };
}

function parseLinearProjectsResponse(raw: unknown): LinearProjectsResponse {
  const payload = toRecord(raw);
  return {
    projects: toRecordArray(payload['projects']).map(parseLinearProject),
  };
}

function toRecordList(value: unknown): string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === 'string')
    : [];
}

async function fetchGitHubRepos(
  credentialId: string,
  search?: string,
  page?: number,
): Promise<GitHubReposResponse> {
  const params = new URLSearchParams({ credential_id: credentialId });
  if (search !== undefined && search !== '') {
    params.set('search', search);
  }
  if (page !== undefined) {
    params.set('page', String(page));
  }

  const res = await fetch(`${API_URL}/api/v1/integrations/github/repos?${params}`, {
    headers: { 'X-Bulk-Operation': 'true', ...getAuthHeaders() },
  });
  if (!res.ok) {
    throw new Error(`Failed to fetch GitHub repos: ${res.status}`);
  }
  return parseGitHubReposResponse(await res.json());
}

async function fetchGitHubIssues(
  credentialId: string,
  owner: string,
  repo: string,
  state?: string,
  page?: number,
): Promise<GitHubIssuesResponse> {
  const params = new URLSearchParams({ credential_id: credentialId });
  if (state !== undefined) {
    params.set('state', state);
  }
  if (page !== undefined) {
    params.set('page', String(page));
  }

  const res = await fetch(
    `${API_URL}/api/v1/integrations/github/repos/${owner}/${repo}/issues?${params}`,
    { headers: { 'X-Bulk-Operation': 'true', ...getAuthHeaders() } },
  );
  if (!res.ok) {
    throw new Error(`Failed to fetch GitHub issues: ${res.status}`);
  }
  return parseGitHubIssuesResponse(await res.json());
}

async function fetchGitHubProjects(
  credentialId: string,
  owner: string,
  isOrg?: boolean,
): Promise<GitHubProjectsResponse> {
  const params = new URLSearchParams({
    credential_id: credentialId,
    owner,
  });
  if (isOrg !== undefined) {
    params.set('is_org', String(isOrg));
  }

  const res = await fetch(`${API_URL}/api/v1/integrations/github/projects?${params}`, {
    headers: { 'X-Bulk-Operation': 'true', ...getAuthHeaders() },
  });
  if (!res.ok) {
    throw new Error(`Failed to fetch GitHub projects: ${res.status}`);
  }
  return parseGitHubProjectsResponse(await res.json());
}

async function fetchLinearTeams(credentialId: string): Promise<LinearTeamsResponse> {
  const res = await fetch(
    `${API_URL}/api/v1/integrations/linear/teams?credential_id=${credentialId}`,
    { headers: { 'X-Bulk-Operation': 'true', ...getAuthHeaders() } },
  );
  if (!res.ok) {
    throw new Error(`Failed to fetch Linear teams: ${res.status}`);
  }
  return parseLinearTeamsResponse(await res.json());
}

async function fetchLinearIssues(
  credentialId: string,
  teamId: string,
  first?: number,
): Promise<LinearIssuesResponse> {
  const params = new URLSearchParams({ credential_id: credentialId });
  if (first !== undefined) {
    params.set('first', String(first));
  }

  const res = await fetch(
    `${API_URL}/api/v1/integrations/linear/teams/${teamId}/issues?${params}`,
    { headers: { 'X-Bulk-Operation': 'true', ...getAuthHeaders() } },
  );
  if (!res.ok) {
    throw new Error(`Failed to fetch Linear issues: ${res.status}`);
  }
  return parseLinearIssuesResponse(await res.json());
}

async function fetchLinearProjects(
  credentialId: string,
  first?: number,
): Promise<LinearProjectsResponse> {
  const params = new URLSearchParams({ credential_id: credentialId });
  if (first !== undefined) {
    params.set('first', String(first));
  }

  const res = await fetch(`${API_URL}/api/v1/integrations/linear/projects?${params}`, {
    headers: { 'X-Bulk-Operation': 'true', ...getAuthHeaders() },
  });
  if (!res.ok) {
    throw new Error(`Failed to fetch Linear projects: ${res.status}`);
  }
  return parseLinearProjectsResponse(await res.json());
}

const useGitHubRepos = (
  credentialId: string,
  search?: string,
  page?: number,
): reactQuery.UseQueryResult<GitHubReposResponse> =>
  reactQuery.useQuery({
    enabled: Boolean(credentialId),
    queryFn: async () => fetchGitHubRepos(credentialId, search, page),
    queryKey: ['integrations', 'github', 'repos', credentialId, search, page],
  });

const useGitHubIssues = (
  credentialId: string,
  owner: string,
  repo: string,
  state?: string,
  page?: number,
): reactQuery.UseQueryResult<GitHubIssuesResponse> =>
  reactQuery.useQuery({
    enabled: Boolean(credentialId) && Boolean(owner) && Boolean(repo),
    queryFn: async () => fetchGitHubIssues(credentialId, owner, repo, state, page),
    queryKey: ['integrations', 'github', 'issues', credentialId, owner, repo, state, page],
  });

const useGitHubProjects = (
  credentialId: string,
  owner: string,
  isOrg?: boolean,
): reactQuery.UseQueryResult<GitHubProjectsResponse> =>
  reactQuery.useQuery({
    enabled: Boolean(credentialId) && Boolean(owner),
    queryFn: async () => fetchGitHubProjects(credentialId, owner, isOrg),
    queryKey: ['integrations', 'github', 'projects', credentialId, owner, isOrg],
  });

const useLinearTeams = (credentialId: string): reactQuery.UseQueryResult<LinearTeamsResponse> =>
  reactQuery.useQuery({
    enabled: Boolean(credentialId),
    queryFn: async () => fetchLinearTeams(credentialId),
    queryKey: ['integrations', 'linear', 'teams', credentialId],
  });

const useLinearIssues = (
  credentialId: string,
  teamId: string,
  first?: number,
): reactQuery.UseQueryResult<LinearIssuesResponse> =>
  reactQuery.useQuery({
    enabled: Boolean(credentialId) && Boolean(teamId),
    queryFn: async () => fetchLinearIssues(credentialId, teamId, first),
    queryKey: ['integrations', 'linear', 'issues', credentialId, teamId, first],
  });

const useLinearProjects = (
  credentialId: string,
  first?: number,
): reactQuery.UseQueryResult<LinearProjectsResponse> =>
  reactQuery.useQuery({
    enabled: Boolean(credentialId),
    queryFn: async () => fetchLinearProjects(credentialId, first),
    queryKey: ['integrations', 'linear', 'projects', credentialId, first],
  });

export {
  useGitHubIssues,
  useGitHubProjects,
  useGitHubRepos,
  useLinearIssues,
  useLinearProjects,
  useLinearTeams,
};
