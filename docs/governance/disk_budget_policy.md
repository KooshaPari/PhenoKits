# Disk Budget Policy (Phenotype repos)

Canonical policy for agent disk-space discipline when dispatching cargo-heavy
work or pushes that trigger `workspace-verify` hooks.

## Pre-dispatch check

Before dispatching a cargo-heavy agent or pushing a branch whose pre-push hook
runs a workspace verify, always run:

```bash
df -h /System/Volumes/Data | tail -1
```

Decision table:

| Free space | Action |
|------------|--------|
| >10 Gi     | Continue. |
| 3-10 Gi    | Pause new dispatches; purge completed-push worktree `target/` dirs. |
| <3 Gi      | Emergency — see `enospc_playbook.md`. |

Abort any new dispatch when free space is under 10 Gi until a purge has been
run and re-verified.

## APFS reality: Trash does not reclaim space

On APFS, moving files to `~/.Trash` does **not** free disk space until the user
empties the Trash from Finder. Agents cannot empty Trash.

Therefore:

- Use `rm -rf` on orphaned worktree `target/` directories to actually reclaim
  space.
- Use `mv ~/.Trash/...` **only** when the user has explicitly asked to
  preserve files for manual recovery. Never use `mv ~/.Trash` as a
  space-recovery strategy.

## Agent-worktree `target/` directory sizing

Observed in practice:

- Each orphaned agent-worktree `target/` directory averages **6-8 GB**.
- Every additional concurrent cargo build above 4 concurrent triggers another
  `workspace-verify` compile cache.
- A session with 5 rounds of multi-agent cargo builds at 4+ concurrency each
  hit ENOSPC 5 times.

## Concurrency rule

Do not stack more than **4 concurrent** pre-push `workspace-verify` runs or
cargo builds across the workspace. Dispatch serially when you can; queue the
rest through the coordination bus.

## What to purge, in order

1. Completed-push agent worktree `target/` directories (branch already on
   `origin`).
2. Archived worktrees under `.worktrees/**` with no uncommitted changes.
3. Cargo global caches last — they re-warm slowly and affect every repo.

Never purge:

- Canonical checkouts (`repos/<project>/target`) if an active agent is
  mid-build there.
- Worktrees containing uncommitted work.
- Worktrees owned by FocalPoint / Helios's area (see
  `multi_session_coordination.md`).

## See also

- `enospc_playbook.md` — level-1/2/3 escalation.
- `long_push_pattern.md` — why pushes trigger `workspace-verify`.
- `multi_session_coordination.md` — how to negotiate disk with peer sessions.
