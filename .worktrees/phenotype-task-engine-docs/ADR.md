# Architecture Decision Records — phenotype-task-engine

---

## ADR-001 — Python asyncio as Execution Runtime

**Date:** 2025-09-01
**Status:** Accepted

### Context
The task engine needed to support hundreds of concurrent I/O-bound tasks (HTTP calls, subprocess
invocations). Thread-per-task (ThreadPoolExecutor) was ruled out due to GIL contention on
CPU-bound tasks. Celery was evaluated but introduces Redis/RabbitMQ infrastructure complexity.

### Decision
Use Python asyncio with a bounded `asyncio.Semaphore` for concurrency control. `asyncio.Queue`
provides the task buffer.

### Consequences
- Zero external broker dependency in the base engine; persistence is pluggable.
- CPU-bound tasks must be offloaded via `run_in_executor` to avoid blocking the event loop.

---

## ADR-002 — Hexagonal Architecture: TaskStore Port

**Date:** 2025-09-10
**Status:** Accepted

### Context
Per Phenotype mandate, persistence is a secondary adapter behind a port. The engine's domain
logic must not import SQLAlchemy, Redis, or any specific persistence library.

### Decision
Define a `TaskStore` Protocol:
```python
class TaskStore(Protocol):
    async def persist(self, task: Task) -> None: ...
    async def update_state(self, task_id: TaskID, state: TaskState) -> None: ...
    async def fetch(self, task_id: TaskID) -> Task: ...
```
`InMemoryTaskStore` and `RedisTaskStore` are adapters.

### Consequences
- Tests use `InMemoryTaskStore` with no network.
- Production uses `RedisTaskStore`; swap requires zero engine code change.

---

## ADR-003 — UUID v7 for Task IDs

**Date:** 2025-09-15
**Status:** Accepted

### Context
UUID v4 is random and un-sortable, complicating time-ordered queries. ULID was considered but
less standard. UUID v7 (time-ordered) is now in the Python `uuid` stdlib (3.11+).

### Decision
Use `uuid.uuid7()` for `TaskID` generation to enable time-ordered sorting without a secondary
`created_at` index.

### Consequences
- Task IDs sort chronologically in lexicographic order.
- Requires Python 3.11+; backport available via `uuid7` PyPI package for 3.10.
