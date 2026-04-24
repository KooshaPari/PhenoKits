# Observably Release Process

## Versioning Scheme

Observably uses **Semantic Versioning (SemVer)**:
- Major: Breaking changes to telemetry APIs
- Minor: New telemetry backends, instrumentation patterns
- Patch: Bug fixes, performance tuning

Current version: `0.0.1` (pre-release)

## Publish Targets

All crates target **crates.io**:

| Crate | Status | Target |
|-------|--------|--------|
| observably-tracing | alpha | crates.io |
| observably-logging | alpha | crates.io |
| observably-metrics | alpha | crates.io |

## Release Registry

The authoritative registry is maintained in:
- **Location**: `./release-registry.toml` (this directory)
- **Format**: TOML collection manifest with per-crate metadata
- **Schema**: Conforms to `docs/governance/release_registry_schema.md`

## Publish Process

1. **Verify telemetry exports**: `cargo test --workspace`
2. **Check span collection**: `cargo run --example tracing --package observably-tracing`
3. **Update versions** in `Cargo.toml` and `release-registry.toml`
4. **Update CHANGELOG.md** with telemetry enhancements
5. **Create and push tags**: `git tag v<version> && git push origin <tag>`
6. **Publish**: `cargo publish --manifest-path crates/<crate>/Cargo.toml`

## Release Registry Location

- **File**: `release-registry.toml` (repository root)
- **Format**: TOML
- **Contents**: Observability collection metadata and all workspace crates
- **Update**: When adding telemetry backends or changing publish targets

## Additional Resources

- **OpenTelemetry**: https://opentelemetry.io/
- **Tracing Crate**: https://docs.rs/tracing/
- **Structured Logging**: See `crates/observably-logging/`
