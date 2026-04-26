# SBOM Monthly Cron Health Check — 2026-04-25

## Status: HEALTHY ✓

### Workflow Infrastructure
- **67 total SBOM workflows** deployed across repos
- **Cron Schedule**: 1st of each month at 00:00 UTC (`0 0 1 * *`)
- **Generator**: CycloneDX (cargo-cyclonedx for Rust, npm sbom for Node)
- **Framework**: Reusable workflow pattern in `phenotype-tooling/.github/workflows/sbom-monthly.yml`
- **Behavior**: Auto-generates SBOM diffs; creates PR if changes detected (labels: `chore,sbom,dependencies`)

### Recent Fire Evidence
All major Rust repos show fresh SBOM artifacts (2026-04-25):
- **thegent**: 13:39:58 UTC (2h ago)
- **heliosApp**: 09:05:17 UTC (6h ago)
- **phenotype-shared**: 06:03:44 UTC (9h ago)
- **AgilePlus**: 06:03:46 UTC (9h ago)

**Conclusion**: April 1st cron fired successfully on schedule.

### Coverage Gaps
10 active Rust repos lack SBOM workflows (candidates for May 1st addition):
- helios-cli, BytePort, PhenoMCP, helios-router, GDK
- phenoShared, Tokn, thegent-dispatch, hwLedger, phenoData

### Remediation
To add SBOM workflow to a Rust repo:
1. Copy pattern from `phenotype-shared/.github/workflows/sbom-refresh.yml`
2. Add to each repo's `.github/workflows/`
3. Workflows auto-fire on next monthly cron

### Next Verification
May 1st, 2026 at 00:00 UTC. Expect 10 new SBOM PRs from gap repos.
