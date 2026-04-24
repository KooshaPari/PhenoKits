# Stashly — CLAUDE.md

Stashly is a productized Phenotype collection of storage & persistence crates. This file extends parent governance; see `/Users/kooshapari/.claude/CLAUDE.md` for global rules.

## Project Overview

- **Name**: Stashly
- **Description**: Independent storage crates (caching, event sourcing, state machines)
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/Stashly`
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
- `stashly-cache`: `crates/phenotype-cache-adapter`
- `stashly-eventstore`: `crates/phenotype-event-sourcing`
- `stashly-statemachine`: `crates/phenotype-state-machine`

Source repos are retained; these are copies for productized distribution.
