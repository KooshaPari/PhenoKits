# AGENTS.md — Settly

Extends shelf-level AGENTS.md rules for Settly.

## Project Identity

- **Name**: Settly
- **Description**: Universal configuration management with layered configs, validation, and environment support
- **Language**: Rust

## Project-Specific Rules

### Test-First Mandate

- **For NEW modules**: test file MUST exist before implementation file
- **For BUG FIXES**: failing test MUST be written before the fix
- **For REFACTORS**: existing tests must pass before AND after

### Quality Gates

All PRs must pass:
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `cargo fmt --check`

### Commit Messages

Format: `<type>(<scope>): <description>`

Types: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`, `ci`

### File Organization

```
src/
├── lib.rs           # Main library entry
├── loader/          # Config file loading
├── validator/       # Config validation
├── layer/           # Config layering
└── format/          # Format parsers (TOML, YAML, JSON)
```

## Testing Requirements

- Unit tests for all public APIs
- Property-based tests for config merging
- Integration tests for format parsers
- Minimum 80% code coverage
