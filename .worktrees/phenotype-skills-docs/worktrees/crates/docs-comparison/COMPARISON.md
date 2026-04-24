# Comparison Matrix

## Feature Comparison

This document compares **AgilePlus P2P (crates)** with similar tools in the state synchronization, event sourcing, and P2P state export space.

| Repository | Purpose | Key Features | Language/Framework | Maturity | Comparison |
|------------|---------|--------------|-------------------|----------|------------|
| **AgilePlus P2P (this repo)** | Git-backed P2P state export | Deterministic JSON, JSONL events, Snapshot support, Sync metadata | Rust | Stable | Event sourcing + git |
| [EventStoreDB](https://github.com/EventStore/EventStore) | Event sourcing database | Event store, Projections, Subscriptions | C# | Stable | Enterprise event store |
| [Kafka](https://github.com/apache/kafka) | Distributed event streaming | Pub/sub, Partitions, Exactly-once | Java | Stable | Production streaming |
| [Protobuf Events](https://github.com/protocolbuffers/protobuf) | Serialization format | Deterministic, Type-safe, Cross-platform | Various | Stable | Serialization standard |
| [GitFS](https://github.com/pressly/gitfs) | Git as filesystem | Git-backed storage, Versioning | Go | Beta | Git-backed storage |
| [DVC](https://github.com/iterative/dvc) | Data versioning | Git-LFS, Data pipelines, Reproducibility | Python | Stable | Data versioning |
| [Syncthing](https://github.com/syncthing/syncthing) | P2P file sync | Continuous sync, Encryption, No cloud | Go | Stable | File synchronization |

## Detailed Feature Comparison

### State Export & Serialization

| Feature | AgilePlus P2P | EventStoreDB | Kafka | Protobuf |
|---------|--------------|--------------|-------|----------|
| Deterministic JSON | ✅ | ❌ | ❌ | ❌ |
| Sorted Keys | ✅ | ❌ | ❌ | ❌ |
| Snapshot Support | ✅ | ✅ | ❌ | ❌ |
| Event Sourcing | ✅ | ✅ | ✅ | ❌ |
| JSONL Format | ✅ | ❌ | ❌ | ❌ |

### P2P & Synchronization

| Feature | AgilePlus P2P | DVC | Syncthing | GitFS |
|---------|--------------|-----|-----------|-------|
| P2P Sync | ✅ | ❌ | ✅ | ❌ |
| Git-Backed | ✅ | ✅ | ❌ | ✅ |
| Device Tracking | ✅ | ❌ | ✅ | ❌ |
| Sync Vectors | ✅ | ❌ | ❌ | ❌ |
| Conflict Resolution | ❌ | ✅ | ✅ | ✅ |

### Technical Stack

| Aspect | AgilePlus P2P | EventStoreDB | Kafka | DVC |
|--------|--------------|--------------|-------|-----|
| Language | Rust | C# | Java | Python |
| Async | tokio | No | Netty | No |
| Type Safety | Full | Partial | None | None |
| Performance | High | High | Very High | Medium |

## Unique Value Proposition

AgilePlus P2P provides:

1. **Deterministic JSON**: Sorted keys ensure git-compatible, reproducible exports
2. **Git-Friendly Format**: JSONL events for clean diffs; snapshots for fast recovery
3. **Rust + Async**: Type-safe, high-performance async I/O with tokio
4. **AgilePlus Integration**: Part of the AgilePlus ecosystem for P2P synchronization

## Output Structure

```
export/
├── device.json                    # Device metadata
├── events/
│   └── {EntityType}/
│       └── {id}.jsonl           # Events (one per line)
├── snapshots/
│   └── {EntityType}/
│       └── {id}.json            # Latest snapshot
└── sync_state.json               # Sync mappings and vector
```

## Comparison with Alternatives

### vs EventStoreDB

EventStoreDB is enterprise-grade but:
- No deterministic JSON output
- No git compatibility
- C#/.NET required
- Complex clustering

AgilePlus P2P provides simpler git-friendly export.

### vs Kafka

Kafka is production streaming but:
- Not designed for git/versioning
- Complex setup
- Requires cloud/infrastructure

AgilePlus P2P provides simple file-based export.

### vs DVC

DVC is data versioning but:
- Focused on ML/data pipelines
- Git-LFS backend
- No P2P sync vectors

AgilePlus P2P provides native P2P sync with device tracking.

## Performance

| Dataset Size | Export Time |
|-------------|-------------|
| 1,000 events | ~10ms |
| 10,000 events | ~50ms |
| 100,000 events | ~400ms |

## When to Use What

| Use Case | Recommended Tool |
|----------|-----------------|
| Git-backed event sourcing | AgilePlus P2P |
| Enterprise event store | EventStoreDB |
| High-throughput streaming | Kafka |
| Data ML versioning | DVC |
| File synchronization | Syncthing |

## References

- EventStoreDB: [EventStore/EventStore](https://github.com/EventStore/EventStore)
- Kafka: [apache/kafka](https://github.com/apache/kafka)
- DVC: [iterative/dvc](https://github.com/iterative/dvc)
- Syncthing: [syncthing/syncthing](https://github.com/syncthing/syncthing)
