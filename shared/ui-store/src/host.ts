import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface Host {
  id: string;
  name: string;
  address: string;
  status: "online" | "offline" | "maintenance" | "error";
  region?: string;
  tags?: string[];
  lastSeenAt?: string;
  metadata?: Record<string, unknown>;
}

export interface HostState {
  hosts: Host[];
  selectedHostId: string | null;
  isLoading: boolean;
  error: string | null;
}

export interface HostActions {
  setHosts: (hosts: Host[]) => void;
  addHost: (host: Host) => void;
  updateHost: (id: string, updates: Partial<Host>) => void;
  removeHost: (id: string) => void;
  selectHost: (id: string | null) => void;
  setHostStatus: (id: string, status: Host["status"]) => void;
  setLoading: (isLoading: boolean) => void;
  setError: (error: string | null) => void;
  getHostById: (id: string) => Host | undefined;
  getOnlineHosts: () => Host[];
  getHostsByRegion: (region: string) => Host[];
}

export type HostStore = HostState & HostActions;

/**
 * Store for managing host/infrastructure state.
 *
 * @example
 * ```tsx
 * const { hosts, selectHost, getOnlineHosts } = useHostStore();
 * ```
 */
export const useHostStore = create<HostStore>()(
  persist(
    (set, get) => ({
      hosts: [],
      selectedHostId: null,
      isLoading: false,
      error: null,

      setHosts: (hosts) => set({ hosts }),

      addHost: (host) =>
        set((state) => ({
          hosts: [...state.hosts, host],
        })),

      updateHost: (id, updates) =>
        set((state) => ({
          hosts: state.hosts.map((h) => (h.id === id ? { ...h, ...updates } : h)),
        })),

      removeHost: (id) =>
        set((state) => ({
          hosts: state.hosts.filter((h) => h.id !== id),
          selectedHostId: state.selectedHostId === id ? null : state.selectedHostId,
        })),

      selectHost: (id) => set({ selectedHostId: id }),

      setHostStatus: (id, status) =>
        set((state) => ({
          hosts: state.hosts.map((h) =>
            h.id === id
              ? {
                  ...h,
                  status,
                  lastSeenAt: status === "online" ? new Date().toISOString() : h.lastSeenAt,
                }
              : h
          ),
        })),

      setLoading: (isLoading) => set({ isLoading }),

      setError: (error) => set({ error }),

      getHostById: (id) => get().hosts.find((h) => h.id === id),

      getOnlineHosts: () => get().hosts.filter((h) => h.status === "online"),

      getHostsByRegion: (region) =>
        get().hosts.filter((h) => h.region?.toLowerCase() === region.toLowerCase()),
    }),
    {
      name: "host-store",
      partialize: (state) => ({
        hosts: state.hosts.slice(0, 100),
        selectedHostId: state.selectedHostId,
      }),
    }
  )
);
