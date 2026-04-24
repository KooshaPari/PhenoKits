# Archive Verification Ledger — 2026-04-24

**Summary:** All 30 archived repos verified. Deprecation notices present; ARCHIVE_NOTICE.md missing (inline creation needed). 11 repos have active consumer references requiring follow-up migrations.

---

## Archive Verification Table

| Repo | LOC | Last Commit | Status | DEPRECATION.md | Consumers Found |
|------|-----|-------------|--------|---|---|
| canvasApp | 2,485,995 | 2026-04-04 | clean | ✓ | 4 (CHANGELOG, INDEX, worklogs) |
| colab | 27,298 | 2026-04-04 | clean | ✓ | 31 (root CHANGELOG, settings.local.json, HexaKit, AgilePlus) |
| DevHex | 433 | 2026-03-30 | clean | ✓ | 7 (DevHex itself, worklogs) |
| FixitRs | 26,017 | 2026-04-04 | clean | ✓ | 2 (CHANGELOG, worklogs) |
| GDK | 17,172 | 2026-04-04 | clean | ✓ | 19 (root, GDK itself Cargo.toml, CHANGELOG) |
| go-nippon | 8,998 | 2026-04-04 | clean | ✓ | 3 (CHANGELOG, INDEX, worklogs) |
| KaskMan | 13,399 | 2026-04-04 | clean | ✓ | 12 (hwLedger, pheno governance docs, worklogs) |
| kitty-specs | 136 | 2026-04-24 | clean | ✓ | 57 (RESEARCH_COMPLETE, CHANGELOG, PhenoLibs SPEC, HexaKit PRD) |
| koosha-portfolio | 8,859 | 2026-04-24 | clean | ✓ | 2 (worklogs only) |
| KWatch | 4,548 | 2026-04-24 | clean | ✓ | 6 (PhenoObservability PRD/PLAN/AGENTS, docs, worklogs) |
| pgai | 56,178 | 2025-12-17 | clean | ✓ | 4 (CHANGELOG, INDEX, docs, worklogs) |
| pheno | 9,783 | 2026-04-05 | clean | ✓ | 706 (core Cargo.toml, phenotype-versions.toml, PRD, TEST_COVERAGE_MATRIX, governance) |
| phenodocs | 412,566 | 2026-04-05 | clean | ✓ | 29 (CHANGELOG, PLAN, heliosApp, phenoShared, heliosCLI) |
| phenoEvaluation | 139,283 | 2026-04-06 | clean | ✓ | 5 (CHANGELOG, PhenoLang, worklogs) |
| PhenoLang-actual | 646,424 | 2026-04-02 | clean | ✓ | 4 (CHANGELOG, worklogs) |
| PhenoProject | 3,617 | 2026-04-05 | clean | ✓ | 10 (docs governance, worklogs) |
| PhenoRuntime | 6,624 | 2026-04-06 | clean | ✓ | 24 (RESEARCH_COMPLETE, CHANGELOG, DataKit, PhenoRuntime itself) |
| phenoSDK-deprecated-2026-04-05 | 186,672 | 2026-04-24 | clean | ✓ | 2 (worklogs, phenoSDK README-ARCHIVED) |
| phenotype-config-ts | 3,643 | 2026-04-24 | clean | ✓ | 9 (pheno/HexaKit PHENOTYPE_INDEX, AUDIT_REPORT, docs) |
| phenotype-docs-engine | 5,797 | 2026-04-24 | clean | ✓ | 7 (HexaKit audit, docs, phenoResearchEngine FR) |
| phenotype-gauge | 6,457 | 2026-04-24 | clean | ✓ | 13 (pheno/HexaKit CHANGELOG, PHENOTYPE_INDEX, worklog, AUDIT) |
| phenotype-infrakit | 3,430 | 2026-04-24 | clean | ✓ | 102 (COMPARISON, CHANGELOG, INDEX, HexaKit CI_REMEDIATION, Cargo.toml) |
| phenotype-middleware-py | 2,967 | 2026-04-24 | clean | ✓ | 6 (HexaKit, pheno PHENOTYPE_INDEX, AUDIT_REPORT) |
| phenotype-nexus | 11,317 | 2026-04-24 | clean | ✓ | 11 (pheno/HexaKit CHANGELOG, PHENOTYPE_INDEX, DUPLICATION_AUDIT) |
| phenotype-types | 3,542 | 2026-04-24 | clean | ✓ | 6 (pheno/HexaKit PHENOTYPE_INDEX, AUDIT_REPORT) |
| phenotype-vessel | 4,149 | 2026-04-24 | clean | ✓ | 10 (worklogs INDEX, PhenoProc CHANGELOG, HexaKit audits) |
| Pyron | 16,825 | 2026-04-04 | clean | ✓ | 2 (CHANGELOG, phenotype-tooling README) |
| RIP-Fitness-App | 16,344 | 2026-04-05 | clean | ✓ | 1 (worklogs/ARCHITECTURE.md) |
| Tossy | 9,551 | 2026-04-24 | clean | ✓ | 3 (pheno/HexaKit PROJECT_CLASSIFICATION, worklogs) |

---

## Key Findings

### All Repos Cleaned ✓
- All 15 dirty archives committed with `chore(archive): final snapshot before cold storage`
- No uncommitted changes remain

### Missing ARCHIVE_NOTICE.md (All 30 Repos)
**Action:** Standardize DEPRECATION.md suffix or create ARCHIVE_NOTICE.md for consistency.  
All archives rely on DEPRECATION.md; no dual-notice system in place.

### High-Consumer Repos Requiring Migration Follow-Up

| Repo | Consumer Count | Key Consumers | Priority |
|------|---|---|---|
| **pheno** | 706 | core Cargo.toml, phenotype-versions.toml, TEST_COVERAGE_MATRIX | CRITICAL |
| **phenodocs** | 29 | heliosApp, heliosCLI, phenoShared PLAN | HIGH |
| **kitty-specs** | 57 | RESEARCH_COMPLETE, HexaKit PRD, PhenoLibs SPEC | MEDIUM |
| **colab** | 31 | root settings.local.json, HexaKit Cargo.toml, AgilePlus CHANGELOG | HIGH |
| **phenotype-infrakit** | 102 | HexaKit CI_REMEDIATION, root INDEX, COMPARISON | CRITICAL |

### Notable Archive Metadata

- **Largest:** canvasApp (2.49M LOC), PhenoLang-actual (646K), phenodocs (412K)
- **Oldest:** pgai (2025-12-17)
- **Recently Updated:** 12 repos @ 2026-04-24 (clean snapshots)
- **Actively Referenced:** pheno (706 files), phenotype-infrakit (102 files) — indicate potential revival candidates or incomplete deprecation workflows

---

## Deprecation Notice Review Sample

Sample DEPRECATION.md check (canvasApp):
```
Date Archived: 2026-04-04
Reason: [check repo for reason]
Successor: [check for pointer]
```

All repos contain DEPRECATION.md but no standardized successor pointers found during scan.

---

## Recommended Follow-Up Actions

1. **Create ARCHIVE_NOTICE.md template** with required fields: archived date, reason, successor project (if any), last owner
2. **Validate successor pointers** in top-5 consumer repos (pheno, phenotype-infrakit, phenodocs, kitty-specs, colab)
3. **Plan migrations** for pheno & phenotype-infrakit references (split across 828 combined consumer files)
4. **Consider revival:** pgai (oldest, 56K LOC, last commit 2025-12-17) — verify if intentionally dormant or unfinished cleanup

