# ADR 002: Dependency Resolution and Registry Strategy

## Status
========================================

**Accepted**

## Context
========================================

Phenotype requires a robust strategy for managing dependencies across internal crates and external ecosystem dependencies. The decision impacts:

1. **Build reproducibility**: Ensuring builds are deterministic and auditable
2. **Supply chain security**: Protecting against dependency confusion and compromised packages
3. **Developer experience**: Speed of dependency resolution and builds
4. **Deployment flexibility**: Support for air-gapped environments and private registries

### Current State

- All crates use path dependencies for internal references
- External dependencies reference crates.io via default registry
- No lockfile enforcement in CI
- No private registry for internal-only crates
- ad-hoc Git dependencies for temporary forks

### Requirements

| Requirement | Priority | Description |
|-------------|----------|-------------|
| R1 | Must | Reproducible builds with locked dependencies |
| R2 | Must | Private registry support for internal crates |
| R3 | Should | Supply chain security scanning |
| R4 | Should | Fast dependency resolution and caching |
| R5 | Could | Air-gapped / offline build support |
| R6 | Could | Mirror/fallback registry configuration |

## Decision
========================================

We will implement a **hybrid registry strategy** with the following components:

### 1. Multi-Registry Configuration

```toml
# .cargo/config.toml (workspace root)
[registries]
crates-io = { index = "sparse+https://index.crates.io/" }
phenotype-private = { index = "https://registry.internal.phenotype.io/git/index" }

[net]
retry = 3                   # Retry failed downloads
git-fetch-with-cli = true  # Use system Git for large repos
```

### 2. Dependency Categorization and Sources

| Category | Source | Examples |
|----------|--------|----------|
| External stable | crates.io | serde, tokio, axum |
| External pinned | crates.io + lockfile | Critical dependencies |
| Internal shared | phenotype-private | phenotype-core, phenotype-errors |
| Internal team | Git references | Team-specific experiments |
| Forked | Git + patch | Temporary forks with upstream fixes |

### 3. Sparse Index Protocol

Adopt Cargo's sparse index for all registry access:

```toml
# Cargo.toml (workspace root)
[workspace]
members = ["crates/*"]

# Enforces sparse index usage (faster, HTTP-only)
[registries.crates-io]
protocol = "sparse"
```

Benefits:
- No Git required for registry operations
- Faster cold-start dependency resolution
- Better CDN integration
- Reduced disk space usage

### 4. Lockfile Enforcement

CI/CD will enforce locked builds:

```yaml
# .github/workflows/ci.yml
jobs:
  build:
    steps:
      - run: cargo build --locked
        # Fails if Cargo.lock needs updates
```

With automated lockfile update workflow:

```yaml
# Weekly dependency updates
- name: Update dependencies
  run: |
    cargo update
    cargo test --workspace
    # Create PR if tests pass
```

### 5. Vendoring for Air-Gapped Builds

Support vendoring for restricted environments:

```bash
# Create vendored distribution
cargo vendor --respect-source-config > .cargo/config.toml

# Result: vendor/ directory with all dependencies
tar czf phenotype-crates-vendored.tar.gz crates/ Cargo.lock .cargo/config.toml vendor/
```

### 6. Private Registry Implementation

Deploy **ktra** for internal registry:

```yaml
# docker-compose.yml for private registry
version: '3'
services:
  registry:
    image: ghcr.io/moritzbuehrer/ktra:latest
    environment:
      - KTRA_INDEX_REMOTE_URL=https://github.com/phenotype/crates-index
      - KTRA_CRATES_IO_TOKEN=${CRATES_IO_TOKEN}
    volumes:
      - ./data:/app
    ports:
      - "8000:8000"
```

Registry selection per crate:

```toml
# phenotype-internal-tool/Cargo.toml
[package]
name = "phenotype-internal-tool"
version = "0.1.0"
publish = ["phenotype-private"]  # Only publish to internal registry
```

## Consequences
========================================

### Positive

1. **Build reproducibility**: Lockfile enforcement ensures consistent builds
2. **Performance**: Sparse index protocol reduces fetch times
3. **Security**: Private registry prevents dependency confusion
4. **Flexibility**: Vendoring enables air-gapped deployments
5. **Compliance**: Audit trail for all dependencies

### Negative

1. **Infrastructure cost**: Private registry requires hosting and maintenance
2. **Complexity**: Multiple registry configurations increase cognitive load
3. **Lockfile conflicts**: Merge conflicts in Cargo.lock require resolution
4. **Vendor size**: Vendored dependencies increase distribution size

### Mitigations

1. **Registry mirroring**: Use pull-through cache for crates.io to reduce external dependency
2. **Lockfile automation**: GitHub Actions to auto-update and test lockfile changes
3. **Documentation**: Clear registry selection guidelines for developers

## Implementation
========================================

### Phase 1: Sparse Index Migration

```toml
# .cargo/config.toml
[registries.crates-io]
protocol = "sparse"
```

Update CI to use latest Cargo (1.70+ for sparse index).

### Phase 2: Private Registry Deployment

```bash
# Deploy ktra
kubectl apply -f ktra-deployment.yaml

# Configure authentication
cargo login --registry phenotype-private --token ${TOKEN}

# Test publish
cargo publish -p phenotype-core --registry phenotype-private --dry-run
```

### Phase 3: Security Scanning Integration

```yaml
# .github/workflows/security.yml
jobs:
  audit:
    steps:
      - uses: rustsec/audit-check@v1
      - uses: EmbarkStudios/cargo-deny-action@v1
        with:
          command: check licenses advisories
```

### Phase 4: Vendoring Support

```bash
# Makefile target for vendored builds
vendor:
	mkdir -p dist
	cargo vendor dist/vendor
	cp Cargo.lock dist/
	tar czf phenotype-crates-vendored.tar.gz dist/
```

## Alternatives Considered
========================================

### Alternative A: Git Dependencies Only

Use Git references for all internal dependencies:

```toml
[dependencies]
phenotype-core = { git = "https://github.com/phenotype/crates", tag = "v0.2.0" }
```

**Rejected**: Git dependencies don't support version resolution, feature unification, or sparse index benefits.

### Alternative B: Single Public Registry

Publish all crates to crates.io, including internal ones:

```toml
[package]
name = "phenotype-internal"
publish = true  # On crates.io
```

**Rejected**: Exposes internal implementation details, requires crates.io namespace reservation, and prevents internal-only APIs.

### Alternative C: Full Vendoring

Vendor all dependencies permanently:

```
crates/
  vendor/
    serde-1.0.193/
    tokio-1.35.0/
    ...
```

**Rejected**: Vendor directory is large (GBs), complicates security updates, and makes dependency updates difficult.

## References
========================================

- Cargo sparse index: https://blog.rust-lang.org/2023/03/09/cargo-sparse-protocol.html
- ktra registry: https://github.com/moritzbuehrer/ktra
- cargo-vendor: https://doc.rust-lang.org/cargo/commands/cargo-vendor.html
- Supply chain security: https://salsa.debian.org/debian/rust-supply-chain

## Decision Log
========================================

| Date | Event |
|------|-------|
| 2024-02-01 | Initial proposal: sparse index + private registry |
| 2024-02-08 | Evaluated Cloudsmith vs self-hosted ktra; selected ktra for cost |
| 2024-02-15 | Added vendoring requirement for compliance |
| 2024-02-20 | ADR accepted; implementation begins Q1 |

**Traceability:** `/// @trace CRATES-ADR-002`
