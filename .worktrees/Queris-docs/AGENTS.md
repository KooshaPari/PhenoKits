# Queris — AGENTS.md

## Project Overview

Database abstraction with ORM and query builder.

## Agent Rules

1. **Read CLAUDE.md first** before making changes
2. **Test first** - write tests before implementation
3. **Clippy clean** - all lints must pass before PR

## Quality Gates

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Architecture

- Follow repository pattern
- Support multiple database backends
- Type-safe query builder

## See Also

- **CLAUDE.md**: `./CLAUDE.md`
- **PRD.md**: `./PRD.md`
