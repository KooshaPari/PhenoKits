# Functional Requirements — phenotype-gauge (gauge)

Traces to: PRD.md epics E1–E6.

---

## FR-PROP — Property-Based Testing Strategies

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-PROP-001 | `valid_uuid(s: &str)` SHALL return `Ok(s)` when `s` is a 36-character, 5-part hyphen-separated UUID string. | Critical | Implemented | E1.1 |
| FR-PROP-002 | `valid_uuid` SHALL return `Err(XddError { category: Property })` when the UUID format is invalid. | Critical | Implemented | E1.1 |
| FR-PROP-003 | `valid_email(s: &str)` SHALL return `Ok(s)` when `s` contains exactly one `@` with non-empty local and domain parts, and the domain contains a `.`. | Critical | Implemented | E1.1 |
| FR-PROP-004 | `valid_email` SHALL return `Err` for strings without `@`, multiple `@`, or empty local/domain parts. | Critical | Implemented | E1.1 |
| FR-PROP-005 | `valid_url(s: &str)` SHALL return `Ok(s)` when `s` begins with `http://` or `https://` and is at least 10 characters. | High | Implemented | E1.1 |
| FR-PROP-006 | `valid_url` SHALL return `Err` when the scheme is absent or the URL is too short. | High | Implemented | E1.1 |
| FR-PROP-007 | `positive_int(n: i64)` SHALL return `Ok(n)` for `n > 0`, `Err` for `n <= 0`. | High | Implemented | E1.1 |
| FR-PROP-008 | `bounded_int(n, min, max)` SHALL return `Ok(n)` when `min <= n <= max`, `Err` otherwise. | High | Implemented | E1.1 |
| FR-PROP-009 | `non_empty_string(s)` SHALL return `Err` when `s.trim().is_empty()`. | High | Implemented | E1.3 |
| FR-PROP-010 | `non_empty(coll: &[T])` SHALL return `Err` when the slice is empty. | High | Implemented | E1.3 |
| FR-PROP-011 | `bounded_length(s, min, max)` SHALL return `Err` when `s.len() < min` or `s.len() > max`. | High | Implemented | E1.3 |
| FR-PROP-012 | `uuid_strategy()` SHALL return a `proptest::Strategy<Value = String>` generating UUID v4 format strings. | Medium | Implemented | E1.2 |
| FR-PROP-013 | `email_strategy()` SHALL return a `proptest::Strategy<Value = String>` generating strings containing `@`. | Medium | Implemented | E1.2 |
| FR-PROP-014 | `int_strategy(min, max)` SHALL return a `proptest::Strategy<Value = i64>` bounded by `[min, max]`. | Medium | Implemented | E1.2 |
| FR-PROP-015 | `non_empty_string_strategy(max_len)` SHALL return a `proptest::Strategy<Value = String>` of length `[1, max_len]`. | Medium | Implemented | E1.2 |
| FR-PROP-016 | `url_strategy()` SHALL return a strategy producing pre-defined valid URL strings. | Low | Implemented | E1.2 |

## FR-CTR — Contract Testing

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-CTR-001 | The library SHALL expose a `Contract` trait with `fn name() -> &'static str` and `fn verify() -> XddResult<()>`. | Critical | Implemented | E2.1 |
| FR-CTR-002 | `ContractVerifier::new()` SHALL construct a verifier with zero assertions and zero failures. | Critical | Implemented | E2.2 |
| FR-CTR-003 | `verifier.assert(condition, expectation, actual)` SHALL increment `assertions` by 1; append a `ContractFailure` when `condition` is false. | Critical | Implemented | E2.2 |
| FR-CTR-004 | `verifier.assert_eq(expected, actual, msg)` SHALL increment `assertions` by 1; append a failure when values differ. | High | Implemented | E2.2 |
| FR-CTR-005 | `verifier.verify::<C>()` SHALL call `C::verify()` and propagate the result. | High | Implemented | E2.2 |
| FR-CTR-006 | `verifier.result(name)` SHALL return `ContractResult { passed: true, ... }` when no failures exist, and `{ passed: false, failures, ... }` otherwise. | Critical | Implemented | E2.2 |
| FR-CTR-007 | `ContractFailure` SHALL carry `expectation: String`, `actual: String`, and `location: Option<String>`. | High | Implemented | E2.3 |
| FR-CTR-008 | `ContractResult` SHALL carry `passed: bool`, `contract_name: String`, `assertions: usize`, and `failures: Vec<ContractFailure>`. | High | Implemented | E2.2 |

## FR-MUT — Mutation Testing

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-MUT-001 | `MutationTracker::new()` SHALL return a tracker with `mutation_score() == 1.0` (no mutations). | Critical | Implemented | E3.1 |
| FR-MUT-002 | `tracker.introduce_mutation(file, line, kind)` SHALL return a unique string ID for the mutation and set its initial status to `Survived`. | Critical | Implemented | E3.1 |
| FR-MUT-003 | `tracker.kill_mutation(id)` SHALL change the mutation's status to `Killed` and increment `killed_mutations`. | Critical | Implemented | E3.1 |
| FR-MUT-004 | `tracker.mutation_score()` SHALL equal `killed / total` for total > 0, or `1.0` for total == 0. | Critical | Implemented | E3.1 |
| FR-MUT-005 | `tracker.mark_equivalent(id)` SHALL remove the mutation from `total_mutations` so it does not affect the score. | High | Implemented | E3.1 |
| FR-MUT-006 | `tracker.record_line_execution(file, line)` SHALL track coverage for the given file. | High | Implemented | E3.3 |
| FR-MUT-007 | `MutationKind` SHALL include at least `Arithmetic`, `Comparison`, `Boolean`, `ValueReplacement`, and `StatementRemoval`. | High | Implemented | E3.2 |
| FR-MUT-008 | `MutationStatus` SHALL include `Killed`, `Survived`, and `Equivalent`. | High | Implemented | E3.2 |
| FR-MUT-009 | `CoverageReport::from_tracker(tracker)` SHALL construct a report with `line_coverage` in `[0.0, 1.0]`. | Medium | Implemented | E3.3 |
| FR-MUT-010 | `CoverageReport` SHALL implement `Serialize` and `Deserialize`. | Medium | Implemented | PRD NFR |

## FR-SPEC — Specification-Driven Development

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-SPEC-001 | `SpecParser::parse(yaml: &str)` SHALL return `Ok(Spec)` for valid YAML matching the spec schema. | Critical | Implemented | E4.2 |
| FR-SPEC-002 | `SpecParser::parse` SHALL return `Err(XddError { category: Spec, ... })` for syntactically invalid YAML. | Critical | Implemented | E4.2 |
| FR-SPEC-003 | `SpecParser::parse_file(path)` SHALL read from disk and delegate to `parse`. | High | Implemented | E4.2 |
| FR-SPEC-004 | `SpecValidator` SHALL be invoked automatically by `SpecParser::parse` so parsing and validation cannot be decoupled. | High | Implemented | E4.2 |
| FR-SPEC-005 | Validation SHALL reject specs with empty `spec.name` or empty `spec.version`. | Critical | Implemented | E4.3 |
| FR-SPEC-006 | Validation SHALL reject features with duplicate IDs. | High | Implemented | E4.3 |
| FR-SPEC-007 | Validation SHALL reject features with empty `name`. | High | Implemented | E4.3 |
| FR-SPEC-008 | Validation SHALL reject features that have no `scenario` and empty `given`, `when`, and `then` arrays. | High | Implemented | E4.3 |
| FR-SPEC-009 | Validation SHALL reject requirements with duplicate IDs. | High | Implemented | E4.3 |
| FR-SPEC-010 | Validation SHALL reject requirements with empty `description`. | High | Implemented | E4.3 |
| FR-SPEC-011 | `Priority` SHALL default to `Medium` when absent from YAML. | Medium | Implemented | E4.4 |
| FR-SPEC-012 | `Status` SHALL default to `Pending` when absent from YAML. | Medium | Implemented | E4.4 |
| FR-SPEC-013 | `Spec`, `Feature`, `Requirement`, `SpecMetadata` SHALL implement `Serialize` and `Deserialize`. | Medium | Implemented | PRD NFR |

## FR-DOM — Domain Error Model

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-DOM-001 | `XddError` SHALL carry `message: String`, `category: ErrorCategory`, and `context: serde_json::Value`. | Critical | Implemented | E5.1 |
| FR-DOM-002 | `XddError::property(msg)`, `::contract(msg)`, `::mutation(msg)`, `::spec(msg)` constructors SHALL exist. | Critical | Implemented | E5.1 |
| FR-DOM-003 | `XddError::with_context(key, value)` SHALL attach arbitrary JSON-serializable context. | High | Implemented | E5.1 |
| FR-DOM-004 | `XddError` SHALL implement `std::error::Error` and `Display`. | Critical | Implemented | E5.1 |
| FR-DOM-005 | `ErrorCategory` SHALL include `Property`, `Contract`, `Mutation`, `Spec`, and `Internal`. | Critical | Implemented | E5.1 |
| FR-DOM-006 | `XddError` SHALL implement `Serialize` and `Deserialize`. | High | Implemented | PRD NFR |
| FR-DOM-007 | `XddResult<T>` SHALL be a type alias for `Result<T, XddError>`. | Critical | Implemented | E5.1 |

## FR-BENCH — Benchmarking (Planned)

| ID | Requirement | Priority | Status | Traces To |
|----|-------------|----------|--------|-----------|
| FR-BENCH-001 | The library SHALL integrate with `criterion` for benchmark execution and statistical analysis. | High | Pending | E6.1 |
| FR-BENCH-002 | Benchmark results SHALL include mean, median, p95, p99, and stddev. | High | Pending | E6.1 |
| FR-BENCH-003 | The library SHALL generate HTML benchmark reports. | Medium | Pending | E6.2 |
| FR-BENCH-004 | `benchmark!` and `group!` macros SHALL provide ergonomic criterion wrappers. | Medium | Pending | E6.3 |
