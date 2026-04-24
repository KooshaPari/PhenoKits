# Architecture Decision Records — phenotype-gauge (gauge)

---

## ADR-001 — Wrap `proptest` and `quickcheck`; do not reimplement property engines

**Status:** Accepted
**Date:** 2026-03-25

### Context

Property-based testing engines (proptest, quickcheck) are mature, well-tested libraries. Writing a new one from scratch would be a significant effort with no quality or performance benefit. The Phenotype xDD governance protocol mandates "wrap/fork/integrate over hand-roll".

### Decision

`gauge` wraps `proptest` (v1.x) for regex-based and numeric strategy generation, and includes `quickcheck` (v1.x) as an alternative. The public `gauge::property::strategies` module exposes validated helpers and proptest strategy wrappers. Users interact with gauge's typed API; gauge delegates to proptest/quickcheck internally.

### Wraps

- `proptest = "1.0"` — strategy generation, `Strategy` trait, `proptest!` macro integration
- `quickcheck = "1.0"` — `Arbitrary` trait-based generation

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| Hand-rolled property engine | Violates wrap-over-handroll mandate; enormous scope |
| Only proptest, no quickcheck | quickcheck's `Arbitrary` approach complements proptest for different use cases |

### Consequences

- Breaking changes in proptest v2+ may require gauge API updates.
- Strategy functions returning `impl Strategy<Value = ...>` tie gauge's return types to proptest's trait bounds.

---

## ADR-002 — Wrap `criterion` for benchmarking

**Status:** Accepted (benchmarking module planned, not yet implemented)
**Date:** 2026-03-25

### Context

Criterion (v0.5) is the de-facto standard for Rust microbenchmarking with statistical rigor. It provides wall-clock timing, statistical analysis, regression detection, and HTML output.

### Decision

`gauge` wraps `criterion = "0.5"` for all benchmark execution. gauge will expose `benchmark!` and `group!` macros that configure criterion measurement groups. The goal is ergonomic macro syntax without removing access to raw criterion configuration.

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| `divan` (newer, no alloc) | Less ecosystem adoption; HTML reports not built in |
| `iai` (instruction counting) | Different measurement model; not statistical |
| Raw criterion without wrapper | No added value from gauge layer |

### Consequences

- `gauge` must re-export criterion bench harness correctly (`harness = false` in Cargo.toml).
- HTML report generation relies on criterion's built-in gnuplot integration.

---

## ADR-003 — Hexagonal architecture for internal module structure

**Status:** Accepted
**Date:** 2026-03-25

### Context

The gauge library itself follows the xDD methodology it supports. Applying hexagonal architecture internally demonstrates the patterns gauge is designed to test and validates that the library's design is consistent with its philosophy.

### Decision

Three layers:

1. **Domain layer** (`src/domain/`) — Pure business logic; zero external dependencies. Defines `XddError`, `XddResult`, `ErrorCategory`. No `use proptest::*` here.
2. **Application layer** — Use cases and port interfaces. Currently thin; will house spec validation orchestration.
3. **Infrastructure layer** — Adapters to proptest (`src/property/`), contract verifier (`src/contract/`), mutation tracker (`src/mutation/`), spec parser (`src/spec/`). These may import serde, proptest, serde_yaml.

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| Flat module structure | Mixes domain and infrastructure; violates SOLID SRP |
| One module per use case | Too granular for a library this size |

### Consequences

- The domain layer has no external deps beyond `serde`; it compiles faster and is trivially testable.
- Infrastructure modules may import proptest or serde_yaml but must not leak those types into the domain layer's public API.

---

## ADR-004 — `XddError` with `serde_json::Value` context over typed error variants

**Status:** Accepted
**Date:** 2026-03-25

### Context

Error types can be designed as rich enums (one variant per error condition) or as a single struct with a category discriminant and a freeform context bag. For a cross-cutting library that must work across property, contract, mutation, and spec domains, a single flexible error struct reduces duplication.

### Decision

`XddError` is a struct with:
- `message: String` — human-readable description
- `category: ErrorCategory` — machine-readable category (5 variants)
- `context: serde_json::Value` — optional structured context

Constructor functions (`XddError::property`, `::contract`, `::mutation`, `::spec`) set the correct category. `with_context(key, value)` attaches arbitrary JSON-serializable data.

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| Large enum with one variant per error | Combinatorial explosion across 4 domains; consumer match statements become fragile |
| `anyhow::Error` | Loses structured category; harder to handle programmatically |
| `thiserror` derive macros | Good option but same explosion problem without a unifying struct |

### Consequences

- Consumers cannot exhaustively match on every possible error condition via the type system alone; they must match on `category`.
- `serde_json::Value` context field makes error serialization trivially easy.

---

## ADR-005 — `SpecParser::parse` couples parsing and validation; no split API

**Status:** Accepted
**Date:** 2026-03-25

### Context

It would be possible to expose `parse_yaml` (raw YAML parse) separately from `SpecValidator::validate`. However, returning an unvalidated `Spec` invites callers to skip validation and use malformed data.

### Decision

`SpecParser::parse` always runs `SpecValidator` after successful YAML parse. The raw `parser::parse_yaml` function is module-private. There is no public API for obtaining an unvalidated `Spec`.

### Alternatives Considered

| Alternative | Reason Rejected |
|-------------|----------------|
| Separate `parse` and `validate` public methods | Callers may skip validation; fail-loud mandate requires correctness by construction |
| Builder pattern with validation step | More ergonomic but same risk of skipping |

### Consequences

- Callers cannot parse a spec without validating it (correct behavior).
- If a caller needs to intentionally bypass validation, they must contribute a well-justified PR to add a `parse_unchecked` API.

---

## ADR-006 — Migration: phenotype-gauge -> libs/gauge

**Status:** Accepted
**Date:** 2026-03-25

### Context

Same governance rationale as ADR-005 in phenotype-nexus: neutral xDD libraries belong under `libs/` in the Phenotype monorepo.

### Decision

Migrate `phenotype-gauge` to `libs/gauge`. The `phenotype-gauge` GitHub repo is archived with `ARCHIVED.md` pointing to the new location.

### Consequences

- New consumers reference `libs/gauge` (path dep) or the published crate.
- `phenotype-gauge` repo is read-only and receives no further development.
