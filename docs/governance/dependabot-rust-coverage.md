# Dependabot Rust (cargo) Coverage Audit & Remediation

**Date:** 2026-04-25
**Triggered by:** org-wide cargo audit surfaced 26 RUSTSEC findings across 6 auditable repos while Dependabot showed 0 cargo-ecosystem alerts org-wide. Investigation confirmed Dependabot **is** functioning where lockfiles are committed; the alert-coverage gap is driven by missing `Cargo.lock` files and two missing `.github/dependabot.yml` configs — not an org-level misconfiguration.

## Diagnostic Table

| Repo | dependabot.yml | cargo entry | Open Alerts | Cargo.lock | Action |
|------|----------------|-------------|-------------|------------|--------|
| AgilePlus | yes | yes | 23 | committed | none — working |
| HexaKit | yes | yes | 30 | committed | none — working |
| pheno | yes | yes | 30 | committed | none — working |
| PhenoLang | yes | yes | 30 | committed | none — working |
| BytePort | yes | yes | 18 | missing | commit `Cargo.lock` to surface bin/CLI alerts |
| thegent | yes | yes | 30 | missing (Go monorepo) | review per-Rust-crate lockfile policy |
| hwLedger | yes | yes | 0 | missing | commit `Cargo.lock` (workspace + tauri) |
| PhenoKits | yes | yes | 1 | missing | commit `Cargo.lock` |
| PhenoObservability | **NO** | n/a | 0 | missing | **PR: add dependabot.yml + commit lockfile** |
| phenoShared | yes | yes | 0 | missing | commit `Cargo.lock` |
| phenotype-tooling | yes | yes | 0 | committed | none — likely clean tree |
| helios-cli | yes | yes | alerts disabled | missing | enable Dependabot alerts on repo settings |
| heliosCLI | **NO** | n/a | 30 | committed | **PR: add dependabot.yml** (alerts already flowing via push scan) |
| AuthKit | yes | yes | 6 | missing | commit `Cargo.lock` |

## Root-Cause Findings

1. **Org-wide Dependabot is NOT broken.** 11 of 14 Rust repos have a working cargo entry; 9 of those have surfaced alerts.
2. **Cargo.lock as gating factor.** Repos missing `Cargo.lock` (hwLedger, PhenoKits, phenoShared, AuthKit, BytePort partial) cannot be audited by Dependabot beyond direct-dependency manifest scanning. Lockfile commit is the canonical fix.
3. **Two repos lack any dependabot.yml.** PhenoObservability (zero coverage) and heliosCLI (alerts coming from push scan but no config to drive PRs).
4. **helios-cli has Dependabot alerts disabled at repo level.** Requires admin toggle in repo settings, not a config change.

## Remediation

### Phase 1 — Add missing dependabot.yml (this audit)
- PRs to: PhenoObservability, heliosCLI
- Canonical config:
  ```yaml
  version: 2
  updates:
    - package-ecosystem: cargo
      directory: "/"
      schedule:
        interval: weekly
        day: monday
      groups:
        minor-and-patch:
          update-types: [minor, patch]
      open-pull-requests-limit: 10
  ```

### Phase 2 — Commit `Cargo.lock` to library repos (separate work)
Standard Rust guidance is to commit `Cargo.lock` for binaries; for libraries the tradeoff is noisier diffs vs. reproducible CI + Dependabot coverage. Given Phenotype's CI-completeness policy and security posture, **commit `Cargo.lock` for all repos** including libraries:
- hwLedger, PhenoKits, phenoShared, AuthKit, BytePort, PhenoObservability

### Phase 3 — Enable alerts on helios-cli
- `gh api repos/KooshaPari/helios-cli/vulnerability-alerts -X PUT`
- Or via repo Settings → Code security → enable Dependabot alerts.

## Provenance
- Script-driven survey via `gh api` against all 14 KooshaPari Rust repos.
- Cross-referenced with `org-cargo-audit-2026-04-25.md`.
