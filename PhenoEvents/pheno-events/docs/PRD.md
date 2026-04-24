# Product Requirements Document — Eventra

**Status:** DRAFT  
**Owner:** Phenotype Engineering  
**Last Updated:** 2026-04-05

---

## Overview

Eventra is an event routing and stream processing system that provides a unified pipeline for event transport, transformation, filtering, and delivery across Phenotype infrastructure. It operates as a sidecar or standalone service with a focus on low-latency, high-throughput event distribution.

The system delivers:
- HTTP/TCP/Unix Socket ingestion endpoints
- Rule-based event routing with predicates
- Stateful and stateless stream processing
- Schema validation and evolution
- OpenTelemetry-native observability
- Built-in retry and dead-letter handling

---

## E1: Event Ingestion

### E1.1: HTTP Ingestion Endpoint

As a producer service, I want to submit events via HTTP so that I can integrate without custom clients.

**Acceptance Criteria:**
- `POST /ingest` accepts single event or batch
- Events are validated against schema (if provided)
- Returns 202 Accepted with event IDs
- Rate limiting returns 429 when exceeded
- Authentication via API key or JWT

### E1.2: TCP Binary Ingestion

As a high-throughput producer, I want to submit events via TCP so that I can minimize overhead.

**Acceptance Criteria:**
- Binary protocol over TCP (CBOR-encoded)
- Connection pooling for performance
- Backpressure handling when overwhelmed
- Keep-alive for connection reuse

### E1.3: Unix Socket Ingestion

As a local producer, I want to submit events via Unix socket so that I can achieve lowest latency.

**Acceptance Criteria:**
- SOCK_STREAM Unix socket
- Zero-copy transfer where possible
- File permissions control access

---

## E2: Event Routing

### E2.1: Rule-Based Routing

As an operator, I want to define routing rules so that events are delivered to the right destinations.

**Acceptance Criteria:**
- Rules evaluated in priority order
- Support for glob patterns in type matching
- Support for regex in subject matching
- Multiple actions per rule (fan-out)
- Rule changes take effect without restart

### E2.2: Predicate Filtering

As a developer, I want to filter events based on content so that only relevant events are processed.

**Acceptance Criteria:**
- JSON path expressions for field access
- Operators: ==, !=, >, <, >=, <=, contains, matches
- Predicates can be combined with AND/OR
- Invalid expressions return clear errors

### E2.3: Event Transformation

As an operator, I want to transform events before delivery so that consumers receive data in their format.

**Acceptance Criteria:**
- Field addition, removal, rename
- Type coercion
- Template-based value generation
- Chain multiple transformations

---

## E3: Stream Processing

### E3.1: Stateless Processors

As a developer, I want to apply stateless transformations so that events can be enriched or normalized.

**Acceptance Criteria:**
- Map: Transform event data
- Filter: Pass/fail based on predicate
- FlatMap: Produce multiple events
- Drop: Discard matching events

### E3.2: Stateful Processors

As a developer, I want to aggregate events over windows so that I can compute metrics.

**Acceptance Criteria:**
- Tumbling windows (non-overlapping)
- Sliding windows (overlapping)
- Session windows (activity-based)
- Count windows (fixed count)
- Aggregate functions: sum, count, min, max, avg

### E3.3: Stream Joins

As a developer, I want to join events from multiple streams so that I can correlate data.

**Acceptance Criteria:**
- Windowed joins
- Equi-join on key fields
- Late event handling
- Grace period configuration

---

## E4: Delivery

### E4.1: Target Adapters

As an operator, I want to deliver events to various targets so that consumers can use their preferred technology.

**Acceptance Criteria:**
- HTTP POST to configured endpoints
- Kafka topic production
- NATS subject publication
- File output with rotation
- Stdout for debugging

### E4.2: Retry and Dead Letter

As an operator, I want events to be retried on failure so that transient issues don't cause data loss.

**Acceptance Criteria:**
- Configurable retry attempts
- Exponential backoff
- Dead letter queue for failed events
- Dead letter inspection and replay

### E4.3: Consumer Groups

As a developer, I want to consume events in a consumer group so that I can scale horizontally.

**Acceptance Criteria:**
- Consumer group management
- Offset tracking and replay
- Rebalance handling
- At-least-once delivery guarantee

---

## E5: Observability

### E5.1: Distributed Tracing

As an operator, I want to trace events through the pipeline so that I can debug latency issues.

**Acceptance Criteria:**
- OpenTelemetry trace propagation
- Span for each pipeline stage
- Trace context injection into events
- Trace sampling configuration

### E5.2: Metrics

As an operator, I want to monitor eventra so that I can detect issues.

**Acceptance Criteria:**
- Prometheus-compatible metrics endpoint
- Key metrics: ingestion rate, latency, queue depth, error rate
- Labels for filtering (source, type, route, target)
- Metric cardinality limits

### E5.3: Health Checks

As a platform engineer, I want health checks so that Eventra can be deployed in orchestrated environments.

**Acceptance Criteria:**
- `/health/live` for liveness
- `/health/ready` for readiness
- Checks: ingestion port, queue depth, target connectivity

---

## E6: Schema Management

### E6.1: Schema Registry

As a developer, I want to register schemas so that events can be validated.

**Acceptance Criteria:**
- CRUD operations via API
- Schema versioning with compatibility modes
- JSON Schema and Avro support
- Schema retrieval by ID or name

### E6.2: Schema Validation

As a developer, I want events validated against schemas so that bad data is caught early.

**Acceptance Criteria:**
- Validation on ingestion
- Reject or tag invalid events (configurable)
- Validation error messages include path

### E6.3: Schema Evolution

As an operator, I want schemas to evolve so that the system can adapt.

**Acceptance Criteria:**
- Compatibility checking before registration
- Auto-registration with evolution
- Backward/Forward/Both compatibility modes

---

## E7: Security

### E7.1: Authentication

As an operator, I want to authenticate producers so that unauthorized events are rejected.

**Acceptance Criteria:**
- API key authentication
- JWT token validation
- mTLS for TCP connections
- OAuth2 token exchange (future)

### E7.2: Authorization

As an operator, I want to control access so that producers can only send to allowed routes.

**Acceptance Criteria:**
- ACL rules for produce/consume
- Resource-based permissions
- Default deny policy

---

## Milestones

| Phase | Target | Features |
|-------|--------|----------|
| **MVP** | Week 1-2 | HTTP ingest, routing, HTTP delivery |
| **v1** | Week 3-4 | TCP ingest, retries, consumer groups |
| **v2** | Week 5-6 | Stream processing, schema registry |
| **v3** | Week 7-8 | Observability, security hardening |

---

## Future Roadmap

- **Phase 2**: Kafka native support
- **Phase 3**: gRPC ingestion
- **Phase 4**: Multi-region replication
- **Phase 5**: Stream SQL query engine
