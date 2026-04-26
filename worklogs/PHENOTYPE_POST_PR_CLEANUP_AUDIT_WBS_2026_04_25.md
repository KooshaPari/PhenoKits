# Phenotype Post-PR Cleanup Audit WBS - 2026-04-25

## Summary

Live GitHub PR cleanup is complete for the checked KooshaPari scope:

- `gh search prs --owner KooshaPari --state open` returned no open PRs.
- Targeted checks for previously blocked one-PR repos (`AppGen`, `KlipDot`, `kwality`) returned zero open PRs.
- The next backlog is not PR triage; it is local drift, stale no-PR branches, governance enforcement mismatch, and docs/worklog consolidation.

## Evidence Snapshot

### Local Drift

Read-only scan covered 218 local git repos:

- 186 repos showed some state drift.
- 126 dirty working trees.
- 108 branches with ahead/behind divergence.
- 49 local-only branches.

Highest-risk local drift candidates:

| Rank | Repo | Evidence | Recommended action |
|---:|---|---|---|
| 1 | `agentapi-plusplus` | `chore/infrastructure-push` ahead 7; 4,681 tracked changes | Stop and inspect before sync; likely separate vendor/dependency cleanup from governance work. |
| 2 | `phenoSDK` | `main` ahead 11; 2,200 changed paths | Review as bulk local rewrite; do not publish blindly. |
| 3 | `PhenoKits/.archive/template-commons` | `main` ahead 2; 245 changed paths | Treat as substantial archival/template rewrite. |
| 4 | shelf root `repos` | branch `adr/canonical-home-verification-2026-04-25` ahead 5; many deleted docs plus untracked repo folders | Resolve or quarantine root git state before branch reconciliation. |
| 5 | `PhenoProc` | `main` ahead 8 / behind 12; 106 changed paths | Needs rebase/merge decision after local diff review. |
| 6 | `PhenoSpecs` | `main` ahead 4; 70 changed paths | Review and publish only if intentional. |
| 7 | `hwLedger` | ahead 8 / behind 16; dirty; one local-only branch | Clean tree, then decide push/delete for local branch. |
| 8 | `.archive/pheno` | ahead 8 / behind 302 | Treat as stale; prefer fresh checkout over incremental sync. |
| 9 | `AuthKit` | ahead 9 / behind 37; dirty; 7 local-only branches | Review branches and working tree together. |
| 10 | `Dino` | ahead 8 / behind 62 | High divergence; use replacement branch strategy. |

### Stale No-PR Branches

All checked repos had zero open PRs; stale remote branch cleanup remains:

| Rank | Repo | Branch | Age / last commit | Main containment | Recommended action |
|---:|---|---|---|---|---|
| 1 | `cliproxyapi-plusplus` | `pr/211` | 139d, 2025-12-08 | not merged | Highest-confidence stale branch; recreate only if still needed. |
| 2 | `cliproxyapi-plusplus` | `pr/683` | 61d, 2026-02-23 | not merged | Stale unmerged branch; cleanup candidate. |
| 3 | `cliproxyapi-plusplus` | `pr/210` | 61d, 2026-02-23 | merged | Safe prune candidate. |
| 4 | `heliosApp` | `releases/{alpha,beta,stable}` | 29d, 2026-03-27 | merged | Keep only if intentionally long-lived release refs. |
| 5 | `heliosApp` | `chore/calver-migration-2026-03` | 27d, 2026-03-29 | not merged | Review for supersession or delete. |
| 6 | `agentapi-plusplus` | `chore/infrastructure-push` | 21d, 2026-04-04 | not merged | Current local checkout; verify before cleanup. |
| 7 | `AgilePlus` | `pr/326` / `gt/birch/1eb0ee53` | 23d, 2026-04-02 | not merged | Stale candidate after cluster cleanup. |
| 8 | `AuthKit` | `chore/add-reusable-workflows` | 19d, 2026-04-05 | not merged | Candidate if no longer active. |

### Governance Drift

The main gap is local governance docs/workflows not matching live GitHub protection:

| Repo | Local state | Live GitHub state | Gap |
|---|---|---|---|
| `AgilePlus` | Strong `.github/RULESET_BASELINE.md`, `CODEOWNERS`, policy/self-merge workflows | no protected branch / no repo rulesets | Baseline documented but not enforced. |
| `heliosApp` | explicit branch-protection docs and required-checks list | weak protection; required checks/reviews/admin/linear history not enabled | Docs and live protection diverge. |
| `thegent` | `CODEOWNERS`, dependabot, quality gate | no branch protection / no rulesets | Strong CI but no root governance contract. |
| `PhenoKits` / `HexaKit` | minimal CI-only `.github` | no branch protection / no rulesets | Missing governance scaffold. |
| `agentapi-plusplus` | `WORKFLOW.md`, `CODEOWNERS`, PR template, policy/security workflows | loose protection; no required checks, no linear history, no admin enforcement | Local governance reads stronger than remote enforcement. |
| `PhenoMCP` | CI, FR coverage, doc links, quality gate | no branch protection / no rulesets | Missing CODEOWNERS, PR template, and ruleset contract. |
| `Tracera` | richer workflow set in `Tracera-recovered` | no branch protection / no rulesets | Truth surface split between `Tracera/` and `Tracera-recovered/`. |

### Docs / Worklog Debt

Debt clusters:

- Shelf root still carries time-bound cleanup material outside a canonical session bundle.
- `worklogs/AGILEPLUS_TEST_FAILURE_TRIAGE_2026_04_25.md` and `docs/governance/org-pr-cleanup-ledger-2026-04-25.md` need a durable home or pointer strategy.
- Duplicated stale PR cleanup notes exist in both `helios-cli/` and `heliosCLI/`: `CODEX_PR_ANALYSIS.md`, `CODEX_PR_LINKS.md`, and `AUDIT_AGENTAPI_CLIPROXY_COLAB.md`.
- Root git diff already shows deleted governance/spec/status docs; unique guidance should be migrated before final deletion is accepted.
- Tracera path split (`Tracera/` points at `PhenoKits`; `Tracera-recovered/` points at `KooshaPari/Tracera`) must be resolved before docs/governance edits.

## Work Breakdown Structure

### P0 - Preserve and classify local work

1. Freeze the high-risk dirty repos before any cleanup mutation:
   - `agentapi-plusplus`
   - `phenoSDK`
   - shelf root `repos`
   - `PhenoProc`
   - `AuthKit`
   - `Dino`
2. For each repo, produce a diff manifest:
   - changed tracked files count
   - untracked file count
   - ahead/behind counts
   - local-only branch list
   - suspected source: docs wave, governance wave, vendor cleanup, dependency drift, or unknown
3. Do not push, reset, or delete until each repo has a keep/split/discard decision.

### P1 - Branch hygiene

1. Start with `cliproxyapi-plusplus` stale `pr/*` branches.
2. Delete only branches confirmed merged into `main` or explicitly superseded:
   - safe first candidate: `pr/210`
3. For unmerged stale branches (`pr/211`, `pr/683`, long 61d tail), capture branch tips and decide:
   - recreate from current `main`
   - archive branch reference in a ledger
   - delete remote branch
4. Review long-lived `heliosApp` release branches before pruning.

### P1 - Governance enforcement

1. Reconcile local governance contracts against GitHub settings for:
   - `AgilePlus`
   - `heliosApp`
   - `thegent`
   - `PhenoKits` / `HexaKit`
   - `agentapi-plusplus`
   - `PhenoMCP`
   - `Tracera`
2. Apply a minimum baseline where missing:
   - protected `main`
   - PR required
   - CODEOWNERS review where CODEOWNERS exists
   - no force push
   - no deletions
   - required checks aligned to actual workflow names
3. Add a drift check that compares local required-check/ruleset docs to live GitHub protection.

### P2 - Docs and worklog consolidation

1. Consolidate current cleanup evidence into a canonical session/report bundle.
2. Preserve root worklogs by moving unique content, not deleting it silently.
3. Retire duplicated `helios-cli` / `heliosCLI` PR research snapshots if superseded by the 2026-04-25 cleanup ledger.
4. Review deleted governance/spec docs for unique content:
   - `alert-sync-policy.md`
   - `heliosCLI-archival-cve-triage.md`
   - `shared-crates-canonical-home-adr-2026-04.md`
   - `docs/org-audit-2026-04/*.md`
5. Resolve the Tracera canonical checkout before editing Tracera governance docs.

## Acceptance Criteria

- Open PR count remains zero for the checked KooshaPari scope.
- Every high-risk dirty repo has a manifest and disposition.
- Stale no-PR branches are either pruned, archived in a ledger, or recreated from current `main`.
- Governance docs and GitHub enforcement match for the priority repos.
- Worklog/session artifacts have one durable canonical home with pointers from legacy locations.

## Assumptions

- Current PR cleanup is complete; new work should not reopen old stale PRs directly.
- Branch deletion is not automatic; stale branch cleanup needs explicit keep/delete evidence.
- Existing user or agent work in dirty repos must be preserved unless explicitly classified as stale.
- Shelf root is a coordination surface, not a product repo; per-repo truth remains authoritative.
