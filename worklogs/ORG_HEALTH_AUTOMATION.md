# Org Health Automation — Setup Complete

**Date:** 2026-04-24  
**Category:** GOVERNANCE  
**Status:** COMPLETE  
**Scope:** Automated org-wide metrics monitoring across 59+ repositories

## Summary

Implemented automated weekly and monthly org-health monitoring with GitHub Actions workflows to track org-wide audit metrics, worklog coverage, and LOC drift.

## What Was Built

### 1. Weekly Org Audit Workflow
**File:** `.github/workflows/org-audit-weekly.yml`

- **Trigger:** Monday 9am Eastern (14:00 UTC), or manual dispatch
- **Jobs:**
  - Build and run `worklog-aggregator` (scans all repos for worklog entries)
  - Build and run `org-audit-aggregator` (if available)
  - Auto-commit results to main with `[skip ci]` flag
- **Outputs:**
  - `docs/org-audit-YYYY-MM/worklog_index.md` — consolidated worklog index
  - `docs/org-audit-YYYY-MM/INDEX.md` — org-wide audit status matrix
- **Billing:** `continue-on-error: true` to respect GitHub Actions billing constraints

### 2. Monthly LOC Drift Workflow
**File:** `.github/workflows/loc-drift-monthly.yml`

- **Trigger:** 1st of each month, 10am Eastern (14:00 UTC), or manual dispatch
- **Jobs:**
  - Install cloc
  - Generate LOC statistics excluding build artifacts, node_modules, etc.
  - Auto-commit report to main
- **Output:** `docs/org-audit-YYYY-MM/loc_drift_YYYY_MM.md`
- **Excludes:** .git, node_modules, .worktrees, target, build, dist, .cargo, .archive

### 3. Automation Documentation
**File:** `docs/governance/org_health_automation.md`

- Automation schedule and trigger times (tables)
- Manual execution instructions for all aggregators
- Metrics tracked per workflow
- Failure handling and recovery procedures
- Output structure and file organization

## Integration Notes

- **Worklog Aggregator:** Already built and available at `tooling/worklog-aggregator/target/release/worklog-aggregator`
- **Org Audit Aggregator:** Conditionally built; workflow logs will note if missing
- **Billing Strategy:** Both workflows fail gracefully per existing GitHub Actions billing policy
- **Commit Strategy:** `[skip ci]` flag prevents recursive CI runs on auto-committed audit results

## Cadence

| Job | Frequency | Day/Time | Output File |
|-----|-----------|----------|------------|
| Worklog + Org Audit | Weekly | Monday 9am ET | `worklog_index.md`, `INDEX.md` |
| LOC Drift | Monthly | 1st of month 10am ET | `loc_drift_YYYY_MM.md` |

## Manual Trigger

Either workflow can be run manually via GitHub UI:
- **Actions → Org Audit Weekly → Run workflow → Run workflow**
- **Actions → LOC Drift Monthly → Run workflow → Run workflow**

Or via CLI:
```bash
gh workflow run org-audit-weekly.yml
gh workflow run loc-drift-monthly.yml
```

## Policy Compliance

- Respects GitHub Actions billing constraints (continue-on-error)
- Uses standard Rust toolchain for aggregator builds
- Cloc for LOC analysis (no custom counting logic)
- UTF-8 markdown output
- [skip ci] flag to prevent CI billing on result commits

## See Also

- **Org Audit History:** `docs/org-audit-YYYY-MM/` — audit results by month
- **Worklog Categories:** `/repos/worklogs/README.md` — worklog structure
- **AgilePlus Specs:** `/repos/AgilePlus/kitty-specs/` — feature tracking
