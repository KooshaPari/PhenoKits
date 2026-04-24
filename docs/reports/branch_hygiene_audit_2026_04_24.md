# Branch & Worktree Hygiene Audit (2026-04-24)

## Executive Summary
- **Total repos audited:** 9 active (AgilePlus, heliosApp, PhenoObservability, PhenoPlugins, PhenoSpecs, Tracely, phenotype-infrakit, Tracera, thegent-dispatch)
- **Repos flagged (>10 branches):** 3 (AgilePlus: 43, heliosApp: 59, PhenoObservability: 16)
- **Registered git worktrees:** 69 across all repos; 19 in AgilePlus alone
- **Stale worktrees (.worktrees/ dirs):** 127 total; ~60 appear abandoned or merged

## Branch Inventory

### High-Risk Repos (>10 branches)

| Repo | Total | Merged | Unmerged | Status |
|------|-------|--------|----------|---------|
| **AgilePlus** | 43 | 6 | 33 | Accumulating unmerged feature branches |
| **heliosApp** | 59 | 6 | 13 | Dependabot (7) + feature branches stale >30d |
| **PhenoObservability** | 16 | 0 | 6 | All branches unmerged; stale >3 weeks |

### Clean Repos (<10 branches)
- PhenoPlugins (8), PhenoSpecs (5), Tracely (5), thegent-dispatch (1) — healthy

## Merged-Stale Candidates (>30 days, safe to delete locally)

### AgilePlus
- `chore/remove-codeowners` (2026-04-02) — merged, 22d old
- `fix/policy-gate-clean` (2026-04-02) — merged, 22d old
- `chore/workspace-phantom-fix` (2026-04-03) — merged, 21d old
- `chore/workspace-phantom-member-fix` (2026-04-03) — merged, 21d old

### heliosApp
- `chore/agent-readiness-governance` (2026-04-02) — merged, 22d old
- `feat/fix-ts-and-vite` (2026-03-29) — merged, 26d old
- `feat/fix-vite-federation-rebased` (2026-03-31) — merged, 24d old
- `feat/governance-and-ci-updates` (2026-03-31) — merged, 24d old
- `feat/vite-federation-fix` (2026-03-31) — merged, 24d old

### heliosApp Dependabot Branches (All stale >30d)
- 7× `dependabot/github_actions/*` and `dependabot/npm_and_yarn/*` — safe to delete, managed by Dependabot

## Unmerged-Stale Candidates (>90 days, recommend archive, DO NOT DELETE)

### AgilePlus (33 unmerged branches — requires review)
Notable branches:
- `agileplus/refactor/cli-event-flow*` (2x variants)
- `chore/deps/lodash-es`, `chore/deps/pyjwt`, `chore/deps/rust-high-sweep`
- `agileplus/chore/dashboard-extraction*` (2x variants)
- `agileplus/chore/runtime-local-deploy*` (2x variants)

Pattern observed: Multiple matching `-clean` variants suggest attempted merge/rebase cleanup iterations. Recommend review with team before deletion.

### PhenoObservability (6 unmerged branches — all stale >21d)
- `chore/add-reusable-workflows`
- `chore/phenoobs-precursor-3-missing-deps`
- `chore/phenoobs-workspace-dedupe`
- `cursor/unused-async-trait-dependency-ccd1`
- `fix/tracely-sentinel-nested-workspace`
- `fix/tracingkit-compile-errors`

All branches unmerged into main for >3 weeks; recommend team review for deprecation.

## Worktree Status

### Registered Worktrees
- **AgilePlus:** 19 registered worktrees
  - 8 in `/repos-wtrees/` (active: `agileplus-high-py`, `agileplus-high-rust`)
  - 8 in `.worktrees/` (mixed: some prunable, some active)
  - 1 in `.claude/worktrees/` (locked agent worktree)
  - 2 marked **prunable** (`chore/deps/pyjwt` and `chore/deps/rust-high-sweep`)

- **heliosApp:** Worktree list not fully captured; appears minimal

### Abandoned Worktree Candidates in `.worktrees/` (127 total)

Pattern analysis of stale directories (last modified >7 days):
- `cmdra`, `cursora`, `docuverse` (2026-04-02 ±) — 22d dormant
- Dashboard/governance/refactor variants (2026-04-02) — matching `*-clean` counterparts indicate completed/merged branches
- Docs-focused worktrees (2026-04-02) — `*-docs` variants appear static

**Observation:** 60+ worktree directories appear to be completed feature branches. Naming patterns suggest:
- `<feature>` and `<feature>-clean` pairs = branch cleanup iterations
- `<project>-docs` = documentation worktrees (often ephemeral)
- Abandoned = last modification >2 weeks AND matching commits already in parent repo main branch

## Recommendations

### Immediate Cleanup (Safe — no team review required)
1. **Delete merged-stale local branches** from AgilePlus/heliosApp:
   - 4 branches from AgilePlus (all 20+ days old, merged)
   - 5 branches from heliosApp (all 22+ days old, merged)
   - 7 Dependabot branches from heliosApp
2. **Prune registered worktrees** marked `prunable` by git (2 in AgilePlus):
   ```bash
   cd AgilePlus && git worktree prune
   ```

### Requires Review (Do NOT auto-delete)
1. **AgilePlus unmerged (33 branches):** Triage feature status with team:
   - Determine if active development or abandoned
   - Consider archiving branch list before cleanup
2. **PhenoObservability (6 unmerged):** Team review recommended before deletion
3. **Worktree `.clean` variants:** Verify merged state before removing from `.worktrees/`

### No Action Required
- Clean repos (PhenoPlugins, PhenoSpecs, Tracely, thegent-dispatch)
- Active worktrees in `/repos-wtrees/` (managed per project policy)

## Audit Metadata

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 |
| **Auditor** | Haiku Agent |
| **Scope** | 9 active repos + 127 worktree entries |
| **Method** | git branch -a, git branch --merged/--no-merged, git worktree list, filesystem scan |
| **Time to cleanup (safe only)** | ~5-10 min per repo |
