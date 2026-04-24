# Rust Workspace Consolidation Analysis — 2026-04

## Executive Summary

**Active Rust workspaces:** 12 (3 large, 3 medium/small cohesive, 6 singletons)
**Total LOC:** ~1.7M Rust across all workspaces
**Consolidation opportunities:** 2 HIGH (Observably → PhenoObservability), 1 MEDIUM (device automation family)
**Anti-consolidation:** 2 LARGE (FocalPoint, AgilePlus) should remain standalone

---

## Current State Matrix

### Tier 1: Large Monoliths (>600K LOC)

| Workspace | Members | LOC | Purpose | Status |
|-----------|---------|-----|---------|--------|
| **FocalPoint** | 57 | 739K | Event replay, entitlements, rules engine | Production |
| **AgilePlus** | 52 | 693K | Project management, plugins, CLI framework | Core product |

**Assessment:** Both are intentionally large, domain-specific, and should remain independent.

### Tier 2: Medium Workspace (200K LOC)

| Workspace | Members | LOC | Purpose | Status |
|-----------|---------|-----|---------|--------|
| **PhenoObservability** | 10 | 213K | Tracing, logging, Dragonfly/QuestDB backends, MCP server | Production |

**Assessment:** Mature, multi-domain observability. Contains 2+ candidate sub-domains (logging, tracing, backends) but cohesive enough to remain unified.

### Tier 3: Small Cohesive Workspaces (1K-50K LOC)

| Workspace | Members | LOC | Purpose | Cohesion | Status |
|-----------|---------|-----|---------|----------|--------|
| **Stashly** | 4 | 7.5K | Cache, event store, state machine, migrations | High (generic infra) | Active |
| **Eidolon** | 4 | 553 | Device automation: core, desktop, mobile, sandbox | High (platform family) | Active |
| **Observably** | 3 | 1.1K | Tracing, logging, sentinel/monitoring | HIGH OVERLAP | ⚠️ Deprecated? |
| **thegent-workspace** | 2 | 42K | CLI tools: JSONL utils, generic utils | Medium | Active |
| **Sidekick** | 2 | 802 | Dispatch, messaging | Medium | Active |

### Tier 4: Single-Crate Workspaces (Singleton Pattern)

| Workspace | LOC | Purpose | Note |
|-----------|-----|---------|------|
| **PlayCua** | 3.9K | CUA UI automation (native) | Platform-specific (UI) |
| **bare-cua** | 3.9K | CUA bare-metal automation | Platform-specific (bare-metal) |
| **thegent-dispatch** | 398 | CLI dispatcher (Forge/Codex/Gemini/Copilot) | Single package, could merge to thegent-workspace |
| **kmobile** | 10K | Mobile device automation | Platform-specific (mobile) |

---

## Consolidation Opportunities

### 1. **Observably → PhenoObservability** (HIGH PRIORITY)

**Finding:** Two observability workspaces with overlapping domains.

#### Observably (3 crates):
- `observably-tracing` — distributed tracing utilities
- `observably-logging` — structured logging
- `observably-sentinel` — monitoring/alerting

#### PhenoObservability (10 crates):
- `pheno-tracing` — distributed tracing with tracing_subscriber
- `helix-logging` — structured logging with correlation IDs
- `tracely-sentinel` — monitoring/alerting (seems duplicate)
- Plus: `pheno-dragonfly`, `pheno-questdb`, `phenotype-mcp-server`, `tracely-core`, `tracingkit`, `phenotype-llm`, `phenotype-surrealdb`

**Issue:** Same functions, different repos, different naming conventions, no cross-reference.

**Recommendation:**
1. **Migrate:** Merge Observably crates into PhenoObservability under unified naming
2. **Rationale:**
   - PhenoObservability is larger, more mature (213K LOC vs 1.1K)
   - Both use identical deps: tokio, serde, thiserror, anyhow, tracing
   - Consolidation enables:
     - Unified release cycle for observability stack
     - Single integration surface (reduces 3→1 workspace import)
     - Shared CI/CD pipeline
3. **Effort:** Low (4 crate moves, namespace adjustments, single Cargo.toml)
4. **Timeline:** 1-2 days (includes cross-repo dep updates)

---

### 2. **Device Automation Family** (MEDIUM PRIORITY)

**Finding:** Four related repos for device automation, kept intentionally separate.

**Repos:**
- `Eidolon` (4 crates) — orchestration core + platform stubs
- `kmobile` (1 package) — mobile-specific implementation
- `KVirtualStage` (?) — sandbox/virtual display
- `PlayCua` (1 package) — UI automation
- `bare-cua` (1 package) — bare-metal automation

**Current Model:** Each repo is independent workspace/package.

**Assessment:** **INTENTIONALLY SEPARATE** (Good design)

**Rationale to Keep Separate:**
1. **Platform-specific dependencies:**
   - `kmobile` needs Android/iOS SDKs
   - `PlayCua` needs X11/Wayland/macOS accessibility APIs
   - `bare-cua` has bare-metal kernel requirements
   - Mixing them creates a 200MB+ build artifact for a single platform
2. **Independent release cycles:**
   - Mobile updates faster than desktop UI automation
   - Bare-metal patches rarely affect UI tools
3. **CI/CD optimization:**
   - Current: Platform-specific CI agents only build relevant crate
   - Merged: All CI agents always build all platforms (expensive, slow)
4. **Clear ownership boundaries:**
   - Each repo can have independent maintainers/roadmaps

**Recommendation:** **DO NOT consolidate.** This is correct separation.

---

### 3. **thegent-dispatch → thegent-workspace** (LOW PRIORITY)

**Finding:** `thegent-dispatch` is a single small binary not in a workspace.

**Option 1: Merge into thegent-workspace**
- Move `thegent-dispatch/src/main.rs` → `thegent-workspace/crates/thegent-dispatch/`
- Pros: Single workspace for CLI tools, unified CI
- Cons: Adds binary to a primarily library workspace
- **Recommendation:** DEFER — low value, thegent-dispatch is stable

**Option 2: Create a CLI workspace**
- Rename `thegent-workspace` → `thegent-cli-workspace`
- Add dispatch, other standalone CLIs
- **Recommendation:** Better separation of concerns if adding more CLIs

---

## Anti-Consolidations (Must Remain Separate)

### FocalPoint (57 crates, 739K LOC)

**Why it's NOT consolidating:**
- Largest Rust workspace in org (nearly 740K LOC)
- Domain-specific rule engine + event replay + entitlements
- Likely published as external package (independent versioning needed)
- 57 member crates already at practical workspace limit
- Adding more would exceed Cargo build times for CI

**Verdict:** **KEEP STANDALONE.** Monolith is intentional, domain-driven.

### AgilePlus (52 crates, 693K LOC)

**Why it's NOT consolidating:**
- Core product with independent release cycle
- Plugin ecosystem (52 crates) tightly coupled by design
- Needs independent version management from PhenoObservability, Stashly, etc.
- Already at workspace size limit

**Verdict:** **KEEP STANDALONE.** Core product architecture.

### Stashly (4 crates, 7.5K LOC)

**Why it's NOT consolidating:**
- Generic infrastructure (cache, event store, state machine, migrations)
- Could be published as a standalone library
- Low coupling to domain-specific code
- Reusable across multiple projects

**Assessment:** **KEEP STANDALONE** (not a problem, well-designed library)

---

## Dependency Alignment

All major workspaces standardize on:
- `tokio 1.39` (async runtime)
- `serde 1.0` (serialization)
- `thiserror 2.0` (error types)
- `anyhow 1.0` (error wrapper)
- `tracing 0.1` (observability)

**No version conflicts** — safe to merge Observably into PhenoObservability without dep resolution issues.

---

## Proposed Consolidation Plan

### Phase 1: Observably → PhenoObservability (Week 1)

1. **Create branch:** `chore/observably-integration` in PhenoObservability
2. **Move crates:**
   - Copy `Observably/crates/observably-*` → `PhenoObservability/crates/`
   - Rename to consolidate: `observably-tracing` → `pheno-tracing-legacy` (or merge with existing `pheno-tracing`)
3. **Update Cargo.toml:**
   - Add 3 members to PhenoObservability `[workspace]` members list
   - Update workspace deps to use workspace versions
4. **Namespace cleanup:**
   - Merge duplicate functionality (e.g., `observably-tracing` + `pheno-tracing`)
   - De-duplicate `sentinel` implementations
5. **Test & merge:**
   - `cargo test --workspace` in PhenoObservability
   - Update cross-repo callers of `Observably::*` → `PhenoObservability::*`
   - Merge to `main`
6. **Cleanup:**
   - Archive `Observably/` repo or repurpose

### Phase 2: Cross-Repo Dependency Updates (Week 2)

1. Find all repos importing `observably`:
   ```bash
   grep -r "observably" --include="Cargo.toml" .
   ```
2. Update to PhenoObservability crate paths
3. Run `cargo test --workspace` on all affected repos
4. Create PR with migration notes

### Phase 3: CI/CD Consolidation (Week 3)

- Merge observability-related CI jobs into single workflow
- Remove `Observably` from CI matrix
- Update docs to point to unified workspace

---

## Workspace Count Summary

| Category | Current | Post-Consolidation | Reduction |
|----------|---------|-------------------|-----------|
| Large monoliths (>600K) | 2 | 2 | — |
| Medium (200K+) | 1 | 1 | — |
| Small cohesive (1K-50K) | 5 | 4 | -1 |
| Singletons | 4 | 4 | — |
| **Total** | **12** | **11** | **-1** |

**Net reduction:** 1 workspace (Observably merged)
**Deprecation:** 1 namespace (observably-* → pheno-observability-*)
**No impact:** Device automation intentionally separate; FocalPoint/AgilePlus remain standalone

---

## Recommendation Summary

| Action | Priority | Effort | Impact |
|--------|----------|--------|--------|
| **Merge Observably → PhenoObservability** | HIGH | 1-2 days | Unified observability stack, cleaner org |
| **Keep device automation separate** | — | — | ✅ Current design is correct |
| **Keep FocalPoint/AgilePlus standalone** | — | — | ✅ Core products, correct isolation |
| **Consider thegent-dispatch → thegent-workspace** | LOW | <1 day | Minor cleanup, low value |
| **Keep Stashly standalone** | — | — | ✅ Reusable library, good separation |

---

## Files Affected by Consolidation

**Merge Observably → PhenoObservability:**
- `PhenoObservability/Cargo.toml` — add 3 members
- `Observably/` — ARCHIVE or DELETE
- All `Cargo.toml` files importing `observably` — UPDATE namespace
- `docs/ARCHITECTURE.md` — update observability section

**No other consolidations recommended at this time.**
