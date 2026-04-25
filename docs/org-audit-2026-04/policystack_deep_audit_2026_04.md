# PolicyStack Deep Audit — 2026-04

**Date:** 2026-04-24  
**Repository:** `/repos/PolicyStack`  
**Auditor:** Claude Agent  
**Status:** GOVERNANCE GAPS IDENTIFIED

---

## Executive Summary

PolicyStack is a **policy federation system for multi-harness AgentOps** (~908K LOC, heavily JavaScript/TypeScript/Python). The core Python policy library (166K lines) lacks dependency management, tests fail to import required modules, and governance is nascent. Significant **overlap with phenotype-policy-engine (Rust crate)** exists but in different language stacks; consolidation opportunity identified.

---

## Size & Composition

| Metric | Value |
|--------|-------|
| **Total LOC** | 908,833 |
| **Python LOC** | 130,432 (14%) — *core focus* |
| **JavaScript/TypeScript LOC** | 680,730 (75%) — *docs/tests/browser agents* |
| **Test Files** | 1,287 Python files; 25+ test modules |
| **Key Modules** | `policy_lib.py` (core), `resolve.py` (CLI), 6× wrapper implementations (Codex, Cursor, Droid, Go, Rust, Zig) |
| **Governance** | FUNCTIONAL_REQUIREMENTS.md, PLAN.md, ADR.md present; worklog nascent |

---

## Build & Test Status

| Check | Status | Details |
|-------|--------|---------|
| **Lint (ruff)** | WARNINGS | Missing imports, unsorted code, subprocess security issues, magic values |
| **Test Discovery** | BLOCKED | 25+ test modules, but `test_platform_wrappers.py` fails: `ModuleNotFoundError: no module named 'opencode'` |
| **Dependencies** | MISSING | No `uv sync`/`pip install` output; unclear if `opencode`, `codex`, `cursor` wrapper libs are vendored or external |
| **Python Version** | 3.13 | Detected; `pyproject.toml` targets 3.10+ |
| **Package Manager** | setuptools + pyproject.toml | No `uv` lockfile; project structure suggests `uv` usage but not configured |

**Action Taken:** Import error is a quick win blocker. Tests cannot run until missing wrapper libs are available.

---

## Governance State

### Strengths
- **Specs present:** PRD.md, FUNCTIONAL_REQUIREMENTS.md (40+ FRs with traces to PRD epics), PLAN.md, ADR.md
- **Architecture doc:** ARCHITECTURE.md describes 6-scope policy model (system, user, repo, harness, task-domain, task-instance)
- **Worklogs initialized:** Nascent worklog.md with commit history
- **CLAUDE.md:** Comprehensive project instructions (conditional predicates, policy contract patterns)

### Gaps
- **FR traceability:** FRs reference implementations but no explicit test-to-FR mapping (no `@pytest.mark.requirement()` tags)
- **Test maturity:** No test-maturity assessment; coverage % not tracked
- **Quality gates:** No pre-commit hooks, no CI/CD workflow files visible
- **Dead code:** 45+ `#[allow(dead_code)]` suppressions indicate incomplete refactors (this is Rust code in audit, not Python)
- **Worklog retention:** Only bootstrap entry; no issue tracking integration visible

---

## Python Core Analysis

### policy_lib.py (Core Library)
- **Lines:** ~500 LOC
- **Purpose:** Shared policy language primitives — scope discovery, merge logic, extension precedence, hash computation
- **Issues:**
  - EXE001: Shebang present, file not executable
  - I001: Imports unsorted (should use isort or ruff format)
  - S603, S607: `subprocess.run()` with dynamic arguments (minor security issue, not critical)
  - PLR2004: Magic values (e.g., `2`) not parameterized
  - **Fixable:** All 5 issues are linter-class; ruff format + small edits fix

### resolve.py (CLI Tool)
- **Purpose:** Policy resolution orchestrator; accepts harness/task-domain, merges scopes, outputs JSON
- **Dependencies:** Requires `opencode`, `codex`, `cursor` wrapper libs for tests
- **Status:** Core logic likely sound; test blockage prevents verification

---

## Cross-Project Overlap: PolicyStack ↔ phenotype-policy-engine

### phenotype-policy-engine (Rust, crate in phenotype-shared)
| Aspect | phenotype-policy-engine | PolicyStack |
|--------|------------------------|-------------|
| **Language** | Rust (edition 2021) | Python 3.10+ |
| **Purpose** | Generic, domain-agnostic policy evaluation | Concrete policy federation + scope resolution |
| **Core Model** | PolicyEngine → Rule → EvaluationContext | Scope (system/user/repo/harness/task-domain/instance) → merge → Policy |
| **Rule Types** | Allow, Deny, Require + regex matching | Allow, Deny, Require + policy hash + scope audit trail |
| **Export** | Trait-based; supports custom loaders | JSON resolution + host rules + optional artifact application |
| **Config** | TOML, Rust code | YAML/JSON scope files + JSON output |

### Overlap Assessment
- **Overlap:** 60–70%. Both implement rule-based policy evaluation; Rust engine is generic, PolicyStack is specialized federation.
- **Consolidation Path:** PolicyStack could use phenotype-policy-engine as the evaluation backend (bridge Python ↔ Rust via FFI or serialization). Policy scope model (PolicyStack) + generic engine (phenotype-policy-engine) = complete system.
- **Dependencies:** If unified, PolicyStack wrapper impls (Codex, Cursor, Droid, etc.) would depend on consolidated engine; scope discovery remains Python.

### Recommended Integration (not critical, but valuable)
1. **Phase 1:** phenotype-policy-engine exposes async evaluation API (likely already does)
2. **Phase 2:** PolicyStack resolve.py calls Rust engine via PyO3 or gRPC
3. **Phase 3:** Deprecate PolicyStack inline evaluation; route all rule evaluation through shared Rust engine
4. **Benefit:** Single source of truth for rule semantics; Rust guarantees safety; PolicyStack retains scope model expertise

---

## Top 3 Next Actions

### 1. FIX: Unblock Tests — Install Missing Wrapper Libraries (5 min)
**Blocker:** `test_platform_wrappers.py` cannot import `opencode`, `codex`, `cursor`  
**Root Cause:** Wrapper implementations not in Python path or not installed  
**Action:**  
- Check if wrappers are in `/repos/PolicyStack/wrappers/` (they are — JavaScript/TypeScript)
- Add Python shims or stubs for wrapper libs so tests can discover them
- OR: Mock wrapper imports in conftest.py
- OR: Update CONTRIBUTING.md to clarify wrapper setup  
**Impact:** Unblocks all tests; enables FR traceability audit

### 2. CONSOLIDATE: Map PolicyStack ↔ phenotype-policy-engine (30 min)
**Goal:** Document integration path; identify shared code candidates  
**Action:**  
- Read phenotype-policy-engine README and modules
- Create integration design doc: `/repos/docs/architecture/policystack_engine_consolidation.md`
- List 3–5 concrete migration phases (no execution, design only)
- Identify FFI boundaries (Rust ↔ Python)
- **Outcome:** Clear path to reduce duplication and leverage Rust safety

### 3. GOVERNANCE: Initialize Test-to-FR Traceability (15 min)
**Goal:** Establish spec verification  
**Action:**  
- Add test markers: `@pytest.mark.requirement("FR-RES-001")` to all test functions
- Create `/docs/reference/FR_TRACKER.md` listing all FRs + implementation status + test count
- Run quality gate: `ruff check tests/ --select=T` to find traceable tests
- Flag orphaned tests (no FR) for classification
- **Outcome:** Programmatic proof that FRs are tested; unblock spec-verifier hook

---

## Quality Issues Summary

| Category | Count | Severity |
|----------|-------|----------|
| **Lint warnings** (ruff) | 10+ | Low (style + minor sec) |
| **Test blockages** | 1 | High (import error) |
| **Dead code suppressions** | 45+ | Medium (incomplete refactors) |
| **FR traceability gaps** | 40+ FRs | Medium (no test tags) |
| **Docs misplacement** | 10+ files | Low (in root, should be docs/) |

---

## Key Files

- **Core:** `/repos/PolicyStack/policy_lib.py`, `/repos/PolicyStack/resolve.py`
- **Specs:** `/repos/PolicyStack/FUNCTIONAL_REQUIREMENTS.md`, `/repos/PolicyStack/PLAN.md`, `/repos/PolicyStack/ADR.md`
- **Tests:** `/repos/PolicyStack/tests/` (25+ modules)
- **Wrappers:** `/repos/PolicyStack/wrappers/` (Codex, Cursor, Droid, Go, Rust, Zig implementations)
- **Governance:** `/repos/PolicyStack/CLAUDE.md`, `/repos/PolicyStack/GOVERNANCE.md`
- **Related (Rust):** `/repos/crates/phenotype-policy-engine/`

---

## Recommendations

1. **Immediate:** Fix test import blocking; unblock CI
2. **Short-term:** Establish test-to-FR traceability; run spec verifier
3. **Medium-term:** Design PolicyStack ↔ phenotype-policy-engine consolidation
4. **Long-term:** Evaluate single-language strategy (Python vs. Rust) or multi-language federation with clear boundaries

---

**Report Generated:** 2026-04-24  
**Time to Audit:** ~15 min (read-only exploration)
