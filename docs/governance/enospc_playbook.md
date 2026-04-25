# ENOSPC Escalation Playbook

Runbook for disk-space emergencies during multi-agent cargo work in the
Phenotype `repos/` workspace.

## Levels

| Level | Free on `/System/Volumes/Data` | Action |
|-------|--------------------------------|--------|
| 1 | `>10 Gi` | Continue normal dispatch. |
| 2 | `3-10 Gi` | Pause new cargo dispatches; purge completed-push worktree `target/` dirs. |
| 3 | `<3 Gi` | Emergency: broadcast alert, pause all dispatches, purge inactive scratch. |

Baseline check:

```bash
df -h /System/Volumes/Data | tail -1
```

## Level 1: Nominal

- Continue dispatching.
- Respect the four-concurrent cap on cargo and `workspace-verify` runs.

## Level 2: Caution

1. Stop dispatching new cargo-heavy agents.
2. Enumerate orphaned agent-worktree `target/` directories whose branch is
   already landed on `origin`.
3. Remove those `target/` dirs with `rm -rf`.
4. Re-run `df -h`.
5. Resume only after free space is above `10 Gi`; otherwise escalate.

## Level 3: Emergency

1. Broadcast on the coordination bus:

   ```json
   {"ts":"<iso>","from":"<side>","kind":"alert","topic":"ENOSPC","detail":"<df output>"}
   ```

2. Pause all dispatches: no new builds, pushes, or tests.
3. Hard-purge inactive scratch:
   - Completed-push worktree `target/` dirs.
   - Cargo global caches if necessary.
   - Archived worktrees under `.worktrees/**` with no uncommitted changes.
4. Wait for the user to empty Trash if prior sessions moved files there.
5. Wait for peer-session acknowledgement before resuming.

## Preservation List

Even at Level 3, preserve:

- Canonical checkouts under `repos/<project>/`.
- Worktrees with an actively running agent.
- Worktrees with uncommitted work.
- FocalPoint and anything Helios has claimed on the bus.

## Post-Incident

After an ENOSPC event:

1. Record the incident in the relevant worklog.
2. Note concurrency at failure time and space reclaimed.
3. Lower the concurrency cap if the event occurred at or below the current cap.

## See Also

- `disk_budget_policy.md`
- `multi_session_coordination.md`
