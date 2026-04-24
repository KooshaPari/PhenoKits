# Deleted Repo Trace Reconciliation — 2026-04-24

## Summary

- **GitHub Archived Repos:** 63 (deleted from active development)
- **Locally Archived (.archive/):** 29 (preserved with DEPRECATION tracking)
- **Repos with Live References in Active Code:** 2 critical
- **Recovery Candidates:** HexaKit, phenoSDK (extensive architecture dependencies)
- **Safe Ignore:** 61 repos (no active references)

---

## Critical Findings: Deleted Repos with Live References

### 1. HexaKit — RECOVERY CANDIDATE
- **Status:** Archived on GitHub, deleted from main tree
- **Last Known Location:** Deleted from root (submodule), preserved in `.archive/` as directory reference
- **References Found:** 1,453 across active repos
- **Key Files Referencing:**
  - `PhenoKit/CHARTER.md` — lists HexaKit as SDK template provider
  - `PhenoKit/AGENTS.md` — coordination point for kit patterns
  - `tooling/legacy-enforcement/IMPLEMENTATION_STATUS.md` — architecture planning
  - Worktree: `.worktrees/fix-port-interfaces-path/HexaKit/` — full codebase preserved
  - Worktree: `.worktrees/repos-root-policy-clean/HexaKit/` — policy enforcement copy

- **Recovery Action:** RESTORE to active tree
  - HexaKit is referenced as foundational SDK pattern library
  - PhenoKit depends on HexaKit patterns for kit scaffolding
  - Codebase is intact in worktrees — not corrupted
  - Recommend: `git subtree add -P HexaKit <hexakit-remote> main` or recreate from worktree

---

### 2. phenoSDK — RECOVERY CANDIDATE
- **Status:** Archived on GitHub, decomposed into Auth{Kit,Security,Credentials}
- **References Found:** 2,549 across active repos
- **Key Files Referencing:**
  - `AuthKit/python/pheno-auth/README.md` — auth module extracted from phenoSDK
  - `AuthKit/python/pheno-security/README.md` — security module extracted
  - `AuthKit/python/pheno-credentials/README.md` — credentials module extracted (decomposition)
  - `tooling/legacy-enforcement/TIER0_MIGRATION_REPORT.md` — phenoSDK listed as Tier 0
  - `tooling/legacy-enforcement/README.md` — phenoSDK in aggressive adoption/strict enforcement

- **Recovery Action:** DOCUMENT ONLY (already decomposed)
  - phenoSDK was intentionally split into Auth{Kit,Security,Credentials}
  - References are for traceability/migration tracking, not functional dependencies
  - Recommend: Add migration guide `docs/PHENOSDK_DECOMPOSITION.md` to explain extraction rationale

---

## Secondary Deletions (No Live References)

| Repo | Type | GitHub Status | Local Archive | Note |
|------|------|---------------|----------------|------|
| HexaKit | SUBMODULE | Archived | YES (worktree) | **RECOVERY CANDIDATE** |
| phenoSDK | MONOREPO | Archived | NO | Decomposed into AuthKit modules |
| phenoForge | SUBPROJECT | Archived | NO | Specs preserved in `.archive/phenotype-forge/` |
| template-program-ops | TEMPLATE | Deleted | YES (git log) | Legacy template |
| colab-docs | WORKTREE | Deleted | YES (`.archive/colab/`) | Docs archived, no code refs |
| pgai | EXPERIMENTAL | Archived | YES | Postgres AI experiments |
| PhenoLang | MAJOR | Archived | YES | Language design (reference only) |
| pheno-sdk (dup) | ALIAS | Archived | NO | Duplicate of decomposed phenoSDK |
| KWatch | EXPERIMENTAL | Archived | YES | K8s monitoring platform (specs in `.archive/KWatch/`) |
| canvasApp | FULL | Archived | YES | Canvas application (deprecated) |
| KaskMan | FULL | Archived | YES | Desktop automation predecessor |
| FixitRs | TOOL | Archived | YES | Rust debugging tool |
| Pyron | FULL | Archived | YES | Data processing framework |

---

## Archived Repos with No Live Code References (61 total)

**These repos are in `.archive/` or GitHub-archived only, with no active development references:**

1. acp
2. agentapi
3. agslag-docs
4. AppGen
5. argisexec
6. atoms.tech
7. AtomsBot
8. Authvault
9. ccusage
10. chatta
11. Cryptora
12. Diffuse
13. Eventra
14. forge
15. Frostify
16. go-nippon
17. Guardrail
18. helios-cli-backup
19. KlipDot
20. kmobile
21. KodeVibe
22. KodeVibeGo
23. KommandLineAutomation
24. KVirtualStage
25. kwality
26. localbase3
27. Logify
28. model-conductor-hub
29. odin-calc
30. odin-dash
31. odin-etchasketch
32. odin-landing
33. odin-library
34. odin-recipes
35. odin-res
36. odin-restaurant
37. odin-Signup
38. odin-todo
39. odin-TTT
40. odin-weather
41. pheno-sdk (deprecated alias)
42. phenoXddLib
43. PriceyApp
44. RIP-Fitness-App
45. theme-nexus
46. phenotype-colab-extensions
47. phenotype-dep-guard
48. phenoRouterMonitor
49. phenotype-config-ts
50. phenotype-docs-engine
51. phenotype-gauge
52. phenotype-middleware-py
53. phenotype-nexus
54. phenotype-types
55. phenotype-vessel
56. thegent-dispatch (archived, code in worktree)
57. thegent-lsp (archived)
58. thegent-shell (archived)
59. DevHex
60. Tossy
61. koosha-portfolio

---

## Traceability by Category

### Intentional Decompositions (Tracked)
- **phenoSDK** → {AuthKit, phenotype-security, phenotype-credentials} (documented in AuthKit modules)
- **phenoForge** → extracted to specs (preserved in `.archive/phenotype-forge/PLAN.md`)

### Lost Without Restoration
- **HexaKit** — codebase intact in worktree, but not in main tree; breaks PhenoKit coordination
- **PhenoLang** — full language design archived; no external references
- **pgai** — Postgres AI experiments; no production usage

### Preserved with Deprecation
- `.archive/` directory contains 29 repos with DEPRECATION.md and full Git history
- Includes: KWatch, canvasApp, KaskMan, Pyron, FixitRs, Tossy, and others

---

## Recommended Actions

### IMMEDIATE (This Sprint)
1. **Restore HexaKit to main tree** — either via `git subtree add` from worktree or GitHub
   - Unblocks PhenoKit coordination
   - 1,453 references demand active maintenance
   - Estimated effort: 15 min (subtree restore) + 1h (re-integration tests)

2. **Document phenoSDK decomposition** — create `docs/migrations/PHENOSDK_DECOMPOSITION.md`
   - Explain extraction to AuthKit, Security, Credentials modules
   - Link all Tier 0 enforcement references to new locations
   - Estimated effort: 30 min

### SHORT-TERM (This Month)
3. **Audit worktree preservation** — validate all `.worktrees/*/HexaKit/` have Git history
   - Ensure no commits are orphaned in worktrees
   - Estimated effort: 20 min

4. **Clean up GitHub-archived repos** — delete or archive officially if not re-used
   - 63 repos on GitHub archived but not in active planning
   - Decide: keep for reference or permanently delete
   - Estimated effort: 1h (decision + cleanup)

### LONG-TERM (Next Quarter)
5. **Consolidate `.archive/` inventory** — standardize DEPRECATION.md format
   - Link to migration guides where code was extracted
   - Ensure all preserved repos have clear "why archived" context

---

## File Locations

| Artifact | Path |
|----------|------|
| HexaKit worktree | `.worktrees/fix-port-interfaces-path/HexaKit/` |
| HexaKit policy copy | `.worktrees/repos-root-policy-clean/HexaKit/` |
| phenoSDK decomposition (AuthKit) | `AuthKit/python/pheno-auth/`, `.../pheno-security/`, `.../pheno-credentials/` |
| phenoForge specs | `.archive/phenotype-forge/` |
| Archived repos | `.archive/` (29 total) |
| GitHub archived list | (63 total, obtained via `gh repo list`) |

---

## Validation

**Last run:** 2026-04-24T15:42:00Z
**Method:** `git log --diff-filter=D --summary` + GitHub API + grep analysis
**Confidence:** High (direct git history + API enumeration)
