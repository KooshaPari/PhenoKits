# Code Entity Map — phenotype-gauge

Mapping between functional requirements and code entities (types, functions, modules).

---

## Module: `property`

**Location**: `src/property/`

**Purpose**: Property-based testing strategies and validators.

### Submodule: `strategies`

**Location**: `src/property/strategies.rs`

**Purpose**: Validation functions and proptest strategy generators.

#### Type: Validation Functions

**Location**: `src/property/strategies.rs`

**Purpose**: Validate input strings and numbers against expected formats.

**Related FRs**: FR-PROP-001–016

**Functions**:

| Function | Purpose | Related FRs | Return Type |
|----------|---------|------------|-------------|
| `valid_uuid(s: &str)` | Validate UUID v4 format (36-char, 5-part hyphen-separated) | FR-PROP-001, FR-PROP-002 | `XddResult<&str>` |
| `valid_email(s: &str)` | Validate email format (one `@`, non-empty local/domain, domain has `.`) | FR-PROP-003, FR-PROP-004 | `XddResult<&str>` |
| `valid_url(s: &str)` | Validate URL (http/https scheme, min 10 chars) | FR-PROP-005, FR-PROP-006 | `XddResult<&str>` |
| `positive_int(n: i64)` | Validate integer > 0 | FR-PROP-007 | `XddResult<i64>` |
| `bounded_int(n, min, max)` | Validate integer within bounds `[min, max]` | FR-PROP-008 | `XddResult<i64>` |
| `non_empty_string(s)` | Validate string is not empty after trim | FR-PROP-009 | `XddResult<&str>` |
| `non_empty(coll: &[T])` | Validate slice is not empty | FR-PROP-010 | `XddResult<&[T]>` |
| `bounded_length(s, min, max)` | Validate string length in `[min, max]` | FR-PROP-011 | `XddResult<&str>` |

#### Type: Strategy Generators

**Location**: `src/property/strategies.rs`

**Purpose**: Generate test inputs for proptest property-based testing.

**Related FRs**: FR-PROP-012–016

**Functions**:

| Function | Returns | Related FRs |
|----------|---------|------------|
| `uuid_strategy()` | `Strategy<Value = String>` | FR-PROP-012 |
| `email_strategy()` | `Strategy<Value = String>` | FR-PROP-013 |
| `int_strategy(min, max)` | `Strategy<Value = i64>` | FR-PROP-014 |
| `non_empty_string_strategy(max_len)` | `Strategy<Value = String>` | FR-PROP-015 |
| `url_strategy()` | `Strategy<Value = String>` | FR-PROP-016 |

---

## Module: `contract`

**Location**: `src/contract/mod.rs`

**Purpose**: Contract-driven development (CDD) for verifying port/adapter implementations.

### Type: `Contract` (Trait)

**Location**: `src/contract/mod.rs::Contract`

**Purpose**: Define a contract that an adapter must satisfy.

**Related FRs**: FR-CTR-001, FR-CTR-005

**Methods**:
- `name() -> &'static str` — Contract identifier
- `verify() -> XddResult<()>` — Execute contract verification

**Example Implementation**:
```rust
impl Contract for MyAdapter {
    fn name() -> &'static str { "my_adapter" }
    fn verify() -> XddResult<()> { Ok(()) }
}
```

---

### Type: `ContractVerifier`

**Location**: `src/contract/mod.rs::ContractVerifier`

**Purpose**: Verify contracts and track assertions.

**Fields**:
- `assertions: usize` — Count of assertions made
- `failures: Vec<ContractFailure>` — Failed assertions

**Related FRs**: FR-CTR-002, FR-CTR-003, FR-CTR-004, FR-CTR-005, FR-CTR-006

**Methods**:

#### `new() -> Self`
- **Purpose**: Create zero-initialized verifier (FR-CTR-002)
- **Returns**: `ContractVerifier` with 0 assertions, 0 failures

#### `assert(&mut self, condition: bool, expectation: &str, actual: &str)`
- **Purpose**: Record an assertion (FR-CTR-003)
- **Behavior**: Increment `assertions` by 1; append `ContractFailure` if condition is false

#### `assert_eq(&mut self, expected: impl PartialEq + Debug, actual: impl PartialEq + Debug, msg: &str)`
- **Purpose**: Record equality assertion (FR-CTR-004)
- **Behavior**: Increment `assertions` by 1; append failure if values differ

#### `verify<C: Contract>(&mut self) -> XddResult<()>`
- **Purpose**: Execute contract verification (FR-CTR-005)
- **Behavior**: Call `C::verify()` and propagate result

#### `result(&self, contract_name: &str) -> ContractResult`
- **Purpose**: Generate result summary (FR-CTR-006)
- **Returns**: `ContractResult { passed: assertions > 0 && failures.is_empty(), ... }`

---

### Type: `ContractFailure`

**Location**: `src/contract/mod.rs::ContractFailure`

**Purpose**: Record a single assertion failure.

**Fields**:
- `expectation: String` — Expected value (FR-CTR-007)
- `actual: String` — Actual value (FR-CTR-007)
- `location: Option<String>` — File/line of assertion (FR-CTR-007)

**Related FRs**: FR-CTR-003, FR-CTR-004, FR-CTR-007

---

### Type: `ContractResult`

**Location**: `src/contract/mod.rs::ContractResult`

**Purpose**: Verification result with failure details.

**Fields**:
- `passed: bool` — Whether all assertions passed (FR-CTR-006)
- `contract_name: String` — Name of verified contract (FR-CTR-008)
- `assertions: usize` — Total assertions run (FR-CTR-008)
- `failures: Vec<ContractFailure>` — Details of failures (FR-CTR-008)

**Related FRs**: FR-CTR-006, FR-CTR-008

---

## Module: `mutation`

**Location**: `src/mutation/mod.rs`

**Purpose**: Mutation testing utilities to measure test quality.

### Type: `MutationTracker`

**Location**: `src/mutation/mod.rs::MutationTracker`

**Purpose**: Track mutations and calculate mutation score.

**Fields**:
- `files: HashMap<String, FileCoverage>` — Per-file coverage tracking
- `total_mutations: usize` — Count of all mutations introduced
- `killed_mutations: usize` — Count of mutations killed by tests

**Related FRs**: FR-MUT-001–010

**Methods**:

#### `new() -> Self`
- **Purpose**: Create tracker with score 1.0 (FR-MUT-001)
- **Returns**: `MutationTracker` with 0 mutations

#### `introduce_mutation(&mut self, file: &str, line: usize, kind: MutationKind) -> String`
- **Purpose**: Add a mutation and return its ID (FR-MUT-002)
- **Behavior**: Generate unique ID, set status to `Survived`, increment `total_mutations`
- **Returns**: Mutation ID string

#### `kill_mutation(&mut self, id: &str)`
- **Purpose**: Mark mutation as killed (FR-MUT-003)
- **Behavior**: Change status to `Killed`, increment `killed_mutations`

#### `mutation_score(&self) -> f64`
- **Purpose**: Calculate kill rate (FR-MUT-004)
- **Formula**: `killed / total` if total > 0, else `1.0`

#### `mark_equivalent(&mut self, id: &str)`
- **Purpose**: Exclude equivalent mutation from score (FR-MUT-005)
- **Behavior**: Change status to `Equivalent`, decrement `total_mutations`

#### `record_line_execution(&mut self, file: &str, line: usize)`
- **Purpose**: Track coverage for line (FR-MUT-006)
- **Behavior**: Increment coverage counters for file

---

### Type: `MutationStatus` (Enum)

**Location**: `src/mutation/mod.rs::MutationStatus`

**Purpose**: State of a mutation.

**Variants**:
- `Killed` — Mutation was caught by test (FR-MUT-008)
- `Survived` — Mutation escaped all tests (FR-MUT-008)
- `Equivalent` — Mutation semantically identical to original (FR-MUT-008)

**Related FRs**: FR-MUT-003, FR-MUT-005, FR-MUT-008

---

### Type: `MutationKind` (Enum)

**Location**: `src/mutation/mod.rs::MutationKind`

**Purpose**: Classify types of mutations.

**Variants**:
- `Arithmetic` — Operator flipped (+ to -) (FR-MUT-007)
- `Comparison` — Comparison changed (== to !=) (FR-MUT-007)
- `Boolean` — Boolean operator negated (&& to ||) (FR-MUT-007)
- `ValueReplacement` — Value replaced with default/null (FR-MUT-007)
- `StatementRemoval` — Statement deleted (FR-MUT-007)

**Related FRs**: FR-MUT-007

---

### Type: `CoverageReport`

**Location**: `src/mutation/mod.rs::CoverageReport`

**Purpose**: Aggregate coverage metrics.

**Fields**:
- `line_coverage: f64` — Fraction of lines executed (FR-MUT-009)
- `branch_coverage: f64` — Fraction of branches executed
- `mutation_score: f64` — Kill rate

**Related FRs**: FR-MUT-009, FR-MUT-010

**Methods**:

#### `from_tracker(tracker: &MutationTracker) -> Self`
- **Purpose**: Build report from tracker (FR-MUT-009)
- **Returns**: `CoverageReport` with calculated metrics

**Traits**:
- `Serialize` (FR-MUT-010)
- `Deserialize` (FR-MUT-010)

---

## Module: `spec`

**Location**: `src/spec/`

**Purpose**: Specification-driven development (SpecDD) — parse, validate, and execute specs.

### Submodule: `parser`

**Location**: `src/spec/parser.rs`

**Purpose**: Parse YAML specifications.

#### Type: `SpecParser`

**Location**: `src/spec/parser.rs::SpecParser`

**Purpose**: Parse and validate YAML specifications.

**Related FRs**: FR-SPEC-001–004

**Methods**:

#### `parse(yaml: &str) -> XddResult<Spec>`
- **Purpose**: Parse YAML and validate (FR-SPEC-001, FR-SPEC-004)
- **Behavior**: Parse YAML → validate → return or error with category `Spec` (FR-SPEC-002)
- **Returns**: `XddResult<Spec>`
- **Error**: `XddError { category: Spec, ... }` (FR-SPEC-002)

#### `parse_file(path: &str) -> XddResult<Spec>`
- **Purpose**: Read file and parse (FR-SPEC-003)
- **Behavior**: Read file → call `parse()` → propagate result

---

### Submodule: `validator`

**Location**: `src/spec/validator.rs`

**Purpose**: Validate spec structure and content.

#### Type: `SpecValidator`

**Location**: `src/spec/validator.rs::SpecValidator`

**Purpose**: Validate parsed specifications against rules.

**Related FRs**: FR-SPEC-005–010

**Validation Rules**:

| Rule | Check | Related FR |
|------|-------|-----------|
| Spec name non-empty | `spec.name.is_empty()` → error | FR-SPEC-005 |
| Spec version non-empty | `spec.version.is_empty()` → error | FR-SPEC-005 |
| Feature IDs unique | No duplicate `feature.id` | FR-SPEC-006 |
| Feature name non-empty | `feature.name.is_empty()` → error | FR-SPEC-007 |
| Feature completeness | Has `scenario` OR all of `given`, `when`, `then` non-empty | FR-SPEC-008 |
| Requirement IDs unique | No duplicate `requirement.id` | FR-SPEC-009 |
| Requirement description non-empty | `requirement.description.is_empty()` → error | FR-SPEC-010 |

---

### Type: `Spec`

**Location**: `src/spec/mod.rs::Spec`

**Purpose**: Root specification document.

**Fields**:
- `spec: SpecMetadata` — Name, version, description
- `features: Vec<Feature>` — BDD features
- `requirements: Vec<Requirement>` — Domain requirements

**Related FRs**: FR-SPEC-001, FR-SPEC-013

**Traits**:
- `Serialize` (FR-SPEC-013)
- `Deserialize` (FR-SPEC-013)

---

### Type: `SpecMetadata`

**Location**: `src/spec/mod.rs::SpecMetadata`

**Purpose**: Specification header.

**Fields**:
- `name: String` — Spec title (FR-SPEC-005)
- `version: String` — Version ID (FR-SPEC-005)
- `description: Option<String>` — Free-form description

**Related FRs**: FR-SPEC-005, FR-SPEC-013

**Traits**:
- `Serialize` (FR-SPEC-013)
- `Deserialize` (FR-SPEC-013)

---

### Type: `Feature`

**Location**: `src/spec/mod.rs::Feature`

**Purpose**: BDD feature with scenarios.

**Fields**:
- `id: String` — Feature ID (FR-SPEC-006)
- `name: String` — Feature name (FR-SPEC-007)
- `description: Option<String>` — Free-form description
- `scenario: Option<Scenario>` — Single scenario (FR-SPEC-008)
- `given: Vec<Condition>` — Setup conditions (FR-SPEC-008)
- `when: Vec<Action>` — User actions (FR-SPEC-008)
- `then: Vec<Assertion>` — Expected outcomes (FR-SPEC-008)

**Related FRs**: FR-SPEC-006–008, FR-SPEC-013

---

### Type: `Requirement`

**Location**: `src/spec/mod.rs::Requirement`

**Purpose**: Domain requirement.

**Fields**:
- `id: String` — Requirement ID (FR-SPEC-009)
- `description: String` — Requirement text (FR-SPEC-010)
- `priority: Priority` — Importance level (FR-SPEC-011)
- `status: Status` — Implementation status (FR-SPEC-012)

**Related FRs**: FR-SPEC-009–013

---

### Type: `Priority` (Enum)

**Location**: `src/spec/mod.rs::Priority`

**Purpose**: Classify requirement importance.

**Variants**:
- `Critical`
- `High`
- `Medium` — Default (FR-SPEC-011)
- `Low`

**Related FRs**: FR-SPEC-011, FR-SPEC-013

---

### Type: `Status` (Enum)

**Location**: `src/spec/mod.rs::Status`

**Purpose**: Track requirement implementation state.

**Variants**:
- `Pending` — Default (FR-SPEC-012)
- `Implemented`
- `Partial`

**Related FRs**: FR-SPEC-012, FR-SPEC-013

---

## Module: `domain`

**Location**: `src/domain/mod.rs`

**Purpose**: Domain layer with pure business logic and error handling.

### Type: `XddError`

**Location**: `src/domain/mod.rs::XddError`

**Purpose**: Unified error type for all xDD operations.

**Fields**:
- `message: String` — Human-readable error message (FR-DOM-001)
- `category: ErrorCategory` — Error classification (FR-DOM-001)
- `context: serde_json::Value` — Additional context (FR-DOM-001)

**Related FRs**: FR-DOM-001, FR-DOM-004, FR-DOM-006, FR-DOM-007

**Methods**:

#### `new(message, category) -> Self`
- **Purpose**: Create error with message and category (FR-DOM-001)

#### `property(msg) -> Self`
- **Purpose**: Create property-testing error (FR-DOM-002)

#### `contract(msg) -> Self`
- **Purpose**: Create contract-testing error (FR-DOM-002)

#### `mutation(msg) -> Self`
- **Purpose**: Create mutation-testing error (FR-DOM-002)

#### `spec(msg) -> Self`
- **Purpose**: Create spec-parsing error (FR-DOM-002)

#### `with_context(key, value) -> Self`
- **Purpose**: Attach JSON context (FR-DOM-003)
- **Behavior**: Insert key/value into `context` field

**Traits**:
- `std::error::Error` (FR-DOM-004)
- `Display` (FR-DOM-004)
- `Serialize` (FR-DOM-006)
- `Deserialize` (FR-DOM-006)

---

### Type: `ErrorCategory` (Enum)

**Location**: `src/domain/mod.rs::ErrorCategory`

**Purpose**: Classify error origin.

**Variants**:
- `Property` — Property-based testing error (FR-DOM-005)
- `Contract` — Contract testing error (FR-DOM-005)
- `Mutation` — Mutation tracking error (FR-DOM-005)
- `Spec` — Specification parsing error (FR-DOM-005)
- `Internal` — Library internal error (FR-DOM-005)

**Related FRs**: FR-DOM-002, FR-DOM-005

**Traits**:
- `Serialize` (FR-DOM-006)
- `Deserialize` (FR-DOM-006)

---

### Type Alias: `XddResult<T>`

**Location**: `src/domain/mod.rs::XddResult`

**Purpose**: Convenient result type for xDD operations.

**Definition**: `type XddResult<T> = Result<T, XddError>;`

**Related FRs**: FR-DOM-007

---

## Traceability Summary

### FRs Fully Implemented (Code Exists)
- FR-PROP-001–016 (all property validation and strategies)
- FR-CTR-001–008 (all contract testing)
- FR-MUT-001–010 (all mutation tracking)
- FR-SPEC-001–013 (all spec parsing, validation, models)
- FR-DOM-001–007 (all error handling)

### FRs Not Yet Implemented
- FR-BENCH-001–004 (benchmarking deferred to future release)

---

## Forward Mapping: FR → Code

| FR | Module | Entity | Method |
|----|--------|--------|--------|
| FR-PROP-001–002 | property | strategies | `valid_uuid()` |
| FR-PROP-003–004 | property | strategies | `valid_email()` |
| FR-PROP-005–006 | property | strategies | `valid_url()` |
| FR-PROP-007 | property | strategies | `positive_int()` |
| FR-PROP-008 | property | strategies | `bounded_int()` |
| FR-PROP-009 | property | strategies | `non_empty_string()` |
| FR-PROP-010 | property | strategies | `non_empty()` |
| FR-PROP-011 | property | strategies | `bounded_length()` |
| FR-PROP-012 | property | strategies | `uuid_strategy()` |
| FR-PROP-013 | property | strategies | `email_strategy()` |
| FR-PROP-014 | property | strategies | `int_strategy()` |
| FR-PROP-015 | property | strategies | `non_empty_string_strategy()` |
| FR-PROP-016 | property | strategies | `url_strategy()` |
| FR-CTR-001 | contract | Contract | trait definition |
| FR-CTR-002 | contract | ContractVerifier | `new()` |
| FR-CTR-003 | contract | ContractVerifier | `assert()` |
| FR-CTR-004 | contract | ContractVerifier | `assert_eq()` |
| FR-CTR-005 | contract | ContractVerifier | `verify()` |
| FR-CTR-006 | contract | ContractVerifier | `result()` |
| FR-CTR-007 | contract | ContractFailure | struct fields |
| FR-CTR-008 | contract | ContractResult | struct fields |
| FR-MUT-001 | mutation | MutationTracker | `new()` |
| FR-MUT-002 | mutation | MutationTracker | `introduce_mutation()` |
| FR-MUT-003 | mutation | MutationTracker | `kill_mutation()` |
| FR-MUT-004 | mutation | MutationTracker | `mutation_score()` |
| FR-MUT-005 | mutation | MutationTracker | `mark_equivalent()` |
| FR-MUT-006 | mutation | MutationTracker | `record_line_execution()` |
| FR-MUT-007 | mutation | MutationKind | enum variants |
| FR-MUT-008 | mutation | MutationStatus | enum variants |
| FR-MUT-009 | mutation | CoverageReport | `from_tracker()` |
| FR-MUT-010 | mutation | CoverageReport | serde derives |
| FR-SPEC-001 | spec | SpecParser | `parse()` |
| FR-SPEC-002 | spec | SpecParser | `parse()` error handling |
| FR-SPEC-003 | spec | SpecParser | `parse_file()` |
| FR-SPEC-004 | spec | SpecParser | integrated validation |
| FR-SPEC-005–010 | spec | SpecValidator | validation rules |
| FR-SPEC-011 | spec | Priority | default variant |
| FR-SPEC-012 | spec | Status | default variant |
| FR-SPEC-013 | spec | Spec, Feature, Requirement, SpecMetadata | serde derives |
| FR-DOM-001 | domain | XddError | struct fields |
| FR-DOM-002 | domain | XddError | constructor methods |
| FR-DOM-003 | domain | XddError | `with_context()` |
| FR-DOM-004 | domain | XddError | trait impls |
| FR-DOM-005 | domain | ErrorCategory | enum variants |
| FR-DOM-006 | domain | XddError | serde derives |
| FR-DOM-007 | domain | XddResult | type alias |

---

## Test Coverage Map

All tests should reference FR IDs via docstring or marker:

```rust
#[test]
/// Traces to: FR-PROP-001, FR-PROP-012
fn test_uuid_validation_and_strategy() { ... }
```

Key test scenarios by module:
- **property**: Validation edge cases (empty, malformed, valid); strategy generation
- **contract**: Assertion tracking, failure reporting, result construction
- **mutation**: Score calculation, state transitions, coverage tracking
- **spec**: YAML parsing, validation rules, serialization
- **domain**: Error construction, context attachment, serialization
