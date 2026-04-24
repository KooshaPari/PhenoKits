import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { Loader2, LogOut } from 'lucide-react';
import { useEffect, useState } from 'react';
import type { ReactElement } from 'react';

import { logger } from '@/lib/logger';
import { useAuthStore } from '@/stores/auth-store';

const LOGOUT_REDIRECT_DELAY_MS = 500;

const LogoutPage = (): ReactElement => {
  const navigate = useNavigate();
  const logout = useAuthStore((state) => state.logout);
  const [isLoggingOut, setIsLoggingOut] = useState(true);

  useEffect(() => {
    const performLogout = async (): Promise<void> => {
      try {
        await logout();
        setTimeout(() => {
          void navigate({ to: '/auth/login' });
        }, LOGOUT_REDIRECT_DELAY_MS);
      } catch (error) {
        logger.error('Logout error:', error);
        await navigate({ to: '/auth/login' });
      } finally {
        setIsLoggingOut(false);
      }
    };

    void performLogout();
  }, [logout, navigate]);

  let statusIcon = <LogOut className='text-primary h-10 w-10' />;
  let statusTitle = 'Signed out successfully';
  let statusDescription = 'Redirecting to login page';

  if (isLoggingOut) {
    statusIcon = <Loader2 className='text-primary h-10 w-10 animate-spin' />;
    statusTitle = 'Signing out...';
    statusDescription = 'Clearing your session';
  }

  return (
    <div className='bg-background flex min-h-screen items-center justify-center'>
      <div className='space-y-6 text-center'>
        <div className='relative mx-auto h-20 w-20'>
          <div className='bg-primary/10 absolute inset-0 animate-pulse rounded-full' />
          <div className='relative flex h-full w-full items-center justify-center'>{statusIcon}</div>
        </div>

        <div className='space-y-2'>
          <h2 className='text-2xl font-bold'>{statusTitle}</h2>
          <p className='text-muted-foreground'>{statusDescription}</p>
        </div>
      </div>
    </div>
  );
};

export const Route = createFileRoute('/auth/logout')({
  component: LogoutPage,
});
