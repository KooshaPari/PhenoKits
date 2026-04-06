# Architecture Decision Records — dbkit

## ADR-001: sqlx as the Database Driver Wrapper

**Status:** Accepted

**Context:** Rust async database libraries include `sqlx`, `diesel`, `sea-orm`, and `tokio-postgres`. The library must choose one as its foundation.

**Decision:** Wrap `sqlx` as the underlying driver. `sqlx` supports PostgreSQL, MySQL, and SQLite with compile-time query checking (optional) and async-first design.

**Rationale:** `sqlx` is the most widely adopted async Rust database library. Its `FromRow` derive macro and query macros align well with `dbkit`s goal of typed query results without an ORM.

**Alternatives Considered:**
- diesel: sync-first; async support is bolted on; less ergonomic for Phenotype patterns.
- sea-orm: full ORM; more abstraction than `dbkit` intends to provide.
- raw tokio-postgres: no multi-database support; higher boilerplate.

**Consequences:** `dbkit` users gain `sqlx` as a transitive dependency. If `sqlx` is also used directly, versions must align.

---

## ADR-002: Embedded Migrations via include_dir

**Status:** Accepted

**Context:** Migrations can be embedded in the binary or loaded from the filesystem at runtime.

**Decision:** Use `include_dir\!` macro to embed migration files at compile time. Runtime filesystem loading is an optional feature.

**Rationale:** Embedded migrations ensure the binary is self-contained and cannot fail due to missing migration files at deployment.

**Consequences:** Migration changes require recompilation. Large migration histories increase binary size slightly.

---

## ADR-003: thiserror for Error Types

**Status:** Accepted

**Context:** Consistent with ecosystem standard (see `apikit` ADR-002).

**Decision:** Use `thiserror` for `DbError`. `anyhow` is not used in library code.

**Consequences:** Callers see typed errors and can match on variants.
