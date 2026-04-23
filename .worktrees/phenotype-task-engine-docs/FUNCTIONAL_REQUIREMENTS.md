# Functional Requirements — phenotype-task-engine

**Version:** 1.0.0
**Traces to:** PRD epics E1–E3

---

## FR-TASK-001 — Durable Task Submission
**SHALL** persist submitted tasks to a durable store before returning a `TaskID` so that
engine restarts do not lose submitted work.
**Traces to:** E1.1

## FR-TASK-002 — Unique Task IDs
**SHALL** generate globally unique `TaskID` values (UUID v7 for time-ordered sorting).
**Traces to:** E1.1

## FR-TASK-003 — State Machine Enforcement
**SHALL** enforce the task state machine: only valid transitions
(pending→running, running→succeeded, running→failed, running→timed_out) are permitted;
invalid transitions raise `InvalidStateTransition`.
**Traces to:** E1.2

## FR-TASK-004 — Exponential Backoff Retry
**SHALL** retry failed tasks with exponential backoff: `delay = min(base * 2^attempt, max)`.
**Traces to:** E1.3

## FR-TASK-005 — Max Retry Enforcement
**SHALL** move a task to permanent `failed` state after exhausting `max_retries` attempts.
**Traces to:** E1.3

## FR-TASK-006 — Configurable Worker Concurrency
**SHALL** limit concurrent task execution to `max_workers` (default: `os.cpu_count()`).
Attempts to run more tasks than `max_workers` queue in `pending`.
**Traces to:** E2.1

## FR-TASK-007 — Task Isolation
**SHALL** run each task in an isolated async context such that an unhandled exception moves
only that task to `failed` and does not terminate the worker pool.
**Traces to:** E2.2

## FR-TASK-008 — Structured State Transition Logs
**SHALL** emit a structured log record on every state transition containing `task_id`,
`old_state`, `new_state`, `duration_ms`, and `error` (if failed).
**Traces to:** E3.1

## FR-TASK-009 — Prometheus Metrics Endpoint
**SHALL** expose `tasks_total{state}`, `tasks_failed_total`, and `task_duration_seconds`
as Prometheus metrics on a configurable HTTP endpoint (default `:9090/metrics`).
**Traces to:** E3.2
