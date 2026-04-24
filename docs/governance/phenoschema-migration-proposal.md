# PhenoSchema — Migration Proposal for Orphaned ArgisRoute Content

**Date:** 2026-04-24
**Author:** audit agent (session #231 follow-up)
**Status:** Proposal — requires decision before execution

## Context

`repos/PhenoSchema/` was marked retired in audit #231. `pheno-xdd/` and `pheno-xdd-lib/` are cleanly published as `KooshaPari/phenoXdd` / `KooshaPari/phenoXddLib`. `Schemaforge/` docs are absorbed into `KooshaPari/PhenoDevOps/rust/forge/docs/absorbed/schemaforge/`. `schemas/` is docs-only. That leaves **ArgisRoute**.

See `repos/PhenoSchema/DEPRECATED.md` for the full migration map.

## Orphaned content: `ArgisRoute/`

```
ArgisRoute/
  ADR.md
  core/schemas/schemas.go    # single Go source file
  docs/
    ADR-001.md, ADR-002.md, ADR-003.md
    SOTA.md
    SPEC.md
    adr/
      ADR-001-architecture-overview.md
      ADR-002-technology-stack.md
      ADR-003-data-persistence.md
      ADR-004-error-handling.md
      ADR-005-integration-api.md
```

### Search results (verified 2026-04-24)

- `argisroute` appears only in **`KooshaPari/thegent` docs** (PRDs, debug tags, session notes from `20251210-argisroute-rebrand-model-management`).
- No code exists anywhere in the KooshaPari org for ArgisRoute.
- `schemas.go` is a single unique Go file with no counterpart.

## Proposal

### Option A (recommended): Fold into thegent/docs + thegent codebase

ArgisRoute is consistently referenced in thegent as "Go Services (ArgisRoute, ArgisHub)" in FR-52 and the Unified Work Stream. The specs belong with the consumer.

1. Move `ArgisRoute/docs/` → `KooshaPari/thegent/docs/specs/argisroute/` (preserve ADR-001..005, SOTA, SPEC).
2. Move `ArgisRoute/core/schemas/schemas.go` → `KooshaPari/thegent/services/argisroute/schemas/schemas.go` (or existing Go services tree).
3. Cross-link from `thegent/docs/specs/prds/API_prd.md` (already references `/argisroute/README.md`).

### Option B: Push as standalone `KooshaPari/ArgisRoute`

ArgisRoute is a named service with 5 ADRs and a SOTA doc — it has the shape of a standalone project. Creating `KooshaPari/ArgisRoute` preserves identity.

Trade-off: one Go file is thin for a standalone repo. Viable if ArgisRoute is expected to grow; otherwise adds a graveyard repo.

### Option C: Discard

Git history in `phenotype-infra` preserves recoverability. Only viable if ArgisRoute is truly dead — but thegent's PRDs actively reference it, so this contradicts current planning.

**Not recommended.**

## Recommendation

**Option A.** ArgisRoute lives inside thegent's Go services story per thegent's own PRDs. Fold docs + the single Go file into thegent rather than creating a ghost-town standalone repo.

## Execution plan (if Option A is approved)

Agent-driven, ~6-10 tool calls:

1. Copy `ArgisRoute/docs/` tree to `thegent/docs/specs/argisroute/`. 1 tool call.
2. Copy `schemas.go` to `thegent/services/argisroute/schemas/schemas.go` (verify no existing file). 2 tool calls.
3. Update thegent cross-links in `API_prd.md` and `UNIFIED_WORK_STREAM.md`. 2 tool calls.
4. Commit to thegent with provenance note. 1 tool call.
5. Delete ArgisRoute from PhenoSchema with reference to thegent commit. 1 tool call.

**Estimated wall clock:** 3-5 minutes.

## Decision required

- [ ] Approve Option A (fold into thegent)
- [ ] Approve Option B (standalone `KooshaPari/ArgisRoute`)
- [ ] Approve Option C (discard)
