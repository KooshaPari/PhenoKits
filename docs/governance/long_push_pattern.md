# Long Push Pattern

Canonical pattern for `git push` operations that trigger slow pre-push hooks,
especially Lefthook `workspace-verify` runs.

## Problem

- Harness-level shell timeouts can kill long-running foreground pushes.
- A push killed mid-hook leaves the local ref advanced while the remote remains
  unchanged.
- Session termination can send SIGHUP or SIGTERM to child processes.

## Pattern

```bash
slug="argis-<short-topic>"
nohup git push -u origin <branch> \
  > /tmp/${slug}-push.log 2>&1 &
disown
```

Then poll:

```bash
tail -n 20 /tmp/${slug}-push.log
git ls-remote origin <branch>
```

`nohup` plus `disown` lets the push survive harness timeouts, shell exit, and
session termination.

## Verification

Never treat a quiet log as success. A hook can run silently.

The authoritative success check is:

```bash
git ls-remote origin <branch>
```

The remote SHA must match:

```bash
git rev-parse <branch>
```

If it does not match, the push is still running or failed.

## When to Use

Use this pattern when:

- The repo has a pre-push `workspace-verify`, `cargo test --workspace`, or
  similar hook.
- The push is expected to take at least three minutes.
- The shell/harness timeout is shorter than the expected hook duration.

## What Not To Do

- Do not use `--no-verify` to shortcut hooks.
- Do not amend commits just to retry a push that may still be running.
- Do not rely on foreground pushes for slow hook paths.

## See Also

- `disk_budget_policy.md`
- `enospc_playbook.md`
