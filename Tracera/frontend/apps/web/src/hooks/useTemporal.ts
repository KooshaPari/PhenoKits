// UseTemporal - React Query hooks for temporal navigation
// Handles branch and version management

import * as ReactQuery from '@tanstack/react-query';

import type { Branch, Version } from '@/components/temporal/TemporalNavigator';

import { client } from '@/api/client';
import { QUERY_CONFIGS, queryKeys } from '@/lib/queryConfig';

const { getAuthHeaders } = client;
const { useMutation, useQuery, useQueryClient } = ReactQuery;

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:4000';
type JsonObject = Record<string, unknown>;
type TemporalConflict = Record<string, unknown>;
type JsonPrimitive = string | number | boolean | null;
type JsonValue = JsonPrimitive | JsonObject | JsonValue[];

function isJsonObject(value: unknown): value is JsonObject {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function isString(value: unknown): value is string {
  return typeof value === 'string';
}

function isNumber(value: unknown): value is number {
  return typeof value === 'number' && Number.isFinite(value);
}

function isBoolean(value: unknown): value is boolean {
  return typeof value === 'boolean';
}

function toDate(value: unknown): Date | null {
  if (value instanceof Date) {
    return Number.isNaN(value.getTime()) ? null : value;
  }

  if (!isString(value) && !isNumber(value)) {
    return null;
  }

  const parsedDate = new Date(value);
  return Number.isNaN(parsedDate.getTime()) ? null : parsedDate;
}

function isBranchStatus(value: unknown): value is Branch['status'] {
  return value === 'active' || value === 'review' || value === 'merged' || value === 'abandoned';
}

function isVersionStatus(value: unknown): value is Version['status'] {
  return value === 'draft' || value === 'published' || value === 'archived';
}

function isJsonValue(value: unknown): value is JsonValue {
  if (value === null) {
    return true;
  }

  if (isString(value) || isNumber(value) || isBoolean(value)) {
    return true;
  }

  if (Array.isArray(value)) {
    return value.every(isJsonValue);
  }

  return isJsonObject(value) && Object.values(value).every(isJsonValue);
}

function parseBranch(value: unknown): Branch | null {
  if (!isJsonObject(value)) {
    return null;
  }

  const id = value['id'];
  const name = value['name'];
  const status = value['status'];
  const description = value['description'];
  const parentId = value['parentId'];
  const author = value['author'];
  const mergeRequestCount = value['mergeRequestCount'];
  const createdAt = toDate(value['createdAt']);
  const updatedAt = toDate(value['updatedAt']);

  if (
    !isString(id) ||
    !isString(name) ||
    !isBranchStatus(status) ||
    !createdAt ||
    !updatedAt ||
    !isNumber(mergeRequestCount)
  ) {
    return null;
  }

  return {
    author: isString(author) ? author : undefined,
    createdAt,
    description: isString(description) ? description : undefined,
    id,
    mergeRequestCount,
    name,
    parentId: isString(parentId) ? parentId : undefined,
    status,
    updatedAt,
  };
}

function parseVersionSnapshot(value: unknown): VersionSnapshot | null {
  if (!isJsonObject(value) || !Object.values(value).every(isJsonValue)) {
    return null;
  }

  const snapshot: VersionSnapshot = {};
  for (const [key, entry] of Object.entries(value)) {
    if (!isJsonValue(entry)) {
      return null;
    }
    snapshot[key] = entry;
  }

  return snapshot;
}

function parseVersion(value: unknown): Version | null {
  if (!isJsonObject(value)) {
    return null;
  }

  const id = value['id'];
  const branchId = value['branchId'];
  const title = value['title'];
  const status = value['status'];
  const description = value['description'];
  const tag = value['tag'];
  const author = value['author'];
  const timestamp = toDate(value['timestamp']);
  if (
    !isString(id) ||
    !isString(branchId) ||
    !isString(title) ||
    !isVersionStatus(status) ||
    !timestamp
  ) {
    return null;
  }

  const resolvedSnapshot =
    value['snapshot'] === undefined ? undefined : parseVersionSnapshot(value['snapshot']);
  if (value['snapshot'] !== undefined && resolvedSnapshot === null) {
    return null;
  }

  return {
    author: isString(author) ? author : undefined,
    branchId,
    description: isString(description) ? description : undefined,
    id,
    snapshot: resolvedSnapshot ?? undefined,
    status,
    tag: isString(tag) ? tag : undefined,
    timestamp,
    title,
  };
}

function parseConflict(value: unknown): TemporalConflict | null {
  return isJsonObject(value) ? value : null;
}

async function parseJson(response: Response): Promise<unknown> {
  return (await response.json()) as unknown;
}

function getArrayFromPayload<Item>(
  payload: unknown,
  key: string,
  parseItem: (item: unknown) => Item | null,
): Item[] {
  if (Array.isArray(payload)) {
    return payload.map(parseItem).filter((item): item is Item => item !== null);
  }

  if (isJsonObject(payload)) {
    const fieldValue = payload[key];
    if (Array.isArray(fieldValue)) {
      return fieldValue.map(parseItem).filter((item): item is Item => item !== null);
    }
  }

  return [];
}
async function fetchBranches(projectId: string): Promise<Branch[]> {
  const res = await fetch(`${API_URL}/api/v1/projects/${projectId}/branches`, {
    headers: getAuthHeaders(),
  });
  if (!res.ok) {
    throw new Error('Failed to fetch branches');
  }
  const data = await parseJson(res);
  return getArrayFromPayload(data, 'branches', parseBranch);
}

export function useBranches(projectId: string): ReactQuery.UseQueryResult<Branch[]> {
  return useQuery({
    queryKey: [queryKeys.branches, projectId],
    queryFn: async () => fetchBranches(projectId),
    ...QUERY_CONFIGS.default,
  });
}

// FETCH VERSIONS
async function fetchVersions(branchId: string): Promise<Version[]> {
  const res = await fetch(`${API_URL}/api/v1/branches/${branchId}/versions`, {
    headers: getAuthHeaders(),
  });
  if (!res.ok) {
    throw new Error('Failed to fetch versions');
  }
  const data = await parseJson(res);
  return getArrayFromPayload(data, 'versions', parseVersion);
}

export function useVersions(branchId: string): ReactQuery.UseQueryResult<Version[]> {
  return useQuery({
    queryKey: [queryKeys.versions, branchId],
    queryFn: async () => fetchVersions(branchId),
    ...QUERY_CONFIGS.default,
  });
}

// CREATE BRANCH
interface CreateBranchInput {
  projectId: string;
  name: string;
  description?: string;
  parentId?: string;
}

async function createBranch(input: CreateBranchInput): Promise<Branch> {
  const res = await fetch(`${API_URL}/api/v1/projects/${input.projectId}/branches`, {
    body: JSON.stringify(input),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'POST',
  });
  if (!res.ok) {
    throw new Error('Failed to create branch');
  }
  const data = await parseJson(res);
  const branch = parseBranch(data);
  if (!branch) {
    throw new Error('Invalid branch payload');
  }
  return branch;
}

export function useCreateBranch(): ReactQuery.UseMutationResult<Branch, Error, CreateBranchInput> {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: createBranch,
    onSuccess: async (_data, variables) =>
      queryClient.invalidateQueries({
        queryKey: [queryKeys.branches, variables.projectId],
      }),
  });
}

// CREATE VERSION
interface CreateVersionInput {
  branchId: string;
  title: string;
  description?: string;
  tag?: string;
  status?: 'draft' | 'published' | 'archived';
}

async function createVersion(input: CreateVersionInput): Promise<Version> {
  const res = await fetch(`${API_URL}/api/v1/branches/${input.branchId}/versions`, {
    body: JSON.stringify(input),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'POST',
  });
  if (!res.ok) {
    throw new Error('Failed to create version');
  }
  const data = await parseJson(res);
  const version = parseVersion(data);
  if (!version) {
    throw new Error('Invalid version payload');
  }
  return version;
}

export function useCreateVersion(): ReactQuery.UseMutationResult<
  Version,
  Error,
  CreateVersionInput
> {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: createVersion,
    onSuccess: async (_data, variables) =>
      queryClient.invalidateQueries({
        queryKey: [queryKeys.versions, variables.branchId],
      }),
  });
}

// MERGE BRANCH
interface MergeBranchInput {
  sourceBranchId: string;
  targetBranchId: string;
  conflictResolution?: 'manual' | 'source' | 'target';
}

interface MergeBranchResult {
  success: boolean;
  conflicts?: TemporalConflict[];
}

async function mergeBranch(input: MergeBranchInput): Promise<MergeBranchResult> {
  const res = await fetch(`${API_URL}/api/v1/branches/${input.targetBranchId}/merge`, {
    body: JSON.stringify({
      conflictResolution: input.conflictResolution ?? 'manual',
      sourceBranchId: input.sourceBranchId,
    }),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'POST',
  });
  if (!res.ok) {
    throw new Error('Failed to merge branches');
  }
  const data = await parseJson(res);
  if (!isJsonObject(data) || typeof data['success'] !== 'boolean') {
    throw new Error('Invalid merge payload');
  }

  const conflicts = Array.isArray(data['conflicts'])
    ? data['conflicts'].map(parseConflict).filter((conflict): conflict is TemporalConflict => conflict !== null)
    : undefined;

  const success = data['success'];

  return conflicts === undefined ? { success } : { conflicts, success };
}

export function useMergeBranch(): ReactQuery.UseMutationResult<
  MergeBranchResult,
  Error,
  MergeBranchInput
> {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: mergeBranch,
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({
          queryKey: [queryKeys.versions, variables.sourceBranchId],
        }),
        queryClient.invalidateQueries({
          queryKey: [queryKeys.versions, variables.targetBranchId],
        }),
        queryClient.invalidateQueries({
          queryKey: [
            queryKeys.branchComparison,
            variables.sourceBranchId,
            variables.targetBranchId,
          ],
        }),
      ]);
    },
  });
}

export type VersionSnapshot = Record<string, JsonValue | undefined>;

// GET VERSION SNAPSHOT
async function getVersionSnapshot(versionId: string): Promise<VersionSnapshot> {
  const res = await fetch(`${API_URL}/api/v1/versions/${versionId}/snapshot`, {
    headers: getAuthHeaders(),
  });
  if (!res.ok) {
    throw new Error('Failed to fetch version snapshot');
  }
  const data = await parseJson(res);
  const snapshot = parseVersionSnapshot(data);
  if (!snapshot) {
    throw new Error('Invalid version snapshot payload');
  }
  return snapshot;
}

export function useVersionSnapshot(versionId: string): ReactQuery.UseQueryResult<VersionSnapshot> {
  return useQuery({
    queryKey: [queryKeys.versionSnapshot, versionId],
    queryFn: async () => getVersionSnapshot(versionId),
    ...QUERY_CONFIGS.default,
  });
}

// UPDATE BRANCH
interface UpdateBranchInput {
  branchId: string;
  name?: string;
  description?: string;
  status?: 'active' | 'review' | 'merged' | 'abandoned';
}

async function updateBranch(input: UpdateBranchInput): Promise<Branch> {
  const res = await fetch(`${API_URL}/api/v1/branches/${input.branchId}`, {
    body: JSON.stringify({
      description: input.description,
      name: input.name,
      status: input.status,
    }),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'PATCH',
  });
  if (!res.ok) {
    throw new Error('Failed to update branch');
  }
  const data = await parseJson(res);
  const branch = parseBranch(data);
  if (!branch) {
    throw new Error('Invalid branch payload');
  }
  return branch;
}

export function useUpdateBranch(): ReactQuery.UseMutationResult<Branch, Error, UpdateBranchInput> {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: updateBranch,
    onSuccess: async () =>
      queryClient.invalidateQueries({
        queryKey: [queryKeys.branches],
      }),
  });
}

// UPDATE VERSION
interface UpdateVersionInput {
  versionId: string;
  title?: string;
  description?: string;
  tag?: string;
  status?: 'draft' | 'published' | 'archived';
}

async function updateVersion(input: UpdateVersionInput): Promise<Version> {
  const res = await fetch(`${API_URL}/api/v1/versions/${input.versionId}`, {
    body: JSON.stringify({
      description: input.description,
      status: input.status,
      tag: input.tag,
      title: input.title,
    }),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'PATCH',
  });
  if (!res.ok) {
    throw new Error('Failed to update version');
  }
  const data = await parseJson(res);
  const version = parseVersion(data);
  if (!version) {
    throw new Error('Invalid version payload');
  }
  return version;
}

export function useUpdateVersion(): ReactQuery.UseMutationResult<
  Version,
  Error,
  UpdateVersionInput
> {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: updateVersion,
    onSuccess: async () =>
      queryClient.invalidateQueries({
        queryKey: [queryKeys.versions],
      }),
  });
}

// DELETE BRANCH
async function deleteBranch(branchId: string): Promise<void> {
  const res = await fetch(`${API_URL}/api/v1/branches/${branchId}`, {
    headers: getAuthHeaders(),
    method: 'DELETE',
  });
  if (!res.ok) {
    throw new Error('Failed to delete branch');
  }
}

export function useDeleteBranch(): ReactQuery.UseMutationResult<void, Error, string> {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: deleteBranch,
    onSuccess: async () =>
      queryClient.invalidateQueries({
        queryKey: [queryKeys.branches],
      }),
  });
}

// COMPARE BRANCHES
interface ComparisonResult {
  divergencePoint: Version | null;
  sourceVersions: Version[];
  targetVersions: Version[];
  commonVersions: Version[];
  conflicts: TemporalConflict[];
}

async function compareBranches(
  sourceBranchId: string,
  targetBranchId: string,
): Promise<ComparisonResult> {
  const res = await fetch(
    `${API_URL}/api/v1/branches/${sourceBranchId}/compare/${targetBranchId}`,
    { headers: getAuthHeaders() },
  );
  if (!res.ok) {
    throw new Error('Failed to compare branches');
  }
  const data = await parseJson(res);
  if (!isJsonObject(data)) {
    throw new Error('Invalid branch comparison payload');
  }

  const divergencePoint = parseVersion(data['divergencePoint']);
  const sourceVersions = Array.isArray(data['sourceVersions'])
    ? data['sourceVersions'].map(parseVersion).filter((version): version is Version => version !== null)
    : [];
  const targetVersions = Array.isArray(data['targetVersions'])
    ? data['targetVersions'].map(parseVersion).filter((version): version is Version => version !== null)
    : [];
  const commonVersions = Array.isArray(data['commonVersions'])
    ? data['commonVersions'].map(parseVersion).filter((version): version is Version => version !== null)
    : [];
  const conflicts = Array.isArray(data['conflicts'])
    ? data['conflicts'].map(parseConflict).filter((conflict): conflict is TemporalConflict => conflict !== null)
    : [];

  return {
    commonVersions,
    conflicts,
    divergencePoint,
    sourceVersions,
    targetVersions,
  };
}

export function useCompareBranches(
  sourceBranchId: string,
  targetBranchId: string,
): ReactQuery.UseQueryResult<ComparisonResult> {
  return useQuery({
    queryKey: [queryKeys.branchComparison, sourceBranchId, targetBranchId],
    queryFn: async () => compareBranches(sourceBranchId, targetBranchId),
    ...QUERY_CONFIGS.default,
  });
}
