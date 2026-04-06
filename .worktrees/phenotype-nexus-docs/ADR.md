# Architecture Decision Records — phenotype-nexus (nexus)

---

## ADR-001 — Use `dashmap` for concurrent registry state

**Status:** Accepted
**Date:** 2026-03-25

### Context

The registry must be `Send + Sync` and support concurrent reads and writes from multiple async tasks running on a tokio multi-thread runtime. A standard `std::sync::RwLock<HashMap>` would work but introduces contention on the write lock during high-frequency registration and discovery operations. The goal is sub-millisecond discovery latency at 1000 registered services.

### Decision

Use `dashmap` (v5.x) as the backing store for the service registry. `dashmap` is a lock-free concurrent `HashMap` implementation using sharding; it provides `Send + Sync` automatically and avoids a single global write lock.

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| `std::sync::RwLock<HashMap>` | Single global lock; contention risk at high concurrency |
| `tokio::sync::RwLock<HashMap>` | Async-aware but still single-lock; not truly lock-free |
| External key-value store (Redis) | Adds a network dependency; nexus is a library, not a service |

### Consequences

- Dependency on `dashmap = "5.0"`.
- `DashMap` is not a standard type; consumers cannot treat registry state as a plain `HashMap`.
- Upgrade path: future versions could replace `dashmap` with `papaya` (newer lock-free map) if performance benchmarks justify it.

---

## ADR-002 — Hash-consigning for `Service` state deduplication

**Status:** Accepted
**Date:** 2026-03-25

### Context

In environments with many services registering and deregistering rapidly, duplicate `Service` structs with identical fields would consume redundant heap allocations. The Cargo.toml description calls out "hashconsign-based state management" as a core feature.

### Decision

Identical `Service` instances (same name, address, tags) share a single allocation using an intern-on-insert pattern. Before storing a new registration, compute a canonical hash of the `Service` fields and check against an existing entry. If an identical entry exists, reuse it.

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| No deduplication | Wastes memory when the same service instance is re-registered |
| `Arc<Service>` reference counting without interning | Still allocates a new `Arc` per registration call |

### Consequences

- `Service` must implement `Hash` and `Eq` based on its fields.
- The interning table adds a second data structure in memory (tolerable; bounded by the number of unique services).

---

## ADR-003 — In-memory only; no persistence in v0.1

**Status:** Accepted
**Date:** 2026-03-25

### Context

A persistence layer (disk, database) would add significant complexity and external dependencies. The v0.1 scope is to provide a correct, fast, in-process registry suitable for single-node or testing scenarios.

### Decision

The registry stores all state in process memory. State is lost when the process exits. A snapshot export/import API is provided as a future extension point (FR-STT-002, FR-STT-003) but not implemented in v0.1.

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| SQLite via `rusqlite` | Disk I/O overhead; adds C build dep |
| etcd / Consul client | Turns nexus into a remote-client library; scope creep |
| `sled` embedded database | Async complexity; out of v0.1 scope |

### Consequences

- Library is suitable for testing, single-node microservices, and as an in-process mock registry.
- Multi-node deployments require an external registry (Consul, etcd) and nexus becomes an adapter over that.

---

## ADR-004 — Library-only crate; no binary/daemon surface

**Status:** Accepted
**Date:** 2026-03-25

### Context

A service registry could be implemented as a standalone daemon that all services connect to over a network socket. However, this requires IPC, serialization, and a network server — significantly more scope than a library.

### Decision

`nexus` is a `lib` crate only (no `[[bin]]` targets). It is embedded into the process that needs registry functionality. Services communicate with the registry via function calls, not network sockets.

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| HTTP server daemon | Requires network, serialization, auth — scope creep for v0.1 |
| gRPC daemon | Same concerns; also adds prost/tonic build complexity |

### Consequences

- Suitable for monolith-first or in-process microservice setups where all service instances run in the same process (e.g., tests, integration rigs).
- For true distributed service discovery, nexus acts as the client-side registry view; an external store backs it.

---

## ADR-005 — Migration: phenotype-nexus -> libs/nexus

**Status:** Accepted
**Date:** 2026-03-25

### Context

The Phenotype polyrepo governance model specifies that neutral infrastructure libraries should live under `libs/` in the monorepo rather than as standalone `phenotype-*` repos. This reduces duplication, simplifies versioning, and enables shared governance.

### Decision

Migrate `phenotype-nexus` to `libs/nexus` as the canonical location. The `phenotype-nexus` GitHub repo is archived (read-only) with an `ARCHIVED.md` pointing to the new location.

### Consequences

- New consumers should reference `libs/nexus` (path dep in monorepo) or the published crate.
- The `phenotype-nexus` repo remains for historical reference and external links, but receives no further development.
