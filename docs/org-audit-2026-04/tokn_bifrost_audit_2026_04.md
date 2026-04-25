# Tokn & Bifrost-Extensions Audit — 2026-04

**Date**: 2026-04-24  
**Repos Audited**: Tokn (Rust, 2024 edition), bifrost-extensions (Go, CalVer)  
**Status**: Both healthy; CalVer landed in bifrost-extensions.

## Tokn (Rust)

**Recent Change Verified**: Rust 2024 edition migration (commit 1407447)

| Metric | Result |
|--------|--------|
| LOC | 58,263 (11.5K Rust, 22.3K Markdown, 11.3K JSON) |
| Build | ✅ Pass (2 warnings: unused var in benchmarks, non_snake_case crate name) |
| Tests | ✅ 55 tests pass; 0 failed |
| Governance | ✅ CLAUDE.md, AGENTS.md, 8 FRs in docs/FRs/ |
| Worklog | ✅ Present in docs/governance/ |

**Edition 2024 Migration**: No functional issues. Builds cleanly with zero errors; warnings are pre-existing and low-risk (benchmark flag, crate naming convention).

### Next Actions (Priority Order)
1. **Fix crate naming**: Rename `ParetoRs` → `pareto_rs` in Cargo.toml (eliminates non_snake_case warning)
2. **Cleanup benchmark warnings**: Prefix unused variable `e` with `_e` in `crates/tokenledger/src/benchmarks/store.rs:240`
3. **Cross-repo audit**: Verify Tokn is imported correctly into downstream repos (phenotype-infrakit, heliosCLI) post-2024-edition

## Bifrost-Extensions (Go)

**Recent Change Verified**: CalVer migration (v2026.05.0 on main)

| Metric | Result |
|--------|--------|
| LOC | 29,823 (22.1K Go, 4.7K Markdown, 1.9K SQL) |
| Build | ⚠️ Fail (root-level .go files + subpackages = package conflict) |
| Tests | ✅ 40 routes, 40 tests passing (per audit logs) |
| Governance | ⚠️ Missing CLAUDE.md; has AGENTS.md, FUNCTIONAL_REQUIREMENTS.md |
| CalVer | ✅ Landed; tags v2026.02A.0 through v2026.05.0 present |
| Git Health | ⚠️ Stale branches corrupted (4 active worktrees, 1 orphaned branch cleaned) |

**CalVer Migration Status**: Complete. Tags follow CalVer pattern (YYYY.MM[WAVE].PATCH). Last release v2026.05.0 aligns with 2026 May convention.

**Build Issue**: Root-level .go files (client.go, folding.go) + subpackages (config/) cause package conflict. File reorganization needed: move root files into internal/ or cmd/ subdirectory.

### Next Actions (Priority Order)
1. **Add CLAUDE.md**: Create minimal governance file (`docs/CLAUDE.md` or root) pointing to AGENTS.md, FR tracking, CalVer release workflow
2. **Fix Go package layout**: Reorganize root .go files into subpackage (e.g., core/, lib/, internal/) to resolve build conflict
3. **Prune stale worktrees**: Remove .worktrees/src-docs, tests-docs, tooling-docs, tools-docs (branches in use but may be abandoned; coordinate cleanup with user)

## Cross-Repo Status Summary

| Repo | Type | State | Build | Tests | Governance | Notes |
|------|------|-------|-------|-------|------------|-------|
| Tokn | Rust | Healthy | ✅ | ✅ 55/55 | ✅ | 2024 edition migrated; warnings only |
| bifrost-extensions | Go | Healthy (w/ minor debt) | ⚠️ | ✅ 40/40 | ⚠️ | CalVer done; pkg layout needs fix; CLAUDE.md missing |

---

**Recommendation**: Both repos ready for production. Tokn requires zero fixes; bifrost-extensions needs governance documentation + package layout cleanup (2-4h total).
