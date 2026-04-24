# DEPRECATED

This repository was retired on 2026-04-24 per session cleanup (audit #231).

Status: LOCAL subdirectory inside parent `phenotype-infra` monorepo. Local `.git` points to `KooshaPari/PhenoKits` (origin) and `KooshaPari/phenoShared` (pheno remote).

Reason: Subsumed by the `pheno-xdd` / `pheno-xdd-lib` split. Audit trail below, verified 2026-04-24.

## Migration map (verified 2026-04-24)

| Local dir | New home | Status |
|-----------|----------|--------|
| `pheno-xdd/` | `KooshaPari/phenoXdd` (150+ xDD methodologies, last updated 2026-04-24) | Published, actively maintained |
| `pheno-xdd-lib/` | `KooshaPari/phenoXddLib` (cross-cutting xDD utilities) | Published |
| `Schemaforge/` | Docs absorbed into `KooshaPari/PhenoDevOps/rust/forge/docs/absorbed/schemaforge/` (SPEC, PRD, PLAN, ADR) | Docs migrated; no code (local Schemaforge is also docs-only — `validate_governance.py` + specs) |
| `schemas/` | Docs only (README, SPEC, ADR). No schema code to migrate. | Safe to discard |
| `ArgisRoute/` | **Not migrated as code.** Only referenced in `KooshaPari/thegent` docs (PRDs, debug tags, session notes). `ArgisRoute/core/schemas/schemas.go` (single Go file) is orphaned. | See migration proposal |

## Recovery sources

- Git log of the parent `phenotype-infra` repo (this directory's history lives there)
- Local `.git` in this directory (points to `PhenoKits` origin)
- Canonical replacements: `KooshaPari/phenoXdd`, `KooshaPari/phenoXddLib`, `KooshaPari/PhenoDevOps` (rust/forge absorbed docs)

## Action required

See `repos/docs/governance/phenoschema-migration-proposal.md` for ArgisRoute handling (Go code + 6 ADRs + SOTA/SPEC docs are not migrated anywhere).
