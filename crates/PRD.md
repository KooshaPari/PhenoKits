# Product Requirements Document (PRD) - crates

## 1. Executive Summary

**crates** is the Phenotype shared Rust workspace containing domain-agnostic infrastructure crates used across the entire Phenotype ecosystem. It provides battle-tested, reusable primitives for common concerns like caching, event sourcing, state machines, policy enforcement, and configuration management. Each crate is independently consumable with zero inter-crate dependencies, enabling teams to adopt only what they need.

**Vision**: To be the foundational infrastructure layer for all Phenotype services, providing such high-quality, well-documented, and reliable primitives that no team ever needs to reinvent common infrastructure patterns again.

**Mission**: Deliver a curated set of Rust crates that set the standard for code quality, documentation, testing, and API design within the organization and the broader Rust community.

**Current Status**: Active development with 4+ crates in production use across multiple services.

---

## 2. Problem Statement

### 2.1 Current Challenges

Building reliable infrastructure for production services involves significant challenges:

**Repeated Implementation**: Teams constantly reimplement the same infrastructure patterns:
- Event sourcing stores with subtle consistency bugs
- Cache implementations without proper eviction or metrics
- State machines with incomplete transition handling
- Policy engines that are hard to test and modify
- Configuration management that lacks validation

**Quality Variance**: Ad-hoc implementations vary in quality:
- Insufficient error handling and edge case coverage
- Missing observability and metrics
- Inconsistent APIs across services
- Different testing strategies and coverage levels
- Varying documentation quality

**Integration Complexity**: Combining multiple infrastructure concerns is error-prone:
- Caching with event sourcing requires careful invalidation
- Policy decisions often need audit trails
- State machines may need persistence and recovery
- Configuration changes must trigger appropriate updates

**Maintenance Burden**: Each custom implementation requires ongoing maintenance:
- Security updates for dependencies
- Performance optimizations
- Bug fixes for edge cases discovered in production
- Documentation updates
- Compatibility with evolving Rust ecosystem

### 2.2 Impact

Without shared infrastructure:
- 30-50% of engineering time spent on infrastructure rather than business logic
- Inconsistent behavior across services for common patterns
- Harder to move engineers between teams due to different infrastructure
- Duplicated security vulnerabilities across implementations
- Slower incident response due to unfamiliar code

### 2.3 Target Solution

The crates workspace provides:
1. **Battle-Tested Primitives**: Thoroughly tested implementations of common patterns
2. **Zero Dependencies Between Crates**: Each crate is fully independent
3. **Hexagonal Architecture**: Clear ports and adapters for extensibility
4. **Comprehensive Documentation**: Every public API documented with examples
5. **Production Ready**: Metrics, observability, and operational features built-in

---

## 3. Target Users & Personas

### 3.1 Primary Personas

#### Alex - Senior Rust Developer
- **Role**: Building core services in the Phenotype ecosystem
- **Pain Points**: Need reliable infrastructure primitives; don't want to build from scratch
- **Goals**: Drop-in solutions for common patterns; strong type safety; great performance
- **Technical Level**: Expert
- **Usage Pattern**: Import specific crates; extend with custom adapters

#### Jordan - Performance Engineer
- **Role**: Optimizing system performance and resource usage
- **Pain Points**: Need visibility into cache hit rates, event store performance
- **Goals**: Detailed metrics; low overhead; configurable performance characteristics
- **Technical Level**: Expert
- **Usage Pattern**: Configure tuning parameters; analyze metrics; benchmark

#### Taylor - New Team Member
- **Role**: Recently joined, learning the Phenotype stack
- **Pain Points**: Need to understand how things work quickly
- **Goals**: Clear documentation; working examples; consistent patterns
- **Technical Level**: Intermediate
- **Usage Pattern**: Reading docs; following examples; asking questions

### 3.2 Secondary Personas

#### Morgan - Architect
- **Role**: Designing system architecture and making technology choices
- **Pain Points**: Need to understand capabilities and trade-offs
- **Goals**: Architecture decision records; comparative analysis; extensibility

#### Riley - Library Author
- **Role**: Building new crates to extend the ecosystem
- **Pain Points**: Need to follow established patterns
- **Goals**: Consistent API design; testing strategies; documentation standards

---

## 4. Functional Requirements

### 4.1 phenotype-event-sourcing

#### FR-ES-001: Append-Only Event Store
**Priority**: P0 (Critical)
**Description**: Immutable event storage with hash chain integrity
**Acceptance Criteria**:
- Atomic append operations per aggregate stream
- Monotonically increasing sequence numbers
- SHA-256 hash chain linking events
- Tamper detection via hash verification
- Stream partitioning by aggregate type and ID

#### FR-ES-002: Event Envelope
**Priority**: P0 (Critical)
**Description**: Rich event metadata container
**Acceptance Criteria**:
- Unique event ID (UUID v4)
- Aggregate type and ID
- Sequence number within stream
- Timestamp (UTC with nanosecond precision)
- Previous event hash
- Event type discriminator
- Payload serialization (JSON, protobuf, or custom)
- Metadata map for custom attributes

#### FR-ES-003: Snapshots
**Priority**: P1 (High)
**Description**: Aggregate state snapshots for performance
**Acceptance Criteria**:
- Snapshot creation at configurable intervals
- Snapshot versioning
- Atomic save with metadata
- Efficient loading (skip event replay)
- Snapshot lifecycle management

#### FR-ES-004: Storage Adapters
**Priority**: P0 (Critical)
**Description**: Pluggable storage backends
**Acceptance Criteria**:
- InMemoryStore for testing
- SqlxStore for SQL databases
- SqliteStore for embedded use
- S3Store for object storage
- Adapter trait for custom implementations

#### FR-ES-005: Event Projections
**Priority**: P1 (High)
**Description**: Read model construction from events
**Acceptance Criteria**:
- Projection trait definition
- Async projection processing
- Projection replay capability
- Multiple projection support per stream
- Eventually consistent read models

### 4.2 phenotype-cache-adapter

#### FR-CACHE-001: Two-Tier Cache
**Priority**: P0 (Critical)
**Description**: L1 LRU + L2 concurrent map cache
**Acceptance Criteria**:
- L1: Bounded LRU cache (fast, thread-local or sharded)
- L2: Unbounded concurrent hash map (DashMap)
- Promotion from L2 to L1 on access
- Eviction from L1 when full
- Thread-safe operations

#### FR-CACHE-002: TTL Support
**Priority**: P0 (Critical)
**Description**: Time-based expiration
**Acceptance Criteria**:
- Per-entry TTL configuration
- Global default TTL
- Lazy expiration on access
- Background cleanup task
- TTL extension on access (optional)

#### FR-CACHE-003: Cache Policies
**Priority**: P1 (High)
**Description**: Configurable caching strategies
**Acceptance Criteria**:
- LRU (Least Recently Used)
- LFU (Least Frequently Used)
- FIFO (First In First Out)
- Custom eviction policies
- Size-based eviction

#### FR-CACHE-004: Metrics
**Priority**: P1 (High)
**Description**: Comprehensive cache statistics
**Acceptance Criteria**:
- Hit/miss counters
- Eviction counters (LRU, TTL)
- Access latency histograms
- Size tracking
- Metrics export (Prometheus format)

#### FR-CACHE-005: Cache Loader
**Priority**: P1 (High)
**Description**: Automatic cache population
**Acceptance Criteria**:
- Loader trait for custom loading logic
- Async loading support
- Cache-aside pattern
- Write-through and write-behind options
- Bulk loading support

### 4.3 phenotype-state-machine

#### FR-FSM-001: State Definition
**Priority**: P0 (Critical)
**Description**: Type-safe state representation
**Acceptance Criteria**:
- State trait for custom states
- State data container support
- State serialization/deserialization
- State versioning for migrations

#### FR-FSM-002: Transitions
**Priority**: P0 (Critical)
**Description**: Valid state transitions
**Acceptance Criteria**:
- Transition table definition
- Compile-time transition validation where possible
- Runtime transition guards
- Transition metadata (name, description)
- Self-transitions support

#### FR-FSM-003: Transition Guards
**Priority**: P1 (High)
**Description**: Conditional transition enabling
**Acceptance Criteria**:
- Guard trait for custom logic
- Async guard evaluation
- Guard composition (AND, OR, NOT)
- Detailed guard failure reasons
- Guard caching for performance

#### FR-FSM-004: Actions
**Priority**: P1 (High)
**Description**: Side effects on transitions
**Acceptance Criteria**:
- Entry actions (on state entry)
- Exit actions (on state exit)
- Transition actions (during transition)
- Async action support
- Action error handling and rollback

#### FR-FSM-005: Persistence
**Priority**: P1 (High)
**Description**: State machine persistence
**Acceptance Criteria**:
- State machine serialization
- Event-sourced persistence integration
- Snapshot support
- Recovery from persistence
- Migration support for state changes

### 4.4 phenotype-policy-engine

#### FR-POLICY-001: Policy Definition
**Priority**: P0 (Critical)
**Description**: Declarative policy specification
**Acceptance Criteria**:
- TOML-based policy files
- Rule composition (AND, OR, NOT)
- Condition types: equals, contains, regex, range, exists
- Action definitions (allow, deny, audit)
- Policy versioning

#### FR-POLICY-002: Context Evaluation
**Priority**: P0 (Critical)
**Description**: Rich evaluation context
**Acceptance Criteria**:
- Subject attributes (user, role, groups)
- Resource attributes (type, owner, tags)
- Environment attributes (time, location, device)
- Request attributes (action, parameters)
- Custom context providers

#### FR-POLICY-003: Policy Evaluation
**Priority**: P0 (Critical)
**Description**: Efficient policy decision
**Acceptance Criteria**:
- Decision: Allow, Deny, NotApplicable
- Obligations (actions required on allow)
- Advice (suggestions on deny)
- Evaluation tracing for debugging
- Decision caching

#### FR-POLICY-004: Hot Reloading
**Priority**: P1 (High)
**Description**: Dynamic policy updates
**Acceptance Criteria**:
- File watcher integration
- Policy validation before activation
- Graceful policy transition
- Rollback on validation failure
- Audit logging of policy changes

#### FR-POLICY-005: Audit Logging
**Priority**: P1 (High)
**Description**: Comprehensive decision logging
**Acceptance Criteria**:
- Decision logging (allow and deny)
- Context snapshot at decision time
- Policy version reference
- Structured logging format
- SIEM integration

### 4.5 phenotype-config

#### FR-CONFIG-001: Configuration Loading
**Priority**: P0 (Critical)
**Description**: Multi-source configuration
**Acceptance Criteria**:
- File sources (TOML, YAML, JSON)
- Environment variables
- Command line arguments
- Remote sources (HTTP)
- Kubernetes ConfigMaps/Secrets

#### FR-CONFIG-002: Configuration Resolution
**Priority**: P0 (Critical)
**Description**: Layered configuration
**Acceptance Criteria**:
- Base configuration layer
- Environment-specific overlays
- Local developer overrides
- Clear precedence rules
- Variable interpolation

#### FR-CONFIG-003: Validation
**Priority**: P0 (Critical)
**Description**: Schema-based validation
**Acceptance Criteria**:
- Deserialize to strongly-typed structs
- Validation annotations
- Custom validators
- Detailed error messages
- Partial validation support

#### FR-CONFIG-004: Hot Reloading
**Priority**: P1 (High)
**Description**: Dynamic configuration updates
**Acceptance Criteria**:
- File watcher integration
- Change notification callbacks
- Validation on reload
- Rollback on invalid changes
- Graceful degradation

#### FR-CONFIG-005: Secret Handling
**Priority**: P1 (High)
**Description**: Secure secret management
**Acceptance Criteria**:
- Secret marker types
- Integration with secret providers
- Redaction in logs and debug output
- Audit logging of secret access
- Memory clearing on drop

### 4.6 Cross-Cutting Concerns

#### FR-CROSS-001: Error Handling
**Priority**: P0 (Critical)
**Description**: Consistent error handling
**Acceptance Criteria**:
- thiserror-based error types
- Structured error contexts
- Error chaining with source
- Serialize errors to JSON
- Display formatting with context

#### FR-CROSS-002: Observability
**Priority**: P1 (High)
**Description**: Built-in metrics and tracing
**Acceptance Criteria**:
- OpenTelemetry tracing integration
- Metrics export (Prometheus)
- Structured logging (tracing crate)
- Span creation for operations
- Context propagation

#### FR-CROSS-003: Testing Support
**Priority**: P1 (High)
**Description**: Test helpers and utilities
**Acceptance Criteria**:
- Mock implementations for traits
- Test fixtures and builders
- Deterministic testing helpers
- Performance test utilities
- Documentation examples as tests

---

## 5. Non-Functional Requirements

### 5.1 Performance

#### NFR-PERF-001: Throughput
**Priority**: P1 (High)
**Description**: High-throughput operations
**Requirements**:
- Event store: 10,000+ events/second append
- Cache: 1,000,000+ operations/second
- Policy engine: < 1ms per decision
- State machine: < 100µs per transition

#### NFR-PERF-002: Latency
**Priority**: P1 (High)
**Description**: Low latency for common operations
**Requirements**:
- Cache hit: < 1µs
- Event append: < 5ms
- Policy evaluation: < 10ms
- Config reload: < 100ms

#### NFR-PERF-003: Memory Efficiency
**Priority**: P1 (High)
**Description**: Efficient memory usage
**Requirements**:
- Bounded memory usage for all caches
- Streaming operations where possible
- Zero-copy where applicable
- Memory pool reuse

### 5.2 Reliability

#### NFR-REL-001: Correctness
**Priority**: P0 (Critical)
**Description**: Provably correct implementations
**Requirements**:
- Extensive unit tests (>90% coverage)
- Property-based testing
- Concurrency stress tests
- Fuzz testing for parsers
- Formal verification where applicable

#### NFR-REL-002: Failure Handling
**Priority**: P0 (Critical)
**Description**: Graceful degradation
**Requirements**:
- No panics in production code
- Clear error propagation
- Recovery mechanisms
- Circuit breaker support
- Retry with backoff

### 5.3 Maintainability

#### NFR-MAINT-001: Documentation
**Priority**: P0 (Critical)
**Description**: Comprehensive documentation
**Requirements**:
- Every public item documented
- Runnable examples
- Architecture decision records
- Migration guides
- Changelog maintenance

#### NFR-MAINT-002: API Stability
**Priority**: P1 (High)
**Description**: Stable public APIs
**Requirements**:
- Semantic versioning
- Deprecation notices
- Migration paths
- Backward compatibility testing

### 5.4 Integration

#### NFR-INT-001: Async Support
**Priority**: P0 (Critical)
**Description**: First-class async/await support
**Requirements**:
- All I/O operations async
- tokio compatibility
- async-trait for trait methods
- Blocking operation alternatives

#### NFR-INT-002: Serialization
**Priority**: P1 (High)
**Description**: Multiple serialization formats
**Requirements**:
- serde support for all data types
- JSON support
- Protocol Buffers support
- Binary formats (bincode, messagepack)
- Custom serialization traits

---

## 6. User Stories

### 6.1 Developer Stories

#### US-DEV-001: Event Sourcing
**As a** service developer
**I want to** use the event sourcing crate
**So that** I get audit trails and temporal queries for free
**Acceptance Criteria**:
- Import phenotype-event-sourcing crate
- Define events as simple structs
- Store and retrieve events reliably
- Build projections for read models
- Replay events for debugging

#### US-DEV-002: Caching
**As a** performance-conscious developer
**I want to** add caching to my service
**So that** I improve response times
**Acceptance Criteria**:
- Configure cache size and TTL
- Use cache loader for automatic population
- Monitor hit rates via metrics
- Evict entries programmatically
- Clear metrics on demand

#### US-DEV-003: State Machines
**As a** developer implementing workflows
**I want to** define state machines declaratively
**So that** my workflow logic is clear and testable
**Acceptance Criteria**:
- Define states as enums
- Specify valid transitions
- Add guards for conditional logic
- Persist state machine to event store
- Test state machine transitions

### 6.2 Platform Stories

#### US-PLAT-001: Policy Enforcement
**As a** platform engineer
**I want to** enforce organization policies
**So that** all services comply with security requirements
**Acceptance Criteria**:
- Define policies in TOML
- Hot reload without restart
- Audit all policy decisions
- Integrate with identity provider
- Custom context attributes

#### US-PLAT-002: Configuration Management
**As a** platform engineer
**I want to** standardize configuration across services
**So that** operational complexity is reduced
**Acceptance Criteria**:
- Base configuration templates
- Environment-specific overlays
- Validation prevents misconfiguration
- Hot reload for dynamic updates
- Secret integration

---

## 7. Feature Specifications

### 7.1 Workspace Structure

```
crates/
├── phenotype-event-sourcing/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── event.rs
│   │   ├── store.rs
│   │   ├── snapshot.rs
│   │   └── projection.rs
│   └── tests/
├── phenotype-cache-adapter/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── cache.rs
│   │   ├── lru.rs
│   │   └── metrics.rs
│   └── tests/
├── phenotype-state-machine/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── machine.rs
│   │   ├── state.rs
│   │   └── transition.rs
│   └── tests/
├── phenotype-policy-engine/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── policy.rs
│   │   ├── engine.rs
│   │   └── context.rs
│   └── tests/
└── phenotype-config/
    ├── src/
    │   ├── lib.rs
    │   ├── loader.rs
    │   ├── validator.rs
    │   └── source.rs
    └── tests/
```

### 7.2 Crate Template

Each crate follows this structure:

```rust
// lib.rs
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod error;
pub mod types;

pub use error::{Error, Result};
pub use types::*;

use std::fmt::Debug;

/// Core trait for [feature]
pub trait CoreTrait: Debug + Send + Sync {
    /// Perform the primary operation
    fn operation(&self) -> Result<()>;
}
```

### 7.3 Quality Gates

**Before Release**:
- [ ] 90%+ test coverage
- [ ] All public APIs documented
- [ ] Examples compile and run
- [ ] Clippy warnings resolved
- [ ] Security audit (cargo audit)
- [ ] Performance benchmarks
- [ ] Memory leak tests (miri)
- [ ] Concurrency tests (loom)

---

## 8. Success Metrics

### 8.1 Adoption Metrics

| Metric | Baseline | Target (6mo) | Target (12mo) |
|--------|----------|--------------|---------------|
| Services Using Crates | 0 | 20 | 50 |
| Crates Published | 4 | 8 | 15 |
| Downloads/Month | 0 | 1,000 | 5,000 |

### 8.2 Quality Metrics

| Metric | Target |
|--------|--------|
| Test Coverage | > 90% |
| Documentation Coverage | 100% |
| Open Issues | < 10 per crate |
| Mean Time to Resolution | < 3 days |

### 8.3 Performance Metrics

| Metric | Target |
|--------|--------|
| Event Store Append | < 5ms p99 |
| Cache Hit | < 1µs |
| Policy Decision | < 10ms |
| Memory Leaks | 0 (miri verified) |

---

## 9. Release Criteria

### 9.1 Version 1.0 (Current Crates)
- [ ] phenotype-event-sourcing stable
- [ ] phenotype-cache-adapter stable
- [ ] phenotype-state-machine stable
- [ ] phenotype-policy-engine stable
- [ ] phenotype-config stable
- [ ] All quality gates passed

### 9.2 Exit Criteria

**Ready When**:
1. All tests passing
2. Documentation complete
3. Examples working
4. Security audit clean
5. Performance benchmarks met
6. API review completed

---

*Document Version*: 1.0  
*Last Updated*: 2026-04-05  
*Author*: Phenotype Architecture Team  
*Status*: Active
