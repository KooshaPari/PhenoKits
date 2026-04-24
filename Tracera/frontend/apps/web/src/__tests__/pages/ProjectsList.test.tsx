import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type React from 'react';

import { ProjectsListView } from '@/views/ProjectsListView';

const mockUseProjects = vi.fn();

vi.mock('@tanstack/react-router', async () => {
  const actual =
    await vi.importActual<typeof import('@tanstack/react-router')>('@tanstack/react-router');

  return {
    ...actual,
    Link: ({
      children,
      to,
      ...props
    }: {
      children: React.ReactNode;
      to: string;
    } & React.AnchorHTMLAttributes<HTMLAnchorElement>) => (
      <a href={to} {...props}>
        {children}
      </a>
    ),
    useSearch: () => ({}),
  };
});

vi.mock('@/hooks/useProjects', () => ({
  useDeleteProject: () => ({
    mutate: vi.fn(),
  }),
  useProjects: () => mockUseProjects(),
}));

vi.mock('@/views/projects-list/CreateProjectDialog', () => ({
  CreateProjectDialog: ({ open }: { open: boolean }) =>
    open ? (
      <div role='dialog' aria-label='Create Project'>
        Create Project
      </div>
    ) : null,
}));

vi.mock('@/views/projects-list/EditProjectDialog', () => ({
  EditProjectDialog: ({ open }: { open: boolean }) =>
    open ? <div role='dialog'>Edit Project</div> : null,
}));

vi.mock('@/views/projects-list/ExportDialog', () => ({
  ExportDialog: ({ open }: { open: boolean }) =>
    open ? <div role='dialog'>Export Registry</div> : null,
}));

vi.mock('@/views/projects-list/ImportDialog', () => ({
  ImportDialog: ({ open }: { open: boolean }) =>
    open ? <div role='dialog'>Import Registry</div> : null,
}));

vi.mock('@/views/projects-list/ProjectDeleteDialog', () => ({
  ProjectDeleteDialog: ({ open }: { open: boolean }) =>
    open ? <div role='dialog'>Delete Project</div> : null,
}));

const projects = [
  {
    id: 'project-alpha',
    name: 'Project Alpha',
    description: 'First project',
    createdAt: '2026-01-01T00:00:00Z',
    updatedAt: '2026-01-02T00:00:00Z',
  },
  {
    id: 'project-beta',
    name: 'Project Beta',
    description: 'Second project',
    createdAt: '2026-01-03T00:00:00Z',
    updatedAt: '2026-01-04T00:00:00Z',
  },
];

const renderProjectsList = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      mutations: { retry: false },
      queries: { retry: false },
    },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <ProjectsListView />
    </QueryClientProvider>,
  );
};

describe('ProjectsListView', () => {
  beforeEach(() => {
    mockUseProjects.mockReturnValue({ data: projects, isLoading: false });
  });

  it('renders the current project registry shell and project cards', () => {
    renderProjectsList();

    expect(screen.getByRole('heading', { name: /project registry/i })).toBeInTheDocument();
    expect(screen.getByText('Project Alpha')).toBeInTheDocument();
    expect(screen.getByText('Project Beta')).toBeInTheDocument();
    expect(screen.getByText('First project')).toBeInTheDocument();
  });

  it('filters visible projects by search query', async () => {
    const user = userEvent.setup();
    renderProjectsList();

    await user.type(screen.getByPlaceholderText(/filter registries/i), 'beta');

    expect(screen.getByText('Project Beta')).toBeInTheDocument();
    expect(screen.queryByText('Project Alpha')).not.toBeInTheDocument();
  });

  it('opens the create project dialog from the header action', async () => {
    const user = userEvent.setup();
    renderProjectsList();

    await user.click(screen.getByRole('button', { name: /new registry/i }));

    expect(screen.getByRole('dialog', { name: /create project/i })).toBeInTheDocument();
  });

  it('renders the empty state for an empty registry', () => {
    mockUseProjects.mockReturnValue({ data: [], isLoading: false });

    renderProjectsList();

    expect(screen.getByText(/registry vacant/i)).toBeInTheDocument();
  });
});
