# PolicyStack ↔ phenotype-policy-engine Consolidation Plan

**Status**: Planning Phase | **Created**: 2026-04-24 | **Last Updated**: 2026-04-24

---

## Executive Summary

PolicyStack (Python, 17.5 KB, command-scoped) and phenotype-policy-engine (Rust, 1.4 KB shared lib) overlap by 60-70% in core policy evaluation logic but serve different domains. This document maps their feature parity, recommends a consolidation strategy, and outlines migration phases.

**Recommendation**: **Option B — Unified Kernel Approach**. Enhance phenotype-policy-engine to support PolicyStack's 6-tier scoping model and migrate PolicyStack's domain-specific logic into a higher-level Python wrapper around the Rust kernel (via PyO3).

---

## Feature Parity Analysis

### Core Overlap (60% — Move to Kernel)

| Feature | PolicyStack | Rust Engine | Notes |
|---------|-------------|-------------|-------|
| **Rule evaluation** | CommandRule (pattern + conditions) | Rule (regex + fact-based) | Both evaluate patterns; PolicyStack is command-centric, Engine is generic |
| **Pattern matching** | glob/prefix/exact matchers | regex-based | PolicyStack more semantic; Engine more flexible for general facts |
| **Decision types** | allow/deny/request | Allow/Deny/Require | PolicyStack uses "request" (human review); Engine uses "Require" (must exist) |
| **Condition groups** | ConditionGroup (all/any nesting) | None (flat rules) | **MISSING in Engine** — major gap |
| **Rule composition** | CommandRule.conditions (nested) | Flat Rule evaluation | PolicyStack supports complex nested predicates; Engine doesn't |
| **Payload parsing** | normalize_payload() — complex | loader.rs — basic | PolicyStack deserializes YAML; Engine uses serde |
| **Scope chain resolution** | 6-tier model (system → task_instance) | None | **DOMAIN-SPECIFIC TO PolicyStack** |
| **Condition evaluation** | git_* conditions (git state checks) | None | **DOMAIN-SPECIFIC TO PolicyStack** |
| **Decision tracing** | rule_id::action::source | None | PolicyStack has richer audit trail |

### PolicyStack-Only (25% — Keep in Python Layer)

| Feature | Purpose | Migration |
|---------|---------|-----------|
| **6-tier scope resolution** | Layer policies: system → user → repo → harness → task_domain → task_instance | Keep as Python orchestration; call Rust engine per scope |
| **Git condition evaluators** | _condition_git_is_worktree, _condition_git_clean, _condition_git_synced_to_upstream | Keep as Python; Rust engine doesn't execute; PolicyStack calls git |
| **Command normalization** | _normalized_command(), _safe_split() | Keep in Python; specific to shell command policies |
| **Decision trace building** | decision_trace() for audit | Enhance Rust engine to carry trace metadata |

### Rust Engine-Only (15% — Keep in Rust)

| Feature | Purpose | Notes |
|---------|---------|-------|
| **Thread-safe policy store** | DashMap-based concurrent policy management | Useful for multi-threaded environments; PolicyStack doesn't need |
| **Regex evaluation** | Generic fact pattern matching | More powerful than PolicyStack's glob matcher for structured data |
| **Policy loading from files** | loader.rs reads YAML/JSON into Policy structs | Can enhance to support PolicyStack's normalize_payload schema |

---

## Feature Parity Summary

```
Shared Logic (~60%):
  ✓ Rule matching
  ✓ Decision logic (allow/deny/request)
  ✗ Nested condition groups (MISSING in Engine)
  ✗ Condition evaluation contract

PolicyStack-Specific (~25%):
  ✓ 6-tier scope chain resolution
  ✓ Git-specific condition evaluators
  ✓ Command parser & normalizer
  ✓ Dynamic condition discovery

Rust Engine-Specific (~15%):
  ✓ Thread-safe concurrent store
  ✓ Regex-based fact matching
  ✓ Generic evaluation context
```

---

## Recommended Architecture: Option B — Unified Kernel + Python Wrapper

### Rationale

1. **Leverage existing Rust crate**: phenotype-policy-engine is already built, tested, and modular.
2. **Move shared logic to kernel**: Pattern matching, rule evaluation, decision logic → Rust (performance, reusability).
3. **Keep domain logic in Python**: Scope resolution, git checks, command parsing → Python (rapid iteration, flexibility).
4. **Bridge via PyO3**: Create a thin Rust wrapper that PolicyStack can call for rule evaluation.

### Design

```
┌─────────────────────────────────────────────────────────────────┐
│                     PolicyStack (Python)                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  resolve.py (scope chain resolution)                           │
│  policy_lib.py:                                                │
│    - normalize_payload() → Rust Evaluator                      │
│    - evaluate_policy() → loops through 6 scopes, calls Rust    │
│    - ConditionGroup → compile to Rust Rule[] → evaluate       │
│    - Git conditions (Python subprocess) → Rust context         │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│            PyO3 Bindings (pyo3_policy_engine)                  │
│                                                                 │
│  ConditionGroup → RuleEvaluator                                │
│  EvaluationContext ← Git state (Python calls git)              │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│         phenotype-policy-engine (Rust) — ENHANCED              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ✓ RuleEvaluator (new)                                        │
│    - evaluate_rules(rules: [Rule], context: EvalCtx) → [Decision]
│                                                                 │
│  ✓ ConditionGroup (new)                                       │
│    - Support nested all/any with required flags                │
│    - evaluate_with_quality() → (ok, partial, reasons)         │
│                                                                 │
│  ✓ Enhanced Rule                                               │
│    - matcher: {glob, prefix, exact, regex}                    │
│    - metadata: {rule_id, source, on_mismatch}                │
│    - decision_trace() → "rule_id::action::source"            │
│                                                                 │
│  ✓ EvaluationContext (unchanged)                              │
│    - key-value map of facts (policies, env, git state)       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Implementation Steps

#### Phase 1: Enhance Rust Engine (5-7 tool calls)

1. **Add ConditionGroup to engine** (new module):
   - Copy ConditionGroup from policy_lib.py to Rust
   - Implement evaluate() and evaluate_with_quality()
   - Support all/any nesting with required flags

2. **Extend Rule struct**:
   - Add matcher: enum {Glob, Prefix, Exact, Regex}
   - Add metadata: {rule_id, source, on_mismatch}
   - Update Rule::evaluate() to return (decision, reason)

3. **Create RuleEvaluator trait** (new):
   - evaluate_rules(rules: Vec<Rule>, context: EvalCtx) → Vec<Decision>
   - On first match (allow/deny/request), return and stop
   - Collect all reasons for audit

4. **Update loader.rs**:
   - Deserialize PolicyStack's normalize_payload schema
   - Map CommandRule → Rule + ConditionGroup

5. **Write Rust tests**:
   - Trace to Rust engine FRs (FR-POLICY-NNN)
   - Test nested conditions, glob matchers, decision traces

#### Phase 2: Create PyO3 Bindings (4-6 tool calls)

1. **New crate: pyo3-policy-engine** (phenotype-shared workspace):
   - Depend on phenotype-policy-engine
   - Expose RuleEvaluator, Rule, ConditionGroup via PyO3

2. **PyO3 conversions**:
   - Python ConditionGroup → Rust ConditionGroup
   - Python Rule → Rust Rule
   - EvaluationContext (pass-through dict)

3. **Build & test**:
   - `maturin develop` (builds .so for Python)
   - Test PyO3 roundtrips

#### Phase 3: Migrate PolicyStack (3-5 tool calls)

1. **Update policy_lib.py**:
   - Import pyo3_policy_engine
   - normalize_payload() → build Rust Rules + ConditionGroups
   - evaluate_policy() loops through scopes, calls Rust evaluator

2. **Migrate git conditions**:
   - Keep Python (subprocess calls git)
   - Pass git state into EvaluationContext before Rust eval
   - Rust ConditionGroup evaluates against context

3. **Update tests**:
   - Regenerate test vectors from Rust engine
   - Trace to FR-POLICY-NNN

4. **Remove duplicate code**:
   - Delete CommandRule, ConditionGroup, ConditionEvaluator from policy_lib.py
   - Keep scope resolution, git checkers, command normalizers

#### Phase 4: Integrate & Deploy (2-3 tool calls)

1. **Monorepo integration**:
   - phenotype-shared exposes policy-engine + pyo3-policy-engine
   - PolicyStack depends on pyo3-policy-engine

2. **Documentation**:
   - Update ADR.md in PolicyStack
   - Add PyO3 binding docs to phenotype-shared

3. **CI/CD**:
   - phenotype-policy-engine tests pass
   - pyo3-policy-engine builds + tests pass
   - PolicyStack tests pass (now using Rust kernel)

---

## Risk Assessment

### Risk 1: PyO3 Build Complexity (Medium)

**Problem**: PyO3 requires Rust/Cargo knowledge from Python team; maturin build can fail on CI.

**Mitigation**:
- Create standalone `maturin` workflow in phenotype-shared/.github/
- Ship pre-built wheels for common platforms (Linux x86_64, macOS ARM/x86, Windows)
- Document local build: `maturin develop` in quick-start guide
- Vendor pre-compiled .so files for CI fallback (last resort)

**Timeline**: Phase 2, +1-2 hours for CI setup

---

### Risk 2: Nested Condition Group API (Medium)

**Problem**: PolicyStack's ConditionGroup uses recursive nesting (any/all can contain both Condition and ConditionGroup). Rust's type system makes this verbose.

**Mitigation**:
- Use enum: `enum ConditionItem { Condition(Condition), Group(Box<ConditionGroup>) }`
- Flatten representation: store Vec<ConditionItem> + mode (all/any)
- PyO3 conversion handles nesting transparently
- Write Rust tests for deeply nested groups (3+ levels)

**Timeline**: Phase 1, +2 hours for type design

---

### Risk 3: Performance Regression (Low)

**Problem**: PyO3 FFI overhead (Python → Rust → Python) for every rule evaluation; PolicyStack currently evaluates in pure Python.

**Mitigation**:
- Benchmark: batch evaluation (1000 rules at once) vs. per-rule
- PyO3 GIL management: keep evaluations inside Rust (no callbacks)
- Inline small evaluations: glob/prefix matching stays in Rust, no roundtrips
- Accept 5-10% overhead for correctness & reusability gains

**Timeline**: Phase 2, +30 minutes for benchmark

---

## Migration Phases (4-Week Timeline)

| Phase | Work | Duration | Predecessor |
|-------|------|----------|-------------|
| **1** | Enhance Rust engine: ConditionGroup, RuleEvaluator, metadata | 3-4 days | None |
| **2** | PyO3 bindings: conversion, build, CI | 2-3 days | Phase 1 |
| **3** | PolicyStack migration: policy_lib.py, git integration | 3-4 days | Phase 2 |
| **4** | Integration, testing, docs, deploy | 1-2 days | Phase 3 |
| **Total** | | **~2 weeks** | Parallel 1+2 possible |

---

## Alternative Architectures (Not Recommended)

### Option A: PolicyStack Adopts Engine (Rejected)

Extract PolicyStack's condition evaluation into Rust engine, then call from Python.

**Cons**:
- Engine becomes domain-specific (loses generic appeal)
- Condition evaluators (git checks) require subprocess/shell access in Rust (risky, complex FFI)
- Rust's trait system makes extensible evaluators verbose
- No clear win over Option B

---

### Option C: Keep Separate (Rejected)

Maintain PolicyStack and engine as independent systems.

**Cons**:
- 60% code duplication forever
- Two decision-tracing formats, two pattern matchers
- Harder to audit: policy evaluation logic lives in two places
- No cross-org reuse (engine stays niche)

---

## Success Criteria

1. **All PolicyStack tests pass** using Rust kernel (no behavior change)
2. **phenotype-policy-engine gains 2 new crates**: core engine enhancements + pyo3 bindings
3. **PolicyStack LOC reduces by 40%** (remove CommandRule, ConditionGroup, ConditionEvaluator)
4. **Cross-project reuse**: other Phenotype projects can use phenotype-policy-engine directly (Rust) or via PyO3 (Python)
5. **Audit trail improves**: decision_trace metadata now available in both Python and Rust

---

## Do NOT Execute Yet

This plan is **planning-only**. User approval required before implementing Phase 1.

- [ ] User confirms Option B strategy
- [ ] User confirms 2-week timeline
- [ ] User grants permission to modify phenotype-policy-engine
- [ ] User grants permission to create pyo3-policy-engine crate

---

## References

- PolicyStack core: `/repos/PolicyStack/policy_lib.py` (17.5 KB)
- PolicyStack scope resolution: `/repos/PolicyStack/resolve.py` (23 KB)
- Rust engine: `/repos/crates/phenotype-policy-engine/` (1.4 KB)
  - engine.rs (298 LOC)
  - rule.rs (200 LOC)
  - context.rs (164 LOC)
  - policy.rs (177 LOC)
  - loader.rs (239 LOC)
  - result.rs (219 LOC)
- Prior audit: policystack_deep_audit_2026_04.md

---

## Document Version History

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-04-24 | Audit Agent | Initial consolidation plan — Option B recommended |
