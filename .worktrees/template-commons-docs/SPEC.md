# template-commons Specification

> Shared cross-domain template primitives for the Phenotype template ecosystem

## Overview

template-commons provides the base layer of shared primitives consumed by all domain and language templates in the Phenotype template system.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    template-commons                               │
│                                                                  │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│  │   Schema     │ │  Validation  │ │  Composition │          │
│  │   Primitives │ │  Rules       │ │  Contracts   │          │
│  └──────────────┘ └──────────────┘ └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

## Layer Contract

| Field | Value |
|-------|-------|
| layer_type | commons |
| versioning | semver |
| consumers | All domain and language templates |

## Composition Rules

- Downstream templates pin commons version explicitly
- Breaking changes require major version bump
- `task check` must pass before release
- Changelog update required for contract changes
