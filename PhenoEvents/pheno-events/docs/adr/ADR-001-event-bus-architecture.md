# ADR-001: Event Bus Architecture - In-Memory with Pluggable Backends

## Status

Accepted

## Context

pheno-events needs to serve multiple use cases within the phenotype ecosystem:

1. **Development/Testing**: Simple, zero-dependency event coordination
2. **Single-Process Apps**: In-memory pub/sub for modular architecture
3. **Distributed Systems**: Integration with external message brokers
4. **Webhook Delivery**: Reliable outbound HTTP notifications
5. **Event Sourcing**: Persistent event streams for audit and replay

The challenge is designing a system that scales from simple to complex without forcing complexity on simple use cases.

## Decision

We will implement a **layered architecture** with:

1. **Core Layer**: Pure in-memory event bus (`EventBus`, `SimpleEventBus`)
2. **Protocol Layer**: Optional integrations (NATS, Redis, etc.)
3. **Application Layer**: Webhooks, event sourcing, projections

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  Webhook    │  │   Event     │  │   Projections       │ │
│  │  Manager    │  │   Store     │  │   (CQRS)            │ │
│  └──────┬──────┘  └──────┬──────┘  └─────────────────────┘ │
└─────────┼────────────────┼──────────────────────────────────┘
          │                │
          ▼                ▼
┌─────────────────────────────────────────────────────────────┐
│                    Protocol Layer (Optional)                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  NATS       │  │   Redis     │  │   Custom            │ │
│  │  JetStream  │  │   Pub/Sub   │  │   Adapters          │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Core Layer                              │
│  ┌─────────────────┐  ┌─────────────────────────────────┐   │
│  │   EventBus      │  │      SimpleEventBus             │   │
│  │  (async)        │  │      (sync)                     │   │
│  │                 │  │                                 │   │
│  │  • Wildcards    │  │  • Basic pub/sub                │   │
│  │  • Async        │  │  • Sync handlers                │   │
│  │  • Error        │  │  • Minimal overhead             │   │
│  │    handling     │  │                                 │   │
│  └─────────────────┘  └─────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Core Design Principles

1. **Zero-dependency core**: Basic functionality works without external services
2. **Optional extras**: Advanced features via extras: `pip install pheno-events[nats]`
3. **Async-first**: Core `EventBus` uses asyncio; `SimpleEventBus` for sync code
4. **Wildcard support**: Hierarchical event names (`user.*`, `*.created`)
5. **Error isolation**: One handler failure doesn't affect others

### Event Structure

```python
@dataclass
class Event:
    name: str                 # Event type (e.g., "user.created")
    data: Any                 # Payload
    timestamp: datetime       # UTC
    source: str | None        # Originating service/component
    correlation_id: str     # For distributed tracing
```

### API Design

```python
# Core usage
bus = EventBus()

@bus.on("user.created")
async def send_welcome_email(event):
    await email_service.send(event.data["email"])

# Publish
await bus.publish("user.created", {"email": "user@example.com"})

# Wildcard subscription
@bus.on("user.*")
async def log_user_event(event):
    logger.info(f"User event: {event.name}")

# Unsubscribe
bus.unsubscribe("user.created", send_welcome_email)
```

## Consequences

### Positive

1. **Simple getting started**: `pip install pheno-events` and use immediately
2. **Gradual complexity**: Add features as needed
3. **Test-friendly**: In-memory bus enables fast, deterministic tests
4. **Framework-agnostic**: Works with FastAPI, Django, Flask, etc.
5. **Type-safe**: Full mypy support with `py.typed`

### Negative

1. **Two bus classes**: Users must choose between `EventBus` and `SimpleEventBus`
2. **Memory limits**: In-memory storage bounded by RAM
3. **No persistence by default**: Events lost on process restart
4. **Handler errors logged not propagated**: Silent failures possible

### Mitigations

1. Clear documentation on when to use each bus type
2. `EventStore` for persistent event sourcing
3. Integration with external brokers for production durability
4. Configurable error handling (callback vs exception)

## Alternatives Considered

### Alternative 1: Single Unified Bus

```python
class UnifiedBus:
    def __init__(self, backend: str = "memory"):
        self.backend = backend
```

**Rejected**: Forces all users to understand backend abstraction even for simple use cases.

### Alternative 2: Pure External Broker

Require NATS/Redis for all functionality.

**Rejected**: Adds operational burden for development and simple deployments.

### Alternative 3: Only Async

No synchronous `SimpleEventBus`.

**Rejected**: Many Python applications still use synchronous patterns. Forcing async would exclude significant use cases.

## Related Decisions

- ADR-002: Event Sourcing Implementation (append-only streams)
- ADR-003: Webhook Delivery Strategy (retry + circuit breaker)

## References

- [Event Bus SOTA Research](./../research/EVENT_BUS_SOTA.md)
- [NATS Documentation](https://docs.nats.io/)
- [Python asyncio best practices](https://docs.python.org/3/library/asyncio.html)

---

*ADR-001 | pheno-events Architecture | Accepted*
