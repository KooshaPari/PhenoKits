# Duplication Worklog

Cross-repo duplication tracking for the Phenotype org shared-crate shelf.

---

## 2026-04-24 — 3-crate dedupe scoping

Audit of duplication for three canonical shared Rust crates (state-machine covered in a parallel audit). Scope: `/Users/kooshapari/CodeProjects/Phenotype/repos`, excluding `target/`, `.worktrees/`, `-wtrees/`, `.archive/`. No builds run.

### phenotype-event-sourcing

**Copies found (5 distinct directories, 4 actually source-bearing):**

| Path | Version | LOC | pub items | Last commit | Kind |
|------|---------|-----|-----------|-------------|------|
| `repos/crates/phenotype-event-sourcing` | `workspace` (0.1.0) | 1,816 | 37 | 2026-04-06 `feat(P2): fork strategy` | **Canonical** |
| `repos/PhenoProc/crates/phenotype-shared/crates/phenotype-event-sourcing` | `workspace` | 1,816 | 37 | 2026-04-02 | Bit-identical to canonical (diff -qr: empty) |
| `repos/PhenoProc/crates/phenotype-event-sourcing` | `0.2.0` | 57 | 3 | 2026-04-03 | **Stub** (mod decls only, no impls) |
| `repos/DataKit/rust/phenotype-event-sourcing` | `0.2.0` | 57 | 3 | 2026-04-05 | **Stub** (verbatim of the PhenoProc stub + `async_store` mod) |
| `repos/hwLedger/vendor/phenotype-event-sourcing` | `0.0.1` | 1,747 | 37 | vendored | **Divergent fork** — `event.rs`, `hash.rs`, `memory.rs`, `snapshot.rs` all differ vs canonical |

**Canonical home:** `repos/crates/phenotype-event-sourcing` is the live canonical. It is the most recently touched copy (2026-04-06) and is already wired into the root workspace `[workspace]` members and `[workspace.dependencies]`. The nested `PhenoProc/crates/phenotype-shared/crates/phenotype-event-sourcing` is bit-identical but **not** a git submodule — it was duplicated by copy during the 2026-04-02 `chore/infrastructure-standardization` merge and now lags behind canonical by one fork-strategy commit.

**API surface:** `error, event, hash, memory, snapshot, store` modules. 37 pub items across 6 modules. Re-exports: `EventSourcingError, EventEnvelope, compute_hash, detect_gaps, verify_chain, InMemoryEventStore, Snapshot, SnapshotConfig, EventStore`. Both canonical and the PhenoProc/phenotype-shared nested copy match exactly. The PhenoProc + DataKit stubs expose only module declarations (no types yet). hwLedger vendor diverges in 4 of 6 impl files — a non-trivial fork, likely ahead/behind on hash-chain semantics.

**Consumers:**
- `repos/hwLedger/crates/hwledger-ledger` → `phenotype-event-sourcing.workspace = true`, resolved via hwLedger's root `[workspace.dependencies]` pointing at `path = "vendor/phenotype-event-sourcing"`.
- `repos/crates/phenotype-application` — dep **commented out** with note "pre-existing compilation errors".
- `repos/PhenoProc/crates/phenotype-core` does **not** depend on it; only the stub sits in PhenoProc's workspace members.
- `repos/DataKit/rust` — stub is a workspace member but no consumer was found.

**Canonical recommendation:** keep `repos/crates/phenotype-event-sourcing`. Delete the PhenoProc/phenotype-shared nested copy (redundant). Delete the PhenoProc and DataKit stubs (they export nothing real and would immediately shadow the canonical if anyone added them as path deps). Resolve the hwLedger vendor fork by diffing its changes forward into canonical, then retargeting hwledger's workspace dep.

**Migration plan (DO NOT execute):**
- `repos/hwLedger/Cargo.toml` L145-149: replace `path = "vendor/phenotype-event-sourcing"` with `path = "../crates/phenotype-event-sourcing"` (after reconciling the fork); drop `vendor/phenotype-event-sourcing` from `members`.
- `repos/PhenoProc/crates/phenotype-shared/Cargo.toml` L8: remove `crates/phenotype-event-sourcing` from members; `rm -rf` the nested crate dir.
- `repos/PhenoProc/Cargo.toml` + `PhenoProc/crates/phenotype-event-sourcing`: if PhenoProc wants it, add `phenotype-event-sourcing = { path = "../crates/phenotype-event-sourcing" }` to `[workspace.dependencies]` and delete the stub. If it doesn't, just remove the stub + workspace member entry.
- `repos/DataKit/rust/Cargo.toml` L6: same — either retarget to canonical path or drop the member entry and remove the stub dir.

### phenotype-cache-adapter

**Copies found (3 source-bearing directories):**

| Path | Version | LOC | pub items | Last commit | Kind |
|------|---------|-----|-----------|-------------|------|
| `repos/crates/phenotype-cache-adapter` | `workspace` | 812 | 22 | 2026-04-06 | **Canonical** |
| `repos/PhenoProc/crates/phenotype-shared/crates/phenotype-cache-adapter` | `workspace` | 812 | 22 | 2026-03-27 | Bit-identical to canonical (diff -qr: empty) |
| `repos/PhenoProc/crates/phenotype-cache-adapter` | `0.2.0` | 78 | 2 | 2026-04-04 `fix: restore and fix PhenoProc workspace crates` | **Stub** (MetricsHook trait + CacheEntry only) |
| `repos/DataKit/rust/phenotype-cache-adapter` | `0.2.0` | 185 | — | 2026-04-05 | Small (stub-ish) |

Note: DataKit grep hit a separate `phenotype-cache-adapter` directory (185 LOC lib.rs) — larger than the PhenoProc stub but still far below canonical's 812 LOC; likely a partial port.

**Canonical home:** `repos/crates/phenotype-cache-adapter`. Most recent, wired into root workspace, matches the nested phenotype-shared copy byte-for-byte.

**API surface:** 22 pub items across the crate — two-tier cache (L1 LRU + L2 Moka/DashMap), TTL, metrics hooks. The PhenoProc/phenotype-cache-adapter stub exposes only `MetricsHook` trait and a private `CacheEntry` — no `Cache`, no tier API, would not satisfy consumers.

**Consumers:**
- `repos/PhenoProc/crates/phenotype-core/Cargo.toml` L53: `phenotype-cache-adapter = { workspace = true }`, resolving via PhenoProc root `[workspace.dependencies]` → `path = "crates/phenotype-cache-adapter"` (the **stub**). This is a latent runtime bug: `phenotype-core` is pointing at a stub that does not expose the expected surface. Either the `phenotype-core` import is currently unused or builds are broken locally.
- No consumer found under `repos/crates/`, hwLedger, AuthKit, or DataKit top-level.

**Canonical recommendation:** keep `repos/crates/phenotype-cache-adapter`. Delete both the nested phenotype-shared copy and the PhenoProc stub. Retarget `PhenoProc/crates/phenotype-core` to the canonical path.

**Migration plan (DO NOT execute):**
- `repos/PhenoProc/Cargo.toml` L70: change `phenotype-cache-adapter = { path = "crates/phenotype-cache-adapter" }` to `path = "../crates/phenotype-cache-adapter"`.
- `repos/PhenoProc/Cargo.toml` L21: remove `crates/phenotype-cache-adapter` from members; `rm -rf` the stub dir.
- `repos/PhenoProc/crates/phenotype-shared/Cargo.toml` L6: remove member entry; `rm -rf` the nested copy.
- `repos/DataKit/rust`: audit the 185-LOC copy — either finish the port (consuming canonical) or drop.

### phenotype-policy-engine

**Copies found (2 source-bearing directories — NAME COLLISION, not a true duplicate):**

| Path | Version | LOC | pub items | Last commit | Kind |
|------|---------|-----|-----------|-------------|------|
| `repos/crates/phenotype-policy-engine` | `workspace` | 1,402 | 14 | 2026-04-06 | **Canonical** (rule-based engine) |
| `repos/PhenoProc/crates/phenotype-shared/crates/phenotype-policy-engine` | `workspace` | — | — | 2026-03-27 | Nested copy (alongside bit-identical event-sourcing/cache-adapter; assume same pattern) |
| `repos/AuthKit/rust/phenotype-policy-engine` | `workspace` | 646 | 18 | 2026-04-05 | **Name collision** — completely different API (ABAC/RBAC-style: `Subject`, `Resource`, `Action`, `Decision`, `PolicyEngine` trait) |

**Canonical home:** `repos/crates/phenotype-policy-engine`. Exposes `context::EvaluationContext, engine::PolicyEngine (concrete), error::PolicyEngineError, loader, policy::Policy, result::{PolicyResult, Severity, Violation}, rule::{Rule, RuleType}` — a rule-based compliance/validation engine with TOML loader.

**AuthKit's crate is not a copy** — it shares the same crate name but implements an ABAC/RBAC authorization engine (`Subject`, `Resource`, `Action`, `AttributeValue`, `Decision`, `Condition`, `ComparisonOp`, `InMemoryPolicyEngine`, `RbacPolicyBuilder`). Zero overlap in type names. This is a naming bug: two different domains both claimed `phenotype-policy-engine`. If both ever end up in the same dependency graph, cargo will refuse to compile.

**Consumers:**
- `repos/PhenoProc/crates/phenotype-core/Cargo.toml` L50: `phenotype-policy-engine = { workspace = true }` → PhenoProc root `[workspace.dependencies]` L65 `path = "crates/phenotype-policy-engine"`. But there is **no** `PhenoProc/crates/phenotype-policy-engine` outside `phenotype-shared/crates/` — the path dep is broken unless `phenotype-shared` is being treated as the nested workspace. (The nested `phenotype-shared/crates/phenotype-policy-engine` was added in PR #56 on 2026-03-27.)
- `repos/AuthKit/rust/Cargo.toml` consumes its own local `phenotype-policy-engine` (collision variant).
- No hwLedger or DataKit consumer.

**Canonical recommendation:** keep `repos/crates/phenotype-policy-engine` as the canonical rule-based engine. **Rename AuthKit's crate** to `authkit-policy-engine` (or `phenotype-authz-engine`) — this is the highest-priority fix because the name collision will eventually break a cross-repo build. Delete the phenotype-shared nested copy.

**Migration plan (DO NOT execute):**
- `repos/AuthKit/rust/phenotype-policy-engine/Cargo.toml`: change `name = "phenotype-policy-engine"` to `name = "authkit-policy-engine"`; rename the directory. Update `AuthKit/rust/Cargo.toml` member entry and all local consumers.
- `repos/PhenoProc/Cargo.toml` L65: change path to `../crates/phenotype-policy-engine`; drop the stale `crates/phenotype-policy-engine` workspace member entry (L15).
- `repos/PhenoProc/crates/phenotype-shared/Cargo.toml` L10: remove member entry; `rm -rf` the nested copy.

### Cross-crate observations

- **PhenoProc is the top duplication source.** It hosts two parallel duplication vectors: (a) a `phenotype-shared/crates/` workspace that bit-copies `repos/crates/*` (3 crates, ~4,000 LOC of duplicated source), and (b) top-level stubs under `PhenoProc/crates/phenotype-{event-sourcing,cache-adapter}` (~135 LOC of broken stubs that `PhenoProc/crates/phenotype-core` actually imports). A single bulk-fix PR against `PhenoProc/` can eliminate all three crates' duplication plus the state-machine parallel.
- **`phenotype-shared` workspace verdict:** `repos/PhenoProc/crates/phenotype-shared/` is a real Cargo workspace (root `Cargo.toml` declares 11 member crates under `crates/`), **not a submodule** (`.gitmodules` has no entry), **not under active maintenance** (last commit 2026-04-02, two weeks stale relative to canonical `repos/crates/` at 2026-04-06). It was a snapshot copy that has now drifted. Treat it as deletable.
- **`find` for phenotype-shared directories:** only `repos/PhenoProc/crates/phenotype-shared` exists at depth ≤4. No other shared-workspace home. The canonical shelf at `repos/crates/*` is not named "phenotype-shared" but functions as one (root `repos/Cargo.toml` lists all four crates as workspace members).
- **Name collision risk:** `AuthKit/rust/phenotype-policy-engine` is the single most dangerous item in this audit — any future workspace that pulls both canonical and AuthKit via git-deps will fail to resolve. Fix before any other cross-repo consolidation.
- **hwLedger fork:** `vendor/phenotype-event-sourcing` is the only place with real divergence (4 impl files differ). Reconciliation requires a code review, not a path swap.

**Execution order (by risk × consumer count):**
1. **AuthKit policy-engine rename** (highest risk; 5-min crate-rename). Unblocks any future consolidation that touches AuthKit.
2. **PhenoProc stub deletion + retarget to `repos/crates/`** (fixes latent `phenotype-core` compile bug on cache-adapter + policy-engine). ~4 Cargo.toml edits, 3 `rm -rf`.
3. **Nested `PhenoProc/crates/phenotype-shared/crates/*` deletion** (zero consumers — cosmetic duplication only, but ~4,000 LOC of drifted source). Ship alongside #2 in the same PR.
4. **hwLedger vendor reconciliation** (requires diff review; last, because it needs design input on hash-chain semantics).
5. **DataKit stubs** (lowest priority — no consumers; just workspace clutter).

Files referenced:
- `/Users/kooshapari/CodeProjects/Phenotype/repos/Cargo.toml`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-{event-sourcing,cache-adapter,policy-engine}/Cargo.toml`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoProc/Cargo.toml`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoProc/crates/phenotype-shared/Cargo.toml`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoProc/crates/phenotype-core/Cargo.toml`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/hwLedger/Cargo.toml`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/hwLedger/vendor/phenotype-event-sourcing/src/{event,hash,memory,snapshot}.rs`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/AuthKit/rust/Cargo.toml`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/AuthKit/rust/phenotype-policy-engine/src/lib.rs`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/DataKit/rust/Cargo.toml`

---

## 2026-04-24 — AgilePlus core.proto 4-copy SSOT scoping

Scoping of audit #118 finding: AgilePlus has 4 parallel proto trees. Read-only, GH API only (repo `KooshaPari/AgilePlus`, HEAD tree).

### Inventory (SHA-256 = git blob SHA from GH tree API)

All four "contracts" trees contain the same 5 files (`agents, agileplus, common, core, integrations`). `proto/agileplus/v1/` omits `agileplus.proto` and has 4 files. Blob shas from `gh api repos/KooshaPari/AgilePlus/git/trees/HEAD?recursive=true`:

| Path | agents | agileplus | common | core | integrations |
|------|--------|-----------|--------|------|--------------|
| `kitty-specs/001-.../contracts/` | `5755ba8` (2514B) | `145de45` (4225B) | `7890303` (2206B) | `d80d948` (2226B) | `83a8fc5` (3614B) |
| `docs/specs/001-.../contracts/` | `5755ba8` | `145de45` | `7890303` | `d80d948` | `83a8fc5` |
| `.archive/kitty-specs/001-.../contracts/` | `5755ba8` | `145de45` | `7890303` | `d80d948` | `83a8fc5` |
| `proto/agileplus/v1/` | `1a13ab5` (2612B) | — (not present) | `5a4b480` (2028B) | `f789f2c` (3062B) | `f75af0c` (4214B) |

### Diff classification per unique filename

- **agents.proto** — 3-way IDENTICAL across kitty-specs/docs-specs/.archive (`5755ba8`, 2514B). `proto/` version `1a13ab5` (2612B) DIVERGENT — ~100B larger, implementation has evolved from spec.
- **agileplus.proto** — IDENTICAL across the 3 spec trees (`145de45`). **Absent from `proto/`** entirely. This file is spec-only; never codegen-consumed. Likely a historical umbrella service stub.
- **common.proto** — IDENTICAL across 3 spec trees (`7890303`, 2206B). `proto/` version (`5a4b480`, 2028B) DIVERGENT — smaller, refined.
- **core.proto** — IDENTICAL across 3 spec trees (`d80d948`, 2226B). `proto/` version (`f789f2c`, 3062B) DIVERGENT — ~40% larger, grew with implementation.
- **integrations.proto** — IDENTICAL across 3 spec trees (`83a8fc5`, 3614B). `proto/` version (`f75af0c`, 4214B) DIVERGENT — ~16% larger.

**Summary:** Not a 4-copy problem — it's a **2-variant × multi-location** problem. Three locations are bit-identical (spec frozen). `proto/` is the evolved implementation, structurally similar but distinct API surface.

### Canonical recommendation

**`proto/agileplus/v1/` is canonical SSOT.** Rationale:
1. All build consumers already point there (see below).
2. `buf.yaml` declares `modules: [path: proto]`.
3. `buf.gen.yaml` generates Rust/Python stubs from `proto/` into `rust/src/gen` and `python/src/agileplus_proto/gen`.
4. Spec-kit narrative in `WP00-proto-scaffold.md` explicitly states `kitty-specs/.../contracts/` is **"the canonical proto source of truth to copy and adapt"** into `proto/agileplus/v1/core.proto`. I.e., `kitty-specs/contracts` is the **design spec snapshot**; `proto/` is the **living contract**. The spec has served its purpose.

**Disposition per path:**
- `proto/` → **canonical, keep as-is**.
- `kitty-specs/001-.../contracts/` → **freeze + demote** to historical design artifact. Either (a) delete after tagging `spec-001-frozen` for archeology, or (b) replace with a `README.md` pointer: "Original design; evolved impl at `/proto/agileplus/v1/`."
- `docs/specs/001-.../contracts/` → **regenerate from canonical** via CI (same pattern as Tracera OpenAPI-commit). Doc-site copy should be a downstream artifact, not a hand-edited source.
- `.archive/kitty-specs/001-.../contracts/` → **delete outright** per `.archive/` pruning policy. It's an archived-then-snapshotted copy of an already-frozen spec. Pure dead weight.

### Consumers affected

Build-config references to `.proto` paths (GH search + direct fetch of `build.rs` / `buf.*`):

| Consumer | References | Needs change? |
|----------|-----------|---------------|
| `crates/agileplus-grpc/build.rs` | `../../proto/agileplus/v1/{core,agents,common,integrations}.proto`, includes `../../proto` | **No** — already canonical |
| `agileplus-agents/crates/agileplus-agent-service/build.rs` | computes repo-root `proto/agileplus/v1/agents.proto` | **No** — already canonical |
| `rust/build.rs` | `../proto/agileplus/v1/{common,core,agents,integrations}.proto` | **No** — already canonical |
| `buf.yaml` | `modules: [path: proto]` | **No** |
| `buf.gen.yaml` | outputs from `proto/` to `rust/src/gen`, `python/src/agileplus_proto/gen` | **No** |
| `kitty-specs/.../tasks/WP00-proto-scaffold.md` | narrative reference to `contracts/*.proto` as "copy-from source" | **Update** — describe migration from design spec to living proto |
| `docs/specs/.../tasks/WP00-proto-scaffold.md` | same narrative | **Update or regenerate** |
| `.archive/kitty-specs/.../tasks/WP00-proto-scaffold.md` | same narrative | Deleted with archive |

**Count: 0 build consumers need path changes**, 2-3 narrative docs need updates. This is purely a spec/docs cleanup — no codegen risk.

### Migration plan (stacked PRs, DO NOT execute yet)

**PR 1 — Delete `.archive/kitty-specs/001-.../contracts/`** (and the rest of that archived feature if policy allows).
- Risk: none. `.archive/` is explicitly dead per repo policy.
- Verification: `grep -r ".archive/kitty-specs/001" --include="*.rs" --include="*.toml" --include="*.yaml"` → expect zero hits.

**PR 2 — Freeze `kitty-specs/001-.../contracts/` as historical.**
- Replace the 5 `.proto` files with a single `contracts/README.md` containing: blob-SHA manifest of the frozen spec (already captured above), pointer to `proto/agileplus/v1/` for live schemas, and a note that `agileplus.proto` (umbrella) was dropped during implementation.
- Optionally: tag `git tag spec-001-frozen <commit>` before removal for archeological recoverability.
- Update `kitty-specs/001-.../tasks/WP00-proto-scaffold.md` to past-tense ("WP00 completed; see `/proto/agileplus/v1/`").
- Risk: low. Spec-kit tooling may expect `contracts/` to exist — verify `agileplus` CLI behavior on a frozen-spec directory before landing.

**PR 3 — Auto-regen `docs/specs/001-.../contracts/` from `/proto/` via CI.**
- Add a CI job (Rust binary per Phenotype scripting policy; no shell) that on push-to-main runs `cp proto/agileplus/v1/*.proto docs/specs/001-.../contracts/` (or better: generates a rendered `.md` view per proto via `protoc --doc_out=`). Commit back via bot.
- Pattern: same as Tracera OpenAPI-commit flow.
- Alternative: delete `docs/specs/001-.../contracts/` entirely and have the doc-site ingest from `/proto/` at render time (VitePress can read .proto files directly via a loader).
- Risk: doc-site build must understand new location. Validate locally before merging.

**PR 4 (optional) — Publish `proto/` as a buf.build module.**
- Push `proto/` to `buf.build/kooshapari/agileplus` so downstream Phenotype repos (cliproxyapi-plusplus, agentapi-plusplus) can consume via `buf` dependency rather than git submodule.
- Out of scope for this SSOT cleanup; tracked separately.

### Risks

- **prost-build cache** — none. Build scripts reference `proto/` only; no cache invalidation needed as files aren't moving.
- **doc-site regen** — PR 3 is the only real risk. Needs validation that VitePress/docsite build tolerates regenerated `.proto` files (or can render from `/proto/` directly).
- **Consumer stubs** — `rust/src/gen/` and `python/src/agileplus_proto/gen/` are codegen outputs from `proto/`; unaffected.
- **Spec-kit tooling** — `agileplus` CLI may expect `kitty-specs/<id>/contracts/*.proto`. Test `agileplus status 001`, `agileplus validate 001`, and `agileplus accept 001` after PR 2 before merging.
- **Cross-repo importers** — none found. Only `KooshaPari/AgilePlus` repo contains these protos; no external Phenotype repo imports them via path.

### Files referenced

- `repos/AgilePlus/proto/agileplus/v1/{agents,common,core,integrations}.proto` (canonical SSOT)
- `repos/AgilePlus/kitty-specs/001-spec-driven-development-engine/contracts/*.proto` (frozen design spec, 5 files)
- `repos/AgilePlus/docs/specs/001-spec-driven-development-engine/contracts/*.proto` (doc-site mirror, should be regenerated)
- `repos/AgilePlus/.archive/kitty-specs/001-spec-driven-development-engine/contracts/*.proto` (delete)
- `repos/AgilePlus/crates/agileplus-grpc/build.rs`
- `repos/AgilePlus/agileplus-agents/crates/agileplus-agent-service/build.rs`
- `repos/AgilePlus/rust/build.rs`
- `repos/AgilePlus/buf.yaml`, `repos/AgilePlus/buf.gen.yaml`
