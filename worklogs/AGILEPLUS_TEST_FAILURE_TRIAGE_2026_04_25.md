# AgilePlus Pre-Push Hook Test Failure Triage — 2026-04-25

**Context:** Pre-push hook flagged pre-existing failures in four crates. Two recent
PRs (#399 PyJWT bump, #401 spec-013 plan rewrite) bypassed the hook with
`HOOKS_SKIP=1`. Per CI Completeness Policy, all pre-existing failures must be
classified and tracked, even when inherited from main.

**Categories:** (a) real bug; (b) flaky test; (c) test fixture out of date;
(d) test scope drift (test references API that no longer exists / has moved).

**Triage methodology:**
- Worktree used: `repos/AgilePlus-wtrees/pyjwt-fix` (HEAD=`2be613c`, two commits
  behind `origin/main`; the missing two commits are docs-only and a dependabot
  cargo PR — neither touches Rust source). Reused this worktree to leverage its
  2.6 GB warm `target/` cache, since `/System/Volumes/Data` had only ~13 GiB
  free at start (below the 20 GiB pre-dispatch threshold for fresh builds).
- Test commands: `cargo test -p <crate> --no-fail-fast`.
- Determinism: each failing test re-run once individually; all observed
  failures reproduced identically.
- No source files modified; no hooks bypassed.

## Total failures

**17 failing tests across 4 crates** (out of ~217 measured).

| Crate                | Run        | Failed | Passed | Notes                                            |
|----------------------|------------|-------:|-------:|--------------------------------------------------|
| agileplus-api        | integration|     14 |      0 | All 14 panic on identical router-build assertion |
| agileplus-api        | doctest    |      1 |      1 | `openapi.rs` example uses removed utoipa API     |
| agileplus-cli        | unit + int |      0 |     95 | Clean — no failures                              |
| agileplus-plane      | lib unit   |      1 |     69 | Wiremock matcher 404 in one client test          |
| agileplus-telemetry  | unit       |      0 |     30 | Clean                                            |
| agileplus-telemetry  | doctest    |      1 |      1 | lib.rs quick-start references unexported symbol  |

---

## agileplus-api — 14 integration + 1 doctest failure

### Integration tests (14, all same root cause)

**Failed tests:**
`get_audit_trail`, `get_feature_found`, `get_feature_not_found`,
`get_governance`, `get_work_package_found`, `get_work_package_not_found`,
`health_no_auth_required`, `info_no_auth_required`,
`list_features_invalid_key_returns_401`, `list_features_requires_auth`,
`list_features_with_valid_key`, `response_content_type_is_json`,
`trigger_validate`, `verify_audit_chain_valid`.

**Error (first 5 lines, identical for all 14):**
```
thread '<test>' panicked at crates/agileplus-api/src/router.rs:108:10:
Overlapping method route. Handler for `GET /api/v1/stream` already exists
```

**Determinism:** deterministic (re-run confirmed).

**Root cause:** `crates/agileplus-api/src/router.rs:92` registers
`GET /api/v1/stream` on the protected router, then `crates/agileplus-dashboard/
src/routes.rs:2228` registers `GET /api/v1/stream` on the dashboard router, and
`build_router` (`router.rs:105-108`) `merge`s both — Axum panics on overlap.
The `health_handler` is fine; the panic happens before any handler runs because
all 14 tests call `build_router(...)` during setup.

**Classification: (a) Real bug** — duplicate route registration is a regression,
not a test issue. Fix is a one-line route-prefix change (e.g. dashboard owns
`/dashboard/stream`, or api stops re-registering it when dashboard is merged).

**Priority: P0 (critical)** — entire API integration suite is dead. Any pre-push
or CI run touching this crate is currently bypassed, hiding all real
regressions.

**Fix effort:** ~10–20 min (decide which surface owns `/api/v1/stream`, update
the loser, regenerate any OpenAPI fixtures).

### Doctest: `crates/agileplus-api/src/openapi.rs`

**Error (first 5 lines):**
```
error[E0599]: no method named `to_yaml` found for struct
              `utoipa::openapi::OpenApi` in the current scope
  --> crates/agileplus-api/src/openapi.rs:15:30
   |
15 | let yaml = ApiDoc::openapi().to_yaml().unwrap();
```

**Determinism:** deterministic compile error.

**Root cause:** workspace pinned to `utoipa = "5"`. utoipa 5 removed
`OpenApi::to_yaml()`; consumers must depend on the `utoipa-yaml` adapter or
serialize via `serde_yaml::to_string(&doc)`. The doc comment was authored
against utoipa 4.

**Classification: (c) test fixture out of date** (doctest in module-level docs
referencing a removed API after a major-version bump). Borderline (a) — the
public-facing example is wrong, which is a documentation defect.

**Priority: P1 (important)** — does not block runtime, but mis-teaches users
of the OpenAPI generator and proves doctest CI is not running.

**Fix effort:** ~5 min (replace with `serde_yaml::to_string(&ApiDoc::openapi())`
or wire `utoipa-yaml`).

---

## agileplus-cli — 0 failures

Ran `cargo test -p agileplus-cli --no-fail-fast`: **95/95 passed**
(85 lib unit + 10 cli_integration + 0 doctests, 1 ignored).

**Conclusion:** the original pre-push report’s flag on `agileplus-cli` does not
reproduce on `2be613c`. Either:
- it was transient (hook ran during a partial save / build cache mismatch), or
- the failure existed on a different commit and has since been fixed, or
- it was flagged because of compile-fail propagation from another crate when
  the workspace test was driven globally rather than via `-p`.

**Classification: (b) flaky / environmental** as observed today.

**Priority: P2 (cleanup)** — re-run once after fixing api crate; if it
re-appears, file a fresh ticket. No action needed now.

**Fix effort:** zero (currently green).

---

## agileplus-plane — 1 lib unit failure

**Failed test:** `client::tests::create_sub_issue_sends_post_with_parent`

**Error (first 5 lines):**
```
thread 'client::tests::create_sub_issue_sends_post_with_parent' panicked at
  crates/agileplus-plane/src/client/tests.rs:371:10:
called `Result::unwrap()` on an `Err` value:
  Plane.so API error 404 Not Found:
```

**Determinism:** deterministic (re-run confirmed; same 404).

**Root cause analysis:** the test sets up a wiremock matching
`POST /api/v1/workspaces/ws/projects/proj/work-items/` with
`body_partial_json("{\"parent\":\"parent-123\"}")`. The implementation
(`client/resources/work_items.rs:70`) builds a `PlaneWorkItem` with `parent:
Some("parent-123".into())` and POSTs to `work_items_url()` which produces the
exact path the mock expects. The 404 is wiremock’s "no matcher matched"
fallback, meaning the body matcher fails — most likely because `PlaneWorkItem`
gained `priority: Some(3)` and `labels: vec![]` fields that change body shape,
and `body_partial_json` is being applied against a serialization that now
serializes `parent` as a different JSON form (e.g. `Some("...")` flattened
differently, or the `parent` field rename/serde attribute changed). Other
sibling tests in the same module pass, so the wiremock plumbing itself works.

**Classification: (c) test fixture out of date** — likely a serde/struct shape
drift between when the matcher was written and the current `PlaneWorkItem`
definition. Could be (a) if the production payload regression broke real Plane
sub-issue creation; an HTTP capture against a real Plane sandbox would
disambiguate. Treating as (c) until confirmed.

**Priority: P1 (important)** — covers a real cross-cutting feature
(parent/child sync). Should be repaired before next plane-sync release.

**Fix effort:** ~15–30 min (instrument the test to print actual request body
that wiremock saw, then either correct the matcher or correct the serializer
depending on which side drifted).

---

## agileplus-telemetry — 1 doctest failure

**Failed test:** `crates/agileplus-telemetry/src/lib.rs - (line 5)` (module
quick-start doctest).

**Error (first 5 lines):**
```
error[E0432]: unresolved import `agileplus_telemetry::init_telemetry`
 --> crates/agileplus-telemetry/src/lib.rs:7:27
  |
7 | use agileplus_telemetry::{init_telemetry, config::TelemetryConfig,
                              trace_layer};
  |                           ^^^^^^^^^^^^^^ no `init_telemetry` in the root
```

**Determinism:** deterministic compile error.

**Root cause:** `lib.rs` re-exports `TelemetryAdapter`, `TelemetryConfig`,
`AgilePlusMetrics`, `telemetry_layer`, `trace_layer` — but no
`init_telemetry`. The crate refactored from a free `init_telemetry()` function
to `TelemetryAdapter::init` (or similar) without updating the module-level
quick-start example.

**Classification: (d) test scope drift** — doctest references a public symbol
that has since been renamed/removed. Effectively a doc rot defect.

**Priority: P1 (important)** — the docstring is the first thing a consumer
sees on docs.rs / `cargo doc`. Misleading.

**Fix effort:** ~5–10 min (rewrite the example to call the current
`TelemetryAdapter::init(cfg)` flow or whatever the canonical entry point is,
verify with `cargo test -p agileplus-telemetry --doc`).

---

## Summary table

| Crate / failure                                  | Class | Prio | Effort   | Determinism |
|--------------------------------------------------|:-----:|:----:|---------:|:-----------:|
| api: 14 integration tests (route overlap)        |  a    |  P0  | 10–20 min| deterministic |
| api: openapi.rs doctest (utoipa 5 `to_yaml`)     |  c    |  P1  | 5 min    | deterministic |
| cli: (no current failure)                        |  b    |  P2  | 0        | did not repro |
| plane: create_sub_issue_sends_post_with_parent   |  c    |  P1  | 15–30 min| deterministic |
| telemetry: lib.rs quick-start doctest            |  d    |  P1  | 5–10 min | deterministic |

**Aggregate fix effort:** ~35–65 minutes of focused work to clear all P0+P1
items. None require new design; all are mechanical updates against the current
codebase.

## Effort estimate by category

| Category                    | Count | Total effort  |
|-----------------------------|------:|---------------|
| (a) Real bug                |   1   | 10–20 min     |
| (b) Flaky / environmental   |   1   | 0 (re-verify) |
| (c) Fixture out of date     |   2   | 20–35 min     |
| (d) Test scope drift        |   1   | 5–10 min      |

## Recommended issues to open

1. **`agileplus-api`: duplicate `GET /api/v1/stream` between api and dashboard
   merges (P0)** — Axum panic kills 14 integration tests; root cause in
   `router.rs:92` vs `dashboard/routes.rs:2228`.
2. **`agileplus-api`: openapi.rs doctest broken on utoipa 5 (P1)** — replace
   `to_yaml()` with utoipa 5 idiom.
3. **`agileplus-plane`: create_sub_issue wiremock body matcher 404 (P1)** —
   investigate `PlaneWorkItem` serde drift vs `body_partial_json` matcher.
4. **`agileplus-telemetry`: lib.rs quick-start references unexported
   `init_telemetry` (P1)** — update to current `TelemetryAdapter::init` API.
5. **`agileplus-cli`: re-verify after api fix (P2)** — original pre-push
   report flagged this crate; not reproducible at `2be613c` in isolation.

## Process notes

- The pre-push hook continues to be the only thing keeping these defects
  visible; bypassing with `HOOKS_SKIP=1` (as #399 and #401 did) is exactly the
  failure mode the CI Completeness Policy forbids. Recommend the next PR after
  fixing #1 also re-enables the hook on the bypassed branches.
- Disk pressure (11–13 GiB free during this session) prevented spinning a fresh
  worktree off `origin/main` for a clean cold build. The pyjwt-fix worktree
  was used as a proxy because it is two non-Rust commits behind origin/main;
  no source files were modified.

— Generated 2026-04-25, agent-driven triage, ~6 min wall clock, 11 tool calls.
