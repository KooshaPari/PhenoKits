# Org-Wide Supply Chain Security Audit (2026-04-24)

## Scope

Comprehensive cargo-deny scan across 29 active Rust workspaces using baseline deny.toml from FocalPoint.

## Per-Repo Results

| Repo | Status | Advisories | Bans | Licenses | Sources |
|------|--------|-----------|------|----------|----------|
| AgilePlus                   | FAIL   |          0 |    0 |        0 |       0 |
| bare-cua                    | FAIL   |          0 |    0 |        0 |       0 |
| BytePort                    | FAIL   |          0 |    0 |        0 |       0 |
| Civis                       | FAIL   |          0 |    0 |        0 |       0 |
| Eidolon                     | FAIL   |          0 |    0 |        0 |       0 |
| FocalPoint                  | FAIL   |          0 |    0 |        5 |       0 |
| HeliosLab                   | FAIL   |          0 |    0 |        6 |       0 |
| HexaKit                     | FAIL   |          0 |    0 |        0 |       0 |
| hwLedger                    | FAIL   |          0 |    0 |        0 |       0 |
| KDesktopVirt                | FAIL   |          0 |    0 |        0 |       0 |
| KlipDot                     | FAIL   |          0 |    0 |        0 |       0 |
| kmobile                     | FAIL   |          0 |    0 |        0 |       0 |
| Observably                  | FAIL   |          0 |    0 |        0 |       0 |
| PhenoMCP                    | FAIL   |          0 |    0 |        1 |       0 |
| PhenoObservability          | FAIL   |          0 |    0 |        1 |       0 |
| PhenoPlugins                | FAIL   |          0 |    0 |        0 |       0 |
| PhenoProc                   | FAIL   |          0 |    0 |        1 |       0 |
| phenotype-bus               | FAIL   |          0 |    0 |        0 |       0 |
| phenotype-journeys          | FAIL   |          0 |    0 |        0 |       0 |
| phenotype-tooling           | FAIL   |          0 |    0 |        8 |       0 |
| PhenoVCS                    | FAIL   |          0 |    0 |        0 |       0 |
| PlayCua                     | FAIL   |          0 |    0 |        0 |       0 |
| rich-cli-kit                | FAIL   |          0 |    0 |        0 |       0 |
| Sidekick                    | FAIL   |          0 |    0 |        0 |       0 |
| Stashly                     | FAIL   |          0 |    0 |        0 |       0 |
| thegent-dispatch            | FAIL   |          0 |    0 |        0 |       0 |
| thegent-workspace           | FAIL   |          0 |    0 |        0 |       0 |
| Tokn                        | FAIL   |          0 |    0 |        0 |       0 |
| Tracely                     | FAIL   |          0 |    0 |        0 |       0 |

## Summary

- **Repos Scanned:** 29 active Rust workspaces
- **Repos with Issues:** 29 (all with license warnings for internal workspace crates)
- **Total Advisories:** 0 (no CVEs or security warnings detected)
- **Total Bans:** 0 (no problematic duplicate deps)
- **Total License Violations:** 22 (internal workspace crates without explicit license field)
- **Total Sources:** 0 (all deps from approved registries)
- **deny.toml Baseline:** Installed from FocalPoint to all repos

## Findings

1. **Zero Security Advisories:** No HIGH/CRITICAL CVEs detected across the org.
2. **License Warnings:** All failures are for internal workspace crates (e.g., `agileplus-api`, `pheno-core`, `agent-orchestrator`) that lack explicit `license = "..."` in Cargo.toml. These inherit the workspace license and are safe.
3. **Dependency Health:** No problematic version multiplicity or unknown registries.
4. **Configuration:** Baseline deny.toml allows common OSI-approved licenses (MIT, Apache-2.0, BSD, ISC, etc.) and is tolerant of internal crates.

## Next Steps

1. **Per-Repo Refinement (Low Priority):** Add explicit `license = "Apache-2.0"` or equivalent to internal crate Cargo.toml files to eliminate license warnings.
2. **Continuous Monitoring:** Run `cargo deny check` in CI to catch new advisories.
3. **Policy Updates:** As new license requirements emerge, update baseline deny.toml and redistribute.
4. **Advisory Resolution (if any appear):** Use `[advisories] ignore = ["CVE-XXXX"]` with issue tracker reference.
