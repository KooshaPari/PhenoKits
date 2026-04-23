# ENOSPC Escalation Playbook

Canonical runbook for disk-space emergencies during multi-agent cargo work on
the Phenotype `repos/` workspace.

## Quick reference

| Level | Free on `/System/Volumes/Data` | Action |
|-------|--------------------------------|--------|
| 1     | >10 Gi                         | Continue normal dispatch. |
| 2     | 3-10 Gi                        | Pause new cargo dispatches; purge completed-push worktree `target/` dirs. |
| 3     | <3 Gi                          | Emergency: broadcast alert, pause all dispatches, hard-purge everything not actively holding uncommitted work. |

Baseline check:

```bash
df -h /System/Volumes/Data | tail -1
```

## Level 1 — Nominal (>10 Gi free)

- Continue dispatching.
- Still respect the 4-concurrent cap on cargo/`workspace-verify` runs (see
  `disk_budget_policy.md`).

## Level 2 — Caution (3-10 Gi free)

1. Stop dispatching new cargo-heavy agents. Queue them.
2. Enumerate orphaned agent-worktree `target/` directories whose branch is
   already landed on `origin`.
3. `rm -rf` those `target/` dirs. Do **not** use `mv ~/.Trash` — Trash does
   not reclaim APFS space.
4. Re-run `df -h`. If back above 10 Gi, resume at Level 1.
5. If still under 10 Gi after the purge, escalate to Level 3.

## Level 3 — Emergency (<3 Gi free)

1. Broadcast on the coordination bus (see `multi_session_coordination.md`):
   ```json
   {"ts":"<iso>","from":"<side>","kind":"alert","topic":"ENOSPC","detail":"<df output>"}
   ```
2. Pause **all** dispatches — no new builds, no new pushes, no new tests.
3. Hard-purge everything not actively holding uncommitted work:
   - Completed-push worktree `target/` dirs.
   - Cargo global caches (`~/.cargo/registry/cache/`, `~/.cargo/git/`).
   - Archived worktrees under `.worktrees/**` with no uncommitted changes.
4. Wait for the user to empty the Trash (agents cannot do this).
5. Wait for peer session to `ack` the ENOSPC alert before resuming.

## Preservation list (never touch)

Even at Level 3, always preserve:

- Canonical checkouts under `repos/<project>/`.
- Worktrees with an actively running agent.
- Worktrees with uncommitted work (check `git status --short`).
- **FocalPoint** and anything Helios has `claim`ed on the bus.

## Why `rm -rf`, not Trash

- APFS does not reclaim space on `mv ~/.Trash`. Files stay billed against the
  volume until the user empties Trash from Finder.
- Agents cannot empty Trash.
- Therefore `rm -rf` is the only effective purge for agents.
- Reserve `mv ~/.Trash` exclusively for cases where the user has asked to
  preserve for manual review. In that case, the space cost is on the user.

## Post-incident

After an ENOSPC event:

1. Record the round in the session's worklog under `worklogs/GOVERNANCE.md`
   or `worklogs/PERFORMANCE.md` (pick by dominant cause).
2. Note the concurrency at time of failure and how much space was reclaimed.
3. Update the concurrency cap downward if the event was triggered at or
   below the current cap.

## See also

- `disk_budget_policy.md`
- `multi_session_coordination.md`
- `long_push_pattern.md`
