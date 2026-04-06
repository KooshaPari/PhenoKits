# AGENTS.md — agileplus-plugin-sqlite

Extends shelf-level agent rules. See `AgilePlus/AGENTS.md` for canonical definitions.

## Project Identity

- **Name**: agileplus-plugin-sqlite
- **Type**: Rust library (AgilePlus SQLite plugin)
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/agileplus-plugin-sqlite`

## AgilePlus Integration

All work MUST be tracked in AgilePlus:
- Reference: `.agileplus/` directory
- CLI: `agileplus <command>` (from project root)
- Specs: `.agileplus/specs/<feature-id>/`

## Quick Commands

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```
