# Stashly — Agent Instructions

## Overview

Stashly is a productized collection of storage & persistence crates. AI agents working here should:

1. Maintain independent crate structure (no inter-crate dependencies)
2. Update Cargo.toml workspace with new crates
3. Add new crates to README.md table with provenance notes
4. Verify `cargo check --workspace` and `cargo test --workspace` pass
5. Track all work in AgilePlus per CLAUDE.md

## Typical Tasks

- **Add a new crate**: Copy source, update workspace Cargo.toml, update README, test
- **Update dependencies**: Edit workspace.dependencies, verify all crates build
- **Documentation**: Add extraction notes to docs/EXTRACTION_PLAN.md
- **Quality**: Run `cargo clippy --workspace` and `cargo fmt` before committing

## Code Organization

```
Stashly/
├── Cargo.toml (workspace manifest)
├── README.md (product description + crate table)
├── CHANGELOG.md
├── AGENTS.md (this file)
├── CLAUDE.md (project governance)
├── docs/
│   └── EXTRACTION_PLAN.md
└── crates/
    ├── stashly-cache/
    ├── stashly-eventstore/
    ├── stashly-statemachine/
    └── [future crates]
```

## Quality Gate

All work must pass:
```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```
