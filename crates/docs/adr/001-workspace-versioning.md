# ADR 001: Workspace Structure and Versioning Strategy

## Status
========================================

**Accepted**

## Context
========================================

The Phenotype crates ecosystem comprises 40+ interdependent Rust crates spanning foundation, infrastructure, storage, security, and testing layers. As the system scales, we need a consistent approach to:

1. **Version management**: Independent vs. unified versioning across crates
2. **Dependency coordination**: Internal crate dependencies and external dependency alignment
3. **Release cadence**: Synchronized vs. independent release schedules
4. **Breaking change propagation**: Managing SemVer implications across the dependency graph

### Constraints

- Must support both internal monorepo development and external crate consumption
- Must enable rapid iteration on unstable crates while maintaining stability for core crates
- Must integrate with existing CI/CD pipelines
- Must support eventual publication to crates.io for open-source components

### Options Considered

| Option | Description | Pros | Cons |
|--------|-------------|------|------|
| A | Single unified version | Simple releases, atomic changes | Over-releasing stable crates, version churn |
| B | Fully independent versions | SemVer accuracy per crate | Complex coordination, version matrix explosion |
| C | Hybrid: shared for foundation, independent for rest | Balanced approach | Two mental models |
| D | Workspace-coordinated independent versions | Best of both worlds | Tooling required |

## Decision
========================================

We will adopt **Option D: Workspace-coordinated independent versioning**.

### Structure

```
crates/
├── Cargo.toml              # Workspace root with shared config
├── phenotype-core/         # v0.2.x - stable, slow iteration
├── phenotype-errors/        # v0.2.x - follows core
├── phenotype-time/          # v0.2.x - utility crate
├── phenotype-logging/       # v0.3.x - active development
├── phenotype-telemetry/     # v0.1.x - experimental
└── phenotype-mcp-core/      # v0.1.x - rapidly evolving
```

### Key Mechanisms

1. **Workspace inheritance for metadata**

```toml
# Cargo.toml (workspace root)
[workspace.package]
edition = "2021"
rust-version = "1.75"
license = "MIT OR Apache-2.0"
authors = ["Phenotype Labs <dev@phenotype.io>"]
repository = "https://github.com/phenotype-labs/phenotype"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["rt", "macros"] }
```

2. **Independent versions with path dependencies**

```toml
# phenotype-logging/Cargo.toml
[package]
name = "phenotype-logging"
version = "0.3.2"        # Independent version
edition.workspace = true
license.workspace = true

[dependencies]
# External: use workspace version
serde = { workspace = true }

# Internal: path for dev, version for publishing
phenotype-core = { path = "../phenotype-core", version = "0.2.0" }
phenotype-errors = { path = "../phenotype-errors", version = "0.2.0" }
```

3. **Category-based version coordination**

| Category | Versioning | Examples |
|----------|------------|----------|
| Foundation | Conservative, shared minor | core, errors, time |
| Infrastructure | Semi-independent | logging, telemetry, config |
| Domain-specific | Fully independent | mcp-core, security-aggregator |
| Testing | Dev-only, unversioned | testing, test-fixtures |

## Consequences
========================================

### Positive

1. **Accurate SemVer**: Each crate accurately reflects its own stability and API evolution
2. **Reduced churn**: Stable foundation crates don't bump unnecessarily
3. **Flexible iteration**: Experimental crates can iterate rapidly without affecting stable ones
4. **Clear contracts**: Version numbers communicate stability expectations
5. **Workspace consistency**: Shared external dependencies reduce version conflicts

### Negative

1. **Cognitive overhead**: Developers must understand multiple versioning tracks
2. **Coordination complexity**: Cross-crate breaking changes require synchronized releases
3. **Tooling dependency**: Requires cargo-smart-release or similar for automation
4. **Documentation burden**: Must document version compatibility matrices

### Mitigations

1. **Automated change detection**: Use `cargo-smart-release` to detect affected crates from Git history
2. **Compatibility CI**: Test all dependent crates on PR to catch breaking changes
3. **Version dashboard**: Automated tracking of version states across the ecosystem

## Implementation
========================================

### Phase 1: Tooling Setup

```bash
# Install release automation
cargo install cargo-smart-release

# Configure GitHub Actions for coordinated releases
```

### Phase 2: Migration

```bash
# Assign initial versions based on maturity
# Foundation crates: 0.2.0 (existing stability)
# Infrastructure: 0.1.0 (declared stable enough)
# Experimental: 0.0.1 (explicitly unstable)

# Update all Cargo.toml files with workspace inheritance
```

### Phase 3: Automation

```yaml
# .github/workflows/release.yml
name: Release

on:
  workflow_dispatch:
    inputs:
      bump:
        type: choice
        options: [patch, minor, major]

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      
      - name: Detect changes
        run: cargo smart-release --bump ${{ github.event.inputs.bump }} --no-confirm --no-publish
      
      - name: Run tests
        run: cargo test --workspace
      
      - name: Publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
        run: cargo smart-release --bump ${{ github.event.inputs.bump }} --no-confirm --execute
```

## References
========================================

- RFC: Cargo workspaces (rust-lang/rfcs#1525)
- Tool: cargo-smart-release (https://github.com/Byron/cargo-smart-release)
- Tool: cargo-workspaces (https://github.com/pksunkara/cargo-workspaces)
- Research: [RUST_PACKAGING_SOTA.md](./RUST_PACKAGING_SOTA.md)

## Decision Log
========================================

| Date | Event |
|------|-------|
| 2024-01-15 | Initial proposal |
| 2024-01-22 | Reviewed with team; concerns about complexity raised |
| 2024-01-29 | Hybrid approach accepted; automation requirements defined |
| 2024-02-05 | ADR accepted; implementation scheduled |

**Traceability:** `/// @trace CRATES-ADR-001`
