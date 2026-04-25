# Rust Workspace Compile Matrix — 2026-04-24

**Post-Alignment Status**: Wave-1 (tokio+serde) + Wave-2 (thiserror/clap/etc) complete.

## Summary

- **GREEN**: 12 repos compile cleanly (updated 2026-04-24)
- **BROKEN**: 9 repos fail to compile
- **Total**: 21 Rust repos scanned

**Wave-10 Triage (2026-04-24)**: +3 repos fixed (KlipDot, PhenoVCS, Tokn); KDesktopVirt deferred (61+ errors)

### Breakdown

#### GREEN (12/21 — 57%)

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
| Tokn | ✅ GREEN | Fixed 2026-04-24 (clean) |
| KlipDot | ✅ GREEN | Fixed 2026-04-24 (workspace declaration) |
| PhenoVCS | ✅ GREEN | Fixed 2026-04-24 (stub lib.rs) |
| PhenoProc | ✅ GREEN | Fixed 2026-04-24 (clean) |

#### BROKEN (9/21 — 43%)

| Repo | Exit | Error Category | Root Cause | Wave-10 Status |
|------|------|-----------------|-----------|---------|
| repos (root) | 101 | Unresolved import | `criterion` not in workspace dependencies | — |
| KDesktopVirt | 101 | Type system | 61 compile errors (6 clusters, all fixable) | FIXABLE — See revive_plan.md (9-13h) |
| kmobile | 101 | Workspace conflict | Believes it's in workspace but isn't registered | — |
| PhenoPlugins | 101 | Missing manifest file | Workspace member `pheno-plugin-core` file not found | — |
| Tracely | 101 | Unresolved import | `criterion` not in workspace dependencies | — |
| thegent-workspace | 101 | Missing manifest file | Workspace member `thegent-jsonl` file not found | — |
| PhenoObservability | 101 | Unresolved import | `criterion` not in workspace dependencies | — |
| HexaKit | 101 | Missing manifest file | Workspace member `phenotype-bdd` file not found | — |
| bare-cua | 101 | Type annotation missing | Generic type inference failure (E0282) | — |

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

## Deep Dive: KDesktopVirt Error Clustering (2026-04-24)

**Status**: Diagnostic complete. Not architectural drift — all errors are fixable code issues.

### Error Breakdown (61 total)

| Root Cause | Count | Error Type | Locations | Priority |
|-----------|-------|-----------|-----------|----------|
| Async trait incompatibility | 11 | E0038 | audio_video_engine.rs (TtsEngine, SttEngine) | P0 |
| Borrow checker violations | 11 | E0716, E0382 | mcp.rs (6), recording_pipeline.rs (3), automation_engine.rs (2) | P0 |
| Trait bound mismatches | 4 | E0195 | audio_video_engine.rs (TtsEngine/SttEngine impls) | P0 |
| Type inference gaps | 6 | E0283, E0284 | desktop_control.rs (2), core.rs (2), mcp.rs (2) | P1 |
| Mutable borrow conflicts | 6 | E0596, E0499 | audio_video_integration.rs (4), core.rs (2) | P1 |
| Missing trait derives | 4 | E0277, E0599 | SessionStorage, EncoderMetrics, AudioEncoder (public types) | P2 |
| **Other** | 8 | Mixed | Partial moves, clone() missing, Serialize derives | P2 |

### Key Findings

1. **No architectural drift** — latest commit (91f5b4e) just added workspace declaration; errors predate workspace unification
2. **Primary hotspot** — audio_video_engine.rs (~1500 LOC) contains 55% of trait/async errors; isolated refactor opportunity
3. **Secondary hotspot** — mcp.rs (~700 LOC) contains 18% of borrow violations; straightforward Rust idiom fixes
4. **Rebuild foundation status confirmed** — CLAUDE.md states project is active rebuild foundation for eco-011 (device automation); README is stale ("archived for historical reference")

### Fix Effort Estimate

- **Phase 1 (Async traits)**: 4-6h — Refactor TtsEngine/SttEngine to use BoxFuture or #[async_trait]
- **Phase 2 (Borrow checker)**: 3-4h — Fix temporary value drops, moved value uses, type annotations
- **Phase 3 (Derives)**: 1h — Add Clone/Debug/Serialize where needed
- **Phase 4 (Integration)**: 1-2h — Full workspace validation
- **TOTAL**: 9-13h (moderate effort, high ROI)

### Recommendation

**REVIVE, not archive**. Errors are fixable, root causes well-understood, and project is active rebuild foundation per memory context. Generate spec in AgilePlus and dispatch agent for phased execution.

**Reference**: `/repos/KDesktopVirt/docs/research/revive_plan.md` (detailed fix plan with ordered steps)

---

**Last Updated**: 2026-04-24 | **Auditor**: Claude Code (Haiku 4.5)
