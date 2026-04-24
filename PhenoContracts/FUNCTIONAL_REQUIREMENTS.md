# Functional Requirements — PhenoContracts

Traces to: PRD.md epics E1–E6.
ID format: FR-PHENOCONTRACTS-{NNN}.

---

## API Contract Specification

**FR-PHENOCONTRACTS-001**: The system SHALL define REST API contracts using OpenAPI 3.1 with mandatory input/output schemas and example requests.
Traces to: E1.1

**FR-PHENOCONTRACTS-002**: The system SHALL validate API requests against contract schemas and return [HttpStatusCode::BAD_REQUEST] for schema violations.
Traces to: E1.2

**FR-PHENOCONTRACTS-003**: The system SHALL generate API client SDKs (Rust, Python, TypeScript) from contract definitions.
Traces to: E1.3

---

## Service Contract Registry

**FR-PHENOCONTRACTS-004**: The system SHALL maintain a registry of service contracts indexed by (service_name, version, operation).
Traces to: E2.1

**FR-PHENOCONTRACTS-005**: The system SHALL detect contract breaking changes and flag incompatibilities when clients upgrade to newer versions.
Traces to: E2.2

---

## Contract Versioning

**FR-PHENOCONTRACTS-006**: The system SHALL support semantic versioning of contracts (MAJOR, MINOR, PATCH) with deprecation warnings.
Traces to: E3.1

---

## Trace & Test Guidance

All tests MUST reference a Functional Requirement (FR):

```rust
// Traces to: FR-PHENOCONTRACTS-NNN
#[test]
fn test_contract_validation() { ... }
```
