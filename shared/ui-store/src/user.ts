import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface User {
  id: string;
  email: string;
  name: string;
  avatar?: string;
  role: "admin" | "user" | "viewer";
  preferences?: {
    theme: "light" | "dark" | "system";
    language: string;
    notifications: boolean;
  };
}

export interface UserState {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  apiKeys: string[];
}

export interface UserActions {
  setUser: (user: User | null) => void;
  updateUser: (updates: Partial<User>) => void;
  setAuthenticated: (isAuthenticated: boolean) => void;
  setLoading: (isLoading: boolean) => void;
  setError: (error: string | null) => void;
  logout: () => void;
  updatePreferences: (preferences: Partial<User["preferences"]>) => void;
  addApiKey: (keyId: string) => void;
  removeApiKey: (keyId: string) => void;
  hasPermission: (permission: string) => boolean;
}

export type UserStore = UserState & UserActions;

/**
 * Store for managing user authentication and profile state.
 *
 * @example
 * ```tsx
 * const { user, isAuthenticated, logout } = useUserStore();
 * ```
 */
export const useUserStore = create<UserStore>()(
  persist(
    (set, get) => ({
      user: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,
      apiKeys: [],

      setUser: (user) =>
        set({
          user,
          isAuthenticated: !!user,
          error: null,
        }),

      updateUser: (updates) =>
        set((state) => ({
          user: state.user ? { ...state.user, ...updates } : null,
        })),

      setAuthenticated: (isAuthenticated) => set({ isAuthenticated }),

      setLoading: (isLoading) => set({ isLoading }),

      setError: (error) => set({ error }),

      logout: () =>
        set({
          user: null,
          isAuthenticated: false,
          apiKeys: [],
          error: null,
        }),

      updatePreferences: (preferences) =>
        set((state) => ({
          user: state.user
            ? {
                ...state.user,
                preferences: { ...state.user.preferences, ...preferences },
              }
            : null,
        })),

      addApiKey: (keyId) =>
        set((state) => ({
          apiKeys: [...state.apiKeys, keyId],
        })),

      removeApiKey: (keyId) =>
        set((state) => ({
          apiKeys: state.apiKeys.filter((k) => k !== keyId),
        })),

      hasPermission: (permission) => {
        const user = get().user;
        if (!user) return false;

        const rolePermissions: Record<string, string[]> = {
          admin: ["*"],
          user: ["read", "write", "deploy"],
          viewer: ["read"],
        };

        const permissions = rolePermissions[user.role] || [];
        return permissions.includes("*") || permissions.includes(permission);
      },
    }),
    {
      name: "user-store",
      partialize: (state) => ({
        user: state.user,
        isAuthenticated: state.isAuthenticated,
        preferences: state.user?.preferences,
      }),
    }
  )
);
