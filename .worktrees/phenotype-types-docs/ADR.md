# Architecture Decision Records — phenotype-types

---

## ADR-001 — Pydantic v2 as Validation Backbone

**Date:** 2025-08-01
**Status:** Accepted

### Context
Phenotype Python packages need runtime validation, JSON serialisation, and JSON Schema
generation from a single type definition. Options: dataclasses (no validation), attrs
(verbose), Pydantic v1 (outdated), Pydantic v2 (Rust core, 10x faster).

### Decision
Use Pydantic v2 for all shared types. TypedDict is retained for simple read-only shapes
that do not need validation.

### Consequences
- All Phenotype Python packages pin `pydantic>=2.0` as a dependency.
- Pydantic v2's Rust core provides negligible import overhead.
- JSON Schema generation is built-in via `model.model_json_schema()`.

---

## ADR-002 — Separate Module Per Domain

**Date:** 2025-08-10
**Status:** Accepted

### Context
A single `types.py` file risks becoming a 1000-line monolith. Domain separation makes
imports explicit and avoids circular imports.

### Decision
Organise types into domain modules: `phenotype_types/task.py`, `phenotype_types/research.py`,
`phenotype_types/skill.py`, etc. Top-level `__init__.py` re-exports all public types.

### Consequences
- Consumers can import from top-level (`from phenotype_types import Task`) or module-level
  (`from phenotype_types.task import Task`).
- Adding new domains requires only a new module file.

---

## ADR-003 — NewType for Semantic IDs

**Date:** 2025-08-15
**Status:** Accepted

### Context
String-typed IDs (`task_id: str`) allow accidental mixing of different ID types at type-check
time. `NewType` wrapping creates distinct types at zero runtime cost.

### Decision
All ID types use `NewType`: `TaskID = NewType("TaskID", UUID)`,
`SkillID = NewType("SkillID", str)`, etc.

### Consequences
- Mypy/pyright catch `task_func(skill_id)` mismatches at static analysis time.
- No runtime overhead; `NewType` is erased at runtime.
