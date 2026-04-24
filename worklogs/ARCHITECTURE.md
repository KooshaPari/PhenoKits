# ARCHITECTURE — Worklog
**Category: ARCHITECTURE**

Architecture decisions, library extraction candidates, and large-scale refactoring plans.

## 2026-04-24 — Rust workspace consolidation analysis

**Scope:** Active Rust workspaces across 30+ Phenotype repos.

**Findings:**
- **12 active workspaces** totaling ~1.7M LOC
- **2 large monoliths** (FocalPoint 739K/57 crates, AgilePlus 693K/52 crates) — intentionally standalone
- **1 medium workspace** (PhenoObservability 213K/10 crates) — mature, production observability
- **5 small cohesive** (Stashly, Eidolon, Observably, thegent-workspace, Sidekick) — generic infrastructure or platform-specific
- **4 singletons** (PlayCua, bare-cua, thegent-dispatch, kmobile) — single crate, platform-specific

**Consolidation opportunity (HIGH):** `Observably` (3 crates, 1.1K LOC) **→ PhenoObservability**
- Duplicate domains: both provide tracing, logging, sentinel/monitoring
- Located in separate repos with different namespaces (observably-* vs pheno-tracing, pheno-questdb, etc.)
- PhenoObservability is larger, more mature, production-focused
- Same core deps: tokio 1.39, serde 1.0, thiserror 2.0, anyhow 1.0, tracing 0.1 (no conflicts)
- **Benefit:** Unified release cycle, single integration surface, shared CI/CD pipeline
- **Effort:** 1-2 days (4 crate moves, namespace consolidation, cross-repo dep updates)

**Anti-consolidations (CORRECT SEPARATION):**
- **Device automation family** (Eidolon, kmobile, PlayCua, bare-cua, KVirtualStage): Platform-specific dependencies warrant isolation. Consolidation would bloat builds (200MB+), couple independent release cycles, and complicate CI parallelization.
- **FocalPoint/AgilePlus:** Core products, large domains, need independent versioning + release cycles
- **Stashly:** Generic infrastructure library (cache, event store, state machine); reusable across projects; correct to keep standalone

**Result:** Reduce 12 → 11 workspaces by merging Observably; keep device automation & core products intentionally separate.

**See:** `docs/org-audit-2026-04/workspace_consolidation_map.md` for full consolidation matrix, dependency alignment, and phase-based implementation plan.

## 2026-04-24 — Large-file decomposition audit

**Scope:** Rust/Go/Python source across 20 top-tier Phenotype-org repos.
**Thresholds:** hard limit 500 LOC, target 350 LOC.
**Exclusions:** `target/`, `.worktrees/`, `-wtrees/`, `.archive/`, `vendor/`, `node_modules/`, `.git/`, `.venv/`, `.history/`, `dist/`, `build/`, `__pycache__/`, `/.agileplus/plane/` (vendored Plane fork), `/sidecars/omlx-fork/` (vendored MLX fork), `/original_source/` (pre-migration copies), `.claude/worktrees/`, `*.pb.go`, `*_pb2.py`, `queries.sql.go`, django migrations, `mcp-forge/internal/protocol/ts{protocol,json}.go` (Microsoft LSP port), and any file whose first 3 lines contain `Code generated`, `DO NOT EDIT`, `@generated`, or `AUTO-GENERATED`.

**Totals (post-filter):**
- Files >500 LOC: **908**
- Files 350–500 LOC: **830**
- Total >350 LOC: **1,738**

### Per-repo summary

| Repo | >500 LOC | 350–500 LOC | Top offender (path, LOC) |
|---|---:|---:|---|
| AgilePlus | 23 | 20 | `crates/agileplus-dashboard/src/routes.rs` (2640) |
| AuthKit | 9 | 6 | `rust/phenotype-security-aggregator/src/lib.rs` (781) |
| BytePort | 1 | 2 | `backend/nvms/lib/aws.go` (547) |
| Civis | 0 | 1 | `crates/engine/src/engine.rs` (395) |
| DataKit | 0 | 7 | `python/pheno-caching/src/pheno_caching/cold/disk_cache.py` (462) |
| Dino | 1 | 5 | `src/Tools/DinoforgeMcp/dinoforge_mcp/server.py` (1330) |
| GDK | 6 | 3 | `src/quality_metrics.rs` (971) |
| HeliosLab | 2 | 1 | `pheno-db/src/lib.rs` (720) |
| McpKit | 11 | 9 | `rust/mcp-forge/internal/utilities/edit_test.go` (1116) |
| PhenoKits | 25 | 37 | `HexaKit/agileplus/crates/agileplus-dashboard/src/routes.rs` (2640) |
| PhenoObservability | 14 | 22 | `ObservabilityKit/python/pheno-monitoring/src/pheno_monitoring/analytics_dashboard.py` (1439) |
| PhenoPlugins | 2 | 2 | `crates/pheno-plugin-git/src/lib.rs` (534) |
| PhenoProc | 95 | 147 | `python/pheno-observability/src/pheno_observability/metrics/advanced.py` (1167) |
| ResilienceKit | 6 | 5 | `python/pheno-deploy/src/pheno_deploy/cli.py` (1821) |
| Tracera | 378 | 270 | `tests/integration/repositories/test_repositories_core_full_coverage.py` (4032) |
| cliproxyapi-plusplus | 92 | 77 | `test/thinking_conversion_test.go` (3641) |
| hwLedger | 24 | 19 | `crates/hwledger-ffi/src/lib.rs` (1284) |
| phenotype-journeys | 4 | 1 | `crates/phenotype-journey-core/src/pipeline.rs` (1066) |
| thegent | 211 | 196 | `crates/thegent-hooks/src/main.rs` (5468) |
| phenotype-infrakit | 0 | 0 | — (no Rust/Go/Py source matching audit path) |

### Top-20 cross-repo decomposition candidates

Priority = LOC × (top-level items ÷ 10). Files with ≤2 items dropped (single-megablock refactor class, not multi-module split). `.../routes.rs` appears twice because PhenoKits vendors a copy of the AgilePlus dashboard — fix-once upstream and re-vendor.

| # | Priority | LOC | Items | Repo | Path |
|--:|--:|--:|--:|---|---|
| 1 | 62,899 | 4032 | 156 | Tracera | `tests/integration/repositories/test_repositories_core_full_coverage.py` |
| 2 | 44,291 | 5468 | 81 | thegent | `crates/thegent-hooks/src/main.rs` |
| 3 | 28,489 | 3201 | 89 | Tracera | `src/tracertm/api/routers/item_specs.py` |
| 4 | 23,566 | 2266 | 104 | cliproxyapi-plusplus | `pkg/llmproxy/config/config.go` |
| 5 | 23,496 | 2640 | 89 | AgilePlus | `crates/agileplus-dashboard/src/routes.rs` |
| 6 | 23,496 | 2640 | 89 | PhenoKits | `HexaKit/agileplus/crates/agileplus-dashboard/src/routes.rs` (mirror of #5) |
| 7 | 22,126 | 2405 | 92 | thegent | `src/thegent/phench/service.py` |
| 8 | 21,774 | 3202 | 68 | thegent | `hooks/hook-dispatcher/src/main.rs` |
| 9 | 17,003 | 3334 | 51 | Tracera | `tests/integration/api/test_api_layer_full_coverage.py` |
| 10 | 16,340 | 3026 | 54 | cliproxyapi-plusplus | `pkg/llmproxy/api/handlers/management/auth_files.go` |
| 11 | 15,346 | 3009 | 51 | Tracera | `tests/unit/repositories/test_specification_repository.py` |
| 12 | 14,872 | 2187 | 68 | thegent | `tests/commands/test_apps_main.py` |
| 13 | 14,628 | 2120 | 69 | PhenoKits | `HexaKit/tests/test_phench_runtime.py` |
| 14 | 14,566 | 2111 | 69 | thegent | `tests/test_phench_runtime.py` |
| 15 | 13,353 | 1993 | 67 | thegent | `tests/test_e2e_cli_aliases.py` |
| 16 | 12,818 | 2465 | 52 | thegent | `tests/test_unit_cli_coverage_c.py` |
| 17 | 12,512 | 2720 | 46 | Tracera | `src/tracertm/services/spec_analytics_service.py` |
| 18 | 11,851 | 1693 | 70 | cliproxyapi-plusplus | `pkg/llmproxy/watcher/watcher_test.go` |
| 19 | 11,134 | 1713 | 65 | Tracera | `tests/unit/repositories/test_run_repository.py` |
| 20 | 10,950 | 1825 | 60 | Tracera | `tests/integration/repositories/test_repositories_integration.py` |

### Decomposition hints (top-10)

Non-test production entries, plus the highest-impact test to show the pattern.

1. **`Tracera/tests/integration/repositories/test_repositories_core_full_coverage.py`** (4032 / 156) — omnibus repository-coverage test. Cluster by repository-under-test: `test_repositories_core/{test_item_spec_repo,test_blockchain_repo,test_run_repo,test_integration_repo,test_spec_repo}.py`. Share fixtures via `tests/integration/repositories/conftest.py`.

2. **`thegent/crates/thegent-hooks/src/main.rs`** (5468 / 81) — clap subcommand hub. 30+ `cmd_*` handlers coexist with `BreakerState`/`DebounceState`/`Manifest`/cache helpers. Split into `main.rs` (clap dispatch only) + modules: `commands/quality.rs` (`cmd_quality_gate`, `cmd_security_pipeline`, `cmd_complexity_ratchet`), `commands/cache.rs` (cache_* + `ensure_cache_dir`), `commands/breaker.rs` (`cmd_breaker_*`), `commands/reconcile.rs` (`cmd_stop_reconcile`, `cmd_teammate_*`, `cmd_harvest`), `commands/agileplus.rs` (`cmd_agileplus_cycle`, `cmd_friction_detect`, `cmd_task_completed`), `state/manifest.rs` (`Manifest`, `FileManifest`), `state/breaker.rs` (`BreakerState`, `DebounceState`).

3. **`Tracera/src/tracertm/api/routers/item_specs.py`** (3201 / 89) — single FastAPI router bundling six spec types. Split by item type: `routers/item_specs/{requirement,test,epic,user_story,task,defect,stats}.py`; parent `routers/item_specs/__init__.py` re-exports APIRouter union. Pydantic models already named by type prefix — trivial partition.

4. **`cliproxyapi-plusplus/pkg/llmproxy/config/config.go`** (2266 / 104) — config loader + sanitizers + env overrides in one file. Split: `config/load.go` (`LoadConfig`, `LoadConfigOptional`, `ApplyEnvOverrides`), `config/sanitize.go` (`Sanitize*` family for Codex/Claude/Kiro/Cursor/Gemini/OpenAI), `config/oauth.go` (`SanitizeOAuth*`, `OAuthUpstreamURL`), `config/save.go` (`SaveConfigPreserveComments*`), `config/providers.go` (`GetAPIKey`/`GetBaseURL`/`GetName`/`GetProviderByName`/`InjectPremadeFromEnv`).

5. **`AgilePlus/crates/agileplus-dashboard/src/routes.rs`** (2640 / 89) — 53 handlers + config structs + HTML helpers. Split: `routes/mod.rs` (Axum wiring), `routes/dashboard.rs` (home/root/kanban), `routes/projects.rs` (project summaries + features), `routes/plane.rs` (plane sync + health endpoints), `routes/settings.rs` (`*SettingsForm` handlers), `routes/evidence.rs` (evidence/artifact/media), `util/html.rs` (`html_escape`, `percent_encode_path`, `is_htmx`, `artifact_type_for_ext`), `config.rs` (`DashboardConfig`, `PlaneConfig`, `AgentConfig`, `ServiceConfig`). **Note: PhenoKits mirrors this file — fix upstream then re-vendor.**

6. **`PhenoKits/HexaKit/agileplus/crates/agileplus-dashboard/src/routes.rs`** — mirror of #5. Resolve as a single upstream refactor + re-sync, not a second refactor.

7. **`thegent/src/thegent/phench/service.py`** (2405 / 92) — phench module orchestrator. Split by verb-object grouping: `phench/service/target.py` (`init_target`, `bootstrap_target`, `load_target_lock`, `lock_target`, `materialize_target`, `target_status`, `target_timeline`, `list_targets`, `_list_targets_in_root`), `phench/service/snapshot.py` (`create_target_snapshot`, `list_target_snapshots`, `show_target_snapshot`, `_snapshot_id`, `_stable_payload_hash`), `phench/service/repos.py` (`import_repos`, `discover_repos`, `set_repo_ref`, `add_repo`), `phench/service/modules.py` (`add_module_to_target`, `audit_shared_modules*`, `sync_project_modules_from_repos`, `build_catalog`, `list_modules`), `phench/service/env.py` (`set_env_profile`, `get_env_profile`, `_run_env_doctor_for_materializations`), `phench/service/exec.py` (`_run_single_repo_target`, `_resolve_repo_runner_and_command`, `build_project_execution_matrix`), `phench/service/_manifest.py` (underscored helpers).

8. **`thegent/hooks/hook-dispatcher/src/main.rs`** (3202 / 68) — secrets scanner + governance scanner + worker pool + hook runners all in one `main`. Split: `main.rs` (clap dispatch), `scan/secrets.rs` (`SecretPattern`, `SecretMatch`, `scan_content_for_secrets`, `get_named_secret_patterns`, `mask_secret`), `scan/governance.rs` (`GovernanceViolation`, `scan_noqa_violations`, `scan_todo_no_ticket_violations`, `scan_function_length_violations`, `scan_hardcoded_cred_violations`), `scan/spiral.rs` (`Spiral*` types + trend/selector helpers), `runner/pool.rs` (`Worker`, `WorkerPool`, `TempFile` + `Drop`), `commands/governance.rs` (`cmd_governance`, `cmd_scan_secrets`), `commands/hooks.rs` (`run_doc_location_guard`, `run_session_cleanup`, `run_prompt_submit_guard`, `run_governance_scan`).

9. **`Tracera/tests/integration/api/test_api_layer_full_coverage.py`** (3334 / 51) — omnibus API test. Split by router: `test_api_layer/{test_item_specs,test_links,test_traces,test_sync,test_export,test_config}.py`; shared fixtures in `tests/integration/api/conftest.py`.

10. **`cliproxyapi-plusplus/pkg/llmproxy/api/handlers/management/auth_files.go`** (3026 / 54) — CRUD + many provider `Request*Token` handlers + callback forwarder. Split: `auth_files/crud.go` (`ListAuthFiles`, `GetAuthFileModels`, `DownloadAuthFile`, `UploadAuthFile`, `DeleteAuthFile`, `PatchAuthFile*`), `auth_files/callback.go` (`startCallbackForwarder`, `stopCallbackForwarder*`, `validateCallbackForwarderTarget`, `managementCallbackURL`), `auth_files/token_store.go` (`tokenStoreWithBaseDir`, `saveTokenRecord`, `deleteTokenRecord`, `authIDForPath`), `auth_files/providers.go` (`RequestAnthropicToken`, `RequestGeminiCLIToken`, `RequestCodexToken`, `RequestAntigravityToken`, `RequestQwenToken`, `RequestKimiToken`, `RequestIFlowToken`, `RequestGitHubToken`), `auth_files/codex_claims.go` (`extractCodexIDTokenClaims`, `authEmail`, `authAttribute`).

### Execution order

**Phase A — high-public-impact entry points (ship first):**
1. AgilePlus `routes.rs` + PhenoKits mirror (#5/#6) — user-facing dashboard, downstream consumers import route modules.
2. cliproxyapi-plusplus `config.go` (#4) — every executor/handler imports config.
3. cliproxyapi-plusplus `auth_files.go` (#10) — management API surface.
4. Tracera `item_specs.py` router (#3) — public REST surface.
5. thegent `thegent-hooks/main.rs` (#2) + `hook-dispatcher/main.rs` (#8) — CLI entry points; binary still ships same commands, internal-only refactor.

**Phase B — private internals:**
6. Tracera `spec_analytics_service.py` (#17) — 40+ domain types + analyzers; split by analysis concern (EARS, ODC, flakiness, WSJF/RICE, impact).
7. thegent `phench/service.py` (#7) — private orchestrator, no external import surface.

**Phase C — test file splits (low risk, high LOC win):**
8. Tracera test megafiles (#1, #9, #11, #19, #20) — zero downstream impact, pure organizational refactor.
9. thegent test megafiles (#12–#16) — same.

**Easiest first target:** Tracera `item_specs.py` router (#3). Pydantic model names already prefixed by item type (`Requirement*`, `Test*`, `Epic*`, `UserStory*`, `Task*`, `Defect*`) — near-mechanical split, FastAPI router union keeps external routes stable.

**Hardest:** thegent `thegent-hooks/main.rs` (#2). 81 items, 30+ clap subcommands, shared state structs (`BreakerState`, `DebounceState`, `Manifest`, `FileManifest`) used cross-command — requires a `state/` module with careful borrow/lifetime design before commands can move out. Also upstream-facing CLI; argv contract must not drift.

**Not in scope (different refactor class, items ≤ 2):** megablock `impl` files and single-function modules — these need intra-function extraction, not file-splitting PRs.

## 2026-04-24 — Workspace manifest drift inventory

**Scope:** All Cargo workspace manifests under `/Users/kooshapari/CodeProjects/Phenotype/repos/` (max-depth 4, excluding `target/`, `.worktrees/`, `-wtrees/`, `.archive/`, `node_modules/`). 48 workspace roots discovered, 47 auditable (1 TOML parse skip).

**Purpose:** Unblock Dependabot Rust update runs that fail because `cargo metadata` cannot resolve phantom `members = [...]` entries, or because git-dep URLs point to non-existent `KooshaPari/*` repos.

**Methodology:** Read-only — parsed each workspace root with `tomllib`, resolved explicit member paths against filesystem, scanned all child `Cargo.toml` files for `git = "..."` deps referencing `KooshaPari/`, checked each candidate via `gh repo view` (cached). No edits, no builds.

**Classification thresholds:**
- **Clean** — all members resolve, all `KooshaPari/*` git deps resolve
- **Minor drift** — 1–3 phantom members
- **Major drift** — ≥ 4 phantom members
- **Git-dep drift** — 0 phantom members but ≥ 1 phantom git dep

### Summary counts

| Classification | Count |
|----------------|------:|
| Clean          |    39 |
| Minor drift    |     5 |
| Major drift    |     4 |
| Git-dep drift  |     1 |
| **Total drifted** | **10** |

### Drift table

| Repo (ws root) | Classification | Phantom Members | Phantom Git Deps | Last-touched (workspace Cargo.toml) |
|---|---|---:|---:|---|
| `AgilePlus` | Minor drift | 1 | 3 | `1aa0ab0` 2026-03-31 — feat(dashboard): implement Phase 2 web dashboard |
| `BytePort` | Minor drift | 2 | 0 | (untracked in parent `repos` git) |
| `.` (repos root) | Git-dep drift | 0 | 3 | `3cb60471a` 2026-04-02 — feat: apply governance standards (#73) |
| `HexaKit` | Major drift | 19 | 0 | (untracked; no embedded `.git`) |
| `PhenoKits/HexaKit` | Major drift | 19 | 0 | `6016c04` 2026-04-02 — chore: apply phenotype governance standards |
| `PhenoLang-actual` | Major drift | 11 | 1 | `28a6f36` 2026-04-02 — chore: commit all pending changes across workspace |
| `PhenoLang-actual/phenotype-infrakit` | Major drift | 15 | 0 | `8e5a941` 2026-04-02 — ci: add Cargo.toml and quality-gate.sh blocked by gitignore |
| `PhenoLibs/rust` | Minor drift | 2 | 0 | `0f5e653` 2026-04-05 — Initial commit: PhenoKit - 2026-04-05 |
| `PhenoPlugins` | Minor drift | 3 | 0 | `bf3a79b` 2026-04-03 — Add phenoVessel as pheno-plugin-vessel crate |
| `thegent-workspace` | Minor drift | 2 | 0 | `6a221db` 2026-04-05 — feat: initialize TheGent workspace with jsonl and utils crates |

**Most recent drift-introducing commit:** `1aa0ab0` (2026-03-31) in AgilePlus — post-Phase-2 dashboard landing left `crates/agileplus-fixtures` on the members list without the crate directory being committed. All other drift predates 2026-04-06.

### Per-repo detail

#### AgilePlus — Minor drift
- **Phantom members (1):**
  - `crates/agileplus-fixtures`
- **Phantom git deps (3):** (from child crates referencing org-external plugin crates that were never extracted)
  - `https://github.com/KooshaPari/agileplus-plugin-core` (404)
  - `https://github.com/KooshaPari/agileplus-plugin-git` (404)
  - `https://github.com/KooshaPari/agileplus-plugin-sqlite` (404)
- **Last touched:** `1aa0ab0` 2026-03-31 — feat(dashboard): Phase 2
- **Fix strategy:** **Prune.** The `agileplus-fixtures` extraction never landed; drop from members. For the git deps, either (a) publish stub crates at those URLs, or (b) relocate consumers to use `libs/plugin-*` from this same workspace (those do exist — `libs/plugin-registry`, `libs/plugin-sample`, `libs/plugin-cli`, `libs/plugin-git`). Strategy (b) is almost certainly correct — the `agileplus-plugin-*` names look like a pre-rename leftover.
- **Note:** Original task description claimed "~50 phantom members" — actual count is **1**. Earlier cleanup must have already landed; only one fixture crate reference remains. Re-verify task #79 closeout.

#### BytePort — Minor drift
- **Phantom members (2):**
  - `backend/byteport`
  - `backend/nvms`
- **Last touched:** untracked in parent `repos` git index; this appears to be a scaffold that was never committed
- **Fix strategy:** **Prune** if BytePort is scaffolding-only, or **Restore** by populating `backend/byteport/` and `backend/nvms/` with crate skeletons. Needs project-owner decision.

#### `.` (repos root workspace) — Git-dep drift
- **Phantom git deps (3):** same three `agileplus-plugin-*` URLs as AgilePlus
- **Last touched:** `3cb60471a` 2026-04-02 — feat: apply governance standards (#73)
- **Fix strategy:** **Prune + repoint.** Same as AgilePlus — these references propagated into the org-root workspace during governance sweep. Repoint to in-workspace `libs/plugin-*` paths.

#### HexaKit — Major drift
- **Phantom members (19):** `crates/phenotype-bdd`, `crates/phenotype-config-core`, `crates/phenotype-core`, `crates/phenotype-event-bus`, `crates/phenotype-compliance-scanner`, `crates/phenotype-infrastructure`, `crates/phenotype-project-registry`, `crates/phenotype-security-aggregator`, `crates/phenotype-mock`, `crates/phenotype-test-fixtures`, `crates/phenotype-testing`, `Logify`, `Metron`, `Tasken`, `Eventra`, `Traceon`, `Stashly`, `Settly`, `Authvault`
- **Last touched:** no embedded `.git`; not in parent `repos` git index either (directory is untracked)
- **Fix strategy:** **Restore or delete wholesale.** This is an incomplete extraction scaffold — same 19 phantoms as PhenoKits/HexaKit, suggesting HexaKit was copy-pasted as a workspace prototype. Confirm HexaKit's purpose; if it's a planning stub, gitignore it. If it's meant to be real, decide per-member whether to restore or prune.

#### PhenoKits/HexaKit — Major drift
- **Phantom members (19):** identical set to HexaKit/ above
- **Last touched:** `6016c04` 2026-04-02 — chore: apply phenotype governance standards
- **Fix strategy:** **Restore or Prune, mirroring HexaKit decision.** The two HexaKit workspaces are clearly forks of the same scaffold. The 11 `crates/phenotype-*` phantoms (bdd, config-core, core, event-bus, compliance-scanner, infrastructure, project-registry, security-aggregator, mock, test-fixtures, testing) overlap with phantoms seen in PhenoLang-actual and PhenoLang-actual/phenotype-infrakit — strong signal this was a coordinated (but incomplete) extraction wave. The 8 PascalCase names (Logify…Authvault) look like planned sub-projects that never materialized.

#### PhenoLang-actual — Major drift
- **Phantom members (11):** `crates/phenotype-bdd`, `crates/phenotype-config-core`, `crates/phenotype-core`, `crates/phenotype-event-bus`, `crates/phenotype-compliance-scanner`, `crates/phenotype-infrastructure`, `crates/phenotype-project-registry`, `crates/phenotype-security-aggregator`, `crates/phenotype-mock`, `crates/phenotype-test-fixtures`, `crates/phenotype-testing`
- **Phantom git deps (1):** `https://github.com/KooshaPari/agileplus-plugin-core`
- **Last touched:** `28a6f36` 2026-04-02 — chore: commit all pending changes across workspace
- **Fix strategy:** **Prune** — this repo is marked `PhenoLang-actual` (parallel scaffold); members were inherited from the shared scaffold and never implemented here.

#### PhenoLang-actual/phenotype-infrakit — Major drift
- **Phantom members (15):** `crates/phenotype-analytics`, `crates/phenotype-bdd`, `crates/phenotype-compliance-scanner`, `crates/phenotype-config-core`, `crates/phenotype-config-loader`, `crates/phenotype-contract-tests`, `crates/phenotype-git-core`, `crates/phenotype-health`, `crates/phenotype-http-client`, `crates/phenotype-project-registry`, `crates/phenotype-rate-limiter`, `crates/phenotype-security-aggregator`, `crates/phenotype-sentry-config`, `crates/phenotype-testing`, `crates/phenotype-validation`
- **Last touched:** `8e5a941` 2026-04-02 — ci: add Cargo.toml and quality-gate.sh blocked by gitignore
- **Fix strategy:** **Restore probable.** Several of these (`phenotype-health`, `phenotype-config-core`, `phenotype-git-core`) correspond to the Phase 1 LOC reduction shared crates that exist in the canonical `phenotype-infrakit` repo. This nested copy was likely scaffolded then never synced. Action: either (a) rsync the real `phenotype-infrakit/crates/*` content in, or (b) delete this nested workspace entirely since the canonical one lives at `repos/phenotype-infrakit/` (not inside PhenoLang-actual).

#### PhenoLibs/rust — Minor drift
- **Phantom members (2):** `phenotype-config`, `phenotype-bdd`
- **Last touched:** `0f5e653` 2026-04-05 — Initial commit: PhenoKit - 2026-04-05
- **Fix strategy:** **Prune.** PhenoLibs/rust was initialized as a split from PhenoKit; the two scaffold members were never filled in. Drop from members list.

#### PhenoPlugins — Minor drift
- **Phantom members (3):** `crates/pheno-plugin-core`, `crates/pheno-plugin-git`, `crates/pheno-plugin-sqlite`
- **Last touched:** `bf3a79b` 2026-04-03 — Add phenoVessel as pheno-plugin-vessel crate
- **Fix strategy:** **Prune or Restore.** Only `crates/pheno-plugin-vessel` actually exists. These three phantoms are the same `*-plugin-core/git/sqlite` triad that shows up as phantom git deps in AgilePlus and repos-root — strongly implies a shared plugin trio was renamed/relocated mid-extraction. Resolve the plugin-plugin naming split before pruning (decide whether the canonical home is `AgilePlus/libs/plugin-*`, `PhenoPlugins/crates/pheno-plugin-*`, or external `KooshaPari/agileplus-plugin-*` repos).

#### thegent-workspace — Minor drift
- **Phantom members (2):** `crates/thegent-jsonl`, `crates/thegent-utils`
- **Last touched:** `6a221db` 2026-04-05 — feat: initialize TheGent workspace with jsonl and utils crates
- **Fix strategy:** **Restore.** Commit message says "initialize…with jsonl and utils crates", so the crates were intended to exist. Either the crate directories were never committed or they live in a parallel tree. Check `thegent/crates/` (the sibling workspace at `thegent/crates/Cargo.toml`) — likely the real home; if so, either relocate members or drop this workspace as a stub.

### Cross-cutting observations

1. **Shared phantom cluster `crates/phenotype-{bdd,config-core,core,event-bus,compliance-scanner,infrastructure,project-registry,security-aggregator,mock,test-fixtures,testing}`** appears in 3 workspaces (HexaKit, PhenoKits/HexaKit, PhenoLang-actual). All three got the same scaffold on or around 2026-04-02 during a governance sweep (`feat: apply governance standards`), which appears to have templated a workspace manifest without the child crate directories.
2. **`agileplus-plugin-{core,git,sqlite}` URL cluster** is a second bulk phantom — appears as phantom git deps in AgilePlus, repos root, and PhenoLang-actual; overlaps semantically with phantom members in PhenoPlugins (`pheno-plugin-{core,git,sqlite}`). This is a single unresolved rename/extraction from 2026-04-02 → 2026-04-03.
3. **No CI or Dependabot config was touched** by this audit. Fixes require follow-up writes.
4. **Task description drift:** Task #79 claim "AgilePlus has ~50 phantom members" does not match current state (1 phantom). Suggest closing task #79 re-verify cycle and re-filing against the actual drifted repos above.

### Recommended unblock ordering for Dependabot

1. **AgilePlus** and **repos root** — prune 1 member + repoint 3 git deps to in-workspace paths. Lowest effort, highest yield (these are the most-consumed roots).
2. **PhenoLang-actual** (both nested workspaces) — these are duplicate scaffolds of canonical repos; decide delete-vs-restore before further Dependabot runs.
3. **HexaKit / PhenoKits/HexaKit** — resolve HexaKit's status (stub vs real); current state is Dependabot-hostile for 19 phantoms.
4. **PhenoPlugins, PhenoLibs/rust, thegent-workspace, BytePort** — minor pruning once upstream plugin-naming decision lands.

**Read-only audit artifacts:** raw JSON at `/tmp/ws_audit.json`, script at `/tmp/ws_audit.py` (ephemeral).

## 2026-04-24 — OpenAPI/spec coverage audit

Read-only GH API audit of machine-readable API contracts (OpenAPI, JSON Schema, Protobuf) across 16 Phenotype repos that expose HTTP/RPC surfaces. Goal: locate spec gaps and drift ahead of spec-kit adoption + cross-project reuse migration.

### Summary table

| Repo | OpenAPI | .proto | Freshness (latest spec commit) | Classification |
|---|---|---|---|---|
| AgilePlus | none | `proto/agileplus/v1/{core,common,agents,integrations}.proto` | 2026-03-31 | Partial — gRPC covered (4 protos); 13 REST route files in `crates/agileplus-api/src/routes/` have NO OpenAPI. Dashboard REST is spec-free. |
| thegent | `apps/byteport/backend/api/openapi.yaml` + `docs/reference/api/ts-stubs/openapi.d.ts` + `docs/reports/data/phenosdk_openapi_snapshot_2026-03-29.json` | none | 2026-03-29 | Partial — only byteport sub-app + phenosdk snapshot. Main MCP servers + REST surfaces unspec'd. |
| PhenoObservability | none | none | n/a | Missing — Rust observability platform with logging/metrics/tracing APIs, zero spec files. |
| cliproxyapi-plusplus | `api/openapi.yaml` | `api/proto/llmproxy.proto` | 2026-03-29 | Complete — both REST + gRPC spec'd; handlers in `pkg/llmproxy/api/handlers/` aligned with proto. <6mo old. |
| BytePort | none | `.history/.../bytebridge_*.proto` (editor history only) | 2024-11-27 | Missing — Tauri backend + REST, only artifact is IDE auto-save history from Nov 2024; no tracked contract. |
| AuthKit | none | none | n/a | Missing — Rust auth SDK with hex-arch (`src/{domain,application,adapters,infrastructure}`). If it exposes HTTP (via adapters), no spec exists. Possibly library-only — needs confirmation. |
| Tracera | none (CI workflow references auto-gen but no checked-in spec) | none | n/a | Missing — `.github/workflows/openapi-docs.yml` generates OpenAPI from `backend/internal/handlers/**` Go code on push, but no spec is committed to the repo. gRPC service definitions also absent. |
| hwLedger | none | none (only `governance-v1.json` spec-kit contract) | 2026-04-19 | Missing — ledger API not documented. The governance JSON is a spec-kit artifact, not an API contract. |
| McpKit | none | none | n/a | Missing — Go MCP framework SDK; no tool manifest or schema. |
| PhenoMCP | none | none | n/a | Missing — polyglot MCP implementation; no schemas. |
| phenotype-ops-mcp | none | none | n/a | Missing — Go MCP wrapper for nanos/ops unikernels. Tool schemas implicit in `{images,instances,packages,ops}.go`. |
| AgentMCP | n/a | n/a | n/a | N/A — repo does not exist under KooshaPari (404). |
| DataKit | none | none | n/a | Missing (or N/A) — Python storage/events SDK; likely library but events imply schemas. |
| Civis | none | `infra/agent-workspace/schemas/event-envelope.v1.json`, `task-envelope.v1.json` | 2026-02-23 | Partial — event/task envelope JSON schemas exist; no HTTP endpoints found (no handler files). Schema-only project; classify as Complete-for-schemas. |
| GDK | none | none | n/a | N/A — Rust git-workflow toolkit, appears to be a library. |
| ResilienceKit | none | none | n/a | N/A — Python circuit-breakers SDK, library-only. |

### Classification counts

- Complete: 1 (cliproxyapi-plusplus)
- Partial: 3 (AgilePlus, thegent, Civis)
- Missing: 8 (PhenoObservability, BytePort, AuthKit, Tracera, hwLedger, McpKit, PhenoMCP, phenotype-ops-mcp)
- N/A or unresolved: 4 (AgentMCP-404, DataKit, GDK, ResilienceKit)

### Top-5 repos needing spec bootstrap

1. **Tracera** — has CI workflow wired to regenerate OpenAPI on every push, but the generated artifact is never committed. Quick win: run the generator locally, commit `backend/api/openapi.yaml`. Also missing `.proto` for declared gRPC surface.
2. **AgilePlus** — 13 REST route files (`audit`, `backlog`, `branch`, `cycle`, `events`, `features`, `governance`, `import`, ...) with zero OpenAPI. gRPC side is healthy (4 protos current to 2026-03-31). REST dashboard is the biggest undocumented blast radius in the org.
3. **PhenoObservability** — "polyglot observability platform" with logging/metrics/tracing APIs and no contracts at all. Extraction/reuse mandate blocked until metric APIs are spec'd.
4. **BytePort** — only IDE-history `.proto` from Nov 2024; current backend has no tracked contract. Tauri commands + REST need spec'd before UI consumers can be generated.
5. **phenotype-ops-mcp** — MCP tool schema lives only in Go code (`images.go`, `instances.go`, `packages.go`, `ops.go`). MCPs need explicit tool manifests for agent discovery.

### Stale/drift candidates with specifics

- **BytePort** — Nov 2024 `.history/` protos are ~17 months old and live in an IDE auto-save dir (not a source-controlled contract path). Treat as drift, not spec.
- **AgilePlus archive duplication** — the same `.proto` set lives in three places: `proto/agileplus/v1/` (canonical), `kitty-specs/001-spec-driven-development-engine/contracts/`, `docs/specs/001-.../contracts/`, AND `.archive/kitty-specs/001-.../contracts/`. Four copies of the same core.proto. Single-source-of-truth violation; `kitty-specs` + `docs/specs` + `.archive` copies risk drifting silently from `proto/`.
- **thegent OpenAPI snapshot** — `docs/reports/data/phenosdk_openapi_snapshot_2026-03-29.json` is a point-in-time capture; no process regenerates it. TS stubs in `docs/reference/api/ts-stubs/openapi.d.ts` derive from it and will drift.
- **Tracera** — `openapi-docs.yml` workflow runs on every push to main/develop but doesn't commit output. Workflow without an artifact = invisible drift; every push silently re-derives a spec no one reviews.
- **Civis schemas** — frozen at v1 (2026-02-23); no v2 or evolution path visible. Acceptable if envelope is stable, risky if events are actively added.

### Cross-repo duplication / gaps worth flagging

- **gRPC without .proto**: AgilePlus's REST side and Tracera's entire surface (repo is described as "REST + gRPC") are missing proto definitions. Service contracts are unrecoverable from code without a spec regen pipeline.
- **MCP tool schemas**: None of the 4 MCP-adjacent repos (McpKit, PhenoMCP, phenotype-ops-mcp, AgentMCP) ship a tool manifest. Each MCP defines tools in code — agent-to-tool discovery must introspect Go/Rust AST instead of reading a JSON schema.
- **Auth spec duplication**: AuthKit has zero spec files, but `cliproxyapi-plusplus/api/openapi.yaml` likely defines auth endpoints for its proxy surface. If AuthKit ever exposes HTTP, expect the same auth contract to be inlined in cliproxy's OpenAPI — candidate for extraction into a shared `@phenotype/auth-spec` repo.
- **No organization-level OpenAPI bundle**: Zero evidence of a cross-repo OpenAPI aggregator / mock server / client-codegen pipeline. Each repo is an island.

### Read-only — no files modified outside this worklog.

## 2026-04-24 — MCP tool manifest audit

Follow-up to #118 "zero MCP tool manifests anywhere". Source: read-only GitHub API inspection of each repo's default branch (no clones, no builds).

### Per-repo state

| Repo | Stack | Tool count | Manifest present? | Classification | Recommendation |
|------|-------|------------|-------------------|----------------|----------------|
| `KooshaPari/McpKit` | Rust workspace (+ Go/Py/TS stubs, vendored `mcp-forge` for multi-language LSP) | 0 concrete tools (SDK scaffolding only) | No | **Empty / stub** — `registry.yaml` status: `planning`, all workspaces have `packages: []` | Defer. Bake manifest-generation into the SDK itself (`phenotype-mcp-core::Tool` already has `#[derive(Serialize)]`) so downstream servers get it for free. Emit via `impl Server { fn dump_tools(&self) -> Value }` + a `mcpkit-dump-tools` CLI. |
| `KooshaPari/PhenoMCP` | Rust (`pheno-mcp` bin, edition 2024) + 3 DB-client crates (meilisearch/qdrant/surrealdb) | 0 (bin is `fn main() { println!("PhenoMCP"); }`) | No | **Empty / stub** — has ADR/PLAN/PRD markdown but no server implementation yet | Defer until server is implemented. Bake `#[tool]` attribute usage (rmcp) or explicit `Tool { name, description, input_schema }` entries from the start so `tools.json` can be generated day one. |
| `KooshaPari/phenotype-ops-mcp` | Go (`metoro-io/mcp-golang` + stdio transport) | **5** (`pkg_load`, `instance_logs`, `instance_create`, `list_instances`, `list_images`) | No | **Source-only** — tools wired in `main.go` via `server.RegisterTool(name, desc, handler)`; handlers in `packages.go`, `instances.go`, `images.go` | **Easiest first target.** Single file (`main.go`, 45 lines), stable API (nanos/ops CLI wrapper), registrations are literal string pairs trivially extractable. |
| `KooshaPari/AgentMCP` | — | — | — | **Does not exist (404)** — earlier audit confirmed | Remove from catalog or create as stub. |

Tool-count estimates: McpKit = 0 (SDK), PhenoMCP = 0 (stub bin), ops-mcp = 5 (confirmed by direct grep of `server.RegisterTool(...)` in `main.go`).

### Bootstrap approach per stack

**Go (`phenotype-ops-mcp` — canonical first target).** `metoro-io/mcp-golang` derives input schemas via reflection on the handler's request struct, so a manifest can be emitted without upstream changes:
- Add a `--dump-tools` flag in `main.go` that constructs the same 5 `RegisterTool` calls against an in-memory recorder (or calls the real server's list_tools handler before `Serve()`), JSON-serializes the result, writes `tools.json`, and exits 0.
- Alternative: a `TestMain`-driven Go test (`tools_manifest_test.go`) that does the same and writes `tools.json` as a test fixture. Keeps the runtime binary clean; CI enforces freshness via `git diff --exit-code tools.json`.
- Either way, commit `tools.json` at repo root. ~30 LOC change.

**Rust (rmcp-style).** For PhenoMCP (future) and any McpKit-built server:
- `phenotype-mcp-core::handlers::Handlers::handle_tools_list` already produces the exact JSON shape. Expose it via a `bin/dump-tools.rs` (or `cargo run --bin dump-tools`) that constructs the server's handler registry and prints the serialized tool list. Commit output as `tools.json`.
- `build.rs`-time generation is tempting but fragile (can't run the async runtime at build time, and tool registration often depends on runtime config). Prefer a test or a small bin.
- Long-term: add a `#[derive(ToolManifest)]` proc-macro in `phenotype-mcp-core` that walks the `ToolHandler` impl and emits a `const TOOL_MANIFEST: &str` — gives free static discovery with zero CI plumbing.

**Python (`fastmcp` / `@tool`).** Not applicable to any of the 3 live repos today. Pattern when needed: `mcp dev --dump-tools > tools.json`, or a pytest fixture that serializes the server's tool registry.

### First-target recommendation

**`phenotype-ops-mcp`** — the only repo with actual tools today, has exactly 5 of them in a single file, uses a reflection-based SDK so manifest emission is mechanical, and the repo is small enough (10 `.go` files, ~400 LOC total) that a manifest PR reviews in minutes. Expected PR shape:
- Add `cmd/dump-tools/main.go` (or a `--dump-tools` flag on the existing binary).
- Commit `tools.json` with the 5 tools (name, description, JSON schema for each handler's request struct).
- Add a CI check: `go run ./cmd/dump-tools > /tmp/tools.json && diff tools.json /tmp/tools.json`.
- No runtime behaviour change; purely additive static-discovery artifact.

Once the pattern is validated on ops-mcp, replicate in PhenoMCP (when it gains tools) and bake into the McpKit SDK so every downstream server ships a manifest by default.

### Read-only — no source files modified; only this worklog updated.

## 2026-04-24 — PhenoObservability workspace normalization scoping

**Context:** Finding from PhenoObservability issue #150. `phenotype-health` + `phenotype-health-axum` need utoipa wiring and OpenAPI drift-gate flip-on. Blocker: these crates are NOT reachable from the root `[workspace]` members list, so `cargo build --workspace` / `cargo run -p phenotype-health-axum` from repo root fail.

### Current workspace topology (via GH API, read-only)

```
PhenoObservability/  (KooshaPari/PhenoObservability@main)
├── Cargo.toml                          [workspace] members = crates/* (10 crates)
│     ├── crates/helix-logging/
│     ├── crates/pheno-dragonfly/
│     ├── crates/pheno-questdb/
│     ├── crates/pheno-tracing/
│     ├── crates/phenotype-llm/
│     ├── crates/phenotype-mcp-server/
│     ├── crates/phenotype-surrealdb/
│     ├── crates/tracely-core/
│     ├── crates/tracely-sentinel/       (+ nested fuzz/Cargo.toml)
│     └── crates/tracingkit/
│
├── rust/                               ← NO Cargo.toml here (orphan parent)
│     ├── phenotype-health/             ← uses version.workspace = true
│     ├── phenotype-health-axum/        ← 9-endpoint HTTP wrapper (#150 target)
│     ├── phenotype-health-cli/
│     ├── phenotype-logging/
│     ├── phenotype-metrics/
│     ├── phenotype-mock/
│     └── phenotype-telemetry/
│
├── ObservabilityKit/                   ← git submodule (gitlink, type=commit)
│                                         .gitmodules MISSING → broken submodule config
│                                         Claimed to host ObservabilityKit/rust/* duplicate set
│
├── KWatch/                             ← also gitlink (submodule, out of scope)
├── go/ python/ ts/ zig/ mojo/ wasi/ ffi/ bindings/
└── health/ logging/ metrics/ tracing/ alerting/ dashboards/ …
```

**Classification:** **Orphan + broken-submodule hybrid.**

- Root workspace is flat and healthy for `crates/*`.
- `rust/phenotype-health*` crates reference `version.workspace = true`, `edition.workspace = true`, etc. — they REQUIRE a workspace parent. With no `rust/Cargo.toml` and no entry in root `members`, they currently do not build via any top-level invocation. Likely built only via ad-hoc `cargo build --manifest-path rust/phenotype-health/Cargo.toml` using the root workspace as an implicit ancestor — fragile.
- `ObservabilityKit` is a submodule in the GitHub tree but `.gitmodules` is absent → submodule URL unresolvable on fresh clone. Cannot inspect its `rust/` subtree via API.

**Why split exists (inferred):** `rust/` is the legacy pre-consolidation path (pre-dates the `crates/*` convention introduced with the tracely merger). `ObservabilityKit` was a forked/extracted kit that got re-embedded as a submodule but never properly wired. No explicit intent doc found in README or AGENTS.md for keeping them split.

### Migration options

**Option A — Merge `rust/*` into root workspace (recommended)**
- Edit root `Cargo.toml` `members` list: add 7 paths (`rust/phenotype-health`, `rust/phenotype-health-axum`, `rust/phenotype-health-cli`, `rust/phenotype-logging`, `rust/phenotype-metrics`, `rust/phenotype-mock`, `rust/phenotype-telemetry`).
- Optionally relocate `rust/phenotype-*` → `crates/phenotype-*` for uniformity (adds git mv churn but aligns with README architecture diagram).
- Drift-gate consumer becomes: `cargo run -p phenotype-health-axum -- --dump-openapi` from repo root. Clean.
- **File changes:** 1 edit (root `Cargo.toml`), or 1 edit + 7 dir moves if relocating.
- **Risks:** Low. `version.workspace = true` already assumes root workspace inheritance; wiring members in makes the existing assumption explicit.
- **Downstream impact:** None for crate consumers (path dependencies unchanged if we skip the relocation). `cargo build --workspace` now builds all 17 crates instead of 10.

**Option B — Promote `rust/` to its own workspace**
- Create `rust/Cargo.toml` with `[workspace] members = ["phenotype-health", …]` and its own `[workspace.package]` stanza.
- Root workspace and `rust/` workspace remain separate (two `cargo build --workspace` invocations needed).
- Drift-gate: `cargo run --manifest-path rust/Cargo.toml -p phenotype-health-axum -- --dump-openapi`.
- **File changes:** 1 new file (`rust/Cargo.toml`) + duplicate `[workspace.package]` fields (version, edition, rust-version, license, authors).
- **Risks:** Medium. Two parallel workspaces drift over time (tokio version skew, edition bumps). CI must lint both. Defeats the "one monorepo" posture the README asserts.
- **Downstream impact:** Drift-gate CI job gets a slightly longer invocation; no consumer impact.

**Option C — Publish `phenotype-health` and cross-reference via `[patch.crates-io]`**
- Requires crates.io publish (or private registry). Overkill for internal, pre-1.0 code.
- **Not recommended.** Dropped.

### Blockers

1. **ObservabilityKit submodule is broken** — gitlink exists but `.gitmodules` is missing. Cannot see if the duplicate `ObservabilityKit/rust/phenotype-health*` still exists (issue #150 implied it does). Before normalization, must decide: (a) restore `.gitmodules` and treat it as an external vendor, (b) absorb its contents as regular dirs, or (c) delete the gitlink. This is independent of workspace wiring but should be resolved in the same PR to avoid reintroducing duplication.
2. **Submodule git history** — if absorbing, `git rm` + re-add as regular tree loses the submodule's internal history unless preserved via `git subtree add --squash`.
3. **Nested fuzz workspace** — `crates/tracely-sentinel/fuzz/Cargo.toml` is likely its own micro-workspace (cargo-fuzz convention). Verify it still builds after any `members` regex changes; safe to exclude explicitly if needed.

### Recommendation

**Option A, without relocation.** Single 1-line edit to `members = [...]` in root `Cargo.toml` adds the 7 `rust/*` paths. Unblocks utoipa wiring and the OpenAPI drift-gate with the simplest possible invocation (`cargo run -p phenotype-health-axum …`). Handle the ObservabilityKit submodule decision in a separate but adjacent PR — do not couple.

**Estimated effort:**
- Workspace fix: 1 tool call / ~2 min.
- Verification (`cargo check --workspace`, `cargo test -p phenotype-health-axum`): 3–5 min local CI.
- ObservabilityKit submodule disposition: separate ~20-min decision + PR.

### Read-only scoping — no Cargo.toml edits, no clones, no git operations.

## 2026-04-24 — PhenoKits workspace drift design scoping (#103)

**Scope:** Read-only investigation of `KooshaPari/PhenoKits` root `Cargo.toml` `[workspace] members` vs actual repo contents. No edits.

### Current topology (GH API, default branch)

Root `Cargo.toml` declares:
```
members = [
  "templates", "template-domain", "template-program-ops",
  "HexaKit", "libs/rust", "libs/go", "libs/python", "libs/typescript",
]
```

Actual repo state:

| Declared member | Exists? | Has `Cargo.toml`? | Notes |
|---|---|---|---|
| `templates/` | yes (dir) | no | Container holding 6 sub-templates (`clean-rust/`, `hexagonal/`, `phenotype-api/`, `api/`, `microservice-scaffold/`, `webapp/`). `templates/clean-rust/Cargo.toml` exists, as do crates under `templates/hexagonal/hexagonal-rs/` etc. |
| `template-domain/` | **404 NOT FOUND** | — | Directory does not exist in repo. |
| `template-program-ops/` | **404 NOT FOUND** | — | Directory does not exist in repo. |
| `HexaKit` | gitlink (submodule per `.gitmodules`) | unknown without clone | Submodule `url=https://github.com/KooshaPari/HexaKit.git`, not a workspace crate in-tree. |
| `libs/rust/` | yes (dir) | no | Container: `phenotype-id/`, `phenotype-logging/`, `phenotype-testing/`. **None has `Cargo.toml`**. `phenotype-id/` has only `go/` `python/` `typescript/` subtrees (Rust crate *absent*). `phenotype-logging/` and `phenotype-testing/` each contain `README.md`, `pyproject.toml`, `src/` — they are **Python packages mis-filed under `libs/rust/`**. |
| `libs/go/` | yes | no | Container with `phenotype-go-{auth,cli,config,kit,middleware}/` subdirs. |
| `libs/python/` | yes | no | Container with `phenotype-py-kit/`. |
| `libs/typescript/` | yes | no | Container with `plugin-typescript/`. |

**Result:** 8/8 declared members fail cargo workspace resolution. Zero valid Rust workspace crates reachable from root.

### Intent evidence

1. **Repo identity.** `README.md` and Cargo.toml comments describe a "12-category artifact platform" covering templates, configs, libs (multi-language), governance, security, etc. Nothing in the README claims this is a cargo workspace; `libs/` is explicitly multi-language (rust/go/python/typescript siblings).
2. **Git history.** `Cargo.toml` has a **single commit** (`abb5827d`, 2026-04-06, "Initial commit: PhenoKits 12-category artifact platform"). The workspace stanza was never iterated — it was a placeholder written alongside the initial scaffold.
3. **Misfiled Rust libs.** The only entries under `libs/rust/` with actual package manifests are `pyproject.toml` (phenotype-logging, phenotype-testing) or polyglot bindings (phenotype-id has go/python/typescript but **no Rust**). There are no Rust crates there at all.
4. **Real Rust crates exist elsewhere.** `templates/clean-rust/Cargo.toml` and `templates/hexagonal/hexagonal-rs/` are bona-fide Rust scaffolding, but they are **templates for consumers to instantiate**, not workspace members (compiling them as members would bind the repo to their scaffold state).
5. **`HexaKit`** is a submodule pointer — compiling it as a member would force submodule init for every `cargo` invocation and is inconsistent with the submodule pattern.
6. **Missing stubs.** `template-domain/` and `template-program-ops/` were never committed; no trace in the tree at default branch.

**Conclusion on intent:** the `[workspace]` stanza in root `Cargo.toml` is **vestigial boilerplate from initial scaffold**. The author's true model (per README + directory layout) is a *polyglot artifact catalog*, not a Rust workspace. The member list is aspirational/typo — it mixes container dirs with crate dirs and references three paths that were never created.

### Options

#### Option A — Convert to glob members (`libs/rust/*`, plus template crates)
- Change: `members = ["libs/rust/*", "templates/clean-rust", "templates/hexagonal/hexagonal-rs"]`.
- Effort: ~2 tool calls (edit + local `cargo check`).
- **Blocker:** `libs/rust/*` currently contains **zero Rust crates** — globs would expand to Python/polyglot dirs and fail just as hard. Would also force templates to be compilable, coupling scaffold versions to workspace lockfile.
- Risk: high — masks the structural mistake (Python packages under `libs/rust/`); does not match README semantics.

#### Option B — Remove `[workspace]` stanza entirely from root Cargo.toml
- Change: delete `[workspace]`, `[workspace.package]`, and root `resolver`. Let each Rust crate (wherever it lives) be a standalone package, or introduce sub-workspaces later (e.g. `libs/rust/Cargo.toml` when real Rust crates appear).
- Effort: ~1 tool call (delete ~15 lines) + 1 verification (`cargo metadata --manifest-path=…` on any standalone crate).
- Matches README: PhenoKits is a polyglot artifact catalog, not a cargo workspace.
- Risk: low. The current Cargo.toml is non-functional anyway (no member resolves); removing it breaks nothing that was working. Downstream repos don't depend on PhenoKits as a workspace.
- Side effect: root Cargo.toml could be deleted outright, or kept as a pure `[workspace.package]` metadata shim if we want to preserve license/version central defaults (but nothing references it today).

#### Option C — Restore the 3 missing stub crates + add Cargo.toml at each container
- Change: create `templates/Cargo.toml`, `template-domain/Cargo.toml`, `template-program-ops/Cargo.toml`, `HexaKit/Cargo.toml` (via submodule?), plus Cargo.toml shims at `libs/{rust,go,python,typescript}/`.
- Effort: 8+ new files, reconcile with submodule, invent package semantics (what does `libs/go` mean as a Rust crate?). ~10–15 tool calls.
- Risk: very high — fabricates intent, forces Rust packaging onto non-Rust containers, entangles submodule lifecycle with workspace build. No evidence this was ever the plan.

### Ranked recommendation

1. **Option B — remove `[workspace]` stanza.** Matches README intent, one-file edit, zero risk, fixes #103 correctly.
2. **Option A — glob members.** Only becomes viable *after* (a) relocating the misfiled Python packages out of `libs/rust/`, and (b) at least one real Rust crate landing in `libs/rust/`. Premature today.
3. **Option C — restore stubs.** Reject. No evidence the stubs were ever intended; would embed Rust packaging assumptions across an explicitly polyglot catalog.

**Follow-on (out of scope for #103):** file a cleanup issue for the misfiled Python packages under `libs/rust/phenotype-logging` and `libs/rust/phenotype-testing` (they should live under `libs/python/`), and for `libs/rust/phenotype-id/` which has no Rust artifacts despite its path.

### Read-only scoping — no Cargo.toml edits, no clones, no git operations.
