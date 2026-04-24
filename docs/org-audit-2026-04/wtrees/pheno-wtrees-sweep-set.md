# Pheno Sweeps: npm-sweep, py-sweep, rust-sweep

**Location:** `pheno-wtrees/{npm-sweep,py-sweep,rust-sweep}`  
**Origin:** NOT A GIT REPO (detached filesystem copies)  
**Last Commit:** N/A  
**Status:** Orphaned; no git tracking  

## Classification

**EPHEMERAL ANALYSIS ARTIFACTS** — Pre-commit analysis tooling (dependency audit, duplication scan, LOC analysis).

## Audit Scorecard (10-Dimension)

| Dimension | Score | Notes |
|-----------|-------|-------|
| **Purpose Clarity** | 9/10 | Clear: language-specific dependency/duplication sweeps |
| **Documentation** | 8/10 | README.md + CLAUDE.md present (likely generated) |
| **Code Health** | 7/10 | 1,462 source files per sweep (copies of parent workspace) |
| **Active Development** | 1/10 | No git tracking; appears to be analysis snapshot |
| **Isolation** | 9/10 | Completely isolated (not git-tracked) |
| **Test Coverage** | 0/10 | Not applicable; analysis tools, not code |
| **API Stability** | N/A | Not a library/service |
| **Dependency Health** | 8/10 | Likely generated from pheno workspace deps |
| **Deployment Readiness** | 0/10 | Not deployable; analysis artifact |
| **Maintainability** | 2/10 | Orphaned; no VCS; likely stale |

## Recommendation

**CLEAN UP**: These are analysis snapshots (npm/py/rust-sweep). They should be:
1. **Regenerated** if needed (not persisted in repo)
2. **Deleted** as they are 1.5K source files × 3 = stale cruft

If sweeps are CI/pre-commit analysis, move to `.gitignore` and regenerate on-demand.

## Size Impact

- **3 sweeps × 1,462 files × ~300KB avg** = ~1.3GB redundant storage
- Candidates for cleanup: delete all 3 sweeps, add sweep/ to .gitignore

---

**Audit Date:** 2026-04-24  
**Auditor:** Haiku 4.5 Agent  
