# ADR-003: Structured Concurrency with asyncio.TaskGroup

## Status
**ACCEPTED**

## Context

PhenoLang's pheno-async module orchestrates complex deployment workflows across multiple cloud providers and services. Current async patterns exhibit:

- **Task Leaks**: Background tasks orphaned when parent fails
- **Cleanup Failures**: Resources not released on cancellation
- **Unclear Lifetimes**: Task relationships not explicit in code
- **Error Handling**: Exceptions in background tasks lost or mishandled

With 681 files across 33 modules, we need a consistent approach to async resource management that prevents leaks and ensures cleanup.

## Decision

We will adopt **Structured Concurrency** using Python's `asyncio.TaskGroup` (3.11+) and `asyncio.timeout`.

### Decision Details

#### Core Principle: Parent-Child Task Relationships

All concurrent operations must follow structured concurrency:

```python
import asyncio

# ❌ UNSTRUCTURED: Orphaned tasks possible
async def bad_pattern():
    task1 = asyncio.create_task(worker1())  # May outlive parent
    task2 = asyncio.create_task(worker2())  # No explicit relationship
    await task1  # If task2 fails, task1 continues orphaned

# ✅ STRUCTURED: All tasks complete together
async def good_pattern():
    async with asyncio.TaskGroup() as tg:
        tg.create_task(worker1())  # Child of TaskGroup
        tg.create_task(worker2())  # Child of TaskGroup
    # Guaranteed: Both tasks complete (success or exception) before exit
```

#### Deployment Orchestration Pattern

```python
class DeploymentOrchestrator:
    """Orchestrate deployment with structured concurrency."""
    
    async def deploy_all(self, deployment: Deployment) -> DeploymentResult:
        """Deploy all resources with proper cancellation."""
        resources = list(deployment.resources.values())
        
        # TaskGroup ensures all provision tasks complete together
        async with asyncio.TaskGroup() as tg:
            tasks = [
                tg.create_task(self._provision_resource(r))
                for r in resources
            ]
        
        # All tasks done (success or first exception)
        results = [t.result() for t in tasks]
        return DeploymentResult(deployment.id, results)
    
    async def _provision_resource(self, resource: Resource) -> ProvisionResult:
        """Provision single resource with timeout."""
        try:
            async with asyncio.timeout(300):  # 5 minute timeout
                provider = await self._get_provider(resource.provider_type)
                
                # Nested TaskGroup for sub-operations
                async with asyncio.TaskGroup() as tg:
                    setup = tg.create_task(provider.setup(resource.config))
                    network = tg.create_task(provider.configure_network(resource))
                    storage = tg.create_task(provider.allocate_storage(resource))
                
                return ProvisionResult.success(resource.id)
                
        except asyncio.TimeoutError:
            return ProvisionResult.timeout(resource.id)
        except* ProviderError as eg:
            return ProvisionResult.failed(resource.id, eg.exceptions)
    
    async def deploy_with_rollback(self, deployment: Deployment) -> DeploymentResult:
        """Deploy with automatic rollback on failure."""
        provisioned: List[Resource] = []
        
        try:
            async with asyncio.TaskGroup() as tg:
                for resource in deployment.resources.values():
                    task = tg.create_task(self._provision_with_tracking(resource, provisioned))
                    
        except* Exception as eg:
            # ExceptionGroup: Multiple failures possible
            # Rollback all successfully provisioned resources
            async with asyncio.TaskGroup() as tg:
                for resource in provisioned:
                    tg.create_task(self._rollback_resource(resource))
            
            raise DeploymentFailed(eg.exceptions)
    
    async def _provision_with_tracking(self, resource: Resource, provisioned: List) -> None:
        """Provision and track for potential rollback."""
        result = await self._provision_resource(resource)
        if result.success:
            provisioned.append(resource)
        return result
```

#### Supervisor Pattern for Long-Running Tasks

```python
from typing import Callable, Awaitable, Set
from dataclasses import dataclass
from enum import Enum, auto

class WorkerState(Enum):
    RUNNING = auto()
    CRASHED = auto()
    STOPPED = auto()

@dataclass
class Worker:
    name: str
    coro: Callable[[], Awaitable[None]]
    dependencies: Set[str]
    max_restarts: int = 5
    restart_window: float = 60.0

class Supervisor:
    """Supervise workers with structured restart strategies."""
    
    def __init__(self):
        self._workers: Dict[str, Worker] = {}
        self._states: Dict[str, WorkerState] = {}
        self._tasks: Dict[str, asyncio.Task] = {}
        self._restart_counts: Dict[str, List[float]] = {}
        self._shutdown_event = asyncio.Event()
    
    def add_worker(self, worker: Worker) -> None:
        self._workers[worker.name] = worker
    
    async def run(self) -> None:
        """Run all workers respecting dependencies."""
        # Topological sort by dependencies
        ordered = self._topological_sort()
        
        async with asyncio.TaskGroup() as tg:
            for name in ordered:
                task = tg.create_task(self._run_worker(name))
                self._tasks[name] = task
        
        # All workers stopped
        logger.info("Supervisor shutdown complete")
    
    async def _run_worker(self, name: str) -> None:
        """Run single worker with restart logic."""
        worker = self._workers[name]
        
        while not self._shutdown_event.is_set():
            self._states[name] = WorkerState.RUNNING
            
            try:
                await worker.coro()
                # Clean exit
                self._states[name] = WorkerState.STOPPED
                return
                
            except asyncio.CancelledError:
                self._states[name] = WorkerState.STOPPED
                raise
                
            except Exception as e:
                self._states[name] = WorkerState.CRASHED
                logger.error(f"Worker {name} crashed: {e}")
                
                if not self._should_restart(name, worker):
                    logger.error(f"Worker {name} exceeded restart limits")
                    raise SupervisorShutdown(f"Worker {name} failed")
    
    def _should_restart(self, name: str, worker: Worker) -> bool:
        """Check if worker should be restarted."""
        now = asyncio.get_event_loop().time()
        
        # Clean old history
        history = self._restart_counts.get(name, [])
        history = [t for t in history if now - t < worker.restart_window]
        
        if len(history) >= worker.max_restarts:
            return False
        
        history.append(now)
        self._restart_counts[name] = history
        return True
    
    async def shutdown(self) -> None:
        """Graceful shutdown of all workers."""
        self._shutdown_event.set()
        
        # Cancel all tasks - TaskGroup handles cleanup
        for task in self._tasks.values():
            task.cancel()
    
    def _topological_sort(self) -> List[str]:
        """Dependency-aware worker ordering."""
        # Kahn's algorithm for topological sort
        in_degree = {name: len(w.dependencies) for name, w in self._workers.items()}
        queue = [name for name, deg in in_degree.items() if deg == 0]
        result = []
        
        while queue:
            name = queue.pop(0)
            result.append(name)
            
            for other_name, worker in self._workers.items():
                if name in worker.dependencies:
                    in_degree[other_name] -= 1
                    if in_degree[other_name] == 0:
                        queue.append(other_name)
        
        if len(result) != len(self._workers):
            raise CircularDependency("Worker dependencies contain cycle")
        
        return result
```

#### Resource Management with Async Context Managers

```python
from contextlib import asynccontextmanager
from typing import AsyncIterator

@asynccontextmanager
async def managed_resource_pool(
    config: PoolConfig
) -> AsyncIterator[ResourcePool]:
    """Manage resource pool lifecycle with structured cleanup."""
    pool = ResourcePool(config)
    
    try:
        await pool.initialize()
        yield pool
    finally:
        # Guaranteed cleanup even on cancellation
        await pool.drain()
        await pool.close()

class ResourcePool:
    """Resource pool with structured lifecycle."""
    
    def __init__(self, config: PoolConfig):
        self._config = config
        self._resources: asyncio.Queue[Resource] = asyncio.Queue()
        self._in_use: Set[Resource] = set()
        self._health_check_task: Optional[asyncio.Task] = None
    
    async def initialize(self) -> None:
        """Initialize pool with health checker."""
        # Create initial resources
        async with asyncio.TaskGroup() as tg:
            for _ in range(self._config.min_size):
                tg.create_task(self._add_resource())
        
        # Start health checker as supervised task
        self._health_check_task = asyncio.create_task(
            self._health_check_loop()
        )
    
    async def drain(self) -> None:
        """Drain pool, waiting for in-use resources."""
        # Stop health checker
        if self._health_check_task:
            self._health_check_task.cancel()
            try:
                await self._health_check_task
            except asyncio.CancelledError:
                pass
        
        # Wait for in-use resources with timeout
        async with asyncio.timeout(self._config.drain_timeout):
            while self._in_use:
                await asyncio.sleep(0.1)
        
        # Close remaining resources
        while not self._resources.empty():
            resource = self._resources.get_nowait()
            await resource.close()
    
    @asynccontextmanager
    async def acquire(self) -> AsyncIterator[Resource]:
        """Acquire resource with automatic return."""
        resource = await self._resources.get()
        self._in_use.add(resource)
        
        try:
            yield resource
        finally:
            self._in_use.discard(resource)
            await self._resources.put(resource)
```

### Timeout and Cancellation Patterns

```python
class DeploymentTimeoutPolicy:
    """Configurable timeout policy for deployments."""
    
    RESOURCE_PROVISION = 300.0      # 5 minutes
    HEALTH_CHECK = 60.0              # 1 minute
    ROLLBACK = 180.0                 # 3 minutes
    TOTAL_DEPLOYMENT = 1800.0        # 30 minutes

async def deploy_with_full_timeout(deployment: Deployment) -> None:
    """Deploy with nested timeout scopes."""
    
    # Outer: Total deployment timeout
    async with asyncio.timeout(DeploymentTimeoutPolicy.TOTAL_DEPLOYMENT):
        
        # Phase 1: Resource provisioning (parallel)
        async with asyncio.TaskGroup() as tg:
            for resource in deployment.resources.values():
                tg.create_task(
                    provision_with_timeout(resource)
                )
        
        # Phase 2: Health checks (sequential)
        for resource in deployment.resources.values():
            async with asyncio.timeout(DeploymentTimeoutPolicy.HEALTH_CHECK):
                await wait_for_healthy(resource)
        
        # Phase 3: Service binding
        async with asyncio.TaskGroup() as tg:
            for service in deployment.services.values():
                tg.create_task(
                    bind_service_with_timeout(service)
                )

async def provision_with_timeout(resource: Resource) -> None:
    """Provision with timeout and cleanup."""
    try:
        async with asyncio.timeout(DeploymentTimeoutPolicy.RESOURCE_PROVISION):
            await provision_resource(resource)
    except asyncio.TimeoutError:
        # Trigger cleanup
        await partial_cleanup(resource)
        raise ResourceProvisionTimeout(resource.id)
    except asyncio.CancelledError:
        # Always clean up on cancellation
        await partial_cleanup(resource)
        raise
```

## Consequences

### Positive
- **No Task Leaks**: Child tasks always complete before parent
- **Clear Lifetimes**: Task relationships explicit in code structure
- **Reliable Cleanup**: `finally` blocks and context managers guaranteed to run
- **Simplified Error Handling**: `ExceptionGroup` captures all failures
- **Cancellation Propagation**: Parent cancellation automatically cancels children

### Negative
- **Python 3.11+ Required**: TaskGroup introduced in 3.11
- **Learning Curve**: Team must understand structured concurrency concepts
- **Constraint**: Cannot fire-and-forget; all tasks must be awaited
- **Migration**: Existing unstructured code needs refactoring

### Mitigations

1. **Compatibility**: Backport using `aiotools.TaskGroup` or `trio` for older Python
2. **Training**: Document patterns in `docs/patterns/async.md`
3. **Linting**: Add custom pylint rule to detect unstructured `create_task()`
4. **Migration**: Gradual module-by-module conversion with tests

## Implementation Plan

### Phase 1: Infrastructure (Week 1)
- [ ] Add `TaskGroup` wrappers for common patterns
- [ ] Implement supervisor pattern for long-running services
- [ ] Create managed resource pool base class
- [ ] Add timeout policy configuration

### Phase 2: Migration (Weeks 2-4)
- [ ] Convert pheno-async orchestration to TaskGroup
- [ ] Update deployment workflows
- [ ] Migrate health check loops
- [ ] Add structured cleanup to all resources

### Phase 3: Hardening (Weeks 5-6)
- [ ] Add cancellation testing to all async code
- [ ] Implement supervisor-based service management
- [ ] Performance profiling and optimization
- [ ] Documentation and training

## Alternatives Considered

### Alternative 1: trio
- **Pros**: Native structured concurrency, excellent design
- **Cons**: Third-party dependency, ecosystem differences from asyncio
- **Verdict**: Rejected; prefer stdlib solution, but trio patterns inform our design

### Alternative 2: Manual Task Management
- **Pros**: Fine-grained control, works with older Python
- **Cons**: Error-prone, verbose, easy to introduce leaks
- **Verdict**: Rejected; explicit goal to move away from manual management

### Alternative 3: Process-based Concurrency
- **Pros**: True parallelism, process isolation
- **Cons**: Higher overhead, serialization costs
- **Verdict**: Rejected; I/O-bound workloads don't benefit

## Related Decisions
- ADR-002: Domain-Driven Design (async repository pattern)
- ADR-004: Vector Database Architecture
- ADR-005: Code Generation from Specifications

## References
- Smith, N. (2018). Notes on Structured Concurrency.
- Sústrik, M. (2018). Structured Concurrency.
- Python 3.11 TaskGroup documentation
- Trio documentation (conceptual reference)
