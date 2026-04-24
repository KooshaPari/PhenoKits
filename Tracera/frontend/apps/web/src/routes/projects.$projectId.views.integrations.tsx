import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { useEffect } from 'react';

export const Route = createFileRoute('/projects/$projectId/views/integrations')({
  component: ProjectIntegrationsPage,
});

function ProjectIntegrationsPage() {
  const { projectId } = Route.useParams();
  const navigate = useNavigate();

  useEffect(() => {
    navigate({
      params: { projectId },
      replace: true,
      search: { tab: 'integrations' },
      to: '/projects/$projectId/settings',
    }).catch(() => undefined);
  }, [navigate, projectId]);

  return null;
}
