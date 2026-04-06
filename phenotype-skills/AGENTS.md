# AGENTS.md — phenotype-skills

## Project Overview

- **Name**: phenotype-skills
- **Description**: Skills framework for agents - multi-language bindings with Rust core
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-skills`
- **Language Stack**: Rust (core), with bindings
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-skills

# Rust core
cargo build
cargo test

# Benchmarks
cargo bench

# Documentation
cargo doc --open
```

## Architecture

```
phenotype-skills/
├── benches/                  # Benchmarks
├── bindings/                 # Language bindings
├── Cargo.lock               # Dependencies
├── docs/                    # Documentation
├── src/                     # Rust source
├── target/                  # Build output
└── tests/                   # Integration tests
```

## Quality Standards

### Rust
- **Line length**: 100 characters
- **Formatter**: `cargo fmt`
- **Linter**: `cargo clippy -- -D warnings`
- **Tests**: `cargo test`
- **Benchmarks**: `cargo bench`

## Git Workflow

### Branch Naming
Format: `skills/<type>/<description>`

Examples:
- `skills/feat/skill-registry`
- `skills/fix/binding-memory`

### Commit Format
```
<type>(skills): <description>

Examples:
- feat(skills): add skill discovery
- fix(bindings): resolve FFI issue
```

## File Structure

```
phenotype-skills/
├── src/
│   ├── lib.rs               # Library entry
│   └── [modules]
├── bindings/                # Language bindings
├── tests/                   # Tests
├── benches/                 # Benchmarks
└── docs/                    # Documentation
```

## CLI Commands

```bash
# Build
cargo build

# Test
cargo test

# Benchmark
cargo bench

# Lint
cargo clippy -- -D warnings
cargo fmt
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Build fails | Check Cargo.toml dependencies |
| Test failures | Verify skill fixtures |
| Benchmark errors | Check criterion setup |

## Dependencies

- **phenotype-skills/ in .worktrees/**: Related worktrees
- **AgilePlus**: Work tracking
- **MCP spec**: For skill interface alignment

## Agent Notes

When working in phenotype-skills:
1. Rust core with language bindings
2. Benchmarks for performance-critical paths
3. Skills are reusable agent capabilities
4. Coordinate with MCP integration
