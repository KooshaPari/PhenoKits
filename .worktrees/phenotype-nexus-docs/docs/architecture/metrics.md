# Service Mesh Metrics — phenotype-nexus

> Observability instrumentation for service mesh and directory operations

**Version**: 1.0 | **Status**: Active | **Last Updated**: 2026-04-04

## Overview

This document defines the metrics, benchmarks, SLOs/SLIs, and observability
practices for phenotype-nexus service mesh operations. phenotype-nexus provides
in-process service registry and discovery; these metrics ensure operators can
observe registration latencies, discovery throughput, and system health in
production deployments.

## Table of Contents

1. [Core Metrics Definitions](#core-metrics-definitions)
2. [Service Discovery Metrics](#service-discovery-metrics)
3. [Registration Health Metrics](#registration-health-metrics)
4. [Benchmark Methodology](#benchmark-methodology)
5. [SLO and SLI Definitions](#slo-and-sli-definitions)
6. [Observability Best Practices](#observability-best-practices)
7. [Alerting Thresholds](#alerting-thresholds)
8. [Dashboard Recommendations](#dashboard-recommendations)

---

## Core Metrics Definitions

### Latency Metrics

| Metric Name | Type | Description | Unit |
|-------------|------|-------------|------|
| `nexus.registry.register.latency` | Histogram | Time to register a service | milliseconds |
| `nexus.registry.deregister.latency` | Histogram | Time to deregister a service | milliseconds |
| `nexus.discovery.lookup.latency` | Histogram | Time to discover a service | milliseconds |
| `nexus.discovery.filter.latency` | Histogram | Time to filter services by tags | milliseconds |
| `nexus.balancer.select.latency` | Histogram | Time to select endpoint via load balancer | milliseconds |
| `nexus.health.check.latency` | Histogram | Time to perform health check | milliseconds |

#### Latency Percentiles

All latency histograms MUST support the following percentiles:

- **P50** (median): Typical response time
- **P90**: 90th percentile — detects outliers
- **P95**: 95th percentile — SLA boundary
- **P99**: 99th percentile — rare event detection
- **P99.9**: 99.9th percentile — extreme event detection

#### Target Latency Values

| Operation | P50 | P90 | P95 | P99 |
|-----------|-----|-----|-----|-----|
| Service Registration | < 1ms | < 5ms | < 10ms | < 50ms |
| Service Deregistration | < 0.5ms | < 2ms | < 5ms | < 20ms |
| Service Discovery | < 0.3ms | < 1ms | < 2ms | < 10ms |
| Tag-based Filtering | < 0.5ms | < 2ms | < 5ms | < 25ms |
| Load Balancer Selection | < 0.1ms | < 0.3ms | < 0.5ms | < 1ms |
| Health Check | < 5ms | < 15ms | < 25ms | < 100ms |

### Throughput Metrics

| Metric Name | Type | Description | Unit |
|-------------|------|-------------|------|
| `nexus.registry.register.total` | Counter | Total registration operations | count |
| `nexus.registry.deregister.total` | Counter | Total deregistration operations | count |
| `nexus.discovery.lookup.total` | Counter | Total discovery operations | count |
| `nexus.discovery.lookup.hits` | Counter | Successful discovery lookups | count |
| `nexus.discovery.lookup.misses` | Counter | Failed discovery lookups (service not found) | count |
| `nexus.balancer.select.total` | Counter | Total load balancer selections | count |

#### Throughput Targets

| Operation | Sustained Rate | Burst Capacity |
|-----------|----------------|---------------|
| Registrations/sec | 10,000 | 50,000 |
| Deregistrations/sec | 10,000 | 50,000 |
| Discoveries/sec | 100,000 | 500,000 |
| Load Balancer Selections/sec | 500,000 | 1,000,000 |

### Availability Metrics

| Metric Name | Type | Description | Unit |
|-------------|------|-------------|------|
| `nexus.registry.status` | Gauge | Registry availability (1 = up, 0 = down) | boolean |
| `nexus.registry.services.total` | Gauge | Current number of registered services | count |
| `nexus.registry.services.healthy` | Gauge | Number of healthy services | count |
| `nexus.registry.services.unhealthy` | Gauge | Number of unhealthy services | count |

---

## Service Discovery Metrics

### Discovery Operation Metrics

```rust
// Example: Recording discovery metrics
use phenotype_nexus::{Registry, metrics};

async fn discover_service() -> Result<Service> {
    let start = std::time::Instant::now();
    
    let service = registry
        .discover()
        .name("payment-gateway")
        .tags(["v2", "production"])
        .await?;
    
    let elapsed = start.elapsed().as_millis() as f64;
    metrics::histogram!("nexus.discovery.lookup.latency", elapsed);
    metrics::increment!("nexus.discovery.lookup.total");
    
    Ok(service)
}
```

### Discovery Success Rate

**Formula**: `lookup_hits / lookup_total * 100`

| Scenario | Target Success Rate |
|----------|--------------------|
| Service exists and healthy | 99.99% |
| Service exists but unhealthy | 95% (returns degraded) |
| Service does not exist | 100% (proper error) |

### Discovery Cache Metrics

When caching is enabled:

| Metric Name | Type | Description |
|-------------|------|-------------|
| `nexus.cache.hits` | Counter | Cache hit count |
| `nexus.cache.misses` | Counter | Cache miss count |
| `nexus.cache.evictions` | Counter | Cache eviction count |
| `nexus.cache.size` | Gauge | Current cache size |

**Target Cache Hit Rate**: > 80% for production workloads

---

## Registration Health Metrics

### Health State Transitions

```
                    ┌─────────────┐
    ┌───────────────│  Healthy    │◄──────────────┐
    │               └──────┬──────┘               │
    │                      │                      │
    │ health_check pass    │ health_check fail    │ recovery
    │                      │                      │
    │               ┌──────▼──────┐               │
    └──────────────►│  Unhealthy  │───────────────┘
                    └──────┬──────┘
                           │
                    TTL expired
                           │
                    ┌──────▼──────┐
                    │   Expired   │
                    └─────────────┘
```

### Health Check Metrics

| Metric Name | Type | Description |
|-------------|------|-------------|
| `nexus.health.checks.total` | Counter | Total health checks performed |
| `nexus.health.checks.passed` | Counter | Passed health checks |
| `nexus.health.checks.failed` | Counter | Failed health checks |
| `nexus.health.transitions` | Counter | Health state transitions |

---

## Benchmark Methodology

### Benchmark Environment

| Component | Specification |
|-----------|---------------|
| CPU | Apple Silicon M3 Pro / AWS c7g.medium equivalent |
| Memory | 16GB RAM minimum |
| Runtime | Tokio multi-thread runtime |
| Concurrency | 100 concurrent tasks (configurable) |
| Iterations | 10,000 minimum per test |
| Warm-up | 1,000 iterations before measurement |

### Benchmark Suite

#### 1. Registration Throughput Benchmark

```rust
#[tokio::test]
async fn benchmark_concurrent_registration() {
    let registry = ServiceRegistry::new();
    let num_services = 10_000;
    let concurrency = 100;
    
    let start = Instant::now();
    
    let handles: Vec<_> = (0..concurrency)
        .map(|i| {
            let registry = registry.clone();
            tokio::spawn(async move {
                for j in 0..num_services / concurrency {
                    let service = Service::new(
                        format!("service-{}-{}", i, j),
                        format!("192.168.1.{}.{}:8080", i, j),
                    );
                    registry.register(service).await.unwrap();
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let elapsed = start.elapsed();
    let throughput = num_services as f64 / elapsed.as_secs_f64();
    
    println!("Registration throughput: {:.2} ops/sec", throughput);
}
```

#### 2. Discovery Latency Benchmark

```rust
#[tokio::test]
async fn benchmark_discovery_latency() {
    let registry = ServiceRegistry::new();
    
    // Pre-register services
    for i in 0..1000 {
        registry.register(Service::new(
            format!("service-{}", i),
            format!("192.168.1.{}:8080", i),
        )).await.unwrap();
    }
    
    // Benchmark discovery
    let mut latencies = Vec::new();
    for _ in 0..10_000 {
        let start = Instant::now();
        registry.discover().name("service-500").await.unwrap();
        latencies.push(start.elapsed().as_secs_f64());
    }
    
    // Calculate percentiles
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("P50: {}ms", latencies[5000] * 1000);
    println!("P99: {}ms", latencies[9900] * 1000);
}
```

#### 3. Load Balancer Benchmark

```rust
#[tokio::test]
async fn benchmark_load_balancer() {
    let registry = ServiceRegistry::new();
    let balancer = RoundRobinBalancer::new();
    
    // Register 10 instances of same service
    for i in 0..10 {
        registry.register(Service::new(
            format!("api-gateway-{}", i),
            format!("10.0.0.{}:8080", i),
        )).await.unwrap();
    }
    
    let mut counts = HashMap::new();
    for _ in 0..100_000 {
        let selected = balancer.select(&registry).await;
        *counts.entry(selected.address()).or_insert(0) += 1;
    }
    
    // Verify distribution (each should be ~10,000)
    for (addr, count) in &counts {
        let deviation = (count - 10_000).abs() as f64 / 10_000.0;
        assert!(deviation < 0.01, "Load imbalance detected for {}", addr);
    }
}
```

### Benchmark Results Format

All benchmarks MUST output results in the following JSON format:

```json
{
  "benchmark_name": "registration_throughput",
  "environment": {
    "cpu": "Apple M3 Pro",
    "memory_gb": 16,
    "os": "macOS 14.4",
    "tokio_version": "1.35.0"
  },
  "parameters": {
    "num_services": 10000,
    "concurrency": 100
  },
  "results": {
    "throughput_ops_per_sec": 125432.5,
    "latency_p50_ms": 0.8,
    "latency_p99_ms": 4.2,
    "latency_p99_9_ms": 8.1,
    "errors": 0
  },
  "timestamp": "2026-04-04T23:00:00Z"
}
```

---

## SLO and SLI Definitions

### Service Level Objectives

| SLO | Target | Window |
|-----|--------|--------|
| Discovery Availability | 99.99% | 30-day rolling |
| Registration Availability | 99.95% | 30-day rolling |
| Discovery Latency P99 | < 10ms | 5-minute rolling |
| Registration Latency P99 | < 50ms | 5-minute rolling |
| Successful Health Checks | 99.9% | 1-hour rolling |

### Service Level Indicators

#### SLI: Discovery Availability

**Definition**: Percentage of discovery requests that return a healthy service
within the timeout threshold.

```rust
// SLI calculation
let total_discoveries = discovery_lookup_total.get();
let successful_discoveries = discovery_lookup_hits.get();
let availability = (successful_discoveries / total_discoveries) * 100.0;

// SLO: availability >= 99.99%
```

#### SLI: Registration Freshness

**Definition**: Percentage of registered services that have valid (non-expired)
health status.

```rust
let total_services = registry_services_total.get();
let healthy_services = registry_services_healthy.get();
let stale_services = registry_services_unhealthy.get();
let freshness = (healthy_services / total_services) * 100.0;
```

#### SLI: Discovery Latency

**Definition**: 99th percentile latency of service discovery operations.

```rust
let latency_p99 = histogram_percentile(
    "nexus.discovery.lookup.latency",
    0.99
).get();

// SLO: latency_p99 < 10ms
```

### Error Budget

| SLO | Error Budget (30-day) | Burn Rate |
|-----|----------------------|-----------|
| 99.99% availability | 43.8 minutes | 1.0x |
| 99.95% availability | 3.65 hours | 1.0x |
| 99.9% availability | 7.3 hours | 1.0x |

---

## Observability Best Practices

### 1. Structured Logging

All logs MUST use structured JSON format:

```rust
use tracing::info;

info!(
    operation = "service_registration",
    service_name = %service.name,
    service_address = %service.address,
    registration_latency_ms = elapsed.as_millis(),
    registry_size = registry.len(),
    "Service registered successfully"
);
```

### 2. Trace Context Propagation

Support W3C Trace Context for distributed tracing:

```rust
use opentelemetry::propagation::TraceContextPropagator;
use tracing_opentelemetry::OpenTelemetrySpanExt;

fn register_with_trace(service: Service) {
    let span = tracing::info_span!(
        "register_service",
        service.name = %service.name
    );
    
    let ctx = span.context();
    let propagator = TraceContextPropagator::new();
    
    // Inject trace context into service metadata
    let mut headers = HashMap::new();
    propagator.inject(&ctx, &mut headers);
    
    // ... register with headers
}
```

### 3. Metrics Cardinality Management

**Rules**:
- Service names: HIGH cardinality — use `service_name` label
- Endpoints: HIGH cardinality — use `endpoint` label
- Operation types: LOW cardinality — use `operation` label with enum values
- Environment tags: LOW cardinality — use `env` label with `dev|staging|prod`

**Avoid**:
- Labels with unbounded unique values (request IDs, user IDs)
- High-frequency timestamp labels
- Debug labels in production

### 4. Health Check Patterns

```rust
use std::time::{Duration, Instant};

pub struct HealthCheck {
    registry: ServiceRegistry,
    threshold: Duration,
}

impl HealthCheck {
    pub async fn check(&self) -> HealthStatus {
        let start = Instant::now();
        
        // Check registry responsiveness
        let services = self.registry.list_services().await;
        let elapsed = start.elapsed();
        
        if elapsed > self.threshold {
            return HealthStatus::Degraded {
                latency_ms: elapsed.as_millis() as u64,
                reason: "registry_slow_response",
            };
        }
        
        HealthStatus::Healthy {
            latency_ms: elapsed.as_millis() as u64,
            service_count: services.len(),
        }
    }
}
```

### 5. Dashboard Panels

Recommended Grafana panels:

1. **Overview Dashboard**
   - Service registration rate (ops/sec)
   - Discovery request rate (ops/sec)
   - Registry size (total services)
   - Health ratio (% healthy)

2. **Latency Dashboard**
   - P50/P90/P95/P99 discovery latency
   - P50/P90/P95/P99 registration latency
   - Latency heatmap

3. **Availability Dashboard**
   - Discovery success rate
   - Registration success rate
   - Error rate by type
   - SLO error budget remaining

---

## Alerting Thresholds

| Alert | Condition | Severity | SLO Impact |
|-------|-----------|----------|------------|
| Discovery Latency High | P99 > 10ms for 5min | Warning | 99.99% SLO |
| Discovery Latency Critical | P99 > 50ms for 1min | Critical | 99.99% SLO |
| Discovery Failure Rate | errors/total > 0.1% | Warning | 99.99% SLO |
| Registration Latency High | P99 > 50ms for 5min | Warning | 99.95% SLO |
| Registry Unhealthy Ratio | unhealthy/total > 5% | Warning | Registration SLO |
| Cache Hit Rate Low | hits/(hits+misses) < 60% | Info | Performance |

---

## Dashboard Recommendations

### Key Panels for Service Mesh Operations

1. **SLO Tracking Panel**
   - Error budget consumption
   - Burn rate indicator
   - Projected SLO breach date

2. **Service Graph Panel**
   - Registered services by status
   - Discovery request heatmap
   - Inter-service dependency map

3. **Resource Utilization Panel**
   - Memory usage by registry size
   - Lock contention metrics (via `dashmap` internals)
   - Goroutine/Task count

---

## Appendix: Metric Definitions Schema

All metrics MUST conform to this schema:

```yaml
metric:
  name: string (dot-notation: nexus.<component>.<operation>.<metric>)
  type: counter | gauge | histogram
  description: string
  unit: string
  labels:
    - name: string
      cardinality: low | medium | high
      values: string[] (for enum labels)
  buckets: float[] (for histograms only)
```
