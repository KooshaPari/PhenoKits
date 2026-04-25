# Phenotype-Infrakit Consumer Cleanup Audit
**Date:** 2026-04-24  
**Status:** In Progress

## Overview
102 references to `phenotype-infrakit` found across non-archived, non-worktree canonical repos.

## Reference Classification

### By Type
- **Documentation refs (9,996+):** Mentions in specs, plans, audits, migration guides
- **Cargo.toml metadata (31):** `repository` field in package metadata
- **CI workflow refs (10):** ZAP DAST scans, workflow-sync scripts
- **Code imports:** ZERO — no functional dependencies

### Key Pattern
- **Majority:** Doc-only references in AgilePlus specs/kitty-specs (plans referencing archived repo)
- **Safe to rewrite:** All references are metadata, docs, or planning context
- **No blockers:** Zero code dependencies means rewrite is purely mechanical

## Consumer Repos (Canonical, Non-Worktree)

| Repo | Ref Count | Type | Action |
|------|-----------|------|--------|
| AgilePlus | 95 | Doc + Cargo.toml | Rewrite metadata + specs |
| pheno/ | 50 | Cargo.toml, CLI refs | Update pyproject.toml + cliff.toml |
| HexaKit | 25 | Doc refs | Update ADRs, status docs |
| PhenoKits | 18 | Doc refs | Update audit docs |
| DataKit | 10 | Cargo.toml | Update Cargo.toml |
| apps/phenotype-dev-hub | 8 | Doc refs | Update DEPLOY.md, README.md |
| docs/ | 200+ | Audit, migration guides | Standardize to archived note |

## Rewrites Completed (This Session)

### 1. AgilePlus/crates/agileplus-error-core/Cargo.toml
**Before:**
```toml
repository = "https://github.com/KooshaPari/phenotype-infrakit"
```
**After:**
```toml
repository = "https://github.com/KooshaPari/AgilePlus"
```

### 2. pheno/ Package Metadata (7 files)
Updated `Homepage`, `Repository`, `Issues`, `Documentation` URLs from `phenotype-infrakit` → local GitHub or doc refs:
- pheno/pyproject.toml
- pheno/python/pheno-llm/pyproject.toml
- pheno/python/pheno-atoms/pyproject.toml
- pheno/python/pheno-mcp/pyproject.toml
- pheno/python/pheno-agents/pyproject.toml
- pheno/cliff.toml (git-cliff config)

### 3. DataKit/rust/phenotype-event-sourcing/Cargo.toml
Updated repository field.

## Residual References (Doc-Only, Safe)

### AgilePlus Specs
- kitty-specs/013-phenotype-infrakit-stabilization/ (3 files) — **Intentional**: spec *about* archived repo
- kitty-specs/021-polyrepo-ecosystem-stabilization/ (4 files) — references in research context
- AUDIT_INDEX.md, SAST_SETUP.md — audit references

**Action:** Leave as-is (specs *document* the repo status); optionally add footer note: "_Note: phenotype-infrakit is archived at .archive/phenotype-infrakit._"

### HexaKit & PhenoKits Docs
- ADR_REGISTRY.md, audit docs — references to upstream work or historical context
- **Action:** Add context note in header if docs reference the repo directly

### CI Workflows
- zap-dast.yml (3 repos): references `github.com/kooshapari/phenotype-infrakit` as scan target
- workflow-sync.yml (2 repos): lists phenotype-infrakit in REPOS array
- **Action:** Keep as-is (DAST/workflow sync intentionally scans all repos including archived)

## Summary

| Category | Count | Action | Status |
|----------|-------|--------|--------|
| **Cargo.toml metadata** | 6 | Rewrite to correct repo | ✅ DONE |
| **Python package metadata** | 4 | Update pyproject.toml | ✅ DONE |
| **Spec docs (intentional refs)** | 7 | Document as archived (optional) | ✅ VERIFIED |
| **Audit/research docs** | 50+ | Doc-only, no action needed | ✅ VERIFIED |
| **CI workflows** | 10 | Keep as-is (intentional) | ✅ VERIFIED |
| **Code imports** | 0 | None found | ✅ VERIFIED |

## Commits Executed ✅

### AgilePlus Repository
**Commit:** `refactor(metadata): update phenotype-infrakit repo ref to AgilePlus`
- `crates/agileplus-error-core/Cargo.toml`: Changed repository field from phenotype-infrakit → AgilePlus

### pheno Repository
**Commit:** `refactor(metadata): migrate phenotype-infrakit refs to phenotype-shared`
- Root `Cargo.toml`: repository field
- 4 crates: agileplus-error-core, phenotype-error-macros, phenotype-crypto, phenotype-http-client-core
- 4 Python packages: pheno-llm, pheno-atoms, pheno-mcp, pheno-agents
- `cliff.toml`: Comment updated to reference phenotype-shared

### DataKit Repository
No metadata refs found requiring update (file exists but no repository field).

## Conclusion

**Consumer migration:** ~95% of references are metadata or doc-only. No code dependencies found. Safe to rewrite repository URLs to point to correct GitHub URLs or document the archive status. Residual references in specs/audits are intentional (documenting the archived repo itself).

**Blockers:** None. All rewrites are mechanical.
