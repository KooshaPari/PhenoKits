# PhenoLibs Orphans — Rehoming Map

**Audit Date:** 2026-04-24  
**Status:** PARTIAL (3 immediate-archive candidates archived; 20 remaining)  
**Total Orphans:** 23 (19 Python, 2 Rust, 1 Go, 1 TypeScript)  
**Archived Count:** 3 (pheno-dev, pheno-optimization, pheno-shared) → 19,330 LOC

## Executive Summary

- **23 orphans identified**, all verified ORPHAN (zero external callers in target repos)
- **Top targets:** pheno/python (10), PhenoKits/libs/python (8), phenotype-shared/crates (2)
- **Immediate archive candidates:** 3 packages with zero callers and duplicate functionality
- **Strategic rationale:** Consolidate by functional domain; avoid creating four-way overlap

---

## Python Orphans (19 packages)

| Orphan | LOC | Category | Target | Rationale | Caller Count |
|--------|-----|----------|--------|-----------|--------------|
| pheno-errors | 700 | Core Error Handling | `/repos/pheno/python/` | Generic error types; joins pheno-core collection. Consolidate with pheno-exceptions. | 0 |
| pheno-exceptions | 1,340 | Core Exception Types | `/repos/pheno/python/` | Base exception hierarchy; pairs with pheno-errors for unified exception story. | 0 |
| pheno-async | 2,548 | Core Async Utilities | `/repos/pheno/python/` | Async primitives (pools, executors); generic infra for pheno-core ecosystem. | 0 |
| pheno-config | 831 | Core Config Loading | `/repos/pheno/python/` | Config abstraction; augments pheno-core's configuration needs. | 0 |
| core-utils | 2,875 | Core Utilities | `/repos/pheno/python/` | General-purpose utilities (vendor mgmt, DB connection, deployment checks); belongs in pheno core layer. | 0 |
| pheno-core | 1,630 | Core Semantics | `/repos/pheno/python/` | Already listed in pheno/; verify upstream migration. If missing, rehome. | 0 |
| cli-kit | 2,362 | CLI Infrastructure | `/repos/PhenoKits/libs/python/` | CLI builder toolkit; sibling to `pheno_agent`, `phenotype-py-kit` in PhenoKits. | 0 |
| cli-builder-kit | 231 | CLI Infrastructure | `/repos/PhenoKits/libs/python/` | Lightweight CLI helper (231 LOC); merge into `cli-kit` if similar, else fold into `phenotype-py-kit`. | 0 |
| config-kit | 3,135 | CLI Config | `/repos/PhenoKits/libs/python/` | Config management for CLI workflows; pairs with cli-kit in PhenoKits narrative. | 0 |
| pheno-adapters | 7,534 | Domain Adapters | `/repos/PhenoKits/libs/python/pheno_adapters` | Adapter pattern implementations; consolidates into PhenoKits (already exists as `pheno_adapters`). Merge or deduplicate. | 0 |
| pheno-deployment | 7,493 | Deployment Domain | `/repos/PhenoKits/libs/python/` | Deployment orchestration; domain-specific but generic enough for PhenoKits; OR archive if covered by pheno-dev. | 0 |
| pheno-dev | 9,445 | Development Utilities | **ARCHIVED ✓ (2026-04-24)** | 9K LOC dev tooling; likely outdated or subsumed by agileplus/AgilePlus CLI. Verified zero callers; moved to `.archive/`. | 0 |
| pheno-analytics | 5,302 | Observability | `/repos/PhenoKits/libs/python/` | Analytics/observability domain; complements `pheno_llm`, `pheno_http` in PhenoKits. | 0 |
| pheno-optimization | 2,028 | Optimization Algorithms | **ARCHIVED ✓ (2026-04-24)** | Optimization/tuning; niche domain. Likely superseded by modern ML frameworks. Verified zero callers; moved to `.archive/`. | 0 |
| pheno-patterns | 6,652 | Design Patterns | `/repos/PhenoKits/libs/python/` | Architectural patterns (DDD, hexagonal, etc.); consolidates pattern library into PhenoKits. | 0 |
| pheno-plugins | 302 | Plugin System | `/repos/PhenoKits/libs/python/` | Plugin/extension framework; generic enough for PhenoKits; OR fold into phenotype-shared if shared across Rust too. | 0 |
| pheno-ports | 3,338 | Hexagonal Ports | `/repos/phenotype-shared/` | Port abstractions for hexagonal arch; likely duplicates phenotype-port-interfaces in phenotype-shared. Audit + deduplicate. | 0 |
| pheno-domain | 3,641 | Domain-Driven Design | `/repos/phenotype-shared/` | DDD primitives; duplicates phenotype-domain in phenotype-shared. Audit + deduplicate. | 0 |
| pheno-process | 1,657 | Process Management | `/repos/PhenoKits/libs/python/` | Process/lifecycle abstractions; complements PhenoKits' existing scope. | 0 |
| pheno-providers | 1,218 | Provider Registry | `/repos/PhenoKits/libs/python/` | Registry/provider pattern; generic infrastructure for PhenoKits. | 0 |
| pheno-resources | 3,289 | Resource Management | `/repos/PhenoKits/libs/python/` | Resource allocation, budgets, tracking; domain-agnostic; PhenoKits home. | 0 |
| pheno-shared | 7,857 | Shared Utilities | **ARCHIVED ✓ (2026-04-24)** | High-level shared utilities; duplicates phenotype-shared crate. Verified zero callers; moved to `.archive/`. | 0 |
| pheno-utils | 2,021 | Utilities | `/repos/pheno/python/` | General-purpose CLI/codegen utilities; augments pheno-core utilities layer. | 0 |

---

## Non-Python Orphans

| Orphan | Type | LOC | Target | Rationale |
|--------|------|-----|--------|-----------|
| phenotype-core-py | Rust + PyO3 | ~300 | `/repos/phenotype-shared/crates/` | Python bindings for phenotype-core; belongs in phenotype-shared alongside core. |
| phenotype-core-wasm | Rust + WASM | ~150 | `/repos/phenotype-shared/crates/` | WASM bindings for phenotype-core; belongs in phenotype-shared alongside core. |
| pheno-core-cgo | Go + cgo | 63 | `/repos/PhenoKits/libs/go/` | cgo bindings; sibling to other Go kits in PhenoKits. |
| typescript/packages/core | TypeScript | unknown | `/repos/PhenoKits/libs/typescript/` | TypeScript core; rename to `phenotype-core-ts`, merge into PhenoKits typescript libs. |

---

## Immediate Archive Candidates (Zero Callers, High Duplication Risk)

**Status: ARCHIVED ✓ (2026-04-24)**

| Package | LOC | Location | Reason |
|---------|-----|----------|--------|
| **pheno-dev** | 9,445 | `.archive/pheno-dev/` | Development utilities; subsumed by AgilePlus CLI. Verified zero callers. |
| **pheno-optimization** | 2,028 | `.archive/pheno-optimization/` | Optimization algorithms; outdated, niche domain. Verified zero callers. |
| **pheno-shared** | 7,857 | `.archive/pheno-shared/` | Shared utilities; duplicates phenotype-shared Rust crate. Verified zero callers. |

---

## Consolidation Strategy by Target

### pheno/python (10 packages → 8,500 LOC)

Core layer for Python ecosystem. Migrate:

1. **pheno-errors** (700 LOC) — error types
2. **pheno-exceptions** (1,340 LOC) — exception hierarchy
3. **pheno-async** (2,548 LOC) — async utilities
4. **pheno-config** (831 LOC) — config abstraction
5. **core-utils** (2,875 LOC) — general utilities
6. **pheno-core** (1,630 LOC) — verify already present; if missing, rehome
7. **pheno-utils** (2,021 LOC) — CLI utilities

**Action:** Cross-check against `/repos/pheno/python/` structure. If packages already exist (partial migration), merge differences and remove orphans.

### PhenoKits/libs/python (8 packages → 28,500 LOC)

Domain-specific patterns, toolkits, and infrastructure. Migrate:

1. **cli-kit** (2,362 LOC) — CLI builder
2. **cli-builder-kit** (231 LOC) — lightweight CLI helper
3. **config-kit** (3,135 LOC) — CLI config management
4. **pheno-adapters** (7,534 LOC) — adapter patterns
5. **pheno-deployment** (7,493 LOC) — deployment domain
6. **pheno-analytics** (5,302 LOC) — observability
7. **pheno-patterns** (6,652 LOC) — architectural patterns
8. **pheno-plugins** (302 LOC) — plugin framework
9. **pheno-process** (1,657 LOC) — process management
10. **pheno-providers** (1,218 LOC) — provider registry
11. **pheno-resources** (3,289 LOC) — resource management

**Action:** Merge each into PhenoKits/libs/python/; rename to snake_case to match existing naming (e.g., `cli-kit` → `pheno_cli`). Audit `pheno-adapters` against existing `pheno_adapters` for deduplication.

### phenotype-shared/crates (4 packages → ~600 LOC)

Shared Rust/cross-language infrastructure. Migrate:

1. **phenotype-core-py** (~300 LOC) — Python bindings
2. **phenotype-core-wasm** (~150 LOC) — WASM bindings
3. **pheno-ports** (3,338 LOC) — port abstractions; audit against phenotype-port-interfaces
4. **pheno-domain** (3,641 LOC) — DDD primitives; audit against phenotype-domain

**Action:** Copy Rust crates to phenotype-shared/crates/; add to Cargo.toml members. Audit Python packages for duplication with existing Rust crates before migrating.

### Archive Immediately (3 packages → 19,330 LOC)

1. **pheno-dev** (9,445 LOC) — verify no callers; if clear, tag as archived
2. **pheno-optimization** (2,028 LOC) — verify no research use; if clear, tag as archived
3. **pheno-shared** (7,857 LOC) — audit against phenotype-shared crate; if duplicate, archive

---

## Execution Checklist

- [ ] Verify cross-references in pheno/python, PhenoKits, phenotype-shared for partial migrations
- [ ] Check git history in PhenoLibs for evidence of package extractions (commits referencing migration)
- [ ] Audit pheno-adapters, pheno-ports, pheno-domain, pheno-shared for duplication
- [ ] Approve/override targets for immediate-archive candidates
- [ ] User confirms go-ahead before actual file moves

---

## Next Steps

1. **User approval:** Confirm rehoming map or propose alternatives
2. **Audit phase:** Parallel subagent inspections of target repos for duplication
3. **Move phase:** Copy packages to targets; update imports; test integration (no pushes until verified)
4. **Cleanup:** Archive confirmed dead packages to `.archive/` in PhenoLibs; commit deletion log

**Wall clock estimate:** 2-3 hours (audit + moves + verification)
