# CLAUDE.md — Settly

## Project Identity

- **Name**: Settly
- **Description**: Universal configuration management with layered configs, validation, and environment support
- **Location**: `remote-clones/Settly/`
- **Language**: Rust
- **License**: MIT OR Apache-2.0

## Architecture

Hexagonal (Ports & Adapters):
- Config trait is the port (interface)
- Loader implementations are adapters (file loaders, env vars, etc.)
- Validator for configuration validation
- Layer system for config precedence

## Quick Commands

```bash
# Build
cargo build

# Test
cargo test

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --check

# Documentation
cargo doc --open
```

## Key Files

| Path | Purpose |
|------|---------|
| `src/lib.rs` | Main library entry |
| `src/loader.rs` | Configuration loading |
| `src/validator.rs` | Config validation |
| `src/layer.rs` | Configuration layering |
| `src/format/` | Format parsers (TOML, YAML, JSON) |

## Testing Requirements

- Unit tests for all public APIs
- Property-based tests for config merging
- Integration tests for format parsers
- Minimum 80% code coverage
