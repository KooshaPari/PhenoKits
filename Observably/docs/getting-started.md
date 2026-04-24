# Observably Getting Started

## Why Observably?

Observably provides production-grade observability primitives: distributed tracing (OpenTelemetry), structured logging, metrics emission, and PII filtering. As a bridge across the Phenotype ecosystem, Observably captures events from Sidekick dispatch, Eidolon automation, and Stashly state changes—then enriches them with traces and context for debugging, compliance, and optimization.

**Key problems Observably solves:**

- **Distributed tracing** — Link dispatch → automation → caching into a single causal chain (OTEL-based)
- **PII-safe logging** — Automatically scrub sensitive data (credentials, tokens, PII) before emitting logs
- **Cross-collection observability** — Subscribe to events from any collection and emit traces
- **Metrics and alerts** — Emit Prometheus metrics for SLAs, latency, error rates
- **Compliance audit trail** — Immutable trace records for regulatory requirements

## Install

Add the observability crates:

```bash
cargo add observably-tracing
cargo add observably-logging
cargo add observably-sentinel
```

Or in your `Cargo.toml`:

```toml
[dependencies]
observably-tracing = { path = "../../observably/crates/observably-tracing" }
observably-logging = { path = "../../observably/crates/observably-logging" }
observably-sentinel = { path = "../../observably/crates/observably-sentinel" }

tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
opentelemetry = "0.20"
phenotype-bus = { path = "../../phenotype-bus" }
```

## Quickstart (20 lines)

```rust
use observably_tracing::{init_tracer, start_span};
use observably_logging::{Logger, LogLevel};
use tracing::info;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing with OTEL exporter
    let tracer = init_tracer("my-service")?;

    // Create a span for this operation
    let span = start_span(&tracer, "user_action")?;

    // Structured logging
    let logger = Logger::new();
    logger.log(LogLevel::Info, "User logged in", None)?;

    // Emit trace
    info!("Operation completed");

    Ok(())
}
```

## Common Patterns

### Pattern 1: Distributed Tracing with Context

Trace a request through multiple systems: dispatch → automation → caching.

```rust
use observably_tracing::{start_span, with_parent};
use tracing::info;

let root_span = start_span(&tracer, "dispatch_request")?;

// Nested span for automation
let auto_span = with_parent(&tracer, &root_span, "trigger_automation")?;
info!(parent: &auto_span, "Taking screenshot");

// Nested span for caching
let cache_span = with_parent(&tracer, &root_span, "store_result")?;
info!(parent: &cache_span, "Cache hit");

// All three spans linked by common trace ID
println!("Trace ID: {}", root_span.trace_id());
```

### Pattern 2: PII Filtering for Compliance

Automatically scrub credentials and sensitive fields before logging.

```rust
use observably_logging::{Logger, LogLevel, PiiFilter};

let filter = PiiFilter::default()
    .add_pattern("password", "***")
    .add_pattern("api_key", "***");

let logger = Logger::new().with_pii_filter(filter);

// These are automatically sanitized
logger.log(LogLevel::Info, "User data: {\"email\":\"user@example.com\",\"password\":\"secret123\"}", None)?;
// Output: User data: {"email":"user@example.com","password":"***"}
```

### Pattern 3: Cross-Collection Event Tracing via phenotype-bus

Subscribe to events from other collections and emit traces.

```rust
use phenotype_bus::{Bus, Event};
use observably_tracing::start_span;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DispatchStarted {
    pub dispatch_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AutomationCompleted {
    pub dispatch_id: String,
}

impl Event for DispatchStarted {
    fn event_name(&self) -> &'static str { "DispatchStarted" }
}

impl Event for AutomationCompleted {
    fn event_name(&self) -> &'static str { "AutomationCompleted" }
}

// Subscribe to dispatch events
let dispatch_bus = Bus::<DispatchStarted>::new(100);
let mut rx = dispatch_bus.subscribe();

let tracer = init_tracer("observably-collector")?;

while let Ok(event) = rx.recv().await {
    // Start a span for this dispatch
    let span = start_span(&tracer, &format!("dispatch_{}", event.dispatch_id))?;

    // Subscribe to completion and link it
    let completion_bus = Bus::<AutomationCompleted>::new(100);
    let mut completion_rx = completion_bus.subscribe();

    if let Ok(completion) = completion_rx.recv().await {
        info!(parent: &span, "Automation completed: {:?}", completion);
    }
}
```

### Pattern 4: Metrics Emission (Prometheus-compatible)

Emit metrics for dispatch latency, cache hit rate, automation success.

```rust
use observably_sentinel::{Counter, Histogram};

// Count dispatch events
let dispatch_count = Counter::new("dispatch_total");
dispatch_count.inc()?;

// Measure automation latency (milliseconds)
let latency_histogram = Histogram::new("automation_latency_ms", vec![10.0, 50.0, 100.0, 500.0])?;
latency_histogram.observe(42.5)?;

// Track cache hit ratio
let cache_hits = Counter::new("cache_hits_total");
let cache_misses = Counter::new("cache_misses_total");
cache_hits.inc()?;

let hit_rate = cache_hits.value() as f64 
    / (cache_hits.value() + cache_misses.value()) as f64;
println!("Cache hit rate: {:.2}%", hit_rate * 100.0);
```

## Cross-Collection Integration

Observably is the observability hub for Phenotype collections via **phenotype-bus**:

- **Subscribes to**: Events from Sidekick (dispatch), Eidolon (automation), Stashly (cache/events)
- **Emits**: `TraceRecorded`, `MetricsEmitted` events
- **Consumed by**: Stashly (stores traces), Paginary (documents observed workflows)

See [phenotype-bus](../../phenotype-bus/README.md) for event patterns. Observably integrates with [Sidekick](../Sidekick/README.md) (traces dispatch decisions), [Eidolon](../Eidolon/README.md) (traces automation), [Stashly](../Stashly/README.md) (stores traces as events), and [Paginary](../Paginary/README.md) (documents observability patterns).

## Next Steps

- Explore [observably-tracing OTEL integration](./crates/observably-tracing/README.md)
- Read [PII filtering guide](./docs/PII_FILTERING.md)
- Review [metrics and alerting](./docs/METRICS.md)
- Check the [cross-collection demo](../../docs/collections/cross_collection_demo.md)
