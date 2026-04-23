# Architecture Decision Records — httpkit

## ADR-001: tower-Compatible Middleware

**Status:** Accepted

**Context:** Rust HTTP middleware patterns vary between frameworks. The library must be composable with existing Axum/Hyper/tower stacks.

**Decision:** All middleware implements `tower::Layer` and `tower::Service`. This makes `httpkit` middleware drop-in compatible with any tower-based HTTP framework.

**Rationale:** `tower` is the de facto standard middleware abstraction in the Rust async ecosystem. Axum, Hyper, and tonic all use tower.

**Alternatives Considered:**
- Framework-specific middleware traits: limits reuse to one framework.
- Custom middleware trait: duplicates tower without benefit.

**Consequences:** `tower` is a direct dependency. Users must understand tower service composition to extend middleware chains.

---

## ADR-002: Deterministic Clock Injection for Time-Dependent Logic

**Status:** Accepted

**Context:** Rate limiters and circuit breakers depend on wall-clock time. Tests must control time to be deterministic.

**Decision:** All time-dependent middleware accepts a clock abstraction (trait or function pointer) that defaults to `std::time::Instant::now()` but can be overridden in tests.

**Rationale:** Avoids `sleep` in tests and makes timing logic fully deterministic.

**Consequences:** Slightly more complex constructor signatures; hidden by sensible defaults.

---

## ADR-003: Zero Unsafe Code

**Status:** Accepted

**Context:** `httpkit` has no performance requirements that justify unsafe Rust.

**Decision:** `#\![forbid(unsafe_code)]` in lib.rs.

**Consequences:** Any future unsafe requirements must be explicitly justified and this ADR updated.
