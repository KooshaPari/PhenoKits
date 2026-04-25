# PhenoLibs Python Migration Deferred Plan

**Date:** 2026-04-24
**Status:** Deferred - do not execute without renewed review
**Scope:** PhenoLibs Python packages blocked by unresolved legacy `pheno.*` imports

## Context

This note follows the Option A PhenoLibs migration work in
`docs/governance/phenolibs-migration-proposal.md`.

The low-risk target-repo migrations were handled separately:

- Rust bindings to PhenoProc.
- Go cgo and TypeScript core to PhenoKits.
- Python CLI/tooling packages to PhenoKits.
- `core-utils` to `pheno`.

This document covers the remaining Python packages that were not migrated because
they require import-graph rewrites and likely namespace architecture decisions.

## Diagnosis

- `pheno.*` is not the PyPI `pheno` package. PyPI `pheno==0.1.0` is owned by
  `hosford42`, points to `github.com/hosford42/xcs`, and is "Genetic programming
  for arbitrary domains", unrelated to KooshaPari/Phenotype.
- Current `KooshaPari/pheno` has limited top-level `pheno` namespace code at
  `pheno/python/phenosdk/src/pheno` and `pheno/python/src/pheno`, but it only
  covers small `mcp`, `shared`, `adapters`, `auth`, and `vector` surfaces.
- The blocked PhenoLibs packages are split-package extractions: package dirs are
  `pheno_domain`, `pheno_adapters`, etc., but many internal imports still assume
  an older monolithic namespace shape like `pheno.domain`, `pheno.application`,
  `pheno.logging`, `pheno.config`, `pheno.infra`.
- So `pheno.*` most likely refers to a retired/pre-extraction PhenoSDK namespace
  facade, not a live external dependency.

## Deferred Packages

The following packages are intentionally not migrated in the low-risk round:

- `PhenoLibs/python/pheno-async`
- `PhenoLibs/python/pheno-adapters`
- `PhenoLibs/python/pheno-analytics`
- `PhenoLibs/python/pheno-deployment`
- `PhenoLibs/python/pheno-dev`
- `PhenoLibs/python/pheno-domain`
- `PhenoLibs/python/pheno-optimization`
- `PhenoLibs/python/pheno-patterns`
- `PhenoLibs/python/pheno-plugins`
- `PhenoLibs/python/pheno-ports`
- `PhenoLibs/python/pheno-process`
- `PhenoLibs/python/pheno-providers`

## Migration Plan

### 1. Build a local import map first.

- Canonicalize to current split package names, not PyPI `pheno`.
- Example: `pheno.domain.*` -> `pheno_domain.*`,
  `pheno.adapters.*` -> `pheno_adapters.*`,
  `pheno.analytics.*` -> `pheno_analytics.*`.

### 2. Batch A: foundation packages into `pheno/python`.

- `pheno-domain`: migrate first; rewrite internal `pheno.domain.*` imports to
  `pheno_domain.*`.
- `pheno-ports`: migrate after domain; rewrite `pheno.domain.*` and decide/fold
  existing duplicates against `pheno_core.observability` and `pheno_mcp`.
- `pheno-async`: rewrite `pheno.logging.core.logger` to existing
  `pheno_core.logging` or add a narrow logging adapter in-package.

### 3. Batch B: application/adapters layer.

- `pheno-adapters`: depends heavily on `pheno_domain`, `pheno_ports`, and
  missing `pheno.application`.
- Before migrating, decide whether `pheno.application` should become a new
  package, e.g. `pheno-application`, or whether those use-case/DTO/port files
  already exist elsewhere.
- Do not import until `pheno.application.*` has a canonical destination.

### 4. Batch C: analytics/dev/optimization self-contained-ish packages.

- `pheno-analytics`: mostly rewrite `pheno.analytics.*` ->
  `pheno_analytics.*`; map logging/cache imports.
- `pheno-dev`: mostly rewrite `pheno.dev.*` -> `pheno_dev.*`; map
  config/database/events dependencies.
- `pheno-optimization`: mostly rewrite `pheno.optimization.*` ->
  `pheno_optimization.*`; replace `pheno.utilities.rate_limiter`.

### 5. Batch D: patterns and process.

- `pheno-patterns`: depends on domain/application/adapters/observability, so
  migrate only after Batch B.
- `pheno-process`: blocked on `pheno.config` and `pheno.infra`; likely needs
  mapping to `pheno_core.config` plus a new or existing infra package.

### 6. Batch E: plugins/providers last.

- `pheno-plugins`: blocked on database/security/config namespaces.
- `pheno-providers`: blocked on `pheno.adapters.base_registry`.
- Migrate only after adapters and registry ownership is resolved.

## Recommendation

Do not add a `pheno.*` compatibility shim just to make imports pass. Treat these
as incomplete split-package migrations and rewrite imports to the canonical
package modules, adding only real missing packages where a whole layer exists,
especially `pheno-application` / infra if confirmed by source review.

## Guardrails

- Do not execute these migrations as straight directory imports.
- Do not depend on PyPI `pheno`; it is unrelated.
- Do not add broad compatibility facades without a target architecture decision.
- Do not migrate Batch B or later before Batch A import ownership is validated.
- Preserve history with subtree/split patterns if and when these packages are
  migrated.
- Keep one PR per target repository and one coherent batch per PR.

