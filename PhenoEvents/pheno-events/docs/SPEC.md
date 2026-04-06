# Eventra Specification

> Event routing, stream processing, and observability pipeline for Phenotype infrastructure

**Version**: 1.0  
**Status**: Draft  
**Last Updated**: 2026-04-05  
**Project**: Eventra  

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Event Model](#event-model)
4. [Routing Engine](#routing-engine)
5. [Stream Processing](#stream-processing)
6. [Transport Adapters](#transport-adapters)
7. [Observability](#observability)
8. [Schema Management](#schema-management)
9. [Security](#security)
10. [Performance Engineering](#performance-engineering)
11. [CLI Specification](#cli-specification)
12. [Configuration](#configuration)
13. [References](#references)

---

## Overview

Eventra is an event routing and stream processing system designed for high-throughput, low-latency event distribution within the Phenotype ecosystem. It provides a unified pipeline for event transport, transformation, filtering, and delivery across diverse systems.

### Value Proposition

| Capability | Eventra | Kafka | NATS | RabbitMQ |
|------------|---------|-------|------|----------|
| **Lightweight** | ✅ Single binary | ❌ JVM | ✅ Serverless | ⚠️ Erlang |
| **Native routing** | ✅ Rule-based | ⚠️ Topic | ✅ Subjects | ⚠️ Exchange |
| **Stream processing** | ✅ Built-in | ⚠️ Kafka Streams | ❌ JetStream | ❌ No |
| **Schema registry** | ✅ Built-in | ⚠️ Confluent | ❌ No | ❌ No |
| **ephemeral subscribers** | ✅ Yes | ❌ No | ✅ Yes | ⚠️ Temp |
| **HTTP ingestion** | ✅ Yes | ❌ No | ❌ No | ❌ No |
| **Observability** | ✅ OTLP native | ⚠️ JMX | ⚠️ Prometheus | ⚠️ Prometheus |

### Core Features

1. **Event Routing**: Rule-based routing with predicates and transformations
2. **Stream Processing**: Stateful and stateless stream operations
3. **Transport Abstraction**: Pluggable transports (HTTP, TCP, Unix Socket)
4. **Schema Registry**: JSON Schema and Avro support with evolution
5. **Observability**: OpenTelemetry-native tracing, metrics, logging
6. **Backpressure**: Built-in flow control across the pipeline
7. **Replay**: Event replay with consumer group offsets

---

## Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              Eventra Architecture                                │
│                                                                                   │
│  ┌───────────────────────────────────────────────────────────────────────────┐ │
│  │                              Ingestion Layer                                 │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │ │
│  │  │  HTTP    │  │   TCP    │  │  Unix   │  │   gRPC   │                  │ │
│  │  │  Server  │  │  List.  │  │  Socket │  │  Server  │                  │ │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘                  │ │
│  │        └──────────────┴──────────────┴──────────────┘                       │ │
│  └───────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                              │
│                                    ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────────────┐ │
│  │                           Pipeline Layer                                     │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │ │
│  │  │  Router  │→│  Filter  │→│Transform │→│  Queue   │                  │ │
│  │  │ (rules)  │  │ (preds) │  │ (enrich) │  │ (buffer)│                  │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘                  │ │
│  └───────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                              │
│                                    ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────────────┐ │
│  │                           Delivery Layer                                    │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │ │
│  │  │  HTTP    │  │   Kafka  │  │   NATS   │  │   Disk   │                  │ │
│  │  │  Client  │  │ Producer │  │ Publisher│  │  Replay  │                  │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘                  │ │
│  └───────────────────────────────────────────────────────────────────────────┘ │
│                                                                                   │
│  ┌───────────────────────────────────────────────────────────────────────────┐ │
│  │                         Control Plane                                       │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │ │
│  │  │  Admin   │  │ Schema   │  │ Consumer │  │ Observ.  │                  │ │
│  │  │  API     │  │ Registry │  │ Groups   │  │ (OTLP)   │                  │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘                  │ │
│  └───────────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Event Flow

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              Event Flow Through Eventra                           │
│                                                                                   │
│  Producer                                                                Consumer  │
│    │                                                                       ▲     │
│    │                                                                       │     │
│    ▼                                                                       │     │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐ │     │
│  │  Ingest    │────▶│   Route    │────▶│   Filter   │────▶│  Deliver   │─┘     │
│  │  (parse)   │     │  (rules)   │     │  (preds)   │     │  (retry)   │       │
│  └─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘       │
│         │                   │                   │                                      │
│         │                   │                   │                                      │
│         ▼                   ▼                   ▼                                      │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                           │
│  │   Schema    │     │  Transform  │     │   Queue     │                           │
│  │  Validate   │     │  (enrich)   │     │  (buffer)   │                           │
│  └─────────────┘     └─────────────┘     └─────────────┘                           │
│                                                                                   │
│  Trace: ingestion → route → filter → transform → queue → deliver                    │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Event Model

### Event Structure

```go
type Event struct {
    // ID uniquely identifies the event (ULID for sortability)
    ID ULID `json:"id"`
    
    // Type is the event type name (e.g., "user.created")
    Type string `json:"type"`
    
    // Source identifies the producer (e.g., "auth-service")
    Source string `json:"source"`
    
    // Subject is the entity the event concerns (e.g., "user-123")
    Subject string `json:"subject"`
    
    // Data is the event payload
    Data json.RawMessage `json:"data"`
    
    // Metadata contains transport-level information
    Metadata EventMetadata `json:"metadata"`
    
    // Extensions for middleware
    Extensions map[string]any `json:"extensions,omitempty"`
}

type EventMetadata struct {
    // Timestamp is when the event was produced
    Timestamp time.Time `json:"timestamp"`
    
    // ContentType describes the data format
    ContentType string `json:"content_type"`
    
    // TraceID for distributed tracing
    TraceID string `json:"trace_id,omitempty"`
    
    // SpanID for distributed tracing
    SpanID string `json:"span_id,omitempty"`
    
    // PartitionKey for ordering
    PartitionKey string `json:"partition_key,omitempty"`
    
    // SchemaID references the schema in the registry
    SchemaID string `json:"schema_id,omitempty"`
    
    // SchemaVersion for schema evolution
    SchemaVersion int `json:"schema_version,omitempty"`
}
```

### Event Types

| Category | Examples | Routing Key Pattern |
|----------|----------|-------------------|
| **Domain** | `user.created`, `order.completed` | `<domain>.<entity>.<action>` |
| **System** | `health.ping`, `config.changed` | `system.<service>.<event>` |
| **Audit** | `access.login`, `data.exported` | `audit.<actor>.<action>` |
| **Metric** | `counter.increment`, `gauge.set` | `metric.<name>.<operation>` |

### ULID for Event IDs

```go
import "github.com/oklog/ulid/v2"

// NewEventID generates a sortable unique ID
func NewEventID() (ULID, error) {
    entropy := ulid.Monotonic(rand.Reader, 0)
    return ulid.New(ulid.Timestamp(time.Now()), entropy)
}

// Event ID structure (26 characters):
// 01ARZ3NDEKTSV4RRFFQ69G5FAV  ← timestamp + entropy
// |──────| |───────────────|
// Timestamp (48 bits)    Entropy (80 bits)
```

---

## Routing Engine

### Route Rules

```go
type RouteRule struct {
    // Name identifies the route
    Name string `json:"name"`
    
    // Priority determines evaluation order (higher = first)
    Priority int `json:"priority"`
    
    // Match defines what events this route matches
    Match RouteMatch `json:"match"`
    
    // Actions defines what happens to matching events
    Actions []RouteAction `json:"actions"`
    
    // Config contains route-specific configuration
    Config RouteConfig `json:"config,omitempty"`
}

type RouteMatch struct {
    // Types matches event types (glob supported)
    Types []string `json:"types,omitempty"`
    
    // Sources matches event sources
    Sources []string `json:"sources,omitempty"`
    
    // Subjects matches subjects (supports regex)
    Subjects string `json:"subjects,omitempty"`
    
    // Predicates are additional filter expressions
    Predicates []Predicate `json:"predicates,omitempty"`
    
    // All is true if all predicates must match
    All bool `json:"all,omitempty"`
}

type RouteAction struct {
    // Target is the destination
    Target string `json:"target"`
    
    // Transform applies transformations before delivery
    Transform []Transform `json:"transform,omitempty"`
    
    // Retry configures retry behavior
    Retry *RetryConfig `json:"retry,omitempty"`
    
    // Timeout for delivery
    Timeout time.Duration `json:"timeout,omitempty"`
}
```

### Route Matching Examples

```yaml
routes:
  # Route all user events to analytics
  - name: user-events-to-analytics
    priority: 100
    match:
      types:
        - "user.*"
    actions:
      - target: analytics-pipeline

  # Route failed orders to dead letter
  - name: failed-orders
    priority: 200
    match:
      types:
        - "order.*"
      predicates:
        - expression: "data.status == 'failed'"
    actions:
      - target: dead-letter-queue

  # Route by partition key for ordering
  - name: ordered-events
    priority: 50
    match:
      types:
        - "session.*"
    config:
      partition_by: "data.session_id"
    actions:
      - target: ordered-stream
```

### Predicate Expression Language

```go
type Predicate struct {
    // Field is the JSON path to evaluate
    Field string `json:"field"`
    
    // Operator is the comparison operator
    Operator PredicateOperator `json:"operator"`
    
    // Value is the comparison value
    Value any `json:"value"`
}

enum PredicateOperator {
    EQ           // ==
    NEQ          // !=
    GT           // >
    GTE          // >=
    LT           // <
    LTE          // <=
    CONTAINS     // string contains
    STARTS_WITH  // string prefix
    ENDS_WITH    // string suffix
    MATCHES      // regex match
    IN           // value in array
    NOT_IN       // value not in array
    EXISTS       // field exists
    NOT_EXISTS   // field doesn't exist
}
```

---

## Stream Processing

### Processor Types

| Processor | Type | Description |
|-----------|------|-------------|
| **Map** | Stateless | Transform each event |
| **Filter** | Stateless | Pass/fail based on predicate |
| **FlatMap** | Stateless | Produce multiple events per input |
| **Aggregate** | Stateful | Combine events over window |
| **Join** | Stateful | Correlate events from multiple streams |
| **Window** | Stateful | Tumbling/sliding windows |
| **Drop** | Stateless | Discard events meeting condition |
| **Throttle** | Stateful | Rate limiting |

### Stream Definitions

```go
type Stream struct {
    // Name identifies the stream
    Name string `json:"name"`
    
    // Source is where events come from
    Source StreamSource `json:"source"`
    
    // Processors define the transformation pipeline
    Processors []Processor `json:"processors"`
    
    // Output defines where events go
    Output StreamOutput `json:"output"`
    
    // Config contains stream configuration
    Config StreamConfig `json:"config,omitempty"`
}

type StreamSource struct {
    // Type is the source type
    Type SourceType `json:"type"`
    
    // Config contains source-specific config
    Config map[string]any `json:"config,omitempty"`
}

enum SourceType {
    ROUTE       // From route matching
    INGEST      // Direct HTTP ingest
    KAFKA       // Kafka topic
    NATS        // NATS subject
    FILE        // File tail
    GENERATOR   // Test data generator
}

type StreamOutput struct {
    // Type is the output type
    Type OutputType `json:"type"`
    
    // Config contains output-specific config
    Config map[string]any `json:"config,omitempty"`
}

enum OutputType {
    DROP         // Discard
    LOG          // Print to log
    HTTP         // HTTP POST
    KAFKA        // Kafka topic
    NATS         // NATS subject
    FILE         // File write
    STDOUT       // Standard output
}
```

### Windowed Aggregation

```go
type WindowConfig struct {
    // Type is the window type
    Type WindowType `json:"type"`
    
    // Size is the window duration
    Size time.Duration `json:"size"`
    
    // Slide is the window slide (for sliding windows)
    Slide time.Duration `json:"slide,omitempty"`
    
    // EmitStrategy controls when results are emitted
    EmitStrategy WindowEmitStrategy `json:"emit_strategy"`
}

enum WindowType {
    TUMBLING   // Non-overlapping windows
    SLIDING    // Overlapping windows
    SESSION    // Activity-based windows
    COUNT      // Count-based windows
}

enum WindowEmitStrategy {
    ON_CLOSE       // Emit when window closes
    ON_CHANGE      // Emit when aggregate changes
    ON_TIMER       // Emit on schedule
}
```

---

## Transport Adapters

### HTTP Ingestion

```go
type HTTPServer struct {
    // Address to listen on
    Address string
    
    // Port to listen on
    Port int
    
    // TLS config
    TLS *TLSConfig
    
    // Max body size
    MaxBodySize int64
    
    // Read/Write timeouts
    ReadTimeout  time.Duration
    WriteTimeout time.Duration
    
    // Auth handler
    Auth AuthHandler
    
    // Rate limiter
    RateLimiter *RateLimiter
}

type IngestRequest struct {
    // Events to ingest (batch supported)
    Events []Event `json:"events"`
    
    // ContentType for data
    ContentType string `json:"content_type,omitempty"`
    
    // Source identification
    Source string `json:"source,omitempty"`
    
    // Routing hints
    RoutingHints map[string]string `json:"routing_hints,omitempty"`
}
```

### TCP Binary Protocol

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         Eventra Binary Protocol                                  │
│                                                                                   │
│  ┌──────────┬──────────┬──────────┬──────────┬──────────────────────────┐   │
│  │ Version  │  Flags   │ Timestamp│   TTL    │       Payload             │   │
│  │ (1 byte)│ (2 bytes)│ (8 bytes)│ (4 bytes)│   (variable)              │   │
│  └──────────┴──────────┴──────────┴──────────┴──────────────────────────┘   │
│                                                                                   │
│  Version: Protocol version (currently 1)                                         │
│  Flags:   Compression, encrypted, etc.                                           │
│  Timestamp: Event timestamp (Unix ms)                                             │
│  TTL:     Time to live in milliseconds (0 = infinite)                            │
│  Payload: CBOR-encoded event                                                     │
│                                                                                   │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Unix Socket (High Performance)

```go
type UnixSocketConfig struct {
    // Path to the socket file
    Path string
    
    // Permissions for socket file
    Perm uint32
    
    // Backlog for listen()
    Backlog int
    
    // Read buffer size
    ReadBufferSize int
    
    // Write buffer size
    WriteBufferSize int
}
```

---

## Observability

### OpenTelemetry Integration

```go
type ObservabilityConfig struct {
    // Enabled toggles observability
    Enabled bool `yaml:"enabled"`
    
    // ServiceName identifies this service
    ServiceName string `yaml:"service_name"`
    
    // ServiceVersion is this version
    ServiceVersion string `yaml:"service_version"`
    
    // OTLP endpoint for traces/metrics
    OTLPEndpoint string `yaml:"otlp_endpoint"`
    
    // Sampling configuration
    Sampling *SamplingConfig `yaml:"sampling"`
}

type SamplingConfig struct {
    // Type is the sampler type
    Type SamplerType `yaml:"type"`
    
    // Ratio for probabilistic sampling
    Ratio float64 `yaml:"ratio,omitempty"`
    
    // ParentBased enables parent-based sampling
    ParentBased bool `yaml:"parent_based"`
}
```

### Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `eventra.events.ingested` | Counter | source, type | Events ingested |
| `eventra.events.routed` | Counter | route, action | Events routed |
| `eventra.events.filtered` | Counter | route | Events filtered |
| `eventra.events.delivered` | Counter | target | Events delivered |
| `eventra.events.failed` | Counter | target, error | Delivery failures |
| `eventra.queue.depth` | Gauge | queue | Queue depth |
| `eventra.processing.latency` | Histogram | processor | Processing time |
| `eventra.route.latency` | Histogram | route | Route evaluation time |

### Tracing

| Span | Description |
|------|-------------|
| `eventra.ingest` | HTTP/TCP ingestion |
| `eventra.parse` | Event parsing and validation |
| `eventra.route` | Route evaluation |
| `eventra.filter` | Predicate evaluation |
| `eventra.transform` | Transformation processing |
| `eventra.queue` | Queue operations |
| `eventra.deliver` | Delivery to target |

---

## Schema Management

### Schema Registry

```go
type Schema struct {
    // ID is the unique schema identifier
    ID string `json:"id"`
    
    // Name is the schema name
    Name string `json:"name"`
    
    // Version is the schema version
    Version int `json:"version"`
    
    // Type is the schema type
    Type SchemaType `json:"type"`
    
    // Schema is the actual schema document
    Schema json.RawMessage `json:"schema"`
    
    // Compatibility mode
    Compatibility CompatibilityMode `json:"compatibility"`
    
    // CreatedAt is when the schema was registered
    CreatedAt time.Time `json:"created_at"`
}

enum SchemaType {
    JSON_SCHEMA
    AVRO
    PROTOBUF
}

enum CompatibilityMode {
    BACKWARD       // New schema can read old data
    FORWARD        // Old schema can read new data
    BOTH           // Bidirectional compatibility
    NONE           // No compatibility checking
}
```

### Schema Evolution

```go
type SchemaEvolution struct {
    // Strategy for handling schema changes
    Strategy EvolutionStrategy `json:"strategy"`
    
    // AutoRegister enables automatic registration
    AutoRegister bool `json:"auto_register"`
    
    // DefaultCompatibility sets new schema defaults
    DefaultCompatibility CompatibilityMode `json:"default_compatibility"`
}

enum EvolutionStrategy {
    STRICT     // Reject incompatible changes
    LENIENT    // Allow some incompatibilities
    AUTO       // Auto-resolve when possible
}
```

---

## Security

### Authentication

| Method | Description | Use Case |
|--------|-------------|----------|
| **API Key** | Static key in header | Internal services |
| **JWT** | Signed tokens | Multi-tenant |
| **mTLS** | Client certificates | High security |
| **OAuth2** | Token exchange | External integrations |

### Authorization

```go
type ACL struct {
    // Rules define permissions
    Rules []ACLRules `json:"rules"`
    
    // Default is the default policy
    Default ACLPolicy `json:"default"`
}

type ACLRule struct {
    // Principal identifies who
    Principal string `json:"principal"`
    
    // Actions define what they can do
    Actions []ACLAction `json:"actions"`
    
    // Resources define what they affect
    Resources []string `json:"resources"`
    
    // Conditions are additional constraints
    Conditions map[string]any `json:"conditions,omitempty"`
}

enum ACLAction {
    INGEST      // Submit events
    CONSUME     // Receive events
    MANAGE      // Modify config
    READ        // Read-only access
}

enum ACLPolicy {
    ALLOW       // Default allow
    DENY        // Default deny
}
```

---

## Performance Engineering

### Throughput Targets

| Metric | Target | Environment |
|--------|--------|-------------|
| **Ingestion** | 500K events/sec | Production |
| **Latency (p99)** | <5ms | Production |
| **Route evaluation** | <100μs | Hot path |
| **Memory (per instance)** | <512MB | Production |

### Optimization Strategies

| Optimization | Technique | Impact |
|--------------|------------|--------|
| Zero-copy parsing | Pool event buffers | -30% memory |
| SIMD validation | Parallel JSON parsing | +2x throughput |
| Lock-free queues | SPSC queues | -50% latency |
| Batch delivery | Accumulate for batch | +5x throughput |
| Connection pooling | Reuse connections | -40% overhead |

---

## CLI Specification

### Commands

| Command | Description |
|---------|-------------|
| `eventra serve` | Start the server |
| `eventra route list` | List routes |
| `eventra route create` | Create a route |
| `eventra route delete` | Delete a route |
| `eventra stream list` | List streams |
| `eventra stream create` | Create a stream |
| `eventra schema list` | List schemas |
| `eventra schema register` | Register a schema |
| `eventra consumer list` | List consumers |
| `eventra consumer create` | Create a consumer |
| `eventra metrics` | Show metrics |
| `eventra health` | Health check |

### Flags

| Flag | Short | Type | Default | Description |
|------|-------|------|---------|-------------|
| `--config` | `-c` | path | `eventra.yaml` | Config file |
| `--log-level` | `-l` | string | `info` | Log level |
| `--metrics-port` | | int | `9090` | Metrics port |
| `--admin-port` | | int | `8080` | Admin port |
| `--ingest-port` | | int | `8081` | Ingest port |

---

## Configuration

### Config File Structure

```yaml
server:
  ingest:
    http:
      address: "0.0.0.0"
      port: 8081
    tcp:
      enabled: true
      port: 8082
    unix_socket:
      enabled: false
      path: "/var/run/eventra.sock"
  
  admin:
    address: "0.0.0.0"
    port: 8080

routing:
  default_timeout: 30s
  max_queue_depth: 10000
  retry_attempts: 3

streams:
  buffer_size: 1000
  flush_interval: 100ms

observability:
  enabled: true
  service_name: "eventra"
  otlp_endpoint: "localhost:4317"
  sampling:
    type: "parent_based"
    ratio: 0.1

security:
  enabled: true
  api_keys:
    - key: "dev-key-123"
      name: "development"
    - key: "prod-key-456"
      name: "production"
```

---

## References

- [CloudEvents Spec](https://cloudevents.io/) - Event specification
- [ULID](https://github.com/ulid/spec) - Sortable unique IDs
- [OpenTelemetry](https://opentelemetry.io/) - Observability
- [JSON Schema](https://json-schema.org/) - Schema definition
- [NATS](https://nats.io/) - Pub/sub messaging
- [Kafka](https://kafka.apache.org/) - Event streaming
- [gRPC](https://grpc.io/) - RPC framework

---

## 14. Implementation Architecture

### 14.1 Module Structure

```
eventra/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── server/
│   │   ├── mod.rs
│   │   ├── http.rs          # HTTP server
│   │   ├── tcp.rs           # TCP server
│   │   ├── grpc.rs          # gRPC server
│   │   └── unix.rs          # Unix socket server
│   ├── ingestion/
│   │   ├── mod.rs
│   │   ├── parser.rs        # Event parsing
│   │   ├── validator.rs     # Schema validation
│   │   └── buffer.rs        # Ingestion buffering
│   ├── routing/
│   │   ├── mod.rs
│   │   ├── engine.rs        # Route evaluation engine
│   │   ├── matcher.rs       # Pattern matching
│   │   ├── predicates.rs    # Predicate evaluation
│   │   └── cache.rs         # Route result caching
│   ├── processing/
│   │   ├── mod.rs
│   │   ├── stream.rs        # Stream processor
│   │   ├── window.rs        # Window operations
│   │   ├── aggregate.rs     # Aggregation functions
│   │   └── join.rs          # Stream joining
│   ├── delivery/
│   │   ├── mod.rs
│   │   ├── http.rs          # HTTP client
│   │   ├── kafka.rs         # Kafka producer
│   │   ├── nats.rs          # NATS publisher
│   │   ├── file.rs          # File writer
│   │   └── retry.rs         # Retry logic
│   ├── schema/
│   │   ├── mod.rs
│   │   ├── registry.rs      # Schema storage
│   │   ├── validator.rs     # Schema validation
│   │   └── evolution.rs     # Compatibility checking
│   ├── observability/
│   │   ├── mod.rs
│   │   ├── tracing.rs       # OpenTelemetry traces
│   │   ├── metrics.rs       # Prometheus metrics
│   │   └── logging.rs       # Structured logging
│   ├── security/
│   │   ├── mod.rs
│   │   ├── auth.rs          # Authentication
│   │   ├── acl.rs           # Authorization
│   │   └── tls.rs           # TLS handling
│   ├── config/
│   │   ├── mod.rs
│   │   ├── loader.rs        # Config loading
│   │   └── validator.rs     # Config validation
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── memory.rs        # In-memory storage
│   │   ├── disk.rs          # Disk-based storage
│   │   └── queue.rs         # Persistent queues
│   └── cluster/
│       ├── mod.rs
│       ├── gossip.rs        # Gossip protocol
│       ├── consensus.rs     # Raft consensus
│       └── replication.rs   # Data replication
```

### 14.2 Event Processing Pipeline

```rust
/// Main event processing pipeline
pub struct Pipeline {
    /// Ingestion stage
    ingestion: Arc<dyn IngestionHandler>,
    
    /// Routing stage  
    router: Arc<Router>,
    
    /// Processing stage
    processors: Vec<Arc<dyn Processor>>,
    
    /// Delivery stage
    delivery: Arc<dyn DeliveryHandler>,
    
    /// Observability
    tracer: Arc<dyn Tracer>,
    meter: Arc<dyn Meter>,
}

impl Pipeline {
    /// Process a single event through the pipeline
    pub async fn process(&self, event: Event) -> Result<PipelineResult, PipelineError> {
        let span = self.tracer.start_span("pipeline.process");
        
        // Stage 1: Ingestion (parsing, validation)
        let parsed = self.ingestion.handle(event).await?;
        
        // Stage 2: Routing (route selection)
        let routes = self.router.route(&parsed).await?;
        
        // Stage 3: Processing (transformations)
        let mut processed = parsed;
        for processor in &self.processors {
            processed = processor.process(processed).await?;
        }
        
        // Stage 4: Delivery
        let results = self.delivery.deliver(processed, routes).await?;
        
        span.end();
        Ok(PipelineResult::new(results))
    }
}
```

### 14.3 Concurrency Model

```rust
/// Pipeline concurrency architecture
pub struct ConcurrentPipeline {
    /// Workers for ingestion (IO-bound)
    ingestion_workers: WorkerPool<IoBound>,
    
    /// Workers for routing (CPU-bound)
    routing_workers: WorkerPool<CpuBound>,
    
    /// Workers for processing (mixed)
    processing_workers: WorkerPool<Mixed>,
    
    /// Workers for delivery (IO-bound)
    delivery_workers: WorkerPool<IoBound>,
    
    /// Cross-stage channels
    channels: ChannelGraph,
}

/// Worker pool configuration
pub struct WorkerPool<T: WorkerType> {
    workers: Vec<Worker<T>>,
    queue: Arc<dyn WorkQueue>,
    metrics: WorkerMetrics,
    _phantom: PhantomData<T>,
}

/// Worker implementation
impl<T: WorkerType> WorkerPool<T> {
    pub fn spawn<F>(&self, f: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let permit = self.queue.acquire();
        self.metrics.queued_work.inc();
        
        tokio::spawn(async move {
            let _permit = permit;
            f.await
        })
    }
}
```

### 14.4 Memory Management

```rust
/// Zero-copy event buffer pool
pub struct EventBufferPool {
    /// Size classes for different event sizes
    small_pool: Pool<1024>,     // 1KB buffers
    medium_pool: Pool<8192>,   // 8KB buffers
    large_pool: Pool<65536>,    // 64KB buffers
    
    /// Dynamic allocation for oversized events
    oversized: Mutex<Vec<Vec<u8>>>,
}

impl EventBufferPool {
    /// Get a buffer for an event of approximate size
    pub fn acquire(&self, size_hint: usize) -> BufferGuard {
        match size_hint {
            0..=1024 => BufferGuard::Small(self.small_pool.get()),
            1025..=8192 => BufferGuard::Medium(self.medium_pool.get()),
            8193..=65536 => BufferGuard::Large(self.large_pool.get()),
            _ => BufferGuard::Dynamic(self.allocate_dynamic(size_hint)),
        }
    }
}

/// Smart pointer for pooled buffers
pub enum BufferGuard {
    Small(PoolGuard<1024>),
    Medium(PoolGuard<8192>),
    Large(PoolGuard<65536>),
    Dynamic(Vec<u8>),
}

impl Deref for BufferGuard {
    type Target = [u8];
    
    fn deref(&self) -> &Self::Target {
        match self {
            BufferGuard::Small(g) => g.as_slice(),
            BufferGuard::Medium(g) => g.as_slice(),
            BufferGuard::Large(g) => g.as_slice(),
            BufferGuard::Dynamic(v) => v.as_slice(),
        }
    }
}
```

---

## 15. API Specification (Detailed)

### 15.1 REST API Endpoints

#### 15.1.1 Event Ingestion

```yaml
paths:
  /v1/events:
    post:
      summary: Ingest events
      description: |
        Submit one or more events for routing and processing.
        Events are validated, routed, and delivered asynchronously.
      tags:
        - Ingestion
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                events:
                  type: array
                  items:
                    $ref: '#/components/schemas/Event'
                  maxItems: 1000
                options:
                  $ref: '#/components/schemas/IngestOptions'
      responses:
        '202':
          description: Events accepted
          content:
            application/json:
              schema:
                type: object
                properties:
                  accepted:
                    type: integer
                  rejected:
                    type: integer
                  errors:
                    type: array
                    items:
                      $ref: '#/components/schemas/IngestError'
                  trace_id:
                    type: string
        '400':
          description: Invalid request
        '401':
          description: Unauthorized
        '429':
          description: Rate limited

  /v1/events/batch:
    post:
      summary: Batch ingest events (compressed)
      description: |
        Submit large batches of events using compression.
        Supports gzip, zstd, and lz4 compression.
      tags:
        - Ingestion
      requestBody:
        required: true
        content:
          application/vnd.eventra.batch+json:
            schema:
              type: object
              properties:
                format:
                  type: string
                  enum: [json, avro, protobuf]
                compression:
                  type: string
                  enum: [none, gzip, zstd, lz4]
                count:
                  type: integer
                data:
                  type: string
                  format: binary
      responses:
        '202':
          description: Batch accepted
        '400':
          description: Invalid batch

  /v1/events/{event_id}:
    get:
      summary: Get event by ID
      description: Retrieve a specific event by its ULID
      tags:
        - Events
      parameters:
        - name: event_id
          in: path
          required: true
          schema:
            type: string
            pattern: '^[0-7][0-9A-HJKMNP-TV-Z]{25}$'
      responses:
        '200':
          description: Event found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Event'
        '404':
          description: Event not found
```

#### 15.1.2 Route Management

```yaml
  /v1/routes:
    get:
      summary: List routes
      description: List all configured routes
      tags:
        - Routes
      parameters:
        - name: prefix
          in: query
          description: Filter by name prefix
          schema:
            type: string
        - name: status
          in: query
          description: Filter by status
          schema:
            type: string
            enum: [active, paused, error]
      responses:
        '200':
          description: Route list
          content:
            application/json:
              schema:
                type: object
                properties:
                  routes:
                    type: array
                    items:
                      $ref: '#/components/schemas/Route'
                  total:
                    type: integer
                  page:
                    type: integer

    post:
      summary: Create route
      description: Create a new routing rule
      tags:
        - Routes
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/RouteCreateRequest'
      responses:
        '201':
          description: Route created
          headers:
            Location:
              description: URL of created route
              schema:
                type: string

  /v1/routes/{route_id}:
    get:
      summary: Get route details
      tags:
        - Routes
      parameters:
        - name: route_id
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Route details
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Route'

    put:
      summary: Update route
      tags:
        - Routes
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/RouteUpdateRequest'
      responses:
        '200':
          description: Route updated

    delete:
      summary: Delete route
      tags:
        - Routes
      responses:
        '204':
          description: Route deleted

    post:
      summary: Route actions
      description: Perform actions on a route (pause, resume, test)
      tags:
        - Routes
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                action:
                  type: string
                  enum: [pause, resume, test, reload]
      responses:
        '200':
          description: Action completed
```

#### 15.1.3 Stream Management

```yaml
  /v1/streams:
    get:
      summary: List streams
      tags:
        - Streams
      parameters:
        - name: source_type
          in: query
          schema:
            type: string
            enum: [route, kafka, nats, file, generator]
      responses:
        '200':
          description: Stream list
          content:
            application/json:
              schema:
                type: object
                properties:
                  streams:
                    type: array
                    items:
                      $ref: '#/components/schemas/Stream'

    post:
      summary: Create stream
      tags:
        - Streams
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/StreamCreateRequest'
      responses:
        '201':
          description: Stream created

  /v1/streams/{stream_id}/stats:
    get:
      summary: Get stream statistics
      tags:
        - Streams
      responses:
        '200':
          description: Stream statistics
          content:
            application/json:
              schema:
                type: object
                properties:
                  events_in:
                    type: integer
                  events_out:
                    type: integer
                  bytes_processed:
                    type: integer
                  avg_latency_ms:
                    type: number
                  uptime_seconds:
                    type: integer
                  status:
                    type: string
                    enum: [running, paused, error, stopped]
```

#### 15.1.4 Schema Registry

```yaml
  /v1/schemas:
    get:
      summary: List schemas
      tags:
        - Schemas
      parameters:
        - name: type
          in: query
          schema:
            type: string
            enum: [json, avro, protobuf]
      responses:
        '200':
          description: Schema list

    post:
      summary: Register schema
      tags:
        - Schemas
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/SchemaRegistration'
      responses:
        '201':
          description: Schema registered

  /v1/schemas/{schema_id}/versions:
    get:
      summary: List schema versions
      tags:
        - Schemas
      responses:
        '200':
          description: Version list

  /v1/schemas/{schema_id}/versions/{version}:
    get:
      summary: Get specific schema version
      tags:
        - Schemas
      responses:
        '200':
          description: Schema document
          content:
            application/json:
              schema:
                type: object

  /v1/schemas/validate:
    post:
      summary: Validate event against schema
      tags:
        - Schemas
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                event:
                  $ref: '#/components/schemas/Event'
                schema_id:
                  type: string
      responses:
        '200':
          description: Validation result
          content:
            application/json:
              schema:
                type: object
                properties:
                  valid:
                    type: boolean
                  errors:
                    type: array
                    items:
                      type: object
                      properties:
                        field:
                          type: string
                        message:
                          type: string
```

#### 15.1.5 Consumer Groups

```yaml
  /v1/consumers:
    get:
      summary: List consumer groups
      tags:
        - Consumers
      responses:
        '200':
          description: Consumer list

    post:
      summary: Create consumer group
      tags:
        - Consumers
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/ConsumerGroup'
      responses:
        '201':
          description: Consumer group created

  /v1/consumers/{group_id}/offsets:
    get:
      summary: Get consumer offsets
      tags:
        - Consumers
      responses:
        '200':
          description: Offset information
          content:
            application/json:
              schema:
                type: object
                properties:
                  partitions:
                    type: array
                    items:
                      type: object
                      properties:
                        partition:
                          type: integer
                        current_offset:
                          type: integer
                        committed_offset:
                          type: integer
                        lag:
                          type: integer

    post:
      summary: Commit offsets
      tags:
        - Consumers
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                offsets:
                  type: array
                  items:
                    type: object
                    properties:
                      partition:
                        type: integer
                      offset:
                        type: integer
      responses:
        '200':
          description: Offsets committed
```

### 15.2 WebSocket API

```yaml
  /v1/ws/subscribe:
    get:
      summary: WebSocket event subscription
      description: |
        Real-time event subscription via WebSocket.
        Supports filtering by event types, sources, and predicates.
      tags:
        - WebSocket
      parameters:
        - name: types
          in: query
          description: Comma-separated event types
          schema:
            type: string
        - name: sources
          in: query
          description: Comma-separated sources
          schema:
            type: string
      responses:
        '101':
          description: WebSocket upgrade successful

  /v1/ws/admin:
    get:
      summary: WebSocket admin stream
      description: Real-time metrics and control events
      tags:
        - WebSocket
      responses:
        '101':
          description: WebSocket upgrade successful
```

---

## 16. Deployment Patterns

### 16.1 Single Node Development

```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  eventra:
    image: eventra/eventra:latest
    ports:
      - "8080:8080"  # Admin
      - "8081:8081"  # Ingest
    environment:
      EVENTRA_LOG_LEVEL: debug
      EVENTRA_STORAGE_TYPE: memory
    volumes:
      - ./config.dev.yaml:/etc/eventra/config.yaml:ro
    command: ["serve", "--config", "/etc/eventra/config.yaml"]

  eventra-ui:
    image: eventra/eventra-ui:latest
    ports:
      - "3000:3000"
    environment:
      EVENTRA_API_URL: http://eventra:8080
    depends_on:
      - eventra
```

### 16.2 High Availability Production

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  eventra-1:
    image: eventra/eventra:${EVENTRA_VERSION}
    hostname: eventra-1
    environment:
      EVENTRA_NODE_ID: eventra-1
      EVENTRA_CLUSTER_ENABLED: "true"
      EVENTRA_CLUSTER_SEEDS: "eventra-2,eventra-3"
      EVENTRA_REPLICATION_FACTOR: 3
    volumes:
      - eventra-1-data:/data
      - ./config.prod.yaml:/etc/eventra/config.yaml:ro
    networks:
      - eventra-cluster
    healthcheck:
      test: ["CMD", "eventra", "health"]
      interval: 10s
      timeout: 5s
      retries: 3

  eventra-2:
    image: eventra/eventra:${EVENTRA_VERSION}
    hostname: eventra-2
    environment:
      EVENTRA_NODE_ID: eventra-2
      EVENTRA_CLUSTER_ENABLED: "true"
      EVENTRA_CLUSTER_SEEDS: "eventra-1,eventra-3"
    volumes:
      - eventra-2-data:/data
      - ./config.prod.yaml:/etc/eventra/config.yaml:ro
    networks:
      - eventra-cluster

  eventra-3:
    image: eventra/eventra:${EVENTRA_VERSION}
    hostname: eventra-3
    environment:
      EVENTRA_NODE_ID: eventra-3
      EVENTRA_CLUSTER_ENABLED: "true"
      EVENTRA_CLUSTER_SEEDS: "eventra-1,eventra-2"
    volumes:
      - eventra-3-data:/data
      - ./config.prod.yaml:/etc/eventra/config.yaml:ro
    networks:
      - eventra-cluster

  haproxy:
    image: haproxy:2.8
    ports:
      - "80:80"
      - "443:443"
      - "8080:8080"
    volumes:
      - ./haproxy.cfg:/usr/local/etc/haproxy/haproxy.cfg:ro
      - ./certs:/certs:ro
    depends_on:
      - eventra-1
      - eventra-2
      - eventra-3

volumes:
  eventra-1-data:
  eventra-2-data:
  eventra-3-data:

networks:
  eventra-cluster:
    driver: bridge
```

### 16.3 Kubernetes Deployment

```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: eventra
  labels:
    app.kubernetes.io/name: eventra

---
# k8s/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: eventra-config
  namespace: eventra
data:
  config.yaml: |
    server:
      ingest:
        http:
          port: 8081
          max_body_size: 1048576  # 1MB
      admin:
        port: 8080
    
    cluster:
      enabled: true
      replication_factor: 3
      min_isr: 2
    
    routing:
      max_queue_depth: 100000
      default_timeout: 30s
    
    storage:
      type: persistent
      path: /data
    
    observability:
      enabled: true
      service_name: eventra
      otlp_endpoint: otel-collector.monitoring:4317

---
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: eventra-headless
  namespace: eventra
spec:
  clusterIP: None
  selector:
    app.kubernetes.io/name: eventra
  ports:
    - name: ingest
      port: 8081
      targetPort: 8081
    - name: admin
      port: 8080
      targetPort: 8080
    - name: cluster
      port: 9090
      targetPort: 9090

---
# k8s/service-ingest.yaml
apiVersion: v1
kind: Service
metadata:
  name: eventra-ingest
  namespace: eventra
  annotations:
    prometheus.io/scrape: "true"
    prometheus.io/port: "9090"
spec:
  type: LoadBalancer
  selector:
    app.kubernetes.io/name: eventra
  ports:
    - name: ingest
      port: 80
      targetPort: 8081
    - name: admin
      port: 8080
      targetPort: 8080

---
# k8s/statefulset.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: eventra
  namespace: eventra
spec:
  serviceName: eventra-headless
  replicas: 3
  podManagementPolicy: Parallel
  selector:
    matchLabels:
      app.kubernetes.io/name: eventra
  template:
    metadata:
      labels:
        app.kubernetes.io/name: eventra
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "9090"
    spec:
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
            - weight: 100
              podAffinityTerm:
                labelSelector:
                  matchExpressions:
                    - key: app.kubernetes.io/name
                      operator: In
                      values:
                        - eventra
                topologyKey: kubernetes.io/hostname
      containers:
        - name: eventra
          image: eventra/eventra:v1.0.0
          imagePullPolicy: IfNotPresent
          ports:
            - containerPort: 8080
              name: admin
            - containerPort: 8081
              name: ingest
            - containerPort: 9090
              name: metrics
          env:
            - name: EVENTRA_NODE_ID
              valueFrom:
                fieldRef:
                  fieldPath: metadata.name
            - name: EVENTRA_CLUSTER_ENABLED
              value: "true"
            - name: EVENTRA_CLUSTER_SEEDS
              value: "eventra-0.eventra-headless,eventra-1.eventra-headless,eventra-2.eventra-headless"
            - name: RUST_LOG
              value: info
          volumeMounts:
            - name: config
              mountPath: /etc/eventra
              readOnly: true
            - name: data
              mountPath: /data
          livenessProbe:
            httpGet:
              path: /health/live
              port: 8080
            initialDelaySeconds: 10
            periodSeconds: 10
          readinessProbe:
            httpGet:
              path: /health/ready
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 5
          resources:
            requests:
              memory: "512Mi"
              cpu: "500m"
            limits:
              memory: "2Gi"
              cpu: "2000m"
      volumes:
        - name: config
          configMap:
            name: eventra-config
  volumeClaimTemplates:
    - metadata:
        name: data
      spec:
        accessModes: ["ReadWriteOnce"]
        storageClassName: fast-ssd
        resources:
          requests:
            storage: 100Gi

---
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: eventra-hpa
  namespace: eventra
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: StatefulSet
    name: eventra
  minReplicas: 3
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
    - type: Pods
      pods:
        metric:
          name: eventra_queue_depth
        target:
          type: AverageValue
          averageValue: "1000"
  behavior:
    scaleUp:
      stabilizationWindowSeconds: 60
      policies:
        - type: Percent
          value: 100
          periodSeconds: 60
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
        - type: Percent
          value: 10
          periodSeconds: 60
```

---

## 17. Operational Procedures

### 17.1 Installation

```bash
#!/bin/bash
# install.sh - Eventra installation script

set -euo pipefail

VERSION="${1:-latest}"
INSTALL_DIR="${2:-/usr/local/bin}"
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

# Map architecture names
case "$ARCH" in
    x86_64) ARCH="amd64" ;;
    aarch64) ARCH="arm64" ;;
    arm64) ARCH="arm64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Map OS names
case "$OS" in
    linux) OS="linux" ;;
    darwin) OS="macos" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Download
URL="https://github.com/eventra/eventra/releases/download/${VERSION}/eventra-${VERSION}-${OS}-${ARCH}.tar.gz"
echo "Downloading Eventra ${VERSION} for ${OS}/${ARCH}..."
curl -fsSL "$URL" -o /tmp/eventra.tar.gz

# Install
echo "Installing to ${INSTALL_DIR}..."
tar -xzf /tmp/eventra.tar.gz -C /tmp
sudo mv /tmp/eventra "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/eventra"

# Verify
if command -v eventra &> /dev/null; then
    echo "Eventra installed successfully!"
    eventra --version
else
    echo "Installation complete. Add ${INSTALL_DIR} to your PATH."
fi

# Cleanup
rm -f /tmp/eventra.tar.gz
```

### 17.2 Upgrade Procedures

```bash
#!/bin/bash
# upgrade.sh - Rolling upgrade procedure

VERSION="$1"
NAMESPACE="${2:-eventra}"

# Pre-upgrade checks
echo "Running pre-upgrade checks..."
kubectl exec -n "$NAMESPACE" eventra-0 -- eventra health
kubectl exec -n "$NAMESPACE" eventra-1 -- eventra health
kubectl exec -n "$NAMESPACE" eventra-2 -- eventra health

# Check lag
echo "Checking consumer lag..."
MAX_LAG=$(kubectl exec -n "$NAMESPACE" eventra-0 -- eventra consumer lag --max)
if [ "$MAX_LAG" -gt 1000 ]; then
    echo "Warning: High consumer lag detected ($MAX_LAG)"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Rolling upgrade
echo "Starting rolling upgrade to version $VERSION..."

for i in 2 1 0; do
    POD="eventra-$i"
    echo "Upgrading $POD..."
    
    # Cordon node
    kubectl exec -n "$NAMESPACE" "$POD" -- eventra admin cordon
    
    # Wait for traffic to drain
    echo "Waiting for traffic to drain..."
    sleep 30
    
    # Update image
    kubectl set image -n "$NAMESPACE" statefulset/eventra \
        eventra="eventra/eventra:$VERSION"
    
    # Wait for rollout
    kubectl rollout status -n "$NAMESPACE" statefulset/eventra
    
    # Verify
    kubectl exec -n "$NAMESPACE" "$POD" -- eventra health
    
    echo "$POD upgraded successfully"
done

echo "Upgrade complete!"
echo "Verifying cluster health..."
kubectl exec -n "$NAMESPACE" eventra-0 -- eventra cluster status
```

### 17.3 Backup and Restore

```bash
#!/bin/bash
# backup.sh - Backup procedure

BACKUP_DIR="${1:-/backup/eventra}"
RETENTION_DAYS="${2:-30}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_PATH="$BACKUP_DIR/eventra_backup_$TIMESTAMP"

echo "Starting backup to $BACKUP_PATH..."

# Create backup directory
mkdir -p "$BACKUP_PATH"

# Export routes
echo "Exporting routes..."
eventra route list --format json > "$BACKUP_PATH/routes.json"

# Export streams
echo "Exporting streams..."
eventra stream list --format json > "$BACKUP_PATH/streams.json"

# Export schemas
echo "Exporting schemas..."
eventra schema list --format json > "$BACKUP_PATH/schemas.json"

# Export configuration
echo "Exporting configuration..."
eventra config export > "$BACKUP_PATH/config.yaml"

# Backup data directory (if persistent storage)
echo "Backing up data directory..."
if [ -d "/data/eventra" ]; then
    tar -czf "$BACKUP_PATH/data.tar.gz" -C / data/eventra
fi

# Create manifest
cat > "$BACKUP_PATH/manifest.json" <<EOF
{
  "version": "1.0",
  "timestamp": "$TIMESTAMP",
  "files": [
    "routes.json",
    "streams.json",
    "schemas.json",
    "config.yaml",
    "data.tar.gz"
  ]
}
EOF

# Compress backup
tar -czf "$BACKUP_PATH.tar.gz" -C "$BACKUP_DIR" "eventra_backup_$TIMESTAMP"
rm -rf "$BACKUP_PATH"

# Clean up old backups
echo "Cleaning up backups older than $RETENTION_DAYS days..."
find "$BACKUP_DIR" -name "eventra_backup_*.tar.gz" -mtime +$RETENTION_DAYS -delete

echo "Backup complete: $BACKUP_PATH.tar.gz"
```

```bash
#!/bin/bash
# restore.sh - Restore procedure

BACKUP_FILE="$1"

echo "Restoring from $BACKUP_FILE..."

# Extract backup
TMPDIR=$(mktemp -d)
tar -xzf "$BACKUP_FILE" -C "$TMPDIR"

BACKUP_DIR=$(find "$TMPDIR" -type d -name "eventra_backup_*" | head -1)

# Verify manifest
if [ ! -f "$BACKUP_DIR/manifest.json" ]; then
    echo "Error: Invalid backup (no manifest found)"
    exit 1
fi

# Stop Eventra (if running)
echo "Stopping Eventra..."
systemctl stop eventra || true

# Restore data (if included)
if [ -f "$BACKUP_DIR/data.tar.gz" ]; then
    echo "Restoring data..."
    tar -xzf "$BACKUP_DIR/data.tar.gz" -C /
fi

# Start Eventra
echo "Starting Eventra..."
systemctl start eventra
sleep 5

# Restore configuration
echo "Restoring configuration..."
if [ -f "$BACKUP_DIR/config.yaml" ]; then
    eventra config import "$BACKUP_DIR/config.yaml"
fi

# Restore schemas
echo "Restoring schemas..."
if [ -f "$BACKUP_DIR/schemas.json" ]; then
    eventra schema import "$BACKUP_DIR/schemas.json"
fi

# Restore routes
echo "Restoring routes..."
if [ -f "$BACKUP_DIR/routes.json" ]; then
    eventra route import "$BACKUP_DIR/routes.json"
fi

# Restore streams
echo "Restoring streams..."
if [ -f "$BACKUP_DIR/streams.json" ]; then
    eventra stream import "$BACKUP_DIR/streams.json"
fi

# Cleanup
rm -rf "$TMPDIR"

echo "Restore complete!"
echo "Verify with: eventra health"
```

### 17.4 Monitoring Setup

```yaml
# prometheus-rules.yaml
groups:
  - name: eventra
    rules:
      - alert: EventraHighErrorRate
        expr: rate(eventra_events_failed_total[5m]) / rate(eventra_events_ingested_total[5m]) > 0.1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High error rate in Eventra"
          description: "Error rate is {{ $value | humanizePercentage }} for {{ $labels.instance }}"

      - alert: EventraHighLatency
        expr: histogram_quantile(0.99, rate(eventra_processing_latency_seconds_bucket[5m])) > 0.1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High processing latency in Eventra"
          description: "p99 latency is {{ $value }}s for {{ $labels.instance }}"

      - alert: EventraQueueDepth
        expr: eventra_queue_depth > 10000
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Eventra queue depth high"
          description: "Queue depth is {{ $value }} for {{ $labels.queue }}"

      - alert: EventraDown
        expr: up{job="eventra"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Eventra instance down"
          description: "Instance {{ $labels.instance }} is down"

      - alert: EventraReplicationLag
        expr: eventra_replication_lag > 10000
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Eventra replication lag high"
          description: "Replication lag is {{ $value }} events"

---
# grafana-dashboard.json (excerpt)
{
  "dashboard": {
    "title": "Eventra Overview",
    "panels": [
      {
        "title": "Events Ingested",
        "type": "stat",
        "targets": [
          {
            "expr": "rate(eventra_events_ingested_total[1m])",
            "legendFormat": "{{ source }}"
          }
        ]
      },
      {
        "title": "Processing Latency",
        "type": "heatmap",
        "targets": [
          {
            "expr": "rate(eventra_processing_latency_seconds_bucket[1m])"
          }
        ]
      },
      {
        "title": "Queue Depth",
        "type": "timeseries",
        "targets": [
          {
            "expr": "eventra_queue_depth",
            "legendFormat": "{{ queue }}"
          }
        ]
      }
    ]
  }
}
```

---

## 18. Troubleshooting Guide

### 18.1 Common Issues

#### 18.1.1 High Memory Usage

**Symptoms**: OOM kills, high memory in metrics

**Diagnosis**:
```bash
eventra metrics --type memory
eventra debug heap-profile
eventra stats buffers
```

**Solutions**:
1. Reduce buffer sizes in config
2. Enable backpressure
3. Add more nodes to distribute load
4. Check for memory leaks in custom processors

#### 18.1.2 High Latency

**Symptoms**: p99 latency > 100ms

**Diagnosis**:
```bash
eventra trace --duration 60s
eventra profile cpu --duration 30s
eventra stats queue-depths
```

**Solutions**:
1. Enable request coalescing
2. Tune batch sizes
3. Add more processing workers
4. Check downstream service health

#### 18.1.3 Consumer Lag

**Symptoms**: Growing queue depth, delayed processing

**Diagnosis**:
```bash
eventra consumer lag
eventra stream stats --name <stream>
eventra route metrics --name <route>
```

**Solutions**:
1. Scale consumer groups
2. Increase processing parallelism
3. Optimize slow transformations
4. Check for blocking I/O

#### 18.1.4 Connection Issues

**Symptoms**: Connection refused, TLS errors

**Diagnosis**:
```bash
eventra health --verbose
eventra netstat
eventra tls verify
```

**Solutions**:
1. Check firewall rules
2. Verify TLS certificates
3. Review connection limits
4. Check DNS resolution

### 18.2 Debug Mode

```bash
# Enable debug logging
eventra serve --log-level debug

# Enable trace logging (very verbose)
eventra serve --log-level trace

# Enable pprof endpoints
eventra serve --pprof-bind localhost:6060

# Capture diagnostic snapshot
eventra debug snapshot --output /tmp/debug.tar.gz
```

---

## 19. Performance Tuning

### 19.1 Kernel Tuning

```bash
# /etc/sysctl.d/99-eventra.conf

# Network tuning
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 134217728
net.ipv4.tcp_wmem = 4096 65536 134217728
net.core.netdev_max_backlog = 300000

# File descriptor limits
fs.file-max = 2097152
fs.nr_open = 2097152

# Virtual memory
vm.swappiness = 10
vm.dirty_ratio = 40
vm.dirty_background_ratio = 10

# Apply
sysctl -p /etc/sysctl.d/99-eventra.conf
```

### 19.2 Systemd Service

```ini
# /etc/systemd/system/eventra.service
[Unit]
Description=Eventra Event Router
After=network.target

[Service]
Type=simple
User=eventra
Group=eventra
ExecStart=/usr/local/bin/eventra serve --config /etc/eventra/config.yaml
Restart=always
RestartSec=5

# Resource limits
LimitNOFILE=1048576
LimitNPROC=1048576

# CPU affinity (optional)
CPUAffinity=0-7

# Memory limits
MemoryMax=4G
MemorySwapMax=0

# Sandboxing
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/eventra /var/log/eventra

[Install]
WantedBy=multi-user.target
```

### 19.3 Benchmarking

```bash
# Install benchmarking tool
eventra-benchmark --version

# Basic throughput test
eventra-benchmark throughput \
    --target localhost:8081 \
    --duration 60s \
    --concurrency 100 \
    --events-per-request 10

# Latency distribution test
eventra-benchmark latency \
    --target localhost:8081 \
    --duration 300s \
    --percentiles 50,99,99.9

# Load test with custom events
eventra-benchmark load \
    --target localhost:8081 \
    --rate 10000 \
    --duration 600s \
    --event-file ./test-events.jsonl

# Comparative benchmark
eventra-benchmark compare \
    --targets eventra=localhost:8081,kafka=localhost:9092 \
    --duration 60s
```

---

## 20. Security Hardening

### 20.1 Network Security

```yaml
# iptables rules for Eventra
security:
  firewall:
    # Default deny
    policy: deny
    
    # Allow rules
    allow:
      # Health checks from load balancer
      - port: 8080
        sources:
          - 10.0.0.0/8  # Internal network
        
      # Ingestion from application servers
      - port: 8081
        sources:
          - 10.0.1.0/24  # App servers
        
      # Cluster communication
      - port: 9090
        sources:
          - 10.0.0.0/8  # Cluster nodes
    
    # Rate limiting
    rate_limit:
      port: 8081
      requests_per_second: 10000
      burst: 50000
```

### 20.2 Audit Logging

```yaml
audit:
  enabled: true
  
  # Events to audit
  events:
    - auth.failure
    - auth.success
    - config.change
    - route.create
    - route.delete
    - stream.create
    - stream.delete
    - schema.update
  
  # Output
  output:
    type: file
    path: /var/log/eventra/audit.log
    rotation:
      size: 100MB
      count: 10
  
  # Forward to SIEM (optional)
  forward:
    enabled: true
    url: https://siem.company.com/api/events
    headers:
      Authorization: Bearer ${SIEM_TOKEN}
```

---

## 21. Multi-Tenancy

### 21.1 Tenant Isolation

```yaml
multi_tenancy:
  enabled: true
  
  # Isolation level
  isolation: namespace  # namespace | cluster | hybrid
  
  # Tenant configuration
  tenants:
    - id: acme-corp
      name: "Acme Corporation"
      quotas:
        events_per_second: 10000
        storage_gb: 500
        streams: 100
        routes: 50
      resources:
        cpu: "2"
        memory: "4Gi"
      
    - id: globex
      name: "Globex Inc"
      quotas:
        events_per_second: 5000
        storage_gb: 200
        streams: 50
        routes: 25
      resources:
        cpu: "1"
        memory: "2Gi"
```

### 21.2 Tenant API

```bash
# Create tenant
eventra tenant create \
    --id acme-corp \
    --name "Acme Corporation" \
    --quota events_per_second=10000 \
    --quota storage_gb=500

# List tenants
eventra tenant list

# Get tenant stats
eventra tenant stats --id acme-corp

# Update quotas
eventra tenant update \
    --id acme-corp \
    --quota events_per_second=20000

# Delete tenant
eventra tenant delete --id acme-corp
```

---

## 22. Migration Guides

### 22.1 From NATS

```yaml
# Migration configuration
migration:
  from:
    type: nats
    url: nats://localhost:4222
    subjects:
      - "orders.*"
      - "users.*"
  
  to:
    type: eventra
    url: http://eventra:8081
  
  options:
    dual_write: true
    duration: 30d
    rollback_window: 7d
```

### 22.2 From Kafka

```yaml
migration:
  from:
    type: kafka
    brokers:
      - kafka-1:9092
      - kafka-2:9092
    topics:
      - orders
      - users
      - inventory
    consumer_group: eventra-migration
  
  to:
    type: eventra
    url: http://eventra:8081
  
  mapping:
    - from_topic: orders
      to_routes:
        - order-processing
        - order-analytics
    - from_topic: users
      to_routes:
        - user-events
  
  options:
    offset_reset: earliest
    batch_size: 1000
```

---

## 23. Ecosystem Integrations

### 23.1 OpenTelemetry Collector

```yaml
# otel-collector-config.yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
      http:
        endpoint: 0.0.0.0:4318

processors:
  batch:
    timeout: 1s
    send_batch_size: 1024

exporters:
  eventra:
    endpoint: http://eventra:8081
    headers:
      Authorization: ${EVENTRA_API_KEY}
    retry_on_failure:
      enabled: true
      initial_interval: 5s
      max_interval: 30s
      max_elapsed_time: 300s

service:
  pipelines:
    traces:
      receivers: [otlp]
      processors: [batch]
      exporters: [eventra]
    metrics:
      receivers: [otlp]
      processors: [batch]
      exporters: [eventra]
    logs:
      receivers: [otlp]
      processors: [batch]
      exporters: [eventra]
```

### 23.2 Terraform Provider

```hcl
# main.tf
terraform {
  required_providers {
    eventra = {
      source = "eventra/eventra"
      version = "~> 1.0"
    }
  }
}

provider "eventra" {
  endpoint = "http://eventra:8080"
  api_key  = var.eventra_api_key
}

# Routes
resource "eventra_route" "orders_to_warehouse" {
  name     = "orders-to-warehouse"
  priority = 100
  
  match {
    types   = ["order.created", "order.updated"]
    sources = ["order-service"]
  }
  
  action {
    target  = "http://warehouse-api:8080/events"
    timeout = "30s"
    
    retry {
      attempts     = 3
      backoff      = "exponential"
      max_delay    = "60s"
    }
  }
}

# Streams
resource "eventra_stream" "analytics" {
  name = "analytics-stream"
  
  source {
    type = "route"
    config = {
      route = eventra_route.orders_to_warehouse.name
    }
  }
  
  processor {
    type = "filter"
    config = {
      predicate = "data.total > 100"
    }
  }
  
  output {
    type = "kafka"
    config = {
      topic = "high-value-orders"
    }
  }
}

# Schemas
resource "eventra_schema" "order_event" {
  name        = "order-event"
  type        = "json"
  version     = 1
  compatibility = "backward"
  
  schema = jsonencode({
    type = "object"
    required = ["order_id", "customer_id", "items"]
    properties = {
      order_id = {
        type = "string"
        format = "uuid"
      }
      customer_id = {
        type = "string"
      }
      items = {
        type = "array"
        items = {
          type = "object"
          properties = {
            sku = { type = "string" }
            quantity = { type = "integer" }
            price = { type = "number" }
          }
        }
      }
    }
  })
}
```

---

## 24. Glossary

| Term | Definition |
|------|------------|
| **Event** | A discrete unit of data representing something that happened |
| **Stream** | A continuous flow of events |
| **Route** | A rule matching events to destinations |
| **Processor** | A component transforming or analyzing events |
| **Sink** | A destination for events |
| **Source** | An origin of events |
| **Consumer Group** | A set of consumers sharing workload |
| **Backpressure** | Flow control mechanism |
| **Window** | A bounded set of events in time |
| **Aggregation** | Combining multiple events into a result |
| **Join** | Correlating events from multiple streams |
| **Schema** | A definition of event structure |
| **ULID** | Universally Unique Lexicographically Sortable Identifier |
| **OTLP** | OpenTelemetry Protocol |
| **OCC** | Optimistic Concurrency Control |

---

## 25. Appendices

### 25.1 License

Eventra is licensed under the Apache License 2.0.

```
Copyright 2026 Eventra Project Authors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

### 25.2 Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines.

### 25.3 Changelog

See [CHANGELOG.md](./CHANGELOG.md) for version history.

### 25.4 Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2026-04-05 | Eventra Team | Initial comprehensive specification |

---

*This specification is a living document. As Eventra evolves, this document will be updated to reflect the current state and future direction.*

*For questions or clarifications, please open an issue on the Eventra repository or contact the maintainers.*

