# PRD — phenotype-task-engine

**Version:** 1.0.0
**Stack:** Python 3.11+, TypeScript
**Repo:** `KooshaPari/phenotype-task-engine`
**Status:** Archived (2026-03-25) — migrated to `packages/phenotype-task/`

---

## Overview

`phenotype-task-engine` is the async task execution engine for the Phenotype agent platform.
It schedules, executes, monitors, and retries tasks submitted by agent workflows. Tasks are
units of work with defined inputs, outputs, retry policies, and timeout contracts.

---

## Epics

### E1 — Task Lifecycle Management
**Goal:** Reliable at-least-once execution with full observability.

#### E1.1 Task Submission
- As an agent, I want to submit a task with a typed payload so the engine can schedule and
  execute it without loss.
- **Acceptance:** `submit(task: Task) -> TaskID` returns a stable ID; task is durable.

#### E1.2 Task State Machine
- **Acceptance:** Tasks transition: `pending → running → (succeeded | failed | timed_out)`.
  No silent terminal states.

#### E1.3 Retry Policy
- As an operator, I want configurable retry with exponential backoff so transient failures
  self-heal.
- **Acceptance:** `max_retries`, `base_delay_s`, `max_delay_s` configurable per task type.

### E2 — Worker Pool
**Goal:** Concurrent task execution with resource limits.

#### E2.1 Worker Concurrency
- **Acceptance:** `max_workers` config controls concurrency; default is `os.cpu_count()`.

#### E2.2 Task Isolation
- **Acceptance:** Each task runs in an isolated async context; unhandled exceptions in one
  task do not crash the pool.

### E3 — Observability
**Goal:** Full task audit trail for agent debugging.

#### E3.1 Structured Logs
- **Acceptance:** Every state transition emits a structured log record with `task_id`, `state`,
  `duration_ms`.

#### E3.2 Metrics
- **Acceptance:** Exposes `tasks_total`, `tasks_failed`, `task_duration_seconds` Prometheus
  metrics on `/metrics`.
