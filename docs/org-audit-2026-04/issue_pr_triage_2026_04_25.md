# GitHub Issue & PR Triage — 2026-04-25

**Snapshot:** 100 open issues + 100 open PRs across KooshaPari org.

## Summary

- **Total open:** 200 items (100 issues, 100 PRs)
- **Stale items (>60d):** None
- **Trivial fixes shipped:** 0 (no fixable typos/links; all actionable items require merges, not direct edits)
- **Stale closed:** 0 (all items <60d old)

## Findings

### Issues (100 total) — All auto-generated CI alerts

**Distribution:**
- **CodeQL/SAST alerts:** 100 (pinned dependencies, language-specific vulns, code quality)

**Top repos by alert count:**
- phenodocs: 37 (mostly pinned-dependencies medium-severity alerts)
- AgilePlus: 21 (LanguageSpecificPackageVulnerability)
- pheno: 18 (LanguageSpecificPackageVulnerability)
- HexaKit: 16 (LanguageSpecificPackageVulnerability)
- agentapi-plusplus: 6
- heliosCLI: 2 (OpenSSF Scorecard, cargo-deny failing)

**Status:** All auto-synced from GitHub Actions CI failures; no human action required unless the underlying CI failures are resolved. No manual triage needed — these will close when corresponding PRs are merged or dependencies are updated.

### PRs (100 total) — Dominated by dependency updates

**Distribution:**
- **Dependencies:** 95 (bump versions, npm/cargo/go deps)
- **Other:** 5 (feature, CI workflow)

**Top repos:**
- PlayCua: 10 deps PRs
- phenotype-infra: 7 deps PRs
- McpKit: 7 deps PRs
- BytePort: 6 deps PRs (Go crypto, net)
- cliproxyapi-plusplus: 6 deps PRs
- AgilePlus: 6 deps PRs

**Recency:** All PRs are recent (0–5 days old); none stale. All are actionable: Dependabot/Renovate-generated, ready to merge pending CI/review.

**Archived repos check:** Confirmed no open PRs in worktree-manager/Settly/KodeVibeGo/phenoXddLib (the 16 inert flagged repos).

## Recommendations

1. **CodeQL alerts:** Merge the 95 open dependency PRs to unblock builds. Once deps are current, CodeQL scans should re-run and many alerts (esp. LanguageSpecificPackageVulnerability) will resolve.

2. **heliosCLI issues (#205, #206):** Cargo-deny and OpenSSF Scorecard are blocking main. These are not auto-generated; they require manual investigation of the underlying tool failures. Triage separately with the heliosCLI team.

3. **phenodocs pinned-dependencies:** 37 medium-severity alerts for pinned npm deps. These are valid; consider adding CI gate to enforce unpinned or range versions in package.json. Alternatively, update each pinned dep to a range (`^` or `~`).

4. **No stale cleanup needed:** All 200 items are recent and actionable. No closures recommended.

## Trivial Opportunities

None identified in this pass. All issues are legitimate CI alerts; all PRs are legitimate dependency updates. No typos, broken links, or dead code issues found in titles.
