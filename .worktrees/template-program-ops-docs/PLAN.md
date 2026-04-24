# PLAN.md — template-program-ops

## Phase 1: Scaffold
- Define control plane manifest for all template layers
- Set up spec-kitty workspace (`kitty-specs/layered-template-platform/`)
- Establish WP DAG and execution lanes
- Create `task check` and `task release:prep` quality gates

## Phase 2: Validation
- Verify all 10+ language layer contracts are well-formed
- Run cross-layer composition checks (commons → lang → domain)
- Validate reconcile rules alignment across all layers
- Ensure spec-kitty research workflow produces actionable specs

## Phase 3: Stabilization
- Freeze layer contract schema — no structural changes without migration plan
- Add integration tests: full scaffold cycle for each domain template
- Document dependency graph and version pinning strategy
- Establish release cadence and changelog aggregation

## Phase 4: Release
- Tag initial program-wide release
- Publish layer registry and cross-reference docs
- Announce template program availability to Phenotype org
- Establish ongoing governance runbook
