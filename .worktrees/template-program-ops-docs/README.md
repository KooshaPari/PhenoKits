# template-program-ops

Control plane for the Phenotype layered template program.

## Layer Contract

- layer_type: program_ops
- layer_name: template-program-ops
- versioning: semver

## Mission

Define and govern reusable template layers with minimal duplication:

1. `template-commons`
2. `template-lang-python`
3. `template-lang-go`
4. `template-lang-typescript`
5. `template-lang-rust`
6. `template-lang-mojo`
7. `template-lang-zig`
8. `template-lang-swift`
9. `template-lang-kotlin`
10. `template-lang-elixir-hex`
11. domain template repos (wave 2)

## Spec Kitty Workflow

```bash
spec-kitty research --feature layered-template-platform --force
```

Primary feature workspace:

- `kitty-specs/layered-template-platform/`

## Operational Workflow

1. Run `task check` before release.
2. Keep manifest/reconcile files aligned for any contract-affecting change.
3. Run `task release:prep` as final pre-release gate.

## Outputs

- Layer contract specs
- WP DAG and execution lanes
- Reconcile contract and acceptance criteria
