# Stashly Release Process

## Versioning Scheme

Stashly uses **Semantic Versioning (SemVer)**:
- Major: Breaking changes to storage trait APIs
- Minor: New storage implementations, backends
- Patch: Performance improvements, bug fixes

Current version: `0.0.1` (pre-release)

## Publish Targets

All crates target **crates.io**:

| Crate | Status | Target |
|-------|--------|--------|
| stashly-cache | alpha | crates.io |
| stashly-eventstore | alpha | crates.io |
| stashly-statemachine | alpha | crates.io |
| stashly-persistence | alpha | crates.io |

## Release Registry

The authoritative registry is maintained in:
- **Location**: `./release-registry.toml` (this directory)
- **Format**: TOML collection manifest with per-crate metadata
- **Schema**: Conforms to `docs/governance/release_registry_schema.md`

## Publish Process

1. **Run storage tests**: `cargo test --workspace`
2. **Benchmark cache tiers**: `cargo bench --package stashly-cache`
3. **Verify event hash chains**: `cargo test --package stashly-eventstore -- --include-ignored`
4. **Update versions** in `Cargo.toml` and `release-registry.toml`
5. **Update CHANGELOG.md** with storage improvements
6. **Tag and publish**: `git tag v<version> && cargo publish ...`

## Release Registry Location

- **File**: `release-registry.toml` (repository root)
- **Format**: TOML
- **Contents**: Storage collection metadata and all workspace crates
- **Update**: When adding storage backends or changing publish targets

## Additional Resources

- **Cache Design**: See `crates/stashly-cache/src/`
- **Event Sourcing**: See `crates/stashly-eventstore/`
- **State Machines**: See `crates/stashly-statemachine/`
