# PRD — phenotype-nexus (nexus)

## Product Overview

`nexus` is a Rust library providing service registry and discovery with hash-consign-based state management. It enables microservices in the Phenotype ecosystem to register themselves, discover peers, monitor health, and receive load-balanced endpoints without coupling to a specific service mesh.

**Status:** Archived (2026-03-25). Source migrated to `libs/nexus/` as the neutral service discovery registry under the Phenotype monorepo governance model. This PRD captures the intended product scope.

**Package:** `nexus` v0.1.0 (crate name: `nexus`)
**Repository:** `KooshaPari/phenotype-nexus`
**Language:** Rust 2021 edition
**License:** MIT

---

## Epics and User Stories

### E1 — Service Registry

**E1.1** As a microservice developer, I want to register a service instance (name + address) so that other services can discover it.
- Acceptance: `registry.register(Service::new("user-svc", "localhost:8080")).await` returns `Ok(())`.
- Acceptance: Registering the same service twice replaces the existing entry without error.

**E1.2** As a microservice developer, I want to deregister a service instance so that stale endpoints are removed from the registry.
- Acceptance: After `registry.deregister("user-svc")`, subsequent `discover("user-svc")` returns empty or `Err(NotFound)`.

**E1.3** As a platform operator, I want the registry to support tagging services (key/value labels) so that discovery can be filtered by tag.
- Acceptance: `Service::new("auth-svc", "localhost:9090").with_tag("region", "us-west")`.
- Acceptance: `registry.discover_by_tag("region", "us-west")` returns only matching instances.

### E2 — Service Discovery

**E2.1** As a microservice developer, I want to discover all healthy instances of a named service so that I can route requests to them.
- Acceptance: `registry.discover("user-svc").await` returns `Vec<ServiceInstance>` (may be empty).
- Acceptance: Only instances that have passed the most recent health check are returned.

**E2.2** As a microservice developer, I want the discovery result to be load-balanced across available instances so that traffic is distributed evenly.
- Acceptance: `services.next()` applies the configured load-balancing strategy (default: round-robin).

### E3 — Health Monitoring

**E3.1** As a platform operator, I want the registry to perform automatic health checks on registered services so that unhealthy instances are automatically removed from discovery results.
- Acceptance: A configurable polling interval triggers health checks (default: 30 s).
- Acceptance: An instance that fails N consecutive checks (configurable, default: 3) is marked unhealthy and excluded from discovery.
- Acceptance: An instance that returns healthy after a failure period is re-included automatically.

**E3.2** As a service developer, I want to supply a custom health check endpoint or function so that nexus can probe my service correctly.
- Acceptance: `Service::new(...).with_health_check("/healthz")` — nexus performs an HTTP GET to verify liveness.

### E4 — Load Balancing

**E4.1** As a microservice developer, I want round-robin load balancing as the default strategy so that requests are spread evenly across instances.
- Acceptance: Sequential `discover` + `next()` calls cycle through all healthy instances.

**E4.2** As a platform operator, I want to select random or consistent-hash load balancing per registry so that specific use cases (stateful services, cache locality) are supported.
- Acceptance: `RegistryConfig::new().with_load_balancer(LoadBalancer::ConsistentHash)`.

### E5 — State Management

**E5.1** As a platform operator, I want registry state to use hash-consigning so that equal service entries are deduplicated and memory usage stays bounded.
- Acceptance: Two `Service` instances with identical fields share a single allocation in the internal store.
- Acceptance: State can be exported and re-imported for warm restart scenarios.

---

## Non-Functional Requirements

| Category | Requirement |
|----------|-------------|
| Performance | Discovery for 1000 registered services < 1 ms (99th percentile) |
| Concurrency | Registry must be safe to use from multiple async tasks simultaneously (no data races) |
| Error handling | All public API functions return `Result<T, NexusError>`; no panics in happy path |
| Dependencies | Must compile with tokio full-feature runtime; no C foreign-function dependencies |
| Portability | Must compile on Linux, macOS, and Windows (x86-64) |

---

## Out of Scope (v0.1)

- Persistent storage (disk/database) — registry is in-memory only.
- Distributed consensus / gossip protocol.
- gRPC or HTTP server surface — nexus is a library, not a daemon.
- Authentication or authorization for registry operations.
