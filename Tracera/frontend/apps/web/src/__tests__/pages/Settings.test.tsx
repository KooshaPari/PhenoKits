import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { SettingsView } from '@/views/SettingsView';

const mockSaveSettings = vi.fn();
const mockUseProjects = vi.fn();

vi.mock('@/api/settings', () => ({
  saveSettings: (...args: unknown[]) => mockSaveSettings(...args),
}));

vi.mock('@/hooks/useProjects', () => ({
  useProjects: () => mockUseProjects(),
}));

vi.mock('@/pages/projects/views/IntegrationsView', () => ({
  IntegrationsView: () => <div>Integrations Surface</div>,
  default: () => <div>Integrations Surface</div>,
}));

const renderSettings = () => {
  const queryClient = new QueryClient({
    defaultOptions: { mutations: { retry: false }, queries: { retry: false } },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <SettingsView />
    </QueryClientProvider>,
  );
};

describe('SettingsView', () => {
  beforeEach(() => {
    mockSaveSettings.mockResolvedValue({});
    mockUseProjects.mockReturnValue({ data: [{ id: 'project-1', name: 'Alpha' }] });
  });

  it('renders the current settings navigation', () => {
    renderSettings();

    expect(screen.getByRole('heading', { name: /system preferences/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /identity/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /visuals/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /engine access/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /comms/i })).toBeInTheDocument();
  });

  it('edits identity fields and saves the current settings payload', async () => {
    const user = userEvent.setup();
    renderSettings();

    const displayName = screen.getByDisplayValue('System Administrator');
    await user.clear(displayName);
    await user.type(displayName, 'Trace Operator');
    await user.click(screen.getByRole('button', { name: /synchronize parameters/i }));

    expect(mockSaveSettings).toHaveBeenCalledWith(
      expect.objectContaining({ displayName: 'Trace Operator' }),
    );
  });

  it('switches to visuals and communication tabs', async () => {
    const user = userEvent.setup();
    renderSettings();

    await user.click(screen.getByRole('tab', { name: /visuals/i }));
    expect(screen.getByText('Interface Directives')).toBeInTheDocument();

    await user.click(screen.getByRole('tab', { name: /comms/i }));
    expect(screen.getByText('Telemetry & Comms')).toBeInTheDocument();
  });
});
