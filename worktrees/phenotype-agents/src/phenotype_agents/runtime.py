"""Agent runtime and execution management.

Provides AgentPool for concurrent agent execution, worker management,
and resource allocation.
"""

from __future__ import annotations
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Callable, Optional
import asyncio
import uuid


class AgentStatus(str, Enum):
    """Status of an agent instance."""
    PENDING = "pending"
    RUNNING = "running"
    WAITING = "waiting"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"


@dataclass
class AgentInstance:
    """An instance of an agent in the runtime.
    
    Attributes:
        instance_id: Unique identifier for this instance
        agent_id: ID of the agent definition
        status: Current execution status
        input: Input data for this instance
        output: Output data (set when completed)
        error: Error message (set if failed)
        created_at: When instance was created
        started_at: When execution started
        completed_at: When execution completed
        metadata: Additional instance metadata
    """
    instance_id: str
    agent_id: str
    status: AgentStatus
    input: dict[str, Any] = field(default_factory=dict)
    output: Optional[dict[str, Any]] = None
    error: Optional[str] = None
    created_at: datetime = field(default_factory=datetime.utcnow)
    started_at: Optional[datetime] = None
    completed_at: Optional[datetime] = None
    metadata: dict[str, Any] = field(default_factory=dict)
    
    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "instance_id": self.instance_id,
            "agent_id": self.agent_id,
            "status": self.status.value,
            "input": self.input,
            "output": self.output,
            "error": self.error,
            "created_at": self.created_at.isoformat(),
            "started_at": self.started_at.isoformat() if self.started_at else None,
            "completed_at": self.completed_at.isoformat() if self.completed_at else None,
            "metadata": self.metadata,
        }


@dataclass
class ResourceLimits:
    """Resource limits for agent execution."""
    max_concurrent: int = 10
    max_memory_mb: int = 1024
    max_cpu_percent: int = 100
    max_execution_time_seconds: int = 3600
    max_retries: int = 3


@dataclass
class ExecutionMetrics:
    """Metrics for agent execution."""
    total_executions: int = 0
    successful_executions: int = 0
    failed_executions: int = 0
    cancelled_executions: int = 0
    total_execution_time_ms: float = 0.0
    average_execution_time_ms: float = 0.0
    peak_concurrent: int = 0
    current_concurrent: int = 0


@dataclass
class AgentPool:
    """Pool for managing concurrent agent executions.
    
    Provides worker management, resource allocation, and execution
    coordination for multiple agents.
    
    Attributes:
        agent_registry: Registry of available agents
        max_concurrent: Maximum concurrent executions
        active_instances: Currently running instances
        completed_instances: Completed instance history
        metrics: Execution metrics
    """
    max_concurrent: int = 10
    active_instances: dict[str, AgentInstance] = field(default_factory=dict)
    completed_instances: list[AgentInstance] = field(default_factory=list)
    metrics: ExecutionMetrics = field(default_factory=ExecutionMetrics)
    agent_handlers: dict[str, Callable] = field(default_factory=dict)
    
    def register_handler(
        self,
        agent_id: str,
        handler: Callable[[dict[str, Any]], dict[str, Any]],
    ) -> None:
        """Register an agent execution handler.
        
        Args:
            agent_id: ID of the agent
            handler: Async function that executes the agent
        """
        self.agent_handlers[agent_id] = handler
    
    async def execute(
        self,
        agent_id: str,
        input_data: dict[str, Any],
        metadata: Optional[dict[str, Any]] = None,
        wait: bool = True,
    ) -> AgentInstance:
        """Execute an agent.
        
        Args:
            agent_id: ID of the agent to execute
            input_data: Input data for the agent
            metadata: Optional metadata for the instance
            wait: If True, wait for completion
            
        Returns:
            AgentInstance with execution results
        """
        # Check capacity
        if len(self.active_instances) >= self.max_concurrent:
            if wait:
                # Wait for capacity
                while len(self.active_instances) >= self.max_concurrent:
                    await asyncio.sleep(0.1)
            else:
                # Return pending instance
                instance = AgentInstance(
                    instance_id=str(uuid.uuid4()),
                    agent_id=agent_id,
                    status=AgentStatus.PENDING,
                    input=input_data,
                    metadata=metadata or {},
                )
                self.completed_instances.append(instance)
                return instance
        
        # Create instance
        instance = AgentInstance(
            instance_id=str(uuid.uuid4()),
            agent_id=agent_id,
            status=AgentStatus.RUNNING,
            input=input_data,
            metadata=metadata or {},
        )
        
        self.active_instances[instance.instance_id] = instance
        self.metrics.total_executions += 1
        self.metrics.current_concurrent = len(self.active_instances)
        self.metrics.peak_concurrent = max(
            self.metrics.peak_concurrent,
            self.metrics.current_concurrent
        )
        
        # Execute asynchronously
        asyncio.create_task(self._execute_instance(instance))
        
        if wait:
            while instance.status == AgentStatus.RUNNING:
                await asyncio.sleep(0.1)
        
        return instance
    
    async def _execute_instance(self, instance: AgentInstance) -> None:
        """Execute an agent instance.
        
        Args:
            instance: AgentInstance to execute
        """
        start_time = datetime.utcnow()
        instance.started_at = start_time
        
        try:
            handler = self.agent_handlers.get(instance.agent_id)
            if not handler:
                raise ValueError(f"No handler registered for agent: {instance.agent_id}")
            
            # Execute handler
            if asyncio.iscoroutinefunction(handler):
                result = await handler(instance.input)
            else:
                result = handler(instance.input)
            
            # Mark completed
            instance.output = result
            instance.status = AgentStatus.COMPLETED
            
        except Exception as e:
            instance.error = str(e)
            instance.status = AgentStatus.FAILED
            self.metrics.failed_executions += 1
            
        finally:
            instance.completed_at = datetime.utcnow()
            completion_time = (instance.completed_at - start_time).total_seconds() * 1000
            self.metrics.total_execution_time_ms += completion_time
            self.metrics.average_execution_time_ms = (
                self.metrics.total_execution_time_ms / self.metrics.total_executions
            )
            
            if instance.status == AgentStatus.COMPLETED:
                self.metrics.successful_executions += 1
            
            # Move to completed
            del self.active_instances[instance.instance_id]
            self.completed_instances.append(instance)
            self.metrics.current_concurrent = len(self.active_instances)
    
    def get_instance(self, instance_id: str) -> Optional[AgentInstance]:
        """Get an instance by ID."""
        return self.active_instances.get(instance_id)
    
    def get_active_instances(self, agent_id: Optional[str] = None) -> list[AgentInstance]:
        """Get all active instances, optionally filtered by agent."""
        instances = list(self.active_instances.values())
        if agent_id:
            instances = [i for i in instances if i.agent_id == agent_id]
        return instances
    
    def get_completed_instances(
        self,
        agent_id: Optional[str] = None,
        limit: int = 100,
    ) -> list[AgentInstance]:
        """Get recent completed instances."""
        instances = list(reversed(self.completed_instances))
        if agent_id:
            instances = [i for i in instances if i.agent_id == agent_id]
        return instances[:limit]
    
    def cancel_instance(self, instance_id: str) -> bool:
        """Cancel a running instance.
        
        Note: Actual cancellation depends on agent implementation
        supporting cancellation tokens.
        
        Args:
            instance_id: ID of instance to cancel
            
        Returns:
            True if cancelled, False if not found or already completed
        """
        instance = self.active_instances.get(instance_id)
        if not instance:
            return False
        
        if instance.status not in (AgentStatus.RUNNING, AgentStatus.PENDING):
            return False
        
        instance.status = AgentStatus.CANCELLED
        instance.completed_at = datetime.utcnow()
        self.metrics.cancelled_executions += 1
        
        del self.active_instances[instance_id]
        self.completed_instances.append(instance)
        self.metrics.current_concurrent = len(self.active_instances)
        
        return True
    
    def cancel_all(self, agent_id: Optional[str] = None) -> int:
        """Cancel all running instances.
        
        Args:
            agent_id: If provided, only cancel instances of this agent
            
        Returns:
            Number of instances cancelled
        """
        cancelled = 0
        instance_ids = list(self.active_instances.keys())
        
        for instance_id in instance_ids:
            instance = self.active_instances[instance_id]
            if agent_id is None or instance.agent_id == agent_id:
                if self.cancel_instance(instance_id):
                    cancelled += 1
        
        return cancelled
    
    def get_metrics(self) -> dict[str, Any]:
        """Get current metrics."""
        return {
            "total_executions": self.metrics.total_executions,
            "successful": self.metrics.successful_executions,
            "failed": self.metrics.failed_executions,
            "cancelled": self.metrics.cancelled_executions,
            "success_rate": (
                self.metrics.successful_executions / self.metrics.total_executions
                if self.metrics.total_executions > 0
                else 0.0
            ),
            "average_execution_time_ms": self.metrics.average_execution_time_ms,
            "peak_concurrent": self.metrics.peak_concurrent,
            "current_concurrent": self.metrics.current_concurrent,
        }
    
    def cleanup_completed(self, keep_last: int = 1000) -> int:
        """Clean up old completed instances.
        
        Args:
            keep_last: Number of completed instances to keep
            
        Returns:
            Number of instances removed
        """
        if len(self.completed_instances) <= keep_last:
            return 0
        
        removed = len(self.completed_instances) - keep_last
        self.completed_instances = self.completed_instances[-keep_last:]
        return removed


@dataclass 
class HandoffManager:
    """Manager for agent-to-agent handoffs.
    
    Coordinates the transfer of state and context between agents.
    """
    pending_handoffs: dict[str, dict[str, Any]] = field(default_factory=dict)
    completed_handoffs: list[dict[str, Any]] = field(default_factory=list)
    
    def initiate_handoff(
        self,
        from_agent: str,
        to_agent: str,
        state: dict[str, Any],
        priority: int = 0,
    ) -> str:
        """Initiate a handoff between agents.
        
        Args:
            from_agent: Source agent ID
            to_agent: Target agent ID
            state: State to transfer
            priority: Handoff priority (higher = more urgent)
            
        Returns:
            Handoff ID
        """
        handoff_id = str(uuid.uuid4())
        
        handoff = {
            "handoff_id": handoff_id,
            "from_agent": from_agent,
            "to_agent": to_agent,
            "state": state,
            "priority": priority,
            "status": "pending",
            "created_at": datetime.utcnow().isoformat(),
        }
        
        self.pending_handoffs[handoff_id] = handoff
        return handoff_id
    
    def complete_handoff(self, handoff_id: str) -> bool:
        """Mark a handoff as completed.
        
        Args:
            handoff_id: ID of handoff to complete
            
        Returns:
            True if completed, False if not found
        """
        handoff = self.pending_handoffs.get(handoff_id)
        if not handoff:
            return False
        
        handoff["status"] = "completed"
        handoff["completed_at"] = datetime.utcnow().isoformat()
        
        self.pending_handoffs.pop(handoff_id)
        self.completed_handoffs.append(handoff)
        
        return True
    
    def get_pending(self, agent_id: Optional[str] = None) -> list[dict[str, Any]]:
        """Get pending handoffs.
        
        Args:
            agent_id: If provided, filter by target agent
            
        Returns:
            List of pending handoffs
        """
        pending = list(self.pending_handoffs.values())
        
        if agent_id:
            pending = [h for h in pending if h["to_agent"] == agent_id]
        
        # Sort by priority (descending)
        pending.sort(key=lambda h: h["priority"], reverse=True)
        
        return pending
    
    def cancel_handoff(self, handoff_id: str) -> bool:
        """Cancel a pending handoff.
        
        Args:
            handoff_id: ID of handoff to cancel
            
        Returns:
            True if cancelled, False if not found
        """
        handoff = self.pending_handoffs.pop(handoff_id, None)
        if not handoff:
            return False
        
        handoff["status"] = "cancelled"
        handoff["cancelled_at"] = datetime.utcnow().isoformat()
        self.completed_handoffs.append(handoff)
        
        return True
