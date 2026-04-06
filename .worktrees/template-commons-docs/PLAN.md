# PLAN.md — template-commons

## Phase 1: Scaffold
- Define layer contract manifest (`template.manifest.json`)
- Establish reconcile rules (`reconcile.rules.yaml`)
- Set up `task check` quality gate
- Create minimal docs (`index.md`, `UPGRADE.md`, `BRANCH_PROTECTION.md`)

## Phase 2: Validation
- Verify all downstream language layers compose cleanly on commons primitives
- Run `task quality` across manifest, docs, and contracts
- Validate semver bump workflow end-to-end
- Ensure pinned-version consumption works from domain templates

## Phase 3: Stabilization
- Freeze public contract surface — no breaking changes without major bump
- Add integration tests with each language layer (go, python, typescript, rust, zig, swift, kotlin, mojo, elixir)
- Document cross-layer dependency graph
- Establish changelog discipline for contract-affecting changes

## Phase 4: Release
- Tag initial semver release
- Publish to internal package index
- Update downstream domain templates to pin released version
- Announce layer availability to template-program-ops
