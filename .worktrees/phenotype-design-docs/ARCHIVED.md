# Archived: MIGRATED to packages/phenotype-design

## Status

This repository has been migrated to [packages/phenotype-design](../packages/phenotype-design/)

## Migration Date

2026-03-25

## What Changed

| Item | Old | New |
|------|-----|-----|
| Package Name | `@kooshapari/design` | `@phenotype/design` |
| Repository URL | `kooshapari/phenotype-design` | `phenotype/packages` |
| Location | `phenotype-design/` | `packages/phenotype-design/` |

## Action Required

If you have dependencies on this package, update:

```json
// OLD
"@kooshapari/design": "file:../phenotype-design"

// NEW
"@phenotype/design": "file:../packages/phenotype-design"
```

## Verification

```bash
cd packages/phenotype-design
npm run build
```
