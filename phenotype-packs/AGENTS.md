# AGENTS.md — phenotype-packs

## Project Overview

- **Name**: phenotype-packs
- **Description**: Multi-language phenotype packs - Go, Python, Rust, TypeScript implementations
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-packs`
- **Language Stack**: Go, Python, Rust, TypeScript
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-packs

# Go packs
cd go && go build ./...

# Python packs
cd python && pip install -e .

# Rust packs
cd rust && cargo build

# TypeScript packs
cd typescript && npm install && npm run build
```

## Architecture

```
phenotype-packs/
├── docs/                     # Documentation
├── go/                       # Go implementations
├── python/                   # Python implementations
├── rust/                     # Rust implementations
└── typescript/               # TypeScript implementations
```

## Quality Standards

### Go
- **Formatter**: `gofmt`
- **Linter**: `golangci-lint`
- **Tests**: `go test ./...`

### Python
- **Formatter**: `ruff format`
- **Linter**: `ruff check`
- **Tests**: `pytest`

### Rust
- **Formatter**: `cargo fmt`
- **Linter**: `cargo clippy -- -D warnings`
- **Tests**: `cargo test`

### TypeScript
- **Formatter**: `prettier`
- **Type checker**: `tsc --noEmit`

## Git Workflow

### Branch Naming
Format: `packs/<type>/<description>` or `<language>/<type>/<description>`

Examples:
- `packs/feat/new-pack-type`
- `go/feat/http-pack`
- `python/fix/pack-loading`

### Commit Format
```
<type>(<scope>): <description>

Scope: go, python, rust, ts

Examples:
- feat(go): add HTTP client pack
- fix(python): resolve pack discovery
```

## File Structure

```
phenotype-packs/
├── docs/
├── go/
│   └── [Go pack implementations]
├── python/
│   └── [Python pack implementations]
├── rust/
│   └── [Rust pack implementations]
└── typescript/
    └── [TS pack implementations]
```

## CLI Commands

```bash
# Go
cd go && go build ./... && go test ./...

# Python
cd python && pip install -e ".[dev]" && pytest

# Rust
cd rust && cargo build && cargo test

# TypeScript
cd typescript && npm install && npm run build
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Pack not loading | Check pack manifest |
| Cross-language mismatch | Verify pack spec compliance |

## Dependencies

- **phenotype-packs/**: Multi-language pack framework
- **HexaKit**: Kit patterns

## Agent Notes

When working in phenotype-packs:
1. Multi-language pack implementations
2. Each language has its own subdirectory
3. Packs should be semantically equivalent across languages
4. Coordinate pack format with phenotype-registry
