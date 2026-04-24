/**
 * Tests for useProjects hook
 */

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { act, renderHook, waitFor } from '@testing-library/react';
import React from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { useCreateProject, useProject, useProjects } from '../../hooks/useProjects';
import { useAuthStore } from '@/stores/authStore';

// Mock fetch (vi.fn() compatible with fetch at runtime)
const mockFetch = vi.fn();
globalThis.fetch = mockFetch as unknown as typeof fetch;
const TEST_TOKEN = 'test-token';

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      mutations: { retry: false },
      queries: { retry: false },
    },
  });

  return ({ children }: { children: React.ReactNode }) =>
    React.createElement(QueryClientProvider, { client: queryClient }, children);
};

describe(useProjects, () => {
  beforeEach(() => {
    globalThis.fetch = mockFetch as unknown as typeof fetch;
    vi.clearAllMocks();
    act(() => {
      useAuthStore.setState({ token: TEST_TOKEN } as any);
    });
  });

  afterEach(() => {
    act(() => {
      useAuthStore.setState({ token: null } as any);
    });
  });

  it('should fetch projects', async () => {
    const mockProjects = [
      { description: 'Desc 1', id: '1', name: 'Project 1' },
      { description: 'Desc 2', id: '2', name: 'Project 2' },
    ];

    mockFetch.mockResolvedValueOnce({
      json: async () => mockProjects,
      ok: true,
    });

    const { result } = renderHook(() => useProjects(), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBeTruthy();
    });

    expect(result.current.data).toEqual(mockProjects);
    expect(mockFetch).toHaveBeenCalledOnce();
    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/v1/projects'),
      expect.objectContaining({
        credentials: 'include',
        headers: expect.objectContaining({
          Authorization: `Bearer ${TEST_TOKEN}`,
        }),
      }),
    );
  });

  it('should handle fetch error', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 500,
    });

    const { result } = renderHook(() => useProjects(), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isError).toBeTruthy();
    });

    expect(result.current.error).toBeTruthy();
  });
});

describe(useProject, () => {
  beforeEach(() => {
    globalThis.fetch = mockFetch as unknown as typeof fetch;
    vi.clearAllMocks();
    act(() => {
      useAuthStore.setState({ token: TEST_TOKEN } as any);
    });
  });

  afterEach(() => {
    act(() => {
      useAuthStore.setState({ token: null } as any);
    });
  });

  it('should fetch a single project', async () => {
    const mockProject = {
      description: 'Description',
      id: '1',
      name: 'Project 1',
    };

    mockFetch.mockResolvedValueOnce({
      json: async () => mockProject,
      ok: true,
    });

    const { result } = renderHook(() => useProject('1'), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBeTruthy();
    });

    expect(result.current.data).toEqual(mockProject);
    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/v1/projects/1'),
      expect.objectContaining({
        headers: expect.objectContaining({
          Authorization: `Bearer ${TEST_TOKEN}`,
          'X-Bulk-Operation': 'true',
        }),
      }),
    );
  });

  it('should not fetch when id is empty', () => {
    const { result } = renderHook(() => useProject(''), {
      wrapper: createWrapper(),
    });

    expect(result.current.fetchStatus).toBe('idle');
    expect(mockFetch).not.toHaveBeenCalled();
  });
});

describe(useCreateProject, () => {
  beforeEach(() => {
    globalThis.fetch = mockFetch as unknown as typeof fetch;
    vi.clearAllMocks();
    act(() => {
      useAuthStore.setState({ token: TEST_TOKEN } as any);
    });
  });

  afterEach(() => {
    act(() => {
      useAuthStore.setState({ token: null } as any);
    });
  });

  it('should create a project', async () => {
    const newProject = {
      description: 'New Description',
      name: 'New Project',
    };

    const createdProject = {
      id: '1',
      ...newProject,
    };

    mockFetch.mockResolvedValueOnce({
      json: async () => createdProject,
      ok: true,
    });

    const { result } = renderHook(() => useCreateProject(), {
      wrapper: createWrapper(),
    });

    result.current.mutate(newProject);

    await waitFor(() => {
      expect(result.current.isSuccess).toBeTruthy();
    });

    expect(result.current.data).toEqual(createdProject);
    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/v1/projects'),
      expect.objectContaining({
        method: 'POST',
      }),
    );
  });

  it('should handle create error', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 400,
    });

    const { result } = renderHook(() => useCreateProject(), {
      wrapper: createWrapper(),
    });

    result.current.mutate({
      description: 'Description',
      name: 'New Project',
    });

    await waitFor(() => {
      expect(result.current.isError).toBeTruthy();
    });
  });
});
