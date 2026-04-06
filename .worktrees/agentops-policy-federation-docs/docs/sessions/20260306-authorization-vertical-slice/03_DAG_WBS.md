# DAG / WBS

1. Extend schema and semantic validation for `policy.authorization`. Completed.
2. Implement normalized rule model and evaluator. Completed.
3. Implement target compiler for native vs shim rule split. Completed.
4. Seed layered policies with realistic guardrails. Completed.
5. Add focused tests for precedence and compilation. Completed.
6. Add Codex runtime interception and shell wrapper. Completed.
7. Validate CLI and wrapper flows and document residual gaps.
- Status: completed
- Predecessors: 1, 2, 3, 4, 5, 6
- Result:
  - `policyctl intercept` and `policyctl uninstall` round-trips executed in `agentops-policy-federation` and passed command-level validation.
  - `policyctl install-runtime` and `policyctl uninstall-runtime` execute successfully against launcher and config backup surfaces.
  - Residual gaps documented in `05_KNOWN_ISSUES.md` (droid import runtime and cursor CLI smoke validation timeout).

8. Expand e2e validation matrix (live behavior).
- Status: completed
- Predecessors: 7
- Plan:
- Completed:
  - Added `scripts/validate_e2e_matrix.py`.
  - Executed end-to-end matrix across codex/cursor/factory/claude for:
    - exec allow, deny, ask
    - write deny
    - network ask
  - Persisted results at:
    - `docs/sessions/20260306-authorization-vertical-slice/artifacts/e2e_matrix.json`
    - `docs/sessions/20260306-authorization-vertical-slice/artifacts/e2e_matrix.md`

9. Resolve launcher runtime blockers.
- Status: completed
- Predecessors: 8
- Plan:
  - Added fallback launcher dispatch (`command -v <fallback>`) in `cli/src/policy_federation/integrations.py` for
    `codex`, `cursor`, `droid`, and `claude`.
  - Added runtime integration test coverage asserting fallback dispatch in generated launcher wrappers.
  - Re-ran wrapper patch/unpatch and matrix checks against the installed matrix surface.

10. Add adapter coverage and parity.
- Status: completed
- Predecessors: 9
- Plan:
  - Expanded compiler tests to assert compile parity across all harness targets for:
    - native exec allow/ask handling
    - runtime-shim routing of non-exec actions (`write`, `network`, `shutdown`)
    - conditional and missing command pattern handling
  - Added assertions for factory ask-native conversion and wrapper-specific permission dialects.

11. Resolve nested scalar conflict behavior for `append_unique` merge strategy.
- Status: completed
- Predecessors: 10
- Plan:
  - Updated `_append_unique_items` to replace existing dict items with the same `id` in append-unique merges.
  - Added unit tests for:
    - id-based replacement in `_append_unique_items`
    - `_merge_maps(..., strategy=\"append_unique\")` behavior for policy rule layers
  - Re-ran unit suite + compile + matrix validator.

12. Validate `extends` inheritance and override behavior at resolve time.
- Status: completed
- Predecessors: 11
- Plan:
  - Added resolver regression test for parent-child task-domain policy inheritance with `extends`.
  - Verified child rule overrides parent by `id` and preserves merged layer ordering.

13. Make runtime-only compile artifacts explicit for interception.
- Status: completed
- Predecessors: 12
- Plan:
  - Updated compiler shim output so all runtime-interceptor rules set `requires_runtime_check: true`.
  - Preserved runtime wrapper routing metadata while adding deterministic `actions` values on all shim rules.
  - Added unit assertions that runtime shims (including conditional/missing-command and ask-conversion cases) carry runtime-check markers and canonical actions.
