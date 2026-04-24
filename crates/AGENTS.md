# AGENTS.md — crates

## Project Overview

- **Name**: crates
- **Description**: Rust workspace for phenotype shared infrastructure crates
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/crates`
- **Language Stack**: Rust (edition 2021)
- **Published**: Internal (Phenotype org)

## Quick Start Commands

```bash
# Navigate to workspace
cd /Users/kooshapari/CodeProjects/Phenotype/repos/crates

# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Check with clippy
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt
```

## Architecture

```
crates/
├── ffi_utils/                    # FFI utilities
├── phenotype-application/        # Application framework
├── phenotype-async-traits/       # Async trait definitions
├── phenotype-bdd/                # BDD testing framework
├── phenotype-bid/                # BID (Business Identifier)
├── phenotype-cache-adapter/      # Cache implementations
├── phenotype-compliance-scanner/ # Compliance scanning
├── phenotype-config/             # Configuration management
│   └── AGENTS.md                # Has own agent rules
├── phenotype-content-hash/       # Content hashing
├── phenotype-contracts/          # Contract definitions
├── phenotype-core/               # Core abstractions
├── phenotype-domain/             # Domain models
├── phenotype-error-core/         # Error handling core
├── phenotype-error-macros/       # Error macros
├── phenotype-errors/             # Error types
├── phenotype-event-bus/          # Event bus
├── phenotype-event-sourcing/     # Event sourcing
├── phenotype-health/             # Health checks
├── phenotype-health-axum/        # Axum health integration
├── phenotype-health-cli/         # Health CLI
├── phenotype-http-adapter/       # HTTP adapter
├── phenotype-http-client-core/   # HTTP client
├── phenotype-in-memory-store/    # In-memory storage
├── phenotype-infrastructure/     # Infrastructure
├── phenotype-iter/               # Iterator utilities
├── phenotype-logging/            # Logging framework
├── phenotype-macros/             # Procedural macros
├── phenotype-mcp-asset/          # MCP asset management
├── phenotype-mcp-core/           # MCP core
├── phenotype-mcp-framework/      # MCP framework
├── phenotype-mcp-testing/        # MCP testing
├── phenotype-metrics/            # Metrics collection
├── phenotype-mock/               # Mocking utilities
├── phenotype-policy-engine/      # Policy engine
├── phenotype-port-interfaces/    # Port interfaces
├── phenotype-port-traits/        # Port traits
├── phenotype-postgres-adapter/   # PostgreSQL adapter
├── phenotype-project-registry/   # Project registry
└── phenotype-redis-adapter/      # Redis adapter
```

## Quality Standards

### Code Quality Mandate
- **All linters must pass**: `cargo clippy --workspace -- -D warnings`
- **All tests must pass**: `cargo test --workspace`
- **Formatter**: `cargo fmt` (mandatory)

### Style Constraints
- **Line length**: 100 characters
- **File size target**: ≤350 lines per source file, hard limit ≤500 lines
- **Typing**: Full type annotations required

### Test-First Mandate
- **For NEW modules**: test file MUST exist before implementation file
- **For BUG FIXES**: failing test MUST be written before the fix
- **For REFACTORS**: existing tests must pass before AND after

### Crate Standards
- All public types must implement `Debug` and `Clone` where practical
- Error types must use `thiserror` with proper `#[from]` conversions
- No inter-crate dependencies unless explicitly designed

## Git Workflow

### Branch Naming
Format: `crates/<type>/<description>` or `<crate-name>/<type>/<description>`

Examples:
- `crates/feat/new-crate`
- `phenotype-config/fix/parsing`

### Commit Format
```
<type>(<crate>): <description>

Types: feat, fix, chore, docs, refactor, test
```

Examples:
- `feat(phenotype-config): add TOML provider`
- `fix(phenotype-cache): TTL expiration bug`

## File Structure

Each crate follows standard Rust layout:
```
phenotype-<name>/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── ...
└── [AGENTS.md for complex crates]
```

## CLI Commands

```bash
# Workspace-level operations
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt

# Single crate operations
cargo build -p phenotype-config
cargo test -p phenotype-domain
cargo doc --package phenotype-core --open

# Check specific crate
cd phenotype-<name> && cargo check
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Workspace build fails | Check `Cargo.toml` workspace definition |
| Missing crate | Verify crate is listed in workspace members |
| Dependency conflict | Check workspace-level dependency management |
| Crate has own AGENTS.md | Follow those rules for crate-specific work |

## Dependencies

- **Workspace**: Defined in root `Cargo.toml`
- **AgilePlus**: Work tracking in `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- **thegent**: Governance base rules

## Agent Notes

When working in crates:
1. Check if specific crate has its own `AGENTS.md`
2. Follow workspace rules for cross-cutting changes
3. Keep crates independently consumable (no tight coupling)
4. Some crates may have special rules (phenotype-config has its own AGENTS.md)
