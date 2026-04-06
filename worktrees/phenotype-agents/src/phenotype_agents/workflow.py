"""Workflow engine for multi-agent orchestration.

Provides topological execution, dependency resolution, and parallel execution
support for multi-agent workflows.
"""

from __future__ import annotations
import asyncio
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Callable, Optional
import uuid

from .schema import WorkflowSpec, WorkflowStep, AgentSpec, AgentRegistry


class StepStatus(str, Enum):
    """Status of a workflow step."""
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    SKIPPED = "skipped"


class WorkflowStatus(str, Enum):
    """Status of a workflow execution."""
    CREATED = "created"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"


@dataclass
class StepResult:
    """Result of a workflow step execution."""
    step_id: str
    status: StepStatus
    output: Optional[Any] = None
    error: Optional[str] = None
    started_at: Optional[datetime] = None
    completed_at: Optional[datetime] = None
    duration_ms: float = 0.0

    @property
    def success(self) -> bool:
        """Check if step was successful."""
        return self.status == StepStatus.COMPLETED


@dataclass
class WorkflowContext:
    """Context for workflow execution.
    
    Provides shared state and data flow between workflow steps.
    """
    workflow_id: str
    inputs: dict[str, Any] = field(default_factory=dict)
    outputs: dict[str, Any] = field(default_factory=dict)
    metadata: dict[str, Any] = field(default_factory=dict)
    created_at: datetime = field(default_factory=datetime.utcnow)

    def get_input(self, key: str, default: Any = None) -> Any:
        """Get input value with optional default."""
        return self.inputs.get(key, default)

    def set_output(self, key: str, value: Any) -> None:
        """Set output value."""
        self.outputs[key] = value

    def get_output(self, key: str, default: Any = None) -> Any:
        """Get output value with optional default."""
        return self.outputs.get(key, default)


class WorkflowEngine:
    """Engine for executing multi-agent workflows.
    
    Supports:
    - Topological execution based on dependencies
    - Parallel execution of independent steps
    - Retry policies and failure handling
    - Progress tracking and metrics
    
    Attributes:
        registry: Agent registry for looking up agent specifications
        max_parallel: Maximum number of parallel steps
        failure_strategy: How to handle step failures
    """
    
    def __init__(
        self,
        registry: Optional[AgentRegistry] = None,
        max_parallel: int = 3,
        failure_strategy: str = "stop",
    ):
        self.registry = registry or AgentRegistry()
        self.max_parallel = max_parallel
        self.failure_strategy = failure_strategy
        
        # Step execution functions
        self._step_handlers: dict[str, Callable] = {}
        
        # Metrics
        self._execution_count = 0
        self._total_duration_ms = 0.0
    
    def register_handler(self, agent_name: str, handler: Callable) -> None:
        """Register a handler function for an agent.
        
        Args:
            agent_name: Name of the agent
            handler: Async function that executes the step
        """
        self._step_handlers[agent_name] = handler
    
    async def execute(
        self,
        spec: WorkflowSpec,
        inputs: dict[str, Any],
    ) -> WorkflowResult:
        """Execute a workflow specification.
        
        Args:
            spec: Workflow specification
            inputs: Input data for the workflow
            
        Returns:
            WorkflowResult with outputs and metrics
        """
        self._execution_count += 1
        start_time = datetime.utcnow()
        
        # Create context
        context = WorkflowContext(
            workflow_id=str(uuid.uuid4()),
            inputs=inputs,
        )
        
        # Create step tracking
        step_results: dict[str, StepResult] = {}
        for step in spec.steps:
            step_results[step.step_id] = StepResult(
                step_id=step.step_id,
                status=StepStatus.PENDING,
            )
        
        # Build dependency graph
        dependency_graph = self._build_dependency_graph(spec.steps)
        
        # Execute steps in topological order
        status = WorkflowStatus.RUNNING
        try:
            completed = set()
            while len(completed) < len(spec.steps):
                # Get ready steps (all dependencies met)
                ready_steps = self._get_ready_steps(
                    spec.steps, step_results, completed, dependency_graph
                )
                
                if not ready_steps:
                    if len(completed) < len(spec.steps):
                        # Deadlock - no steps can run
                        status = WorkflowStatus.FAILED
                        break
                    break
                
                # Execute ready steps (up to max_parallel)
                batch = ready_steps[:spec.max_parallel]
                batch_results = await self._execute_batch(batch, spec, context)
                
                # Update results
                for step_id, result in batch_results.items():
                    step_results[step_id] = result
                    if result.success:
                        completed.add(step_id)
                        context.set_output(result.step_id, result.output)
                    else:
                        # Handle failure
                        if spec.failure_strategy == "stop":
                            status = WorkflowStatus.FAILED
                            # Mark remaining as skipped
                            for s in spec.steps:
                                if s.step_id not in completed:
                                    step_results[s.step_id].status = StepStatus.SKIPPED
                            break
                        elif spec.failure_strategy == "retry":
                            # TODO: Implement retry logic
                            status = WorkflowStatus.FAILED
                            break
                
                if status == WorkflowStatus.FAILED:
                    break
            
            if status != WorkflowStatus.FAILED:
                status = WorkflowStatus.COMPLETED
        
        except Exception as e:
            status = WorkflowStatus.FAILED
            context.metadata["error"] = str(e)
        
        # Calculate duration
        end_time = datetime.utcnow()
        duration_ms = (end_time - start_time).total_seconds() * 1000
        self._total_duration_ms += duration_ms
        
        return WorkflowResult(
            workflow_id=context.workflow_id,
            status=status,
            outputs=context.outputs,
            step_results=step_results,
            duration_ms=duration_ms,
            started_at=start_time,
            completed_at=end_time,
        )
    
    def _build_dependency_graph(
        self,
        steps: list[WorkflowStep]
    ) -> dict[str, set[str]]:
        """Build a dependency graph from steps.
        
        Returns:
            Dict mapping step_id to set of step_ids it depends on
        """
        graph: dict[str, set[str]] = {}
        
        for step in steps:
            dependencies = set()
            for input_key, input_source in step.input_mapping.items():
                # Parse input source (format: "step_id.output_key")
                if "." in input_source:
                    dep_step = input_source.split(".")[0]
                    dependencies.add(dep_step)
            graph[step.step_id] = dependencies
        
        return graph
    
    def _get_ready_steps(
        self,
        steps: list[WorkflowStep],
        results: dict[str, StepResult],
        completed: set[str],
        dependency_graph: dict[str, set[str]],
    ) -> list[WorkflowStep]:
        """Get steps that are ready to execute.
        
        A step is ready if:
        1. It hasn't been executed yet
        2. All its dependencies have completed successfully
        """
        ready = []
        
        for step in steps:
            if step.step_id in completed:
                continue
            
            result = results.get(step.step_id)
            if result and result.status != StepStatus.PENDING:
                continue
            
            # Check dependencies
            deps = dependency_graph.get(step.step_id, set())
            if all(dep in completed for dep in deps):
                # Check condition if present
                if step.condition:
                    # TODO: Evaluate condition expression
                    pass
                ready.append(step)
        
        return ready
    
    async def _execute_batch(
        self,
        steps: list[WorkflowStep],
        spec: WorkflowSpec,
        context: WorkflowContext,
    ) -> dict[str, StepResult]:
        """Execute a batch of steps in parallel.
        
        Args:
            steps: Steps to execute
            spec: Workflow specification
            context: Workflow context
            
        Returns:
            Dict mapping step_id to StepResult
        """
        tasks = [
            self._execute_step(step, spec, context)
            for step in steps
        ]
        
        results = await asyncio.gather(*tasks, return_exceptions=True)
        
        output: dict[str, StepResult] = {}
        for step, result in zip(steps, results):
            if isinstance(result, Exception):
                output[step.step_id] = StepResult(
                    step_id=step.step_id,
                    status=StepStatus.FAILED,
                    error=str(result),
                )
            else:
                output[step.step_id] = result
        
        return output
    
    async def _execute_step(
        self,
        step: WorkflowStep,
        spec: WorkflowSpec,
        context: WorkflowContext,
    ) -> StepResult:
        """Execute a single workflow step.
        
        Args:
            step: Step to execute
            spec: Workflow specification
            context: Workflow context
            
        Returns:
            StepResult with execution outcome
        """
        start_time = datetime.utcnow()
        
        # Resolve inputs
        inputs = {}
        for input_key, input_source in step.input_mapping.items():
            if "." in input_source:
                source_step, source_key = input_source.split(".", 1)
                value = context.get_output(f"{source_step}.{source_key}")
                if value is None:
                    value = context.get_input(input_source)
            else:
                value = context.get_input(input_source)
            inputs[input_key] = value
        
        # Get handler
        handler = self._step_handlers.get(step.agent_name)
        if not handler:
            return StepResult(
                step_id=step.step_id,
                status=StepStatus.FAILED,
                error=f"No handler registered for agent: {step.agent_name}",
                started_at=start_time,
                completed_at=datetime.utcnow(),
            )
        
        try:
            # Execute with timeout
            output = await asyncio.wait_for(
                handler(step.step_id, inputs),
                timeout=step.timeout_seconds,
            )
            
            end_time = datetime.utcnow()
            duration_ms = (end_time - start_time).total_seconds() * 1000
            
            return StepResult(
                step_id=step.step_id,
                status=StepStatus.COMPLETED,
                output=output,
                started_at=start_time,
                completed_at=end_time,
                duration_ms=duration_ms,
            )
        
        except asyncio.TimeoutError:
            return StepResult(
                step_id=step.step_id,
                status=StepStatus.FAILED,
                error=f"Step timed out after {step.timeout_seconds}s",
                started_at=start_time,
                completed_at=datetime.utcnow(),
            )
        
        except Exception as e:
            return StepResult(
                step_id=step.step_id,
                status=StepStatus.FAILED,
                error=str(e),
                started_at=start_time,
                completed_at=datetime.utcnow(),
            )


@dataclass
class WorkflowResult:
    """Result of a workflow execution."""
    workflow_id: str
    status: WorkflowStatus
    outputs: dict[str, Any]
    step_results: dict[str, StepResult]
    duration_ms: float
    started_at: datetime
    completed_at: datetime
    
    @property
    def success(self) -> bool:
        """Check if workflow was successful."""
        return self.status == WorkflowStatus.COMPLETED
    
    @property
    def failed_steps(self) -> list[str]:
        """Get list of failed step IDs."""
        return [
            step_id for step_id, result in self.step_results.items()
            if result.status == StepStatus.FAILED
        ]
