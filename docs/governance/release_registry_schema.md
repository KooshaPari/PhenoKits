# Release Registry Schema

This document defines the structure and semantics of release registries for Phenotype named collections.

## Overview

Release registries enable consumers to:
- Discover all sub-crates within a collection
- Check version and stability information
- Understand release status and maturity
- Locate source repositories

Two registry types:
1. **Collection Registry** — per-collection file (e.g., `Sidekick/release-registry.toml`)
2. **Master Index** — workspace-level index of all collections (`phenotype-collections.toml`)

## Collection Registry Format

Location: `<Collection>/release-registry.toml`

### [collection] Section

Top-level metadata about the collection.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Human-readable collection name (e.g., "Sidekick") |
| `version` | string | Yes | Semantic version of the collection (e.g., "0.0.1") |
| `repo` | string | Yes | GitHub repository URL |
| `description` | string | No | One-line description of the collection's purpose |
| `authors` | string or array | No | Author(s) or organization |
| `license` | string | No | SPDX license identifier (e.g., "Apache-2.0", "MIT OR Apache-2.0") |
| `theme` | string | No | Category label for UI/organization (e.g., "agent utilities", "storage & persistence") |

### [[crate]] Sections

One section per sub-crate. Defines version, status, and stability.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Crate/package name (e.g., "sidekick-dispatch") |
| `version` | string | Yes | Semantic version of this crate (e.g., "0.0.1") |
| `status` | string | Yes | Release status: `stub`, `alpha`, `beta`, `stable` |
| `stability` | string | Yes | Stability marker: `0.x` (breaking), `1.x+` (stable) |
| `description` | string | No | One-line description of crate purpose |
| `source` | string | No | Source repository or project name (origin of code) |
| `type` | string | No | Artifact type: `crate` (Rust), `npm-package`, `vitepress-app` (TypeScript-specific) |

## Status Values

| Status | Meaning | Version | API Stability |
|--------|---------|---------|----------------|
| `stub` | Placeholder; minimal or no implementation | 0.0.1 | Breaking (not usable) |
| `alpha` | Early implementation; API may change | 0.x.0 | Breaking; rapid iteration |
| `beta` | Feature-complete; approaching stabilization | 0.x or 1.0-rc | Minor breaking possible |
| `stable` | Production-ready; semantic versioning enforced | 1.0+ | Semantic versioning (MAJOR.MINOR.PATCH) |

## Master Index Format

Location: `phenotype-collections.toml` (workspace root)

A TOML file with [[collection]] sections referencing each collection registry.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Collection name |
| `registry` | string | Yes | Relative path to the collection's `release-registry.toml` |
| `theme` | string | No | Category label (echoed from collection metadata) |
| `description` | string | No | Brief description of the collection |

## Example: Sidekick Collection

**Collection Registry** (`Sidekick/release-registry.toml`):

```toml
[collection]
name = "Sidekick"
version = "0.0.1"
repo = "https://github.com/KooshaPari/Sidekick"
description = "Agent utility crates"
authors = ["Koosha Pari <kooshapari@gmail.com>"]
license = "Apache-2.0"
theme = "agent utilities"

[[crate]]
name = "sidekick-dispatch"
version = "0.0.1"
status = "alpha"
stability = "0.x"
description = "Multi-provider agent dispatch"
source = "thegent-dispatch"

[[crate]]
name = "sidekick-messaging"
version = "0.0.1"
status = "stub"
stability = "0.x"
description = "Multi-provider messaging"
source = "agent-imessage"
```

**Master Index Entry** (`phenotype-collections.toml`):

```toml
[[collection]]
name = "Sidekick"
registry = "Sidekick/release-registry.toml"
theme = "agent utilities"
description = "Polyglot Rust + Python agent utilities"
```

## Conventions

### Semantic Versioning

- Collections and crates use **semantic versioning** (MAJOR.MINOR.PATCH)
- During `0.x` phase (status: `alpha`), minor version bumps may include breaking changes
- At `1.0+`, follow strict semantic versioning: breaking changes require MAJOR bump

### Stability Fields

- **`0.x` stability** — crate is pre-release; breaking changes allowed, no API guarantees
- **`1.x+` stability** — crate is stable; breaking changes forbidden without MAJOR version bump

### Source Traceability

- `source` field records the original repository or project name from which code was extracted
- Enables tracking of code provenance and simplifies backports

### Type Field (Optional)

Clarifies artifact type for multi-language collections (e.g., Paginary):
- `crate` — Rust workspace crate (default for Rust collections)
- `npm-package` — npm package in a JavaScript/TypeScript monorepo
- `vitepress-app` — VitePress documentation app

## Updating Registries

When releasing new versions:

1. **Update collection `version`** in `[collection]` section
2. **Update per-crate `version`** for each affected `[[crate]]`
3. **Update `status`** if stability changes (e.g., `alpha` → `beta`)
4. **Commit** with message: `chore(releases): update registries for <collection>-v<version>`

Example:

```bash
# Update Sidekick to 0.1.0, promoting dispatch to beta
# Sidekick/release-registry.toml: version = "0.1.0"
# Sidekick/release-registry.toml: crate.sidekick-dispatch status = "beta"

git add Sidekick/release-registry.toml
git commit -m "chore(releases): Sidekick v0.1.0, dispatch beta"
```

## Validation & Tooling

### Manual Validation

```bash
# Check TOML syntax
toml-lint phenotype-collections.toml Sidekick/release-registry.toml
```

### Programmatic Consumption

Registries are TOML; parse with any TOML library:

```rust
use toml::from_str;

let content = std::fs::read_to_string("Sidekick/release-registry.toml")?;
let registry: CollectionRegistry = from_str(&content)?;
println!("{} v{}", registry.collection.name, registry.collection.version);
```

## Related Documentation

- **AgilePlus Spec Governance**: `docs/governance/spec_governance.md`
- **Scripting Policy**: `docs/governance/scripting_policy.md`
- **Phenotype Org Reuse Protocol**: Parent CLAUDE.md (Phenotype/CLAUDE.md)
