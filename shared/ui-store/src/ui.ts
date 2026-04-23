import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface Toast {
  id: string;
  title?: string;
  description?: string;
  variant?: "default" | "success" | "error" | "warning" | "info";
  duration?: number;
}

export interface ModalState {
  isOpen: boolean;
  type: string | null;
  data?: unknown;
}

export interface UIState {
  theme: "light" | "dark" | "system";
  sidebarOpen: boolean;
  toasts: Toast[];
  activeModal: ModalState;
  breadcrumbs: { label: string; href?: string }[];
  isLoading: boolean;
}

export interface UIActions {
  setTheme: (theme: UIState["theme"]) => void;
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  addToast: (toast: Omit<Toast, "id">) => string;
  removeToast: (id: string) => void;
  clearToasts: () => void;
  openModal: (type: string, data?: unknown) => void;
  closeModal: () => void;
  setBreadcrumbs: (breadcrumbs: UIState["breadcrumbs"]) => void;
  setLoading: (isLoading: boolean) => void;
}

export type UIStore = UIState & UIActions;

let toastIdCounter = 0;

/**
 * Store for managing UI state (theme, modals, toasts, etc.).
 *
 * @example
 * ```tsx
 * const { theme, addToast, openModal } = useUIStore();
 * ```
 */
export const useUIStore = create<UIStore>()(
  persist(
    (set, get) => ({
      theme: "system",
      sidebarOpen: true,
      toasts: [],
      activeModal: { isOpen: false, type: null },
      breadcrumbs: [],
      isLoading: false,

      setTheme: (theme) => set({ theme }),

      toggleSidebar: () =>
        set((state) => ({ sidebarOpen: !state.sidebarOpen })),

      setSidebarOpen: (open) => set({ sidebarOpen: open }),

      addToast: (toast) => {
        const id = `toast-${++toastIdCounter}`;
        const duration = toast.duration ?? 5000;

        set((state) => ({
          toasts: [...state.toasts, { ...toast, id }],
        }));

        // Auto-remove toast after duration
        setTimeout(() => {
          get().removeToast(id);
        }, duration);

        return id;
      },

      removeToast: (id) =>
        set((state) => ({
          toasts: state.toasts.filter((t) => t.id !== id),
        })),

      clearToasts: () => set({ toasts: [] }),

      openModal: (type, data) =>
        set({
          activeModal: { isOpen: true, type, data },
        }),

      closeModal: () =>
        set({
          activeModal: { isOpen: false, type: null },
        }),

      setBreadcrumbs: (breadcrumbs) => set({ breadcrumbs }),

      setLoading: (isLoading) => set({ isLoading }),
    }),
    {
      name: "ui-store",
      partialize: (state) => ({
        theme: state.theme,
        sidebarOpen: state.sidebarOpen,
      }),
    }
  )
);
