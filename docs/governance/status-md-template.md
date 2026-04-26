# STATUS.md — Canonical Template

> Copy this file to the root of any repo as `STATUS.md`. Answers must be reviewable in **30 seconds**.
> README.md defers to STATUS.md for project state. This file is the source of truth.
> Verify-then-write: every claim below must be confirmed against the live tree before commit
> (see `~/.claude/projects/.../memory/feedback_verify_origin_not_canonical.md` and
> `feedback_audit_freshness_decay.md`).

---

## State

**`Experimental` | `Alpha` | `Beta` | `Stable` | `Deprecated` | `Archived`**

One sentence: why this state.

## Last Verified

`YYYY-MM-DD` by `<owner-or-agent-id>` against commit `<sha>` on `<branch>`.

## Owner

- **Primary:** `<github-handle>` / `<email>`
- **Backup:** `<github-handle>` / `<team>`
- **Identity verification:** confirm against `repos/docs/governance/identity-verification.md` before assigning.

## What Works (Verified)

- [ ] `<capability>` — verified by `<test-or-command>` on `<date>`
- [ ] `<capability>` — verified by `<test-or-command>` on `<date>`

## What Doesn't Work Yet (Honest)

- [ ] `<gap>` — tracked in `<issue-link>`
- [ ] `<known-broken-path>` — tracked in `<issue-link>`

## Hello World (Commands That Actually Work)

```bash
# Copy-paste reproducible. Every command must be verified within the last 14 days.
git clone <repo-url> && cd <repo>
<install-cmd>
<run-cmd>
# Expected output: <one-line>
```

## Where to Ask Questions

- **Issues:** `<github-issues-url>`
- **Discussions:** `<discussions-or-slack-url>`
- **Internal tracker:** `<agileplus-feature-id>` or `<linear-url>`

## Known Issues

- `<github-issues-url>?q=is%3Aopen+label%3Abug>`
- `<internal-tracker-url>`

## Update Cadence

- Updated on every release (mandatory).
- Minimum weekly refresh of `Last Verified` while state is `Alpha` or `Beta`.
- Update on state transition; new release without STATUS bump is a CI failure.

---

*Template version: 1.0 — `repos/docs/governance/status-md-template.md`*
