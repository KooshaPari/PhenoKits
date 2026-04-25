# PhenoLibs Coverage Audit — 2026-04-24

**Status:** DEPRECATED (2026-04-24). Remote deleted; local-only backup. Migration 90% complete.

## Language Breakdown (529 files)

| Language | LOC | Files | Status |
|----------|-----|-------|--------|
| **Python** | 77,429 | ~400 | ~30 packages; mostly orphaned |
| **Rust** | 1,458 | ~30 | 6 crates; all migrated → PhenoProc |
| **TypeScript** | 157 | 5 | 1 package; orphaned |
| **Go** | 63 | 1 | 1 stub; orphaned |
| **Total** | **79,107** | **529** | Mixed migration status |

## Build & Test Status

### Rust
- **Build:** FAIL — workspace root missing `rust-version` in `workspace.package`
- **Crates migrated:** All 6 crates (phenotype-{core, error-core, error-macros, http-client-core, iter, macros}) extracted → `PhenoProc/crates/`
- **Adoption:** phenotype-error-core widely adopted across thegent, hwLedger, HexaKit

### Python
- **Test:** FAIL — 30 packages; missing dependencies (not installed to workspace env)
- **Migration status:** ~19 packages **NOT verified as migrated** (cli-kit, config-kit, pheno-async, pheno-analytics, pheno-deployment, pheno-domain, pheno-exceptions, pheno-optimization, pheno-patterns, pheno-plugins, pheno-ports, pheno-process, pheno-providers, pheno-errors, pheno-adapters, pheno-shared, core-utils, cli-builder-kit, pheno-config)
- **Known extractions:** `pheno-core` → `KooshaPari/pheno/python/`; some kit packages → `PhenoKits/libs/python/`

### Go
- **Single stub:** `pheno-core-cgo/entity.go` (63 LOC) — orphaned, no home found

### TypeScript
- **Single package:** `packages/core` — no verified migration; may be orphaned

## Extraction History

### Successfully Extracted (post-2026-04-05)

1. **Conft** (Config management) — Extracted to standalone workspace
2. **Evalora** (Evaluation/BDD) — phenotype-bdd extracted to standalone workspace
3. **Rust crates (all 6)** → PhenoProc/crates/ (2026-04-05 onwards)

### Not Verified as Extracted (19+ Python packages)

Risk: Content may exist in `KooshaPari/PhenoKits` or `KooshaPari/pheno` but not exhaustively verified. See migration proposal at `repos/docs/governance/phenolibs-migration-proposal.md`.

## Candidate Extractions (If Repo Were Active)

**None.** All packages either:
- Extracted already (Conft, Evalora, Rust crates)
- Orphaned (19 Python packages, pheno-core-cgo, TS packages/core)
- Too small to warrant standalone workspace (<500 LOC)

Recommendation: **Do not extract further.** Consolidate orphaned packages into:
- **phenotype-shared** (Rust only, per workspace governance)
- **PhenoKits** (per-language kit consolidation)
- **pheno** (Python canonical source)

## Push Status

**Local-only. Remote deleted.**
- Remote: `https://github.com/KooshaPari/PhenoKit.git` (404 as of 2026-04-24)
- Local branch `main`: 7 commits ahead of origin/main
- Untracked: `.github/`, `docs/reference/`, worklog.md
- Dirty submodules: `python/pheno-shared`, `typescript/`

**Action:** Do NOT push. Local copy is a backup for migration archaeology only.

## Governance & Next Steps

1. **Verify Python package migrations:** Cross-check 19 packages against `PhenoKits` and `KooshaPari/pheno` for completeness
2. **Handle orphans:** Move pheno-core-cgo + TS/packages/core to appropriate archive or consolidation target
3. **Close loop:** Update DEPRECATED.md with final migration destinations once verified
4. **Decommission:** Archive local copy to S3 or Time Machine; delete local checkout after confirmation

**Related:** See `repos/docs/governance/phenolibs-migration-proposal.md` for recovery and consolidation plan.
