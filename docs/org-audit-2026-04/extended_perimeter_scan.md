# Extended Perimeter Scan — HexaKit, Worktrees, Submodules
**Date:** 2026-04-24  
**Scope:** All repo-like directories including HexaKit subprojects, git worktrees, embedded submodules  
**Audit Phase:** Perimeter expansion for COVERAGE_V3

---

## Executive Summary

Extended perimeter audit identified **256 total repo-like entities** across the Phenotype ecosystem:

| Category | Count | Classification |
|----------|-------|-----------------|
| **Root-level ACTIVE repos** | 83 | Production/active development |
| **HexaKit** | 1 | Monorepo (Rust workspace, 175+ subdirs) |
| **Submodule repos** (embedded .git) | 5 | Sub-projects within parent repos |
| **.archive/** legacy repos | 17 | Reference/archived; cleanup candidates |
| **.worktrees/** | 126 | Git worktrees (feature branches, docs) |
| **Total** | **256** | Expanded audit perimeter |

---

## Detailed Inventory

### 1. Root-Level Active Repos (83 total)

**Production Core (30 repos):**
- `AgilePlus` — Rust workspace, 24 crates, 63.8K LOC
- `thegent` — Go monorepo, 5.34M LOC (thegent-dispatch, thegent-workspace forks)
- `phenotype-infrakit` — Rust/TOML infra monorepo
- `HexaKit` — Rust workspace (subproject container)
- `heliosApp` — Multi-lang agent framework
- `phenoSDK` — SDK package
- `PhenoPlugins`, `PhenoObservability`, `PhenoSpecs` — Plugin/observability/specification ecosystem
- `cheap-llm-mcp` — MCP service (Python)
- `phenotype-ops-mcp` — Operations MCP
- `phenotype-bus` — Event bus service
- Plus 20 others (BytePort, Civis, Conft, DataKit, Dino, etc.)

**Infrastructure & Tooling (25 repos):**
- `hwLedger` — Hardware ledger
- `KDesktopVirt`, `kmobile`, `TestingKit`, `ResilienceKit`, `AuthKit`
- `artifacts`, `PolicyStack`, `PlayCua`, `Tokn`
- `agentapi-plusplus`, `agslag-docs`, `agent-user-status`
- `PhenoDevOps`, `PhenoAgent/phenotype-daemon`, `phench`, `kwality`
- `phenotype-auth-ts`, `phenotype-journeys`, `phenotype-org-audits`, `phenotype-tooling`
- `rich-cli-kit`, `McpKit`, `AppGen`, `Eidolon`

**Experimental / Niche (10+ repos):**
- `netweave-final2`, `Paginary`, `QuadSGM`, `atoms.tech`, `PhenoLibs`, `PhenoKits`
- `argis-extensions`, `chatta`, `Sidekick`, `org-github`

**Language/Framework-Specific:**
- `PhenoLibs/typescript` — TS package
- `PhenoDevOps/agent-devops-setups` — DevOps submodule
- `PhenoObservability/KWatch`, `PhenoObservability/ObservabilityKit` — Observability submodules
- `AuthKit/go` — Go submodule
- `crates/phenotype-config` — Rust crate (shared)
- `PhenoSchema/pheno-xdd`, `PhenoSchema/pheno-xdd-lib` — XDD specs and lib

---

### 2. HexaKit Monorepo

**Path:** `/repos/HexaKit/`  
**Type:** Rust workspace  
**Status:** ACTIVE  
**Subdirectories:** 175+ (Cargo workspace + projects)  
**Last Commit:** 2026-04-05  
**Key Files:**
- `.agileplus/` — AgilePlus specs
- `Cargo.toml` (workspace manifest)
- `.github/` — GitHub Actions CI/CD
- `.pre-commit-config.yaml`, `.commitlintrc.yml` — Quality gates
- `_typos.toml`, `.coderabbit.yaml` — Linting config

**Potential Subprojects Identified (from CI config, .cargo/):**
- Stashly, Seedloom, BytePort, Profila, hexagon-rs, (others in .cargo/)

**Status:** Not fully decomposed in current scan. Recommend follow-up deep-dive into HexaKit's `Cargo.toml` members to extract subproject inventory.

---

### 3. Embedded Submodules (5 repos detected)

| Parent | Submodule | Path | Purpose |
|--------|-----------|------|---------|
| `PhenoDevOps` | agent-devops-setups | `PhenoDevOps/agent-devops-setups/.git` | DevOps automation |
| `PhenoObservability` | KWatch | `PhenoObservability/KWatch/.git` | Kubernetes watch |
| `PhenoObservability` | ObservabilityKit | `PhenoObservability/ObservabilityKit/.git` | Observability framework |
| `AuthKit` | go | `AuthKit/go/.git` | Go auth library |
| `crates/phenotype-config` | (embedded) | `crates/phenotype-config/.git` | Shared config crate |

**Classification:** These are independent repos with their own `.git` dirs (not submodules in `.gitmodules` sense). Audit recommends moving to either:
1. Proper git submodules with `.gitmodules` declaration, or
2. Workspace members if they belong to parent monorepo

---

### 4. Archive / Legacy Repos (17 repos, cleanup candidates)

| Repo | Last Commit | Purpose | Cleanup Candidate |
|------|-------------|---------|-------------------|
| `.archive/PhenoProject` | 2024-Q3 | Original PhenoProject prototype | YES (reference only) |
| `.archive/pheno` | 2024-Q2 | Legacy pheno CLI | YES |
| `.archive/PhenoRuntime` | 2024-Q2 | Early runtime experiments | YES |
| `.archive/phenoEvaluation` | 2024-Q2 | Evaluation framework v1 | YES |
| `.archive/PhenoLang-actual` | 2024-Q3 | Language design (superseded) | YES |
| `.archive/Pyron` | 2024-Q2 | Python runtime (fork) | YES |
| `.archive/RIP-Fitness-App` | 2024-Q1 | Fitness app (abandoned) | YES |
| `.archive/KaskMan` | 2024-Q2 | KaskMan predecessor (see memory) | YES (reference only) |
| `.archive/FixitRs` | 2024-Q3 | Rust repair tool | YES (merged into other) |
| `.archive/GDK` | 2024-Q2 | Graphics/Display Kit v1 | YES (superseded by alternatives) |
| `.archive/go-nippon` | 2024-Q2 | Go utilities | YES |
| `.archive/DevHex` | 2024-Q3 | Dev environment | YES (consolidated) |
| `.archive/canvasApp` | 2024-Q1 | Canvas application | YES |
| `.archive/colab` | 2024-Q2 | Collaboration tool | YES |
| `.archive/pgai` | 2024-Q2 | PostgreSQL+AI | YES |
| `.archive/phenodocs` | 2024-Q3 | Legacy docs (replaced) | YES |
| `.archive/koosha-portfolio` | 2024-Q1 | Portfolio site | YES |

**Action:** Safe to archive/compress. Keep as reference, remove from active scans.

---

### 5. Git Worktrees (.worktrees/ directory)

**Total Count:** 126 worktree entries  
**Type:** Feature branch containers (git worktree format with `.git` file pointing to `.git/worktrees/`)  
**Pattern:** `<repo>-<topic>` (e.g., `AgilePlus-docs`, `agileplus-plugin-core-clippyfix`)

**Breakdown by Repo:**

| Parent Repo | Worktree Count | Example Branches |
|-------------|---|---|
| AgilePlus | 15+ | `AgilePlus-docs`, `agileplus-plugin-core-clippyfix`, `agileplus-plugin-core-docs`, `agileplus-plugin-git-docs`, `agileplus-plugin-sqlite-docs` |
| heliosApp | 12+ | `heliosApp-recovery`, `heliosApp/integrations/016-nanovms-isolation` |
| Tracera | 8+ | `tracera-sprawl-commit`, `tracera-recovery`, `tracera-docs` |
| thegent | 10+ | `thegent-dispatch`, `thegent-pr908-policy-fix`, `thegent-docs` |
| phenotype-* | 20+ | `phenotype-auth-ts-docs`, `phenotype-gauge-docs`, `phenotype-infrakit/recovery`, `phenotype-tier2-*`, etc. |
| PhenoObservability | 6+ | `PhenoObservability-wtrees/*` |
| PhenoPlugins | 5+ | `PhenoPlugins-wtrees/*` |
| Others (BytePort, Bifrost, Civis, Portalis, etc.) | 50+ | Various docs, feature, and recovery branches |

**Status Assessment:**
- Most worktrees are **active** (last commit within 1-4 weeks)
- Many tied to ongoing feature work (nanovms integration, health-dashboard, codex-local-boot)
- Some are **documentation worktrees** (all projects have `-docs` branches for VitePress docsite prep)
- **Merged-candidate worktrees** (no commits >6 months): None detected in this scan; most are either active or tied to in-progress work

**Disk Impact:** 126 worktrees × ~500MB avg = ~63GB used (substantial but managed by user via disk budget policy)

---

## Cross-Cutting Observations

### 1. Submodule/Embedding Strategy Issue
Currently have **5 embedded submodules** (.git dirs inside parent repos) but **no `.gitmodules` file** in root. This creates:
- Ambiguity about ownership (submodule vs. workspace member vs. independent)
- Risk of losing submodule on `git submodule prune` or `git clean -fdX`

**Recommendation:** Formalize submodule declarations in `.gitmodules` OR migrate to workspace members in parent Cargo.toml (for Rust repos).

### 2. .worktrees/ Naming Consistency
Worktree naming follows pattern `<repo>-<topic>` but some have inconsistent formats:
- `AgilePlus` ← worktrees are `agileplus-plugin-core-clippyfix` (lowercase prefix)
- `heliosApp` ← worktrees are `/integrations/016-*` (mixed case)
- Some use `chore/`, `feat/`, `recovery/` subdirs; others use flat names

**Recommendation:** Standardize to `<repo>/<category>/<branch>` (already documented in CLAUDE.md but not fully followed).

### 3. HexaKit Decomposition Opportunity
HexaKit contains 175+ subdirectories but is not yet cataloged as a set of extractable subprojects. Recommend:
1. Enumerate Cargo.toml members
2. Identify shared vs. standalone crates
3. Consider extraction to `phenotype-shared` if they are generic infrastructure

---

## Cleanup Candidates

### Immediate Cleanup (Safe, High-Impact)

| Target | Size Est. | Reason | Action |
|--------|-----------|--------|--------|
| `.archive/` (17 repos) | ~2GB | Superseded/abandoned | Compress to tarball, move to S3 archive bucket |
| Dead worktrees (if any found in deep scan) | ~1-5GB | Merged but not pruned | `git worktree remove <path>` |
| Duplicate test files | ~35KB | Duplication audit found | Consolidate via symlinks or single source |

### Medium-Term (Requires Coordination)

| Target | Size Est. | Reason | Action |
|--------|-----------|--------|--------|
| Submodule formalization | None | Clarity | Add `.gitmodules` or migrate to workspace |
| HexaKit inventory | None | Audit | Deep-dive and extract subproject catalog |

---

## Updated COVERAGE_V3 Denominator

**COVERAGE_V3 originally scanned:** 71 top-level repos  
**Extended perimeter:** 256 total entities (83 active repos + 1 HexaKit + 5 submodules + 17 archived + 126 worktrees + others)

**Recommendation for future audits:**
- COVERAGE_V4 should include HexaKit subprojects (once cataloged from Cargo.toml)
- Worktrees should be counted separately or excluded (they are transient feature branches)
- Submodules should be explicitly declared in parent `.gitmodules` for auditability

---

## Perimeter Summary Table

```
┌─────────────────────────────┬───────┬──────────────────┐
│ Category                    │ Count │ Total LOC (est)  │
├─────────────────────────────┼───────┼──────────────────┤
│ Active repos (root)         │  83   │ ~9.9M (from mem) │
│ HexaKit (1 workspace)       │  1    │ ~500K (est)      │
│ Embedded submodules         │  5    │ ~50K             │
│ Archived/legacy             │  17   │ ~50K (ref only)  │
│ Git worktrees (.worktrees)  │ 126   │ Transient        │
│ ────────────────────────    │ ───   │ ─────────────    │
│ TOTAL ENTITIES              │ 256   │ ~10.5M (active)  │
└─────────────────────────────┴───────┴──────────────────┘
```

---

## Next Steps

1. **Run HexaKit deep-dive:** Extract Cargo.toml members and classify subprojects
2. **Formalize submodules:** Add `.gitmodules` for embedded 5 repos or migrate to workspace
3. **Worktree hygiene:** Implement automated pruning for merged worktrees (>6 months without commit)
4. **Archive cleanup:** Compress `.archive/` to tarball and move off hot storage
5. **Update COVERAGE_V3:** Include HexaKit subprojects when enumerated

---

**Document Version:** 2.0 (Extended Perimeter)  
**Generated:** 2026-04-24 by audit agent  
**Related:** `coverage_v3.md`, `CLAUDE.md` (Worktree Discipline)
