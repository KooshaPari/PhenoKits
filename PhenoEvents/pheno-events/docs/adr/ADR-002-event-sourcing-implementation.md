# ADR-002: Event Sourcing Implementation - File-Based with Optional Persistence

## Status

Accepted

## Context

Event sourcing is a pattern where state changes are stored as a sequence of events rather than just the current state. This provides:

- Complete audit trail
- Ability to replay events to rebuild state
- Temporal queries ("What was the state at time T?")
- Natural integration with event-driven architectures

pheno-events needs to support event sourcing while maintaining its philosophy of simplicity and zero-dependency core functionality.

## Decision

We will implement event sourcing with:

1. **In-memory default**: Fast, zero-dependency storage for development/testing
2. **File-based persistence**: JSONL (JSON Lines) files for durability
3. **Stream-per-aggregate**: Each aggregate has its own event stream
4. **Optimistic concurrency**: Version-based conflict detection
5. **Async API**: All operations are async for consistency with `EventBus`

### Storage Format

**JSONL (JSON Lines)** - One event per line:

```jsonl
{"event_id": "uuid-1", "event_type": "OrderPlaced", "aggregate_id": "order-123", "aggregate_type": "Order", "data": {"customer_id": "cust-456"}, "metadata": {}, "timestamp": "2024-01-01T00:00:00", "version": 1}
{"event_id": "uuid-2", "event_type": "ItemAdded", "aggregate_id": "order-123", "aggregate_type": "Order", "data": {"sku": "ABC", "qty": 2}, "metadata": {}, "timestamp": "2024-01-01T00:00:01", "version": 2}
```

**File naming**: `{aggregate_id}.jsonl`

**Storage layout**:
```
storage_path/
├── order-123.jsonl
├── order-124.jsonl
├── user-456.jsonl
└── ...
```

### Data Model

```python
@dataclass
class StoredEvent:
    event_id: str          # UUID v4
    event_type: str      # Domain event type
    aggregate_id: str    # Aggregate identifier
    aggregate_type: str  # Aggregate type (Order, User, etc.)
    data: dict[str, Any] # Event payload
    metadata: dict      # System metadata
    timestamp: str       # ISO 8601 UTC
    version: int         # Sequence number within stream
```

### API Design

```python
# Initialize
store = EventStore(storage_path=Path("./events"))

# Append event
event = await store.append(
    event_type="OrderPlaced",
    aggregate_id="order-123",
    aggregate_type="Order",
    data={"customer_id": "cust-456", "amount": 100.00},
    metadata={"source": "web-api", "ip": "1.2.3.4"}
)

# Get all events for an aggregate
events = await store.get_stream("order-123")

# Replay to rebuild state
async def apply_order_event(order, event):
    if event.event_type == "OrderPlaced":
        return Order(id=event.aggregate_id, **event.data)
    elif event.event_type == "ItemAdded":
        order.items.append(event.data)
    return order

order = await store.replay("order-123", apply_order_event)

# Query events
all_orders = await store.get_events(aggregate_type="Order")
recent = await store.get_events(from_version=100)
```

### Concurrency Model

**Optimistic concurrency control** using version numbers:

```
Read aggregate events -> Get current version -> Append with expected version
                                    │
                                    ▼
                            If version mismatch:
                            raise ConcurrencyError
```

**Note**: Current implementation does not enforce optimistic concurrency at the storage layer but provides version metadata for application-level enforcement.

### Loading from Disk

```python
def _load_from_disk(self):
    """Load all events from JSONL files."""
    for file in self.storage_path.glob("*.jsonl"):
        with file.open("r") as f:
            for line in f:
                if line.strip():
                    event_data = json.loads(line)
                    self._events.append(StoredEvent.from_dict(event_data))
```

**Performance characteristics**:
- Load: O(N) where N = total events
- Append: O(1) - append to file
- Get stream: O(N) where N = events in stream (no index yet)
- Memory: O(N) for in-memory cache

## Consequences

### Positive

1. **Zero dependencies**: No database required
2. **Human-readable**: JSONL files can be inspected with standard tools
3. **Append-only**: Immutable history, easy to backup
4. **Version control friendly**: Small streams can be committed
5. **Migration path**: Can upgrade to EventStoreDB/DB later

### Negative

1. **No indexing**: O(N) lookups for aggregate streams
2. **No transactions**: File append is not atomic across streams
3. **Disk usage**: JSONL is verbose compared to binary formats
4. **No compaction**: Event streams grow indefinitely
5. **Single writer**: No coordination for concurrent writes

### Mitigations

| Issue | Mitigation | Future Work |
|-------|-----------|-------------|
| No indexing | Acceptable for small streams (<10K events) | Add SQLite index |
| No transactions | Document limitation | Add WAL mode |
| Verbose format | gzip compression option | Binary format (msgpack) |
| No compaction | Archive old events | Snapshot support |
| Single writer | Document single-process | Add file locking |

## Alternatives Considered

### Alternative 1: SQLite Backend

```python
store = EventStore(backend="sqlite", path="events.db")
```

**Rejected**: Adds sqlite3 dependency (though in stdlib) and complexity. May be added later as optional backend.

### Alternative 2: PostgreSQL JSONB

```python
store = EventStore(backend="postgresql", dsn="postgres://...")
```

**Rejected**: External database requirement conflicts with zero-dependency philosophy. Suitable for `pheno-events[postgres]` extra.

### Alternative 3: EventStoreDB Integration Only

```python
store = EventStore(backend="eventstoredb", connection=...)
```

**Rejected**: External service requirement. EventStoreDB integration will be supported but not required.

### Alternative 4: Pure In-Memory

No persistence option.

**Rejected**: Loses audit trail on restart. File persistence is minimal overhead.

## Implementation Details

### Initialization

```python
def __init__(self, storage_path: Path | None = None):
    self.storage_path = Path(storage_path) if storage_path else None
    self._events: list[StoredEvent] = []

    if self.storage_path:
        self.storage_path.mkdir(parents=True, exist_ok=True)
        self._load_from_disk()
```

### Append Operation

```python
async def append(
    self,
    event_type: str,
    aggregate_id: str,
    aggregate_type: str,
    data: dict[str, Any],
    metadata: dict[str, Any] | None = None,
    version: int = 1,
) -> StoredEvent:
    event = StoredEvent(
        event_type=event_type,
        aggregate_id=aggregate_id,
        aggregate_type=aggregate_type,
        data=data,
        metadata=metadata or {},
        version=version,
    )

    self._events.append(event)

    if self.storage_path:
        stream_file = self._get_stream_file(aggregate_id)
        with stream_file.open("a") as f:
            f.write(json.dumps(event.to_dict()) + "\n")

    return event
```

### Replay Operation

```python
async def replay(self, aggregate_id: str, reducer: Any) -> Any:
    """Replay events to rebuild state."""
    events = await self.get_stream(aggregate_id)
    state = None

    for event in events:
        state = reducer(state, event)

    return state
```

## Future Enhancements

1. **Snapshot support**: Periodic state snapshots for fast replay
2. **SQLite backend**: Optional indexed storage
3. **PostgreSQL backend**: For production deployments
4. **EventStoreDB adapter**: Native EventStoreDB integration
5. **Stream compaction**: Remove redundant events (for non-audit streams)
6. **Schema evolution**: Event versioning and migration
7. **Subscriptions**: Real-time notifications for new events

## Migration Path

```python
# Phase 1: File-based (current)
store = EventStore(storage_path="./events")

# Phase 2: SQLite for larger datasets
store = EventStore(backend="sqlite", path="events.db")

# Phase 3: PostgreSQL for production
store = EventStore(backend="postgresql", dsn="...")

# Phase 4: EventStoreDB for CQRS
store = EventStore(backend="eventstoredb", endpoint="...")
```

## References

- [Event Bus SOTA Research](./../research/EVENT_BUS_SOTA.md) - Event Sourcing section
- [EventStoreDB Documentation](https://developers.eventstore.com/)
- [JSON Lines Specification](https://jsonlines.org/)
- Martin Fowler: [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)

---

*ADR-002 | pheno-events Event Sourcing | Accepted*
