# PhenoEvents

Distributed event-driven architecture primitives and multi-broker event bus enabling loosely-coupled service communication, event sourcing, and reactive patterns across the Phenotype ecosystem.

## Overview

PhenoEvents is the foundational event-driven messaging layer for Phenotype microservices. It provides a unified, broker-agnostic abstraction over Kafka, RabbitMQ, AWS pub/sub, and GCP Cloud Pub/Sub—enabling services to publish domain events and subscribe to event streams without tight coupling. Integrated event sourcing with append-only audit logs, schema management, and deduplication ensures consistency and temporal queryability across the platform.

## Technology Stack

- **Core**: Rust (async/await) with Tokio runtime
- **Message Brokers**: Kafka (rdkafka v0.36+), RabbitMQ (lapin v0.34+), AWS SNS/SQS (aws-sdk v0.28+), GCP Pub/Sub
- **Serialization**: Apache Avro, Protocol Buffers v3, serde JSON with compression (gzip, snappy)
- **Event Store**: Append-only ledger with SHA-256 hash chains and temporal indexing (phenotype-event-sourcing)
- **Async Runtime**: Tokio with configurable work-stealing executor and batch processing
- **Schema Registry**: Confluent Schema Registry client (HTTP), local TOML schema fallback, runtime schema validation
- **Monitoring**: Prometheus metrics, structured logging (tracing), OpenTelemetry export

## Key Features

- **Broker Abstraction**: Single trait for Kafka, RabbitMQ, cloud pub/sub
- **Event Schema Management**: Versioned event definitions with backward compatibility
- **Event Sourcing**: Append-only log with temporal queries and projections
- **Dead Letter Queues**: Automatic retry + DLQ routing for failed events
- **Fan-Out Subscriptions**: Multiple subscribers per event type with independent offsets
- **Filtering & Routing**: Content-based routing, event header predicates
- **Observability**: Structured event logging, trace correlation, metrics
- **Deduplication**: Idempotent event handling with distributed state

## Quick Start

```bash
# Clone the repository
git clone https://github.com/KooshaPari/PhenoEvents.git
cd PhenoEvents

# Review CLAUDE.md for conventions
cat CLAUDE.md

# Build the core event system
cargo build --all-features
cargo build --release

# Run tests (includes broker integration tests)
cargo test --all -- --test-threads=1

# Start a local event broker (optional; requires Docker)
docker-compose -f examples/docker-compose.yml up -d

# Run an example producer and consumer
cargo run --example producer -- --broker kafka --topic events.test
cargo run --example consumer -- --broker kafka --topic events.test
```

## Project Structure

```
PhenoEvents/
  Cargo.toml                     # Workspace manifest
  crates/
    phenotype-events-core/       # Event trait, subscriber interface
    phenotype-events-kafka/      # Kafka producer/consumer impl
    phenotype-events-rabbitmq/   # RabbitMQ integration
    phenotype-events-cloud/      # AWS SNS/SQS, GCP Pub/Sub impl
    phenotype-events-sourcing/   # Event store, projections, snapshots
    phenotype-events-schema/     # Schema registry client, validation
  examples/
    producer.rs                  # Publish domain events
    consumer.rs                  # Subscribe and handle events
    event_sourcing.rs            # Event store snapshot & replay
  tests/
    integration/
      test_kafka_broker.rs       # Kafka end-to-end tests
      test_event_sourcing.rs     # Event store consistency
```

## Related Phenotype Projects

- **[phenotype-bus](../phenotype-bus/)** — Event bus implementation; primary consumer of PhenoEvents
- **[Tracera](../Tracera/)** — Distributed tracing; subscribes to PhenoEvents for trace correlation
- **[cloud](../cloud/)** — Multi-tenant platform; uses event sourcing for audit trail

## Governance & Contributing

- **CLAUDE.md** — Workspace conventions, broker integration policies
- **Event Catalog**: [docs/events/](docs/events/)
- **Schema Definitions**: [schemas/](schemas/)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)

See [AGENTS.md](AGENTS.md) for testing requirements and spec traceability.
