# Code Entity Map — phenotype-nexus

Mapping between functional requirements and code entities (types, functions, modules).

---

## Module: `registry`

**Location**: `src/lib.rs::registry`

**Purpose**: Service registration, deregistration, and discovery.

### Type: `ServiceInstance`

**Location**: `src/lib.rs::registry::ServiceInstance`

**Purpose**: Represents a registered service instance with address, metadata, and health state.

**Fields**:
- `name: String` — Service name (logical identifier)
- `address: String` — Network address (host:port)
- `tags: HashMap<String, String>` — Key/value metadata
- `healthy: bool` — Health status flag

**Related FRs**:
- FR-REG-001, FR-REG-006 (service type and tags)
- FR-DIS-001, FR-DIS-003 (discovery filtering)

**Methods**:
- `new(name, address) -> Self` — Creates healthy instance (FR-REG-001)
- `with_tag(key, value) -> Self` — Attaches metadata tag (FR-REG-006)
- `set_healthy(healthy)` — Updates health state (FR-DIS-003, FR-HLT-004)

---

### Type: `RegistryError`

**Location**: `src/lib.rs::registry::RegistryError`

**Purpose**: Error type for registry operations.

**Variants**:
- `NotFound(String)` — Service not found (FR-REG-005, FR-ERR-002, FR-ERR-003)

**Related FRs**:
- FR-ERR-001, FR-ERR-002 (error handling)
- FR-ERR-003 (error variants)

**Derives**: `Error` (via thiserror crate)

---

### Type: `ServiceRegistry`

**Location**: `src/lib.rs::registry::ServiceRegistry`

**Purpose**: In-memory, concurrent service registry with lock-free access via DashMap.

**Fields**:
- `inner: Arc<DashMap<String, ServiceInstance>>` — Backing store (FR-CON-002)

**Key Properties**:
- Implements `Clone` (FR-CON-001)
- Uses `Arc<DashMap>` for thread-safe, lock-free access (FR-CON-002)
- No `Mutex` or `RwLock` in hot path

**Related FRs**:
- FR-REG-001, FR-REG-002, FR-REG-003 (registration)
- FR-REG-004, FR-REG-005 (deregistration)
- FR-REG-007 (tag-based discovery)
- FR-DIS-001, FR-DIS-002, FR-DIS-003 (discovery)
- FR-STT-001 (deduplication via DashMap)
- FR-ERR-001, FR-ERR-002 (error handling)
- FR-CON-001, FR-CON-002 (concurrency)

**Methods**:

#### `new() -> Self`
- **Purpose**: Create empty registry (FR-REG-001)
- **Related FRs**: FR-REG-001

#### `register(&self, svc: ServiceInstance)`
- **Purpose**: Register or replace a service instance (FR-REG-002, FR-REG-003)
- **Related FRs**: FR-REG-002, FR-REG-003, FR-STT-001
- **Implementation**: `DashMap::insert()` with automatic overwrite

#### `deregister(&self, name: &str) -> Result<(), RegistryError>`
- **Purpose**: Remove service by name (FR-REG-004, FR-REG-005)
- **Related FRs**: FR-REG-004, FR-REG-005, FR-ERR-001
- **Implementation**: Returns `RegistryError::NotFound` if name not found

#### `discover(&self, name: &str) -> Vec<ServiceInstance>`
- **Purpose**: Return all healthy instances of named service (FR-DIS-001, FR-DIS-002, FR-DIS-003)
- **Related FRs**: FR-DIS-001, FR-DIS-002, FR-DIS-003
- **Implementation**: Filters by name and `healthy: true`; returns empty vec if none found

#### `discover_by_tag(&self, key: &str, value: &str) -> Vec<ServiceInstance>`
- **Purpose**: Return healthy instances matching tag (FR-REG-007)
- **Related FRs**: FR-REG-007, FR-DIS-001, FR-DIS-003
- **Implementation**: Filters by tag and health status

#### `set_health(&self, name: &str, healthy: bool) -> Result<(), RegistryError>`
- **Purpose**: Mark instance healthy or unhealthy (FR-DIS-003, FR-HLT-004)
- **Related FRs**: FR-DIS-003, FR-HLT-004, FR-ERR-001
- **Implementation**: Updates `healthy` flag via mutable iterator

#### `len(&self) -> usize`
- **Purpose**: Return total count of registered services
- **Implementation**: Delegates to `DashMap::len()`

#### `is_empty(&self) -> bool`
- **Purpose**: Check if registry is empty
- **Implementation**: Delegates to `DashMap::is_empty()`

---

## Traceability Summary

### FRs Fully Implemented (Code Exists)
- FR-REG-001, FR-REG-002, FR-REG-003, FR-REG-004, FR-REG-005, FR-REG-006, FR-REG-007
- FR-DIS-001, FR-DIS-002, FR-DIS-003
- FR-STT-001
- FR-ERR-001, FR-ERR-002, FR-ERR-003 (partial — `NotFound` exists, `AlreadyExists`/`Internal` not defined)
- FR-CON-001, FR-CON-002

### FRs Not Yet Implemented (Code Missing)
- FR-DIS-004 (iterator with load-balancing)
- FR-DIS-005 (default round-robin)
- FR-HLT-001, FR-HLT-002, FR-HLT-003, FR-HLT-004, FR-HLT-005 (automatic health monitoring)
- FR-LB-001, FR-LB-002, FR-LB-003 (load-balancing strategies)
- FR-STT-002, FR-STT-003 (snapshot serialization)

---

## Module Decomposition

| Module | Code Files | FRs Covered | Status |
|--------|-----------|------------|--------|
| `registry` | `src/lib.rs` | FR-REG-*, FR-DIS-001/002/003, FR-ERR-001/002/003, FR-STT-001, FR-CON-* | Implemented |
| `health` | — | FR-DIS-003, FR-HLT-* | Pending (module not created) |
| `balancer` | — | FR-LB-*, FR-DIS-004/005 | Pending (module not created) |
| `snapshot` | — | FR-STT-002/003 | Pending (module not created) |

---

## Forward Mapping: FR → Code

| FR | Code Entity | Method |
|----|-------------|--------|
| FR-REG-001 | `ServiceRegistry` | `new()` |
| FR-REG-002 | `ServiceRegistry` | `register()` |
| FR-REG-003 | `ServiceRegistry` | `register()` |
| FR-REG-004 | `ServiceRegistry` | `deregister()` |
| FR-REG-005 | `RegistryError` | `NotFound` variant |
| FR-REG-006 | `ServiceInstance` | `with_tag()` |
| FR-REG-007 | `ServiceRegistry` | `discover_by_tag()` |
| FR-DIS-001 | `ServiceRegistry` | `discover()` |
| FR-DIS-002 | `ServiceRegistry` | `discover()` |
| FR-DIS-003 | `ServiceRegistry` | `discover()`, `set_health()` |
| FR-STT-001 | `ServiceRegistry::inner` | `Arc<DashMap>` |
| FR-ERR-001 | `ServiceRegistry` | Return types use `Result` |
| FR-ERR-002 | `RegistryError` | `#[derive(Error)]` |
| FR-CON-001 | `ServiceRegistry` | `Clone`, `Arc<DashMap>` |
| FR-CON-002 | `ServiceRegistry::inner` | `DashMap` |

---

## Reverse Mapping: Code → FRs

| Code Entity | Type | Related FRs |
|-------------|------|------------|
| `ServiceInstance` | struct | FR-REG-001, FR-REG-006, FR-DIS-001, FR-DIS-003 |
| `RegistryError` | enum | FR-REG-005, FR-ERR-001, FR-ERR-002, FR-ERR-003 |
| `ServiceRegistry` | struct | FR-REG-001–007, FR-DIS-001–003, FR-STT-001, FR-ERR-001, FR-CON-001–002 |
| `ServiceRegistry::register()` | method | FR-REG-002, FR-REG-003, FR-STT-001 |
| `ServiceRegistry::deregister()` | method | FR-REG-004, FR-REG-005, FR-ERR-001 |
| `ServiceRegistry::discover()` | method | FR-DIS-001, FR-DIS-002, FR-DIS-003 |
| `ServiceRegistry::discover_by_tag()` | method | FR-REG-007, FR-DIS-001, FR-DIS-003 |
| `ServiceRegistry::set_health()` | method | FR-DIS-003, FR-HLT-004, FR-ERR-001 |

---

## Test Coverage Map

All tests should reference FR IDs via docstring or marker:

```rust
#[test]
/// Traces to: FR-REG-001, FR-DIS-001
fn test_register_and_discover() { ... }
```

Key test scenarios:
- Registration and discovery (FR-REG-001–003, FR-DIS-001–002)
- Deregistration with error handling (FR-REG-004–005, FR-ERR-*)
- Tag-based discovery (FR-REG-006–007)
- Health filtering (FR-DIS-003, FR-HLT-004)
- Concurrency and thread-safety (FR-CON-*)
