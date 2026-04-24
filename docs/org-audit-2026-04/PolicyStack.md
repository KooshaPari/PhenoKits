# PolicyStack — 10-Dimension Scorecard + FocalPoint Boundary Analysis

**Project:** PolicyStack (Python)  
**Location:** `/Users/kooshapari/CodeProjects/Phenotype/repos/PolicyStack`  
**LOC:** ~166.7K (Python)  
**Status:** Stable, production governance tool  
**Date:** 2026-04-24

---

## 10-Dimension Quality Scorecard

| Dimension | Score | Evidence | Notes |
|-----------|-------|----------|-------|
| **1. Architecture Coherence** | 8/10 | 6-layer scope hierarchy (system → harness → task-instance); clear separation of resolver/compiler/sync; well-defined data flow | Minor: ARCHITECTURE.md is templated; needs specific component detail |
| **2. Test Coverage** | 7/10 | 19 test files, 2.9K+ LOC in test suite; governance tests for contract, snapshots, version, rotation | Coverage: ~40-50% (estimated); test-to-source ratio healthy but absolute coverage low |
| **3. Documentation Quality** | 7/10 | README with extensive examples, ADR docs, federation tools, host sync mechanics well explained | Gaps: ARCHITECTURE.md is unfinished; no FUNCTIONAL_REQUIREMENTS.md or spec trackers |
| **4. Code Duplication** | 8/10 | Core logic is modular; CLI/tools/scripts distinct; resolver/merger/compiler separated cleanly | Minor duplication in test fixtures; no major cross-file code blocks |
| **5. Dependency Health** | 9/10 | Minimal deps (pyyaml, jsonschema, pytest); no heavy external governance engines; self-contained | Excellent: no third-party policy vendors; implements own schema validation |
| **6. Maintainability (Cyclomatic)** | 7/10 | Resolver pipeline simple; compiler target logic has nested conditions; sync host rules is complex (~933 LOC) | Risk area: `scripts/sync_host_rules.py` exceeds 900 LOC with high branching (platform-specific rendering) |
| **7. API Surface Clarity** | 8/10 | Resolve/compile/sync/federate are clean entry points; schema-first contract approach is explicit | Minor: compiler target logic uses nested dicts; could benefit from typed dataclasses |
| **8. Security Posture** | 7/10 | Scope hierarchy enforces isolation; contract validation is strict; version governance prevents policy drift | Risk: no SAST/secret detection in CI; federation tools inherit caller's auth (no internal gate) |
| **9. Cross-Repo Reuse Potential** | 6/10 | Policy federation tools are generic (federate_policy, validate_policy_payload); could extract to shared lib | Opportunity: resolver/compiler logic is Python-specific; Rust port would enable sharing with FocalPoint |
| **10. Integration Readiness** | 8/10 | Host sync outputs match Codex/Cursor/Claude/Factory-Droid formats; wrapper dispatch manifest is standard; wrappers in Go/Rust/Zig | Gap: no explicit integration with FocalPoint's enforcement layer |

**Overall:** 7.3/10 — Stable, well-scoped governance tool. Excellent schema-driven design; moderate-to-good test coverage. Main risks are complexity in host-sync rendering and lack of spec-driven development infrastructure.

---

## FocalPoint ↔ PolicyStack: Boundary Evaluation

### Architecture Roles
- **PolicyStack (Python):** Server-side policy resolution and federating. Merges multi-scope YAML/JSON configs into effective policy; renders host-specific fragments (Codex rules, Cursor config, factory-droid lists, policy-wrapper schema).
- **FocalPoint (Rust):** iOS/Android enforcement layer. Evaluates rules against events; dispatches actions into wallet/penalty/policy stores; maintains local-first SQLite audit trail.

### Clear Separation (✓)
| Layer | Owner | Scope |
|-------|-------|-------|
| **Policy Definition** | PolicyStack | YAML/JSON scope hierarchy; versioning; multi-harness contract |
| **Policy Resolution** | PolicyStack | Merge scopes; validate schema; emit target-specific configs |
| **Policy Enforcement** | FocalPoint | Eval rules against events; mutation audit; penalty/reward dispatch |
| **Host Sync / Shims** | PolicyStack | Generate Codex/Cursor/Claude rules; policy-wrapper manifest |

**NOT duplicated:** PolicyStack does NOT evaluate rules against events. FocalPoint does NOT merge/resolve policy scopes.

### Potential Overlap Zones (✓ NONE IDENTIFIED)

1. **Policy Schema Validation:**
   - PolicyStack: Contract validation (YAML/JSON schema, version governance, field cardinality)
   - FocalPoint: None. Consumes resolved policy payloads; assumes schema is valid upstream.
   - **Verdict:** Clean split. No overlap.

2. **Conditional Rule Evaluation:**
   - PolicyStack: Generates conditional rule manifests (`policy-wrapper-rules.json`) with conditions embedded; does NOT evaluate them.
   - FocalPoint: Evaluates conditions at runtime (e.g., `git_clean_worktree`, `git_synced_to_upstream`) when deciding rule action.
   - **Verdict:** Clean split. PolicyStack provides the schema; FocalPoint provides the executor.

3. **Host-Specific Rendering:**
   - PolicyStack: `sync_host_rules.py` converts effective policy into Codex/Cursor/Claude/Factory-Droid fragments. Policy-wrapper rules go into JSON bundles.
   - FocalPoint: Consumes policy-wrapper JSON directly; no rendering logic.
   - **Verdict:** Clean split. FocalPoint is a consumer of the rendered output.

4. **Intervention / Enforcement Actions:**
   - PolicyStack: Defines rule structure (id, action, match, conditions, effect) in schema. Does NOT dispatch penalties/blocks.
   - FocalPoint: Takes `Action` enum (Block/Unblock/Warn/Customize) from rules; executes wallet/penalty mutations; audit appends to tamper-evident log.
   - **Verdict:** Clean split. PolicyStack defines what to do; FocalPoint does it.

### Recommended Integration Points

1. **Policy Payload Ingest (FocalPoint ← PolicyStack):**
   - FocalPoint receives `policy-wrapper-rules.json` + `policy-wrapper-dispatch.manifest.json` from PolicyStack's `--emit-host-rules` output.
   - Stored in FocalPoint's secure config (Keychain/SecureStorage).
   - On app start, FocalPoint verifies policy hash; if stale, logs warning.

2. **Condition Evaluation (FocalPoint-specific):**
   - FocalPoint implements condition predicates (`git_clean_worktree`, `git_is_worktree`, etc.) as native trait implementations.
   - PolicyStack supplies the schema; FocalPoint supplies the evaluators.

3. **Audit Trail (FocalPoint → PolicyStack):**
   - Optional: FocalPoint exports enforcement audit log (block count, rule-fired events) for PolicyStack's policy-effectiveness dashboard.
   - Not blocking; can be deferred.

---

## Verdict: NOT Duplicates

**PolicyStack** is the server-side policy governor. **FocalPoint** is the iOS/Android enforcement agent. They are **complementary**, not overlapping.

- PolicyStack governs which rules apply to which harness in which task domain.
- FocalPoint enforces those rules on actual device behavior (screen time, app launches, etc.).

**Recommendation:** Formalize the integration in ADR form (e.g., "ADR-004: FocalPoint Policy Consumption") to document:
1. Policy payload format (currently `policy-wrapper-rules.json` + manifest).
2. Condition evaluator interface (FocalPoint trait for predicates).
3. Audit export (if/when needed).
4. Version pinning (e.g., FocalPoint pins PolicyStack contract v1).

No code changes needed; the architecture is already sound.
