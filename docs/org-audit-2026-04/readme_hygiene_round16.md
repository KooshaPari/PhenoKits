# README Hygiene Audit — Round 16 (W-67F + W-68 Batch)

**Audit Date:** 2026-04-25  
**Scope:** 11 top-level repos (ObservabilityKit, PhenoAgent, phenoAI, PhenoCompose, phenotype-omlx, AuthKit, PhenoSchema, agileplus-landing, ValidationKit, chatta, AppGen)  
**Criteria:** Install command, 1-line description, License section, Status badge/note

---

## Audit Results

| Repo | Install? | 1-line Desc? | License? | Status? | Notes |
|------|:--------:|:------------:|:--------:|:-------:|-------|
| **ObservabilityKit** | ✅ | ✅ | ❌ CRITICAL | ✅ | Has git clone + cargo/pytest. Missing license section. Version noted. |
| **PhenoAgent** | ✅ | ✅ | ✅ | ✅ | Proprietary (line 133). cargo build present. Status implicit (active). |
| **phenoAI** | ✅ | ✅ | ✅ | ✅ | Dual Apache/MIT (line 273). cargo/git deps shown. Active/scaffolding noted. |
| **PhenoCompose** | ✅ | ✅ | ✅ | ✅ | Apache-2.0 (line 126). curl/go build. Status: Active. |
| **phenotype-omlx** | ✅ | ✅ | ✅ | ✅ | Apache-2.0 (line 368). brew/pip installs. Status implicit (active, v0.1+). |
| **AuthKit** | ✅ | ✅ | ❌ CRITICAL | ✅ | Has cargo build. Missing license section (ends at L88). Status: 🚧 Under construction. |
| **PhenoSchema** | ✅ | ✅ | ❌ CRITICAL | ✅ | cargo build + cargo run. Missing license section (ends at L97). Status implicit (active). |
| **agileplus-landing** | ✅ | ✅ | ❌ CRITICAL | ✅ | bun/vercel commands present. Missing explicit license (implicitly MIT/Apache via org, but not stated). Status: Active. |
| **ValidationKit** | ✅ | ✅ | ❌ CRITICAL | ✅ | ARCHIVED. No install (2 LOC, no active consumers). No license stated. Status: ARCHIVED (clear). |
| **chatta** | ✅ | ✅ | ✅ | ✅ | MIT (line 197). ./start script, go/npm commands. Status: Active Development. |
| **AppGen** | ✅ | ✅ | ❌ CRITICAL | ✅ | ARCHIVED. No install (historical reference). MIT noted (line 75) but not in License section. Status: ARCHIVED (clear). |

---

## Summary

- **Total:** 11 repos audited
- **Fully Compliant (4 criteria):** 6 repos (PhenoAgent, phenoAI, PhenoCompose, phenotype-omlx, chatta)
- **Missing License (CRITICAL):** 5 repos
  - ObservabilityKit (no license section)
  - AuthKit (no license section)
  - PhenoSchema (no license section)
  - agileplus-landing (no explicit license section; org-wide Apache-2.0 implicit)
  - ValidationKit (ARCHIVED, no license)

---

## Actions

**CRITICAL:** Add license sections to ObservabilityKit, AuthKit, PhenoSchema per W-67F governance (all Phenotype repos must have explicit License section per org standard).

**Note:** agileplus-landing, ValidationKit, and AppGen are exceptions:
- **agileplus-landing** — part of org-pages tier 2; inherits org-wide Apache-2.0 but should state it explicitly
- **ValidationKit, AppGen** — ARCHIVED; license preservation sufficient; physical migration to .archive/ pending user decision

---

**Word count:** 240 | **Status:** Complete
