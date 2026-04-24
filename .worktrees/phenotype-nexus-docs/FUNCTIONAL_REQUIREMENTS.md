# Functional Requirements — phenotype-nexus (nexus)

Traces to: PRD.md epics E1–E5.

---

## FR-REG — Registry

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-REG-001 | The library SHALL expose a `Registry` type that callers can instantiate. | Critical | Pending | E1.1 |
| FR-REG-002 | `Registry::register(Service)` SHALL store the service and return `Ok(())` on success. | Critical | Pending | E1.1 |
| FR-REG-003 | Registering an already-registered service name+address SHALL overwrite the existing entry without error. | High | Pending | E1.1 |
| FR-REG-004 | `Registry::deregister(name)` SHALL remove the service entry and return `Ok(())`. | High | Pending | E1.2 |
| FR-REG-005 | Deregistering a non-existent service SHALL return `Err(NexusError::NotFound)`. | Medium | Pending | E1.2 |
| FR-REG-006 | `Service` SHALL support key/value tag metadata via `Service::with_tag(key, value)`. | Medium | Pending | E1.3 |
| FR-REG-007 | `Registry::discover_by_tag(key, value)` SHALL return only instances matching the tag. | Medium | Pending | E1.3 |

## FR-DIS — Discovery

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-DIS-001 | `Registry::discover(name)` SHALL return all healthy instances of the named service. | Critical | Pending | E2.1 |
| FR-DIS-002 | `Registry::discover(name)` SHALL return an empty list (not error) when no instances are registered. | High | Pending | E2.1 |
| FR-DIS-003 | Only instances that have passed the most recent health check SHALL be returned by `discover`. | High | Pending | E2.1, E3.1 |
| FR-DIS-004 | The discovery result SHALL implement an iterator whose `next()` applies the configured load-balancing strategy. | High | Pending | E2.2 |
| FR-DIS-005 | The default load-balancing strategy SHALL be round-robin. | Medium | Pending | E4.1 |

## FR-HLT — Health Monitoring

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-HLT-001 | The registry SHALL perform automatic periodic health checks on all registered services. | High | Pending | E3.1 |
| FR-HLT-002 | The health check polling interval SHALL be configurable (default: 30 seconds). | Medium | Pending | E3.1 |
| FR-HLT-003 | An instance that fails N consecutive checks (configurable, default: 3) SHALL be marked unhealthy. | High | Pending | E3.1 |
| FR-HLT-004 | An unhealthy instance that subsequently passes a health check SHALL be re-included in discovery. | High | Pending | E3.1 |
| FR-HLT-005 | `Service::with_health_check(path)` SHALL set an HTTP endpoint that nexus probes for liveness. | Medium | Pending | E3.2 |

## FR-LB — Load Balancing

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-LB-001 | The library SHALL provide a `LoadBalancer` enum with at least `RoundRobin`, `Random`, and `ConsistentHash` variants. | Medium | Pending | E4.2 |
| FR-LB-002 | `RegistryConfig::with_load_balancer(strategy)` SHALL configure the strategy for a registry instance. | Medium | Pending | E4.2 |
| FR-LB-003 | `ConsistentHash` load balancing SHALL hash on a caller-supplied key to route to a stable instance. | Low | Pending | E4.2 |

## FR-STT — State Management

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-STT-001 | Identical `Service` instances SHALL share a single allocation in the internal store (hash-consigning). | High | Pending | E5.1 |
| FR-STT-002 | The registry state SHALL be exportable to a serializable snapshot format. | Low | Pending | E5.1 |
| FR-STT-003 | A registry SHALL be constructable from a snapshot for warm restart. | Low | Pending | E5.1 |

## FR-ERR — Error Handling

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-ERR-001 | All public API functions SHALL return `Result<T, NexusError>`; no panics in library code. | Critical | Pending | PRD NFR |
| FR-ERR-002 | `NexusError` SHALL implement `std::error::Error` and `Display`. | Critical | Pending | PRD NFR |
| FR-ERR-003 | `NexusError` SHALL include at minimum `NotFound`, `AlreadyExists`, and `Internal` variants. | High | Pending | FR-REG-005 |

## FR-CON — Concurrency

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-CON-001 | `Registry` SHALL be `Send + Sync` so it can be shared across async tasks. | Critical | Pending | PRD NFR |
| FR-CON-002 | Internal state SHALL use `dashmap` (or equivalent lock-free concurrent map) to avoid global locks. | High | Pending | PRD NFR, Cargo.toml |
