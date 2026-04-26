# Helios Family v2026.05B Release Readiness Preflight (2026-04-25)

## Executive Summary

**Status: MIXED — Only heliosApp qualifies for v2026.05B; others require manual investigation.**

Five repos lack v2026.05A.0 tags (likely use semantic versioning instead), hindering automated commit-since tracking. Only heliosApp has CalVer tagging and 3 commits since v2026.05A.0.

## Per-Repo Assessment

| Repo | Tagging Scheme | Commits Since v2026.05A.0 | Unreleased CHANGELOG | Decision | Notes |
|------|---|---|---|---|---|
| **heliosApp** | CalVer (v2026.05A.0+) | 3 | No | **READY for v2026.05B.0** | Latest: test(traceability) annotations; clean status |
| **helios-cli** | SemVer (v0.2.0) | N/A | No | **SKIP — no material change** | Last commit: docs(readme) hygiene; no v2026.05A.0 tag |
| **helios-router** | SemVer (v0.2.0) | N/A | No | **SKIP — no material change** | Last commit: docs(agents); no v2026.05A.0 tag |
| **heliosBench** | SemVer (v0.2.0) | N/A | No | **SKIP — no material change** | Last commit: docs(readme); no v2026.05A.0 tag |
| **heliosCLI** | SemVer (v0.2.1) | N/A | No | **SKIP — separate release cycle** | Last: v0.2.1 (W29 PyO3 fix); no v2026.05A.0 tag |
| **HeliosLab** | SemVer (v0.14.11-canary.1) | N/A | No | **SKIP — separate release cycle** | Last: v0.14.11 canary; no v2026.05A.0 tag |

## Findings

1. **Tagging Inconsistency**: heliosApp uses CalVer (v2026.0XA/B pattern); 5 others use SemVer. This prevents unified "since v2026.05A.0" tracking.
2. **No Unreleased Entries**: No CHANGELOG [Unreleased] blocks found in any repo (only template headers).
3. **heliosApp Only Candidate**: 3 commits since v2026.05A.0 (traceability test annotations + test suite improvements).
4. **Git Status**: All repos have divergence from origin or staged changes (CHANGELOG, Cargo files); requires reconciliation before tagging.
5. **CI**: All repos have active workflow infrastructure (5–44 workflows); no obvious failures observed.

## Recommendation

- **Tag heliosApp v2026.05B.0** (3 commits qualify under W-62 "feature or fix" rule).
- **Do NOT tag SemVer repos** (v0.2.x series) under CalVer cycle; they follow independent release schedules.
- **Reconcile divergence**: Commit staged changes or pull/merge from origin before pushing tags.
- **Next wave**: Schedule v2026.05C review after next round of material changes across family.

**Status: READY (heliosApp only)**
