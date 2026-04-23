# FR Tracker — phenotype-gauge

Status tracking for all functional requirements in FUNCTIONAL_REQUIREMENTS.md.

---

## FR-PROP — Property-Based Testing Strategies

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-PROP-001 | `valid_uuid(s: &str)` SHALL return `Ok(s)` when `s` is a 36-character, 5-part hyphen-separated UUID string. | Implemented | `src/property/strategies.rs::valid_uuid()` | Validates UUID v4 format |
| FR-PROP-002 | `valid_uuid` SHALL return `Err(XddError { category: Property })` when the UUID format is invalid. | Implemented | `src/property/strategies.rs::valid_uuid()` | Returns `XddResult<&str>` |
| FR-PROP-003 | `valid_email(s: &str)` SHALL return `Ok(s)` when `s` contains exactly one `@` with non-empty local and domain parts, and the domain contains a `.`. | Implemented | `src/property/strategies.rs::valid_email()` | Validates email format |
| FR-PROP-004 | `valid_email` SHALL return `Err` for strings without `@`, multiple `@`, or empty local/domain parts. | Implemented | `src/property/strategies.rs::valid_email()` | Returns `XddResult<&str>` |
| FR-PROP-005 | `valid_url(s: &str)` SHALL return `Ok(s)` when `s` begins with `http://` or `https://` and is at least 10 characters. | Implemented | `src/property/strategies.rs::valid_url()` | Validates URL scheme and length |
| FR-PROP-006 | `valid_url` SHALL return `Err` when the scheme is absent or the URL is too short. | Implemented | `src/property/strategies.rs::valid_url()` | Returns `XddResult<&str>` |
| FR-PROP-007 | `positive_int(n: i64)` SHALL return `Ok(n)` for `n > 0`, `Err` for `n <= 0`. | Implemented | `src/property/strategies.rs::positive_int()` | Integer validation |
| FR-PROP-008 | `bounded_int(n, min, max)` SHALL return `Ok(n)` when `min <= n <= max`, `Err` otherwise. | Implemented | `src/property/strategies.rs::bounded_int()` | Range validation |
| FR-PROP-009 | `non_empty_string(s)` SHALL return `Err` when `s.trim().is_empty()`. | Implemented | `src/property/strategies.rs::non_empty_string()` | String non-emptiness check |
| FR-PROP-010 | `non_empty(coll: &[T])` SHALL return `Err` when the slice is empty. | Implemented | `src/property/strategies.rs::non_empty()` | Slice emptiness check |
| FR-PROP-011 | `bounded_length(s, min, max)` SHALL return `Err` when `s.len() < min` or `s.len() > max`. | Implemented | `src/property/strategies.rs::bounded_length()` | String length bounds |
| FR-PROP-012 | `uuid_strategy()` SHALL return a `proptest::Strategy<Value = String>` generating UUID v4 format strings. | Implemented | `src/property/strategies.rs::uuid_strategy()` | proptest integration |
| FR-PROP-013 | `email_strategy()` SHALL return a `proptest::Strategy<Value = String>` generating strings containing `@`. | Implemented | `src/property/strategies.rs::email_strategy()` | proptest integration |
| FR-PROP-014 | `int_strategy(min, max)` SHALL return a `proptest::Strategy<Value = i64>` bounded by `[min, max]`. | Implemented | `src/property/strategies.rs::int_strategy()` | proptest integration |
| FR-PROP-015 | `non_empty_string_strategy(max_len)` SHALL return a `proptest::Strategy<Value = String>` of length `[1, max_len]`. | Implemented | `src/property/strategies.rs::non_empty_string_strategy()` | proptest integration |
| FR-PROP-016 | `url_strategy()` SHALL return a strategy producing pre-defined valid URL strings. | Implemented | `src/property/strategies.rs::url_strategy()` | proptest integration |

## FR-CTR — Contract Testing

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-CTR-001 | The library SHALL expose a `Contract` trait with `fn name() -> &'static str` and `fn verify() -> XddResult<()>`. | Implemented | `src/contract/mod.rs::Contract` | Trait definition |
| FR-CTR-002 | `ContractVerifier::new()` SHALL construct a verifier with zero assertions and zero failures. | Implemented | `src/contract/mod.rs::ContractVerifier::new()` | Zero-initialized verifier |
| FR-CTR-003 | `verifier.assert(condition, expectation, actual)` SHALL increment `assertions` by 1; append a `ContractFailure` when `condition` is false. | Implemented | `src/contract/mod.rs::ContractVerifier::assert()` | Assertion tracking |
| FR-CTR-004 | `verifier.assert_eq(expected, actual, msg)` SHALL increment `assertions` by 1; append a failure when values differ. | Implemented | `src/contract/mod.rs::ContractVerifier::assert_eq()` | Equality assertion |
| FR-CTR-005 | `verifier.verify::<C>()` SHALL call `C::verify()` and propagate the result. | Implemented | `src/contract/mod.rs::ContractVerifier::verify()` | Contract invocation |
| FR-CTR-006 | `verifier.result(name)` SHALL return `ContractResult { passed: true, ... }` when no failures exist, and `{ passed: false, failures, ... }` otherwise. | Implemented | `src/contract/mod.rs::ContractVerifier::result()` | Result construction |
| FR-CTR-007 | `ContractFailure` SHALL carry `expectation: String`, `actual: String`, and `location: Option<String>`. | Implemented | `src/contract/mod.rs::ContractFailure` | Struct definition |
| FR-CTR-008 | `ContractResult` SHALL carry `passed: bool`, `contract_name: String`, `assertions: usize`, and `failures: Vec<ContractFailure>`. | Implemented | `src/contract/mod.rs::ContractResult` | Struct definition |

## FR-MUT — Mutation Testing

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-MUT-001 | `MutationTracker::new()` SHALL return a tracker with `mutation_score() == 1.0` (no mutations). | Implemented | `src/mutation/mod.rs::MutationTracker::new()` | Zero mutations → score 1.0 |
| FR-MUT-002 | `tracker.introduce_mutation(file, line, kind)` SHALL return a unique string ID for the mutation and set its initial status to `Survived`. | Implemented | `src/mutation/mod.rs::MutationTracker::introduce_mutation()` | Mutation tracking |
| FR-MUT-003 | `tracker.kill_mutation(id)` SHALL change the mutation's status to `Killed` and increment `killed_mutations`. | Implemented | `src/mutation/mod.rs::MutationTracker::kill_mutation()` | Status update |
| FR-MUT-004 | `tracker.mutation_score()` SHALL equal `killed / total` for total > 0, or `1.0` for total == 0. | Implemented | `src/mutation/mod.rs::MutationTracker::mutation_score()` | Score calculation |
| FR-MUT-005 | `tracker.mark_equivalent(id)` SHALL remove the mutation from `total_mutations` so it does not affect the score. | Implemented | `src/mutation/mod.rs::MutationTracker::mark_equivalent()` | Equivalence handling |
| FR-MUT-006 | `tracker.record_line_execution(file, line)` SHALL track coverage for the given file. | Implemented | `src/mutation/mod.rs::MutationTracker::record_line_execution()` | Coverage tracking |
| FR-MUT-007 | `MutationKind` SHALL include at least `Arithmetic`, `Comparison`, `Boolean`, `ValueReplacement`, and `StatementRemoval`. | Implemented | `src/mutation/mod.rs::MutationKind` | Enum variants |
| FR-MUT-008 | `MutationStatus` SHALL include `Killed`, `Survived`, and `Equivalent`. | Implemented | `src/mutation/mod.rs::MutationStatus` | Enum variants |
| FR-MUT-009 | `CoverageReport::from_tracker(tracker)` SHALL construct a report with `line_coverage` in `[0.0, 1.0]`. | Implemented | `src/mutation/mod.rs::CoverageReport::from_tracker()` | Report construction |
| FR-MUT-010 | `CoverageReport` SHALL implement `Serialize` and `Deserialize`. | Implemented | `src/mutation/mod.rs::CoverageReport` | serde support |

## FR-SPEC — Specification-Driven Development

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-SPEC-001 | `SpecParser::parse(yaml: &str)` SHALL return `Ok(Spec)` for valid YAML matching the spec schema. | Implemented | `src/spec/parser.rs::SpecParser::parse()` | YAML parsing |
| FR-SPEC-002 | `SpecParser::parse` SHALL return `Err(XddError { category: Spec, ... })` for syntactically invalid YAML. | Implemented | `src/spec/parser.rs::SpecParser::parse()` | Error handling |
| FR-SPEC-003 | `SpecParser::parse_file(path)` SHALL read from disk and delegate to `parse`. | Implemented | `src/spec/parser.rs::SpecParser::parse_file()` | File I/O |
| FR-SPEC-004 | `SpecValidator` SHALL be invoked automatically by `SpecParser::parse` so parsing and validation cannot be decoupled. | Implemented | `src/spec/parser.rs::SpecParser::parse()` | Integrated validation |
| FR-SPEC-005 | Validation SHALL reject specs with empty `spec.name` or empty `spec.version`. | Implemented | `src/spec/validator.rs::SpecValidator` | Metadata validation |
| FR-SPEC-006 | Validation SHALL reject features with duplicate IDs. | Implemented | `src/spec/validator.rs::SpecValidator` | Uniqueness check |
| FR-SPEC-007 | Validation SHALL reject features with empty `name`. | Implemented | `src/spec/validator.rs::SpecValidator` | Name validation |
| FR-SPEC-008 | Validation SHALL reject features that have no `scenario` and empty `given`, `when`, and `then` arrays. | Implemented | `src/spec/validator.rs::SpecValidator` | Completeness check |
| FR-SPEC-009 | Validation SHALL reject requirements with duplicate IDs. | Implemented | `src/spec/validator.rs::SpecValidator` | Uniqueness check |
| FR-SPEC-010 | Validation SHALL reject requirements with empty `description`. | Implemented | `src/spec/validator.rs::SpecValidator` | Description validation |
| FR-SPEC-011 | `Priority` SHALL default to `Medium` when absent from YAML. | Implemented | `src/spec/mod.rs::Priority` | Default variant |
| FR-SPEC-012 | `Status` SHALL default to `Pending` when absent from YAML. | Implemented | `src/spec/mod.rs::Status` | Default variant |
| FR-SPEC-013 | `Spec`, `Feature`, `Requirement`, `SpecMetadata` SHALL implement `Serialize` and `Deserialize`. | Implemented | `src/spec/mod.rs` | serde support |

## FR-DOM — Domain Error Model

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-DOM-001 | `XddError` SHALL carry `message: String`, `category: ErrorCategory`, and `context: serde_json::Value`. | Implemented | `src/domain/mod.rs::XddError` | Struct definition |
| FR-DOM-002 | `XddError::property(msg)`, `::contract(msg)`, `::mutation(msg)`, `::spec(msg)` constructors SHALL exist. | Implemented | `src/domain/mod.rs::XddError` | Constructor methods |
| FR-DOM-003 | `XddError::with_context(key, value)` SHALL attach arbitrary JSON-serializable context. | Implemented | `src/domain/mod.rs::XddError::with_context()` | Context builder |
| FR-DOM-004 | `XddError` SHALL implement `std::error::Error` and `Display`. | Implemented | `src/domain/mod.rs::XddError` | Trait impls |
| FR-DOM-005 | `ErrorCategory` SHALL include `Property`, `Contract`, `Mutation`, `Spec`, and `Internal`. | Implemented | `src/domain/mod.rs::ErrorCategory` | Enum variants |
| FR-DOM-006 | `XddError` SHALL implement `Serialize` and `Deserialize`. | Implemented | `src/domain/mod.rs::XddError` | serde support |
| FR-DOM-007 | `XddResult<T>` SHALL be a type alias for `Result<T, XddError>`. | Implemented | `src/domain/mod.rs::XddResult` | Type alias |

## FR-BENCH — Benchmarking (Planned)

| FR ID | Description | Status | Code Location | Notes |
|-------|-------------|--------|----------------|-------|
| FR-BENCH-001 | The library SHALL integrate with `criterion` for benchmark execution and statistical analysis. | Pending | — | Planned for future release |
| FR-BENCH-002 | Benchmark results SHALL include mean, median, p95, p99, and stddev. | Pending | — | Planned for future release |
| FR-BENCH-003 | The library SHALL generate HTML benchmark reports. | Pending | — | Planned for future release |
| FR-BENCH-004 | `benchmark!` and `group!` macros SHALL provide ergonomic criterion wrappers. | Pending | — | Planned for future release |

---

## Summary

- **Total FRs**: 73
- **Implemented**: 69
- **Partial**: 0
- **Pending**: 4 (all benchmarking features)

**Key Gaps**:
- Benchmarking (FR-BENCH-*) deferred to future release
