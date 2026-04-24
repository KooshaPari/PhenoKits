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

## Cross-Collection Integration

Stashly is part of the **Phenotype named collections**:

- **Sidekick** — Agent dispatch & presence
- **Eidolon** — Device automation
- **Observably** — Distributed tracing & observability
- **Stashly** (this) — State, events, caching, migrations
- **Paginary** — Knowledge collection (specs, tutorials, handbooks)

### Event Bus

Stashly uses **phenotype-bus** to subscribe to events from other collections and store them in the event store:

```rust
use phenotype_bus::{Bus, Event};
use stashly_eventstore::EventStore;

// Subscribe to Sidekick dispatch events
let mut rx = dispatch_bus.subscribe();

let event_store = EventStore::new();

while let Ok(event) = rx.recv().await {
    // Append to event store for replaying / auditing
    event_store.append(
        &event.event_name(),
        serde_json::to_value(event)?,
    ).await?;
}
```

Stashly's state machines can also emit events for other collections:

```rust
pub struct StateTransitioned {
    pub from_state: String,
    pub to_state: String,
}

impl Event for StateTransitioned {
    fn event_name(&self) -> &'static str { "StateTransitioned" }
}

// Emit for Observably to trace
state_transition_bus.publish(StateTransitioned { /* ... */ }).await?;
```

See `../../phenotype-bus/README.md` and `../../docs/org-audit-2026-04/collection_build_matrix.md` for integration details.

## Provenance

- **stashly-cache**: Extracted from `crates/phenotype-cache-adapter`
- **stashly-eventstore**: Extracted from `crates/phenotype-event-sourcing`
- **stashly-statemachine**: Extracted from `crates/phenotype-state-machine`
- Source repos retained; these are copies for productized distribution.

## License

Apache-2.0
