import { createFileRoute, redirect } from '@tanstack/react-router';

import { requireAuth } from '@/lib/route-guards';

const FeaturesPage = (): null => null;

export const Route = createFileRoute('/projects/$projectId/features')({
  beforeLoad: async ({ params }) => {
    await requireAuth();
    // eslint-disable-next-line typescript-eslint/only-throw-error
    throw redirect({
      params,
      search: { tab: 'features' },
      to: '/projects/$projectId/specifications',
    });
  },
  component: FeaturesPage,
});
