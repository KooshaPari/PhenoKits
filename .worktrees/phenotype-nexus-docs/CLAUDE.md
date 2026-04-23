# CLAUDE.md - phenotype-nexus

## Project Overview

**phenotype-nexus** is an in-process service registry and discovery library for microservices,
written in Rust. It provides concurrent service registration, health tracking, and multiple
load-balancing strategies (round-robin, random, consistent-hash) using lock-free `dashmap`
data structures.

> **Archived:** This repo has been migrated to `libs/nexus` in the main Phenotype workspace
> monorepo. All active development continues there. This standalone repo is preserved for
> historical reference and for consumers that pin directly to it.

## Stack

| Component | Library / Version |
|-----------|------------------|
| Language | Rust (edition 2021) |
| Async runtime | tokio 1.x |
| Concurrent map | dashmap 5.x |
| Serialization | serde 1.x + serde_json |
| Error types | thiserror 1.x |

## Key Commands

```bash
# Build
cargo build

# Run all tests
cargo test

# Type-check without linking
cargo check

# Lint (clippy)
cargo clippy -- -D warnings

# Format
cargo fmt

# Build docs
cargo doc --open
```

## Architecture

- **Single crate** — no sub-crates or workspace members.
- **Domain-first layout** — modules are organized around domain concepts:
  - `registry` — service registration and deregistration
  - `discovery` — service lookup and health filtering
  - `balancer` — load-balancing strategies (round-robin, random, consistent-hash)
  - `health` — health-state types and transitions
- **No global state** — all state is encapsulated in `ServiceRegistry`, which is `Clone + Send + Sync`.
- **Lock-free concurrency** — `dashmap::DashMap` used throughout; no `Mutex`/`RwLock` needed for the hot path.

## Phenotype Org Rules

- UTF-8 encoding only in all text files.
- Worktree discipline: canonical repo stays on `main`.
- CI completeness: fix all CI failures before merging.
- Never commit agent directories (`.claude/`, `.codex/`, `.cursor/`).
- Wrap/fork over hand-roll: prefer extending existing OSS over reimplementing.
- Fail loud — no silent/graceful degradation; required config must be required.
