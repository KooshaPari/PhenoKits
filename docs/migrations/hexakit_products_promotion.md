# HexaKit Products Promotion — Migration Map (2026-04)

**Status**: Decision document  
**Scope**: 6 PRODUCTS identified in HexaKit org-audit; 27 LIBRARIES evaluated  
**Action**: Extract high-value products; archive duplicates; consolidate config  

---

## Executive Summary

HexaKit audit identified **6 top-tier products** (3K+ LOC). Current status:

| Product | LOC | Current Location | Decision | Rationale |
|---------|-----|------------------|----------|-----------|
| **agileplus** | 8,108 | HexaKit/ + AgilePlus/ (canonical) | Archive duplicate in HexaKit | Canonical repo is AgilePlus/ (main org project) |
| **python workspace** | 10,163 | HexaKit/python/ | **Keep in HexaKit** | Shared test/pheno utilities; no standalone need yet |
| **phenotype-policy-engine** | 4,456 | HexaKit/crates/ | **Keep in HexaKit** | Widely used; extraction to phenotype-infrakit deferred |
| **phenotype-retry** | 3,312 | HexaKit/crates/ | **Keep in HexaKit** | Foundational; extraction blocked (no phenotype-infrakit) |
| **phenotype-xdd-lib** | 2,988 | HexaKit/crates/ | **Promote to phenotype-tooling** | XDD patterns fit CLI/toolkit ecosystem |
| **clikit** | 1,010 | HexaKit/clikit | **Promote to phenotype-tooling** | Universal CLI framework; consolidates with tooling ecosystem |

---

## Product-by-Product Analysis

### 1. agileplus (8,108 LOC) — ARCHIVE DUPLICATE

**Findings:**
- HexaKit contains copy under `HexaKit/agileplus/agileplus/` (~8K LOC)
- Canonical repo `AgilePlus/` is the authoritative source (~63K LOC across 24 crates)
- HexaKit version is stale/partial scaffold

**Decision: REMOVE from HexaKit**
- Move `HexaKit/agileplus/` → `.archive/agileplus-hexakit-duplicate/`
- Update `docs/migrations/` with reason: "Canonical location is AgilePlus/ repo"
- Commit: `chore(hexakit): archive agileplus duplicate; canonical in AgilePlus/`

---

### 2. python workspace (10,163 LOC) — KEEP IN HEXAKIT

**Findings:**
- Shared Python utilities: pheno-atoms, pheno-core, agents, MCP integrations
- Supports HexaKit Rust crates (tests, contracts, integration tests)
- No standalone consumption signals; tightly coupled to HexaKit Rust workspace

**Decision: KEEP**
- Location: `HexaKit/python/`
- Rationale: Workspace-scoped test/utility infrastructure; extraction would fragment dependencies
- Action: Document in HexaKit README as "Shared Python test & utility layer"
- No extraction planned; mark as "internal workspace support"

---

### 3. phenotype-policy-engine (4,456 LOC) — KEEP IN HEXAKIT

**Findings:**
- Rule-based policy evaluation engine; production-ready
- TOML config; widely adopted in Phenotype services
- Audit recommends extraction to phenotype-infrakit if used in 3+ projects
- **Status**: phenotype-infrakit does NOT currently exist in repos/
- Known usage: AgilePlus, agentapi-plusplus, thegent policy decisions

**Decision: KEEP (short-term)**
- Location: `HexaKit/crates/phenotype-policy-engine/`
- Rationale: phenotype-infrakit not yet materialized; premature extraction adds friction
- Future action: Extract to phenotype-infrakit when foundational Rust workspace is established
- Add TODO to `docs/migrations/`: "Extract phenotype-policy-engine to phenotype-infrakit (when ready)"

---

### 4. phenotype-retry (3,312 LOC) — KEEP IN HEXAKIT

**Findings:**
- Exponential backoff + retry strategies for async operations
- Foundational; used across multiple Phenotype services
- No standalone consumption signals; tightly coupled to Rust workspace

**Decision: KEEP (short-term)**
- Location: `HexaKit/crates/phenotype-retry/`
- Rationale: extraction to phenotype-infrakit deferred (same reason as policy-engine)
- Future action: Same migration timeline as phenotype-policy-engine

---

### 5. phenotype-xdd-lib (2,988 LOC) — PROMOTE TO PHENOTYPE-TOOLING

**Findings:**
- XDD (eXtreme Driven Design) patterns and utilities
- Foundational architecture library; unlikely to exceed 5K LOC
- Fits phenotype-tooling ecosystem (CLI, architectural patterns, meta-frameworks)
- No blocking dependencies; clean Rust crate with tests

**Decision: EXTRACT**
- **Source**: `HexaKit/crates/phenotype-xdd-lib/` → **Target**: `phenotype-tooling/crates/phenotype-xdd-lib/`
- **Steps**:
  1. Copy crate to phenotype-tooling/crates/
  2. Update Cargo.toml in phenotype-tooling root to include
  3. Run tests: `cd phenotype-tooling && cargo test --package phenotype-xdd-lib`
  4. Update HexaKit Cargo.toml to depend on phenotype-tooling (external path or git)
  5. Deprecate HexaKit version with TODO
- **Commit**: `chore(phenotype-tooling): extract phenotype-xdd-lib from HexaKit`

---

### 6. clikit (1,010 LOC) — PROMOTE TO PHENOTYPE-TOOLING

**Findings:**
- Universal CLI framework with hexagonal architecture
- Well-documented; foundational for CLI tooling ecosystem
- Active; no blockers for independent consumption
- Aligns with phenotype-tooling mission: developer tools and abstractions

**Decision: EXTRACT**
- **Source**: `HexaKit/clikit/` → **Target**: `phenotype-tooling/crates/clikit/`
- **Steps**:
  1. Copy crate to phenotype-tooling/crates/
  2. Update Cargo.toml in phenotype-tooling root
  3. Run tests: `cd phenotype-tooling && cargo test --package clikit`
  4. Update HexaKit Cargo.toml to depend on phenotype-tooling (git reference or published crate)
  5. Deprecate HexaKit version
- **Commit**: `chore(phenotype-tooling): extract clikit from HexaKit`

---

## 27 LIBRARIES — Deduplication & Cleanup

### Tier 1 — Verify Uniqueness (9 crates, high-value)

| Crate | LOC | Status | Action |
|-------|-----|--------|--------|
| phenotype-testing | 2,126 | Shared test infrastructure | Keep in HexaKit; document as test utilities |
| phenotype-infrastructure | 1,648 | Infrastructure abstractions | Keep; foundational for workspace |
| phenotype-health | 1,576 | Health check trait + impls | Keep; core abstraction |
| phenotype-mock | 1,516 | Mock implementations | Keep; test support |
| phenotype-cost-core | 1,480 | Cost tracking | Keep; domain-specific |
| phenotype-bdd | 1,358 | BDD test framework | Keep; test infrastructure |
| phenotype-event-sourcing | 1,284 | Event store with hash chains | Keep; architectural pattern |
| phenotype-iter | 1,094 | Iterator utilities | Keep; reusable utility |
| phenotype-project-registry | 1,088 | Project metadata registry | Keep; workspace-scoped |

**Deduplication**: No evidence of duplication in other repos. Keep all in HexaKit.

### Tier 2 — Archive Stubs (14 crates, <500 LOC each)

| Crate | LOC | Reason | Action |
|-------|-----|--------|--------|
| phenotype-observability | 0 | WIP stub; no code | Move to `.archive/` |
| phenotype-guard | 0 | Unused abstraction | Move to `.archive/` |
| pheno-guard | 0 | Duplicate stub | Move to `.archive/` |
| phenotype-config-core (crates/) | 142 | Duplicate (also in libs/); incomplete | Remove from crates/; keep libs/ version |
| phenotype-crypto | 1 | Placeholder only | Move to `.archive/` |
| phenotype-git-core | 1 | Placeholder only | Move to `.archive/` |
| phenotype-http-client-core | 1 | Placeholder only | Move to `.archive/` |
| phenotype-process | 1 | Placeholder only | Move to `.archive/` |
| phenotype-mcp | 1 | Placeholder only | Move to `.archive/` |
| phenotype-rate-limiter | ~400 | Incomplete; similar to phenotype-retry scope | Keep for now; document as "experimental" |
| phenotype-validation | ~350 | Input validation utilities | Keep; potentially reusable |
| phenotype-time | ~300 | Time abstractions | Keep; foundation for telemetry |
| agileplus-dashboard-server | 0 | Framework scaffolding only | Move to `.archive/` |
| agileplus-dashboard-client | 0 | Framework scaffolding only | Move to `.archive/` |

**Action**: Create `.archive/hexakit-stubs/` and move 9 zero-LOC projects + agileplus-dashboard components.

---

## Execution Plan (Next Steps)

### Phase 1: Cleanup (1 hour)
1. **Archive agileplus duplicate**
   - `git mv HexaKit/agileplus/ .archive/agileplus-hexakit-duplicate/`
   - Update HexaKit README to note canonical location
   - Commit: `chore(hexakit): archive agileplus duplicate; canonical in AgilePlus/`

2. **Archive empty stubs**
   - Create `.archive/hexakit-empty-stubs/` directory
   - Move 9 projects with 0-1 LOC into it
   - Commit: `chore(hexakit): archive 9 empty/stub projects for clarity`

3. **Remove duplicate config-core from crates/**
   - Remove `HexaKit/crates/phenotype-config-core/`
   - Keep `HexaKit/libs/phenotype-config-core/`
   - Commit: `chore(hexakit): deduplicate phenotype-config-core (crates/ → libs/)`

### Phase 2: Extractions (2 hours)
1. **Extract phenotype-xdd-lib → phenotype-tooling**
   - Copy crate, update Cargo.toml
   - Verify tests pass
   - Update HexaKit to reference phenotype-tooling (git)
   - Commit both repos

2. **Extract clikit → phenotype-tooling**
   - Same pattern as xdd-lib
   - Commit both repos

### Phase 3: Documentation (30 minutes)
1. Create `/docs/migrations/hexakit_library_deduplication.md` listing all Tier 2+ archival decisions
2. Update HexaKit README to explain product stratification (products vs libraries vs stubs)
3. Add comments to Cargo.toml for extracted crates referencing phenotype-tooling

### Commit Message (Final)
```
chore(hexakit): products promotion map + decisions

- Archive agileplus duplicate (canonical: AgilePlus/)
- Archive 9 empty stubs for workspace clarity
- Deduplicate phenotype-config-core (keep libs/ version only)
- Promote phenotype-xdd-lib + clikit to phenotype-tooling (target: next phase)
- Keep 27 core libraries in HexaKit as foundational infrastructure
- Document decisions in docs/migrations/hexakit_products_promotion.md

FUTURE WORK:
- Extract phenotype-xdd-lib and clikit when phenotype-tooling reaches stable state
- Consider extraction of policy-engine + retry to phenotype-infrakit (when created)
- Consolidate phenotype-python as standalone if consumption grows beyond HexaKit scope
```

---

## Summary Table

| Item | Classification | Action | Timeline |
|------|----------------|--------|----------|
| agileplus | PRODUCT (duplicate) | Archive | Phase 1 (now) |
| python workspace | PRODUCT (shared) | Keep | — |
| phenotype-policy-engine | PRODUCT (core) | Keep (defer extraction) | Phase 3 (future) |
| phenotype-retry | PRODUCT (core) | Keep (defer extraction) | Phase 3 (future) |
| phenotype-xdd-lib | PRODUCT (extraction) | → phenotype-tooling | Phase 2 |
| clikit | PRODUCT (extraction) | → phenotype-tooling | Phase 2 |
| 27 LIBRARIES | Mixed | 9 archive; 18 keep | Phase 1 |

---

## Risk Assessment

**Low Risk:**
- Archiving agileplus duplicate (canonical elsewhere)
- Archiving empty stubs (no code loss)
- Deduplicating config-core (keep better-located version)

**Medium Risk:**
- Extracting xdd-lib / clikit requires phenotype-tooling integration testing
- Mitigation: Run full test suite before commits; verify no broken dependencies

**Deferred Risk:**
- phenotype-policy-engine + phenotype-retry extractions require phenotype-infrakit creation
- Revisit when foundational Rust workspace is available
