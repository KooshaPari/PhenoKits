# Tasks: Org Sunset Maturity Program

## P0

- [ ] SUNSET-001: Create current repo-state ledger with all active/archived repos and maturity state.
- [x] SUNSET-002: Produce updated initial dispositions for `agentapi-plusplus`, `phenoSDK`, `PhenoProc`, `Dino`, `PhenoSpecs`, shelf root, and `AuthKit/go`.
  - Evidence: `worklogs/P0_DRIFT_DISPOSITION_2026_04_26.md`
- [ ] SUNSET-003: Decide `AuthKit/go` policy: submodule, vendored nested repo, or flattened directory.
- [ ] SUNSET-004: Create shared-crate canonical-home implementation PR for Phase 1 crates into `phenoShared`.
- [x] SUNSET-004A: Decide errors dual-interface strategy before implementing release prep.
  - Evidence: `docs/governance/adr-2026-04-26-errors-dual-interface-strategy.md`
- [x] SUNSET-004B: Define target file map and API surface for errors dual-interface implementation.
  - Evidence: `docs/governance/errors-dual-interface-target-state-2026-04-26.md`

## P1

- [ ] SUNSET-005: Inventory rulesets for all active repos and classify missing/weak/strong.
- [x] SUNSET-005A: Sample priority rulesets and record blocked vs actionable gaps.
  - Evidence: `docs/governance/ruleset-gap-ledger-2026-04-26.md`
- [ ] SUNSET-006: Apply governance baseline to priority rule-less repos after CI truth is known.
- [x] SUNSET-006A: Tighten `phenoShared` live ruleset without enabling self-blocking CODEOWNER review.
  - Evidence: `docs/governance/sunset-maturity-batch-3-execution-log-2026-04-26.md`
- [ ] SUNSET-007: Inventory stale no-PR branches and capture tip SHAs.
- [ ] SUNSET-008: Prune only merged or superseded stale branches.
- [x] SUNSET-008A: Inventory worktree/recovery containers before pruning.
  - Evidence: `worklogs/WORKTREE_QUARANTINE_LEDGER_2026_04_26.md`

## P2

- [ ] SUNSET-009: Consolidate dated worklogs into category worklogs.
- [ ] SUNSET-010: Review temporary/deleted governance docs for unique content before deletion.
- [ ] SUNSET-011: Resolve `Tracera` vs `Tracera-recovered` canonical checkout and document final routing.

## P3

- [ ] SUNSET-012: For each `SUNSET_READY` repo, add successor/canonical-home README note.
- [ ] SUNSET-013: Remove Dependabot configs before archive.
- [ ] SUNSET-014: Publish final org maturity ledger and acceptance evidence.
- [x] SUNSET-014A: Verify fresh PhenoKits clone for future governance work while shelf checkout remains quarantined.
  - Evidence: `docs/governance/sunset-maturity-batch-3-execution-log-2026-04-26.md`
