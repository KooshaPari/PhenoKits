# Stashly — Phenotype Storage & Persistence Collection

Stashly is a curated collection of independent, battle-tested storage crates for Rust. Caching, event sourcing, state machines, and persistence patterns as standalone cargo-installable packages.

## Crates

| Crate | Purpose | Status |
|-------|---------|--------|
| `stashly-cache` | Two-tier LRU + DashMap cache with TTL support | Extracted from phenotype-cache-adapter |
| `stashly-eventstore` | Append-only event store with SHA-256 hash chains | Extracted from phenotype-event-sourcing |
| `stashly-statemachine` | Generic FSM with transition guards and context | Extracted from phenotype-state-machine |

## Quick Start

```toml
[dependencies]
stashly-cache = { path = "crates/stashly-cache" }
stashly-eventstore = { path = "crates/stashly-eventstore" }
stashly-statemachine = { path = "crates/stashly-statemachine" }
```

Each crate is independently importable and has no inter-crate dependencies.

## Workspace

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Provenance

- **stashly-cache**: Extracted from `crates/phenotype-cache-adapter`
- **stashly-eventstore**: Extracted from `crates/phenotype-event-sourcing`
- **stashly-statemachine**: Extracted from `crates/phenotype-state-machine`
- Source repos retained; these are copies for productized distribution.

## License

Apache-2.0
