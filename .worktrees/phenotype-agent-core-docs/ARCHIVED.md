# Archived: MIGRATED to packages/phenotype-agent

## Status

This repository has been migrated to [packages/phenotype-agent](../packages/phenotype-agent/)

## Migration Date

2026-03-25

## What Changed

| Item | Old | New |
|------|-----|-----|
| Package Name | `@phenotype/agent-core` | `@phenotype/agent` |
| Repository URL | `kooshapari/phenotype-agent-core` | `phenotype/packages` |
| Location | `phenotype-agent-core/` | `packages/phenotype-agent/` |

## Action Required

If you have dependencies on this package, update:

```json
// OLD
"@phenotype/agent-core": "file:../phenotype-agent-core"

// NEW
"@phenotype/agent": "file:../packages/phenotype-agent"
```

## Verification

```bash
cd packages/phenotype-agent
npm run build
```
