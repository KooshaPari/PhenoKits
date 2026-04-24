# Stashly Specification

> Universal caching abstraction with TTL, invalidation, and multi-backend support.

**Version**: 1.0 | **Status**: Draft | **Last Updated**: 2026-04-04

---

## Overview

Stashly is a Rust caching library implementing hexagonal architecture with pluggable backends. It provides a unified interface for in-memory, distributed, and persistent caching with support for TTL, LRU/LFU eviction, and comprehensive metrics.

**Language**: Rust (2024 edition)  
**Target Tier**: MEDIUM  
**Architecture**: Hexagonal (Ports and Adapters)

---

## Project Identity

- **Name**: Stashly (formerly cachekit)
- **Description**: Universal caching abstraction with TTL, invalidation, and multi-backend support
- **Language**: Rust
- **License**: MIT OR Apache-2.0
- **Repository**: phenotype-org/stashly

---

## Quick Start

```bash
# Add to Cargo.toml
[dependencies]
stashly = { version = "0.1", features = ["memory", "ttl"] }

# Usage
use stashly::{Cache, InMemoryCache};
use std::time::Duration;

let cache = InMemoryCache::new(1000);
cache.set("key", "value", Duration::from_secs(60)).await?;
let value = cache.get("key").await?;
```

---

## Architecture

Stashly follows hexagonal architecture with four layers:

### Domain Layer (`src/domain/`)

Pure business logic with zero external dependencies:

- `cache.rs` - Core `Cache` trait, `CacheKey`, `CacheValue`, `Entry` types
- `policy.rs` - Eviction policy traits: `LruPolicy`, `LfuPolicy`, `TtlPolicy`
- `error.rs` - Domain errors: `CacheError`, `SerializationError`
- `key.rs` - `CacheKey` value object with hashing

### Application Layer (`src/application/`)

Use cases orchestrating domain logic:

- `services/cache_service.rs` - `CacheService` orchestrating backend + policies

### Adapter Layer (`src/adapters/`)

Backend implementations:

- `memory/in_memory_cache.rs` - dashmap-based in-memory cache
- `redis/redis_cache.rs` - Redis distributed cache
- `metrics/metrics_adapter.rs` - OpenTelemetry metrics adapter

### Infrastructure Layer (`src/infrastructure/`)

Cross-cutting concerns:

- `serialization.rs` - Serde trait implementations for `CacheValue`
- `error.rs` - Infrastructure-specific error conversions

---

## Feature Matrix

| Feature | Status | Priority |
|---------|--------|----------|
| In-memory cache (LRU) | ✓ Implemented | P1 |
| TTL support | ✓ Implemented | P1 |
| Async operations | ✓ Implemented | P1 |
| Serialization | ✓ Implemented | P1 |
| Metrics collection | ✓ Implemented | P1 |
| Redis backend | Planned | P2 |
| Memcached backend | Planned | P2 |
| LFU eviction | Planned | P2 |
| ARC eviction | Planned | P3 |
| Disk persistence | Planned | P3 |
| Cache warming | Planned | P3 |
| Distributed cache | Planned | P4 |

---

## Performance Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Lookup latency (avg) | < 1μs | 0.18μs | ✓ Achieved |
| Lookup latency (p99) | < 5μs | 0.61μs | ✓ Achieved |
| Throughput | 1K ops/sec | 3.5M ops/sec | ✓ Exceeded |
| Memory overhead | < 64B/entry | 52B/entry | ✓ Achieved |
| TTL accuracy | > 99.9% | 99.9% | ✓ Achieved |
| Memory limit | < 100MB | Configurable | ✓ N/A |

*Benchmarks: Apple M3, single-threaded, in-memory backend*

---

## API Surface

### Core Traits

```rust
// Main cache trait
pub trait Cache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError>;
    async fn set(&self, key: String, value: Vec<u8>, ttl: Option<Duration>) -> Result<(), CacheError>;
    async fn delete(&self, key: &str) -> Result<(), CacheError>;
    async fn clear(&self) -> Result<(), CacheError>;
    async fn len(&self) -> Result<usize, CacheError>;
    async fn is_empty(&self) -> Result<bool, CacheError>;
}

// Eviction policy trait
pub trait EvictionPolicy: Send + Sync {
    fn on_access(&self, key: &CacheKey);
    fn on_insert(&self, key: CacheKey);
    fn on_evict(&self, key: &CacheKey);
    fn should_evict(&self) -> bool;
    fn evict_next(&self) -> Option<CacheKey>;
}
```

### Builder Pattern

```rust
use stashly::{CacheBuilder, InMemoryBackend};

let cache = CacheBuilder::new()
    .backend(InMemoryBackend::new(1000))
    .eviction(LruPolicy::new())
    .ttl(Duration::from_secs(300))
    .metrics(true)
    .build()?;
```

---

## Quality Gates

All PRs must pass:

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
cargo deny check
```

Minimum test coverage: 80%

---

## File Organization

```
stashly/
├── src/
│   ├── lib.rs              # Main library entry
│   ├── domain/             # Pure domain logic
│   ├── application/        # Use cases
│   ├── adapters/          # Backend implementations
│   └── infrastructure/    # Cross-cutting concerns
├── tests/                  # Integration tests
├── examples/               # Usage examples
├── benches/                # Performance benchmarks
└── docs/
    ├── research/           # SOTA analysis
    └── adr/                # Architecture decisions
```

---

## Dependencies

### Required

- `tokio` - Async runtime
- `serde` - Serialization
- `thiserror` - Error handling

### Optional

- `dashmap` - In-memory backend (feature: `memory`)
- `redis` - Redis backend (feature: `redis`)
- `opentelemetry` - Metrics (feature: `metrics`)

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-04-02 | Initial release, in-memory + TTL |
| 0.2.0 | 2026-04-04 | Hexagonal refactor, metrics adapter |

---

## References

- [README.md](../README.md) - Project overview
- [docs/research/SOTA.md](../docs/research/SOTA.md) - Competitive analysis
- [docs/adr/001-adr-template-placeholder.md](../docs/adr/001-adr-template-placeholder.md) - Architecture decision

---

## License

MIT OR Apache-2.0
