# PRD — phenotype-types

**Version:** 1.0.0
**Stack:** Python 3.11+ (TypedDict, dataclasses, Pydantic v2)
**Repo:** `KooshaPari/phenotype-types`

---

## Overview

`phenotype-types` is the shared type definitions library for all Python-based Phenotype
packages. It provides TypedDict classes, dataclasses, Pydantic v2 models, and type aliases
that enforce a common data contract across the research engine, task engine, docs engine,
ops tooling, and agent skill packages.

---

## Epics

### E1 — Core Domain Types
**Goal:** Define the canonical data shapes for all Phenotype domain concepts.

#### E1.1 Task Domain Types
- As a task engine author, I want `Task`, `TaskID`, `TaskState`, and `TaskResult` types
  imported from a shared package so all engines agree on the same schema.
- **Acceptance:** `from phenotype_types import Task, TaskState` works in any Phenotype Python
  package with `phenotype-types` installed.

#### E1.2 Research Domain Types
- **Acceptance:** `Evidence`, `ResearchReport`, `Citation`, `ConfidenceScore` types are
  importable and carry Pydantic validators.

#### E1.3 Agent Skill Types
- **Acceptance:** `Skill`, `SkillManifest`, `SkillInput`, `SkillOutput` types are defined
  and exportable.

### E2 — Schema Validation
**Goal:** All types are validated at runtime via Pydantic v2.

#### E2.1 JSON Schema Export
- **Acceptance:** `phenotype_types.schemas.export()` produces JSON Schema documents for
  all registered types, usable for contract testing.

#### E2.2 Strict Mode
- **Acceptance:** All Pydantic models use `model_config = ConfigDict(strict=True)` so
  coercive type casting (str → int) is rejected.
