# Phenotype Governance Index

Canonical policies and runbooks for Phenotype-org repos.

| Policy | File | Scope |
|--------|------|-------|
| Scripting language hierarchy | `scripting_policy.md` | All scripts, tools, CI, hooks, codegen across the org. |
| Disk budget policy | `disk_budget_policy.md` | Pre-dispatch disk checks, APFS reclaim reality, concurrency cap. |
| Multi-session coordination | `multi_session_coordination.md` | Argis ↔ Helios bus: JSONL outboxes, advisory locks, ENOSPC pact. |
| Long push pattern | `long_push_pattern.md` | nohup + disown for pushes with slow `workspace-verify` hooks. |
| ENOSPC escalation playbook | `enospc_playbook.md` | Level-1/2/3 response to disk pressure during multi-agent cargo work. |

## How to use

- Agents must consult the relevant policy **before** taking the governed
  action, not after.
- New patterns learned in a session should be codified here rather than
  living only in MEMORY or ad-hoc worklogs.
- Changes to these policies ship as normal PRs and should link the worklog
  entry that motivated them.
