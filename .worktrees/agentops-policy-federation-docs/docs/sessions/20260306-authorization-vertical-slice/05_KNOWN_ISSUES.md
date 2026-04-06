# Known Issues

- The compiler treats `exec` as the only first-class native action. `write`, `network`, and
  other non-`exec` actions are enforced at runtime.
- Runtime interception semantics are now explicit in compiler shims:
  - all runtime shim entries include `requires_runtime_check: true`.
  - all runtime shim entries include deterministic `actions` arrays for routing.
- `append_unique` merge strategy did not replace nested scalar conflicts. **Resolved**
  - `cli/src/policy_federation/resolver.py::_append_unique_items` now replaces matching
    dict entries with the same `id` so downstream overrides can update nested rules instead of
    duplicating entries.

## Environment-level residuals

- Full launcher runtime smoke checks remain constrained by environment command budget.
- `agentops-policy-federation` is currently a plain directory (no git metadata) at
  this checkout path, so branch/PR operations were completed elsewhere.
