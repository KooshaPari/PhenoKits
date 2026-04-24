import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface Deployment {
  id: string;
  name: string;
  status: "pending" | "building" | "deploying" | "running" | "failed" | "stopped";
  provider: string;
  region: string;
  createdAt: string;
  updatedAt: string;
  url?: string;
  error?: string;
}

export interface DeploymentState {
  deployments: Deployment[];
  selectedDeploymentId: string | null;
  isLoading: boolean;
  error: string | null;
}

export interface DeploymentActions {
  setDeployments: (deployments: Deployment[]) => void;
  addDeployment: (deployment: Deployment) => void;
  updateDeployment: (id: string, updates: Partial<Deployment>) => void;
  removeDeployment: (id: string) => void;
  selectDeployment: (id: string | null) => void;
  setLoading: (isLoading: boolean) => void;
  setError: (error: string | null) => void;
  getDeploymentById: (id: string) => Deployment | undefined;
  getRecentDeployments: (limit?: number) => Deployment[];
}

export type DeploymentStore = DeploymentState & DeploymentActions;

/**
 * Store for managing deployment state.
 *
 * @example
 * ```tsx
 * const { deployments, selectDeployment } = useDeploymentStore();
 * ```
 */
export const useDeploymentStore = create<DeploymentStore>()(
  persist(
    (set, get) => ({
      deployments: [],
      selectedDeploymentId: null,
      isLoading: false,
      error: null,

      setDeployments: (deployments) => set({ deployments }),

      addDeployment: (deployment) =>
        set((state) => ({
          deployments: [deployment, ...state.deployments],
        })),

      updateDeployment: (id, updates) =>
        set((state) => ({
          deployments: state.deployments.map((d) =>
            d.id === id ? { ...d, ...updates, updatedAt: new Date().toISOString() } : d
          ),
        })),

      removeDeployment: (id) =>
        set((state) => ({
          deployments: state.deployments.filter((d) => d.id !== id),
          selectedDeploymentId:
            state.selectedDeploymentId === id ? null : state.selectedDeploymentId,
        })),

      selectDeployment: (id) => set({ selectedDeploymentId: id }),

      setLoading: (isLoading) => set({ isLoading }),

      setError: (error) => set({ error }),

      getDeploymentById: (id) => get().deployments.find((d) => d.id === id),

      getRecentDeployments: (limit = 10) =>
        get()
          .deployments.slice()
          .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
          .slice(0, limit),
    }),
    {
      name: "deployment-store",
      partialize: (state) => ({
        deployments: state.deployments.slice(0, 50), // Limit persisted deployments
        selectedDeploymentId: state.selectedDeploymentId,
      }),
    }
  )
);
