# AGENTS.md — phenotype-patch

## Project Identity

- **Name**: phenotype-patch
- **Description**: Rust library for parsing, creating, and applying unified diffs and patches
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-patch`
- **Language Stack**: Rust (edition 2021)
- **Type**: Library/Utility

## Agent Responsibilities

### Forge (Implementation)
- Implement diff parsing per unified diff specification
- Add three-way merge capabilities
- Maintain zero-copy parsing where possible
- Write comprehensive unit tests

### Helios (Testing)
- Run `cargo test` before any PR
- Verify edge cases (empty patches, conflict markers)
- Bench performance-critical parsing paths

## Development Commands

```bash
cargo check    # Type check
cargo test     # Run tests
cargo clippy   # Lint
cargo fmt      # Format code
```

## Quality Standards

- **Clippy warnings**: Zero tolerance (`-D warnings`)
- **No unsafe code** unless documented
- **Error handling**: Typed errors, no panics
- **FR traceability**: All tests MUST reference FR identifiers

## Branch Discipline

- Feature branches: `feat/<feature-name>`
- Bug fixes: `fix/<issue-name>`
- Worktrees preferred for parallel work

## CI/CD

- GitHub Actions workflow in `.github/workflows/`
- Must pass before merge to main
