# ARCHIVED - Package Moved

## Status: ARCHIVED

This package has been **moved** to the new monorepo structure.

## New Location

```
packages/phenotype-task/
```

## Migration Guide

If you were depending on this package, update your references:

### Before
```json
{
  "dependencies": {
    "@phenotype/task-engine": "^0.1.0"
  }
}
```

### After
```json
{
  "dependencies": {
    "@phenotype/task": "^0.1.0"
  }
}
```

## Rationale

See ADR-0002: Package Classification Framework for the rationale behind this reorganization.

## Date Archived

2026-03-25

## See Also

- `governance/adrs/0002-package-classification-framework.md`
- `governance/MIGRATION.md`
