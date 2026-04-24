# Stashly Getting Started

## Why Stashly?

Stashly provides battle-tested storage and state primitives: two-tier LRU caching with TTL, append-only event sourcing (with cryptographic hash chains), and generic finite state machines with transition guards. Whether you're caching screenshots from Eidolon automation, storing dispatch results from Sidekick, or building event-sourced workflows, Stashly ensures correctness, auditability, and performance.

**Key problems Stashly solves:**

- **Smart caching** — Two-tier (in-memory + persistent) with LRU eviction and TTL support
- **Event sourcing** — Immutable append-only log with SHA-256 hash chains for integrity
- **State machines** — Generic FSM with guarded transitions and context management
- **Cross-collection persistence** — Subscribe to events and store them durably for replay/audit

## Install

Add the crates you need:

```bash
cargo add stashly-cache
cargo add stashly-eventstore
cargo add stashly-statemachine
```

Or in your `Cargo.toml`:

```toml
[dependencies]
stashly-cache = { path = "../../stashly/crates/stashly-cache" }
stashly-eventstore = { path = "../../stashly/crates/stashly-eventstore" }
stashly-statemachine = { path = "../../stashly/crates/stashly-statemachine" }

tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
phenotype-bus = { path = "../../phenotype-bus" }
```

## Quickstart (20 lines)

```rust
use stashly_cache::TwoTierCache;
use stashly_eventstore::EventStore;
use stashly_statemachine::StateMachine;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize 2-tier cache (in-mem LRU + persistent backend)
    let cache = TwoTierCache::new(100, 1000)?; // 100 in-mem, 1000 total
    cache.insert("screenshot-1", b"image data", 3600).await?; // TTL 1h

    // Append-only event store with hash chains
    let store = EventStore::new("./events.log")?;
    store.append("UserLogin", serde_json::json!({"user": "alice"}))?;

    // Finite state machine with guards
    let mut fsm = StateMachine::new("idle");
    fsm.transition("run")?;
    println!("State: {}", fsm.current_state());

    Ok(())
}
```

## Common Patterns

### Pattern 1: Caching with Automatic Expiration

Cache screenshots, dispatch results, and other short-lived data with TTL.

```rust
use stashly_cache::{TwoTierCache, CachePolicy};
use std::time::Duration;

let cache = TwoTierCache::new(100, 1000)?; // 100 in-mem, 1000 on disk

// Cache a screenshot with 1-hour TTL
cache.insert(
    "automation-screenshot-001",
    b"PNG image data...",
    3600, // seconds
).await?;

// Retrieve (hits in-memory if not expired)
match cache.get("automation-screenshot-001").await? {
    Some(data) => println!("Cache hit: {} bytes", data.len()),
    None => println!("Cache miss or expired"),
}

// Manual eviction (LRU)
cache.evict_if_needed().await?;

// Check cache stats
let stats = cache.stats().await?;
println!("Hits: {}, Misses: {}", stats.hits, stats.misses);
```

### Pattern 2: Event Sourcing with Hash Chains

Store immutable event log for audit trail and replay.

```rust
use stashly_eventstore::{EventStore, Event};
use serde_json::json;

let store = EventStore::new("./events.log")?;

// Append events (each linked to previous hash)
store.append("UserLogin", json!({"user_id": "alice", "timestamp": 1234567890}))?;
store.append("ScreenshotTaken", json!({"path": "before.png", "size": 12345}))?;
store.append("CacheStored", json!({"key": "screenshot-1", "ttl": 3600}))?;

// Retrieve event history (hash chain verified)
let events = store.read_all()?;
for event in events {
    println!("{}: {:?} (hash: {})", event.event_type, event.data, event.hash);
}

// Verify integrity (detect tampering)
let is_valid = store.verify_chain()?;
println!("Event chain valid: {}", is_valid);

// Replay: rebuild state from events
let mut state = "initial";
for event in store.read_all()? {
    match event.event_type.as_str() {
        "UserLogin" => state = "logged_in",
        "ScreenshotTaken" => state = "screenshot_cached",
        _ => {}
    }
}
```

### Pattern 3: State Machines with Transition Guards

Define complex workflows with guarded state transitions.

```rust
use stashly_statemachine::{StateMachine, TransitionGuard, Context};

// Define workflow states and transitions
let mut fsm = StateMachine::new("idle");

// Add transition with guard
fsm.add_transition("idle", "running", |ctx: &Context| {
    // Only allow transition if resources are available
    ctx.get("ready").map(|v| v == "true").unwrap_or(false)
})?;

// Set context
let mut ctx = Context::new();
ctx.set("ready", "true");
fsm.set_context(ctx);

// Attempt transition
match fsm.transition("running") {
    Ok(_) => println!("Transition succeeded"),
    Err(e) => println!("Guard rejected: {}", e),
}

// Check current state
println!("Current: {}", fsm.current_state());

// Publish state change event
let state_event = json!({"from": "idle", "to": "running", "timestamp": now()});
```

### Pattern 4: Cross-Collection Persistence via phenotype-bus

Subscribe to events from Sidekick/Eidolon/Observably and store them durably.

```rust
use phenotype_bus::{Bus, Event};
use stashly_eventstore::EventStore;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DispatchStarted {
    pub dispatch_id: String,
    pub agent: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AutomationCompleted {
    pub dispatch_id: String,
    pub screenshots: usize,
}

impl Event for DispatchStarted {
    fn event_name(&self) -> &'static str { "DispatchStarted" }
}

impl Event for AutomationCompleted {
    fn event_name(&self) -> &'static str { "AutomationCompleted" }
}

// Subscribe to cross-collection events
let dispatch_bus = Bus::<DispatchStarted>::new(100);
let mut rx = dispatch_bus.subscribe();

let store = EventStore::new("./dispatch_log.log")?;

while let Ok(event) = rx.recv().await {
    // Store in immutable event log
    store.append(
        "DispatchStarted",
        serde_json::to_value(&event)?,
    )?;

    println!("Stored dispatch event: {}", event.dispatch_id);
}
```

## Cross-Collection Integration

Stashly is the persistence layer for Phenotype via **phenotype-bus**:

- **Subscribes to**: Events from Sidekick (dispatch), Eidolon (automation), Observably (traces)
- **Emits**: `EventStored`, `CacheHit`, `StateMachineTransitioned` events
- **Consumed by**: Observably (traces storage operations), Paginary (documents event schemas)

See [phenotype-bus](../../phenotype-bus/README.md) for event patterns. Stashly works with [Sidekick](../Sidekick/README.md) (caches dispatch results), [Eidolon](../Eidolon/README.md) (stores automation events), [Observably](../Observably/README.md) (traces persistence), and [Paginary](../Paginary/README.md) (documents state machine workflows).

## Next Steps

- Explore [stashly-cache eviction policies](./crates/stashly-cache/README.md)
- Read [event sourcing patterns](./docs/EVENT_SOURCING.md)
- Review [FSM guard patterns](./docs/STATEMACHINE_PATTERNS.md)
- Check the [cross-collection demo](../../docs/collections/cross_collection_demo.md)
