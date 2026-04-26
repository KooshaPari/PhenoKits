# AgilePlus: Bare Git Configuration Diagnosis

**Date:** 2026-04-25  
**Severity:** High (blocks Cargo builds + agent workflows)  
**Finding:** W-73B cross-repo audit

## Symptom

AgilePlus canonical repo (`/repos/AgilePlus/`) has `core.bare = true` set in `.git/config`, which blocks:
- `git status`, `git checkout`, `git add`, `git commit` → fatal: "this operation must be run in a work tree"
- Cargo build scripts → error: "did not expect repo at ... to be bare"
- Agent workflows requiring git operations

## Root Cause

Configuration set at line 4 of `.git/config`:
```
[core]
    bare = true
```

The repository **does have a working tree** (files present at root), so this is a misconfiguration, not an intentional bare setup.

## Impact

- **Cargo builds fail** with: `did not expect repo ... to be bare`
- **All git operations blocked** (status, add, commit, checkout, branch)
- **Canonical workflow broken:** Per Phenotype/CLAUDE.md, AgilePlus canonical should support r/w operations on `main`
- **Worktree pattern**: Feature work should use `repos/AgilePlus-wtrees/<topic>/`, but canonical must remain a normal working repository

## Recommended Fix

```bash
git config core.bare false
# (optional) verify with: git config --list --local | grep bare
# (optional) restore HEAD if needed: git reset --hard HEAD~0
```

The fix is **non-destructive**: toggles the flag without touching files or history.

## Verification

After fix, confirm:
- `git status --short --branch` succeeds and shows `main`
- `cargo build` completes without bare-repo error
- All branch tracking entries remain intact (23 branches currently tracked)

## Decision Required

User approval needed before executing `git config core.bare false`. This affects infrastructure-layer configuration for the canonical repository.
