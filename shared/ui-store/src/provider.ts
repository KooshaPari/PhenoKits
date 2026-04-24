import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface Provider {
  id: string;
  name: string;
  alias: string;
  type: "cloud" | "self-hosted" | "hybrid";
  status: "connected" | "disconnected" | "error" | "testing";
  credentials?: {
    apiKey?: string;
    endpoint?: string;
    region?: string;
  };
  config?: Record<string, unknown>;
  lastTestedAt?: string;
  errorMessage?: string;
}

export interface ProviderState {
  providers: Provider[];
  selectedProviderId: string | null;
  isLoading: boolean;
  error: string | null;
}

export interface ProviderActions {
  setProviders: (providers: Provider[]) => void;
  addProvider: (provider: Provider) => void;
  updateProvider: (id: string, updates: Partial<Provider>) => void;
  removeProvider: (id: string) => void;
  selectProvider: (id: string | null) => void;
  setProviderStatus: (id: string, status: Provider["status"]) => void;
  setProviderError: (id: string, errorMessage: string) => void;
  setLoading: (isLoading: boolean) => void;
  setError: (error: string | null) => void;
  getProviderById: (id: string) => Provider | undefined;
  getConnectedProviders: () => Provider[];
}

export type ProviderStore = ProviderState & ProviderActions;

/**
 * Store for managing cloud provider state.
 *
 * @example
 * ```tsx
 * const { providers, selectProvider } = useProviderStore();
 * ```
 */
export const useProviderStore = create<ProviderStore>()(
  persist(
    (set, get) => ({
      providers: [],
      selectedProviderId: null,
      isLoading: false,
      error: null,

      setProviders: (providers) => set({ providers }),

      addProvider: (provider) =>
        set((state) => ({
          providers: [...state.providers, provider],
        })),

      updateProvider: (id, updates) =>
        set((state) => ({
          providers: state.providers.map((p) =>
            p.id === id
              ? {
                  ...p,
                  ...updates,
                  lastTestedAt: updates.status === "connected" ? new Date().toISOString() : p.lastTestedAt,
                }
              : p
          ),
        })),

      removeProvider: (id) =>
        set((state) => ({
          providers: state.providers.filter((p) => p.id !== id),
          selectedProviderId: state.selectedProviderId === id ? null : state.selectedProviderId,
        })),

      selectProvider: (id) => set({ selectedProviderId: id }),

      setProviderStatus: (id, status) =>
        set((state) => ({
          providers: state.providers.map((p) =>
            p.id === id
              ? { ...p, status, lastTestedAt: status === "connected" ? new Date().toISOString() : p.lastTestedAt }
              : p
          ),
        })),

      setProviderError: (id, errorMessage) =>
        set((state) => ({
          providers: state.providers.map((p) =>
            p.id === id ? { ...p, status: "error", errorMessage } : p
          ),
        })),

      setLoading: (isLoading) => set({ isLoading }),

      setError: (error) => set({ error }),

      getProviderById: (id) => get().providers.find((p) => p.id === id),

      getConnectedProviders: () => get().providers.filter((p) => p.status === "connected"),
    }),
    {
      name: "provider-store",
      partialize: (state) => ({
        providers: state.providers.map((p) => ({
          ...p,
          credentials: undefined, // Don't persist credentials
        })),
        selectedProviderId: state.selectedProviderId,
      }),
    }
  )
);
