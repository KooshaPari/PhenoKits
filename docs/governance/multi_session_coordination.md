# Multi-Session Coordination (Argis ↔ Helios)

Canonical protocol for coordinating concurrent agent sessions that share the
`repos/` workspace. Promoted from `repos/.argis-helios-bus/README.md`.

## Bus layout

```
repos/.argis-helios-bus/
  README.md                # schema + protocol (this doc supersedes it)
  argis.outbox.jsonl       # Argis-authored events (append-only)
  helios.outbox.jsonl      # Helios-authored events (append-only)
  locks/
    <resource>.lock        # advisory lock files, one per shared resource
```

- Each party writes **only** to its own `*.outbox.jsonl`.
- Each party reads **both** outboxes.
- JSONL: one JSON object per line, append-only, never rewritten.

## Event schema

Required fields on every event:

| Field   | Type   | Meaning |
|---------|--------|---------|
| `ts`    | string | ISO-8601 UTC timestamp. |
| `from`  | string | `"argis"` or `"helios"`. |
| `kind`  | string | `alert`, `ack`, `claim`, `release`, `status`, `note`. |
| `topic` | string | Short topic tag (e.g. `ENOSPC`, `cargo-queue`). |

Optional: `resource`, `detail`, `ref` (correlation id).

Example:

```json
{"ts":"2026-04-22T18:04:11Z","from":"argis","kind":"alert","topic":"ENOSPC","detail":"free=2.1Gi"}
{"ts":"2026-04-22T18:04:44Z","from":"helios","kind":"ack","topic":"ENOSPC","ref":"2026-04-22T18:04:11Z"}
```

## Advisory locks

- To claim a shared resource, write `locks/<resource>.lock` containing a JSON
  blob: `{"owner":"argis","ts":"...","pid":12345}`.
- Release by `rm` of the lock file **and** an event of kind `release` with the
  matching `resource`.
- Locks are advisory only; honor them unless the owner is demonstrably stale
  (no heartbeat for >15 min).

## ENOSPC pact

When either side detects disk pressure (see `disk_budget_policy.md`):

1. Writer appends:
   `{"kind":"alert","topic":"ENOSPC","detail":"<df output>"}`.
2. Writer pauses all new cargo dispatches and long-running builds.
3. Peer purges its own orphaned `target/` dirs and any completed-push worktree
   scratch space it owns.
4. Peer acknowledges with:
   `{"kind":"ack","topic":"ENOSPC","ref":"<writer-ts>"}`.
5. Writer resumes dispatches only after seeing the `ack` and re-running
   `df -h`.

Neither side purges into the other's workspace. Helios owns FocalPoint and its
associated `target/` trees; Argis must not touch them.

## Territorial rules

- **Helios territory:** `FocalPoint/**`, any path Helios flags via a `claim`
  event.
- **Argis territory:** everything else it owns by default, minus Helios
  claims.
- Canonical `repos/<project>/` checkouts are shared; mutations must be on
  `main` pulls or explicit merge integration only (see root `CLAUDE.md`).

## See also

- `disk_budget_policy.md`
- `enospc_playbook.md`
- `long_push_pattern.md`
