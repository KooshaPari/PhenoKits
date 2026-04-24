import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import type { Link, LinkType } from '@tracertm/types';

import { client } from '@/api/client';
import { QUERY_CONFIGS, queryKeys } from '@/lib/queryConfig';
import { useAuthStore } from '@/stores/authStore';

const { getAuthHeaders } = client;

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:4000';
const LINK_TYPES = new Set<string>([
  'alternative_to',
  'blocks',
  'calls',
  'conflicts_with',
  'depends_on',
  'derives_from',
  'documents',
  'imports',
  'implements',
  'manifests_as',
  'mentions',
  'parent_of',
  'related_to',
  'represents',
  'same_as',
  'supersedes',
  'tests',
  'traces_to',
  'validates',
]);

interface LinkFilters {
  projectId?: string | undefined;
  sourceId?: string | undefined;
  targetId?: string | undefined;
  type?: LinkType | undefined;
  limit?: number | undefined;
  offset?: number | undefined;
  excludeTypes?: LinkType[] | undefined; // ✅ NEW: Filter out specific link types
}

interface ItemSummary {
  id: string;
  status: string;
  title: string;
  view: string;
}

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null;

const asString = (value: unknown, fallback = ''): string =>
  typeof value === 'string' ? value : fallback;

const asNumber = (value: unknown, fallback = 0): number =>
  typeof value === 'number' ? value : fallback;

const isLinkType = (value: unknown): value is LinkType =>
  typeof value === 'string' && LINK_TYPES.has(value);

const parseLink = (value: unknown): Link | null => {
  if (!isRecord(value)) {
    return null;
  }

  const sourceId = value['source_id'] ?? value['sourceId'];
  const targetId = value['target_id'] ?? value['targetId'];
  const projectId = value['project_id'] ?? value['projectId'];
  const type = value['type'];

  if (!isLinkType(type)) {
    return null;
  }

  return {
    ...value,
    createdAt: asString(value['createdAt'] ?? value['created_at']),
    description:
      typeof value['description'] === 'string' ? value['description'] : undefined,
    id: asString(value['id']),
    metadata: isRecord(value['metadata']) ? value['metadata'] : undefined,
    projectId: asString(projectId),
    sourceId: asString(sourceId),
    targetId: asString(targetId),
    type,
    updatedAt: asString(value['updatedAt'] ?? value['updated_at']),
    version: asNumber(value['version'], 1),
  };
};

const parseItemSummary = (value: unknown): ItemSummary | null => {
  if (!isRecord(value)) {
    return null;
  }

  return {
    id: asString(value['id']),
    status: asString(value['status']),
    title: asString(value['title']),
    view: asString(value['view']),
  };
};

async function fetchLinks(filters: LinkFilters = {}): Promise<{ links: Link[]; total: number }> {
  const params = new URLSearchParams();
  if (filters.projectId !== undefined && filters.projectId !== '') {
    params.set('project_id', filters.projectId);
  }
  if (filters.sourceId !== undefined && filters.sourceId !== '') {
    params.set('source_id', filters.sourceId);
  }
  if (filters.targetId !== undefined && filters.targetId !== '') {
    params.set('target_id', filters.targetId);
  }
  if (filters.type !== undefined) {
    params.set('type', filters.type);
  }
  if (filters.limit !== undefined) {
    params.set('limit', String(filters.limit));
  }
  if (filters.offset !== undefined) {
    params.set('offset', String(filters.offset));
  }

  // ✅ NEW: Send excluded types to API for server-side filtering
  if (filters.excludeTypes?.length) {
    params.set('exclude_types', filters.excludeTypes.join(','));
  }

  const res = await fetch(`${API_URL}/api/v1/links?${params}`, {
    headers: {
      'X-Bulk-Operation': 'true',
      ...getAuthHeaders(),
    },
  });
  if (!res.ok) {
    throw new Error('Failed to fetch links');
  }
  const data: unknown = await res.json();
  // API returns { total: number, links: Link[] } or array
  const linksArray = Array.isArray(data)
    ? data
    : isRecord(data) && Array.isArray(data['links'])
      ? data['links']
      : [];
  // Transform snake_case to camelCase for frontend compatibility
  const transformedLinks = linksArray
    .map((link) => parseLink(link))
    .filter((link): link is Link => link !== null);
  const total = isRecord(data) && typeof data['total'] === 'number' ? data['total'] : linksArray.length;
  return {
    links: transformedLinks,
    total,
  };
}

interface CreateLinkData {
  projectId: string;
  sourceId: string;
  targetId: string;
  type: LinkType;
  description?: string;
}

async function createLink(data: CreateLinkData): Promise<Link> {
  const res = await fetch(`${API_URL}/api/v1/links`, {
    body: JSON.stringify({
      description: data['description'],
      project_id: data['projectId'],
      source_id: data['sourceId'],
      target_id: data['targetId'],
      type: data.type,
    }),
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    method: 'POST',
  });
  if (!res.ok) {
    throw new Error('Failed to create link');
  }
  const responseData: unknown = await res.json();
  const link = parseLink(responseData);
  if (link === null) {
    throw new Error('Invalid link response');
  }
  return link;
}

async function deleteLink(id: string): Promise<void> {
  const res = await fetch(`${API_URL}/api/v1/links/${id}`, {
    headers: getAuthHeaders(),
    method: 'DELETE',
  });
  if (!res.ok) {
    throw new Error('Failed to delete link');
  }
}

export function useLinks(filters: LinkFilters = {}) {
  const token = useAuthStore((s) => s.token);
  const key = filters.projectId !== undefined && filters.projectId !== ''
    ? [
        ...queryKeys.links.list(filters.projectId),
        filters.sourceId ?? null,
        filters.targetId ?? null,
        filters.type ?? null,
        filters.limit ?? null,
        filters.excludeTypes ?? null, // ✅ NEW: Include in cache key
      ]
    : [
        'links',
        filters.sourceId ?? null,
        filters.targetId ?? null,
        filters.type ?? null,
        filters.limit ?? null,
        filters.excludeTypes ?? null, // ✅ NEW: Include in cache key
      ];
  return useQuery({
    queryKey: key,
    queryFn: async () => fetchLinks(filters),
    enabled: Boolean(token),
    ...QUERY_CONFIGS.dynamic, // Links change frequently
  });
}

export function useCreateLink() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: createLink,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['links'] });
    },
  });
}

export function useDeleteLink() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: deleteLink,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['links'] });
    },
  });
}

// Graph data hook for visualization
export function useTraceabilityGraph(projectId: string) {
  // ✅ NEW: Limit initial edge rendering
  const MAX_EDGES_INITIAL = 500;
  const [visibleEdgeCount, setVisibleEdgeCount] = useState(MAX_EDGES_INITIAL);

  const { data: items } = useQuery<ItemSummary[]>({
    queryKey: queryKeys.items.list(projectId),
    queryFn: async () => {
      const res = await fetch(`${API_URL}/api/v1/items?project_id=${projectId}`, {
        headers: getAuthHeaders(),
      });
      if (!res.ok) {
        throw new Error('Failed to fetch items');
      }
      const responseData: unknown = await res.json();
      if (!Array.isArray(responseData)) {
        return [];
      }
      return responseData
        .map((item) => parseItemSummary(item))
        .filter((item): item is ItemSummary => item !== null);
    },
    enabled: Boolean(projectId),
    ...QUERY_CONFIGS.graph, // Graph data is expensive, cache longer
  });

  // ✅ FIXED: Fetch links with filtering + reasonable limit
  const { data: linksData } = useLinks({
    projectId,
    limit: 10_000, // ✅ NEW: API limit to prevent massive responses
    excludeTypes: ['implements'], // ✅ NEW: Filter out 84% redundant links
  });

  const allLinks = linksData?.links ?? [];

  // ✅ NEW: Progressive loading - only render first N edges
  const visibleLinks = allLinks.slice(0, visibleEdgeCount);
  const canLoadMore = visibleEdgeCount < allLinks.length;

  // Load more handler
  const onLoadMore = () => {
    setVisibleEdgeCount((prev) => Math.min(prev + 500, allLinks.length));
  };

  return {
    canLoadMore,
    edges: visibleLinks.map((link) => ({
      data: {
        id: link.id,
        source: link.sourceId,
        target: link.targetId,
        type: link.type,
      },
    })),
    isLoading: items === undefined || linksData === undefined,
    nodes: (items ?? []).map((item) => ({
      data: {
        id: item.id,
        label: item.title,
        status: item.status,
        type: item.view.toLowerCase(),
      },
    })),
    onLoadMore,
    totalEdges: allLinks.length,
    visibleEdges: visibleLinks.length,
  };
}
