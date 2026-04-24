# Worktree-Style Embedded Repos Audit Summary

**Audit Date:** 2026-04-24  
**Scope:** 13 `-wtrees/` directories across repos root  
**Total Subdirs Audited:** 35+ embedded repos  

## Executive Summary

The `-wtrees/` directories contain a mix of:
- **Active feature branches** (dependabot, CI fixes) — Ready to merge
- **Orphaned branches** (Docker API migration, old feature work) — Cleanup candidates
- **Ephemeral artifacts** (sweeps, analysis snapshots) — Delete & gitignore

### Key Findings

| Category | Count | Action |
|----------|-------|--------|
| **Merged Branches** (remote exists) | 8 | Merge & close |
| **Orphaned Branches** (local only) | 6 | Review & delete or complete |
| **Ephemeral Artifacts** (non-git) | 15+ | Delete; add to .gitignore |
| **UNKNOWN** (detached/unclear) | 6 | Manual review required |

---

## Distinct Products Found

### NONE

All `-wtrees/` directories are **variants of their parent repos**. No standalone products detected.

---

## Merged Branches Flagged for Cleanup

### Primary Cleanup Targets

| Branch | Parent | Status | Action |
|--------|--------|--------|--------|
| `fix/dependabot-agileplus-pyjwt-lodash` | AgilePlus | 2 commits ahead, -8 behind | **MERGE** (likely PR open) |
| `fix/dependabot-agileplus-high-npm` | AgilePlus | Active remote | **MERGE** |
| `trusted-publishing` | AuthKit, DataKit, ResilienceKit | Recent (Apr 24) | **MERGE** (likely CI/OIDC) |
| `ci-wire` | PhenoPlugins, phenotype-journeys | Lightweight CI changes | **MERGE** |
| `fix/thegent-docker-api-migration` | TheGent | **ORPHANED** (local only) | **REVIEW** (6 commits ahead) |
| `dependabot-*` (5 branches) | TheGent | Dated (Apr 22-23) | **MERGE or DELETE** |
| `moby-v2-migration` | TheGent | Potential breaking changes | **REVIEW** |
| `session-disk-governance` | TheGent | Policy work | **REVIEW or DELETE** |

### Secondary Cleanup (Low Priority)

- `bbox-ground-d6`, `reusable-ci`, `viewer-*`, `shot-*` — hwLedger, phenotype-journeys — old feature work, minor files

---

## Ephemeral Artifacts to Delete

### Size Impact: ~1.3GB

| Artifact | Location | Size Est. | Action |
|----------|----------|-----------|--------|
| `npm-sweep` | pheno-wtrees/ | 400MB | DELETE |
| `py-sweep` | pheno-wtrees/ | 400MB | DELETE |
| `rust-sweep` | pheno-wtrees/ | 400MB | DELETE |
| `*-wtrees/` empty dirs | Multiple | <10MB | DELETE |

**Gitignore Additions:**
```
# Analysis artifacts (regenerate on-demand)
*-sweep/
**/*sweep*/
```

---

## New Audit Entries Created

| File | Branch | Classification |
|------|--------|-----------------|
| `agileplus-dep-pyjwt-lodash.md` | PyJWT/Lodash fix | Merged, ready to close |
| `thegent-docker-api-migration.md` | Docker API refactor | Orphaned, review needed |
| `pheno-wtrees-sweep-set.md` | Analysis snapshots | Ephemeral, delete |

---

## Recommendations (Priority Order)

### IMMEDIATE (1-2h)

1. **Merge active dependabot branches** (AgilePlus, TheGent, others)
2. **Delete ephemeral sweeps** (1.3GB recovery)
3. **Verify orphaned branches** (docker-api-migration, session-disk-governance) — complete or abandon

### MEDIUM (1 week)

4. **Consolidate CI branches** (`ci-wire`, `trusted-publishing` patterns) into canonical workflows
5. **Archive old feature work** (viewer-v0.1.3, shot-gallery, user-story-batch1) to `docs/archive/`

### LONG-TERM

6. **Enforce `-wtrees/` policy**: only active/mergeable branches in wtrees; use .gitignore for ephemeral artifacts
7. **Monthly cleanup sweep**: `git worktree prune` + stale branch detection

---

## Disk Savings Opportunity

- **Delete 3 sweeps:** 1.3GB
- **Archive 8+ old branches:** 200MB
- **Total:** ~1.5GB reclaimed

---

**Next Action:** User to confirm cleanup scope before agent removes branches/directories.

