# Architecture Decision Records — phenotype-evaluation

## ADR-001 — Evaluator Port/Adapter Pattern

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Evaluation logic varies by domain: LLM outputs need semantic scoring; API responses need schema validation; agent traces need behavioral analysis. A single implementation cannot cover all cases.

### Decision
Define a hexagonal `Evaluator` port. Each domain provides its own adapter. The framework orchestrates adapters and aggregates results.

### Consequences
- Adding new evaluation dimensions requires a new adapter, not changes to the orchestrator.
- Adapters are independently testable.

---

## ADR-002 — JSONL as Dataset Format

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Evaluation datasets can be large (thousands of rows). JSONL allows streaming row-by-row without loading the entire dataset into memory.

### Decision
JSONL is the canonical dataset format. Each line is a JSON object with `input`, `expected`, and optional `metadata` fields.

### Consequences
- Datasets are human-readable and git-diffable.
- Streaming evaluation reduces peak memory usage for large datasets.

---

## ADR-003 — Threshold Configuration in YAML

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Thresholds differ per project and evaluation type. Hard-coding them in the framework would require framework changes for each project.

### Decision
Thresholds are defined in a YAML config file passed via `--threshold`. The schema is validated on load.

### Consequences
- Projects own their threshold configs.
- The framework enforces the schema but not the values.
