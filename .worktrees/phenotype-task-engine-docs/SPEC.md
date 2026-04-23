# phenotype-task-engine Specification

> Distributed task execution engine with priority scheduling, retries, and state persistence

## Overview

`phenotype-task-engine` provides a distributed task execution engine:
- **Priority Scheduling**: Multi-level priority queues
- **State Persistence**: SQLite/PostgreSQL storage
- **Retry Logic**: Configurable retry with backoff
- **Distributed Execution**: Worker pool with leader election
- **Event Streaming**: Real-time task events

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Task Engine Architecture                               │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Task Scheduler                                   │   │
│  │                                                                      │   │
│  │   ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐    │   │
│  │   │ P0/Crit │ │ P1/High│ │ P2/Med │ │ P3/Low │ │ P4/Batch│    │   │
│  │   └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘    │   │
│  │        └────────────┴────────────┴────────────┴────────────┘         │   │
│  │                              │                                          │   │
│  └─────────────────────────────┼────────────────────────────────────────┘   │
│                                │                                              │
│                                ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Task Queue                                       │   │
│  │                                                                      │   │
│  │   ┌─────────────────────────────────────────────────────────────┐   │   │
│  │   │  Priority Heap + Delayed Execution + Rate Limiting           │   │   │
│  │   └─────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────┬────────────────────────────────────────┘   │
│                                │                                              │
│                                ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Worker Pool                                     │   │
│  │                                                                      │   │
│  │   ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐      │   │
│  │   │Worker 1│ │Worker 2│ │Worker 3│ │Worker 4│ │Worker N│      │   │
│  │   └────────┘ └────────┘ └────────┘ └────────┘ └────────┘      │   │
│  └─────────────────────────────┬────────────────────────────────────────┘   │
│                                │                                              │
│                                ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Task Executor                                   │   │
│  │                                                                      │   │
│  │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐               │   │
│  │   │  Task Handler│ │  Retry     │ │   Result   │               │   │
│  │   │             │ │  Handler   │ │   Handler  │               │   │
│  │   └─────────────┘ └─────────────┘ └─────────────┘               │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Task Model

```go
type Task struct {
    ID          string
    Name        string
    Payload     []byte
    Priority    Priority
    State       TaskState
    Attempts    int
    MaxAttempts int
    Timeout     time.Duration
    Retries     int
    CreatedAt   time.Time
    ScheduledAt time.Time
    StartedAt   *time.Time
    CompletedAt *time.Time
    Error       *TaskError
    Result      []byte
    Metadata    map[string]string
}

type Priority int

const (
    PriorityCritical Priority = 0  // P0 - Immediate
    PriorityHigh    Priority = 25 // P1 - High
    PriorityNormal  Priority = 50 // P2 - Normal
    PriorityLow    Priority = 75 // P3 - Low
    PriorityBatch   Priority = 100 // P4 - Batch/Background
)

type TaskState string

const (
    TaskStatePending   TaskState = "pending"
    TaskStateScheduled TaskState = "scheduled"
    TaskStateRunning   TaskState = "running"
    TaskStateCompleted TaskState = "completed"
    TaskStateFailed    TaskState = "failed"
    TaskStateCancelled TaskState = "cancelled"
    TaskStateDead      TaskState = "dead"
)
```

## Scheduling

```go
type Scheduler interface {
    Schedule(task *Task) error
    ScheduleDelayed(task *Task, delay time.Duration) error
    Cancel(taskID string) error
    Reschedule(taskID string, at time.Time) error
}

type PriorityScheduler struct {
    queues map[Priority]*PriorityQueue
    mu     sync.RWMutex
}

func (s *PriorityScheduler) Schedule(task *Task) error {
    queue := s.queues[task.Priority]
    return queue.Push(task)
}
```

## Worker Pool

```go
type WorkerPool struct {
    workers    int
    executors  map[string]TaskExecutor
    middleware []WorkerMiddleware
    results    chan *TaskResult
    wg         sync.WaitGroup
}

type TaskExecutor func(ctx context.Context, task *Task) (*TaskResult, error)

type WorkerMiddleware func(next TaskExecutor) TaskExecutor

// Middleware
func LoggingMiddleware(logger Logger) WorkerMiddleware {
    return func(next TaskExecutor) TaskExecutor {
        return func(ctx context.Context, task *Task) (*TaskResult, error) {
            logger.Info("executing task", "id", task.ID)
            result, err := next(ctx, task)
            if err != nil {
                logger.Error("task failed", "id", task.ID, "error", err)
            }
            return result, err
        }
    }
}

func MetricsMiddleware(meter Metrics) WorkerMiddleware {
    return func(next TaskExecutor) TaskExecutor {
        return func(ctx context.Context, task *Task) (*TaskResult, error) {
            start := time.Now()
            result, err := next(ctx, task)
            meter.RecordTaskDuration(task.Name, time.Since(start))
            return result, err
        }
    }
}
```

## Retry Logic

```go
type RetryPolicy struct {
    MaxAttempts int
    Backoff    BackoffStrategy
    Jitter     bool
    OnRetry    func(attempt int, err error)
}

type BackoffStrategy interface {
    Duration(attempt int) time.Duration
}

type ExponentialBackoff struct {
    Base    time.Duration
    Factor  float64
    Max     time.Duration
}

func (b *ExponentialBackoff) Duration(attempt int) time.Duration {
    d := time.Duration(float64(b.Base) * math.Pow(b.Factor, float64(attempt-1)))
    if d > b.Max {
        return b.Max
    }
    return d
}

type LinearBackoff struct {
    Base    time.Duration
    Max     time.Duration
}

## Storage Backends

| Backend | Use Case | Persistence | Performance |
|---------|----------|-------------|-------------|
| **SQLite** | Single node, development | Disk | 10K tasks/sec |
| **PostgreSQL** | Production, multi-node | Disk | 50K tasks/sec |
| **Redis** | High-throughput, caching | Memory + AOF | 100K tasks/sec |
| **etcd** | Distributed coordination | Disk | 20K tasks/sec |

## Observability

### Metrics

```go
type TaskMetrics struct {
    Submitted      counter.Counter
    Completed      counter.Counter
    Failed         counter.Counter
    Retried        counter.Counter
    Duration       histogram.Histogram
    QueueDepth     gauge.Gauge
    WorkerUtil     gauge.Gauge
}
```

### Events

| Event | Payload | Use Case |
|-------|---------|----------|
| TaskSubmitted | Task metadata | Audit logging |
| TaskStarted | Task + Worker | Tracing |
| TaskCompleted | Task + Result | Notifications |
| TaskFailed | Task + Error | Alerting |
| WorkerJoined | Worker info | Scaling |
| WorkerLeft | Worker info | Recovery |

## Reference URLs (40+)

### Task Queue Systems

| # | Project | URL | Description |
|---|---------|-----|-------------|
| 1 | Celery | https://docs.celeryproject.org/ | Distributed task queue |
| 2 | BullMQ | https://bullmq.io/ | Node.js queue system |
| 3 | Sidekiq | https://sidekiq.org/ | Ruby background jobs |
| 4 | Delayed Job | https://github.com/collectiveidea/delayed_job | Ruby jobs |
| 5 | Hangfire | https://www.hangfire.io/ | .NET background jobs |
| 6 | Faktory | https://contribsys.com/faktory/ | Language-agnostic jobs |
| 7 | rq | https://python-rq.org/ | Python Redis queue |
| 8 | NSQ | https://nsq.io/ | Real-time messaging |
| 9 | NATS | https://nats.io/ | Cloud-native messaging |
| 10 | Kafka | https://kafka.apache.org/ | Distributed streaming |

---

*Last updated: 2026-04-02*

type ConstantBackoff struct {
    Duration time.Duration
}
```

## Storage

```sql
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    payload BLOB,
    priority INTEGER NOT NULL DEFAULT 50,
    state TEXT NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    timeout INTEGER NOT NULL DEFAULT 300000,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    scheduled_at TIMESTAMP,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    error TEXT,
    result BLOB,
    metadata TEXT
);

CREATE INDEX idx_tasks_state ON tasks(state);
CREATE INDEX idx_tasks_priority ON tasks(priority, scheduled_at);
CREATE INDEX idx_tasks_scheduled ON tasks(scheduled_at) WHERE state = 'scheduled';
```

## API

```protobuf
service TaskService {
    rpc SubmitTask(SubmitTaskRequest) returns (SubmitTaskResponse);
    rpc GetTask(GetTaskRequest) returns (GetTaskResponse);
    rpc ListTasks(ListTasksRequest) returns (ListTasksResponse);
    rpc CancelTask(CancelTaskRequest) returns (CancelTaskResponse);
    rpc RetryTask(RetryTaskRequest) returns (RetryTaskResponse);
}

service WorkerService {
    rpc RegisterWorker(RegisterWorkerRequest) returns (RegisterWorkerResponse);
    rpc Heartbeat(HeartbeatRequest) returns (HeartbeatResponse);
    rpc ClaimTask(ClaimTaskRequest) returns (ClaimTaskResponse);
    rpc UpdateTask(UpdateTaskRequest) returns (UpdateTaskResponse);
}
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Task submission | <5ms |
| Task scheduling | <10ms |
| Worker startup | <100ms |
| Task execution (simple) | <50ms |
| Throughput | 10K tasks/sec |
| Concurrent workers | 100+ |
