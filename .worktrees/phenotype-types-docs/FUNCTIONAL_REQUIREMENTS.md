# Functional Requirements — phenotype-types

**Version:** 1.0.0
**Traces to:** PRD epics E1–E2

---

## FR-TYPES-001 — Task Type Definitions
**SHALL** define `Task`, `TaskID` (NewType wrapping UUID), `TaskState` (Enum), and
`TaskResult` (TypedDict or Pydantic model) in the `phenotype_types.task` module.
**Traces to:** E1.1

## FR-TYPES-002 — Task State Enum
**SHALL** define `TaskState` as an enum with values: `PENDING`, `RUNNING`, `SUCCEEDED`,
`FAILED`, `TIMED_OUT`.
**Traces to:** E1.1

## FR-TYPES-003 — Research Type Definitions
**SHALL** define `Evidence`, `ResearchReport`, `Citation`, and `ConfidenceScore` (float in
[0.0, 1.0]) in the `phenotype_types.research` module with Pydantic v2 validators.
**Traces to:** E1.2

## FR-TYPES-004 — Confidence Score Range Validation
**SHALL** reject `ConfidenceScore` values outside [0.0, 1.0] with a descriptive
`ValidationError`.
**Traces to:** E1.2

## FR-TYPES-005 — Skill Type Definitions
**SHALL** define `Skill`, `SkillManifest`, `SkillInput`, and `SkillOutput` in the
`phenotype_types.skill` module.
**Traces to:** E1.3

## FR-TYPES-006 — JSON Schema Export
**SHALL** implement `phenotype_types.schemas.export(type_name: str) -> dict` that returns
the JSON Schema for the named type, and `export_all() -> dict[str, dict]` for all types.
**Traces to:** E2.1

## FR-TYPES-007 — Strict Pydantic Config
**SHALL** configure all Pydantic models with `model_config = ConfigDict(strict=True)` to
prevent implicit type coercion.
**Traces to:** E2.2

## FR-TYPES-008 — Zero Runtime Dependencies (Beyond Pydantic)
**SHALL** depend only on `pydantic>=2.0` and the Python standard library; no other
third-party packages are permitted in `phenotype-types`.
**Traces to:** E1.1, E1.2, E1.3
