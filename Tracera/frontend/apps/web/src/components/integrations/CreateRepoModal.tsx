/**
 * Create GitHub repository modal component.
 */

import { Github, Loader2 } from 'lucide-react';
import { type ChangeEvent, type SyntheticEvent, useCallback, useState } from 'react';
import { toast } from 'sonner';

import type { GitHubAppInstallation, GitHubRepo } from '@/api/github';

import { Button } from '@/components/ui/enterprise-button';
import { useCreateGitHubRepo } from '@/hooks/useGitHub';
import { Input } from '@tracertm/ui';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@tracertm/ui/components/Dialog';
import { Label } from '@tracertm/ui/components/Label';
import { Textarea } from '@tracertm/ui/components/Textarea';

const readMessage = (error: object): string | undefined => {
  const message: unknown = Reflect.get(error, 'message');
  return typeof message === 'string' ? message : undefined;
};

export interface CreateRepoModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  installation: GitHubAppInstallation;
  accountId: string;
  onSuccess?: (repo: GitHubRepo) => void;
}

function getCreateRepoErrorMessage(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === 'object' && error !== null) {
    return readMessage(error) ?? 'Failed to create repository';
  }
  return 'Failed to create repository';
}

export const CreateRepoModal = function CreateRepoModal({
  open,
  onOpenChange,
  installation,
  accountId,
  onSuccess,
}: CreateRepoModalProps) {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [isPrivate, setIsPrivate] = useState(false);

  const createRepo = useCreateGitHubRepo();

  const handleSubmit = useCallback(
    async (event: SyntheticEvent<HTMLFormElement>) => {
      event.preventDefault();

      if (!name.trim()) {
        toast.error('Repository name is required');
        return;
      }

      try {
        const trimmedDescription = description.trim();
        const orgValue =
          installation.target_type === 'Organization' ? installation.account_login : undefined;
        const repo = await createRepo.mutateAsync({
          account_id: accountId,
          ...(trimmedDescription !== '' ? { description: trimmedDescription } : {}),
          installation_id: installation.id,
          name: name.trim(),
          ...(orgValue !== undefined && orgValue !== '' ? { org: orgValue } : {}),
          private: isPrivate,
        });

        toast.success(`Repository "${repo.full_name}" created successfully`);
        onSuccess?.(repo);
        onOpenChange(false);

        setName('');
        setDescription('');
        setIsPrivate(false);
      } catch (error) {
        toast.error(getCreateRepoErrorMessage(error));
      }
    },
    [
      accountId,
      createRepo,
      description,
      installation.account_login,
      installation.id,
      installation.target_type,
      isPrivate,
      name,
      onOpenChange,
      onSuccess,
    ],
  );

  const handleNameChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value);
  }, []);
  const handleDescriptionChange = useCallback((event: ChangeEvent<HTMLTextAreaElement>) => {
    setDescription(event.target.value);
  }, []);
  const handlePrivateChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    setIsPrivate(event.target.checked);
  }, []);
  const handleCancel = useCallback(() => {
    onOpenChange(false);
  }, [onOpenChange]);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className='sm:max-w-[500px]'>
        <DialogHeader>
          <DialogTitle className='flex items-center gap-2'>
            <Github className='h-5 w-5' />
            Create New Repository
          </DialogTitle>
          <DialogDescription>
            Create a new repository in {installation.account_login}
          </DialogDescription>
        </DialogHeader>

        <form
          onSubmit={(event) => {
            handleSubmit(event).catch((error: unknown) => {
              toast.error(getCreateRepoErrorMessage(error));
            });
          }}
        >
          <div className='space-y-4 py-4'>
            <div className='space-y-2'>
              <Label htmlFor='repo-name'>Repository Name *</Label>
              <Input
                id='repo-name'
                placeholder='my-awesome-repo'
                value={name}
                onChange={handleNameChange}
                disabled={createRepo.isPending}
                pattern='[a-zA-Z0-9._-]+'
                title='Repository name can only contain letters, numbers, dots, hyphens, and underscores'
              />
              <p className='text-muted-foreground text-xs'>
                Repository name can only contain letters, numbers, dots, hyphens, and underscores
              </p>
            </div>

            <div className='space-y-2'>
              <Label htmlFor='repo-description'>Description</Label>
              <Textarea
                id='repo-description'
                placeholder='A short description of your repository'
                value={description}
                onChange={handleDescriptionChange}
                disabled={createRepo.isPending}
                rows={3}
              />
            </div>

            <div className='flex items-center space-x-2'>
              <input
                type='checkbox'
                id='repo-private'
                checked={isPrivate}
                onChange={handlePrivateChange}
                disabled={createRepo.isPending}
                className='h-4 w-4 rounded border-gray-300'
              />
              <Label htmlFor='repo-private' className='cursor-pointer'>
                Make this repository private
              </Label>
            </div>
          </div>

          <DialogFooter>
            <Button
              type='button'
              variant='outline'
              onClick={handleCancel}
              disabled={createRepo.isPending}
            >
              Cancel
            </Button>
            <Button type='submit' disabled={createRepo.isPending || !name.trim()}>
              {createRepo.isPending && <Loader2 className='mr-2 h-4 w-4 animate-spin' />}
              Create Repository
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
