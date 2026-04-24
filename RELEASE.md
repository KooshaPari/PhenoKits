# phenotype-shared Release Process

## Versioning Scheme

phenotype-shared uses **Semantic Versioning (SemVer)**:
- Major: Breaking changes to hexagonal architecture traits or domain-driven design contracts
- Minor: New adapters, persistence implementations, or utility crates
- Patch: Bug fixes, performance improvements, internal refactoring

Current version: `0.1.0` (pre-release)

## Publish Targets

All crates target **crates.io** as the primary registry:

| Crate | Status | Target |
|-------|--------|--------|
| ffi_utils | alpha | crates.io |
| phenotype-application | alpha | crates.io |
| phenotype-cache-adapter | alpha | crates.io |
| phenotype-domain | alpha | crates.io |
| phenotype-event-sourcing | alpha | crates.io |
| phenotype-http-adapter | alpha | crates.io |
| phenotype-policy-engine | alpha | crates.io |
| phenotype-port-interfaces | alpha | crates.io |
| phenotype-postgres-adapter | alpha | crates.io |
| phenotype-redis-adapter | alpha | crates.io |
| phenotype-state-machine | alpha | crates.io |

## Release Registry

The authoritative registry of all packages and their publish metadata is maintained in:
- **Location**: `./release-registry.toml` (repository root)
- **Format**: TOML collection manifest with per-crate metadata and publish targets
- **Schema**: Conforms to `docs/governance/release_registry_schema.md`

## Publish Process

1. **Verify all tests pass**: `cargo test --workspace`
2. **Check hexagonal architecture compliance**: `cargo clippy --workspace -- -D warnings`
3. **Verify domain-driven design contracts**: Manual review of traits in `phenotype-domain`
4. **Update versions** in workspace `Cargo.toml` and `release-registry.toml`
5. **Update CHANGELOG.md** with architecture improvements
6. **Create git tag**: `git tag v<version>`
7. **Publish crates**: `cargo publish --manifest-path crates/<crate>/Cargo.toml` (order: domain → ports → adapters → apps)
8. **Push tags**: `git push origin <tag>`

## Release Registry Location

- **File**: `release-registry.toml` (repository root)
- **Format**: TOML
- **Contents**: Shared infrastructure collection metadata and all 11 workspace crates
- **Update**: When adding new crates or changing publish targets

## Additional Resources

- **Hexagonal Architecture**: See `crates/phenotype-port-interfaces/`
- **Domain-Driven Design**: See `crates/phenotype-domain/`
- **Cargo Publishing**: https://doc.rust-lang.org/cargo/publishing/
- **Crates.io**: https://crates.io/
- **SemVer**: https://semver.org/

## Notes

- This is a foundational infrastructure library shared across the Phenotype ecosystem
- All crates are independent; no inter-crate dependencies required
- Publish in order: domain primitives first, then ports, then adapters, finally application layer
