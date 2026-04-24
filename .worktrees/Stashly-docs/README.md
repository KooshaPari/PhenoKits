# cachekit

**Universal caching abstraction with TTL, invalidation, and multi-backend support.**

A hexagonal architecture-based caching framework supporting:

- **Multiple Backends**: Memory, Redis, Memcached, Disk
- **TTL Support**: Automatic expiration with configurable duration
- **Invalidation Strategies**: LRU, LFU, TTL-based
- **Async/Await**: Full async support with tokio
- **Serialization**: Automatic serialization of cached values
- **Metrics**: Hit/miss rates, latency tracking

## Architecture

```
cachekit/
├── src/
│   ├── domain/          # Core domain logic (pure)
│   │   ├── cache/     # Cache entities
│   │   ├── policy/    # Eviction policies
│   │   ├── ports/     # Interface definitions
│   │   └── errors/    # Domain errors
│   ├── application/    # Application services
│   │   └── services/  # Cache service
│   ├── adapters/      # Backend adapters
│   │   ├── memory/   # In-memory cache
│   │   ├── redis/    # Redis backend
│   │   └── metrics/  # Metrics adapter
│   └── infrastructure/ # Cross-cutting concerns
├── tests/             # Integration tests
├── examples/          # Usage examples
└── benches/           # Benchmarks
```

## Features

- [x] In-memory cache with LRU eviction
- [x] TTL support
- [x] Async operations
- [x] Serialization support
- [x] Metrics collection
- [ ] Redis backend
- [ ] Memcached backend
- [ ] Distributed cache
- [ ] Cache warming

## Installation

```toml
[dependencies]
cachekit = "0.1"
```

## Quick Start

```rust
use cachekit::{Cache, InMemoryCache};

let cache = InMemoryCache::new(1000);
cache.set("key", "value", Duration::from_secs(60)).await?;
let value = cache.get("key").await?;
```

## Documentation

- [API Documentation](https://docs.rs/cachekit)
- [User Guide](https://cachekit.dev/guide)
- [xDD Methodologies](STANDARDS.md)

## License

MIT OR Apache-2.0
