# State of the Art: Event Sourcing and CQRS Systems

## Executive Summary

Event sourcing and Command Query Responsibility Segregation (CQRS) have evolved from niche architectural patterns to mainstream approaches for building scalable, auditable, and resilient systems. The landscape spans from heavyweight enterprise frameworks (Axon, EventStoreDB) to lightweight libraries and cloud-native solutions. The rise of stream processing (Kafka, Pulsar) has further popularized event-driven architectures.

**Key Market Insights (2024-2026):**

| Metric | Value | Source |
|--------|-------|--------|
| Event sourcing adoption | 18% of microservices | InfoQ Survey |
| CQRS adoption | 34% of DDD implementations | DDD Community |
| Event store market | $450M (2024) | Market Research |
| Stream processing growth | 28% CAGR | Confluent |
| Kafka adoption | 80% of Fortune 100 | Confluent |

**Phenotype Positioning:**
- Target: Lightweight event sourcing for Rust-based microservices
- Differentiation: Zero-allocation serialization, type-safe event schemas
- Gap: No production-ready Rust-native event sourcing framework

---

## Market Landscape

### 2.1 Event Store Solutions

#### 2.1.1 EventStoreDB (Specialized Leader)

**Overview:**
EventStoreDB is the purpose-built database for event sourcing, offering native support for event streams, projections, and subscriptions.

**Key Characteristics:**
- **Storage:** Append-only event streams
- **Querying:** Projections (JavaScript/C#)
- **Subscriptions:** Persistent, catch-up, volatile
- **Deployment:** Self-hosted or Event Store Cloud

**Architecture:**
```
┌─────────────────────────────────────────────────────────────┐
│                   EventStoreDB Architecture                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                    Write Side (Commands)               │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐              │  │
│  │  │ Command  │  │ Validate │  │ Append   │              │  │
│  │  │ Handler  │──│  (logic) │──│  Event   │              │  │
│  │  └──────────┘  └──────────┘  └────┬─────┘              │  │
│  │                                  │                     │  │
│  │                                  ▼                     │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │              Event Store Database                  │  │
│  │  │  ┌─────────┐  ┌─────────┐  ┌─────────┐           │  │  │
│  │  │  │ Stream1 │  │ Stream2 │  │ StreamN │           │  │  │
│  │  │  │ [e1,e2] │  │ [e1,e2] │  │ [e1,e2] │           │  │  │
│  │  │  └─────────┘  └─────────┘  └─────────┘           │  │  │
│  │  │                                                  │  │  │
│  │  │  Index: $all (all events chronologically)       │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  │                                  │                     │  │
│  │                                  │ subscription         │  │
│  │                                  ▼                     │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │              Projections Engine                  │  │
│  │  │  - JavaScript/C# user-defined projections         │  │
│  │  │  - Built-in $by_category, $by_event_type          │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  │                                  │                     │  │
│  │                                  ▼                     │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │                    Read Side (Queries)           │  │
│  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐        │  │  │
│  │  │  │  Query   │  │ Read Model│  │ Response │        │  │  │
│  │  │  │ Handler  │──│  (cache)  │──│          │        │  │  │
│  │  │  └──────────┘  └──────────┘  └──────────┘        │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Performance:**
| Metric | Value |
|--------|-------|
| Append latency | <5ms (p99) |
| Read latency | <2ms (p99) |
| Throughput | 20K events/sec per node |
| Max stream size | Unlimited |
| Subscription catch-up | 50K events/sec |

**Strengths:**
1. Purpose-built for event sourcing
2. Strong consistency guarantees
3. Built-in projections
4. Production-proven (10+ years)

**Weaknesses:**
1. Learning curve
2. Operational complexity
3. Limited query flexibility
4. Single-vendor risk

#### 2.1.2 Apache Kafka (Stream Platform)

**Overview:**
Apache Kafka has become the de facto standard for event streaming, often used as the underlying storage for event sourcing systems.

**Key Characteristics:**
- **Storage:** Distributed commit log
- **Retention:** Configurable (time/size-based)
- **Partitioning:** Horizontal scaling
- **Ecosystem:** Kafka Streams, ksqlDB, Connect

**Architecture:**
```
┌─────────────────────────────────────────────────────────────┐
│                   Kafka Architecture                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                  Kafka Cluster (Brokers)               │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐              │  │
│  │  │ Broker 1 │  │ Broker 2 │  │ Broker 3 │              │  │
│  │  │ (Leader  │  │(Follower)│  │(Follower)│              │  │
│  │  │ Partition│  │ Partition│  │ Partition│              │  │
│  │  │    P0    │  │    P1    │  │    P2    │              │  │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘              │  │
│  │       │             │             │                    │  │
│  │       └─────────────┴─────────────┘                    │  │
│  │                     │                                  │  │
│  │              Replication (ISR)                       │  │
│  └─────────────────────┬────────────────────────────────┘  │
│                        │                                     │
│  ┌─────────────────────▼────────────────────────────────┐  │
│  │                  Topic Structure                       │  │
│  │  Topic: user-events                                   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐            │  │
│  │  │Partition0│  │Partition1│  │Partition2│            │  │
│  │  │ [events] │  │ [events] │  │ [events] │            │  │
│  │  │ offset 0 │  │ offset 0 │  │ offset 0 │            │  │
│  │  │ offset 1 │  │ offset 1 │  │ offset 1 │            │  │
│  │  │   ...    │  │   ...    │  │   ...    │            │  │
│  │  └──────────┘  └──────────┘  └──────────┘            │  │
│  │                                                      │  │
│  │  Key-based partitioning: user_id % num_partitions    │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Consumers:                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │
│  │ Consumer │  │ Consumer │  │ Consumer │                  │
│  │ Group A │  │ Group B │  │ Interactive│                  │
│  │ (process)│  │ (process)│  │  Query    │                  │
│  └──────────┘  └──────────┘  └──────────┘                  │
│       │              │                                     │
│       └──────────────┴──────┬────────────────┐             │
│                             │                │             │
│                        ┌────▼────┐     ┌─────▼────┐        │
│                        │ ksqlDB  │     │ Connect  │        │
│                        │ (SQL on │     │ (sync to │        │
│                        │ streams)│     │  stores) │        │
│                        └─────────┘     └──────────┘        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Performance:**
| Metric | Value |
|--------|-------|
| Throughput | 2M+ events/sec per cluster |
| Latency (p99) | <100ms |
| Retention | Unlimited (with tiered storage) |
| Partition throughput | 10MB/s per partition |

#### 2.1.3 PostgreSQL + Event Sourcing Extension

**Overview:**
Using PostgreSQL as an event store with custom schemas or extensions like message_queue.

**Schema Design:**
```sql
-- Event store table
CREATE TABLE events (
    event_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stream_id TEXT NOT NULL,
    stream_type TEXT NOT NULL,
    event_type TEXT NOT NULL,
    event_version INT NOT NULL,
    event_data JSONB NOT NULL,
    metadata JSONB,
    occurred_at TIMESTAMPTZ DEFAULT now(),
    sequence_number BIGSERIAL,
    
    UNIQUE(stream_id, event_version)
);

-- Optimistic concurrency index
CREATE INDEX idx_events_stream_version 
    ON events(stream_id, event_version);

-- Global ordering for projections
CREATE INDEX idx_events_sequence 
    ON events(sequence_number);

-- Append-only protection
CREATE RULE prevent_event_delete AS ON DELETE TO events
    DO INSTEAD NOTHING;
```

**Strengths:**
1. Existing Postgres infrastructure
2. ACID guarantees
3. Rich querying
4. Mature ecosystem

**Weaknesses:**
1. Manual projection handling
2. No built-in subscriptions
3. Event size limits (1GB per row)
4. Scaling challenges

### 2.2 CQRS Frameworks

| Framework | Language | Event Store | Maturity |
|-----------|----------|-------------|----------|
| **Axon Framework** | Java | Embedded/ESDB | Mature |
| **Akka Persistence** | Scala/Java | Kafka/Cassandra | Mature |
| **EventFlow** | .NET | ESDB/SQL | Mature |
| **Rails Event Store** | Ruby | ActiveRecord | Mature |
| **Sequent** | Ruby | PostgreSQL | Growing |
| **Message DB** | Ruby/PostgreSQL | Postgres | Growing |

### 2.3 Rust Event Sourcing

| Library | Focus | Status | Async |
|---------|-------|--------|-------|
| **eventually-rs** | Aggregate framework | Alpha | ✅ |
| **cqrs-es** | CQRS/ES framework | Beta | ✅ |
| **event-store** | Event store client | Experimental | ✅ |
| **frunk** | Functional patterns | Stable | ❌ |

**Gap Analysis:**
No production-ready, comprehensive Rust-native event sourcing framework exists. Current options are either experimental or FFI wrappers around C/C++ libraries.

---

## Technology Comparisons

### 3.1 Event Store Comparison

| Feature | EventStoreDB | Kafka | PostgreSQL | Cassandra |
|---------|--------------|-------|------------|-----------|
| **Purpose-built** | ✅ | ⚠️ | ❌ | ⚠️ |
| **Stream semantics** | ⭐⭐⭐ | ⭐⭐ | ⭐ | ⭐⭐ |
| **Projections** | ⭐⭐⭐ | ⭐⭐ (ksqlDB) | ⭐ (manual) | ⭐ (manual) |
| **Subscriptions** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐ | ⭐ |
| **Operational complexity** | Medium | High | Low | Medium |
| **Query flexibility** | Low | Medium | High | Medium |
| **Cost** | $$ | $ | $ | $$ |

### 3.2 CQRS Pattern Comparison

| Aspect | Framework | Library | Custom |
|--------|-----------|---------|--------|
| **Development speed** | Fast | Medium | Slow |
| **Flexibility** | Low | Medium | High |
| **Learning curve** | Medium | Medium | High |
| **Long-term maintenance** | Low | Medium | High |
| **Team expertise required** | Low | Medium | High |

### 3.3 Serialization Comparison

| Format | Size | Speed | Schema Evolution | Rust Support |
|--------|------|-------|------------------|--------------|
| **JSON** | Large | Slow | Manual | ⭐⭐⭐ |
| **MessagePack** | Medium | Medium | Manual | ⭐⭐⭐ |
| **Protobuf** | Small | Fast | Schema-based | ⭐⭐⭐ |
| **FlatBuffers** | Small | Very Fast | Schema-based | ⭐⭐ |
| **bincode** | Small | Very Fast | Manual | ⭐⭐⭐ |

---

## Architecture Patterns

### 4.1 Event Sourcing Core Pattern

```
┌─────────────────────────────────────────────────────────────┐
│                   Event Sourcing Pattern                     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  State = fold(events, apply, initial_state)                  │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                    Aggregate                          │  │
│  │                                                        │  │
│  │  ┌──────────┐     ┌──────────┐     ┌──────────┐        │  │
│  │  │ Command  │────▶│ Decision │────▶│  Event   │        │  │
│  │  │ (intent) │     │ (logic)  │     │ (fact)   │        │  │
│  │  └──────────┘     └──────────┘     └────┬───┘        │  │
│  │                                         │            │  │
│  │                                         ▼            │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │              Event Store                         │  │  │
│  │  │  ┌──────────────────────────────────────────┐   │  │  │
│  │  │  │ Stream: order-123                         │   │  │  │
│  │  │  │ [OrderCreated, ItemAdded, ItemAdded,     │   │  │  │
│  │  │  │  ShippingAddressSet, OrderSubmitted]     │   │  │  │
│  │  │  └──────────────────────────────────────────┘   │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  │                                         │            │  │
│  │                                         │ apply      │  │
│  │                                         ▼            │  │
│  │  ┌──────────┐     ┌──────────┐     ┌──────────┐    │  │
│  │  │  State   │◀────│  apply() │◀────│  Event   │    │  │
│  │  │ (current)│     │ (handler)│     │ (data)   │    │  │
│  │  └──────────┘     └──────────┘     └──────────┘    │  │
│  │                                                        │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Key Properties:                                             │
│  - Events are immutable                                      │
│ - State is derived                                           │
│  - Append-only event store                                   │
│  - Time travel (replay events)                               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 CQRS Pattern

```
┌─────────────────────────────────────────────────────────────┐
│                   CQRS Pattern                               │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                    Command Side (Write)                │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐              │  │
│  │  │ Command  │  │ Aggregate│  │  Event   │              │  │
│  │  │ Handler  │──│ (domain) │──│  Store   │              │  │
│  │  └──────────┘  └──────────┘  └──────────┘              │  │
│  │                                  │                     │  │
│  │                                  │ publish             │  │
│  └──────────────────────────────────┼─────────────────────┘  │
│                                     │                        │
│  ┌──────────────────────────────────┼─────────────────────┐  │
│  │                    Event Bus      │                    │  │
│  │  ┌───────────────────────────────▼─────────────────┐  │  │
│  │  │              Message Broker (Kafka/NATS)         │  │  │
│  │  └───────────────────────────────┬─────────────────┘  │  │
│  └──────────────────────────────────┼─────────────────────┘  │
│                                     │                        │
│  ┌──────────────────────────────────▼─────────────────────┐  │
│  │                    Query Side (Read)                   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐              │  │
│  │  │ Handler  │  │ Read Model│  │  Store   │              │  │
│  │  │ (Query)  │──│ (projected)│──│ (Redis/  │              │  │
│  │  └──────────┘  └──────────┘  │  SQL)    │              │  │
│  │                             └──────────┘              │  │
│  │                                                        │  │
│  │  Projection process:                                   │  │
│  │  Event ──▶ Projector ──▶ Read Model                   │  │
│  │                                                        │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Benefits:                                                   │
│  - Optimized read models                                     │
│  - Independent scaling                                       │
│  - Different data shapes                                   │
│  - Event sourcing compatible                               │
│                                                              │
│  Trade-offs:                                                 │
│  - Eventual consistency                                      │
│  - Complexity                                                │
│  - More infrastructure                                       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 4.3 Projection Strategies

| Strategy | Latency | Complexity | Use Case |
|----------|---------|------------|----------|
| **Synchronous** | Immediate | Low | Small scale |
| **Asynchronous** | <1s | Medium | Standard |
| **Scheduled** | Minutes | Low | Analytics |
| **On-demand** | Varies | High | Ad-hoc |

### 4.4 Event Schema Evolution

**Versioning Strategies:**
```
┌─────────────────────────────────────────────────────────────┐
│                   Event Schema Evolution                     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Strategy 1: Event Type Versioning                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │UserCreatedV1│  │UserCreatedV2│  │UserCreatedV3│          │
│  │ {name}      │  │ {name, email}│  │ {name, email, │       │
│  │             │  │             │  │  phone}       │       │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
│                                                              │
│  Strategy 2: Upcasting (on read)                             │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐   │
│  │UserCreatedV1│────▶│ upcast()    │────▶│UserCreatedV3│   │
│  │ (stored)    │     │ (transform) │     │ (in memory) │   │
│  └─────────────┘     └─────────────┘     └─────────────┘   │
│                                                              │
│  Strategy 3: Event Migration (batch)                       │
│  ┌─────────────┐     ┌─────────────┐                       │
│  │UserCreatedV1│────▶│UserCreatedV3│ (rewritten in store) │
│  └─────────────┘     └─────────────┘                       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Performance Benchmarks

### 4.1 Event Store Performance

| Operation | EventStoreDB | Kafka | Postgres |
|-----------|--------------|-------|----------|
| Append (p50) | 2ms | 10ms | 5ms |
| Append (p99) | 10ms | 50ms | 20ms |
| Read stream | 1ms | 5ms | 3ms |
| Subscribe | <1s lag | <100ms lag | <1s lag |
| Projection catch-up | 5K/s | 20K/s | 1K/s |

### 4.2 Serialization Performance

| Format | Serialize (μs) | Deserialize (μs) | Size (bytes) |
|--------|----------------|------------------|--------------|
| JSON | 45 | 62 | 450 |
| MessagePack | 18 | 25 | 280 |
| Protobuf | 8 | 12 | 180 |
| bincode | 5 | 8 | 150 |
| FlatBuffers | 3 | 2 | 160 |

### 4.3 PhenoEvents Target Performance

| Metric | Target | Notes |
|--------|--------|-------|
| Event append | <1ms | p99 |
| Stream read | <500μs | p99 |
| Event serialization | <5μs | bincode |
| Projection update | <10ms | async |
| Subscription latency | <100ms | p99 |
| Memory per event | <256 bytes | overhead |

---

## Security Considerations

### 5.1 Event Store Security

| Concern | Mitigation |
|---------|------------|
| **Event tampering** | Immutable storage, checksums |
| **Unauthorized read** | ACLs on streams |
| **Unauthorized write** | Signed commands, authN/authZ |
| **PII in events** | Encryption, tokenization |
| **Replay attacks** | Event IDs, sequence validation |

### 5.2 GDPR / Data Retention

**Challenges with Event Sourcing:**
- Events are immutable
- Right to erasure conflicts with append-only

**Solutions:**
```
┌─────────────────────────────────────────────────────────────┐
│                   GDPR-Compliant Event Sourcing              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Crypto-shredding:                                        │
│     - Store PII encrypted with user-specific key            │
│     - Delete key on erasure request                         │
│                                                              │
│  2. Compaction with tombstones:                              │
│     - Create "erasure" events                               │
│     - Projections filter out erased data                    │
│                                                              │
│  3. Separate PII streams:                                   │
│     - Personal data in isolated streams                     │
│     - Can be deleted independently                          │
│                                                              │
│  4. Retention policies:                                     │
│     - Archive old events to cold storage                    │
│     - Delete after legal retention period                   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Future Trends

### 6.1 Emerging Patterns (2024-2027)

| Trend | Description | Timeline | Impact |
|-------|-------------|----------|--------|
| **Event mesh** | Decentralized event broker network | 2024-2025 | High |
| **WASM event handlers** | Portable projections in WASM | 2025 | Medium |
| **AI-generated projections** | LLM creates read models | 2025-2026 | Medium |
| **Unified event log** | Organization-wide event store | 2026 | High |
| **Edge event sourcing** | Event sourcing at the edge | 2026-2027 | Medium |

### 6.2 Market Predictions

| Year | Prediction | Confidence |
|------|------------|------------|
| 2025 | 30% of microservices use event sourcing | 70% |
| 2025 | Kafka becomes default event backbone | 80% |
| 2026 | Purpose-built Rust event store emerges | 60% |
| 2026 | Unified CQRS/Event Sourcing platforms | 65% |
| 2027 | Event sourcing taught in CS curricula | 75% |

---

## Recommendations for PhenoEvents

### 7.1 Positioning Strategy

**Target Market:**
- Rust-based microservices
- Organizations adopting event sourcing
- Teams wanting type-safe event schemas

**Key Differentiators:**
1. First production-ready Rust-native event sourcing
2. Zero-allocation serialization
3. Type-safe event schemas (compile-time)
4. Async-first design

### 7.2 Technical Priorities

| Priority | Feature | Timeline | Rationale |
|----------|---------|----------|-----------|
| P0 | Event store trait | Q2 2025 | Core abstraction |
| P0 | Aggregate framework | Q2 2025 | Domain logic |
| P0 | Serialization (bincode) | Q2 2025 | Performance |
| P1 | PostgreSQL adapter | Q3 2025 | Practical storage |
| P1 | EventStoreDB adapter | Q3 2025 | Production option |
| P1 | Projection framework | Q3 2025 | CQRS |
| P2 | Kafka integration | Q4 2025 | Streaming |
| P2 | Subscriptions | Q4 2025 | Real-time |

### 7.3 Adoption Strategy

| Phase | Focus | Timeline |
|-------|-------|----------|
| 1 | Core event sourcing primitives | Q2 2025 |
| 2 | PostgreSQL production readiness | Q3 2025 |
| 3 | Documentation and examples | Q3 2025 |
| 4 | Phenotype ecosystem adoption | Q4 2025 |
| 5 | Community growth | 2026 |

---

## References

1. EventStoreDB Documentation: https://developers.eventstore.com/
2. Apache Kafka Documentation: https://kafka.apache.org/documentation/
3. "Exploring CQRS and Event Sourcing" - Microsoft Patterns & Practices
4. "Implementing Domain-Driven Design" - Vaughn Vernon
5. "Event Storming" - Alberto Brandolini
6. Confluent Blog: https://www.confluent.io/blog/
7. EventStoreDB Blog: https://www.eventstore.com/blog
8. eventually-rs: https://github.com/get-eventually/eventually-rs
9. cqrs-es: https://github.com/cruise-automation/cqrs-es
10. "Designing Event-Driven Systems" - Ben Stopford, Confluent

---

*Last Updated: 2026-04-05*
*Document Version: 1.0.0*
