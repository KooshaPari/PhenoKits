# stashly-eventstore

Append-only event store with SHA-256 hash chain integrity.

**Features:**
- Append-only event log
- SHA-256 hash chain for tamper detection
- In-memory and file-based backends
- Snapshot support for fast recovery
- Event serialization (serde JSON)

**Tests:** Inline `#[test]` in `src/event.rs`, `src/memory.rs`, etc.

**Usage:**
```rust
let store = MemoryEventStore::new();
let event = Event { id: "1", data: "test" };
store.append(event).await.unwrap();
```
