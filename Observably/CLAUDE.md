# Observably — CLAUDE.md

Observably is a productized Phenotype collection of observability crates. This file extends parent governance; see `/Users/kooshapari/.claude/CLAUDE.md` for global rules.

## Project Overview

- **Name**: Observably
- **Description**: Independent observability crates (tracing, logging, metrics, resilience)
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/Observably`
- **Language Stack**: Rust (edition 2021)
- **Type**: Named collection (Stashly-style)

## AgilePlus Mandate

All work MUST be tracked in AgilePlus:
- Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`

## Stack

- **Language**: Rust (edition 2021)
- **Build**: cargo (workspace)
- **Test**: `cargo test --workspace`
- **Lint**: `cargo clippy --workspace -- -D warnings`
- **Format**: `cargo fmt`

## Quality Checks

From this repository root:

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

## Provenance

This collection bootstraps from source repositories:
- `observably-tracing`: `FocalPoint/crates/focus-observability`

Source repos are retained; these are copies for productized distribution.
