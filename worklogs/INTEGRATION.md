# INTEGRATION Worklog
**Category: INTEGRATION**

External integration notes for Phenotype-org repositories. See `worklogs/README.md`
for category conventions and `worklogs/AGENT_ONBOARDING.md` for writing guidance.

---

## [phenotype-ops-mcp] 2026-04-23 — Fork of nanovms/ops-mcp (Task #59)

Forked [`nanovms/ops-mcp`](https://github.com/nanovms/ops-mcp) (Apache-2.0, Go,
~106 KiB) into the KooshaPari namespace as
[`phenotype-ops-mcp`](https://github.com/KooshaPari/phenotype-ops-mcp) and cloned
to `repos/phenotype-ops-mcp` with `upstream` remote pointing back at
`nanovms/ops-mcp`. Rationale: ops-mcp is the canonical MCP server for the nanoVMs
`ops` unikernel toolchain, which makes it the natural bridge between Phenotype's
MCP-first agent stack and nanoVMs unikernels. We already promoted
`phenotype-nanovms-client` (bare-cua -> phenoShared, PR #82 merged) as the
typed Rust client for the `ops` CLI; forking the MCP server lets us extend the
tool surface (auth, multi-tenant instance isolation, observability hooks,
Phenotype policy-engine integration) while tracking upstream. Planned
integration points: (1) wire `phenotype-ops-mcp` tool handlers to call
`phenotype-nanovms-client` instead of shelling out to `ops` directly where
feasible, (2) expose Phenotype auth/tenant context via MCP server metadata,
(3) add observability (phenotype-observability) around each tool invocation,
(4) upstream any non-Phenotype-specific improvements back to
`nanovms/ops-mcp` via PR before landing in the fork. Fork attribution landed on
branch `chore/fork-attribution` (README NOTICE + this worklog entry).

---

## 2026-04-24 — Release-Cut Org-Wide Adoption

Deployed `phenotype-tooling::release-cut` pattern across 8 Tier-A repos.

**Repos onboarded** (7 new + 1 reference):
- **FocalPoint** (Rust, reference): shipped 2026-04-22
- **AgilePlus** (Rust, 24-crate workspace): `.github/workflows/release-dry-run.yml` + docs
- **PhenoObservability** (Rust): minimal single-crate variant
- **Tracely** (Rust, deprecated per audit): adopted with deprecation notice
- **PhenoPlugins** (Rust, single crate): adopted
- **Tracera** (Node, monorepo): adapted for `package.json` triggers + CHANGELOG stub
- **heliosApp** (Node, CalVer): adapted with CalVer version extraction
- **PhenoSpecs** (Markdown docs): adapted for markdown triggers + git tag versioning

**Artifacts delivered**:
- `.github/workflows/release-dry-run.yml` (7 repos): PR-triggered dry-run validation
- `docs/release/tag_command.md` (8 repos): manual tag + recovery procedures
- `CHANGELOG.md` seeded (Tracera, PhenoSpecs + existing repos updated): "Unreleased" sections
- `docs/governance/release_cut_adoption.md`: comprehensive tracker + per-repo worklog breakdown
- `phenotype-tooling/crates/release-cut/README.md`: updated with org-wide adoption guide + customization patterns

**Pattern standardized**:
1. PR-triggered dry-run validates release plan on version/manifest changes
2. Manual execution: `release-cut v<VERSION> --execute` (Rust) or language-specific equivalent (Node/Docs)
3. Recovery: rollback commands for each language variant
4. CHANGELOG: all repos maintain "Unreleased" section

**Worklog commits**:
- `feat(release): adopt phenotype-tooling release-cut workflow` (7 per-repo commits)
- `docs(release-cut): update README with org-wide adoption guide` (phenotype-tooling)
- `docs(org): release-cut adoption tracker — 8 repos onboarded` (repos root)

**Key design decisions**:
- Rust projects: Direct `release-cut` binary (fully automated)
- Node projects: Custom dry-run via version extraction + validation script
- Docs projects: Git tags + CHANGELOG as source of truth (no version bump automation)
- All repos share `.github/workflows/` location + `docs/release/` documentation pattern

---

## 2026-04-24 — Cross-Repo Consolidation Analysis: phenoShared ↔ pheno

**Scope**: Comparative analysis of phenoShared (13K LOC, 12 crates) and pheno (215K LOC monorepo)
to identify duplicates, unique modules, and extraction candidates.

### Key Findings

**Overlap**: 4 duplicated crates (4.8K LOC total overlap, <1% of workspace)
- phenotype-event-sourcing: 1,816 (phenoShared) vs 642 (pheno) → **keep phenoShared**
- phenotype-cache-adapter: 812 (phenoShared) vs 78 (pheno) → **keep phenoShared**
- phenotype-policy-engine: 1,402 (phenoShared) vs 1,823 (pheno) → **keep pheno** (more complete)
- phenotype-state-machine: 664 (phenoShared) vs 355 (pheno) → **keep phenoShared**

**Unique to phenoShared** (8 crates, 7.3K LOC, all HIGH-VALUE):
- phenotype-nanovms-client (2,284 LOC) — NanoVM bridge; **CRITICAL**
- phenotype-domain (1,697 LOC) — Core domain types; **CRITICAL**
- phenotype-port-interfaces (1,257 LOC) — Hexagonal port traits; **CRITICAL**
- phenotype-application (1,097 LOC) — App layer abstractions
- phenotype-postgres-adapter (203 LOC)
- phenotype-redis-adapter (204 LOC)
- phenotype-http-adapter (127 LOC)
- ffi_utils (9 LOC)

**Top 10 Extractable from pheno** (18.5K LOC):
1. phenotype-retry (1,656) — Pure utility, zero agileplus coupling; **TIER 1**
2. phenotype-port-traits (1,004) — Trait definitions; **TIER 1**
3. phenotype-policy-engine (1,823) — Consolidate with phenoShared variant; **TIER 1**
4. phenotype-infrastructure (814) — Observability abstraction; **TIER 2**
5. phenotype-health (788) — Health check abstraction; **TIER 2**
6. phenotype-testing (1,063) — Split into shared fixtures; **TIER 2**
7. phenotype-mock (754) — Test mocking utilities
8. phenotype-cost-core (740) — Billing abstractions [DEFER on demand]
9. phenotype-bdd (679) — BDD scaffolding [DEFER on demand]
10. phenotype-event-bus (393) — Pub/sub event system

### Consolidation Strategy

**Deliverable**: `docs/migrations/pheno_extraction_plan.md` (15-section roadmap covering modules, dependencies, effort, timeline).

**Execution Plan** (3 phases, 16-20h or 6-8h parallel):
1. **Phase 1 (4-6h)**: Create canonical `phenotype-shared` from phenoShared; extract phenotype-retry, port-traits, policy-engine
2. **Phase 2 (6-8h)**: Extract infrastructure, health, testing split
3. **Phase 3 (2-3h)**: Transfer remaining phenoShared unique modules; mark phenoShared deprecated

**Deprecation**: `phenoShared/DEPRECATION.md` created with migration guide for dependent projects.

**Outcome**: Single canonical `phenotype-shared` repo consolidates 15.3K LOC of reusable infrastructure (merged phenoShared unique + best-of-both-duplicate variants + pheno tier-1 extractions), eliminating duplication and establishing clear ownership.

---

## 2026-04-24 — Reusable-workflow adoption audit

**Canonical reusable workflows** (hosted in separate repos on GitHub, not mirrored locally):

- `KooshaPari/template-commons/.github/workflows/reusable-rust-ci.yml@main`
  - Inputs observed: `rust-version` (e.g. `stable`), `enable-coverage` (bool).
- `KooshaPari/template-commons/.github/workflows/reusable-security-scan.yml@main`
  - Inputs: `language` (`rust`/`python`/`go`/`node`); secrets: `SEMGREP_APP_TOKEN`.
- `KooshaPari/phenotypeActions/.github/workflows/validate-governance.yml@main`
  - No inputs; run as governance gate.
- Secondary reusable: `KooshaPari/phenotype-infrakit/.github/workflows/ci.yaml@main` (used by PhenoObservability via `secrets: inherit`).

Reference implementation: `PhenoPlugins/.github/workflows/ci.yml` (ci / security / phenotype-validate jobs, total 33 lines). No reusable workflow is mirrored locally — both host repos (`template-commons`, `phenotypeActions`) exist only on GitHub.

### Adoption matrix

| Repo | Tier | Classification | Delta | Notes |
|------|------|----------------|-------|-------|
| PhenoPlugins | 1 | Already adopted | — | Reference impl |
| PhenoProc | 1 | Already adopted | — | |
| PhenoVCS | 1 | Already adopted | — | |
| thegent | 1 | Already adopted | — | |
| phenoSDK | 1 | Already adopted | — | |
| Tokn | 1 | Already adopted | — | |
| Tracely | 1 | Already adopted | — | |
| hwLedger | 1 | Easy adoption | ~40→15 lines | `rust.yml` fmt/clippy/test on ubuntu-latest; swap for `reusable-rust-ci.yml` |
| AgilePlus | 1 | Needs work | N/A | Custom buf-action Proto lint + multi-job rust gates; composite (keep custom `buf` job, swap rust-check jobs) |
| heliosCLI | 1 | No CI | bootstrap | No `.github/workflows/` dir; bootstrap from PhenoPlugins template |
| PhenoKits | 1 | No CI | bootstrap | No workflows dir |
| Tracera | 1 | Needs work | N/A | Multi-language (Go+Python+Docker+Postgres service); not a simple Rust/Node swap |
| phenotype-journeys | 1 | No CI | bootstrap | No `.github/` dir at all |
| AuthKit | 2 | Easy adoption | ~60→25 lines | Rust+Python+Go; call `reusable-rust-ci` for rust, keep Python/Go blocks or use `reusable-security-scan` per-lang |
| McpKit | 2 | Easy adoption | ~60→25 lines | Mirror of AuthKit layout |
| BytePort | 2 | Easy adoption | ~80→15 lines | Standard Rust fmt/clippy/test — direct swap |
| Civis | 2 | Needs work | N/A | No `ci.yml` (gov/pages only); bootstrap |
| DataKit | 2 | No CI | bootstrap | No workflows dir |
| ResilienceKit | 2 | No CI | bootstrap | No workflows dir |
| GDK | 2 | No CI | only `legacy-tooling-gate.yml`; bootstrap | |
| AgentMCP | 3 | Easy adoption (Python) | ~100→25 lines | Pure Python uv/pytest — needs `reusable-python-ci.yml` (check if template-commons has it) |
| PhenoMCP | 3 | Easy adoption | ~10→15 lines | Trivial `cargo build` only; one-line swap |
| PhenoRuntime | 3 | Easy adoption | ~10→15 lines | Same as PhenoMCP |
| HexaKit | 3 | Needs work | N/A | Mirror of AgilePlus (buf + rust) |
| PhenoLang-actual | 3 | Needs work | N/A | 41 workflows; custom |
| Dino | 3 | Needs work | N/A | .NET / dotnet build — not covered by template-commons |
| KlipDot | 3 | Needs work | N/A | Multi-arch (ubuntu/macos/windows × stable/beta/nightly matrix); intentional cross-platform |
| heliosApp | 3 | Needs work | N/A | Bun/TS pipeline — needs `reusable-bun-ci` (check template-commons) |
| artifacts | 3 | Needs work | N/A | Uses Taskfile runner; not standard toolchain |
| phenoDesign | 3 | Easy adoption | ~30→15 lines | |
| PhenoObservability | 3 | Already adopted | — | Via `phenotype-infrakit` reusable |
| phenoEvaluation | 3 | Already adopted (different reusable) | — | Uses `phenotype-dev/.github` reusable |
| KaskMan / KDesktopVirt / kmobile | 3 | Easy adoption | ~30→15 lines | Device-automation rebuilds (per MEMORY.md); trivial Rust CI |
| AtomsBot / AppGen / AgentMCP / HeliosLab | 3 | Easy adoption | ~30→25 lines | Standard Rust or Node |
| scripts, tooling, phenotype-tooling | 3 | Easy adoption | ~20→15 lines | Small utility repos |

### Prioritized execution queue (top 10)

1. **hwLedger** (tier 1, Rust) — split `rust.yml` into `uses:` call; preserve billing-constraint comment.
2. **AuthKit** (tier 2, poly) — swap Rust job to reusable; keep Python/Go until polyglot reusable exists.
3. **McpKit** (tier 2, poly) — identical diff to AuthKit.
4. **BytePort** (tier 2, Rust) — cleanest direct swap; removes 3 duplicated jobs.
5. **PhenoMCP** (tier 3, Rust) — one-line ci.yml, trivial fit.
6. **PhenoRuntime** (tier 3, Rust) — same shape as PhenoMCP.
7. **phenoDesign** (tier 3) — small repo, clean swap.
8. **KDesktopVirt / kmobile / KaskMan** (tier 3) — trio of rebuild foundations; same diff pattern.
9. **AgilePlus** (tier 1) — partial migration: keep custom `buf` job, swap rust-check → reusable (composite PR, ~40-line delta).
10. **heliosCLI + PhenoKits + phenotype-journeys** (tier 1, no CI) — bootstrap fresh from PhenoPlugins reference; single-file adds.

### Migration template (Rust repos, "Easy adoption")

Replace existing fmt/clippy/test jobs with:

```yaml
jobs:
  ci:
    uses: KooshaPari/template-commons/.github/workflows/reusable-rust-ci.yml@main
    with:
      rust-version: stable
      enable-coverage: true
  security:
    uses: KooshaPari/template-commons/.github/workflows/reusable-security-scan.yml@main
    with:
      language: rust
    secrets:
      SEMGREP_APP_TOKEN: ${{ secrets.SEMGREP_APP_TOKEN }}
  phenotype-validate:
    uses: KooshaPari/phenotypeActions/.github/workflows/validate-governance.yml@main
```

Expected PR: 1 file changed, −40 to −80 LOC, +15 LOC; under 30 net lines in most cases.

### Blockers identified

1. **Polyglot reusables unknown**: need to confirm whether `template-commons` ships `reusable-python-ci.yml`, `reusable-go-ci.yml`, or `reusable-bun-ci.yml`. If not, AuthKit/McpKit/heliosApp/AgentMCP migrations stall or require adding them upstream first.
2. **Two parallel "canonical" reusables exist**: `template-commons` (PhenoPlugins-style) and `phenotype-infrakit/.github/workflows/ci.yaml` (PhenoObservability-style). Decide which is org-canonical before landing broad migrations.
3. **template-commons / phenotypeActions not cloned locally**: cannot inspect the full input surface without fetching them; audit based on observed `with:` usage across adopter ci.yml files only.
4. **Tier-1 gaps (heliosCLI, PhenoKits, phenotype-journeys) have no `.github/` dirs**: these need bootstrap PRs, not migration PRs — scope should be confirmed before proceeding.
5. **Tracera, AgilePlus, HexaKit, PhenoLang-actual** have extensive custom gates (buf, DB services, multi-language) that are out-of-scope for a direct swap; expect composite/hybrid migrations.


## 2026-04-24 — Compute mesh plan

See docs/governance/compute_mesh.md for the 7-node hybrid mesh design.

---

## 2026-04-24 — Workflow duplication audit + reusable consolidation plan

Audited `.github/workflows/` across 73 non-archived KooshaPari repos via `gh api`
(read-only, no clones). 615 workflow files enumerated, 257 bodies hashed for the
14 highest-frequency filenames. Goal: identify duplication follow-ups to the
`phenoShared` migration (PR #135).

### Methodology

- Listed workflows per repo: `gh api repos/KooshaPari/<r>/contents/.github/workflows`.
- For each candidate filename, fetched every body, SHA-256 hashed, grouped by hash
  to bucket byte-identical copies vs. drifted variants.
- Per workflow: LOC × (identical_count − 1) = deletable lines if migrated to a
  central reusable (`uses: KooshaPari/phenoShared/.github/workflows/<name>.yml@main`
  or similar). PR #135 already landed `ci.yml`, `coverage.yml`, `release.yml`,
  `sast.yml` in `phenoShared/.github/workflows/`, but they are *standalone*
  workflows — not yet `workflow_call` reusables. Follow-up PRs must add
  `on: workflow_call:` to `phenoShared` copies, then swap adopter repos to a
  1-line caller stub.

### Scoreboard (filename → frequency, identical-bucket, status)

| Workflow                          | Repo count | Largest identical bucket | Drift buckets | LOC (sample) | Lines deletable | Proposed action          |
|-----------------------------------|-----------:|-------------------------:|--------------:|-------------:|----------------:|--------------------------|
| `legacy-tooling-gate.yml`         | 14         | **14 (100%)**            | 0             | 102          | **1,326**       | Lift-and-shift reusable  |
| `alert-sync-issues.yml`           | 17         | **15 (88%)**             | 2             | 265          | **3,710**       | Lift-and-shift reusable  |
| `sast.yml`                        | 11         | **11 (100%)**            | 0             | 16           | 160             | Lift-and-shift reusable  |
| `coverage.yml`                    | 14         | 11 (79%)                 | 2             | 13           | 130             | Lift-and-shift reusable  |
| `self-merge-gate.yml`             | 12         | 10 (83%)                 | 2             | 9            | 81              | Lift-and-shift reusable  |
| `quality-gate.yml`                | 24         | 9 (38%)                  | 6+            | 9 (stub)     | 72              | Parameterize + reusable  |
| `tag-automation.yml`              | 12         | 8 (67%)                  | 2             | 12           | 84              | Lift-and-shift reusable  |
| `security-guard-hook-audit.yml`   | 16         | 8 (50%) + 5 near-dup     | 3             | 32           | 224             | Parameterize + reusable  |
| `release.yml`                     | 22         | 8 (36%)                  | 8+            | 57           | 399             | Parameterize + reusable  |
| `security-guard.yml`              | 26         | 7 (27%) + 6 + 4          | 8+            | 17           | 102             | Parameterize + reusable  |
| `ci.yml`                          | 39         | 5 (13%)                  | 10+           | —            | —               | **Diverged — skip**      |
| `policy-gate.yml`                 | 19         | 3 (16%)                  | 10+           | —            | —               | **Diverged — skip**      |
| `codeql.yml`                      | 13         | 3 (23%)                  | 8+            | —            | —               | **Diverged — skip**      |
| `release-drafter.yml`             | 18         | 5 (28%) + 4              | 6+            | —            | —               | Hold — low ROI           |

### Top-5 consolidation wins (ordered by lines deletable)

1. **`alert-sync-issues.yml` — 15 byte-identical copies, 265 LOC each → 3,710 lines deletable.**
   Repos: AgilePlus, agentapi-plusplus, Civis, Configra, helios-cli, heliosApp,
   HeliosLab, HexaKit, pheno, Parpoura, PhenoLang, QuadSGM, Tracera, Tokn,
   portage. This is the single biggest win — a 265-line scheduled alert-sync
   script identically replicated across 15 repos.

2. **`legacy-tooling-gate.yml` — 14/14 byte-identical, 102 LOC → 1,326 lines deletable.**
   Repos: argis-extensions, GDK, HexaKit, KDesktopVirt, phenoDesign, phenodocs,
   PhenoHandbook, PhenoPlugins, PhenoSpecs, PhenoProc, phenotype-registry,
   phenoXdd, PhenoVCS, portage. 100% identical — safest migration.

3. **`release.yml` top-bucket — 8 byte-identical, 57 LOC → 399 lines deletable.**
   Repos: Apisync, AuthKit, HeliosLab, phenoShared, phenoXdd, Stashly, Tokn,
   Tasken. `phenoShared` is already in this bucket; migrating the other 7 is a
   direct swap (reusable exists, needs `workflow_call` wrapper).

4. **`security-guard-hook-audit.yml` — 8 identical + 5 near-dup, 32 LOC → 224 lines deletable.**
   Identical repos: agent-devops-setups, Civis, heliosApp, HeliosLab, Parpoura,
   QuadSGM, Tracera, Tokn. 5 additional repos share a second variant that could
   fold in with a single parameter (hook list).

5. **`sast.yml` — 11/11 byte-identical, 16 LOC → 160 lines deletable.**
   Repos: Apisync, argis-extensions, AuthKit, nanovms, HeliosLab, phenoShared,
   phenoResearchEngine, Stashly, Tokn, portage, Tasken. Already shipped in
   phenoShared; needs `workflow_call` conversion + 1-line caller stubs.

### Rollup

- **Estimated PRs:** 10 reusable-wrapper PRs on `phenoShared` (one per workflow,
  adding `on: workflow_call:`), then **~100 adopter PRs** (one per repo×workflow;
  can be batched 3–5 workflows per repo → **~45 repo-scoped PRs** if bundled).
- **Cumulative LOC deletion across top-10 candidates:** **~6,290 lines** from
  adopter repos (pure deletion, replaced by ~5-line caller stubs → net ≈ 5,800
  LOC).
- **Biggest single win:** `alert-sync-issues.yml` alone accounts for 3,710 LOC
  (59% of total savings from this audit). Attack first.
- **Out of scope (diverged):** `ci.yml`, `policy-gate.yml`, `codeql.yml`,
  `release-drafter.yml` — buckets too fragmented; flag as separate
  rationalization task (requires canonicalizing stack policy first).

### Caveats

1. `phenoShared` currently hosts the 4 PR-#135 workflows as **plain** workflows
   (`on: push`/`pull_request`), not reusable (`on: workflow_call`). Converting
   them is prerequisite to migration — PR #135 is step 0, not step 1.
2. 15 repos returned 404 on `/contents/.github/workflows` (no workflows dir);
   excluded from counts.
3. This audit is filename-keyed. Different filenames with near-identical bodies
   (e.g., `security.yml` vs `sast.yml`) were not cross-matched; a content-hash
   pass across all 615 files is a reasonable follow-up.
4. "Parameterize" candidates (`quality-gate.yml`, `release.yml`,
   `security-guard.yml`) require per-repo `with:` inputs; exact input surface
   needs diff-inspection per cluster before a reusable schema is locked.

