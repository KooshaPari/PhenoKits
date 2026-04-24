import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { ProjectDetailView } from '@/views/ProjectDetailView';

const mockUseProject = vi.fn();
const mockUseItems = vi.fn();
const mockUseLinks = vi.fn();

vi.mock('../../hooks/useProjects', () => ({
  useProject: () => mockUseProject(),
}));

vi.mock('../../hooks/useItems', () => ({
  useItems: () => mockUseItems(),
}));

vi.mock('../../hooks/useLinks', () => ({
  useLinks: () => mockUseLinks(),
}));

const project = {
  id: 'project-1',
  name: 'Trace Platform',
  description: 'Current traceability platform',
  createdAt: '2026-01-01T00:00:00Z',
  updatedAt: '2026-01-02T00:00:00Z',
};

const items = [
  {
    id: 'item-1',
    projectId: 'project-1',
    title: 'Requirement Alpha',
    type: 'requirement',
    status: 'done',
    priority: 'high',
    view: 'FEATURE',
    createdAt: '2026-01-01T00:00:00Z',
    updatedAt: '2026-01-02T00:00:00Z',
  },
  {
    id: 'item-2',
    projectId: 'project-1',
    title: 'Feature Beta',
    type: 'feature',
    status: 'todo',
    priority: 'medium',
    view: 'FEATURE',
    createdAt: '2026-01-01T00:00:00Z',
    updatedAt: '2026-01-02T00:00:00Z',
  },
];

describe('ProjectDetailView', () => {
  beforeEach(() => {
    mockUseProject.mockReturnValue({ data: project, error: null, isLoading: false });
    mockUseItems.mockReturnValue({ data: { data: items, total: items.length }, isLoading: false });
    mockUseLinks.mockReturnValue({ data: { data: [{ id: 'link-1' }], total: 1 }, isLoading: false });
  });

  it('renders the current project detail overview', () => {
    render(<ProjectDetailView />);

    expect(screen.getByRole('heading', { name: /trace platform/i })).toBeInTheDocument();
    expect(screen.getByText('Current traceability platform')).toBeInTheDocument();
    expect(screen.getByText(/project health/i)).toBeInTheDocument();
  });

  it('shows recent item activity and view navigation', () => {
    render(<ProjectDetailView />);

    expect(screen.getByRole('heading', { name: /recent item activity/i })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: /views/i })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: /features/i })).toBeInTheDocument();
  });

  it('toggles agent workflows panel from the overview action', async () => {
    const user = userEvent.setup();
    render(<ProjectDetailView />);

    await user.click(screen.getByRole('button', { name: /^show$/i }));

    expect(screen.getByRole('button', { name: /^hide$/i })).toBeInTheDocument();
  });

  it('renders not-found state when project data is unavailable', () => {
    mockUseProject.mockReturnValue({ data: null, error: new Error('missing'), isLoading: false });

    render(<ProjectDetailView />);

    expect(screen.getByText(/project not found/i)).toBeInTheDocument();
  });
});
