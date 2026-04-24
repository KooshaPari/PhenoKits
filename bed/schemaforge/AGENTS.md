# AGENTS.md

## Project Overview

SchemaForge - A schema generation and validation toolkit for the Phenotype ecosystem.

## Tooling

- **Rust**: Primary implementation language
- **Bun**: Package management and scripting
- **Nextest**: Test execution

## Architecture

```
schemaforge/
├── src/              # Core library
├── tests/            # Integration tests
├── docs/             # Documentation
│   ├── journeys/     # User journeys
│   ├── stories/      # Feature stories
│   └── traceability/ # Traceability matrix
└── schema/           # Schema definitions
```

## Commands

```bash
# Development
cargo test --workspace        # Run all tests
cargo build --release         # Build optimized

# Quality Gates
cargo clippy                 # Lint
cargo fmt --check            # Format check
```

## Standards

- ADR (Architecture Decision Records) for significant changes
- Conventional commits for version control
- Semantic versioning for releases
