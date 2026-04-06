"""Multi-agent coordination and team management.

Provides AgentTeam for coordinating multiple agents in collaborative tasks,
including handoffs, broadcasts, and consensus mechanisms.
"""

from __future__ import annotations
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Callable, Optional
import asyncio
import uuid


class TeamRole(str, Enum):
    """Roles within an agent team."""
    COORDINATOR = "coordinator"
    WORKER = "worker"
    REVIEWER = "reviewer"
    ORCHESTRATOR = "orchestrator"
    SPECIALIST = "specialist"


@dataclass
class TeamMember:
    """A member of an agent team.
    
    Attributes:
        member_id: Unique identifier for this team member
        agent_id: ID of the agent definition
        role: Role in the team
        name: Human-readable name
        capabilities: List of capabilities this member provides
        status: Current availability status
        current_task: Current task assignment
        metadata: Additional member metadata
    """
    member_id: str
    agent_id: str
    role: TeamRole
    name: str
    capabilities: list[str] = field(default_factory=list)
    status: str = "available"
    current_task: Optional[str] = None
    metadata: dict[str, Any] = field(default_factory=dict)
    
    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "member_id": self.member_id,
            "agent_id": self.agent_id,
            "role": self.role.value,
            "name": self.name,
            "capabilities": self.capabilities,
            "status": self.status,
            "current_task": self.current_task,
            "metadata": self.metadata,
        }


@dataclass
class TeamTask:
    """A task assigned to a team.
    
    Attributes:
        task_id: Unique identifier for this task
        description: Human-readable task description
        assignee: Member assigned to this task
        status: Current task status
        priority: Task priority (higher = more urgent)
        created_at: When task was created
        started_at: When task execution started
        completed_at: When task completed
        result: Task execution result
        error: Error message if failed
        dependencies: IDs of tasks that must complete first
    """
    task_id: str
    description: str
    assignee: Optional[str] = None
    status: str = "pending"
    priority: int = 0
    created_at: datetime = field(default_factory=datetime.utcnow)
    started_at: Optional[datetime] = None
    completed_at: Optional[datetime] = None
    result: Optional[dict[str, Any]] = None
    error: Optional[str] = None
    dependencies: list[str] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)
    
    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "task_id": self.task_id,
            "description": self.description,
            "assignee": self.assignee,
            "status": self.status,
            "priority": self.priority,
            "created_at": self.created_at.isoformat(),
            "started_at": self.started_at.isoformat() if self.started_at else None,
            "completed_at": self.completed_at.isoformat() if self.completed_at else None,
            "result": self.result,
            "error": self.error,
            "dependencies": self.dependencies,
            "metadata": self.metadata,
        }


@dataclass
class AgentTeam:
    """A team of agents working collaboratively.
    
    Provides coordination mechanisms for multi-agent workflows including
    task assignment, broadcasts, and handoffs.
    
    Attributes:
        team_id: Unique identifier for this team
        name: Team name
        members: Team members
        tasks: Pending and completed tasks
        broadcast_handlers: Callbacks for broadcast messages
    """
    team_id: str
    name: str
    members: dict[str, TeamMember] = field(default_factory=dict)
    tasks: dict[str, TeamTask] = field(default_factory=dict)
    completed_tasks: list[TeamTask] = field(default_factory=list)
    broadcast_handlers: list[Callable] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)
    
    def add_member(
        self,
        agent_id: str,
        role: TeamRole,
        name: str,
        capabilities: Optional[list[str]] = None,
    ) -> str:
        """Add a member to the team.
        
        Args:
            agent_id: ID of the agent definition
            role: Role in the team
            name: Human-readable name
            capabilities: List of capabilities
            
        Returns:
            Member ID
        """
        member_id = str(uuid.uuid4())
        
        member = TeamMember(
            member_id=member_id,
            agent_id=agent_id,
            role=role,
            name=name,
            capabilities=capabilities or [],
        )
        
        self.members[member_id] = member
        return member_id
    
    def remove_member(self, member_id: str) -> bool:
        """Remove a member from the team.
        
        Args:
            member_id: ID of member to remove
            
        Returns:
            True if removed, False if not found
        """
        if member_id in self.members:
            del self.members[member_id]
            return True
        return False
    
    def get_member(self, member_id: str) -> Optional[TeamMember]:
        """Get a team member by ID."""
        return self.members.get(member_id)
    
    def get_members_by_role(self, role: TeamRole) -> list[TeamMember]:
        """Get all members with a specific role."""
        return [m for m in self.members.values() if m.role == role]
    
    def get_members_by_capability(self, capability: str) -> list[TeamMember]:
        """Get all members with a specific capability."""
        return [
            m for m in self.members.values()
            if capability in m.capabilities
        ]
    
    def get_available_members(
        self,
        role: Optional[TeamRole] = None,
        capability: Optional[str] = None,
    ) -> list[TeamMember]:
        """Get available team members, optionally filtered.
        
        Args:
            role: If provided, filter by role
            capability: If provided, filter by capability
            
        Returns:
            List of available members
        """
        members = [m for m in self.members.values() if m.status == "available"]
        
        if role:
            members = [m for m in members if m.role == role]
        
        if capability:
            members = [m for m in members if capability in m.capabilities]
        
        return members
    
    def create_task(
        self,
        description: str,
        assignee: Optional[str] = None,
        priority: int = 0,
        dependencies: Optional[list[str]] = None,
        metadata: Optional[dict[str, Any]] = None,
    ) -> str:
        """Create a task for the team.
        
        Args:
            description: Task description
            assignee: Optional member ID to assign
            priority: Task priority
            dependencies: IDs of prerequisite tasks
            metadata: Additional task metadata
            
        Returns:
            Task ID
        """
        task_id = str(uuid.uuid4())
        
        task = TeamTask(
            task_id=task_id,
            description=description,
            assignee=assignee,
            priority=priority,
            dependencies=dependencies or [],
            metadata=metadata or {},
        )
        
        self.tasks[task_id] = task
        return task_id
    
    def assign_task(self, task_id: str, member_id: str) -> bool:
        """Assign a task to a team member.
        
        Args:
            task_id: ID of task to assign
            member_id: ID of member to assign to
            
        Returns:
            True if assigned, False if not found
        """
        task = self.tasks.get(task_id)
        member = self.members.get(member_id)
        
        if not task or not member:
            return False
        
        task.assignee = member_id
        member.current_task = task_id
        member.status = "busy"
        
        return True
    
    def start_task(self, task_id: str) -> bool:
        """Mark a task as started.
        
        Args:
            task_id: ID of task to start
            
        Returns:
            True if started, False if not found
        """
        task = self.tasks.get(task_id)
        if not task:
            return False
        
        # Check dependencies
        for dep_id in task.dependencies:
            dep = self.tasks.get(dep_id)
            if not dep or dep.status != "completed":
                return False
        
        task.status = "in_progress"
        task.started_at = datetime.utcnow()
        
        return True
    
    def complete_task(
        self,
        task_id: str,
        result: Optional[dict[str, Any]] = None,
    ) -> bool:
        """Mark a task as completed.
        
        Args:
            task_id: ID of task to complete
            result: Task execution result
            
        Returns:
            True if completed, False if not found
        """
        task = self.tasks.get(task_id)
        if not task:
            return False
        
        task.status = "completed"
        task.completed_at = datetime.utcnow()
        task.result = result
        
        # Update member status
        if task.assignee:
            member = self.members.get(task.assignee)
            if member:
                member.current_task = None
                member.status = "available"
        
        # Move to completed
        self.tasks.pop(task_id)
        self.completed_tasks.append(task)
        
        return True
    
    def fail_task(self, task_id: str, error: str) -> bool:
        """Mark a task as failed.
        
        Args:
            task_id: ID of task that failed
            error: Error message
            
        Returns:
            True if marked as failed, False if not found
        """
        task = self.tasks.get(task_id)
        if not task:
            return False
        
        task.status = "failed"
        task.completed_at = datetime.utcnow()
        task.error = error
        
        # Update member status
        if task.assignee:
            member = self.members.get(task.assignee)
            if member:
                member.current_task = None
                member.status = "available"
        
        # Move to completed
        self.tasks.pop(task_id)
        self.completed_tasks.append(task)
        
        return True
    
    def get_ready_tasks(self) -> list[TeamTask]:
        """Get tasks with all dependencies satisfied.
        
        Returns:
            List of tasks ready to be executed
        """
        ready = []
        
        for task in self.tasks.values():
            if task.status != "pending":
                continue
            
            # Check all dependencies are completed
            all_deps_done = all(
                self.tasks.get(dep_id) is None and
                any(t.task_id == dep_id and t.status == "completed"
                    for t in self.completed_tasks)
                for dep_id in task.dependencies
            )
            
            if all_deps_done:
                ready.append(task)
        
        # Sort by priority
        ready.sort(key=lambda t: t.priority, reverse=True)
        return ready
    
    def broadcast(self, message: dict[str, Any], sender: Optional[str] = None) -> None:
        """Broadcast a message to all team members.
        
        Args:
            message: Message to broadcast
            sender: Optional member ID of sender
        """
        for handler in self.broadcast_handlers:
            handler(message, sender=sender)
    
    def add_broadcast_handler(
        self,
        handler: Callable[[dict[str, Any], Optional[str]], None],
    ) -> None:
        """Add a handler for broadcast messages.
        
        Args:
            handler: Function to call with broadcast messages
        """
        self.broadcast_handlers.append(handler)
    
    def handover(
        self,
        from_member_id: str,
        to_member_id: str,
        task_id: Optional[str] = None,
        state: Optional[dict[str, Any]] = None,
    ) -> str:
        """Perform a handover between team members.
        
        Args:
            from_member_id: Source member ID
            to_member_id: Target member ID
            task_id: Optional task to hand over
            state: State to transfer
            
        Returns:
            Handoff ID
        """
        from_member = self.members.get(from_member_id)
        to_member = self.members.get(to_member_id)
        
        if not from_member or not to_member:
            raise ValueError("Member not found")
        
        handoff_id = str(uuid.uuid4())
        
        # Transfer task if specified
        if task_id:
            task = self.tasks.get(task_id)
            if task and task.assignee == from_member_id:
                task.assignee = to_member_id
                to_member.current_task = task_id
                from_member.current_task = None
        
        # Notify broadcast handlers
        self.broadcast({
            "type": "handover",
            "handoff_id": handoff_id,
            "from": from_member_id,
            "to": to_member_id,
            "task_id": task_id,
            "state": state,
        }, sender=from_member_id)
        
        return handoff_id
    
    def get_team_status(self) -> dict[str, Any]:
        """Get overall team status.
        
        Returns:
            Dictionary with team status
        """
        return {
            "team_id": self.team_id,
            "name": self.name,
            "members": {
                "total": len(self.members),
                "available": sum(1 for m in self.members.values() if m.status == "available"),
                "busy": sum(1 for m in self.members.values() if m.status == "busy"),
            },
            "tasks": {
                "pending": sum(1 for t in self.tasks.values() if t.status == "pending"),
                "in_progress": sum(1 for t in self.tasks.values() if t.status == "in_progress"),
                "completed_total": len(self.completed_tasks),
            },
            "metadata": self.metadata,
        }
    
    def to_dict(self) -> dict[str, Any]:
        """Convert team to dictionary."""
        return {
            "team_id": self.team_id,
            "name": self.name,
            "members": {mid: m.to_dict() for mid, m in self.members.items()},
            "tasks": {tid: t.to_dict() for tid, t in self.tasks.items()},
            "completed_tasks": [t.to_dict() for t in self.completed_tasks],
            "metadata": self.metadata,
        }


async def execute_team_workflow(
    team: AgentTeam,
    workflow: list[dict[str, Any]],
    executor: Callable[[str, dict[str, Any]], dict[str, Any]],
) -> list[dict[str, Any]]:
    """Execute a workflow across a team of agents.
    
    Args:
        team: AgentTeam to execute workflow
        workflow: List of workflow steps
        executor: Async function to execute agent tasks
        
    Returns:
        List of execution results
    """
    results = []
    
    for step in workflow:
        # Get assignee
        assignee_id = step.get("assignee")
        if not assignee_id:
            # Auto-assign based on capabilities
            capability = step.get("requires_capability")
            if capability:
                available = team.get_available_members(capability=capability)
                if available:
                    assignee_id = available[0].member_id
        
        if not assignee_id:
            raise ValueError(f"No assignee for step: {step}")
        
        # Create and execute task
        task_id = team.create_task(
            description=step.get("description", ""),
            assignee=assignee_id,
            priority=step.get("priority", 0),
            dependencies=step.get("depends_on", []),
        )
        
        team.start_task(task_id)
        
        try:
            result = await executor(assignee_id, step.get("input", {}))
            team.complete_task(task_id, result=result)
            results.append({"task_id": task_id, "result": result})
        except Exception as e:
            team.fail_task(task_id, str(e))
            results.append({"task_id": task_id, "error": str(e)})
    
    return results
