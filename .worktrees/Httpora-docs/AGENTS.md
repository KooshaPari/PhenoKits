# Httpora — AGENTS.md

## Project Overview

HTTP client/server framework with middleware support.

## Agent Rules

1. **Read CLAUDE.md first** before making changes
2. **Test first** - write tests before implementation
3. **Clippy clean** - all lints must pass before PR
4. **No unsafe code** unless absolutely necessary

## Quality Gates

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo doc
```

## Architecture

Follow tower-based middleware pattern:
- Implement `tower::Layer` for new middleware
- Implement `tower::Service` for request handlers
- Async-first design with tokio

## See Also

- **CLAUDE.md**: `./CLAUDE.md`
- **PRD.md**: `./PRD.md`
