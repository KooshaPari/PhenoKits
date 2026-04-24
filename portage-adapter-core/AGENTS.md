# AGENTS.md — portage-adapter-core

## Agent Onboarding

This repository contains Rust code for: Core adapter library for package management

### Key Paths
- `src/` — Source code
- `crates/` — Workspace crates (if applicable)
- `tests/` — Test suites
- `Cargo.toml` / `package.json` — Dependencies

### Before Starting Work

1. Install dependencies: `cargo fetch` / `npm install` / `go mod download`
2. Verify baseline: `cargo test --workspace`
3. Check lints: `cargo clippy --workspace`

### Committing

Use conventional commits with scope:
```
feat(module): add new feature
fix(module): resolve bug
docs: update README
test: add test coverage
```

### CI/CD

Local verification required before push:
- `cargo test --workspace`
- `cargo clippy --workspace`
- `cargo build`
