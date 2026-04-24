# Functional Requirements — PhenoEvents

Traces to: PRD.md epics E1–E6.
ID format: FR-PHENOEVENTS-{NNN}.

---

## Event Bus & Broker

**FR-PHENOEVENTS-001**: The system SHALL provide an event bus for publishing and subscribing to domain events with topic-based routing.
Traces to: E1.1

**FR-PHENOEVENTS-002**: The system SHALL support multiple backend brokers (in-memory queue, Redis, NATS, RabbitMQ) via pluggable adapters.
Traces to: E1.2

**FR-PHENOEVENTS-003**: The system SHALL guarantee at-least-once delivery with optional exactly-once semantics via idempotency keys.
Traces to: E1.3

---

## Event Schema & Serialization

**FR-PHENOEVENTS-004**: The system SHALL define event types with JSON schema, version numbers, and backward-compatible serialization.
Traces to: E2.1

**FR-PHENOEVENTS-005**: The system SHALL validate inbound events against registered schemas before delivery to subscribers.
Traces to: E2.2

---

## Event Sourcing Integration

**FR-PHENOEVENTS-006**: The system SHALL integrate with phenotype-event-sourcing for append-only event stream storage and replay.
Traces to: E3.1

---

## Trace & Test Guidance

All tests MUST reference a Functional Requirement (FR):

```rust
// Traces to: FR-PHENOEVENTS-NNN
#[test]
fn test_event_delivery() { ... }
```
