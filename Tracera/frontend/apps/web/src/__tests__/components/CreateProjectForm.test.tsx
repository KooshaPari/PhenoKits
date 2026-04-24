/**
 * Tests for CreateProjectForm
 */

import { describe, expect, it, vi } from 'vitest';

import { CreateProjectForm } from '../../components/forms/CreateProjectForm';
import { render, screen, waitFor } from '../utils/test-utils';

describe(CreateProjectForm, () => {
  it('should render form fields', () => {
    const onSubmit = vi.fn();
    const onCancel = vi.fn();

    const { container } = render(<CreateProjectForm onSubmit={onSubmit} onCancel={onCancel} />);

    expect(container.querySelector('form')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /cancel/i })).toBeInTheDocument();
  });

  it('should call onCancel when cancel button is clicked', async () => {
    const onSubmit = vi.fn();
    const onCancel = vi.fn();

    render(<CreateProjectForm onSubmit={onSubmit} onCancel={onCancel} />);

    await user.click(screen.getByRole('button', { name: /cancel/i }));

    expect(onCancel).toHaveBeenCalledOnce();
  });

  it('should show validation errors for empty required fields', async () => {
    const onSubmit = vi.fn();
    const onCancel = vi.fn();

    render(<CreateProjectForm onSubmit={onSubmit} onCancel={onCancel} />);

    await user.click(screen.getByRole('button', { name: /create project/i }));

    await waitFor(() => {
      expect(screen.getByText(/name is required/i)).toBeInTheDocument();
    });

    expect(onSubmit).not.toHaveBeenCalled();
  });

  it('should submit form with valid data', async () => {
    const onSubmit = vi.fn();
    const onCancel = vi.fn();

    const { container } = render(<CreateProjectForm onSubmit={onSubmit} onCancel={onCancel} />);

    const nameInput = screen.getByLabelText(/project name/i);
    await globalThis.user.type(nameInput, 'Test Project');

    await globalThis.user.click(screen.getByRole('button', { name: /create project/i }));

    await waitFor(() => {
      expect(onSubmit.mock.calls[0]?.[0]).toEqual(
        expect.objectContaining({ name: 'Test Project' }),
      );
    });

    expect(container.querySelector('form')).toBeInTheDocument();
  });

  it('should disable submit button when loading', () => {
    const onSubmit = vi.fn();
    const onCancel = vi.fn();

    render(<CreateProjectForm onSubmit={onSubmit} onCancel={onCancel} isLoading />);

    const submitButton = screen.getByRole('button', { name: /creating/i });
    expect(submitButton).toBeDisabled();
  });
});
