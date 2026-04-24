# pheno Extraction Plan — Module-by-Module Analysis

**Date**: 2026-04-24  
**Status**: ANALYSIS COMPLETE — Ready for selective extraction  
**Total pheno LOC**: 215,096  
**Extractable phenotype-* LOC**: ~18,500 (8.6%)  

---

## Executive Summary

The `pheno` repository contains 215K LOC across 64 top-level directories (AgilePlus suite, apps, infra, and shared infrastructure crates). Within this monorepo, **18 phenotype-prefixed infrastructure crates** (18.5K LOC) are candidates for extraction into a **canonical phenotype-shared repository**.

**Key Finding**: 4 crates already have duplicates in `phenoShared/` (overlapping 4.8K LOC). Strategy: keep phenoShared versions as canonical, consolidate unique modules, retire `phenoShared` via DEPRECATION.md.

---

## Part A: Overlap Analysis (phenoShared ↔ pheno)

### Duplicated Crates (Keep pheno version when larger/more complete)

| Crate | phenoShared LOC | pheno LOC | Recommendation |
|-------|---|---|---|
| phenotype-event-sourcing | 1,816 | 642 | **Keep phenoShared** (larger, more complete) |
| phenotype-cache-adapter | 812 | 78 | **Keep phenoShared** (larger, more complete) |
| phenotype-policy-engine | 1,402 | 1,823 | **Keep pheno** (more complete) |
| phenotype-state-machine | 664 | 355 | **Keep phenoShared** (larger, more complete) |
| **Total Overlap** | **4,694** | **2,898** | **Consolidate to phenotype-shared** |

### Unique to phenoShared (8 crates, 7.3K LOC)

These must move into canonical phenotype-shared:

| Crate | LOC | Purpose | Extraction Priority |
|-------|---|---------|---|
| **phenotype-nanovms-client** | 2,284 | NanoVM virtualization bridge | HIGH |
| **phenotype-domain** | 1,697 | Domain types & core entities | HIGH |
| **phenotype-port-interfaces** | 1,257 | Hexagonal port trait definitions | HIGH |
| **phenotype-application** | 1,097 | Application layer base classes | MEDIUM |
| **phenotype-postgres-adapter** | 203 | PostgreSQL port implementation | MEDIUM |
| **phenotype-redis-adapter** | 204 | Redis cache port | MEDIUM |
| **phenotype-http-adapter** | 127 | HTTP server port | MEDIUM |
| **ffi_utils** | 9 | FFI helper utilities | LOW |

---

## Part B: Top 10 Extractable Modules from pheno

These infrastructure crates are domain-agnostic and suitable for reuse across Phenotype projects.

| Rank | Crate | LOC | Purpose | Target | Effort | Notes |
|------|-------|-----|---------|--------|--------|-------|
| **1** | phenotype-policy-engine | 1,823 | Rule-based policy evaluation + TOML config | phenotype-shared | **HIGH** | Near-duplicate of phenoShared version; consolidate to pheno variant |
| **2** | phenotype-retry | 1,656 | Exponential backoff + circuit breaker pattern | phenotype-shared | **HIGH** | Pure utility, zero deps on agileplus |
| **3** | phenotype-testing | 1,063 | Test fixtures, mocking, BDD utilities | phenotype-testing (split) | **MEDIUM** | Split into `-fixtures` (shared) and `-agileplus-fixtures` (project-specific) |
| **4** | phenotype-port-traits | 1,004 | Hexagonal port trait library | phenotype-shared | **MEDIUM** | Dependency for all port implementations |
| **5** | phenotype-infrastructure | 814 | Core infra abstractions (telemetry, tracing) | phenotype-shared | **MEDIUM** | Foundation for observability across projects |
| **6** | phenotype-health | 788 | HealthChecker abstraction + implementations | phenotype-shared | **MEDIUM** | Already in canonical phenotype-infrakit; verify consolidation |
| **7** | phenotype-mock | 754 | Mockito-style mocking for tests | phenotype-testing | **MEDIUM** | Likely internal to agileplus; audit reuse |
| **8** | phenotype-cost-core | 740 | Cost tracking and billing abstractions | phenotype-shared | **LOW** | Interesting but niche; evaluate demand first |
| **9** | phenotype-bdd | 679 | Behavior-driven development scaffolding | phenotype-testing | **LOW** | Heavy agileplus coupling; extract only if demand proven |
| **10** | phenotype-event-bus | 393 | Publish-subscribe event bus | phenotype-shared | **MEDIUM** | Already duplicated; consolidate variants |

---

## Part C: Categorized Extraction Roadmap

### Tier 1: Critical Path (Extract First)
**Effort**: ~4-6h | **Impact**: Enables all downstream work

1. **phenotype-retry** (1,656 LOC)
   - Location: `/pheno/crates/phenotype-retry/`
   - Destination: `phenotype-shared/crates/`
   - Status: Ready (zero agileplus coupling, pure utility)
   - Test Coverage: ✓ Inline tests present

2. **phenotype-port-traits** (1,004 LOC)
   - Location: `/pheno/crates/phenotype-port-traits/`
   - Destination: `phenotype-shared/crates/`
   - Status: Ready (trait definitions only, zero implementations)
   - Dependency: Needed by all port implementations

3. **phenotype-policy-engine** (1,823 pheno vs 1,402 phenoShared)
   - Consolidate pheno variant (more complete) + phenoShared variant
   - Destination: `phenotype-shared/crates/phenotype-policy-engine/`
   - Status: Requires merge/dedup of TOML config patterns
   - Test Coverage: ✓ Both have tests

### Tier 2: High-Value (Extract After Tier 1)
**Effort**: ~6-8h | **Impact**: Observability, testing, and reusable infra

4. **phenotype-infrastructure** (814 LOC)
   - Telemetry, tracing, and infra abstractions
   - Destination: `phenotype-shared/crates/`

5. **phenotype-health** (788 LOC)
   - HealthChecker trait (already in phenotype-infrakit)
   - Action: Verify consolidation with canonical phenotype-infrakit

6. **phenotype-testing** (1,063 LOC, then split)
   - Extract reusable fixtures → `phenotype-shared/crates/phenotype-test-fixtures/`
   - Keep agileplus-specific → `pheno/crates/agileplus-testing/`

### Tier 3: niche/Conditional (Extract Only on Demand)
**Effort**: ~2-4h each | **Impact**: Specialized use cases

7. **phenotype-cost-core** (740 LOC)
   - Billing abstractions; extract only if Tracery/Metron need it
   - Status: **DEFER** — wait for downstream demand signal

8. **phenotype-bdd** (679 LOC)
   - BDD scaffolding; heavy agileplus coupling
   - Status: **DEFER** — audit callers before extraction

### Tier 4: phenoShared Unique Modules (Immediate Transfer)
**Effort**: ~2-3h | **Impact**: Consolidates shared infra

- **phenotype-nanovms-client** (2,284 LOC) → `phenotype-shared/crates/`
- **phenotype-domain** (1,697 LOC) → `phenotype-shared/crates/` (as `phenotype-domain-core`)
- **phenotype-port-interfaces** (1,257 LOC) → `phenotype-shared/crates/`
- **phenotype-application** (1,097 LOC) → `phenotype-shared/crates/` (as `phenotype-application-core`)
- **phenotype-postgres-adapter** (203 LOC) → `phenotype-shared/crates/`
- **phenotype-redis-adapter** (204 LOC) → `phenotype-shared/crates/`
- **phenotype-http-adapter** (127 LOC) → `phenotype-shared/crates/`

---

## Part D: Dependency Audit (Critical Before Extraction)

### High-Risk Extraction (Audit Coupling First)

| Crate | Risk | Blocked By | Status |
|-------|------|-----------|--------|
| phenotype-testing | MEDIUM | agileplus-domain, agileplus-sqlite | Requires fixture split |
| phenotype-bdd | HIGH | agileplus-cli, agileplus-plane | Requires decoupling pass |
| phenotype-cost-core | LOW | agileplus-domain | Check if truly domain-agnostic |

### Safe for Immediate Extraction

- phenotype-retry: Zero agileplus deps ✓
- phenotype-port-traits: Traits only ✓
- phenotype-policy-engine: Declarative config only ✓
- phenotype-infrastructure: Abstraction layer ✓

---

## Part E: Consolidation Strategy

### Step 1: Create phenotype-shared Repository
```bash
cd /repos
# Clone canonical phenotype-shared from existing phenoShared
cp -r phenoShared phenotype-shared
cd phenotype-shared
git init
git add .
git commit -m "chore(init): canonical phenotype-shared from phenoShared"
git remote add origin https://github.com/KooshaPari/phenotype-shared.git
```

### Step 2: Merge Overlapping Crates (Tier 1+2)
```
For each duplicate:
1. Compare phenoShared vs pheno versions
2. Keep larger/more-complete variant
3. Merge test coverage from both
4. Update Cargo.toml members in phenotype-shared
5. Remove pheno duplicate from pheno/Cargo.toml
```

### Step 3: Migrate Unique Modules (Tier 4)
```
For each unique phenoShared crate:
1. Already present in phenotype-shared ✓
2. Tag commit: chore(phenotype-shared): add [crate-name]
3. Update docs/migrations/pheno_extraction_plan.md with status
```

### Step 4: Retire phenoShared
```
In phenoShared/:
1. Create DEPRECATION.md → points to phenotype-shared
2. Commit: chore(deprecation): merged into phenotype-shared
3. Archive directory: mv phenoShared .archive/phenoShared-retired-2026-04-24

In docs/migrations/:
1. Create pheno_extraction_plan_COMPLETED.md (this doc)
2. Document cutover path for projects still using phenoShared
```

---

## Part F: Estimated Effort & Timeline

### Phase 1: Tier 1 Extraction (Critical Path)
- **Duration**: 4-6 hours (agent-driven)
- **Deliverables**:
  - phenotype-shared repo created
  - phenotype-retry, phenotype-port-traits, phenotype-policy-engine migrated
  - Pheno Cargo.toml updated (remove duplicates, add phenotype-shared dependency)
  - All tests passing
  - PR: `feat(consolidation): phenotype-shared tier-1 extraction`

### Phase 2: Tier 2 Extraction (Observability & Testing)
- **Duration**: 6-8 hours (agent-driven)
- **Deliverables**:
  - phenotype-infrastructure, phenotype-health, phenotype-testing extracted
  - Fixture split complete
  - All pheno tests updated to use phenotype-shared deps
  - PR: `feat(consolidation): phenotype-shared tier-2 extraction`

### Phase 3: Tier 4 Transfer (phenoShared Unique)
- **Duration**: 2-3 hours (agent-driven)
- **Deliverables**:
  - All phenoShared unique crates merged into phenotype-shared
  - phenoShared marked deprecated
  - Cutover guide for projects still using phenoShared
  - PR: `docs(migrations): pheno_extraction_plan completed + phenoShared retirement`

### Total Effort
- **Wall-clock**: ~16-20 hours (distributed across 3 phases)
- **Agent parallelization**: Could compress to 6-8 hours with 2-3 parallel extraction agents

---

## Part G: Success Criteria

- [ ] phenotype-shared repo created and published to GitHub
- [ ] All overlapping crates consolidated (keep better variant)
- [ ] All phenoShared unique crates migrated to phenotype-shared
- [ ] pheno/Cargo.toml updated (depends on phenotype-shared, no duplicate crates)
- [ ] All tests passing in both pheno and phenotype-shared
- [ ] DEPRECATION.md in phenoShared with migration path
- [ ] This plan marked COMPLETED in footer
- [ ] All downstream projects audited for phenoShared → phenotype-shared migration

---

## Part H: Dependency Graph (Quick Reference)

```
phenotype-shared (canonical)
├── phenotype-port-traits (traits only)
├── phenotype-retry (utility)
├── phenotype-policy-engine (rules + config)
├── phenotype-infrastructure (observability)
├── phenotype-health (health checks)
├── phenotype-event-sourcing (event log)
├── phenotype-cache-adapter (caching)
├── phenotype-state-machine (FSM)
├── phenotype-nanovms-client (vm bridge)
├── phenotype-domain (core types)
├── phenotype-application (app layer)
├── phenotype-*-adapter (port implementations)
└── phenotype-test-fixtures (reusable fixtures)

pheno (consumer)
├── uses: phenotype-shared::* (post-consolidation)
├── agileplus (project suite)
├── apps/* (user-facing)
├── bifrost/* (services)
├── infra/* (deployment)
└── (removes internal duplicates)
```

---

## Phase 2 Execution Notes (2026-04-24)

**DISCOVERY**: The Phase 2 crates (phenotype-testing, phenotype-mock, phenotype-cost-core, phenotype-bdd, phenotype-event-bus) are **modules within the pheno monorepo**, not standalone Cargo crates.

### Status by Crate
- phenotype-testing: No Cargo.toml; modules depend on monorepo dependencies
- phenotype-mock: No Cargo.toml; heavy internal coupling
- phenotype-cost-core: No Cargo.toml; requires pheno workspace resolution
- phenotype-bdd: No Cargo.toml; depends on cache-adapter, config-core, compliance-scanner
- phenotype-event-bus: No Cargo.toml; depends on health, http-client-core, infrastructure

### Extraction Strategy
**Phase 2 DEFERRED** — requires:
1. Convert each module to standalone Cargo crate (add Cargo.toml)
2. Break internal monorepo dependencies (phenotype-bdd, phenotype-event-bus especially)
3. Create lightweight test shims for pheno consumers
4. Validate no regressions in agileplus or bifrost services

**Estimated Effort**: +4-6h (monorepo module extraction is harder than standalone crate copy)

**Recommended Alternative for User**:
- Execute Phase 1 first (phenotype-retry, phenotype-port-traits, phenotype-policy-engine) — all have Cargo.toml already
- Then plan Phase 2 with explicit decoupling sprints for each module

---

## Footer

**Status**: ⏸️ ANALYSIS COMPLETE, PHASE 2 BLOCKED ON MODULE STRUCTURE  
**Next Action**: Phase 1 execution OR explicit Phase 2 scope reduction  
**Execution Owner**: User decision required  
**Completion Target**: 2026-04-25 Phase 1 only, or 2026-04-26+ Phase 2 with refactoring

---

*This plan was auto-generated during cross-repo analysis session 2026-04-24. Phase 2 discovery logged 2026-04-24.*
