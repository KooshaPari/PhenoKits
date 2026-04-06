# PRD — phenotype-gauge (gauge)

## Product Overview

`gauge` is a Rust library providing a modern benchmarking and xDD (cross-Driven Development) testing framework with statistical analysis. It unifies property-based testing strategies, contract testing for hexagonal-architecture ports/adapters, mutation tracking, and specification-driven development (SpecDD) parsing under a single coherent API.

**Status:** Archived (2026-03-25). Source migrated to `libs/gauge/` as the neutral xDD benchmark/test-quality framework under Phenotype monorepo governance. This PRD captures the full intended product scope.

**Package:** `gauge` v0.2.0 (crate name: `gauge`)
**Repository:** `KooshaPari/phenotype-gauge`
**Language:** Rust 2021 edition
**License:** MIT
**Key dependency boundary:** `gauge` handles test-quality and benchmark reporting; it is explicitly NOT a runtime observability or metrics-collection tool (that domain belongs to `phenotype-metrics` / `thegent-metrics`).

---

## Epics and User Stories

### E1 — Property-Based Testing Strategies

**E1.1** As a Rust developer, I want reusable input-validation strategies (UUID, email, URL, integers, strings) so that I can write property tests without re-implementing validators in every project.
- Acceptance: `valid_uuid(s)`, `valid_email(s)`, `valid_url(s)` return `XddResult<&str>`.
- Acceptance: `positive_int(n)` and `bounded_int(n, min, max)` return `XddResult<i64>`.
- Acceptance: All validators return descriptive `XddError` on failure (not panics).

**E1.2** As a Rust developer, I want proptest strategy wrappers that generate valid inputs for the validators so that I can use them in `proptest!` macros directly.
- Acceptance: `uuid_strategy()`, `email_strategy()`, `url_strategy()`, `int_strategy(min, max)`, `non_empty_string_strategy(max_len)` compile and produce values passing their corresponding validators.

**E1.3** As a Rust developer, I want collection guards (`non_empty`, `bounded_length`) so that property tests can assert structural invariants on arbitrary inputs.
- Acceptance: `non_empty(&[])` returns `Err`; `non_empty(&[1])` returns `Ok`.
- Acceptance: `bounded_length(s, min, max)` returns `Err` when `s.len() < min` or `s.len() > max`.

### E2 — Contract Testing (Port/Adapter Verification)

**E2.1** As a hexagonal-architecture developer, I want a `Contract` trait so that I can express the behavioral contract of a port as a reusable test suite independent of any specific adapter.
- Acceptance: `trait Contract { fn name() -> &'static str; fn verify() -> XddResult<()>; }`.
- Acceptance: Multiple adapters can implement the same contract.

**E2.2** As a developer, I want a `ContractVerifier` utility that records assertions and builds a structured `ContractResult` so that I can generate rich failure reports.
- Acceptance: `verifier.assert(condition, expectation, actual)` increments assertion count.
- Acceptance: `verifier.assert_eq(expected, actual, msg)` records a failure when values differ.
- Acceptance: `verifier.result("contract_name")` returns `ContractResult { passed, assertions, failures }`.

**E2.3** As a developer, I want `ContractFailure` to carry `expectation`, `actual`, and optional source `location` so that failures are actionable without reading source code.

### E3 — Mutation Testing Utilities

**E3.1** As a test-quality engineer, I want a `MutationTracker` that records mutation introductions and kill results so that I can calculate a mutation score for a test suite.
- Acceptance: `tracker.introduce_mutation(file, line, kind)` returns a unique mutation ID.
- Acceptance: `tracker.kill_mutation(id)` increments killed count.
- Acceptance: `tracker.mutation_score()` returns a float in `[0.0, 1.0]`; returns `1.0` when no mutations introduced.
- Acceptance: `tracker.mark_equivalent(id)` removes the mutation from the total count so it does not penalize the score.

**E3.2** As a test-quality engineer, I want `MutationKind` variants (Arithmetic, Comparison, Boolean, ValueReplacement, StatementRemoval) so that mutations can be categorized in reports.

**E3.3** As a test-quality engineer, I want a `CoverageReport` that summarizes line coverage from tracker data so that I can inspect overall test coverage alongside mutation score.
- Acceptance: `CoverageReport::from_tracker(&tracker)` compiles without error.
- Acceptance: `line_coverage` is in `[0.0, 1.0]`.

### E4 — SpecDD (Specification-Driven Development)

**E4.1** As a developer, I want to define executable specifications in YAML with `spec`, `features`, and `requirements` sections so that requirements live alongside tests.
- Acceptance: YAML format: `spec: { name, version, description? }`, `features: [{ id, name, scenario?, given?, when?, then? }]`, `requirements: [{ id, description, priority?, status? }]`.

**E4.2** As a developer, I want `SpecParser::parse(yaml)` to return a validated `Spec` struct so that parsing and validation are coupled and cannot be skipped.
- Acceptance: Valid YAML returns `Ok(Spec)`.
- Acceptance: Invalid YAML returns `Err(XddError { category: Spec, ... })`.
- Acceptance: `SpecParser::parse_file(path)` reads from disk and delegates to `parse`.

**E4.3** As a developer, I want `SpecValidator` to enforce that feature IDs are unique, feature names are non-empty, requirement IDs are unique, and every feature has a scenario or at least one given/when/then step.
- Acceptance: Duplicate feature IDs produce a validation error.
- Acceptance: Feature with no scenario and empty given/when/then lists produces a validation error.

**E4.4** As a developer, I want `Priority` (critical, high, medium, low) and `Status` (pending, implemented, verified, deferred) enums on requirements so that spec documents can track implementation state.

### E5 — Domain Error Model

**E5.1** As a library consumer, I want `XddError` with a human-readable `message`, a machine-readable `ErrorCategory`, and optional JSON `context` so that errors can be handled programmatically and displayed clearly.
- Acceptance: `XddError::property(msg)`, `::contract(msg)`, `::mutation(msg)`, `::spec(msg)` constructors exist.
- Acceptance: `error.with_context(key, value)` attaches arbitrary JSON context.
- Acceptance: `XddError` implements `std::error::Error` and `Display`.

### E6 — Benchmarking (Planned, Not Yet Implemented)

**E6.1** As a Rust developer, I want criterion-backed benchmarks with statistical analysis (mean, median, p95, p99, stddev) so that I can detect performance regressions in CI.

**E6.2** As a Rust developer, I want HTML benchmark reports with flamegraphs so that I can visually inspect performance profiles.

**E6.3** As a developer, I want `benchmark!` and `group!` macros that wrap criterion so that benchmark authoring is ergonomic.

---

## Non-Functional Requirements

| Category | Requirement |
|----------|-------------|
| No panics | All public API functions return `XddResult<T>`; no `unwrap()` in library code paths |
| Composability | Strategies, contracts, and validators are designed to compose — no global state |
| Serde | `XddError`, `CoverageReport`, mutation types, and spec structs implement `Serialize`/`Deserialize` |
| Async-compatible | `tokio` is a dependency; library is safe to use inside async contexts |
| SOLID / DRY | Single responsibility per module; shared abstractions avoid duplication across projects |

---

## Boundary Decisions

- `gauge` is a test-quality and benchmarking framework; it does NOT collect runtime telemetry or expose metrics endpoints.
- Runtime observability belongs to `phenotype-metrics` (Rust) and `thegent-metrics`.
- `gauge` wraps `proptest`, `quickcheck`, and `criterion` rather than reimplementing their engines.
