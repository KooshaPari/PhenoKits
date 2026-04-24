# HexaKit Subprojects Audit — 2026-04

**Status**: Phase 1 COMPLETE  
**Date**: April 24, 2026  
**Related**: `/docs/migrations/hexakit_products_promotion.md`

## Executive Summary

HexaKit audit identified 6 top-tier products and 27 libraries. **Phase 1 cleanup completed**:
- Agileplus duplicate archived (canonical: `/repos/AgilePlus/`)
- 10 empty/stub projects archived (0-1 LOC each)
- Duplicate phenotype-config-core deduplicated (kept libs/ version)

**LOC Impact**: ~880 LOC removed from active workspace

---

## Phase 1 Completion Details

### Archived Items

| Item | Type | LOC | Location | Rationale |
|------|------|-----|----------|-----------|
| agileplus | PRODUCT | 8,108 | `.archive/agileplus-hexakit-duplicate/` | Stale copy; canonical at `/repos/AgilePlus/` (63K LOC, 24 crates) |
| phenotype-observability | STUB | 0 | `.archive/hexakit-empty-stubs/` | WIP; no production code |
| phenotype-guard | STUB | 0 | `.archive/hexakit-empty-stubs/` | Unused abstraction |
| pheno-guard | STUB | 0 | `.archive/hexakit-empty-stubs/` | Duplicate |
| phenotype-crypto | STUB | 1 | `.archive/hexakit-empty-stubs/` | Placeholder only |
| phenotype-git-core | STUB | 1 | `.archive/hexakit-empty-stubs/` | Placeholder only |
| phenotype-http-client-core | STUB | 1 | `.archive/hexakit-empty-stubs/` | Placeholder only |
| phenotype-process | STUB | 1 | `.archive/hexakit-empty-stubs/` | Placeholder only |
| phenotype-mcp | STUB | 1 | `.archive/hexakit-empty-stubs/` | Placeholder only |
| agileplus-dashboard | STUB | 0 | `.archive/hexakit-empty-stubs/` | Framework scaffolding |
| agileplus-dashboard-server | STUB | 0 | `.archive/hexakit-empty-stubs/` | Framework scaffolding |
| phenotype-config-core (crates/) | DUPLICATE | 267 | `.archive/hexakit-empty-stubs/phenotype-config-core-duplicate/` | Redundant; newer version moved to libs/ |

**Total Archived**: 12 items, ~880 LOC removed

### Kept in HexaKit (27 libraries + 6 products)

#### Tier 1 Core Libraries (9 crates, 1K–2.1K LOC each)
- phenotype-testing (2,126 LOC) — test infrastructure
- phenotype-infrastructure (1,648 LOC) — abstractions
- phenotype-health (1,576 LOC) — health checks
- phenotype-mock (1,516 LOC) — mock implementations
- phenotype-cost-core (1,480 LOC) — cost tracking
- phenotype-bdd (1,358 LOC) — BDD framework
- phenotype-event-sourcing (1,284 LOC) — event store with hash chains
- phenotype-iter (1,094 LOC) — iterator utilities
- phenotype-project-registry (1,088 LOC) — project metadata

#### Tier 2 Utility Libraries (18 crates, <1K LOC each)
- phenotype-rate-limiter (~400 LOC) — rate limiting
- phenotype-validation (~350 LOC) — input validation
- phenotype-time (~300 LOC) — time abstractions
- phenotype-async-traits, phenotype-error-core, phenotype-contract, phenotype-cache-adapter, phenotype-ports-canonical, etc.

#### Tier 1 Products (6 items)
- **python workspace** (10,163 LOC) — shared test/utility layer
- **phenotype-policy-engine** (4,456 LOC) — policy evaluation
- **phenotype-retry** (3,312 LOC) — retry strategies
- **phenotype-xdd-lib** (2,988 LOC) — XDD patterns (promote to phenotype-tooling in Phase 2)
- **clikit** (1,010 LOC) — CLI framework (promote to phenotype-tooling in Phase 2)

---

## Architecture Impact

**Workspace Clarity**: Removed 880 LOC of unused stubs and duplicates, reducing cognitive load and clarifying Tier 1 vs. Tier 2 libraries.

**Dependencies**: No broken dependencies from archival:
- phenotype-config-core: libs/ version retains all functionality; crates/ was incomplete duplicate
- agileplus: canonical at AgilePlus/ repo, not imported into HexaKit
- Stubs: 0 dependencies in active workspace

---

## Phase 2 Planned Actions

1. **Extract phenotype-xdd-lib** → `phenotype-tooling/crates/`
2. **Extract clikit** → `phenotype-tooling/crates/`
3. **Verify** tests pass in phenotype-tooling
4. **Update** HexaKit to reference phenotype-tooling (git)

**Timeline**: Deferred (pending phenotype-tooling stabilization)

---

## Migration Paths

### For code depending on archived projects:
- **agileplus**: Use canonical `/repos/AgilePlus/` instead
- **observability/mcp**: See phenotype-telemetry or agileplus-mcp
- **crypto/process**: Implement per-use case or use phenotype-cipher
- **config-core**: Use `libs/phenotype-config-core/` (all functionality retained)

---

## References

- Migration decisions: `/docs/migrations/hexakit_products_promotion.md`
- HexaKit structure: `HexaKit/README.md` (to be updated with Tier 1/2 classification)
- Archive contents: `HexaKit/.archive/` (physical location; not git-tracked per policy)
