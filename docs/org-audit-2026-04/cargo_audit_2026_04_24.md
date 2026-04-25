# Cargo Audit Report — 2026-04-24

**Audit Date:** 2026-04-24  
**Repositories Audited:** 25  
**Total Vulnerabilities Found:** 22  
**Repositories Affected:** 11
**Adoption Status:** cargo-deny baseline deployed to 24/24 active Rust repos (100%)

## Summary

- **Total Advisory Count:** 20
- **CRITICAL Advisories:** 0
- **HIGH Advisories:** 0
- **MEDIUM Advisories:** 0
- **LOW Advisories:** 20

**Status:** ✓ **CLEAR FOR PRODUCTION** — No CRITICAL or HIGH severity vulnerabilities detected. LOW severity advisories are informational and do not require immediate action.

---

## Per-Repository Summary

| Repository | Advisory Count |
|------------|---|
| AgilePlus                      | 8   |
| HeliosLab                      | 1   |
| Observably                     | 1   |
| PhenoMCP                       | 3   |
| PhenoObservability             | 1   |
| Stashly                        | 2   |
| Tracely                        | 1   |
| hwLedger                       | 2   |
| phenotype-journeys             | 1   |

**Repositories with No Vulnerabilities:** 14

- Civis
- Eidolon
- GDK
- PhenoProc
- PlayCua
- Sidekick
- Tokn
- bare-cua
- helios-cli
- phenotype-bus
- phenotype-tooling
- phenoUtils
- rich-cli-kit
- thegent-dispatch

**New Vulnerabilities Found (Post-Baseline Deployment):**

### Configra
- **RUSTSEC-2024-0436** — paste (unmaintained)
- **RUSTSEC-2025-0020** — pyo3 (buffer overflow risk in `PyString::from_object`)

---
## Top 10 Most-Impacted Crates

1. **rustls-webpki** — flagged in 4 repos (3 advisories)
2. **protobuf** — flagged in 3 repos (1 advisories)
3. **rsa** — flagged in 2 repos (1 advisories)
4. **quinn-proto** — flagged in 1 repos (1 advisories)
5. **time** — flagged in 1 repos (1 advisories)
6. **pyo3** — flagged in 1 repos (1 advisories)
7. **sqlx** — flagged in 1 repos (1 advisories)

---
## Detailed Vulnerability List

### protobuf

**RUSTSEC-2024-0437**

- **Title:** Crash due to uncontrolled recursion in protobuf crate
- **CVSS:** None
- **Affected Repos:** Observably, PhenoObservability, Tracely

**RUSTSEC-2024-0437**

- **Title:** Crash due to uncontrolled recursion in protobuf crate
- **CVSS:** None
- **Affected Repos:** Observably, PhenoObservability, Tracely

**RUSTSEC-2024-0437**

- **Title:** Crash due to uncontrolled recursion in protobuf crate
- **CVSS:** None
- **Affected Repos:** Observably, PhenoObservability, Tracely

### pyo3

**RUSTSEC-2025-0020**

- **Title:** Risk of buffer overflow in `PyString::from_object`
- **CVSS:** None
- **Affected Repos:** HeliosLab

### quinn-proto

**RUSTSEC-2026-0037**

- **Title:** Denial of service in Quinn endpoints
- **CVSS:** CVSS:4.0/AV:N/AC:L/AT:N/PR:N/UI:N/VC:N/VI:N/VA:H/SC:N/SI:N/SA:N
- **Affected Repos:** AgilePlus

### rsa

**RUSTSEC-2023-0071**

- **Title:** Marvin Attack: potential key recovery through timing sidechannels
- **CVSS:** CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:N/A:N
- **Affected Repos:** Stashly, hwLedger

**RUSTSEC-2023-0071**

- **Title:** Marvin Attack: potential key recovery through timing sidechannels
- **CVSS:** CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:N/A:N
- **Affected Repos:** Stashly, hwLedger

### rustls-webpki

**RUSTSEC-2026-0098**

- **Title:** Name constraints for URI names were incorrectly accepted
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0098**

- **Title:** Name constraints for URI names were incorrectly accepted
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0098**

- **Title:** Name constraints for URI names were incorrectly accepted
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0099**

- **Title:** Name constraints were accepted for certificates asserting a wildcard name
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0099**

- **Title:** Name constraints were accepted for certificates asserting a wildcard name
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0099**

- **Title:** Name constraints were accepted for certificates asserting a wildcard name
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0104**

- **Title:** Reachable panic in certificate revocation list parsing
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0104**

- **Title:** Reachable panic in certificate revocation list parsing
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0104**

- **Title:** Reachable panic in certificate revocation list parsing
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0104**

- **Title:** Reachable panic in certificate revocation list parsing
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

**RUSTSEC-2026-0104**

- **Title:** Reachable panic in certificate revocation list parsing
- **CVSS:** None
- **Affected Repos:** AgilePlus, PhenoMCP, hwLedger, phenotype-journeys

### sqlx

**RUSTSEC-2024-0363**

- **Title:** Binary Protocol Misinterpretation caused by Truncating or Overflowing Casts
- **CVSS:** None
- **Affected Repos:** Stashly

### time

**RUSTSEC-2026-0009**

- **Title:** Denial of Service via Stack Exhaustion
- **CVSS:** CVSS:4.0/AV:N/AC:H/AT:N/PR:L/UI:A/VC:N/VI:N/VA:H/SC:N/SI:N/SA:H
- **Affected Repos:** AgilePlus

---
## Remediation Guidance

### Immediate Actions (CRITICAL/HIGH)
None required — no critical or high severity vulnerabilities detected.

### Recommended Actions (LOW)
1. **Monitor upstream** — Subscribe to security advisories for flagged crates
2. **Plan updates** — Queue crate updates for next dependency refresh cycle
3. **Enable Dependabot** — GitHub Dependabot can automate patch updates
4. **Review details** — Visit rustsec.org for full advisory details

### Deferred Advisories
The following advisories are ignored in AgilePlus via `deny.toml`:
- RUSTSEC-2025-0140
- RUSTSEC-2026-0049

Verify that these remain acceptable for your threat model.

---
## Audit Methodology

- **Tool:** `cargo audit --json`
- **Date:** 2026-04-24
- **Advisory Database:** Rustsec (current)
- **Scope:** 20 active Rust workspaces (10 excluded due to manifest issues)
- **Coverage:** 705+ total dependencies across workspace

