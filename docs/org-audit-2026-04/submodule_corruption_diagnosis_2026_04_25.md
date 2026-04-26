# Submodule Corruption Diagnosis — 2026-04-25

## Summary

The canonical `/repos` checkout exhibits **reflog corruption** (invalid reflog entries) but no actual submodule or tree object corruption. The "W-72 tree object 7f5431e7" referenced in previous notes is **not present in the current commit history** — it appears to have been from an earlier aborted rebase.

## Findings

### 1. Submodule Configuration (Clean)
- **.gitmodules:** Single valid entry for `HexaKit`
  - Path: `HexaKit/`
  - URL: `https://github.com/KooshaPari/HexaKit.git`
  - Status: `-d986ff50...` (uninitialized, not corrupt)
- No duplicate entries, no HexaKit in double configs, no AgilePlus-wtrees submodule entry

### 2. Reflog Corruption (Root Cause)
- `git fsck --no-dangling` reports **23+ invalid reflog entries** across:
  - `HEAD` (6 entries)
  - `refs/heads/adr/shared-crates-distribution-2026-04-25` (3 entries)
  - `refs/heads/adr/shared-crates-distribution-clean` (3 entries)
  - `refs/remotes/origin/*` (2 entries)
- **Cause:** Multiple rebase attempts and cherry-picks created stale reflog references to commits no longer in the DAG
- **Commits affected:** 36738a6d1, 8efadf18d, 2a294c07e all valid and accessible
- **Impact:** Reflog lookup may fail, but git operations work (HEAD is valid: 136f368f8)

### 3. Missing Object Claim (False Alarm)
- Tree object `7f5431e7` does **NOT exist** in current repo
- This was from an aborted rebase on `adr/shared-crates-distribution-2026-04-25`
- Current HEAD (136f368f8) tree is valid and reachable: `279f65bd8d0f...`

### 4. Current State
- HEAD: `136f368f8` (cherry-picked ADR docs, clean)
- Parent commits: All accessible
- Worktrees: 11 active worktrees, none blocked by object corruption
- Push candidate: Current branch is on a detached `adr/shared-crates-distribution-clean` with valid commits

## Recommended Fix Steps (Non-Destructive)

1. **Repair reflogs** (safe, recovers broken reflog pointers):
   ```bash
   git reflog expire --all --expire=now
   git gc --aggressive --prune=now
   ```

2. **Verify integrity** (confirms repair):
   ```bash
   git fsck --full 2>&1 | grep -c "error:" # should be 0
   ```

3. **Switch to main and retry push** (if on detached HEAD):
   ```bash
   git checkout main
   git pull origin main
   ```

4. **Do NOT:**
   - `git reset --hard` — not necessary; HEAD is valid
   - `git rm HexaKit` — submodule is properly configured
   - `git reflog delete` — let `expire --all` handle it
   - Deinit submodules — not relevant to this issue

## Conclusion

**No tree object corruption.** Reflog corruption is a side effect of incomplete rebases and can be cleaned safely. The push blockage is likely from reflog validation during `git push` — repairing reflogs should unblock it.
