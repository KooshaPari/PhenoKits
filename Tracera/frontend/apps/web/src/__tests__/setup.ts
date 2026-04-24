/**
 * Test setup and configuration
 */

import { cleanup } from '@testing-library/react';
import '@testing-library/jest-dom';
import { createRequire } from 'node:module';
import React from 'react';
import { afterEach, afterAll, beforeAll, beforeEach, vi } from 'vitest';

type TestGlobals = typeof globalThis & {
  WebGL2RenderingContext?: unknown;
  IntersectionObserver?: new (...args: unknown[]) => unknown;
  ResizeObserver?: new (...args: unknown[]) => unknown;
  WebSocket?: new (url: string) => unknown;
  HTMLCanvasElement?: new (...args: unknown[]) => unknown;
  __fetchMock__?: typeof fetch;
  __setFetchImpl__?: (impl: typeof fetch) => void;
  user?: ReturnType<typeof import('@testing-library/user-event').default.setup>;
};

const require = createRequire(import.meta.url);
const userEventModule = require('@testing-library/user-event') as {
  default?: { setup: typeof import('@testing-library/user-event').default.setup } | undefined;
  setup?: typeof import('@testing-library/user-event').default.setup | undefined;
};
const userEvent = userEventModule.default ?? userEventModule;

// Mock WebGL2RenderingContext FIRST before any imports
if (typeof globalThis !== 'undefined') {
  const WebGL2RenderingContextMock = {
    BOOL: 35_670,
    BYTE: 5120,
    FLOAT: 5126,
    INT: 5124,
    SHORT: 5122,
    UNSIGNED_BYTE: 5121,
    UNSIGNED_INT: 5125,
    UNSIGNED_SHORT: 5123,
  };
  Object.defineProperty(globalThis, 'WebGL2RenderingContext', {
    configurable: true,
    value: WebGL2RenderingContextMock as unknown as typeof WebGL2RenderingContext,
    writable: true,
  });
}

// Mock TanStack Router API routes (createAPIFileRoute is from TanStack Start but imported from react-router)
vi.mock('@tanstack/react-router', async () => {
  const actual = await vi.importActual('@tanstack/react-router');
  return {
    ...actual,
    createAPIFileRoute: () => () => ({ GET: vi.fn(), POST: vi.fn() }),
    useNavigate: () => vi.fn(),
    useRouter: () => ({
      navigate: vi.fn(),
    }),
    useLocation: () => ({ pathname: '/' }),
    useMatches: () => [],
    useParams: () => ({}),
    Link: ({ children, to, ...props }: any) =>
      React.createElement(
        'a',
        {
          href: typeof to === 'string' ? to : to?.toString?.(),
          ...props,
        },
        children,
      ),
  };
});

// Mock elkjs to avoid worker initialization issues in tests
vi.mock('elkjs', () => ({
  default: class MockELK {
    async layout() {
      return { children: [], edges: [] };
    }
  },
}));

// Already defined at top of file

// Mock sigma.js to avoid WebGL initialization issues
vi.mock('sigma', () => ({
  default: class MockSigma {
    on = vi.fn();
    off = vi.fn();
    kill = vi.fn();
    getGraph = vi.fn(() => ({
      edges: vi.fn(() => []),
      nodes: vi.fn(() => []),
    }));
  },
}));

// Setup localStorage mock BEFORE importing MSW
const localStorageMock: Storage = (() => {
  let store: Record<string, string> = {};
  return {
    clear() {
      store = {};
    },
    getItem(key: string) {
      return store[key] ?? null;
    },
    key(index: number) {
      const keys = Object.keys(store);
      return keys[index] ?? null;
    },
    get length() {
      return Object.keys(store).length;
    },
    removeItem(key: string) {
      delete store[key];
    },
    setItem(key: string, value: string) {
      store[key] = value.toString();
    },
  };
})();

Object.defineProperty(globalThis, 'localStorage', {
  configurable: true,
  value: localStorageMock,
  writable: true,
});

beforeEach(() => {
  if (typeof globalThis.window !== 'undefined' && !globalThis.window.navigator) {
    Object.defineProperty(globalThis.window, 'navigator', {
      configurable: true,
      value: {},
      writable: true,
    });
  }
  if (typeof globalThis.window !== 'undefined' && !globalThis.window.navigator.clipboard) {
    Object.defineProperty(globalThis.window.navigator, 'clipboard', {
      configurable: true,
      value: {
        readText: vi.fn(async () => ''),
        writeText: vi.fn(async () => {}),
      },
      writable: true,
    });
  }
  (globalThis as TestGlobals).user = userEvent.setup!();
});

// Cleanup after each test
afterEach(async () => {
  cleanup();
  await clearAllStores();
  globalFetchImpl = defaultFetchImpl;
  vi.mocked(fetchMock).mockClear();
  vi.clearAllMocks();
});

// Mock window.matchMedia
if (typeof globalThis.window !== 'undefined') {
  Object.defineProperty(globalThis.window, 'matchMedia', {
    value: vi.fn().mockImplementation((query) => ({
      addEventListener: vi.fn(),
      addListener: vi.fn(),
      dispatchEvent: vi.fn(),
      matches: false,
      media: query,
      onchange: null,
      removeEventListener: vi.fn(),
      removeListener: vi.fn(),
    })),
    writable: true,
  });
}

// Mock navigator.clipboard
if (typeof navigator !== 'undefined') {
  Object.defineProperty(navigator, 'clipboard', {
    configurable: true,
    value: {
      readText: vi.fn(async () => ''),
      writeText: vi.fn(async () => {}),
    },
    writable: true,
  });
}
// Mock IntersectionObserver
const IntersectionObserverMock = class {
  disconnect() {}
  observe() {}
  takeRecords() {
    return [];
  }
  unobserve() {}
};
Object.defineProperty(globalThis, 'IntersectionObserver', {
  configurable: true,
  value: IntersectionObserverMock as unknown as typeof IntersectionObserver,
  writable: true,
});

// Mock ResizeObserver
const ResizeObserverMock = class {
  disconnect() {}
  observe() {}
  unobserve() {}
};
Object.defineProperty(globalThis, 'ResizeObserver', {
  configurable: true,
  value: ResizeObserverMock as unknown as typeof ResizeObserver,
  writable: true,
});

// Mock pointer capture methods for Radix UI components
if (typeof globalThis !== 'undefined' && typeof Element !== 'undefined') {
  Element.prototype.hasPointerCapture = vi.fn().mockReturnValue(false);
  Element.prototype.setPointerCapture = vi.fn();
  Element.prototype.releasePointerCapture = vi.fn();
}

// Mock scrollIntoView for Radix UI components
if (typeof globalThis !== 'undefined' && typeof Element !== 'undefined') {
  Element.prototype.scrollIntoView = vi.fn();
}

// Mock WebSocket
class MockWebSocket {
  static readonly CONNECTING = 0;
  static readonly OPEN = 1;
  static readonly CLOSING = 2;
  static readonly CLOSED = 3;

  url: string;
  readyState: number = MockWebSocket.CONNECTING;
  onopen: ((event: Event) => void) | null = null;
  onclose: ((event: CloseEvent) => void) | null = null;
  onmessage: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: Event) => void) | null = null;

  constructor(url: string) {
    this.url = url;
    setTimeout(() => {
      this.readyState = MockWebSocket.OPEN;
      if (this.onopen) {
        this.onopen(new Event('open'));
      }
    }, 0);
  }

  send(_data: string) {}
  close() {
    this.readyState = MockWebSocket.CLOSED;
    if (this.onclose) {
      this.onclose(new CloseEvent('close'));
    }
  }
  addEventListener(_type: string, _listener: EventListener) {}
  removeEventListener(_type: string, _listener: EventListener) {}
  dispatchEvent(_event: Event) {
    return true;
  }
}

Object.defineProperty(globalThis, 'WebSocket', {
  configurable: true,
  value: MockWebSocket as unknown as typeof WebSocket,
  writable: true,
});

// Mock HTMLCanvasElement for graph visualization
if (typeof globalThis !== 'undefined') {
  const MockCanvas = class {
    width = 300;
    height = 150;

    getContext(_type: string) {
      return {
        arc: vi.fn(),
        beginPath: vi.fn(),
        clearRect: vi.fn(),
        clip: vi.fn(),
        closePath: vi.fn(),
        createImageData: vi.fn(() => ({ data: Array.from({ length: 4 }) })),
        createLinearGradient: vi.fn(() => ({ addColorStop: vi.fn() })),
        createRadialGradient: vi.fn(() => ({ addColorStop: vi.fn() })),
        drawImage: vi.fn(),
        fill: vi.fn(),
        fillRect: vi.fn(),
        fillText: vi.fn(),
        getImageData: vi.fn(() => ({ data: Array.from({ length: 4 }) })),
        lineTo: vi.fn(),
        measureText: vi.fn(() => ({ width: 0 })),
        moveTo: vi.fn(),
        putImageData: vi.fn(),
        rect: vi.fn(),
        restore: vi.fn(),
        rotate: vi.fn(),
        save: vi.fn(),
        scale: vi.fn(),
        setTransform: vi.fn(),
        stroke: vi.fn(),
        transform: vi.fn(),
        translate: vi.fn(),
      };
    }

    toDataURL() {
      return 'data:image/png;base64,iVBORw0KGgo=';
    }

    toBlob(callback: BlobCallback) {
      callback(new Blob());
    }
  };
  Object.defineProperty(globalThis, 'HTMLCanvasElement', {
    configurable: true,
    value: MockCanvas as unknown as typeof HTMLCanvasElement,
    writable: true,
  });
}

// Mock fetch globally for API tests
// Use a delegating mock so tests can override it in beforeEach
const defaultFetchImpl: typeof fetch = async (url) => {
  console.warn(`[WARN] Unmocked fetch to ${url}`);
  return Response.json(
    { error: 'Not mocked' },
    {
      headers: { 'Content-Type': 'application/json' },
      status: 404,
    },
  );
};

let globalFetchImpl: typeof fetch = defaultFetchImpl;

const fetchMock = vi.fn(async (url: string | URL | Request, options?: RequestInit) =>
  globalFetchImpl(url, options),
) as typeof fetch;

Object.defineProperty(globalThis, 'fetch', {
  configurable: true,
  get: () => fetchMock,
  set: (impl: typeof fetch) => {
    globalFetchImpl = impl;
  },
});

if (typeof globalThis.window !== 'undefined') {
  Object.defineProperty(globalThis.window, 'fetch', {
    configurable: true,
    get: () => fetchMock,
    set: (impl: typeof fetch) => {
      globalFetchImpl = impl;
    },
  });
}

// Export so tests can replace the implementation
(globalThis as TestGlobals).__fetchMock__ = fetchMock;
(globalThis as TestGlobals).__setFetchImpl__ = (impl: typeof fetch) => {
  globalFetchImpl = impl;
};

import type { RenderOptions } from '@testing-library/react';

import { render as rtlRender } from '@testing-library/react';
// Add React testing utilities wrapper for provider-based tests

// Create test wrapper with all necessary providers
const AllTheProviders = ({ children }: { children: React.ReactNode }) =>
  React.createElement(React.Fragment, null, children);

// Custom render function that wraps components with providers
export const render = (ui: React.ReactElement, options?: Omit<RenderOptions, 'wrapper'>) =>
  rtlRender(ui, { wrapper: AllTheProviders, ...options });

// Re-export everything from testing library
export * from '@testing-library/react';

// ============================================================================
// MSW Server Setup
// ============================================================================

import { waitFor } from '@testing-library/react';
// MSW TEMPORARILY DISABLED DUE TO GRAPHQL ESM/COMMONJS IMPORT ISSUE
// See: CRITICAL_BLOCKER_MSW_GRAPHQL.md
// tracked: https://github.com/KooshaPari/trace/issues/224
// Start MSW server before all tests
// BeforeAll(() => {
//   Try {
//     Const server = getServer();
//     Server.listen();
//   } catch (error) {
//     Console.warn('MSW server initialization failed:', error);
//     // Continue anyway - tests that don't need HTTP mocking will still work
//   }
// });
// Stop MSW server after all tests
// AfterAll(() => {
//   Try {
//     Const server = getServer();
//     Server.close();
//   } catch (error) {
//     // Ignore cleanup errors
//   }
// });
// Reset handlers after each test
// AfterEach(() => {
//   Try {
//     Const server = getServer();
//     Server.resetHandlers();
//   } catch (error) {
//     // Ignore reset errors
//   }
// });
// ============================================================================
// Async Test Helpers
// ============================================================================

import { getServer } from './mocks/server';

/**
 * Wait for loading state to appear and then disappear
 * Useful for async operations that show loading UI
 */
export const waitForLoadingState = async (container: HTMLElement, timeout: number = 3000) => {
  // Wait for loading indicator to appear
  await waitFor(
    () => {
      const loader = container.querySelector('[data-testid="loading"]');
      if (!loader) {
        throw new Error('Loading indicator not found');
      }
    },
    { timeout: 500 },
  ).catch(() => {
    // Some tests may not have a loading indicator
  });

  // Wait for loading indicator to disappear
  await waitFor(
    () => {
      const loader = container.querySelector('[data-testid="loading"]');
      if (loader) {
        throw new Error('Loading indicator still visible');
      }
    },
    { timeout },
  );
};

/**
 * Wait for an element with text content to appear
 */
export const waitForElementWithText = async (
  container: HTMLElement,
  text: string,
  timeout: number = 3000,
) => {
  let element: HTMLElement | null = null;
  await waitFor(
    () => {
      element = [...container.querySelectorAll('*')].find((el) => el.textContent?.includes(text)) as
        | HTMLElement
        | undefined;
      if (!element) {
        throw new Error(`Element with text "${text}" not found`);
      }
    },
    { timeout },
  );
  return element;
};

/**
 * Clear all stores and caches for a clean test state
 * Includes: zustand stores, React Query cache, localStorage
 */
export const clearAllStores = async () => {
  const [
    { useAuthStore },
    { useChatStore },
    { useItemsStore },
    { useProjectStore },
    { useSyncStore },
    { useUIStore },
    { useWebSocketStore },
  ] = await Promise.all([
    import('../stores/auth-store'),
    import('../stores/chat-store'),
    import('../stores/items-store'),
    import('../stores/project-store'),
    import('../stores/sync-store'),
    import('../stores/ui-store'),
    import('../stores/websocket-store'),
  ]);

  useAuthStore.getState().stopAutoRefresh();
  useWebSocketStore.getState().disconnect();

  if (typeof localStorage !== 'undefined') {
    localStorage.clear();
  }
  if (typeof sessionStorage !== 'undefined') {
    sessionStorage.clear();
  }
  for (const key of [
    'auth_token',
    'tracertm-auth-store',
    'tracertm-chat-store',
    'tracertm-project-store',
    'tracertm-ui-store',
  ]) {
    localStorageMock.removeItem(key);
  }

  if (typeof window !== 'undefined') {
    (window as any).__REACT_QUERY_CACHE__ = undefined;
    window.document.documentElement.classList.remove('dark');
  }

  useAuthStore.persist.clearStorage();
  useProjectStore.persist.clearStorage();
  useUIStore.persist.clearStorage();
  useChatStore.persist.clearStorage();

  useAuthStore.setState({
    account: null,
    authKitRefreshToken: null,
    isAuthenticated: false,
    isLoading: false,
    refreshTimer: null,
    token: null,
    user: null,
  });
  useItemsStore.setState({
    isLoading: false,
    items: new Map(),
    itemsByProject: new Map(),
    loadingItems: new Set(),
    pendingCreates: new Map(),
    pendingDeletes: new Set(),
    pendingUpdates: new Map(),
  });
  useProjectStore.setState({
    currentProject: null,
    currentProjectId: null,
    projectSettings: {},
    recentProjects: [],
  });
  useSyncStore.setState({
    conflicts: [],
    failedMutations: [],
    isOnline: typeof navigator !== 'undefined' ? navigator.onLine : true,
    isSyncing: false,
    lastSyncedAt: null,
    pendingMutations: [],
    syncError: null,
  });
  useUIStore.setState({
    commandPaletteOpen: false,
    currentView: 'FEATURE',
    gridColumns: 3,
    isDarkMode: false,
    layoutMode: 'grid',
    priorityFilter: [],
    searchOpen: false,
    searchQuery: '',
    selectedItemId: null,
    selectedItemIds: [],
    sidebarOpen: true,
    sidebarWidth: 280,
    statusFilter: [],
  });
  useChatStore.setState({
    abortController: null,
    activeConversationId: null,
    bubblePosition: { x: 24, y: 24 },
    context: null,
    conversations: [],
    isOpen: false,
    isStreaming: false,
    mode: 'bubble',
    selectedModel: 'gpt-4o-mini',
    sidebarWidth: 420,
    systemPromptOverride: null,
  });
  useWebSocketStore.setState({
    activeChannels: new Set(),
    events: [],
    isConnected: false,
    lastEvent: undefined,
    reconnectAttempts: 0,
  });
};

/**
 * Wrapper for async test operations with auto-cleanup
 */
export const withAsyncCleanup = async (testFn: () => Promise<void>) => {
  try {
    await testFn();
  } finally {
    await clearAllStores();
  }
};
