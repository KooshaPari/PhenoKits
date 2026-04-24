# phenotype-agent-core

> Agent core with skill orchestration, task execution, and environment management

## Overview

`phenotype-agent-core` is a Go-based agent runtime that provides:
- **Skill Orchestration**: Modular, composable skills with versioning
- **Task Management**: Stateful task execution with persistence
- **Tool Gateway**: Unified interface for tool execution
- **Environment Management**: Sandboxes, containers, and VMs
- **Event-Driven**: Real-time event streaming

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Agent Core Architecture                               │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Agent Runtime                                     │   │
│  │                                                                      │   │
│  │   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐    │   │
│  │   │  Skill Registry │  │  Task Executor  │  │   Tool Gateway  │    │   │
│  │   └────────┬────────┘  └────────┬────────┘  └────────┬────────┘    │   │
│  │            │                     │                     │               │   │
│  │            └─────────────────────┼─────────────────────┘               │   │
│  │                                  │                                   │   │
│  │                        ┌─────────┴─────────┐                       │   │
│  │                        │  Execution Engine  │                       │   │
│  │                        └─────────┬─────────┘                       │   │
│  └────────────────────────────────┼───────────────────────────────────┘   │
│                                     │                                        │
│                                     ▼                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Environment Layer                                  │   │
│  │                                                                      │   │
│  │   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐    │   │
│  │   │  Sandbox Env    │  │  MicroVM Env   │  │  Container Env  │    │   │
│  │   │  (bwrap/firejail)│ │ (Firecracker)  │  │  (runc)        │    │   │
│  │   └─────────────────┘  └─────────────────┘  └─────────────────┘    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Features

### Skill System
- YAML-based skill definitions
- Hot-reload skill registration
- Version management
- Action chaining
- Event triggers

### Task Management
- Stateful task execution
- Task persistence
- Priority scheduling
- Retry with backoff
- Cancellation support

### Tool Gateway
- Unified tool interface
- Permission system
- Resource limits
- Audit logging
- Sandbox integration

### Environment Tiers
| Tier | Technology | Startup | Isolation | Use Case |
|------|------------|---------|-----------|----------|
| Sandbox | bwrap, firejail | <100ms | Process | Fast tasks |
| Container | runc, containerd | <1s | Namespace | Standard |
| MicroVM | Firecracker | <500ms | KVM | Secure |

## Quick Start

### Installation

```bash
go install github.com/kooshapari/phenotype-agent-core@latest
```

### Configuration

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

environment:
  default_tier: "sandbox"
  sandbox:
    runtime: "bwrap"
  microvm:
    runtime: "firecracker"
    memory: "256MB"
    vcpus: 1

storage:
  type: "sqlite"
  path: "./agent.db"
```

### Run

```bash
# Start the agent core
agent-core serve

# Create a task
agent-core task create --skill http --input '{"url": "https://example.com"}'

# List tasks
agent-core task list

# Stream events
agent-core events
```

## CLI Commands

```bash
# Task commands
agent-core task create --skill <name> --input <json>
agent-core task list
agent-core task get <id>
agent-core task cancel <id>
agent-core task retry <id>

# Skill commands
agent-core skill list
agent-core skill get <name>
agent-core skill reload

# Tool commands
agent-core tool list
agent-core tool call <name> --input <json>

# Environment commands
agent-core env list
agent-core env create --tier <sandbox|container|microvm>
agent-core env delete <id>

# Event commands
agent-core events
agent-core events --task <id>
```

## API

### REST API

```bash
# Create task
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"skill": "http", "input": {"url": "https://example.com"}}'

# List tasks
curl http://localhost:8080/api/v1/tasks

# Get task
curl http://localhost:8080/api/v1/tasks/<id>

# Stream events
curl http://localhost:8080/api/v1/events
```

### gRPC

```protobuf
service AgentService {
    rpc CreateTask(CreateTaskRequest) returns (CreateTaskResponse);
    rpc GetTask(GetTaskRequest) returns (GetTaskResponse);
    rpc ListTasks(ListTasksRequest) returns (ListTasksResponse);
    rpc InvokeSkill(InvokeSkillRequest) returns (InvokeSkillResponse);
    rpc ExecuteTool(ExecuteToolRequest) returns (ExecuteToolResponse);
}
```

## Documentation

- [Specification](SPEC.md) - Detailed system specification
- [Implementation Plan](PLAN.md) - Phase-based implementation roadmap
- [API Reference](docs/api.md) - REST and gRPC API documentation
- [Architecture](docs/architecture.md) - System architecture deep-dive

## Development

### Prerequisites

- Go 1.23+
- SQLite or PostgreSQL
- (Optional) NATS for event streaming
- (Optional) Firecracker for MicroVM support

### Building

```bash
# Build
go build -o agent-core ./cmd/server

# Run tests
go test -v ./...

# Run benchmarks
go test -bench=. -benchmem ./...

# Lint
golangci-lint run
```

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## Performance

| Metric | Target |
|--------|--------|
| Task creation | <10ms |
| Skill resolution | <5ms |
| Tool execution (simple) | <50ms |
| Sandbox creation | <100ms |
| MicroVM creation | <500ms |
| Concurrent tasks | 100+ |

## License

MIT License - see [LICENSE](LICENSE) for details.

## References

- [Go Concurrency Patterns](https://go.dev/blog/concurrency)
- [Task Queue Design](https://brandur.org/fragment/task-queues)
- [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)
- [MicroVM Design](https://firecracker-microvm.github.io/)
