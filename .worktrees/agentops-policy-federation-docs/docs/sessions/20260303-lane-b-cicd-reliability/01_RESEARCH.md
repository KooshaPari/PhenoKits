# 01 Research: CI/CD Reliability Failure-Class Snapshot

- Generated at: 2026-03-03 01:46:16 UTC
- Source command:
```bash
gh run list --limit 200 --json databaseId,workflowName,conclusion,createdAt,headBranch,url
```

## Dataset coverage
- Total runs fetched: `200`
- Non-success runs analyzed: `124`

## Top 5 failure classes (by `conclusion`)

| Rank | Failure class | Count | Percentage |
|---:|---|---:|---:|
| 1 | `failure` | 59 | 47.6% |
| 2 | `action_required` | 39 | 31.5% |
| 3 | `skipped` | 25 | 20.2% |
| 4 | `cancelled` | 1 | 0.8% |

## Notes
- Percentages are relative to non-success runs only.
- Raw run inventory is stored in `ci_runs_raw.json` for downstream drilldown.
