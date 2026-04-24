import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { Loader2 } from 'lucide-react';
import { useEffect, useState } from 'react';
import type { ReactElement } from 'react';
import { toast } from 'sonner';

import { logger } from '@/lib/logger';
import { useAuthStore } from '@/stores/auth-store';

const LOGIN_REDIRECT_DELAY_MS = 2000;

const getErrorMessage = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }

  return 'Authentication failed';
};

const AuthCallback = (): ReactElement => {
  const navigate = useNavigate();
  const loginWithCode = useAuthStore((state) => state.loginWithCode);
  const [authError, setAuthError] = useState<string>();

  useEffect(() => {
    const handleCallback = async (): Promise<void> => {
      const params = new URLSearchParams(globalThis.location.search);
      const code = params.get('code');
      const state = params.get('state');

      try {
        if (code === null || code === '') {
          const errorDesc =
            params.get('error_description') ??
            params.get('error') ??
            'No authorization code received';
          throw new Error(errorDesc);
        }

        if (state === null || state === '') {
          throw new Error('No state parameter received');
        }

        await loginWithCode(code, state);
        toast.success('Welcome back!');
        await navigate({ replace: true, to: '/home' });
      } catch (error) {
        const message = getErrorMessage(error);

        logger.error('AuthKit callback failed:', error);
        setAuthError(message);
        toast.error(message);
        setTimeout(() => {
          void navigate({ replace: true, to: '/auth/login' });
        }, LOGIN_REDIRECT_DELAY_MS);
      }
    };

    void handleCallback();
  }, [loginWithCode, navigate]);

  if (authError !== undefined && authError !== '') {
    return (
      <div className='flex min-h-screen items-center justify-center bg-slate-950 p-4'>
        <div className='space-y-6 text-center'>
          <div className='space-y-2'>
            <h2 className='text-2xl font-bold text-red-400'>Authentication failed</h2>
            <p className='text-sm text-slate-400'>{authError}</p>
            <p className='text-sm text-slate-500'>Redirecting to login...</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className='flex min-h-screen items-center justify-center bg-slate-950 p-4'>
      <div className='space-y-6 text-center'>
        <div className='relative mx-auto h-20 w-20'>
          <div className='bg-primary/10 absolute inset-0 animate-pulse rounded-full' />
          <div className='relative flex h-full w-full items-center justify-center'>
            <Loader2 className='text-primary h-10 w-10 animate-spin' />
          </div>
        </div>
        <div className='space-y-2'>
          <h2 className='text-2xl font-bold text-slate-100'>Completing sign in...</h2>
          <p className='text-sm text-slate-400'>Please wait while we verify your credentials.</p>
        </div>
      </div>
    </div>
  );
};

export const Route = createFileRoute('/auth/callback')({
  component: AuthCallback,
});
