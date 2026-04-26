# Disk Budget Policy (Phenotype repos)

Canonical policy for agent disk-space discipline when dispatching cargo-heavy work or pushes that trigger `workspace-verify` hooks.

## Pre-dispatch gate

**Wired binary:** `/repos/FocalPoint/tooling/disk-check/` — run before any cargo-heavy agent dispatch or concurrent builds.

```bash
disk-check --min-gb 30 --warn-gb 10 --path /System/Volumes/Data --verbose
```

Exit codes:
- **0** — OK, >=30GB free, safe to dispatch
- **2** — WARNING, 10-30GB free, advise user to purge; don't auto-block
- **1** — CRITICAL, <10GB free, abort dispatch

## Manual pre-dispatch check

Before dispatching a cargo-heavy agent or pushing a branch whose pre-push hook runs a workspace verify:

```bash
df -h /System/Volumes/Data | tail -1
```

Decision table:

| Free space | Action |
|------------|--------|
| >30 Gi     | Continue (safe for multi-cargo). |
| 10-30 Gi   | Warn user; pause new >4-agent dispatches. |
| <10 Gi     | Emergency — purge immediately (see below). |

## APFS reality: Trash does not reclaim space

On APFS, moving files to `~/.Trash` does NOT free disk space until the user empties Trash manually. **Agents cannot empty Trash.**

Therefore:

- Use `rm -rf` on orphaned worktree `target/` directories to immediately reclaim space.
- Use `mv ~/.Trash` **only** when the user has explicitly asked to preserve files for manual recovery. Never use as a space-recovery strategy.

## Observed sizing

- Each orphaned agent-worktree `target/` directory: **6-8 GB**.
- Every additional concurrent cargo build above 4 concurrent: another `workspace-verify` compile cache.
- Stacked 5 rounds at 4+ concurrency → ENOSPC 5 times.

## Concurrency rule

Do not stack more than **4 concurrent** pre-push `workspace-verify` runs or cargo builds. Dispatch serially when possible; queue the rest through coordination bus.

## What to purge, in order

1. Completed-push agent worktree `target/` directories (branch already on `origin`).
2. Archived worktrees under `.worktrees/**` with no uncommitted changes.
3. Cargo global caches last (re-warm slowly, affect every repo).

**Never purge:**
- Canonical checkouts (`repos/<project>/target`) if an active agent is mid-build.
- Worktrees containing uncommitted work.
- Worktrees owned by FocalPoint / Helios (see `multi_session_coordination.md`).

## Quick purge command

```bash
# Remove completed-push worktrees' targets
find /Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees -name target -type d \
  -exec du -h {} \; | sort -h
# Then: rm -rf /Users/kooshapari/CodeProjects/Phenotype/repos/.worktrees/<branch>/target
```

## See also

- `enospc_playbook.md` — level-1/2/3 escalation.
- `long_push_pattern.md` — why pushes trigger `workspace-verify`.
- `multi_session_coordination.md` — disk negotiation with peer sessions.
