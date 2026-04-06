# Crates - Workspace Plan

**Document ID**: PLAN-CRATES-001  
**Version**: 1.0.0  
**Created**: 2026-04-05  
**Status**: Draft  
**Project Owner**: Phenotype Core Engineering  
**Review Cycle**: Monthly

---

## 1. Project Overview & Objectives

### 1.1 Vision Statement

The `crates/` workspace is Phenotype's foundational Rust infrastructure - a collection of specialized, reusable crates that provide core capabilities across the entire ecosystem. These crates represent the "building blocks" of Phenotype, designed for maximum reusability, performance, and reliability.

### 1.2 Mission Statement

To provide a comprehensive, battle-tested library of Rust crates that enable rapid, consistent, and secure development of Phenotype services and applications while maintaining the highest standards of code quality, performance, and documentation.

### 1.3 Core Objectives

| Objective ID | Description | Success Criteria | Priority |
|--------------|-------------|------------------|----------|
| OBJ-001 | Crate independence | Each crate independently publishable | P0 |
| OBJ-002 | Zero inter-crate dependencies | No internal coupling | P0 |
| OBJ-003 | Full type safety | Compile-time correctness | P0 |
| OBJ-004 | Comprehensive testing | >90% coverage per crate | P0 |
| OBJ-005 | Performance optimization | Benchmarks vs alternatives | P1 |
| OBJ-006 | Documentation excellence | Full rustdoc coverage | P1 |
| OBJ-007 | Cross-platform support | Linux, macOS, Windows | P1 |
| OBJ-008 | WASM compatibility | Core crates WASM-ready | P2 |
| OBJ-009 | Async-first design | Full async/await support | P0 |
| OBJ-010 | Ergonomic APIs | Developer-friendly interfaces | P1 |

### 1.4 Crate Categories

The 50+ crates are organized into logical categories:

| Category | Crates | Purpose |
|----------|--------|---------|
| Core Infrastructure | phenotype-core, phenotype-domain, phenotype-iter | Foundation types |
| Data Access | phenotype-postgres-adapter, phenotype-redis-adapter, phenotype-cache-adapter | Storage |
| Event System | phenotype-event-sourcing, phenotype-event-bus | Event-driven |
| Policy & Security | phenotype-policy-engine, phenotype-security-aggregator | Access control |
| Testing | phenotype-test-fixtures, phenotype-testing, phenotype-mock | Test utilities |
| Protocol | phenotype-http-adapter, phenotype-http-client-core | Network |
| Observability | phenotype-telemetry, phenotype-metrics, phenotype-logging, phenotype-health | Monitoring |
| Development | phenotype-macros, phenotype-error-core, phenotype-serde-adapters | Dev tools |
| Integration | phenotype-mcp-core, phenotype-mcp-framework, phenotype-mcp-asset | MCP |
| Utilities | phenotype-time, phenotype-string, phenotype-content-hash | Helpers |

### 1.5 Target Users

1. **Phenotype Service Developers**: Building services on Phenotype platform
2. **External Contributors**: Using Phenotype crates in their projects
3. **Platform Engineers**: Maintaining and extending crate functionality
4. **DevOps Teams**: Deploying services built on these crates

---

## 2. Architecture Strategy

### 2.1 Workspace Architecture

```
crates/
├── Cargo.toml (workspace root)
├── phenotype-core/           # Core types and traits
├── phenotype-domain/         # Domain modeling
├── phenotype-error-core/     # Error handling foundation
├── phenotype-async-traits/   # Async trait definitions
│
├── phenotype-event-sourcing/ # Append-only event store
├── phenotype-event-bus/      # Pub/sub event system
│
├── phenotype-policy-engine/  # Rule-based access control
├── phenotype-security-aggregator/ # Security event collection
│
├── phenotype-cache-adapter/    # Two-tier caching
├── phenotype-postgres-adapter/ # PostgreSQL access
├── phenotype-redis-adapter/    # Redis operations
├── phenotype-in-memory-store/  # In-memory storage
│
├── phenotype-http-adapter/     # HTTP server abstraction
├── phenotype-http-client-core/ # HTTP client foundation
├── phenotype-port-interfaces/  # Port/adapter interfaces
├── phenotype-port-traits/      # Port trait definitions
│
├── phenotype-telemetry/        # Distributed tracing
├── phenotype-metrics/          # Metrics collection
├── phenotype-logging/          # Structured logging
├── phenotype-health/           # Health checks
├── phenotype-health-axum/      # Axum health integration
├── phenotype-health-cli/       # CLI health tools
│
├── phenotype-mcp-core/         # MCP protocol core
├── phenotype-mcp-framework/    # MCP framework
├── phenotype-mcp-asset/        # MCP asset handling
├── phenotype-mcp-testing/      # MCP test utilities
│
├── phenotype-test-fixtures/    # Test data generation
├── phenotype-testing/          # Test harness
├── phenotype-mock/             # Mock implementations
│
├── phenotype-macros/           # Procedural macros
├── phenotype-serde-adapters/   # Serialization helpers
│
├── phenotype-validation/       # Input validation
├── phenotype-retry/            # Retry logic
├── phenotype-time/             # Time utilities
├── phenotype-string/           # String utilities
├── phenotype-content-hash/     # Content hashing
├── phenotype-iter/             # Iterator extensions
│
├── phenotype-project-registry/ # Project metadata
├── phenotype-contracts/        # Interface contracts
├── phenotype-compliance-scanner/ # Compliance checking
│
├── phenotype-bdd/              # BDD testing
├── phenotype-bid/              # Business ID generation
├── phenotype-state-machine/    # FSM implementation
├── phenotype-application/      # Application framework
└── phenotype-infrastructure/ # Infrastructure abstractions
```

### 2.2 Design Principles

#### 2.2.1 Independence Principle

```rust
// Each crate is completely independent
// No "use phenotype_core::..." in phenotype_event_sourcing
// Instead: define own traits or depend on published crates only

// CORRECT: phenotype-event-sourcing defines its own Event trait
pub trait Event: Serialize + DeserializeOwned + Send + Sync {
    fn event_type(&self) -> &str;
    fn timestamp(&self) -> DateTime<Utc>;
    fn aggregate_id(&self) -> &str;
}

// INCORRECT: depending on another internal crate
// use phenotype_core::Event; // DON'T DO THIS
```

#### 2.2.2 Feature Flag Strategy

```toml
[features]
default = ["std"]
std = ["serde/std", "chrono/std"]
async = ["tokio", "futures"]
wasm = ["js-sys", "wasm-bindgen"]
full = ["std", "async", "wasm", "metrics"]

[dependencies]
serde = { version = "1.0", optional = true }
tokio = { version = "1.40", optional = true }
metrics = { version = "0.24", optional = true }
```

### 2.3 Common Patterns

#### 2.3.1 Error Handling Pattern

All crates use `thiserror` for error definitions:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("key not found: {0}")]
    NotFound(String),
    
    #[error("serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("backend error: {0}")]
    Backend(String),
}
```

#### 2.3.2 Trait Definition Pattern

```rust
// Port trait - defines interface
#[async_trait]
pub trait CachePort: Send + Sync {
    async fn get<K, V>(&self, key: &K) -> Result<Option<V>, CacheError>
    where
        K: AsRef<str> + Send,
        V: DeserializeOwned;
    
    async fn set<K, V>(&self, key: &K, value: &V, ttl: Duration) -> Result<(), CacheError>
    where
        K: AsRef<str> + Send,
        V: Serialize;
}

// Adapter implementation
pub struct RedisCacheAdapter {
    client: redis::Client,
}

#[async_trait]
impl CachePort for RedisCacheAdapter {
    // Implementation...
}
```

---

## 3. Implementation Phases

### 3.1 Phase 0: Foundation Crates (Weeks 1-4)

#### 3.1.1 Deliverables

| Week | Crate | Owner | Acceptance Criteria |
|------|-------|-------|---------------------|
| 1 | phenotype-error-core | Core Team | Error trait hierarchy |
| 1 | phenotype-async-traits | Core Team | Async trait definitions |
| 2 | phenotype-core | Core Team | Core types, IDs, timestamps |
| 2 | phenotype-domain | Core Team | Entity, ValueObject traits |
| 3 | phenotype-serde-adapters | Core Team | Serialization helpers |
| 3 | phenotype-string | Core Team | String utilities |
| 4 | phenotype-time | Core Team | Time handling, intervals |
| 4 | phenotype-iter | Core Team | Iterator extensions |

#### 3.1.2 Success Criteria

- [ ] All foundation crates published to internal registry
- [ ] 100% documentation coverage
- [ ] >95% test coverage
- [ ] No dependencies between foundation crates

### 3.2 Phase 1: Data & Event Crates (Weeks 5-10)

#### 3.2.1 Deliverables

| Week | Crate | Owner | Dependencies |
|------|-------|-------|--------------|
| 5-6 | phenotype-event-sourcing | Event Team | Foundation complete |
| 6-7 | phenotype-event-bus | Event Team | Foundation complete |
| 7-8 | phenotype-cache-adapter | Data Team | Foundation complete |
| 8-9 | phenotype-postgres-adapter | Data Team | SQLx integration |
| 9-10 | phenotype-redis-adapter | Data Team | redis crate |
| 10 | phenotype-in-memory-store | Data Team | For testing |

#### 3.2.2 Success Criteria

- [ ] Event sourcing with SHA-256 hash chain
- [ ] Event bus with pub/sub
- [ ] Two-tier cache (LRU + distributed)
- [ ] Type-safe SQLx integration
- [ ] Redis adapter with connection pooling

### 3.3 Phase 2: Protocol & Policy (Weeks 11-16)

#### 3.3.1 Deliverables

| Week | Crate | Owner |
|------|-------|-------|
| 11-12 | phenotype-port-traits | Arch Team |
| 12-13 | phenotype-port-interfaces | Arch Team |
| 13-14 | phenotype-http-adapter | Protocol Team |
| 14-15 | phenotype-http-client-core | Protocol Team |
| 15-16 | phenotype-policy-engine | Security Team |
| 16 | phenotype-security-aggregator | Security Team |

#### 3.3.2 Success Criteria

- [ ] Hexagonal architecture support
- [ ] HTTP adapter with Axum integration
- [ ] HTTP client with retry/circuit breaker
- [ ] Policy engine with TOML configuration
- [ ] Security event aggregation

### 3.4 Phase 3: Observability (Weeks 17-22)

#### 3.4.1 Deliverables

| Week | Crate | Owner |
|------|-------|-------|
| 17-18 | phenotype-telemetry | Observability Team |
| 18-19 | phenotype-metrics | Observability Team |
| 19-20 | phenotype-logging | Observability Team |
| 20-21 | phenotype-health | Observability Team |
| 21-22 | phenotype-health-axum | Observability Team |

#### 3.4.2 Success Criteria

- [ ] OpenTelemetry integration
- [ ] Prometheus metrics export
- [ ] Structured logging with contexts
- [ ] Health check framework
- [ ] Axum health endpoints

### 3.5 Phase 4: MCP & Testing (Weeks 23-28)

#### 3.5.1 Deliverables

| Week | Crate | Owner |
|------|-------|-------|
| 23-24 | phenotype-mcp-core | MCP Team |
| 24-25 | phenotype-mcp-framework | MCP Team |
| 25-26 | phenotype-mcp-asset | MCP Team |
| 26-27 | phenotype-test-fixtures | Testing Team |
| 27-28 | phenotype-testing | Testing Team |
| 28 | phenotype-mcp-testing | MCP Team |

#### 3.5.2 Success Criteria

- [ ] MCP protocol implementation
- [ ] MCP server framework
- [ ] Asset handling for MCP
- [ ] Test fixture generation
- [ ] Integrated testing harness

### 3.6 Phase 5: Application & Utilities (Weeks 29-32)

#### 3.6.1 Deliverables

| Week | Crate | Owner |
|------|-------|-------|
| 29-30 | phenotype-application | App Team |
| 30-31 | phenotype-infrastructure | Infra Team |
| 31 | phenotype-validation | Core Team |
| 31 | phenotype-retry | Core Team |
| 32 | phenotype-content-hash | Core Team |
| 32 | phenotype-project-registry | Core Team |

---

## 4. Technical Stack Decisions

### 4.1 Core Dependencies

| Category | Crate | Version | Purpose |
|----------|-------|---------|---------|
| Async Runtime | tokio | 1.40+ | Async execution |
| Serialization | serde | 1.0+ | Data serialization |
| JSON | serde_json | 1.0+ | JSON handling |
| Error Handling | thiserror | 1.0+ | Error definitions |
| HTTP Server | axum | 0.7+ | Web framework |
| HTTP Client | reqwest | 0.12+ | HTTP requests |
| Database | sqlx | 0.7+ | Type-safe SQL |
| Redis | redis | 0.25+ | Redis client |
| Tracing | tracing | 0.1+ | Structured logging |
| Metrics | metrics | 0.24+ | Metrics collection |
| Testing | tokio-test | 0.4+ | Async testing |
| Mocking | mockall | 0.12+ | Test mocking |
| Protobuf | prost | 0.12+ | Protocol buffers |
| CLI | clap | 4.0+ | CLI parsing |
| UUID | uuid | 1.0+ | UUID generation |
| Time | chrono | 0.4+ | Date/time handling |
| Regex | regex | 1.10+ | Pattern matching |
| Hashing | sha2 | 0.10+ | Cryptographic hashing |
| Random | rand | 0.8+ | Random generation |
| Base64 | base64 | 0.22+ | Base64 encoding |

### 4.2 Workspace Configuration

```toml
# Root Cargo.toml
[workspace]
members = [
    "phenotype-core",
    "phenotype-domain",
    # ... all crates
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/phenotype/crates"
rust-version = "1.75"

[workspace.dependencies]
# Async
tokio = { version = "1.40", features = ["full"] }
futures = "0.3"
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# HTTP
axum = "0.7"
reqwest = { version = "0.12", features = ["json"] }
tower = "0.4"

# Database
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
redis = { version = "0.25", features = ["tokio-comp"] }

# Observability
tracing = "0.1"
tracing-subscriber = "0.3"
metrics = "0.24"
metrics-exporter-prometheus = "0.14"

# Testing
mockall = "0.12"
tokio-test = "0.4"

# Utilities
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
sha2 = "0.10"
hex = "0.4"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.test]
opt-level = 1
```

---

## 5. Risk Analysis & Mitigation

### 5.1 Risk Register

| Risk ID | Description | Likelihood | Impact | Mitigation |
|---------|-------------|------------|--------|------------|
| R-001 | Dependency conflicts between crates | Medium | High | Workspace-level dep management |
| R-002 | Breaking changes in core dependencies | Medium | High | Lock file, gradual updates |
| R-003 | Performance regression | Low | High | Benchmarks CI, flamegraphs |
| R-004 | API inconsistency across crates | Medium | Medium | API review board |
| R-005 | Documentation drift | High | Medium | Doc tests, CI checks |
| R-006 | WASM compatibility issues | Medium | Medium | WASM CI target |
| R-007 | Security vulnerability in dependency | Low | Critical | Automated scanning |
| R-008 | Build time explosion | Medium | Medium | Incremental builds, caching |
| R-009 | Test flakiness | Medium | Medium | Deterministic tests |
| R-010 | Crate publishing failures | Low | Medium | Dry-run testing |

### 5.2 Mitigation Strategies

#### 5.2.1 Dependency Management

```yaml
# dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10
    groups:
      patch-updates:
        update-types: ["patch"]
      minor-updates:
        update-types: ["minor"]
```

#### 5.2.2 API Consistency

- Weekly API review meetings
- Shared design patterns document
- Automated API compatibility checks
- Cross-crate integration tests

---

## 6. Resource Requirements

### 6.1 Team Structure

| Role | Count | Focus |
|------|-------|-------|
| Rust Core Lead | 1 | Architecture, reviews |
| Rust Developers | 4 | Crate implementation |
| Platform Engineer | 1 | CI/CD, releases |
| Technical Writer | 1 | Documentation |
| QA Engineer | 1 | Testing strategy |

### 6.2 Infrastructure

| Resource | Purpose | Cost/Month |
|----------|---------|------------|
| CI/CD runners | Build, test | $500 |
| crates.io publishing | Distribution | $0 |
| Docs hosting | rustdoc | $100 |
| Benchmark runners | Performance | $200 |

---

## 7. Timeline & Milestones

### 7.1 Phase Milestones

| Phase | Target | Key Deliverable |
|-------|--------|-----------------|
| Phase 0 | Week 4 | Foundation crates ready |
| Phase 1 | Week 10 | Data access layer complete |
| Phase 2 | Week 16 | Protocol & policy ready |
| Phase 3 | Week 22 | Observability complete |
| Phase 4 | Week 28 | MCP & testing ready |
| Phase 5 | Week 32 | All crates complete |

### 7.2 Critical Path

Foundation → Event Sourcing → Policy Engine → Observability → Testing

---

## 8. Dependencies & Blockers

### 8.1 External Dependencies

| Dependency | Required By | Status |
|------------|-------------|--------|
| SQLx 0.7 | postgres-adapter | Available |
| Redis 0.25 | redis-adapter | Available |
| Axum 0.7 | http-adapter | Available |

### 8.2 Internal Dependencies

| Dependency | Required By | Status |
|------------|-------------|--------|
| phenotype-core | All crates | In Progress |
| phenotype-error-core | All crates | In Progress |

---

## 9. Testing Strategy

### 9.1 Testing Levels

| Level | Coverage | Tools |
|-------|----------|-------|
| Unit | 90%+ | cargo test |
| Integration | 80%+ | testcontainers |
| Documentation | 100% | cargo test --doc |
| Benchmarks | Key paths | criterion |

### 9.2 CI Pipeline

```yaml
steps:
  - lint: cargo clippy --all-targets -- -D warnings
  - format: cargo fmt --check
  - test: cargo test --workspace
  - doc: cargo doc --no-deps
  - audit: cargo audit
  - coverage: cargo tarpaulin
```

---

## 10. Deployment Plan

### 10.1 Publishing Strategy

| Phase | Action | Trigger |
|-------|--------|---------|
| Alpha | Internal registry | Feature complete |
| Beta | crates.io | Integration tested |
| Stable | crates.io + docs | Production tested |

### 10.2 Version Strategy

- Semantic versioning
- Independent versioning per crate
- CHANGELOG per crate

---

## 11. Rollback Procedures

### 11.1 Crate Yanking

```bash
# If critical bug found
cargo yank --vers 0.1.0

# Document reason
cargo yank --vers 0.1.0 --reason "Security vulnerability"
```

### 11.2 Dependency Rollback

```toml
# Revert in Cargo.toml
[dependencies]
serde = "=1.0.190"  # Pin to working version
```

---

## 12. Post-Launch Monitoring

### 12.1 Crate Metrics

| Metric | Target | Alert |
|--------|--------|-------|
| Downloads | Growing | Decline |
| Issues | <5 open | >10 open |
| Build time | <5 min | >10 min |
| Test time | <2 min | >5 min |

### 12.2 Quality Gates

| Gate | Threshold |
|------|-----------|
| Coverage | >90% |
| Documentation | 100% public APIs |
| Clippy | 0 warnings |
| Audit | 0 vulnerabilities |

---

**Appendices**

### Appendix A: Crate Registry

| Crate | Purpose | Current Version | Status |
|-------|---------|-----------------|--------|
| phenotype-core | Core types | 0.1.0-alpha | In Progress |
| phenotype-event-sourcing | Event store | 0.1.0-alpha | In Progress |
| phenotype-policy-engine | RBAC | 0.1.0-alpha | In Progress |
| ... | ... | ... | ... |

### Appendix B: Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-04-05 | Initial plan |

---

**Document Control**

- **Status**: Draft
- **Next Review**: 2026-05-05
- **Document Owner**: Core Engineering Lead
