# PhenoLibs — Migration Proposal for Orphaned Content

**Date:** 2026-04-24
**Author:** audit agent (session #231 follow-up)
**Status:** Proposal — requires decision before execution

## Context

`repos/PhenoLibs/` was marked retired in audit #231. Original remote `KooshaPari/PhenoKit` was deleted. Most Rust crates migrated to `KooshaPari/PhenoProc`, and most Python packages migrated to `KooshaPari/pheno` or `KooshaPari/PhenoKits`. A subset of content has **no verified downstream home** and must be handled deliberately.

See `repos/PhenoLibs/DEPRECATED.md` for the full migration map.

## Orphaned content

### Rust (2 crates)

- `rust/phenotype-core-py` — Python bindings for phenotype-core (PyO3).
- `rust/phenotype-core-wasm` — WASM bindings for phenotype-core.

### Go (1 module)

- `go/pheno-core-cgo` — cgo bindings. `go.mod` + `entity.go` + README. Not present anywhere in the KooshaPari org.

### TypeScript (1 package)

- `typescript/packages/core` — npm package core. Not clearly migrated.

### Python (~19 packages)

Present locally but not verified as migrated to `KooshaPari/pheno`, `KooshaPari/PhenoKits`, or `KooshaPari/phenoShared`:

`pheno-errors`, `pheno-async`, `pheno-config`, `pheno-adapters`, `pheno-analytics`, `pheno-deployment`, `pheno-dev`, `pheno-domain`, `pheno-exceptions`, `pheno-optimization`, `pheno-patterns`, `pheno-plugins`, `pheno-ports`, `pheno-process`, `pheno-providers`, `cli-kit`, `cli-builder-kit`, `config-kit`, `core-utils`.

## Proposal

### Option A (recommended): Consolidate into PhenoProc + PhenoKits

1. **Rust bindings** (`phenotype-core-py`, `phenotype-core-wasm`): merge into `KooshaPari/PhenoProc/crates/` alongside `phenotype-core`. These are binding crates for the already-migrated core; they belong in the same workspace.
2. **Go `pheno-core-cgo`**: merge into `KooshaPari/PhenoKits/libs/go/` as `pheno-core-cgo`. Matches the existing `phenotype-go-*` convention.
3. **TypeScript `packages/core`**: rename to `phenotype-core-ts` and merge into `KooshaPari/PhenoKits/libs/typescript/`. Matches existing `plugin-typescript` sibling.
4. **Python orphans**: triage in three buckets:
   - **Core utilities** (`pheno-errors`, `pheno-async`, `pheno-config`, `pheno-exceptions`, `core-utils`): merge into `KooshaPari/pheno/python/` alongside `pheno-core`.
   - **CLI toolkits** (`cli-kit`, `cli-builder-kit`, `config-kit`): merge into `KooshaPari/PhenoKits/libs/python/` as the CLI story there.
   - **Domain-shaped** (`pheno-adapters`, `pheno-analytics`, `pheno-deployment`, `pheno-dev`, `pheno-domain`, `pheno-optimization`, `pheno-patterns`, `pheno-plugins`, `pheno-ports`, `pheno-process`, `pheno-providers`): per-package review. Many overlap with hexagonal patterns already in `KooshaPari/phenoShared` (has `phenotype-port-interfaces`, `phenotype-domain`, `phenotype-application`, `phenotype-event-sourcing`). Likely duplicates — audit for novel content, fold unique bits into phenoShared.

### Option B: Push PhenoLibs as a new standalone repo

Create `KooshaPari/PhenoLibs` as a fresh multi-language mono-kit. Low value — duplicates existing per-language kits and would create four-way overlap with PhenoProc, pheno, PhenoKits, phenoShared.

**Not recommended.**

### Option C: Discard

For any package where content is genuinely stale (pre-hexagonal refactor, superseded design), tag as dead and do not migrate. Git history in local `.git` preserves recoverability.

## Execution plan (if Option A is approved)

Agent-driven, ~15-30 tool calls:

1. For each Rust binding crate: copy to `PhenoProc/crates/`, add to workspace `members`, run `cargo check -p <crate>`. 3 tool calls per crate.
2. Go module: copy to `PhenoKits/libs/go/pheno-core-cgo`, update `go.work`. 2 tool calls.
3. TypeScript: copy to `PhenoKits/libs/typescript/phenotype-core-ts`, rename `name` in `package.json`. 2 tool calls.
4. Python triage: parallel subagent, one pass per package to diff against existing homes and propose keep/fold/discard. 19 subagent dispatches.

**Estimated wall clock:** 15-25 minutes including Python triage.

## Decision required

- [ ] Approve Option A (consolidate into existing kits)
- [ ] Approve Option B (standalone repo)
- [ ] Approve Option C (discard all orphans)
- [ ] Mixed — specify per package
