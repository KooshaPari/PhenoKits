# Multi-Session Coordination

Protocol for coordinating concurrent agent sessions that share the `repos/`
workspace.

## Bus Layout

```text
repos/.argis-helios-bus/
  README.md
  argis.outbox.jsonl
  helios.outbox.jsonl
  locks/
    <resource>.lock
```

- Each party writes only to its own `*.outbox.jsonl`.
- Each party reads both outboxes.
- JSONL is append-only: one JSON object per line.

## Event Schema

Required fields:

| Field | Type | Meaning |
|-------|------|---------|
| `ts` | string | ISO-8601 UTC timestamp. |
| `from` | string | `argis`, `helios`, or another agreed session name. |
| `kind` | string | `alert`, `ack`, `claim`, `release`, `status`, or `note`. |
| `topic` | string | Short topic tag, such as `ENOSPC` or `cargo-queue`. |

Optional fields: `resource`, `detail`, `ref`.

Example:

```json
{"ts":"2026-04-22T18:04:11Z","from":"argis","kind":"alert","topic":"ENOSPC","detail":"free=2.1Gi"}
{"ts":"2026-04-22T18:04:44Z","from":"helios","kind":"ack","topic":"ENOSPC","ref":"2026-04-22T18:04:11Z"}
```

## Advisory Locks

- Claim a shared resource by writing `locks/<resource>.lock` with JSON:
  `{"owner":"argis","ts":"...","pid":12345}`.
- Release by removing the lock file and appending a `release` event with the
  matching resource.
- Locks are advisory; honor them unless the owner is demonstrably stale.

## ENOSPC Pact

When either side detects disk pressure:

1. Append an `alert` event with topic `ENOSPC`.
2. Pause new cargo dispatches and long-running builds.
3. Peer purges its own orphaned `target/` dirs and completed-push scratch.
4. Peer appends an `ack` event referencing the alert timestamp.
5. Writer resumes only after seeing the acknowledgement and re-checking disk.

Neither side purges the other's workspace.

## Territorial Rules

- Helios territory: `FocalPoint/**` and any path Helios claims on the bus.
- Argis territory: paths it claims, excluding Helios claims.
- Canonical `repos/<project>/` checkouts are shared; mutations require an
  explicit branch/worktree or merge-integration path.

## See Also

- `disk_budget_policy.md`
- `enospc_playbook.md`
- `long_push_pattern.md`
