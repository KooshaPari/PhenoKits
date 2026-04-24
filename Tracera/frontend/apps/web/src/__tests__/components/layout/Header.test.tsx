/**
 * Comprehensive Tests for Header Component
 */

import { beforeEach, describe, expect, it, vi } from 'vitest';

// Mock the router hooks BEFORE any imports that use them
vi.mock('@tanstack/react-router', () => ({
  useLocation: () => ({ pathname: '/' }),
  useMatches: () => [],
  useNavigate: () => vi.fn(),
  useParams: () => ({}),
  useRouter: () => ({ navigate: vi.fn() }),
}));

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen } from '@testing-library/react';
import type { ReactNode } from 'react';

import { Header } from '../../../components/layout/Header';
import { ThemeProvider } from '../../../providers/theme-provider';

describe(Header, () => {
  const renderHeader = () => {
    const queryClient = new QueryClient({
      defaultOptions: {
        queries: {
          retry: false,
        },
      },
    });

    const Wrapper = ({ children }: { children: ReactNode }) => (
      <QueryClientProvider client={queryClient}>
        <ThemeProvider>{children}</ThemeProvider>
      </QueryClientProvider>
    );

    return render(<Header />, { wrapper: Wrapper });
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders header with title', () => {
    renderHeader();
    expect(screen.getByText('Dashboard')).toBeInTheDocument();
  });

  it('renders the notification count', () => {
    renderHeader();
    expect(screen.getByText('0')).toBeInTheDocument();
  });

  it('displays sign-in action for anonymous users', () => {
    renderHeader();
    expect(screen.getByText('Sign in')).toBeInTheDocument();
  });

  it('renders accessible header actions', async () => {
    renderHeader();

    const themeButtons = screen.getAllByRole('button');
    expect(themeButtons.length).toBeGreaterThan(0);
  });
});
