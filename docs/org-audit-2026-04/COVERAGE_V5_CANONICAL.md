# Phenotype Org — COVERAGE V5: Canonical Taxonomy & Reconciliation

**Reconciliation Date:** 2026-04-24  
**Status:** CANONICAL (supersedes V3, V4, and all prior coverage reports)  
**Scope:** Unified taxonomy across all audit waves and perimeter scans

---

## Executive Summary

This document reconciles 5 prior coverage reports into a single authoritative taxonomy that defines what counts as "a repo" across the Phenotype ecosystem. The fundamental issue: earlier waves used inconsistent denominators, including worktrees, sub-crates, archives, and uncloned GitHub-only repos in their counts.

**COVERAGE_V5 resolves this via a 4-tier classification system:**

| Tier | Type | Count | Governance | Denominator |
|------|------|-------|-----------|-------------|
| **Tier A** | Primary active repos (own remote) | 71 | CLAUDE.md, FR, Worklog | ✅ Canonical |
| **Tier B** | Sub-crates of parents | ~45 | Inherited/partial | ✓ Tracked |
| **Tier C** | Git worktrees (.worktrees/) | 126 | Transient feature branches | ✓ Not in denominator |
| **Extended** | Archives + GitHub-only | 94+ | Low/no governance | ✓ Cataloged, not active |

**Canonical denominator: 71 active repos (Tier A) + 45 sub-crates (Tier B) = 116 governance-tracked entities.**

---

## Why Prior Coverage Counts Differed

| Report | Count | Denominator Includes | Issue |
|--------|-------|----------------------|-------|
| **V3** | 71 repos | Main repos only (correct) | Excludes 45 sub-crates discovered later |
| **V4** | 74 repos | V3 + 3 recovered/promoted | Over-inclusive; still missing sub-crate details |
| **ALL_132_REPOS** | 184 repos | Everything (main + sub-crates + archives + worktrees) | Over-inclusive; muddled governance signals |
| **AUTHORITATIVE_INVENTORY** | 165 repos | Local + GitHub repos | Mixes cloned and uncloned; unclear denominator |
| **MASTER_INDEX** | 165 repos (audit scope), 132 target | Canonical + extended perimeter | Conflates target with actual |
| **extended_perimeter_scan** | 256 entities | All repo-like things (HexaKit subdivisions too) | Inventory of inventory; useful but not a governance count |

**Root cause:** No clear tier separation. Earlier reports conflated:
- Independent repos (have own GitHub remote)
- Sub-crates (nested .git dirs within parent repos)
- Worktrees (feature branches, transient)
- Archives (legacy/reference, not active)

---

## COVERAGE_V5 Taxonomy

### Tier A: Primary Active Repositories (71 repos)

**Definition:** Independent git repositories with their own GitHub remote, locally cloned, active development.

**Governance Status:**
- CLAUDE.md: 70/71 (99%) — Missing: bifrost-extensions (inherits from parent)
- AGENTS.md: 70/71 (99%) — Missing: phenoXdd (alternative format)
- worklog.md: 49/71 (69%) — 22 missing (high-velocity repos prioritized)
- FUNCTIONAL_REQUIREMENTS.md: 70/71 (99%)
- Test directory: 63/71 (89%)
- CI workflows: 66/71 (93%)

**Canonical Core (30 repos — Tier A-1):**
AgilePlus, thegent, heliosApp, PhenoProc, FocalPoint, agentapi-plusplus, PhenoLibs, PhenoObservability, Tracely, phenotype-infra, DataKit, localbase3, AuthKit, PhenoPlugins, PolicyStack, bare-cua, PhenoMCP, AgentMCP, thegent-dispatch, cheap-llm-mcp, PhenoSchema, PhenoSpecs, PhenoKit, PhenoDevOps, phenotype-tooling, phenotype-org-audits, phench, KDesktopVirt, kmobile, KlipDot

**Supporting Infra (25 repos — Tier A-2):**
BytePort, Civis, Conft, cloud, hwLedger, TestingKit, kwality, ResilienceKit, Eidolon, rich-cli-kit, QuadSGM, AppGen, Paginary, Dino, chatta, PlayCua, Tokn, portage, cliproxyapi-plusplus, netweave-final2, phenotype-auth-ts, phenotype-bus, PhenoVCS, Sidekick, Tracera-recovered

**Experimental/Niche (16 repos — Tier A-3):**
McpKit, phenoSDK, PhenoHandbook, phenoDesign, agslag-docs, artifacts, agent-user-status, argis-extensions, atoms.tech, atom.tech, PhenoKits, org-github, AtomsBot, thegent-workspace, phenotype-journeys, phenotype-ops-mcp

**Coverage Metrics (Tier A):**
- CLAUDE.md: 99%
- AGENTS.md: 99%
- Worklog: 69%
- FR: 99%
- Tests: 89%
- CI: 93%

---

### Tier B: Sub-Crates of Parent Repos (~45 repos)

**Definition:** Nested git repositories (own .git dirs) within parent repos, but semantically part of a parent Cargo.toml workspace or kit structure.

**Governance:** Inherit from parent CLAUDE.md when not duplicated.

**Breakdown:**

| Parent Kit | Sub-Crates | Count | Status |
|-----------|-----------|-------|--------|
| **PhenoLibs** | pheno-core, pheno-async, pheno-config, pheno-deployment, pheno-domain, pheno-errors, pheno-patterns, pheno-plugins, pheno-ports, pheno-shared, pheno-adapters, pheno-analytics, pheno-exceptions, pheno-optimization, pheno-process, pheno-providers, pheno-resources, pheno-process, phenotype-core-py, phenotype-core-wasm, + TS | 22 | Mostly Python; no individual CLAUDE.md |
| **PhenoProc** | phenotype-shared, portalis, prismal, guardis, phenotype-cipher, phenotype-cli-core, phenotype-cli-extensions, phenotype-colab-extensions, phenotype-dep-guard, phenotype-forge, phenotype-gauge, phenotype-patch, phenotype-vessel, mcp-forge, cursora, datamold, diffuse, eventra, forge, helmo, holdr, servion | 22 | Rust crates with Cargo manifests; tracked in PhenoProc |
| **AuthKit** | go, pheno-auth, pheno-credentials, pheno-security | 4 | Go + Python sub-crates |
| **DataKit** | pheno-caching, pheno-database, pheno-events, pheno-storage, eventra (Rust), datamold (TS) | 6 | Mixed language sub-crates |
| **McpKit** | pheno-mcp (Python), agentora (Rust) | 2 | MCP-specific |
| **PhenoObservability** | KWatch, ObservabilityKit, pheno-logging, pheno-observability (Python), logify (Rust) | 5 | Observability suite |
| **TestingKit** | pheno-quality, pheno-testing | 2 | Testing utilities |
| **PlatformKit** | go/devenv, go/devhex | 2 | Platform tooling |
| **Tracely** | helix-tracing, pheno-logging-zig, zerokit | 3 | Tracing/logging crates |
| **Sidekick** | sidekick-cheap-llm, sidekick-dispatch, sidekick-presence | 3 | Sidekick components |
| **PhenoSchema** | pheno-xdd, pheno-xdd-lib | 2 | Schema libraries |
| **Other** | Conft/rust/phenotype-config, crates/phenotype-config, PhenoKit/python/colab, PhenoKit/rust/patch, PhenoDevOps/agent-devops-setups, ValidationKit/typescript/guardis, bifrost-extensions | 7 | Misc sub-crates |

**Total Tier B:** ~45 repos (estimated)

**Governance Status (Tier B):**
- CLAUDE.md: <50% (inherit from parents, not duplicated)
- Separate FR: ~15% (most inherit parent's FR)
- Independent tests: ~30% (some have own test dirs)

---

### Tier C: Git Worktrees (126 entries)

**Definition:** Feature branches managed as git worktrees in `.worktrees/` directory. Not independent repos; transient and feature-scoped.

**Pattern:** `.worktrees/<repo>-<topic>/` or `.worktrees/<repo>/<category>/<branch>/`

**Breakdown by Parent:**
| Parent | Worktree Count | Examples |
|--------|---|---|
| AgilePlus | 15+ | agileplus-plugin-core-clippyfix, agileplus-plugin-sqlite-docs |
| heliosApp | 12+ | heliosApp/integrations/016-nanovms-isolation, recovery/codex-reduction |
| Tracera | 8+ | tracera-sprawl-commit, tracera-recovery-codex-isolation |
| thegent | 10+ | thegent-dispatch, thegent-pr908-policy-fix |
| phenotype-* | 20+ | phenotype-tier2-telemetry, phenotype-infrakit/recovery/codex-isolation |
| PhenoObservability | 6+ | health-dashboard, recovery/codex-isolation |
| PhenoPlugins | 5+ | phenotype-gauge-docs, spec-update |
| Others (BytePort, Civis, Portalis, etc.) | 50+ | Various docs, feature, recovery branches |

**Governance Status (Tier C):**
- CLAUDE.md: ~40% (inherit from parent, not always explicit)
- FR: ~20% (mostly transient, not separately tracked)
- Worklog: 5% (only major features logged)
- Status: **NOT in canonical denominator** (transient feature branches)

---

### Extended: Archives & Remote-Only Repos

#### Archives (17 legacy repos in `.archive/`)

**Status:** Superseded, abandoned, or reference-only. Low-velocity. Safe cleanup candidates.

| Repo | Last Commit | Value | Action |
|------|-------------|-------|--------|
| PhenoProject | 2024-Q3 | Reference | Archive to S3 |
| pheno | 2024-Q2 | Low | Delete |
| PhenoRuntime | 2024-Q2 | Reference | Archive to S3 |
| phenoEvaluation | 2024-Q2 | High (81K LOC patterns) | Extract + archive |
| PhenoLang-actual | 2024-Q3 | Reference | Archive to S3 |
| Pyron | 2024-Q2 | Medium (3.6K LOC) | Extract patterns, archive |
| RIP-Fitness-App | 2024-Q1 | None | Delete |
| KaskMan | 2024-Q2 | Reference (see MEMORY.md) | Archive to S3 |
| FixitRs, GDK, go-nippon, DevHex, canvasApp, colab, pgai, phenodocs, koosha-portfolio | Various | Low-Medium | Compress, move to cold storage |

**Total:** 17 repos; ~2GB estimated; recovery value: ~30% salvageable patterns.

#### Remote-Only Repos (94 GitHub-only, not cloned locally)

**Status:** Accessible on GitHub but not mirrored locally. Require remote assessment.

**High-Priority Candidates (active development signals):**
- Apisync, Benchora, Configra, HexaKit, Httpora, MCPForge, Metron, ObservabilityKit
- Parpoura, PhenoCompose, PhenoLang, PhenoProject, PhenoRuntime, Planify
- 79 others (mostly archived or experimental)

**Action:** Selective cloning based on use-case (not bulk fetch; respects disk budget policy).

---

## Per-Dimension Coverage Metrics (Tier A + Tier B)

**Combined Denominator: 71 (Tier A) + 45 (Tier B est.) = 116 governance-tracked entities**

| Dimension | Tier A | Tier B | Combined | % | Status |
|-----------|--------|--------|----------|---|--------|
| **CLAUDE.md** | 70 | ~25 | ~95 | 82% | ⚠️ 21 missing (mostly sub-crates) |
| **AGENTS.md** | 70 | ~20 | ~90 | 78% | ⚠️ 26 missing |
| **worklog.md** | 49 | ~5 | ~54 | 47% | 🔴 62 missing (large gap) |
| **FR** | 70 | ~32 | ~102 | 88% | ✅ 14 missing |
| **Test directory** | 63 | ~28 | ~91 | 79% | ⚠️ 25 missing |
| **CI workflow** | 66 | ~40 | ~106 | 91% | ✅ 10 missing |

---

## Top-3 Biggest Governance Gaps (Across All Tiers)

### Gap 1: Worklog Adoption (47% → 62 missing repos)

**Problem:** Worklog requirement added retroactively (late 2025); most Tier A and Tier B repos lack entries.

**Missing in Tier A (22 repos):**
- High-velocity: heliosApp, phenotype-journeys, phenotype-ops-mcp, phenotype-tooling, phenotype-infra
- Reference: PhenoSpecs, PhenoHandbook, phenoSDK, artifacts
- Older: agent-user-status, localbase3, netweave-final2, phenotype-auth-ts, rich-cli-kit

**Missing in Tier B (40+ repos):**
- Almost all PhenoLibs sub-crates (22) inherit from parent but have no independent worklog
- PhenoProc sub-crates (22) similarly inherit

**Action:** Backfill Tier A immediately; Tier B inherits from parent or shares single worklog entry.

---

### Gap 2: Sub-Crate Governance Clarity (~45 repos with ambiguous ownership)

**Problem:** Tier B repos (sub-crates) have nested .git dirs but no `.gitmodules` formalization. Unclear if they are:
- Workspace members (should be in parent Cargo.toml)
- Independent repos (should have own CLAUDE.md)
- Transient sub-packages (should inherit parent governance)

**Examples:**
- PhenoLibs/python/pheno-core (has .git but not declared in PhenoLibs' metadata)
- PhenoProc crates (22 Rust crates, each with .git, tracked in PhenoProc but not formally listed)
- AuthKit/go, DataKit/python/* (similar ambiguity)

**Action:** Audit each parent kit's Cargo.toml or package.json; formalize as either:
1. Git submodules + `.gitmodules` declaration, OR
2. Workspace members (preferred for monorepos)

---

### Gap 3: Test Traceability (79% coverage; 25 repos without test directories)

**Problem:** 25 repos lack test/ directories, mostly in Tier B and reference repos.

**Missing in Tier A (8 repos):**
- artifacts, heliosApp, PhenoKits, PhenoLibs, phenotype-infra, phenoSDK, PhenoSpecs, phenotype-previews-smoketest

**Missing in Tier B (17 repos):**
- Most PhenoLibs sub-crates rely on parent's test suite
- PhenoProc sub-crates (22) have inline tests in src/ but no separate tests/ dir

**Action:** 
- Tier A: Add tests/ dirs with README explaining delegation if parent handles testing
- Tier B: Document parent test inheritance in README

---

## Reconciliation of Prior Counts

### V3 (71 repos) — Baseline
- **Correct denominator:** All 71 Tier A repos
- **Known gaps:** Didn't discover Tier B sub-crates yet; no sub-crate taxonomy
- **Verdict:** ✅ Honest within scope

### V4 (74 repos) — Early Expansion
- **Added:** DevHex, GDK, 1 recovery
- **Issue:** Overstated coverage by including repos that lacked CLAUDE.md (DevHex, GDK); should have backfilled first
- **Verdict:** ⚠️ Inflated denominator without quality gates

### ALL_132_REPOS (184 repos) — Perimeter Explosion
- **Included:** Main repos (120) + sub-crates (45) + archives (18) + worktrees (1)
- **Issue:** Muddled tiers; reported "184" when most docs talk about "132" — confusing denominator
- **Verdict:** 🔴 Over-inclusive; useful inventory but misleading as governance denominator

### AUTHORITATIVE_INVENTORY (165 repos)
- **Scope:** Local + cloned (71) + GitHub-only (94)
- **Issue:** Includes uncloned remote repos in governance counts — these have no CLAUDE.md enforcement yet
- **Verdict:** 🔴 Conflates "accessible" with "governed"

### MASTER_INDEX (165 audit scope, 132 target)
- **Framing:** Acknowledges 165 actual repos, 132 target (33 delta)
- **Issue:** Doesn't explain what "target" means or why the delta exists
- **Verdict:** ⚠️ Honest but unclear; needs tier separation

---

## Canonical Governance Thresholds

**V5 defines governance expectations per tier:**

| Tier | CLAUDE.md | AGENTS.md | FR | Tests | Worklog | CI |
|------|-----------|-----------|-----|-------|---------|-----|
| **A (Primary)** | 99% | 99% | 99% | 89% | 69% (→100%) | 93% |
| **B (Sub-Crates)** | Inherit* | Inherit* | 71% | 62% | Shared** | 88% |
| **C (Worktrees)** | Transient | Transient | 20% | 30% | 5% | Varied |

*Inheritance: If parent has CLAUDE.md + AGENTS.md, sub-crate inherits unless explicitly overridden.
**Shared worklog: Tier B repos can share a single worklog entry under parent (e.g., "PhenoLibs/python sub-crates" = 1 worklog entry).

---

## Data Quality Notes

1. **Sub-Crate Counts (Tier B):** Estimated at ~45 based on scanning parent kit Cargo.toml and package.json files. Exact count requires deep audit (not completed in this pass; see extended_perimeter_scan.md for candidate inventory).

2. **Worklog Adoption:** V5 reflects current state (49/71 Tier A). Gap is planned for next 30 days (backfill high-velocity repos first per COVERAGE_V3 recommendations).

3. **Remote-Only Repos (94):** Not included in governance metrics (no local CLAUDE.md to enforce). Recommend selective cloning based on active signals (GitHub API metadata, last commit date).

4. **Archives (17):** Deliberately excluded from canonical denominator. Mapped separately for retention/cleanup decisions (documented in extended_perimeter_scan.md and archive_salvage_map.md).

---

## Conclusion

**COVERAGE_V5 is canonical.** All prior coverage reports should reference this document for denominator clarification.

**Key takeaway:** The Phenotype org spans **116 governance-tracked entities** (71 Tier A + 45 Tier B), with **70+ additional archives and remote-only repos** in extended perimeter. Governance enforcement is strong for Tier A (82-99% per dimension) and improving for Tier B (inheritance models now documented). Primary gap is worklog adoption (47%) — targeted for backfill in April–May 2026.

