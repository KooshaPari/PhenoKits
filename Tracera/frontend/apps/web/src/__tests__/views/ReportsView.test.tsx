import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { ReportsView } from '../../views/ReportsView';

// Mock the API
vi.mock('../../api/endpoints', () => ({
  api: {
    exportImport: {
      export: vi.fn(),
    },
    projects: {
      list: vi.fn(),
    },
  },
}));

describe(ReportsView, () => {
  let queryClient: QueryClient;

  beforeEach(async () => {
    queryClient = new QueryClient({
      defaultOptions: {
        mutations: { retry: false },
        queries: { retry: false },
      },
    });
    vi.clearAllMocks();

    const { api } = await import('../../api/endpoints');
    vi.mocked(api.projects.list).mockResolvedValue([]);
    vi.mocked(api.exportImport.export).mockResolvedValue(new Blob(['test'], { type: 'text/csv' }));
    vi.spyOn(globalThis.URL, 'createObjectURL').mockReturnValue('blob:report');
    vi.spyOn(globalThis.URL, 'revokeObjectURL').mockImplementation(() => {});
    vi.spyOn(HTMLAnchorElement.prototype, 'click').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders reports interface', () => {
    render(
      <QueryClientProvider client={queryClient}>
        <ReportsView />
      </QueryClientProvider>,
    );

    expect(screen.getByText('Intelligence Hub')).toBeInTheDocument();
    expect(screen.getByText('Traceability Matrix')).toBeInTheDocument();
    expect(screen.getByText('Executive Summary')).toBeInTheDocument();
    expect(screen.getByText('Entity Registry')).toBeInTheDocument();
    expect(screen.getByText('Compliance Audit')).toBeInTheDocument();
    expect(screen.getByText('Archive History')).toBeInTheDocument();
  });

  it('displays report templates', () => {
    render(
      <QueryClientProvider client={queryClient}>
        <ReportsView />
      </QueryClientProvider>,
    );

    expect(screen.getByText('Traceability Matrix')).toBeInTheDocument();
    expect(screen.getByText('End-to-end mapping from reqs to implementation.')).toBeInTheDocument();
    expect(screen.getByText('Executive Summary')).toBeInTheDocument();
    expect(screen.getByText('High-level project health and risk assessment.')).toBeInTheDocument();
  });

  it('displays format badges for each template', () => {
    render(
      <QueryClientProvider client={queryClient}>
        <ReportsView />
      </QueryClientProvider>,
    );

    expect(screen.getAllByText('pdf').length).toBeGreaterThan(0);
    expect(screen.getAllByText('xlsx').length).toBeGreaterThan(0);
    expect(screen.getAllByText('csv').length).toBeGreaterThan(0);
    expect(screen.getAllByText('json').length).toBeGreaterThan(0);
  });

  it('handles format selection', async () => {
    const user = userEvent.setup();
    render(
      <QueryClientProvider client={queryClient}>
        <ReportsView />
      </QueryClientProvider>,
    );

    const csvBadges = screen.getAllByText('csv');
    await user.click(csvBadges[0]);

    await waitFor(() => {
      expect(screen.getAllByText('csv').length).toBeGreaterThan(0);
    });
  });

  it('displays project selector', () => {
    render(
      <QueryClientProvider client={queryClient}>
        <ReportsView />
      </QueryClientProvider>,
    );

    expect(screen.getByText('System-Wide Registry')).toBeInTheDocument();
  });

  it('generates report when button is clicked', async () => {
    const user = userEvent.setup();
    const { api } = await import('../../api/endpoints');
    const mockBlob = new Blob(['test'], { type: 'text/csv' });
    (api.exportImport.export as any).mockResolvedValue(mockBlob);
    (api.projects.list as any).mockResolvedValue([{ id: 'proj-1', name: 'Test Project' }]);

    render(
      <QueryClientProvider client={queryClient}>
        <ReportsView />
      </QueryClientProvider>,
    );

    await user.click(screen.getByRole('combobox'));
    await user.click(await screen.findByRole('option', { name: 'Test Project' }));

    await user.click(screen.getAllByText('csv')[0]);

    await user.click(screen.getAllByRole('button', { name: /Compile Engine/i })[0]);

    await waitFor(() => {
      expect(api.exportImport.export).toHaveBeenCalledWith('proj-1', 'csv');
    });
  });

  it('displays recent reports section', () => {
    render(
      <QueryClientProvider client={queryClient}>
        <ReportsView />
      </QueryClientProvider>,
    );

    expect(screen.getByText('Archive History')).toBeInTheDocument();
    expect(screen.getByText('Full Integrity Matrix')).toBeInTheDocument();
    expect(screen.getByText('2h ago • PDF')).toBeInTheDocument();
  });

  it('shows loading state during report generation', async () => {
    const user = userEvent.setup();
    const { api } = await import('../../api/endpoints');
    (api.exportImport.export as any).mockImplementation(
      async () =>
        new Promise((resolve) =>
          setTimeout(() => {
            resolve(new Blob());
          }, 500),
        ),
    );
    (api.projects.list as any).mockResolvedValue([{ id: 'proj-1', name: 'Test Project' }]);

    render(
      <QueryClientProvider client={queryClient}>
        <ReportsView />
      </QueryClientProvider>,
    );

    await user.click(screen.getByRole('combobox'));
    await user.click(await screen.findByRole('option', { name: 'Test Project' }));

    await user.click(screen.getAllByText('pdf')[0]);

    const generateButton = screen.getAllByRole('button', { name: /Compile Engine/i })[0];
    await user.click(generateButton);

    await waitFor(() => {
      expect(generateButton).toBeDisabled();
    });
  });
});
