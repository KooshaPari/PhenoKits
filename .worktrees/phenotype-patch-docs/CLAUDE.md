# phenotype-patch

Rust library for parsing, creating, and applying unified diffs and patches. Supports three-way merge with conflict markers.

## Stack
- Language: Rust
- Key deps: Cargo, nom (or similar parser combinator)

## Structure
- `src/`: Rust library
  - `parse.rs`: Unified/context/side-by-side diff parsing
  - `create.rs`: Diff generation from text or structured data
  - `apply.rs`: Patch application with conflict detection
  - `merge.rs`: Three-way merge

## Key Patterns
- Zero-copy parsing where possible
- Conflict detection returns typed errors (not panics)
- Composable: parse -> transform -> apply pipeline

## Adding New Functionality
- New diff format: add parser in `src/parse.rs`
- New merge strategy: add in `src/merge.rs`
- Run `cargo test` to verify
