# phenotype-agent-core Specification

> Agent core with skill orchestration, task execution, and environment management

## Overview

`phenotype-agent-core` is a Go-based agent runtime that provides:
- Skill orchestration and execution
- Task management and state machines
- Environment management (sandbox, VM, container)
- Tool integration framework
- Event-driven architecture

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Agent Core Architecture                               │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Agent Runtime                                     │   │
│  │                                                                      │   │
│  │   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐    │   │
│  │   │  Skill Registry │  │ Task Executor   │  │  Tool Gateway  │    │   │
│  │   └────────┬────────┘  └────────┬────────┘  └────────┬────────┘    │   │
│  │            │                     │                     │               │   │
│  │            └─────────────────────┼─────────────────────┘               │   │
│  │                                  │                                   │   │
│  │                    ┌─────────────┴─────────────┐                   │   │
│  │                    │      Execution Engine      │                   │   │
│  │                    └─────────────┬─────────────┘                   │   │
│  └──────────────────────────────────┼───────────────────────────────────┘   │
│                                     │                                        │
│                                     ▼                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Environment Layer                                 │   │
│  │                                                                      │   │
│  │   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐    │   │
│  │   │  Sandbox Env    │  │  VM Env        │  │  Container Env  │    │   │
│  │   │  (bwrap/firejail)│ │ (Firecracker)  │  │  (runc)        │    │   │
│  │   └─────────────────┘  └─────────────────┘  └─────────────────┘    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components

### 1. Skill Registry

Skills are modular units of capability that can be orchestrated together.

```go
type Skill struct {
    Name        string
    Description string
    Version     string
    Actions     []Action
    Triggers   []Trigger
    Resources  ResourceRequirements
}

type Action struct {
    Name   string
    Input  Schema
    Output Schema
    Execute func(ctx context.Context, input interface{}) (interface{}, error)
}

type Trigger struct {
    Type   string  // "event", "schedule", "manual"
    Config interface{}
}
```

### 2. Task Executor

Task execution with state management and persistence.

```go
type Task struct {
    ID          string
    Skill       string
    Input       map[string]interface{}
    State       TaskState
    Result      *TaskResult
    CreatedAt   time.Time
    StartedAt   *time.Time
    CompletedAt *time.Time
}

type TaskState string

const (
    TaskStatePending   TaskState = "pending"
    TaskStateRunning   TaskState = "running"
    TaskStateCompleted TaskState = "completed"
    TaskStateFailed    TaskState = "failed"
    TaskStateCancelled TaskState = "cancelled"
)
```

### 3. Tool Gateway

Unified interface for tool execution.

```go
type Tool interface {
    Name() string
    Schema() ToolSchema
    Execute(ctx context.Context, input interface{}) (interface{}, error)
}

type ToolSchema struct {
    Name        string
    Description string
    Parameters  map[string]Parameter
    Returns     map[string]Parameter
}

type Parameter struct {
    Type        string
    Description string
    Required    bool
    Default     interface{}
}
```

### 4. Execution Engine

Core execution logic with retry, timeout, and error handling.

```go
type ExecutionEngine struct {
    TaskExecutor    *TaskExecutor
    SkillRegistry  *SkillRegistry
    ToolGateway    *ToolGateway
    RetryPolicy    RetryPolicy
    TimeoutPolicy  TimeoutPolicy
}

type RetryPolicy struct {
    MaxAttempts int
    Backoff    time.Duration
    Jitter     bool
}

type TimeoutPolicy struct {
    Default    time.Duration
    PerSkill  map[string]time.Duration
}
```

### 5. Environment Manager

Manages execution environments (sandboxes, VMs, containers).

```go
type EnvironmentManager struct {
    DefaultTier  EnvironmentTier
    Adapters    map[EnvironmentTier]EnvironmentAdapter
}

type EnvironmentTier string

const (
    TierSandbox   EnvironmentTier = "sandbox"   // bwrap, firejail
    TierMicroVM   EnvironmentTier = "microvm"   // Firecracker
    TierContainer EnvironmentTier = "container" // runc, containerd
    TierVM       EnvironmentTier = "vm"         // Full VM
)

type EnvironmentAdapter interface {
    Create(ctx context.Context, config EnvironmentConfig) (string, error)
    Start(ctx context.Context, id string) error
    Stop(ctx context.Context, id string) error
    Delete(ctx context.Context, id string) error
    Exec(ctx context.Context, id string, cmd []string) ([]byte, error)
}
```

## Data Flow

```
1. Skill Request
   │
   ▼
2. Skill Resolution (Registry lookup)
   │
   ▼
3. Task Creation & Persistence
   │
   ▼
4. Environment Allocation
   │
   ▼
5. Action Execution Loop
   │   │
   │   ├── Tool Calls (Tool Gateway)
   │   │
   │   ├── Environment Operations
   │   │
   │   └── State Updates
   │
   ▼
6. Result Aggregation
   │
   ▼
7. Task Completion & Cleanup
```

## Event System

```go
type EventType string

const (
    EventTaskCreated     EventType = "task:created"
    EventTaskStarted    EventType = "task:started"
    EventTaskProgress   EventType = "task:progress"
    EventTaskCompleted  EventType = "task:completed"
    EventTaskFailed     EventType = "task:failed"
    EventSkillInvoked   EventType = "skill:invoked"
    EventToolCalled     EventType = "tool:called"
    EventEnvCreated     EventType = "env:created"
    EventEnvDestroyed   EventType = "env:destroyed"
)

type Event struct {
    Type      EventType
    TaskID    string
    Timestamp time.Time
    Data      interface{}
}
```

## API Design

### REST Endpoints

```
POST   /api/v1/tasks              Create a new task
GET    /api/v1/tasks              List all tasks
GET    /api/v1/tasks/:id          Get task details
POST   /api/v1/tasks/:id/cancel  Cancel a task
POST   /api/v1/tasks/:id/retry   Retry a failed task

GET    /api/v1/skills             List all skills
POST   /api/v1/skills             Register a new skill
GET    /api/v1/skills/:name       Get skill details

GET    /api/v1/tools              List all tools
POST   /api/v1/tools/call         Execute a tool

GET    /api/v1/environments       List environments
POST   /api/v1/environments      Create environment
DELETE /api/v1/environments/:id   Delete environment

GET    /api/v1/events            Stream events (SSE)
```

### gRPC Services

```protobuf
service AgentService {
    rpc CreateTask(CreateTaskRequest) returns (CreateTaskResponse);
    rpc GetTask(GetTaskRequest) returns (GetTaskResponse);
    rpc ListTasks(ListTasksRequest) returns (ListTasksResponse);
    rpc CancelTask(CancelTaskRequest) returns (CancelTaskResponse);

    rpc InvokeSkill(InvokeSkillRequest) returns (InvokeSkillResponse);
    rpc ListSkills(ListSkillsRequest) returns (ListSkillsResponse);

    rpc ExecuteTool(ExecuteToolRequest) returns (ExecuteToolResponse);
}

service EventService {
    rpc StreamEvents(StreamEventsRequest) returns (stream Event);
}
```

## Configuration

```yaml
# config.yaml
agent:
  name: "agent-core"
  version: "0.1.0"

execution:
  max_concurrent_tasks: 10
  default_timeout: 5m
  retry:
    max_attempts: 3
    backoff: 1s
    jitter: true

environment:
  default_tier: "sandbox"
  sandbox:
    runtime: "bwrap"
    timeout: 30m
  microvm:
    runtime: "firecracker"
    memory: "256MB"
    vcpus: 1
    timeout: 1h

storage:
  type: "sqlite"  # sqlite, postgres
  path: "/var/lib/phenotype/agent.db"

events:
  type: "nats"  # nats, kafka, in-memory
  url: "nats://localhost:4222"

telemetry:
  enabled: true
  otlp_endpoint: "localhost:4317"
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Task creation latency | < 10ms |
| Skill resolution | < 5ms |
| Tool execution (simple) | < 50ms |
| Environment creation (sandbox) | < 100ms |
| Environment creation (microvm) | < 500ms |
| Concurrent tasks | 100+ |
| Event throughput | 10K/sec |

## Security

- All execution in isolated environments (sandbox/VM)
- Resource limits (CPU, memory, disk)
- Network isolation by default
- Audit logging for all operations
- Secrets management integration

## Observability

- OpenTelemetry traces for all operations
- Prometheus metrics for task execution
- Structured logging (JSON)
- Health check endpoints

## References

- [Go Concurrency Patterns](https://go.dev/blog/concurrency)
- [Context Package](https://pkg.go.dev/context)
- [Worker Pool Pattern](https://github.com/golang-design-patterns/tree/master/workerpool)
- [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)

## TODO

- [ ] Implement skill registry with hot-reload
- [ ] Add gRPC streaming for task progress
- [ ] Integrate with Temporal for workflow
- [ ] Add skill versioning
- [ ] Implement priority scheduling
- [ ] Add resource quotas per tenant
