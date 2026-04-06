# Polyrepo Package Naming & Productization Policy

**Version:** 1.0.0  
**Related:** [PHENOTYPE_ORG_WIDE_ENGINEERING_STANDARD.md](./PHENOTYPE_ORG_WIDE_ENGINEERING_STANDARD.md)

## Tier A — Phenotype-domain–bound (`phenotype-*` allowed)

Use when the package encodes **org/product semantics** (e.g. `phenotype-config`, `phenotype-design`).

## Tier B — Marketable / generic (prefer neutral names)

Reusable libraries useful outside Phenotype: `hexkit-go`, `http-middleware-kit`, etc. **SemVer**, README, CI.

## Tier C — Apps

Product names (`helios-*`, `bifrost-extensions`); compose Tier A/B libraries.

## Productization checklist

Ports/adapters, SemVer, tests, CI, changelog, ADR for boundary.

## Migration

ADR per rename; forward-only; no silent wire renames without decision gate.

## See also

- [PHENOTYPE_PACKAGES_INVENTORY_AND_RENAME_BACKLOG.md](./PHENOTYPE_PACKAGES_INVENTORY_AND_RENAME_BACKLOG.md)
