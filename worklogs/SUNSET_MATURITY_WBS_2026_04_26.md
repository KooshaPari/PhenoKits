# Sunset Maturity WBS - 2026-04-26

## Goal

Move the Phenotype repo ecosystem from "PR queue is clear" to a mature closeout state where
each repo has a known state, local work is preserved or retired intentionally, shared sources
have canonical homes, and archived repos stay quiet.

## Current Backlog

| Priority | Workstream | Lead repos | Done when |
|---|---|---|---|
| P0 | Local drift preservation | `agentapi-plusplus`, `phenoSDK`, `PhenoProc`, `Dino`, `PhenoSpecs`, shelf root, `AuthKit/go` | each has split/discard/publish disposition |
| P0 | Shared-crate canonical homes | `phenoShared`, `pheno`, `PhenoLang`, `HexaKit`, `PhenoProc`, Kit repos | every shared crate has one canonical source |
| P1 | Governance ruleset reconciliation | `AgilePlus`, `heliosApp`, `thegent`, `agentapi-plusplus`, `Tracera`, `phenoShared`, rule-less sampled repos | live rulesets match local docs |
| P1 | Stale branch hygiene | `cliproxyapi-plusplus`, `heliosApp`, `AgilePlus`, `AuthKit` | no stale no-PR branch lacks a ledger entry |
| P2 | Docs/worklog consolidation | shelf `docs/governance`, `worklogs`, repo-local docs | every claim has one durable source |
| P3 | Archive/sunset enforcement | archived repos and low-activity candidates | archived repos have no PRs/dependabot and successor notes |

## Batch 2 Evidence

- Added `docs/governance/sunset-maturity-batch-2-execution-plan-2026-04-26.md`.
- Added `worklogs/P0_DRIFT_DISPOSITION_2026_04_26.md`.
- Added `worklogs/WORKTREE_QUARANTINE_LEDGER_2026_04_26.md`.
- Verified open PR queue is empty after closing/merging the final PhenoKits PRs.
- Confirmed 44 worktree/recovery containers and 142 nested `.git` directories within
  depth 3; no deletion is authorized until each container has a row-level disposition.
- Confirmed priority ruleset gaps for `PhenoMCP`, `PhenoProc`, `PhenoKits`, `Dino`,
  `PhenoSpecs`, and `AuthKit` in the sampled GitHub ruleset API pass.

## Batch 3 Evidence

- Added `docs/governance/sunset-maturity-batch-3-execution-log-2026-04-26.md`.
- Tightened `phenoShared` live ruleset to require one approval and review-thread
  resolution.
- Left `phenoShared` CODEOWNER review disabled because the current CODEOWNERS state
  would self-block the only maintainer.
- Verified a clean PhenoKits clone at `/tmp/phenokits-fresh.TDBzua/PhenoKits`;
  continue governance PRs from that clone until the shelf checkout is repaired.
- Blocked Rust `phenotype-errors` release prep because `crates/phenotype-errors` is
  absent in current `phenoShared`; decide Rust crate creation vs JS `packages/errors`
  release prep before implementation.

## Batch 4 Decision

- Added `docs/governance/adr-2026-04-26-errors-dual-interface-strategy.md`.
- Chose a dual-interface path:
  - Rust source of truth: `phenotype-error-core`.
  - TypeScript interface: `packages/errors` / `@phenotype/errors`.
  - Optional Rust facade: `crates/phenotype-errors` only if a real Rust consumer
    requires that crate name.
  - Shared parity mechanism: explicit schema/fixtures consumed by both Rust and TS
    tests.
- This avoids inventing a missing Rust crate from stale docs while preserving the
  existing TS package as a first-class interface.
- Added `docs/governance/errors-dual-interface-target-state-2026-04-26.md` to pin
  the file map, public API shape, fixture strategy, and migration order.

## Milestones

### M1 - Freeze and Manifest P0 Drift

- Generate per-repo manifests for each P0 repo:
  - branch and tracking status
  - ahead/behind
  - tracked/untracked counts
  - nested gitlink state
  - commit themes
  - keep/split/discard recommendation
- No mutation except creating ledger docs.
- Exit: all P0 repos have written disposition recommendations.
- Status 2026-04-26: initial disposition ledger created from live `git status`
  evidence because the older `PHENOTYPE_LOCAL_DRIFT_MANIFEST_2026_04_25.md`
  was not present in this checkout; per-repo branch-tip manifests still pending.

### M2 - Split Salvage Branches

- Create one current-main branch per coherent work lane.
- Replay only coherent commits/files.
- Do not mix docs/governance, generated cleanup, vendor deletion, and source behavior.
- Exit: each P0 repo has either clean salvage PRs or an explicit discard/quarantine decision.

### M3 - Canonical Shared Source

- Import Phase 1 crates into `phenoShared` from `pheno`.
- Decide canonical home for duplicated Kit crates.
- Convert brittle sibling path deps to versioned git/tag or registry deps.
- Exit: shared-crate audit table has no unlabeled drift mirrors.

### M4 - Governance Baseline

- Inventory live rulesets for all active repos.
- Apply baseline only after CI truth is known.
- Update local rule docs and CODEOWNERS/PR templates in the same lane.
- Exit: priority active repos have matching live rulesets and local governance docs.

### M5 - Branch and Archive Hygiene

- Inventory remote branches with no open PR.
- Delete only merged/superseded branches with tip SHA captured.
- For archived repos, remove Dependabot config before archive.
- Exit: no open PRs, no archived Dependabot emitters, no stale no-PR branch older than 30 days without decision.

## Execution Order

1. P0 drift manifests.
2. Shared-source canonical import plan.
3. Governance ruleset inventory.
4. Salvage branch creation.
5. Branch pruning.
6. Archive/sunset closeout.
7. Final worklog consolidation.

## Acceptance Tests

- `gh search prs --owner KooshaPari --state open --limit 200` returns no open PRs.
- Archived repo sample has no `.github/dependabot.yml`.
- Priority active repos have ruleset inventory saved.
- P0 repos have no unclassified mixed local work.
- Worklogs index links to this WBS and the sunset maturity audit.
