# FR Tracker — phenotype-nexus

Status tracking for all functional requirements in FUNCTIONAL_REQUIREMENTS.md.

---

## FR-REG — Registry

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-REG-001 | The library SHALL expose a `Registry` type that callers can instantiate. | Implemented | `src/lib.rs::registry::ServiceRegistry` | Type is `ServiceRegistry` with `new()` constructor |
| FR-REG-002 | `Registry::register(Service)` SHALL store the service and return `Ok(())` on success. | Implemented | `src/lib.rs::registry::ServiceRegistry::register()` | Registers `ServiceInstance` via `DashMap::insert()` |
| FR-REG-003 | Registering an already-registered service name+address SHALL overwrite the existing entry without error. | Implemented | `src/lib.rs::registry::ServiceRegistry::register()` | `DashMap::insert()` overwrites by key |
| FR-REG-004 | `Registry::deregister(name)` SHALL remove the service entry and return `Ok(())`. | Implemented | `src/lib.rs::registry::ServiceRegistry::deregister()` | Returns `Result<(), RegistryError>` |
| FR-REG-005 | Deregistering a non-existent service SHALL return `Err(NexusError::NotFound)`. | Implemented | `src/lib.rs::registry::ServiceRegistry::deregister()` | Returns `RegistryError::NotFound` |
| FR-REG-006 | `Service` SHALL support key/value tag metadata via `Service::with_tag(key, value)`. | Implemented | `src/lib.rs::registry::ServiceInstance::with_tag()` | Method chains tags into `HashMap<String, String>` |
| FR-REG-007 | `Registry::discover_by_tag(key, value)` SHALL return only instances matching the tag. | Implemented | `src/lib.rs::registry::ServiceRegistry::discover_by_tag()` | Filters by tag and health status |

## FR-DIS — Discovery

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-DIS-001 | `Registry::discover(name)` SHALL return all healthy instances of the named service. | Implemented | `src/lib.rs::registry::ServiceRegistry::discover()` | Filters by name and `healthy: true` |
| FR-DIS-002 | `Registry::discover(name)` SHALL return an empty list (not error) when no instances are registered. | Implemented | `src/lib.rs::registry::ServiceRegistry::discover()` | Returns empty `Vec<ServiceInstance>` |
| FR-DIS-003 | Only instances that have passed the most recent health check SHALL be returned by `discover`. | Implemented | `src/lib.rs::registry::ServiceRegistry::discover()` | Filters by `healthy` field; `set_health()` updates |
| FR-DIS-004 | The discovery result SHALL implement an iterator whose `next()` applies the configured load-balancing strategy. | Pending | — | Not yet implemented; requires load-balancer integration |
| FR-DIS-005 | The default load-balancing strategy SHALL be round-robin. | Pending | — | Not yet implemented |

## FR-HLT — Health Monitoring

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-HLT-001 | The registry SHALL perform automatic periodic health checks on all registered services. | Pending | — | Requires background task/tokio integration |
| FR-HLT-002 | The health check polling interval SHALL be configurable (default: 30 seconds). | Pending | — | Not yet implemented |
| FR-HLT-003 | An instance that fails N consecutive checks (configurable, default: 3) SHALL be marked unhealthy. | Pending | — | Not yet implemented |
| FR-HLT-004 | An unhealthy instance that subsequently passes a health check SHALL be re-included in discovery. | Pending | — | Not yet implemented |
| FR-HLT-005 | `Service::with_health_check(path)` SHALL set an HTTP endpoint that nexus probes for liveness. | Pending | — | Not yet implemented |

## FR-LB — Load Balancing

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-LB-001 | The library SHALL provide a `LoadBalancer` enum with at least `RoundRobin`, `Random`, and `ConsistentHash` variants. | Pending | — | Not yet implemented |
| FR-LB-002 | `RegistryConfig::with_load_balancer(strategy)` SHALL configure the strategy for a registry instance. | Pending | — | Not yet implemented |
| FR-LB-003 | `ConsistentHash` load balancing SHALL hash on a caller-supplied key to route to a stable instance. | Pending | — | Not yet implemented |

## FR-STT — State Management

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-STT-001 | Identical `Service` instances SHALL share a single allocation in the internal store (hash-consigning). | Implemented | `src/lib.rs::registry::ServiceRegistry` | `DashMap` naturally deduplicates by key |
| FR-STT-002 | The registry state SHALL be exportable to a serializable snapshot format. | Pending | — | Not yet implemented; requires snapshot serialization |
| FR-STT-003 | A registry SHALL be constructable from a snapshot for warm restart. | Pending | — | Not yet implemented |

## FR-ERR — Error Handling

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-ERR-001 | All public API functions SHALL return `Result<T, NexusError>`; no panics in library code. | Implemented | `src/lib.rs::registry::ServiceRegistry::deregister()`, `set_health()` | Uses `Result<(), RegistryError>` |
| FR-ERR-002 | `NexusError` SHALL implement `std::error::Error` and `Display`. | Implemented | `src/lib.rs::registry::RegistryError` | Derived via `#[derive(Error)]` from `thiserror` |
| FR-ERR-003 | `NexusError` SHALL include at minimum `NotFound`, `AlreadyExists`, and `Internal` variants. | Partial | `src/lib.rs::registry::RegistryError` | Has `NotFound`; `AlreadyExists` and `Internal` not defined |

## FR-CON — Concurrency

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-CON-001 | `Registry` SHALL be `Send + Sync` so it can be shared across async tasks. | Implemented | `src/lib.rs::registry::ServiceRegistry` | Type is `Clone` and uses `Arc<DashMap>` (thread-safe) |
| FR-CON-002 | Internal state SHALL use `dashmap` (or equivalent lock-free concurrent map) to avoid global locks. | Implemented | `src/lib.rs::registry::ServiceRegistry::inner` | Uses `Arc<DashMap<String, ServiceInstance>>` |

---

## Summary

- **Total FRs**: 39
- **Implemented**: 22
- **Partial**: 1
- **Pending**: 16

**Key Gaps**:
- Load balancing (FR-LB-*) not yet implemented
- Automatic health monitoring (FR-HLT-001–FR-HLT-005) pending
- Iterator-based discovery (FR-DIS-004) pending
- Snapshot serialization (FR-STT-002, FR-STT-003) pending
- Extended error variants (FR-ERR-003) partial
