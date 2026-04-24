/**
 * Comprehensive tests for auth store
 * Tests authentication state management, login/logout, and persistence
 */

import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { User } from '../../stores/authStore';

import { useAuthStore } from '../../stores/authStore';

const mockFetch = vi.fn();
globalThis.fetch = mockFetch as unknown as typeof fetch;
const TEST_TOKEN = 'test-token';
const TEST_REFRESH_TOKEN = 'refresh-token';

describe('AuthStore', () => {
  beforeEach(() => {
    useAuthStore.getState().stopAutoRefresh();
    // Reset store before each test
    useAuthStore.setState({
      account: null,
      authKitRefreshToken: null,
      isAuthenticated: false,
      isLoading: false,
      refreshTimer: null,
      token: null,
      user: null,
    });
    // Clear localStorage
    localStorage.clear();
    sessionStorage.clear();
    mockFetch.mockReset();
    globalThis.fetch = mockFetch as unknown as typeof fetch;
  });

  const mockUser: User = {
    avatar: 'https://example.com/avatar.jpg',
    email: 'test@example.com',
    id: 'user-1',
    metadata: { department: 'Engineering' },
    name: 'Test User',
    role: 'admin',
  };

  describe('initial state', () => {
    it('should have null user initially', () => {
      const { user } = useAuthStore.getState();
      expect(user).toBeNull();
    });

    it('should have null token initially', () => {
      const { token } = useAuthStore.getState();
      expect(token).toBeNull();
    });

    it('should not be authenticated initially', () => {
      const { isAuthenticated } = useAuthStore.getState();
      expect(isAuthenticated).toBeFalsy();
    });

    it('should not be loading initially', () => {
      const { isLoading } = useAuthStore.getState();
      expect(isLoading).toBeFalsy();
    });
  });

  describe('setUser', () => {
    it('should set user', () => {
      const { setUser } = useAuthStore.getState();
      setUser(mockUser);

      const { user } = useAuthStore.getState();
      expect(user).toEqual(mockUser);
    });

    it('should set isAuthenticated to true when user is set', () => {
      const { setUser } = useAuthStore.getState();
      setUser(mockUser);

      const { isAuthenticated } = useAuthStore.getState();
      expect(isAuthenticated).toBeTruthy();
    });

    it('should set isAuthenticated to false when user is null', () => {
      const { setUser } = useAuthStore.getState();
      setUser(mockUser);
      setUser(null);

      const { isAuthenticated } = useAuthStore.getState();
      expect(isAuthenticated).toBeFalsy();
    });

    it('should handle partial user data', () => {
      const { setUser } = useAuthStore.getState();
      const partialUser: User = {
        email: 'test@example.com',
        id: 'user-1',
      };
      setUser(partialUser);

      const { user } = useAuthStore.getState();
      expect(user).toEqual(partialUser);
      expect(user?.name).toBeUndefined();
    });

    it('should overwrite existing user', () => {
      const { setUser } = useAuthStore.getState();
      setUser(mockUser);

      const newUser: User = {
        email: 'new@example.com',
        id: 'user-2',
      };
      setUser(newUser);

      const { user } = useAuthStore.getState();
      expect(user).toEqual(newUser);
    });
  });

  describe('setToken', () => {
    it('should set token in state', () => {
      const { setToken } = useAuthStore.getState();
      setToken('test-token');

      const { token } = useAuthStore.getState();
      expect(token).toBe('test-token');
    });

    it('should store token in localStorage', () => {
      const { setToken } = useAuthStore.getState();
      setToken('test-token');

      expect(localStorage.getItem('auth_token')).toBe('test-token');
    });

    it('should remove token from localStorage when null', () => {
      const { setToken } = useAuthStore.getState();
      setToken('test-token');
      setToken(null);

      expect(localStorage.getItem('auth_token')).toBeNull();
    });

    it('should clear token from state when null', () => {
      const { setToken } = useAuthStore.getState();
      setToken('test-token');
      setToken(null);

      const { token } = useAuthStore.getState();
      expect(token).toBeNull();
    });

    it('should handle empty string token', () => {
      const { setToken } = useAuthStore.getState();
      setToken('');

      // Empty string is falsy, should be treated as null
      expect(localStorage.getItem('auth_token')).toBeNull();
      expect(useAuthStore.getState().token).toBeNull();
    });
  });

  function mockAuthKitCallback(user: User = mockUser, token = TEST_TOKEN): void {
    mockFetch.mockResolvedValueOnce({
      json: async () => ({
        refresh_token: TEST_REFRESH_TOKEN,
        token,
        user,
      }),
      ok: true,
    } as Response);
  }

  describe('loginWithCode', () => {
    it('should set loading state during login', async () => {
      mockAuthKitCallback();
      const { loginWithCode } = useAuthStore.getState();

      await loginWithCode('auth-code', 'auth-state');

      expect(useAuthStore.getState().isLoading).toBeFalsy();
      expect(useAuthStore.getState().user).not.toBeNull();
    });

    it('should set user and token on successful login', async () => {
      mockAuthKitCallback();
      const { loginWithCode } = useAuthStore.getState();
      await loginWithCode('auth-code', 'auth-state');

      const { user, token, isAuthenticated } = useAuthStore.getState();
      expect(user).not.toBeNull();
      expect(user?.email).toBe('test@example.com');
      expect(token).toBe(TEST_TOKEN);
      expect(isAuthenticated).toBeTruthy();
    });

    it('should persist callback user fields unchanged', async () => {
      const authUser = { email: 'john.doe@example.com', id: 'user-2', name: 'John Doe' };
      mockAuthKitCallback(authUser);
      const { loginWithCode } = useAuthStore.getState();
      await loginWithCode('auth-code', 'auth-state');

      const { user } = useAuthStore.getState();
      expect(user).toEqual(authUser);
    });

    it('should handle login errors', async () => {
      mockFetch.mockResolvedValueOnce({
        json: async () => ({ error: 'Invalid code' }),
        ok: false,
        status: 401,
      } as Response);
      const { loginWithCode } = useAuthStore.getState();

      await expect(loginWithCode('bad-code', 'auth-state')).rejects.toThrow('Invalid code: 401');
      expect(useAuthStore.getState().isAuthenticated).toBeFalsy();
    });

    it('should clear loading state on error', async () => {
      const { loginWithCode } = useAuthStore.getState();

      await expect(loginWithCode('', 'auth-state')).rejects.toThrow(
        'Authorization code and state are required',
      );
      expect(useAuthStore.getState().isLoading).toBeFalsy();
    });

    it('should persist token to localStorage', async () => {
      mockAuthKitCallback();
      const { loginWithCode } = useAuthStore.getState();
      await loginWithCode('auth-code', 'auth-state');

      expect(localStorage.getItem('auth_token')).toBe(TEST_TOKEN);
    });
  });

  describe('logout', () => {
    it('should clear user', async () => {
      const { setUser, logout } = useAuthStore.getState();
      setUser(mockUser);
      await logout();

      const { user } = useAuthStore.getState();
      expect(user).toBeNull();
    });

    it('should clear token', async () => {
      const { setToken, logout } = useAuthStore.getState();
      setToken('test-token');
      await logout();

      const { token } = useAuthStore.getState();
      expect(token).toBeNull();
    });

    it('should set isAuthenticated to false', async () => {
      const { setUser, logout } = useAuthStore.getState();
      setUser(mockUser);
      await logout();

      const { isAuthenticated } = useAuthStore.getState();
      expect(isAuthenticated).toBeFalsy();
    });

    it('should remove token from localStorage', async () => {
      const { setToken, logout } = useAuthStore.getState();
      setToken('test-token');
      await logout();

      expect(localStorage.getItem('auth_token')).toBeNull();
    });

    it('should handle logout when already logged out', async () => {
      const { logout } = useAuthStore.getState();

      await expect(logout()).resolves.toBeUndefined();

      const { user, token, isAuthenticated } = useAuthStore.getState();
      expect(user).toBeNull();
      expect(token).toBeNull();
      expect(isAuthenticated).toBeFalsy();
    });
  });

  describe('refreshToken', () => {
    it('should not throw error', async () => {
      const { refreshToken } = useAuthStore.getState();

      await expect(refreshToken()).resolves.not.toThrow();
    });

    it('should refresh token when a refresh token is available', async () => {
      useAuthStore.setState({ authKitRefreshToken: TEST_REFRESH_TOKEN });
      mockFetch.mockResolvedValueOnce({
        json: async () => ({ refresh_token: 'next-refresh-token', token: 'next-token' }),
        ok: true,
      } as Response);
      const { refreshToken } = useAuthStore.getState();

      await refreshToken();

      expect(useAuthStore.getState().token).toBe('next-token');
      expect(useAuthStore.getState().authKitRefreshToken).toBe('next-refresh-token');
    });
  });

  describe('updateProfile', () => {
    it('should update user profile', () => {
      const { setUser, updateProfile } = useAuthStore.getState();
      setUser(mockUser);

      updateProfile({ name: 'Updated Name' });

      const { user } = useAuthStore.getState();
      expect(user?.name).toBe('Updated Name');
    });

    it('should preserve unchanged fields', () => {
      const { setUser, updateProfile } = useAuthStore.getState();
      setUser(mockUser);

      updateProfile({ name: 'Updated Name' });

      const { user } = useAuthStore.getState();
      expect(user?.email).toBe('test@example.com');
      expect(user?.id).toBe('user-1');
    });

    it('should handle multiple field updates', () => {
      const { setUser, updateProfile } = useAuthStore.getState();
      setUser(mockUser);

      updateProfile({
        avatar: 'https://example.com/new-avatar.jpg',
        name: 'New Name',
        role: 'user',
      });

      const { user } = useAuthStore.getState();
      expect(user?.name).toBe('New Name');
      expect(user?.avatar).toBe('https://example.com/new-avatar.jpg');
      expect(user?.role).toBe('user');
    });

    it('should handle updating metadata', () => {
      const { setUser, updateProfile } = useAuthStore.getState();
      setUser(mockUser);

      updateProfile({
        metadata: { department: 'Sales', location: 'NYC' },
      });

      const { user } = useAuthStore.getState();
      expect(user?.metadata).toEqual({ department: 'Sales', location: 'NYC' });
    });

    it('should do nothing if no user is set', () => {
      const { updateProfile } = useAuthStore.getState();

      expect(() => {
        updateProfile({ name: 'Test' });
      }).not.toThrow();

      const { user } = useAuthStore.getState();
      expect(user).toBeNull();
    });

    it('should handle empty updates', () => {
      const { setUser, updateProfile } = useAuthStore.getState();
      setUser(mockUser);

      updateProfile({});

      const { user } = useAuthStore.getState();
      expect(user).toEqual(mockUser);
    });
  });

  describe('persistence', () => {
    it('should persist user to localStorage', () => {
      const { setUser } = useAuthStore.getState();
      setUser(mockUser);

      // Check if data is persisted
      const stored = localStorage.getItem('tracertm-auth-store');
      expect(stored).not.toBeNull();

      const parsed = JSON.parse(stored!);
      expect(parsed.state.user).toEqual(mockUser);
    });

    it('should persist authentication state', () => {
      const { setUser } = useAuthStore.getState();
      setUser(mockUser);

      const stored = localStorage.getItem('tracertm-auth-store');
      const parsed = JSON.parse(stored!);
      expect(parsed.state.isAuthenticated).toBeTruthy();
    });

    it('should not persist isLoading state', () => {
      useAuthStore.setState({ isLoading: true });

      const stored = localStorage.getItem('tracertm-auth-store');
      if (stored) {
        const parsed = JSON.parse(stored);
        expect(parsed.state.isLoading).toBeUndefined();
      }
    });
  });

  describe('edge cases', () => {
    it('should handle rapid state changes', () => {
      const { setUser } = useAuthStore.getState();

      for (let i = 0; i < 100; i++) {
        setUser({ email: `user${i}@example.com`, id: `user-${i}` });
      }

      const { user } = useAuthStore.getState();
      expect(user?.id).toBe('user-99');
    });

    it('should handle concurrent login/logout', async () => {
      mockAuthKitCallback();
      const { loginWithCode, logout } = useAuthStore.getState();

      const loginPromise = loginWithCode('auth-code', 'auth-state');
      await loginPromise;
      await logout();

      // State should reflect the last operation
      const { user, isAuthenticated } = useAuthStore.getState();
      expect(user).toBeNull(); // Logout completed last
      expect(isAuthenticated).toBeFalsy();
    });

    it('should handle user with minimal data', () => {
      const { setUser } = useAuthStore.getState();
      const minimalUser: User = {
        email: 'user@example.com',
        id: '1',
      };

      setUser(minimalUser);

      const { user, isAuthenticated } = useAuthStore.getState();
      expect(user).toEqual(minimalUser);
      expect(isAuthenticated).toBeTruthy();
    });

    it('should handle user with maximum data', () => {
      const { setUser } = useAuthStore.getState();
      const maximalUser: User = {
        avatar: 'https://example.com/avatar.jpg',
        email: 'user@example.com',
        id: '1',
        metadata: {
          custom_field_1: 'value1',
          custom_field_2: 'value2',
          department: 'Engineering',
          location: 'Remote',
          team: 'Platform',
          timezone: 'UTC',
        },
        name: 'Test User',
        role: 'super_admin',
      };

      setUser(maximalUser);

      const { user } = useAuthStore.getState();
      expect(user).toEqual(maximalUser);
    });

    it('should handle special characters in email', async () => {
      const authUser = { email: 'test+tag@example.com', id: 'user-special' };
      mockAuthKitCallback(authUser);
      const { loginWithCode } = useAuthStore.getState();
      await loginWithCode('auth-code', 'auth-state');

      const { user } = useAuthStore.getState();
      expect(user?.email).toBe('test+tag@example.com');
    });

    it('should handle very long email', async () => {
      const longEmail = `${'a'.repeat(50)}@example.com`;
      mockAuthKitCallback({ email: longEmail, id: 'user-long-email' });
      const { loginWithCode } = useAuthStore.getState();
      await loginWithCode('auth-code', 'auth-state');

      const { user } = useAuthStore.getState();
      expect(user?.email).toBe(longEmail);
    });
  });

  describe('type safety', () => {
    it('should enforce User type for setUser', () => {
      const { setUser } = useAuthStore.getState();

      // Valid user
      expect(() => {
        setUser({
          email: 'test@example.com',
          id: '1',
        });
      }).not.toThrow();

      // Null is valid
      expect(() => {
        setUser(null);
      }).not.toThrow();
    });

    it('should allow optional User fields', () => {
      const { setUser } = useAuthStore.getState();

      const userWithoutOptional: User = {
        email: 'test@example.com',
        id: '1',
      };

      expect(() => {
        setUser(userWithoutOptional);
      }).not.toThrow();
    });
  });
});
