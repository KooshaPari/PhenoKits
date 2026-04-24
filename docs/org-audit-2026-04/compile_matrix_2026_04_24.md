# Rust Workspace Compile Matrix — 2026-04-24

**Post-Alignment Status**: Wave-1 (tokio+serde) + Wave-2 (thiserror/clap/etc) complete.

## Summary

- **GREEN**: 9 repos compile cleanly
- **BROKEN**: 12 repos fail to compile
- **Total**: 21 Rust repos scanned

### Breakdown

#### GREEN (9/21 — 43%)

| Repo | Status | Notes |
|------|--------|-------|
| Observably | ✅ GREEN | Clean |
| phenotype-bus | ✅ GREEN | Clean |
| phenotype-tooling | ✅ GREEN | Clean |
| HeliosLab | ✅ GREEN | Clean |
| Eidolon | ✅ GREEN | Clean |
| AgilePlus | ✅ GREEN | Clean (24 crates) |
| rich-cli-kit | ✅ GREEN | Clean |
| Civis | ✅ GREEN | Clean |

#### BROKEN (12/21 — 57%)

| Repo | Exit | Error Category | Root Cause |
|------|------|-----------------|-----------|
| repos (root) | 101 | Unresolved import | `criterion` not in workspace dependencies |
| KDesktopVirt | 101 | Workspace conflict | Believes it's in workspace but isn't registered |
| PhenoVCS | 101 | Missing manifest targets | Workspace member `pheno-vcs-core` has no src/lib.rs |
| kmobile | 101 | Workspace conflict | Believes it's in workspace but isn't registered |
| PhenoPlugins | 101 | Missing manifest file | Workspace member `pheno-plugin-core` file not found |
| PhenoProc | 101 | Unresolved import | `phenotype_observability` traits not exported |
| Tracely | 101 | Unresolved import | `criterion` not in workspace dependencies |
| KlipDot | 101 | Workspace conflict | Believes it's in workspace but isn't registered |
| thegent-workspace | 101 | Missing manifest file | Workspace member `thegent-jsonl` file not found |
| PhenoObservability | 101 | Unresolved import | `criterion` not in workspace dependencies |
| HexaKit | 101 | Missing manifest file | Workspace member `phenotype-bdd` file not found |
| bare-cua | 101 | Type annotation missing | Generic type inference failure (E0282) |

## Error Patterns

### Pattern 1: Workspace Registration (3 repos)
- KDesktopVirt, kmobile, KlipDot
- **Fix**: Add to `repos/Cargo.toml` `[workspace] members` OR add empty `[workspace]` table to package manifest

### Pattern 2: Missing Manifest Files (3 repos)
- PhenoVCS, PhenoPlugins, thegent-workspace, HexaKit
- **Status**: Hard-blocked; crate directories don't exist or lack src/lib.rs

### Pattern 3: Missing Dev Dependency (3 repos)
- repos (root), Tracely, PhenoObservability
- **Fix**: Add `criterion` to workspace-level `[dev-dependencies]` OR individual Cargo.toml

### Pattern 4: Unresolved External Crate (1 repo)
- PhenoProc
- **Fix**: Verify `phenotype_observability` crate exports `init_tracer`, `increment_counter` traits

### Pattern 5: Type Inference (1 repo)
- bare-cua
- **Status**: Requires code fix; `cargo update` did not resolve

## Verification Run

All broken repos attempted `cargo update` before re-check; no additional resolution occurred beyond Pattern 3 (criterion) warnings.

## Recommendations

### Immediate (1-2h)
1. **Workspace registration**: Register KDesktopVirt, kmobile, KlipDot in repos/Cargo.toml or isolate them
2. **Add criterion**: Add to repos/Cargo.toml `[workspace] [dev-dependencies]` or drop from test suites
3. **Verify PhenoProc**: Check phenotype_observability exports; may need re-export pattern

### Medium-Term (3-5h)
4. **Reconstruct missing crates**: PhenoVCS/pheno-vcs-core, PhenoPlugins/pheno-plugin-core, thegent-workspace/thegent-jsonl, HexaKit/phenotype-bdd
5. **bare-cua type fix**: Localize E0282 error and add explicit type annotation

### Long-Term
6. Review integration points for inter-repo dependencies; consider shared workspace or dependency management plan

---

**Last Updated**: 2026-04-24 | **Auditor**: Claude Code (Haiku 4.5)
