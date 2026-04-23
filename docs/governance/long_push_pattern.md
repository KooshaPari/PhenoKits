# Long Push Pattern (nohup + disown)

Canonical pattern for `git push` operations that trigger slow pre-push hooks
(notably Lefthook `workspace-verify` on hwLedger, 300-550 s).

## Problem

- Harness-level shell timeouts kill long-running foreground pushes (default
  ~2 min).
- A push killed mid-hook leaves the local ref updated but the remote
  unchanged, producing inconsistent state and confusing retries.
- Session termination propagates SIGHUP/SIGTERM to child processes by
  default — foreground pushes die with the session.

## Pattern

```bash
slug="argis-<short-topic>"
nohup git push -u origin <branch> \
  > /tmp/${slug}-push.log 2>&1 &
disown
```

Then poll:

```bash
# Tail the log (non-blocking)
tail -n 20 /tmp/${slug}-push.log

# Heartbeat check: did the remote actually advance?
git ls-remote origin <branch>
```

`nohup` + `disown` reparents the push to launchd, so it survives:

- Harness-level timeouts.
- The current shell exiting.
- The session being killed.

## Verification

**Never** treat an empty or quiet log tail as success. A hook that is still
running writes nothing to stdout.

The only authoritative success check is:

```bash
git ls-remote origin <branch>
```

The printed SHA must match `git rev-parse <branch>` locally. If it does not,
the push is either still running or failed; inspect the log file.

## When to use

Use this pattern whenever any of the following is true:

- The repo has a pre-push `workspace-verify`, `cargo test --workspace`, or
  similar hook.
- The push is expected to take ≥3 min.
- You are dispatching inside a harness with a bash timeout shorter than the
  expected push duration.

## What not to do

- Do **not** use `--no-verify` to shortcut the hook. See root `CLAUDE.md`
  governance: hooks must pass, not be bypassed.
- Do **not** amend a commit to retry a push that was "probably" killed
  mid-hook. Use `git ls-remote` to determine state first, then push again if
  needed.
- Do **not** rely on foreground `git push` with a 10-minute harness timeout;
  session kill still propagates SIGHUP.

## See also

- `disk_budget_policy.md` — pre-push disk check.
- `enospc_playbook.md` — what to do if the verify step ENOSPCs.
