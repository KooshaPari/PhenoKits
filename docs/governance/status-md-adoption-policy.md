# STATUS.md Adoption Policy

**Status:** Proposal (2026-04-25)
**Owner:** Phenotype governance
**Template:** [`status-md-template.md`](./status-md-template.md)
**Related:** README verify-vs-claim CI proposal, user-story aggregation 2026-04-26.

---

## Summary

Every repo with README claims must ship a `STATUS.md` at the repo root, authored from the
canonical template. STATUS.md is the **source of truth** for project state; README defers
to it. CI validates README claims against STATUS.md.

## Why

- READMEs across the org over-promise (audit: `org-readme-quality-audit-2026-04-26.md`).
- Users can't tell `Experimental` from `Stable` from `Archived` in 30 seconds.
- Audit freshness decays within one parallel-dispatch loop
  (memory: `feedback_audit_freshness_decay.md`); we need a single, dated, owned surface.
- Verify-then-write principle (memory: `feedback_verify_origin_not_canonical.md`) requires
  a checked-in artifact that captures the last verification timestamp + commit.

## Mandatory Scope

STATUS.md is **mandatory** for any repo that:

1. Has a public or org-internal README that makes capability claims, OR
2. Is referenced by `projects.kooshapari.com`, OR
3. Is listed in any AgilePlus feature spec, OR
4. Has an `org-page` tag or a `<project>.kooshapari.com` landing.

**Exempt:** archived repos (state=`Archived` and confirmed inert), pure mirrors, and
fork-only worktrees with no independent claims.

## Required Sections (from template)

A STATUS.md is invalid if it omits any of:

- `State` (one of the six enum values)
- `Last Verified` (ISO date + verifier identity + commit SHA + branch)
- `Owner` (primary + backup, identity-verified)
- `What Works (Verified)` (each item linked to a test or command)
- `What Doesn't Work Yet (Honest)` (each gap linked to an issue)
- `Hello World` (commands re-run within the last 14 days)
- `Where to Ask Questions`
- `Known Issues` (link to live tracker)

## Authoring Rules

- **Identity verification:** owners must be confirmed against
  `repos/docs/governance/identity-verification.md` before listing. No anonymous owners.
- **Verify-then-write:** every "What Works" bullet must be re-run against the current
  commit before the bullet ships. No carry-over claims from prior STATUS revisions.
- **No marketing language:** `Stable` requires evidence (test pass rate, deploy history,
  consumer count). `Beta` is the default for "it works for me."
- **Honest gaps:** `What Doesn't Work Yet` cannot be empty for non-`Stable` projects.
  Empty gap list on `Alpha`/`Beta` is a review-blocking smell.

## Update Cadence

| State          | Minimum refresh              |
|----------------|------------------------------|
| Experimental   | On every PR that lands       |
| Alpha          | Weekly                       |
| Beta           | Weekly                       |
| Stable         | On every release             |
| Deprecated     | Monthly until archived       |
| Archived       | Frozen; no further updates   |

A release without a STATUS.md bump is a CI failure (see Enforcement).

## Enforcement (CI)

A `verify-vs-claim` CI job (parallel proposal) runs on every PR and:

1. Fails if STATUS.md is missing in an in-scope repo.
2. Fails if any required section is absent or empty.
3. Fails if `Last Verified` is older than the cadence allows.
4. Fails if README contains capability claims that contradict STATUS.md.
5. Fails on release tags if STATUS.md was not modified in the release commit range.

CI runs on standard Linux runners only (per
`~/.claude/CLAUDE.md` GitHub Actions billing constraint).

## Rollout

Phased, agent-driven:

| Phase | Scope                                              | Effort                |
|-------|----------------------------------------------------|-----------------------|
| 1     | Tier 1 repos (AgilePlus, thegent, heliosCLI, civ)  | 4 parallel subagents  |
| 2     | All repos with `org-page` tag                      | 8 parallel subagents  |
| 3     | All remaining in-scope repos                       | 15 parallel subagents |
| 4     | CI enforcement turned on (fail-closed)             | 1 tool call           |

Wall-clock target: phases 1–3 complete in a single session; phase 4 same day.

## Non-Goals

- Replacing AgilePlus specs (STATUS reflects state; specs drive work).
- Replacing CHANGELOG (STATUS is current state; CHANGELOG is history).
- Replacing per-feature trackers (STATUS is repo-level only).

## References

- Template: [`status-md-template.md`](./status-md-template.md)
- README audit: [`org-readme-quality-audit-2026-04-26.md`](./org-readme-quality-audit-2026-04-26.md)
- User-story aggregation: [`userstory-aggregation-2026-04-26.md`](./userstory-aggregation-2026-04-26.md)
- Identity verification memory: `feedback_verify_origin_not_canonical.md`
- Audit freshness memory: `feedback_audit_freshness_decay.md`
