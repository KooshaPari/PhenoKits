# Tasken Specification

> Universal task execution framework with scheduling, workflow orchestration, and plugin support

**Version:** 1.0  
**Status:** Draft  
**Last Updated:** 2026-04-04  
**Language:** Rust  
**Edition:** 2021  
**MSRV:** 1.75  

---

## Overview

Tasken is a Rust framework for task execution with scheduling and workflow orchestration. It follows hexagonal architecture principles to provide a swappable, testable, and extensible task execution engine.

### Design Philosophy

1. **Zero-Cost Abstractions**: Core domain has no infrastructure dependencies
2. **Typed Task Contracts**: Compile-time safety for task inputs and outputs
3. **Multi-Runner Execution**: Same task API, different execution strategies
4. **Plugin Extensibility**: Runtime registration of custom task types
5. **Observable by Default**: Built-in metrics, tracing, and event sourcing

### Target Use Cases

- **CLI Tools**: Sync or background task execution
- **API Services**: Async task handling with timeouts and retries
- **Workflow Engines**: DAG-based task orchestration
- **Cron Replacements**: In-process scheduling without external dependencies
- **Message Processing**: Queue-based worker pool processing

---

## Architecture

### Hexagonal Structure

```
src/
├── domain/              # Pure domain logic (no external dependencies)
│   ├── task.rs          # Task trait and entity
│   ├── workflow.rs      # Workflow and DAG definitions
│   ├── scheduler.rs     # Scheduling logic and types
│   ├── runner.rs        # Runner trait and implementations
│   ├── events.rs        # Domain events
│   ├── errors.rs        # Domain errors
│   └── mod.rs
├── application/         # Application services
│   ├── commands/        # Command handlers
│   ├── queries/         # Query handlers
│   └── services/        # Orchestration services
│       ├── task_service.rs
│       ├── workflow_service.rs
│       └── scheduler_service.rs
├── adapters/            # Infrastructure adapters
│   ├── primary/         # Driving adapters (CLI, HTTP, gRPC)
│   ├── secondary/      # Driven adapters (storage, queue, metrics)
│   └── plugins/         # Plugin system
├── infrastructure/      # Cross-cutting concerns
│   ├── logging.rs       # Structured logging setup
│   ├── tracing.rs       # Distributed tracing setup
│   └── error.rs         # Error handling utilities
└── lib.rs               # Library entry point
```

### Port/Adapter Separation

**Inbound Ports (Driving)**:

- `TaskCommandPort`: Execute tasks, cancel tasks, get status
- `WorkflowCommandPort`: Create workflows, trigger steps, handle results
- `SchedulePort`: Create schedules, cancel schedules, list schedules

**Outbound Ports (Driven)**:

- `StoragePort`: Persist task state, workflow state, schedules
- `QueuePort`: Enqueue/dequeue tasks for worker pool
- `MetricsPort`: Record task metrics (latency, throughput, errors)
- `TracingPort`: Emit spans for task execution

---

## Core Abstractions

### Task Trait

```rust
pub trait Task: Send + Sync {
    type Input: Serialize + DeserializeOwned + Debug;
    type Output: Serialize + DeserializeOwned + Debug;
    
    async fn execute(&self, input: Self::Input) -> Result<Self::Output, TaskError>;
    
    fn metadata(&self) -> TaskMetadata;
}
```

### TaskMetadata

```rust
pub struct TaskMetadata {
    pub id: TaskId,
    pub name: String,
    pub tags: Vec<Tag>,
    pub priority: Priority,
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
}

pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff: Backoff,
    pub retry_on: Vec<TaskErrorKind>,
}

pub enum Backoff {
    Fixed(Duration),
    Linear { base: Duration, max: Duration },
    Exponential { base: Duration, max: Duration, factor: f64 },
}
```

### Runner Types

```rust
pub enum RunnerKind {
    Sync,       // Blocking execution in caller thread
    Async,      // Non-blocking execution on Tokio runtime
    Background, // Detached execution; continues after caller exits
    Queue,     // Enqueued for worker pool processing
}

#[async_trait]
pub trait TaskRunner: Send + Sync {
    async fn execute(&self, task: Arc<dyn Task>) -> TaskResult;
    fn kind(&self) -> RunnerKind;
}

pub struct SyncRunner;
pub struct AsyncRunner;
pub struct BackgroundRunner;
pub struct QueueRunner {
    worker_count: usize,
    queue: Arc<Queue>,
}
```

### Schedule Types

```rust
pub enum Schedule {
    Cron(CronExpression),
    Interval(Duration),
    Delayed(Duration),
    OneShot(DateTime<Utc>),
}

pub struct CronExpression {
    pub second: String,   // 0-59, *
    pub minute: String,   // 0-59, *
    pub hour: String,     // 0-23, *
    pub day_of_month: String,  // 1-31, *
    pub month: String,    // 1-12, *
    pub day_of_week: String,  // 0-6 (Sun-Sat), *
}
```

### Workflow

```rust
pub struct Workflow {
    pub id: WorkflowId,
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub on_failure: OnFailureStrategy,
    pub on_success: OnSuccessStrategy,
}

pub struct WorkflowStep {
    pub id: StepId,
    pub task: Arc<dyn Task>,
    pub depends_on: Vec<StepId>,
    pub condition: Option<Box<dyn Fn(&Value) -> bool + Send>>,
}

pub enum OnFailureStrategy {
    FailFast,
    Continue,
    Run(Vec<StepId>),
    Custom(Arc<dyn Fn(&WorkflowError) -> OnFailureDecision>),
}

pub enum OnSuccessStrategy {
    Complete,
    Continue,
    Run(Vec<StepId>),
}
```

---

## Feature Checklist

### Task Abstraction

- [x] Task trait with typed I/O
- [x] TaskMetadata with tags and priority
- [x] TaskId generation (UUID v4)
- [x] Task events (Scheduled, Started, Completed, Failed, Retrying)

### Scheduling

- [x] Cron expression parsing and scheduling
- [x] Interval-based scheduling
- [x] One-shot delayed execution
- [x] Schedule cancellation
- [x] Schedule listing and inspection
- [x] Timezone support

### Execution

- [x] SyncRunner (blocking)
- [x] AsyncRunner (Tokio async)
- [ ] BackgroundRunner (detached)
- [ ] QueueRunner (worker pool)

### Workflows

- [x] Workflow struct with DAG
- [x] Sequential execution
- [x] Parallel execution (fan-out)
- [x] Conditional branching
- [ ] Workflow persistence
- [ ] Workflow recovery

### Reliability

- [x] Retry policies (fixed, exponential backoff)
- [x] Timeout enforcement
- [ ] Error aggregation
- [ ] Circuit breaker
- [ ] Bulkhead isolation

### Observability

- [ ] Prometheus metrics adapter
- [ ] OpenTelemetry tracing adapter
- [ ] Structured logging
- [ ] Event sourcing

### Persistence (Adapters)

- [ ] In-memory storage adapter
- [ ] Redis storage adapter

### Plugin System

- [ ] Plugin trait definition
- [ ] Plugin registry
- [ ] Built-in plugins (Cron, Interval, Webhook, HTTP)

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|------------|
| Task execution latency (P50) | < 1μs | Microbenchmark |
| Task execution latency (P99) | < 10μs | Microbenchmark |
| Scheduling precision | ±1ms | 1000 iterations |
| Throughput (SyncRunner) | 1M tasks/sec | 1M trivial tasks |
| Memory overhead (idle) | < 10MB | With scheduler running |
| Workflow latency (10 tasks, sequential) | < 100ms | 10×10ms tasks |
| Workflow latency (10 tasks, parallel) | < 100ms | 10×10ms tasks, fan-out |

---

## Configuration

### Default Configuration

```rust
pub struct Config {
    pub runner: RunnerKind,
    pub scheduler: SchedulerConfig,
    pub retry: RetryConfig,
    pub logging: LoggingConfig,
    pub metrics: Option<MetricsConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            runner: RunnerKind::Async,
            scheduler: SchedulerConfig {
                max_schedules: 10_000,
                tick_interval: Duration::from_millis(100),
            },
            retry: RetryConfig {
                max_attempts: 3,
                default_backoff: Backoff::Exponential {
                    base: Duration::from_millis(100),
                    max: Duration::from_secs(30),
                    factor: 2.0,
                },
            },
            logging: LoggingConfig::default(),
            metrics: None,
        }
    }
}
```

### Environment Variables

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `TASKEN_RUNNER` | str | "async" | Runner kind: sync, async, background, queue |
| `TASKEN_LOG_LEVEL` | str | "info" | Log level: trace, debug, info, warn, error |
| `TASKEN_METRICS_ENABLED` | bool | false | Enable Prometheus metrics |
| `TASKEN_METRICS_PORT` | u16 | 9090 | Metrics exporter port |
| `TASKEN_SCHEDULER_MAX` | u32 | 10000 | Maximum concurrent schedules |

---

## Error Handling

### Error Hierarchy

```rust
pub enum TaskenError {
    Task(TaskError),
    Scheduler(SchedulerError),
    Workflow(WorkflowError),
    Storage(StorageError),
    Configuration(ConfigurationError),
}

pub enum TaskError {
    Execute(String),
    Timeout(Duration),
    Cancelled,
    RetryExhausted { attempts: u32, last_error: Box<TaskError> },
}

pub enum SchedulerError {
    ScheduleNotFound(ScheduleId),
    InvalidCronExpression(String),
    SchedulerFull,
}

pub enum WorkflowError {
    StepNotFound(StepId),
    DependencyCycle(Vec<StepId>),
    StepFailed(StepId, Box<TaskError>),
    WorkflowCancelled(WorkflowId),
}
```

---

## Security Considerations

1. **Input Validation**: All task inputs validated before execution
2. **Timeout Enforcement**: Tasks cannot run indefinitely
3. **Resource Limits**: Configurable limits on concurrent tasks/schedules
4. **Plugin Sandboxing**: Future work - untrusted plugins in isolation

---

## Compatibility

### MSRV Policy

- Minimum Supported Rust Version: 1.75
- MSRV increases only on minor version bumps
- Changes announced 2 releases in advance

### Async Runtime Compatibility

- Primary: Tokio 1.x
- Support for async-std: Future consideration

---

## License

MIT OR Apache-2.0

---

## Document Information

**Version:** 1.0  
**Author:** Tasken Architecture Team  
**Review Cycle:** Quarterly  
**Traceability:** `/// @trace SPEC-001`
