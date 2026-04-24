# agent-wave Specification

## Architecture
```
┌─────────────────────────────────────────────────────────────────┐
│                         Agent Wave                               │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────┐    ┌──────────┐    ┌─────────┐    ┌──────────┐   │
│  │ Wave    │───▶│ Lifecycle│───▶│ Executor│───▶│ State    │   │
│  │ Manager │    │ Manager  │    │         │    │ Store   │   │
│  └─────────┘    └──────────┘    └─────────┘    └──────────┘   │
│       │             │              │              │             │
│       ▼             ▼              ▼              ▼             │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Plugin/Extension Registry              │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Components

| Component | Responsibility | Public API |
|-----------|----------------|-----------|
| WaveManager | Orchestrate execution waves | `execute_wave()`, `pause()`, `resume()` |
| LifecycleManager | Agent state transitions | `start()`, `stop()`, `checkpoint()` |
| Executor | Run agent tasks | `execute()`, `execute_parallel()` |
| StateStore | Persist agent state | `save()`, `load()`, `query()` |

## Data Models

```rust
struct Wave {
    id: Uuid,
    agents: Vec<AgentConfig>,
    parallel: bool,
    timeout: Duration,
}

struct AgentConfig {
    name: String,
    image: String,
    env_vars: HashMap<String, String>,
    capabilities: Vec<Capability>,
}

enum Capability {
    FileSystem,
    Network,
    Exec,
    Gitness,
}
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Wave startup | <500ms |
| Agent spawn | <2s |
| Checkpoint save | <100ms |
| Concurrent agents | 50 max |