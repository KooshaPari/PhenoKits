# Phenotype Task Engine

> Async Task Orchestration with Dependency Management

A high-performance task execution engine supporting complex workflows, dependency graphs, and distributed execution.

## Features

- **Async/Await**: Full async execution with tokio
- **Dependency Graph**: DAG-based task dependencies
- **Retry Logic**: Exponential backoff with jitter
- **Parallel Execution**: Maximize resource utilization
- **Distributed**: Support for multi-node execution
- **Observability**: Tracing and metrics for all tasks

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      Phenotype Task Engine                                  │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Task Scheduler                               │   │
│  │                                                                      │   │
│  │   Priority Queue → Dependency Resolution → Resource Allocation        │   │
│  │                                                                      │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Task Executor                                │   │
│  │                                                                      │   │
│  │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │   │
│  │   │  Worker 1    │  │  Worker 2    │  │  Worker N    │         │   │
│  │   │  (CPU: 2)    │  │  (CPU: 2)    │  │  (CPU: 2)    │         │   │
│  │   └──────────────┘  └──────────────┘  └──────────────┘         │   │
│  │                                                                      │   │
│  │   Worker Pool with Resource Limits                                   │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Task Queue                                   │   │
│  │                                                                      │   │
│  │   Redis / NATS / In-Memory with Backpressure                        │   │
│  │                                                                      │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Task Storage                                 │   │
│  │                                                                      │   │
│  │   SQLite / PostgreSQL with Event Sourcing                           │   │
│  │                                                                      │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Quick Start

```bash
# Install
cargo install phenotype-task-engine

# Run a task
pte run --task-file task.yaml

# Start the server
pte server --port 8080

# Submit a task
pte submit --task '{"name": "hello", "command": "echo hello"}'

# List tasks
pte list --status running
```

## Task Types

| Type | Description | Example |
|------|-------------|---------|
| `shell` | Shell command execution | `ls -la` |
| `http` | HTTP request | `GET https://api.example.com` |
| `skill` | Skill execution | Run a phenotype skill |
| `docker` | Container execution | Docker run |
| `vm` | VM execution | Firecracker microVM |
| `dag` | Workflow DAG | Complex pipeline |

## Creating a Task

```yaml
# task.yaml
name: data_pipeline
version: "1.0"
description: Process data pipeline

vars:
  input_bucket: s3://my-bucket/input
  output_bucket: s3://my-bucket/output

tasks:
  - name: download
    type: shell
    command: aws s3 cp {{input_bucket}} /tmp/data
    
  - name: process
    type: skill
    skill: data_processor
    input:
      source: /tmp/data
    depends_on: [download]
    
  - name: upload
    type: shell
    command: aws s3 cp /tmp/output {{output_bucket}}
    depends_on: [process]
    
  - name: notify
    type: http
    method: POST
    url: https://hooks.slack.com/...
    body:
      text: "Pipeline complete!"
    depends_on: [upload]
```

## Configuration

```yaml
# ~/.config/phenotype/task-engine.yaml
server:
  host: 0.0.0.0
  port: 8080
  workers: 4

scheduler:
  strategy: priority  # priority, fifo, round_robin
  max_concurrent: 100
  
executor:
  type: local  # local, distributed
  workers: 8
  
retry:
  max_attempts: 3
  backoff: exponential
  base_delay: 1s
  max_delay: 60s
  
storage:
  type: sqlite  # sqlite, postgres
  path: ~/.local/share/pte/tasks.db
  
queue:
  type: memory  # memory, redis, nats
  
observability:
  tracing: true
  metrics: true
  jaeger_endpoint: http://localhost:14268
```

## Task States

```
Pending → Scheduled → Running → Completing → Completed
                          ↓
                     Failed (with retry)
                          ↓
                     Cancelled
```

## DAG Workflows

```yaml
# complex_workflow.yaml
name: ml_training_pipeline

tasks:
  - name: fetch_data
    type: shell
    command: python fetch.py
    
  - name: preprocess_train
    type: skill
    skill: preprocessor
    input:
      split: train
    depends_on: [fetch_data]
    
  - name: preprocess_test
    type: skill
    skill: preprocessor
    input:
      split: test
    depends_on: [fetch_data]
    
  - name: train_model
    type: docker
    image: pytorch/pytorch:latest
    command: python train.py
    resources:
      gpu: 1
      memory: 32GB
    depends_on: [preprocess_train]
    
  - name: evaluate
    type: skill
    skill: evaluator
    depends_on: [train_model, preprocess_test]
    
  - name: deploy
    type: http
    method: POST
    url: https://api.prod/model/deploy
    depends_on: [evaluate]
    condition: "{{evaluate.accuracy}} > 0.95"
```

## Distributed Execution

```yaml
# distributed.yaml
executor:
  type: distributed
  
cluster:
  nodes:
    - id: node-1
      host: 10.0.1.10
      capacity:
        cpu: 16
        memory: 64GB
        gpu: 4
        
    - id: node-2
      host: 10.0.1.11
      capacity:
        cpu: 16
        memory: 64GB
        gpu: 4

tasks:
  - name: distributed_training
    type: docker
    image: pytorch/pytorch
    command: torchrun --nproc_per_node=8 train.py
    resources:
      nodes: [node-1, node-2]
      gpu_per_node: 4
```

## Documentation

- [SPEC.md](./SPEC.md) - Full specification
- [PLAN.md](./PLAN.md) - Implementation roadmap
- [docs/](docs/) - VitePress documentation

## License

MIT
