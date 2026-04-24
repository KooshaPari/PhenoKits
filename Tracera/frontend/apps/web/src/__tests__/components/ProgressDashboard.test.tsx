import type {
  HealthStatus,
  Milestone,
  MilestoneStatus,
  ProgressSnapshot,
  ProjectMetrics,
  Sprint,
  SprintStatus,
} from '@atoms/types';

import '@testing-library/jest-dom';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { vi } from 'vitest';

import { ProgressDashboard } from '../../components/temporal/ProgressDashboard';

async function selectTab(name: RegExp) {
  const tab = screen.getByRole('tab', { name });
  fireEvent.pointerDown(tab);
  fireEvent.mouseDown(tab);
  fireEvent.mouseUp(tab);
  fireEvent.click(tab);
  await waitFor(() => expect(tab).toHaveAttribute('aria-selected', 'true'));
}

// Mock data
const mockMilestone: Milestone = {
  createdAt: new Date().toISOString(),
  health: 'green' satisfies HealthStatus,
  id: '1',
  itemCount: 10,
  itemIds: ['item-1', 'item-2'],
  name: 'v1.0 Release',
  progress: {
    blockedItems: 0,
    completedItems: 7,
    inProgressItems: 2,
    notStartedItems: 1,
    percentage: 70,
    totalItems: 10,
  },
  projectId: 'proj-1',
  slug: 'v1-0-release',
  status: 'in_progress' satisfies MilestoneStatus,
  targetDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
  updatedAt: new Date().toISOString(),
};

const mockSprint: Sprint = {
  addedPoints: 0,
  completedItemIds: [],
  completedPoints: 35,
  createdAt: new Date().toISOString(),
  durationDays: 14,
  endDate: new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString(),
  health: 'green' satisfies HealthStatus,
  id: 'sprint-1',
  itemCount: 10,
  itemIds: [],
  name: 'Sprint 1',
  plannedPoints: 50,
  projectId: 'proj-1',
  remainingPoints: 15,
  removedPoints: 0,
  slug: 'sprint-1',
  startDate: new Date().toISOString(),
  status: 'active' satisfies SprintStatus,
  updatedAt: new Date().toISOString(),
};

const mockMetrics: ProjectMetrics = {
  atRiskCount: 3,
  blockedCount: 2,
  byLifecycle: {},
  byPriority: {
    critical: 10,
    high: 20,
    low: 30,
    medium: 40,
  },
  byStatus: {
    done: 30,
    in_progress: 30,
    todo: 40,
  },
  byType: {},
  completedLastWeek: 8,
  completedThisWeek: 10,
  overdueCount: 1,
  totalItems: 100,
  velocity: 10 / 7,
};

const mockSnapshot: ProgressSnapshot = {
  createdAt: new Date().toISOString(),
  id: 'snapshot-1',
  metrics: mockMetrics as any,
  projectId: 'proj-1',
  snapshotDate: new Date().toISOString(),
};

describe(ProgressDashboard, () => {
  const defaultProps = {
    milestones: [mockMilestone],
    projectId: 'proj-1',
    sprints: [mockSprint],
  };

  it('renders dashboard with summary cards', () => {
    render(<ProgressDashboard {...defaultProps} />);

    expect(screen.getByText('Overall Progress')).toBeInTheDocument();
    expect(screen.getByText('Active Milestones')).toBeInTheDocument();
    expect(screen.getByText('At Risk')).toBeInTheDocument();
    expect(screen.getByText('Completed')).toBeInTheDocument();
  });

  it('displays correct summary values', () => {
    render(<ProgressDashboard {...defaultProps} />);

    expect(screen.getByText('70%')).toBeInTheDocument(); // Overall progress
    expect(screen.getByText('1')).toBeInTheDocument(); // Active milestones
    expect(screen.getAllByText('0').length).toBeGreaterThanOrEqual(2); // At risk and completed
  });

  it('renders milestone hierarchy', async () => {
    render(<ProgressDashboard {...defaultProps} />);

    await selectTab(/milestones/i);

    expect(screen.getByText('In Progress')).toBeInTheDocument();
    expect(screen.getByText('v1.0 Release')).toBeInTheDocument();
  });

  it('renders sprint timeline', async () => {
    render(<ProgressDashboard {...defaultProps} />);

    await selectTab(/sprints/i);

    expect(screen.getByText('Active Sprint')).toBeInTheDocument();
    expect(screen.getByText('Sprint 1')).toBeInTheDocument();
  });

  it('calls onMilestoneClick when milestone is selected', async () => {
    const onMilestoneClick = vi.fn();
    render(<ProgressDashboard {...defaultProps} onMilestoneClick={onMilestoneClick} />);

    await selectTab(/milestones/i);
    const milestoneElement = screen.getByText('v1.0 Release');
    fireEvent.click(milestoneElement);

    expect(onMilestoneClick).toHaveBeenCalledWith(mockMilestone.id);
  });

  it('calls onSprintClick when sprint is selected', async () => {
    const onSprintClick = vi.fn();
    render(<ProgressDashboard {...defaultProps} onSprintClick={onSprintClick} />);

    await selectTab(/sprints/i);
    const sprintElement = screen.getByText('Sprint 1');
    fireEvent.click(sprintElement);

    expect(onSprintClick).toHaveBeenCalledWith(mockSprint.id);
  });

  it('renders health status section', async () => {
    render(<ProgressDashboard {...defaultProps} />);

    await selectTab(/milestones/i);

    expect(screen.getByText('Milestones')).toBeInTheDocument();
    expect(screen.getByText('In Progress')).toBeInTheDocument();
    expect(screen.getByText('Sprints')).toBeInTheDocument();
  });

  it('renders progress snapshots', () => {
    render(<ProgressDashboard {...defaultProps} />);

    expect(screen.getByRole('tab', { name: /overview/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /velocity/i })).toBeInTheDocument();
  });

  it('handles empty milestones list', async () => {
    render(<ProgressDashboard {...defaultProps} milestones={[]} />);

    await selectTab(/milestones/i);

    expect(screen.getByText('No milestones found')).toBeInTheDocument();
  });

  it('handles empty sprints list', async () => {
    render(<ProgressDashboard {...defaultProps} sprints={[]} />);

    await selectTab(/sprints/i);

    expect(screen.getByText('No sprints found')).toBeInTheDocument();
  });

  it('displays multiple milestones', async () => {
    const milestone2: Milestone = {
      ...mockMilestone,
      id: '2',
      name: 'v1.1 Patch',
    };

    render(<ProgressDashboard {...defaultProps} milestones={[mockMilestone, milestone2]} />);

    await selectTab(/milestones/i);

    expect(screen.getByText('v1.0 Release')).toBeInTheDocument();
    expect(screen.getByText('v1.1 Patch')).toBeInTheDocument();
  });

  it('displays at-risk milestone count correctly', () => {
    const atRiskMilestone: Milestone = {
      ...mockMilestone,
      id: '2',
      health: 'red' satisfies HealthStatus,
      status: 'blocked' satisfies MilestoneStatus,
    };

    render(<ProgressDashboard {...defaultProps} milestones={[mockMilestone, atRiskMilestone]} />);

    expect(screen.getAllByText('1').length).toBeGreaterThan(0);
    expect(screen.getByText('At-Risk Milestones')).toBeInTheDocument();
  });

  it('shows active sprint highlighted', async () => {
    const activeSprint: Sprint = {
      ...mockSprint,
      status: 'active' satisfies SprintStatus,
    };

    const planningSprint: Sprint = {
      ...mockSprint,
      id: 'sprint-2',
      name: 'Sprint 2',
      status: 'planning' satisfies SprintStatus,
    };

    render(<ProgressDashboard {...defaultProps} sprints={[activeSprint, planningSprint]} />);

    await selectTab(/sprints/i);

    expect(screen.getByText('Active Sprint')).toBeInTheDocument();
  });
});
