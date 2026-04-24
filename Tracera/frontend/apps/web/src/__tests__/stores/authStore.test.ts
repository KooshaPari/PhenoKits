/**
 * Tests for authStore
 */

import { act, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { useAuthStore } from '../../stores/authStore';

type TestGlobals = typeof globalThis & {
  __setFetchImpl__?: (impl: typeof fetch) => void;
};

const mockUser = {
  email: 'test@example.com',
  id: '1',
  name: 'Test User',
};

describe('authStore', () => {
  beforeEach(() => {
    const { refreshTimer } = useAuthStore.getState();
    if (refreshTimer) {
      clearInterval(refreshTimer);
    }
    useAuthStore.setState({
      account: null,
      authKitRefreshToken: null,
      isAuthenticated: false,
      isLoading: false,
      refreshTimer: null,
      token: null,
      user: null,
    });
    localStorage.clear();
    sessionStorage.clear();
    vi.restoreAllMocks();
  });

  describe('initial state', () => {
    it('should have correct initial values', () => {
      const { result } = renderHook(() => useAuthStore());

      expect(result.current.user).toBeNull();
      expect(result.current.token).toBeNull();
      expect(result.current.isAuthenticated).toBeFalsy();
      expect(result.current.isLoading).toBeFalsy();
    });
  });

  describe('setUser', () => {
    it('should set user and update authentication status', () => {
      const { result } = renderHook(() => useAuthStore());

      act(() => {
        result.current.setUser({
          email: 'test@example.com',
          id: '1',
          name: 'Test User',
        });
      });

      expect(result.current.user).toEqual({
        email: 'test@example.com',
        id: '1',
        name: 'Test User',
      });
      expect(result.current.isAuthenticated).toBeTruthy();
    });

    it('should clear authentication when user is null', () => {
      const { result } = renderHook(() => useAuthStore());

      // First set a user
      act(() => {
        result.current.setUser({
          email: 'test@example.com',
          id: '1',
        });
      });

      // Then clear it
      act(() => {
        result.current.setUser(null);
      });

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBeFalsy();
    });
  });

  describe('setToken', () => {
    it('should store token in state and localStorage', () => {
      const { result } = renderHook(() => useAuthStore());

      act(() => {
        result.current.setToken('test-token');
      });

      expect(result.current.token).toBe('test-token');
      expect(localStorage.getItem('auth_token')).toBe('test-token');
    });

    it('should remove token from localStorage when null', () => {
      const { result } = renderHook(() => useAuthStore());

      // Set token first
      act(() => {
        result.current.setToken('test-token');
      });

      // Then remove it
      act(() => {
        result.current.setToken(null);
      });

      expect(result.current.token).toBeNull();
      expect(localStorage.getItem('auth_token')).toBeNull();
    });
  });

  describe('loginWithCode', () => {
    it('should authenticate with AuthKit callback response', async () => {
      const { result } = renderHook(() => useAuthStore());
      (globalThis as TestGlobals).__setFetchImpl__?.(async () =>
        Response.json({
          refresh_token: 'refresh-token',
          token: 'mock-jwt-token',
          user: mockUser,
        }),
      );

      await act(async () => {
        await result.current.loginWithCode('auth-code', 'auth-state');
      });

      expect(result.current.isAuthenticated).toBeTruthy();
      expect(result.current.user).toEqual(mockUser);
      expect(result.current.token).toBe('mock-jwt-token');
      expect(result.current.authKitRefreshToken).toBe('refresh-token');
      expect(localStorage.getItem('auth_token')).toBe('mock-jwt-token');
    });

    it('should reject missing callback parameters without authenticating', async () => {
      const { result } = renderHook(() => useAuthStore());

      await act(async () => {
        await expect(result.current.loginWithCode('', 'auth-state')).rejects.toThrow(
          'Authorization code and state are required',
        );
      });

      expect(result.current.isLoading).toBeFalsy();
      expect(result.current.isAuthenticated).toBeFalsy();
      expect(result.current.user).toBeNull();
    });
  });

  describe('logout', () => {
    it('should clear all auth data', async () => {
      const { result } = renderHook(() => useAuthStore());

      act(() => {
        result.current.setUser(mockUser);
        result.current.setToken('mock-jwt-token');
        useAuthStore.setState({ authKitRefreshToken: 'refresh-token' });
      });

      await act(async () => {
        await result.current.logout();
      });

      expect(result.current.user).toBeNull();
      expect(result.current.token).toBeNull();
      expect(result.current.isAuthenticated).toBeFalsy();
      expect(result.current.authKitRefreshToken).toBeNull();
      expect(localStorage.getItem('auth_token')).toBeNull();
    });
  });

  describe('updateProfile', () => {
    it('should update user profile', async () => {
      const { result } = renderHook(() => useAuthStore());

      act(() => {
        result.current.setUser(mockUser);
      });

      act(() => {
        result.current.updateProfile({
          avatar: 'avatar.jpg',
          name: 'Updated Name',
        });
      });

      expect(result.current.user).toEqual({
        avatar: 'avatar.jpg',
        email: 'test@example.com',
        id: '1',
        name: 'Updated Name',
      });
    });

    it('should not update if no user is logged in', () => {
      const { result } = renderHook(() => useAuthStore());

      act(() => {
        result.current.updateProfile({
          name: 'Updated Name',
        });
      });

      expect(result.current.user).toBeNull();
    });
  });

  describe('persistence', () => {
    it('should persist auth state to localStorage', async () => {
      const { result } = renderHook(() => useAuthStore());

      act(() => {
        result.current.setUser(mockUser);
        result.current.setToken('mock-jwt-token');
      });

      const storedData = localStorage.getItem('tracertm-auth-store');
      expect(storedData).toBeTruthy();

      if (storedData) {
        const parsed = JSON.parse(storedData);
        expect(parsed.state.user).toEqual(mockUser);
        expect(parsed.state.token).toBe('mock-jwt-token');
        expect(parsed.state.isAuthenticated).toBeTruthy();
      }
    });
  });
});
