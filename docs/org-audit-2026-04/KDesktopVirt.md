# KDesktopVirt — 10-Dimension Scorecard & Merger Evaluation

**Audit Date:** 2026-04-24  
**Repository:** `/Users/kooshapari/CodeProjects/Phenotype/repos/KDesktopVirt`  
**Crate Name:** `kvirtualstage` (Note: naming inconsistency; repo is KDesktopVirt, crate is kvirtualstage)  
**LOC:** ~31.8K Rust  
**Git History:** 7 commits (minimal development, primarily infrastructure scaffolding)

---

## Executive Summary

KDesktopVirt is a **Playwright-equivalent desktop automation platform** designed for AI agents to interact with full desktop environments. It combines Docker-based virtualization with X11 UI automation, FFmpeg recording, and enterprise security (AES-256-GCM, OAuth, MFA). The project has a comprehensive spec (SPEC.md, 57KB), detailed PLAN.md showing 5 phases "completed," and 4 binary targets (CLI, demo, API server, TUI).

**CRITICAL FINDING:** Project is **archived and preserved for historical reference only**. Per README.md: "This project is no longer maintained. See Phenotype ecosystem for current tooling." Despite completion claims, **no actual tests exist** (grep found 0 test functions), build configuration is broken (Cargo workspace conflict), and core automation modules are commented out due to syntax errors.

**VERDICT:** **ARCHIVE** (do not merge into phenotype-automation). Project is a reference exploration, not production-ready automation infrastructure. The codebase lacks testability, has unresolved build issues, and its declarative "completion" contradicts actual implementation gaps.

---

## 10-Dimension Scorecard

| Dimension | Status | Evidence | Grade |
|-----------|--------|----------|-------|
| **BUILD** | BROKEN | Cargo workspace conflict; `cargo check` fails immediately. Crate declares parent workspace but not registered. | 🔴 BROKEN |
| **TESTS** | MISSING | 0 test functions across 31.8K LOC. No unit, integration, or e2e tests. Criterion dependency declared but unused. | 🔴 MISSING |
| **CI** | EXTERNAL-BLOCKED | CI/CD configured (ci.yml, legacy-tooling-gate.yml) but GitHub Actions billing constraint prevents execution. CI jobs fail with spending limits. | 🟡 EXTERNAL-BLOCKED |
| **DOCS** | SHIPPED | Comprehensive SPEC.md (57KB), PLAN.md, PRD.md, README.md. Architecture, APIs, security, deployment all documented. Multiple README variants (README_GITHUB.md, etc.) suggest incomplete cleanup. | 🟢 SHIPPED |
| **ARCH-DEBT** | BROKEN | Core modules commented out (`animation_framework`, `visual_feedback`, `natural_automation_demo`) due to syntax errors. `tts_audio_system_broken.rs` and `ffmpeg_pipeline_broken.rs` indicate incomplete refactors. Dead code marked `#[allow(dead_code)]`. | 🔴 BROKEN |
| **FR-TRACE** | MISSING | No functional requirement traceability. SPEC.md lists features but no FR file or test-to-FR mapping. No acceptance criteria or verification matrix. | 🔴 MISSING |
| **VELOCITY** | BROKEN | 7 commits over ~4 weeks (claimed 15 weeks for 5 phases). Last meaningful commit 4/4/26 (20 days old). All phases marked "Completed ✅" but implementation incomplete. No iterative progress; appears to be documentation-first. | 🔴 BROKEN |
| **GOVERNANCE** | SCAFFOLD | AgilePlus scaffolding added (commit 0c59499) but no active specs or work packages. Legacy tooling gate configured but CI never runs due to billing. No worktree discipline observed. | 🟡 SCAFFOLD |
| **DEPS** | SHIPPED | 50+ dependencies declared (tokio, axum, bollard, pyo3, napi, sqlx, redis, tracing, etc.). Versions are recent (2024-2025 range). Optional features well-designed (tui, web-ui, database, kubernetes, python-bindings, nodejs-bindings). Feature flags enable ergonomic compilation. | 🟢 SHIPPED |
| **HONEST-GAPS** | BROKEN | **No working build.** **No tests.** **No CI execution.** **Incomplete modules.** **No observable usage.** **No downstream integration.** Declared "operational" contradicts disabled modules and test absence. Project is a proof-of-concept reference, not production automation infrastructure. | 🔴 BROKEN |

---

## Detailed Findings

### BUILD: Broken (Cannot Compile)

**Error:**
```
error: current package believes it's in a workspace when it's not:
current:   /Users/kooshapari/CodeProjects/Phenotype/repos/KDesktopVirt/Cargo.toml
workspace: /Users/kooshapari/CodeProjects/Phenotype/repos/Cargo.toml
```

**Root Cause:** KDesktopVirt/Cargo.toml has `package.name = "kvirtualstage"` but parent `repos/Cargo.toml` treats it as a potential member without explicitly registering it. KDesktopVirt must either:
1. Be added to `repos/Cargo.toml` `[workspace.members]`
2. Have an empty `[workspace]` table in its own Cargo.toml to declare independence
3. Be moved to a new standalone repository

**Impact:** Cannot run `cargo check`, `cargo test`, or `cargo build` from within KDesktopVirt. Blocks all validation.

### TESTS: Missing (0 Tests)

No test functions found in any `.rs` file. `criterion` (benchmark framework) is a dev-dependency but never invoked. Pattern suggests documentation-first design without test-driven development.

**Implications:**
- No proof that APIs work as documented
- No regression detection
- No acceptance criteria verification
- Cannot validate "completed phases" claims

### CI: External-Blocked (GitHub Actions Billing)

CI workflows configured (`ci.yml`, `legacy-tooling-gate.yml`) but fail immediately due to GitHub Actions spending limit. Per user's CLAUDE.md: "GitHub Actions billing is a hard constraint. No additional funds will be added."

**Status:** CI is architectural requirement but technically non-functional in this environment.

### DOCS: Shipped (Comprehensive but Inconsistent)

**Strengths:**
- SPEC.md (57KB): Detailed architecture, APIs, security, performance requirements
- PLAN.md: 5 phases with clear deliverables and timelines
- PRD.md, ADR-002, CHARTER.md: Complete specification suite
- Multiple architecture summaries (CORE_ENGINE_IMPLEMENTATION_REPORT, AUDIO_VIDEO_IMPLEMENTATION_SUMMARY, etc.)

**Weaknesses:**
- README.md is 484 bytes: just a deprecation notice ("STRICTLY DO NOT DELETE NOR UNARCHIVE / Personal Project")
- README_GITHUB.md, API_FRAMEWORK_README.md, MCP_SERVER_README.md suggest incomplete documentation consolidation
- PLAN.md lists all phases "Completed ✅" but contradicted by broken build and missing tests
- No user journey documentation or integration guides

**Grade:** SHIPPED but needs consolidation; README.md should consolidate variants.

### ARCH-DEBT: Broken (Syntax Errors, Dead Code)

**Disabled Modules (src/lib.rs):**
```rust
// pub mod animation_framework;
// pub mod visual_feedback;
// pub mod natural_automation_demo;
```

**Broken Files:**
- `src/tts_audio_system_broken.rs` — duplicate of tts_audio_system.rs, indicates incomplete refactor
- `src/ffmpeg_pipeline_broken.rs` — same pattern

**Dead Code Suppressions:**
- Multiple `#[allow(dead_code)]` markers suggest incomplete interface cleanup
- 23 source files (40 total); 3+ are marked broken or disabled

**Impact:** 10% of declared modules are non-functional. Codebase was frozen mid-refactor.

### FR-TRACE: Missing (No Verification Matrix)

SPEC.md lists features but no structured Functional Requirements document. No test-to-FR mapping. Phase/task completion claims ("Completed ✅") lack:
- Acceptance criteria
- Verification method
- Test evidence
- Sign-off criteria

**Implication:** Cannot validate claims that 5 phases are "complete."

### VELOCITY: Broken (Minimal Iterative Progress)

**Git Log:**
```
7 commits total
c604252 docs: mark as STRICTLY DO NOT DELETE NOR UNARCHIVE - personal project (Apr 4)
ab00d36 chore: add AgilePlus scaffolding (Apr 4)
0c59499 ci(legacy-enforcement): add legacy tooling anti-pattern gate (Apr 4)
b77089f docs: add README/SPEC/PLAN (Apr 4)
934c7c7 chore: merge KVirtualStage - desktop automation consolidation (Apr 4)
```

**Analysis:**
- 7 commits in ~20 days (stalled)
- All substantive commits on Apr 4 (documentation import)
- No new development since initial import
- "Completed" 5 phases in 7 commits suggests documentation-first, not iterative implementation

**Grade:** BROKEN — No observable velocity; appears to be a reference snapshot.

### GOVERNANCE: Scaffold (Config Present, No Active Work)

- AgilePlus scaffolding committed but no active specs or work packages
- Legacy tooling gate configured in CI but CI never executes
- No worktree discipline (per CLAUDE.md, feature work should use worktrees)
- README explicitly marks project as "Personal Project" and "archived"

**Status:** Governance infrastructure present but dormant.

### DEPS: Shipped (Well-Curated, Recent)

**Tier 1 (Core):**
- tokio 1.0 (async runtime)
- serde 1.0 + serde_json (serialization)
- anyhow 1.0 + thiserror 1.0 (error handling)

**Tier 2 (Framework/Infra):**
- axum 0.7 + tower 0.4 (HTTP framework)
- bollard 0.16 (Docker integration)
- sqlx 0.7 (database, optional)
- redis 0.24 (caching, optional)
- tracing 0.1 + tracing-subscriber (observability)

**Tier 3 (Domain-Specific):**
- pyo3 0.20 + napi 2.0 (language bindings, optional)
- ring 0.17 + aes-gcm 0.10 (crypto)
- x11 2.21 + wayland-client 0.31 (UI automation, optional)
- gstreamer 0.21 (audio/video, optional)

**Assessment:** Dependencies are recent (2024-2025), well-scoped via feature flags, and appropriate for the domain. No major version lags detected.

**Grade:** SHIPPED.

### HONEST-GAPS: Broken (Fundamental Execution Gaps)

**Unresolved Issues:**
1. **Build fails** — cannot compile the crate as-is
2. **Tests don't exist** — 0 test functions; cannot verify any claims
3. **Core modules disabled** — animation_framework, visual_feedback, natural_automation_demo commented out
4. **Syntax errors in files** — tts_audio_system_broken.rs, ffmpeg_pipeline_broken.rs
5. **CI never runs** — GitHub Actions billing constraint makes CI non-functional
6. **No observable usage** — no integration with other Phenotype repos
7. **Documentation-first design** — elaborate spec/plan without implementation evidence

**Contradiction:** PLAN.md claims "Current Status: OPERATIONAL ✅" but the project is explicitly archived ("See Phenotype ecosystem for current tooling") and has 0 passing tests.

---

## Merger Suitability Analysis (phenotype-automation candidate)

### Trait Surface & Extensibility

**Finding:** No public traits defined. Codebase is struct + impl, not trait-based. Example:

```rust
pub struct KVirtualStageCore { ... }
pub struct AutomationEngine { ... }
pub struct DesktopControlManager { ... }
```

**Problem:** Difficult to abstract for cross-platform automation (desktop vs. mobile vs. web). Siblings (kmobile, PlayCua, bare-cua) would need wrapper abstractions.

### Event Model Overlap

**KDesktopVirt events:**
- SecurityEvent (src/security_monitoring.rs)
- AuditEvent (src/audit_compliance.rs)
- (No unified event bus found)

**Sibling repos:**
- kmobile (mobile automation) — no shared event definitions found
- PlayCua (playback automation) — structure unknown
- bare-cua (CUA framework) — unknown

**Problem:** No evidence of cross-repo event model harmonization. Each appears to define events in isolation.

### Bus/Protocol Boundary

**KDesktopVirt exposes:**
1. REST API (Axum, src/api.rs)
2. MCP server (Model Context Protocol, src/mcp.rs)
3. CLI (Clap, src/cli.rs)
4. Language bindings (PyO3, NAPI-RS)
5. WebSocket (Tungstenite)

**Problem:** Five different boundary protocols make it unclear which is the canonical contract for phenotype-automation integration. MCP suggests Claude integration focus, not cross-automation-repo integration.

---

## CONSOLIDATION VERDICT: **ARCHIVE** (Do Not Merge)

### Rationale

1. **Project is already explicitly archived.** README.md states: "This project is no longer maintained. See Phenotype ecosystem for current tooling."

2. **Broken build + zero tests = unproduction-ready.** Cannot integrate code that doesn't compile and has no test evidence.

3. **No trait-based abstraction.** Struct-centric design makes cross-repo composition difficult. phenotype-automation needs polymorphic traits (DesktopAutomator, MobileAutomator, WebAutomator) to unify the family.

4. **Not iteratively developed.** 7 commits of documentation import; no evidence of incremental refinement. Reference exploration, not production infrastructure.

5. **Core modules incomplete.** animation_framework, visual_feedback, natural_automation_demo disabled; tts_audio_system_broken.rs and ffmpeg_pipeline_broken.rs indicate abandoned refactors.

6. **No shared event model with siblings.** kmobile, PlayCua, bare-cua show no cross-repo event coordination.

### Recommendation

- **KDesktopVirt**: ARCHIVE in place. Keep for historical reference (per user's intent).
- **phenotype-automation**: Design fresh with trait-based abstraction:
  ```rust
  pub trait DesktopAutomator {
      async fn launch_session(&self, config: SessionConfig) -> Result<SessionHandle>;
      async fn click(&self, session_id: &str, x: u32, y: u32) -> Result<()>;
      async fn screenshot(&self, session_id: &str) -> Result<ImageBuffer>;
  }
  
  pub trait MobileAutomator { ... }
  pub trait WebAutomator { ... }
  
  pub trait UnifiedAutomationEvent { ... }  // Shared event protocol
  ```
  
- **Reuse selectively**: Extract KDesktopVirt's FFmpeg pipeline, WindMouse algorithm, and security_framework into standalone crates if they mature and show cross-repo demand.

---

## Action Items

1. **Confirm archive status** — Move KDesktopVirt to `.archive/` or mark `is:archived` on GitHub if not already done.
2. **phenotype-automation design** — Create spec for new monorepo with trait-based core (DesktopAutomator, MobileAutomator, WebAutomator, UnifiedEvent).
3. **Sibling assessment** — Audit kmobile, PlayCua, bare-cua, KVirtualStage (if separate) for event model alignment.
4. **FFmpeg extraction** (optional) — If KDesktopVirt's FFmpeg pipeline is valuable, extract to `phenotype-media-core` as standalone crate.

---

**End of Audit**
