# Cross-Collection Demo: End-to-End Workflow

## Scenario

This guide demonstrates how the five Phenotype collections coordinate to deliver a complete agent workflow:

1. **Sidekick** detects user status change → emits `UserStatusChanged` to phenotype-bus
2. **Eidolon** subscribes, triggers desktop screenshot automation
3. **Stashly** receives screenshot, caches it with TTL, stores event to event log
4. **Observably** subscribes, records distributed trace with PII filtering
5. **Sidekick** messaging notifies user with trace summary

All collections communicate via **phenotype-bus** — a minimal, type-safe pub/sub event broker.

## The Event Flow

```
┌──────────────────────────────────────────────────────────────────┐
│ USER EVENT: Status changes from "away" to "online"               │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
           ┌─────────────────────────────┐
           │  SIDEKICK: Presence         │
           │  Emits: UserStatusChanged   │
           │  {user_id, status}          │
           └────────────┬────────────────┘
                        │ phenotype-bus
                        ▼
           ┌─────────────────────────────┐
           │  EIDOLON: Automation        │
           │  Subscribes to status       │
           │  Triggers screenshot        │
           │  Emits: ScreenshotTaken     │
           │  {screenshot_path}          │
           └────────────┬────────────────┘
                        │ phenotype-bus
                        ▼
           ┌─────────────────────────────┐
           │  STASHLY: Persistence       │
           │  Subscribes to screenshot   │
           │  Caches with TTL (1h)       │
           │  Stores to event log        │
           │  Emits: ScreenshotCached    │
           │  {cache_key, ttl_secs}      │
           └────────────┬────────────────┘
                        │ phenotype-bus
                        ▼
           ┌─────────────────────────────┐
           │  OBSERVABLY: Tracing        │
           │  Subscribes to cache event  │
           │  Records OTEL trace         │
           │  PII filter on metadata     │
           │  Emits: TraceRecorded       │
           │  {trace_id, span_count}     │
           └────────────┬────────────────┘
                        │ phenotype-bus
                        ▼
           ┌─────────────────────────────┐
           │  SIDEKICK: Messaging        │
           │  Subscribes to trace event  │
           │  Sends user notification    │
           │  "Screenshot cached & traced"
           └─────────────────────────────┘
```

## Event Definitions

Each collection defines its domain events, implementing the `Event` trait:

```rust
// sidekick-presence: User status events
#[derive(Clone, Serialize, Deserialize)]
pub struct UserStatusChanged {
    pub user_id: String,
    pub status: String, // "online", "away", "focus", "inactive"
    pub timestamp: u64,
}

impl Event for UserStatusChanged {
    fn event_name(&self) -> &'static str {
        "UserStatusChanged"
    }
}

// eidolon-core: Automation events
#[derive(Clone, Serialize, Deserialize)]
pub struct ScreenshotTaken {
    pub user_id: String,
    pub screenshot_path: String,
    pub size_bytes: u64,
    pub timestamp: u64,
}

impl Event for ScreenshotTaken {
    fn event_name(&self) -> &'static str {
        "ScreenshotTaken"
    }
}

// stashly-cache: Storage events
#[derive(Clone, Serialize, Deserialize)]
pub struct ScreenshotCached {
    pub user_id: String,
    pub cache_key: String,
    pub ttl_secs: u64,
    pub size_bytes: u64,
    pub timestamp: u64,
}

impl Event for ScreenshotCached {
    fn event_name(&self) -> &'static str {
        "ScreenshotCached"
    }
}

// observably-tracing: Observability events
#[derive(Clone, Serialize, Deserialize)]
pub struct TraceRecorded {
    pub user_id: String,
    pub trace_id: String,
    pub span_count: usize,
    pub latency_ms: u64,
    pub timestamp: u64,
}

impl Event for TraceRecorded {
    fn event_name(&self) -> &'static str {
        "TraceRecorded"
    }
}

// sidekick-messaging: Notification events
#[derive(Clone, Serialize, Deserialize)]
pub struct NotificationSent {
    pub user_id: String,
    pub message: String,
    pub channel: String, // "imessage", "sms", "email"
    pub timestamp: u64,
}

impl Event for NotificationSent {
    fn event_name(&self) -> &'static str {
        "NotificationSent"
    }
}
```

## Runnable Example

Create this file at `crates/sidekick-dispatch/examples/cross_collection_demo.rs`:

```rust
//! Cross-Collection Demo: Sidekick → Eidolon → Stashly → Observably → Messaging
//!
//! This example demonstrates how Phenotype collections coordinate via phenotype-bus
//! to deliver an end-to-end workflow.
//!
//! Run with: cargo run --example cross_collection_demo

use phenotype_bus::{Bus, Event};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

// Event: User status changed
#[derive(Clone, Serialize, Deserialize)]
pub struct UserStatusChanged {
    pub user_id: String,
    pub status: String,
    pub timestamp: u64,
}

impl Event for UserStatusChanged {
    fn event_name(&self) -> &'static str {
        "UserStatusChanged"
    }
}

// Event: Screenshot taken
#[derive(Clone, Serialize, Deserialize)]
pub struct ScreenshotTaken {
    pub user_id: String,
    pub path: String,
    pub size_bytes: u64,
    pub timestamp: u64,
}

impl Event for ScreenshotTaken {
    fn event_name(&self) -> &'static str {
        "ScreenshotTaken"
    }
}

// Event: Screenshot cached
#[derive(Clone, Serialize, Deserialize)]
pub struct ScreenshotCached {
    pub user_id: String,
    pub cache_key: String,
    pub ttl_secs: u64,
    pub timestamp: u64,
}

impl Event for ScreenshotCached {
    fn event_name(&self) -> &'static str {
        "ScreenshotCached"
    }
}

// Event: Trace recorded
#[derive(Clone, Serialize, Deserialize)]
pub struct TraceRecorded {
    pub user_id: String,
    pub trace_id: String,
    pub span_count: usize,
    pub latency_ms: u64,
    pub timestamp: u64,
}

impl Event for TraceRecorded {
    fn event_name(&self) -> &'static str {
        "TraceRecorded"
    }
}

// Event: Notification sent
#[derive(Clone, Serialize, Deserialize)]
pub struct NotificationSent {
    pub user_id: String,
    pub message: String,
    pub timestamp: u64,
}

impl Event for NotificationSent {
    fn event_name(&self) -> &'static str {
        "NotificationSent"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Phenotype Cross-Collection Demo ===\n");

    // Create buses for each event type (in practice, shared application state)
    let status_bus = Arc::new(Bus::<UserStatusChanged>::new(10));
    let screenshot_bus = Arc::new(Bus::<ScreenshotTaken>::new(10));
    let cache_bus = Arc::new(Bus::<ScreenshotCached>::new(10));
    let trace_bus = Arc::new(Bus::<TraceRecorded>::new(10));
    let notify_bus = Arc::new(Bus::<NotificationSent>::new(10));

    // Spawn Eidolon subscriber: listen for status changes, take screenshots
    let eidolon_screenshot_bus = screenshot_bus.clone();
    let eidolon_status_bus = status_bus.clone();
    tokio::spawn({
        async move {
            let mut rx = eidolon_status_bus.subscribe();
            while let Ok(event) = rx.recv().await {
                println!("[EIDOLON] Received status change: {} → {}", 
                    event.user_id, event.status);
                
                // Simulate taking screenshot
                sleep(Duration::from_millis(50)).await;
                
                let screenshot = ScreenshotTaken {
                    user_id: event.user_id.clone(),
                    path: "/tmp/screenshot.png".into(),
                    size_bytes: 12345,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                };
                
                println!("[EIDOLON] Emitting: ScreenshotTaken ({})", screenshot.path);
                eidolon_screenshot_bus.publish(screenshot).await.ok();
            }
        }
    });

    // Spawn Stashly subscriber: listen for screenshots, cache them
    let stashly_cache_bus = cache_bus.clone();
    let stashly_screenshot_bus = screenshot_bus.clone();
    tokio::spawn({
        async move {
            let mut rx = stashly_screenshot_bus.subscribe();
            while let Ok(event) = rx.recv().await {
                println!("[STASHLY] Received screenshot: {} bytes from {}", 
                    event.size_bytes, event.user_id);
                
                // Simulate caching
                sleep(Duration::from_millis(30)).await;
                
                let cached = ScreenshotCached {
                    user_id: event.user_id.clone(),
                    cache_key: "screenshot-001".into(),
                    ttl_secs: 3600,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                };
                
                println!("[STASHLY] Emitting: ScreenshotCached (TTL: {}s)", cached.ttl_secs);
                stashly_cache_bus.publish(cached).await.ok();
            }
        }
    });

    // Spawn Observably subscriber: listen for cache events, record traces
    let observably_trace_bus = trace_bus.clone();
    let observably_cache_bus = cache_bus.clone();
    tokio::spawn({
        async move {
            let mut rx = observably_cache_bus.subscribe();
            while let Ok(event) = rx.recv().await {
                println!("[OBSERVABLY] Received cache event for user: {}", event.user_id);
                
                // Simulate trace recording
                sleep(Duration::from_millis(40)).await;
                
                let traced = TraceRecorded {
                    user_id: event.user_id.clone(),
                    trace_id: "trace-abc123".into(),
                    span_count: 5,
                    latency_ms: 120,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                };
                
                println!("[OBSERVABLY] Emitting: TraceRecorded (trace_id: {})", traced.trace_id);
                observably_trace_bus.publish(traced).await.ok();
            }
        }
    });

    // Spawn Sidekick messaging subscriber: listen for traces, send notifications
    let messaging_notify_bus = notify_bus.clone();
    let messaging_trace_bus = trace_bus.clone();
    tokio::spawn({
        async move {
            let mut rx = messaging_trace_bus.subscribe();
            while let Ok(event) = rx.recv().await {
                println!("[SIDEKICK] Received trace event: {}", event.trace_id);
                
                // Simulate sending notification
                sleep(Duration::from_millis(25)).await;
                
                let notification = NotificationSent {
                    user_id: event.user_id.clone(),
                    message: format!(
                        "Screenshot cached & traced. Trace ID: {} ({} spans, {} ms)",
                        event.trace_id, event.span_count, event.latency_ms
                    ),
                    timestamp: chrono::Utc::now().timestamp() as u64,
                };
                
                println!("[SIDEKICK] Emitting: NotificationSent");
                println!("[SIDEKICK] Message: {}\n", notification.message);
                messaging_notify_bus.publish(notification).await.ok();
            }
        }
    });

    // Main: Emit initial status change event (triggers the whole workflow)
    println!("[MAIN] Publishing: UserStatusChanged (user-001 → online)\n");
    
    let status_event = UserStatusChanged {
        user_id: "user-001".into(),
        status: "online".into(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    };
    
    status_bus.publish(status_event).await?;

    // Wait for all handlers to finish
    sleep(Duration::from_secs(2)).await;

    println!("=== Workflow Complete ===\n");
    Ok(())
}
```

## Compilation and Execution

Add dependencies to `crates/sidekick-dispatch/Cargo.toml`:

```toml
[dependencies]
phenotype-bus = { path = "../../phenotype-bus" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
chrono = "0.4"
```

Run the demo:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/Sidekick
cargo run --example cross_collection_demo
```

Expected output:

```
=== Phenotype Cross-Collection Demo ===

[MAIN] Publishing: UserStatusChanged (user-001 → online)

[EIDOLON] Received status change: user-001 → online
[EIDOLON] Emitting: ScreenshotTaken (/tmp/screenshot.png)

[STASHLY] Received screenshot: 12345 bytes from user-001
[STASHLY] Emitting: ScreenshotCached (TTL: 3600s)

[OBSERVABLY] Received cache event for user: user-001
[OBSERVABLY] Emitting: TraceRecorded (trace_id: trace-abc123)

[SIDEKICK] Received trace event: trace-abc123
[SIDEKICK] Emitting: NotificationSent
[SIDEKICK] Message: Screenshot cached & traced. Trace ID: trace-abc123 (5 spans, 120 ms)

=== Workflow Complete ===
```

## Key Takeaways

1. **Loose coupling** — Collections only depend on phenotype-bus, not each other
2. **Type-safe events** — Each event implements the `Event` trait; compile-time guarantees
3. **Composability** — Add/remove handlers without changing event producers
4. **Auditability** — Every step is logged and traceable
5. **Real-world integration** — Replace `sleep(Duration::from_millis(X))` with real I/O (screenshots, caching, tracing)

## Next Steps

- Run the demo locally
- Add error handling (try `dispatcher.dispatch().map_err(...)`)
- Extend with your own events
- Integrate with actual Eidolon, Stashly, Observably implementations
- See [Sidekick Getting Started](../Sidekick/docs/getting-started.md) for more patterns
