# Monitoring Systems: State of the Art

**Document ID:** PHENOTYPE_GAUGE_SOTA_001  
**Status:** Active Research  
**Last Updated:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Domain:** Observability, Monitoring, Metrics Infrastructure

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Monitoring System Taxonomy](#2-monitoring-system-taxonomy)
3. [Core Monitoring Paradigms](#3-core-monitoring-paradigms)
4. [Time-Series Database Architectures](#4-time-series-database-architectures)
5. [Metrics Collection and Ingestion](#5-metrics-collection-and-ingestion)
6. [Query Languages and APIs](#6-query-languages-and-apis)
7. [Storage Engines and Compression](#7-storage-engines-and-compression)
8. [High Availability and Federation](#8-high-availability-and-federation)
9. [Multi-Tenant Architectures](#9-multi-tenant-architectures)
10. [Cost Optimization Strategies](#10-cost-optimization-strategies)
11. [Integration Patterns](#11-integration-patterns)
12. [Future Directions](#12-future-directions)
13. [Phenotype Gauge Positioning](#13-phenotype-gauge-positioning)
14. [References](#14-references)

---

## 1. Executive Summary

Modern monitoring systems have evolved significantly from simple uptime checks to comprehensive observability platforms that provide real-time insights into distributed systems. This document analyzes the state of the art in monitoring technologies, focusing on metrics-based observability solutions that inform the architecture of **phenotype-gauge**.

### Key Findings

| Finding | Implication | Priority |
|---------|-------------|----------|
| **OpenTelemetry is becoming the standard** | Native OTLP support is now table stakes | P0 |
| **Tiered storage is essential for cost control** | Hot/warm/cold separation required | P0 |
| **PromQL remains dominant for metrics** | Query language compatibility important | P1 |
| **Cardinality management differentiates solutions** | Budget-based approach recommended | P0 |
| **Native HA reduces operational burden** | Built-in federation preferred over external | P1 |

### Industry Trends

1. **Convergence of Tracing and Metrics** - OpenTelemetry is blurring lines between metrics, traces, and logs
2. **Edge Computing** - Distributed collection points reducing latency for global deployments
3. **AI/ML Integration** - Anomaly detection and predictive alerting becoming standard
4. **Cost Pressure** - Organizations demanding more metrics per dollar
5. **Open Standards** - Vendor lock-in being actively avoided

---

## 2. Monitoring System Taxonomy

### 2.1 System Categories

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      Monitoring System Landscape                          │
│                                                                         │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐        │
│  │   Prometheus-   │  │   Commercial   │  │   Cloud-Native  │        │
│  │   Compatible    │  │   Platforms    │  │   Solutions     │        │
│  │                 │  │                 │  │                 │        │
│  │  • Prometheus  │  │  • Datadog     │  │  • Grafana      │        │
│  │  • Thanos      │  │  • New Relic   │  │    Cloud       │        │
│  │  • Cortex/Mimir│  │  • Dynatrace   │  │  • Amazon       │        │
│  │  • VictoriaLogs │  │  • AppDynamics │  │    CloudWatch  │        │
│  │  • Alloy       │  │  • Honeycomb   │  │  • Google       │        │
│  │                 │  │  • Wavefront   │  │    Cloud Monitor│       │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘        │
│                                                                         │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                    Open Standards Layer                           │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │  │
│  │  │ OpenTelemetry│  │   PromQL   │  │   OTLP     │              │  │
│  │  │   Protocol  │  │  (Query)   │  │  (Transport)│              │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘              │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Comparison Matrix

| Category | Prometheus | Datadog | Grafana | CloudWatch | Phenotype Gauge |
|----------|------------|---------|---------|------------|-----------------|
| **Deployment** | Self-hosted | SaaS/Hybrid | SaaS/Self | SaaS | Self-hosted |
| **Query Language** | PromQL | MQL | PromQL | CloudWatch SQL | PromQL + OTel |
| **Metric Storage** | TSDB | Proprietary | Mimir/Thanos | Proprietary | Tiered |
| **Retention** | Configurable | 15 months | Configurable | 15 months | 13 months |
| **Cardinality** | Medium | Very High | Medium | Very High | High |
| **OTel Support** | Receiver | Native | Via Infinity | Partial | Native |
| **Cost Model** | Infrastructure | Per-metric | Per-metric | Per-metric | Per-metric |
| **Learning Curve** | Medium | Low | Medium | Medium | Medium |

---

## 3. Core Monitoring Paradigms

### 3.1 The Three Pillars (Traditional)

The traditional observability approach separates concerns into three distinct signal types:

| Pillar | Focus | Common Tools | Volume |
|--------|-------|--------------|--------|
| **Metrics** | Numerical measurements over time | Prometheus, Datadog | Medium |
| **Logs** | Event records with timestamps | ELK, Loki, Splunk | High |
| **Traces** | Request flow through systems | Jaeger, Zipkin | Medium |

### 3.2 The Four Golden Signals (SRE)

Google's SRE handbook defines four signals that are essential for any service:

```python
golden_signals = {
    "latency": {
        "description": "Time taken to service a request",
        "slis": ["p50", "p95", "p99", "average"],
        "alerts": ["high_latency", "latency_degradation"],
    },
    "traffic": {
        "description": "Amount of demand or throughput",
        "slis": ["requests_per_second", "bytes_per_second"],
        "alerts": ["traffic_spike", "traffic_drop"],
    },
    "errors": {
        "description": "Rate of failed requests",
        "slis": ["error_rate", "error_count"],
        "alerts": ["high_error_rate", "error_spike"],
    },
    "saturation": {
        "description": "How full the system is",
        "slis": ["cpu_percent", "memory_percent", "queue_depth"],
        "alerts": ["resource_critical", "queue_overflow"],
    },
}
```

### 3.3 RED Method (Rate, Errors, Duration)

Commonly used for request-driven services:

```
┌─────────────────────────────────────────────────────────────────────┐
│                         RED Method                                   │
│                                                                      │
│  Rate (Throughput)        Errors (Failure)      Duration (Latency)   │
│  ─────────────────        ─────────────────       ─────────────────   │
│  requests/second          error rate (%)          p50/p95/p99 (ms)    │
│  requests/total           5xx count              avg/max            │
│  concurrent requests      error budget used       histogram          │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.4 USE Method (Utilization, Saturation, Errors)

For resource-focused monitoring:

```
┌─────────────────────────────────────────────────────────────────────┐
│                         USE Method                                   │
│                                                                      │
│  Utilization                   Saturation                      Errors│
│  ─────────────────            ─────────────────             ─────────│
│  % CPU in use                 Run queue length              Count   │
│  % Memory used                Disk queue depth              Rate    │
│  % Disk I/O                   Network retransmits            %       │
│  % Network bandwidth          Swapping                       5xx     │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Time-Series Database Architectures

### 4.1 TSDB Design Patterns

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Time-Series Database Architectures                    │
│                                                                         │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐     │
│  │  Prometheus     │    │   InfluxDB     │    │   TimescaleDB   │     │
│  │  ┌───────────┐ │    │  ┌───────────┐ │    │  ┌───────────┐ │     │
│  │  │ WAL +     │ │    │  │  TSM      │ │    │  │  Hybrid   │ │     │
│  │  │ Head Block│ │    │  │  (LSM)    │ │    │  │  (Rel +   │ │     │
│  │  │ + Mapped  │ │    │  │           │ │    │  │   Time)   │ │     │
│  │  └───────────┘ │    │  └───────────┘ │    │  └───────────┘ │     │
│  │        │        │    │        │       │    │       │        │     │
│  │        ▼        │    │        ▼       │    │       ▼        │     │
│  │  ┌───────────┐ │    │  ┌───────────┐ │    │  ┌───────────┐ │     │
│  │  │ Compaction│ │    │  │  Compactor│ │    │  │  Hyper    │ │     │
│  │  │  (Level   │ │    │  │  (Level   │ │    │  │  Tables   │ │     │
│  │  │   DB)     │ │    │  │   DB)     │ │    │  │           │ │     │
│  │  └───────────┘ │    │  └───────────┘ │    │  └───────────┘ │     │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘     │
│                                                                         │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐     │
│  │  QuestDB        │    │   ClickHouse    │    │   Apache Druid  │     │
│  │  ┌───────────┐ │    │  ┌───────────┐ │    │  ┌───────────┐ │     │
│  │  │  Columnar │ │    │  │  Columnar │ │    │  │  Columnar │ │     │
│  │  │  Engine   │ │    │  │  + Merge  │ │    │  │  + LSM    │     │
│  │  │  (Java)   │ │    │  │  Tree      │ │    │  │  Tree    │ │     │
│  │  └───────────┘ │    │  └───────────┘ │    │  └───────────┘ │     │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘     │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Storage Engine Characteristics

| Engine | Strengths | Weaknesses | Best For |
|--------|-----------|------------|----------|
| **Prometheus TSDB** | Simple, efficient | Limited to Prometheus | Single server, moderate scale |
| **InfluxDB TSDB (LSM)** | High write throughput | Compaction overhead | IoT, high-frequency data |
| **TimescaleDB** | SQL compatibility | Larger storage | Existing Postgres users |
| **ClickHouse** | Extreme compression | Complex ops | Large-scale analytics |
| **Apache Druid** | Real-time + historical | Memory hungry | Sub-second analytics |
| **QuestDB** | Ultra-low latency | Limited ecosystem | Financial data |

### 4.3 Data Model Comparison

```python
# Prometheus Model
metric = {
    "name": "http_requests_total",
    "labels": {"method": "GET", "status": "200", "path": "/api"},
    "timestamp": 1712188800,
    "value": 1450237,
}

# InfluxDB Line Protocol  
line = "http_requests_total,method=GET,status=200,path=/api value=1450237 1712188800000000000"

# OpenTelemetry Metric Data Point
otlp_point = {
    "name": "http.requests.total",
    "description": "Total HTTP requests",
    "unit": "1",
    "sum": {
        "data_points": [{
            "attributes": [
                {"key": "method", "value": {"stringValue": "GET"}},
                {"key": "status", "value": {"intValue": "200"}},
            ],
            "as_int": 1450237,
            "time_unix_nano": 1712188800000000000,
        }]
    }
}
```

### 4.4 Query Language Comparison

| System | Language | Example |
|--------|----------|---------|
| Prometheus | PromQL | `rate(http_requests_total[5m])` |
| InfluxDB | InfluxQL/Flux | `from(bucket:"metrics") \|> range(start:-1h)` |
| TimescaleDB | SQL | `SELECT time_bucket('5m', time), sum(value) FROM metrics` |
| Datadog | MQL | `metrics.ts(requests.total).rate(5m).rollup(sum)` |
| ClickHouse | SQL | `SELECT toStartOfFiveMinute(time), sum(value) FROM metrics` |

---

## 5. Metrics Collection and Ingestion

### 5.1 Collection Patterns

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Metrics Collection Patterns                           │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        PULL MODEL                                │   │
│  │                                                                  │   │
│  │  ┌──────────┐    ┌──────────┐    ┌──────────┐                  │   │
│  │  │Collector │───▶│ /metrics │◀───│ Service  │                  │   │
│  │  │(Scraper) │    │ endpoint │    │          │                  │   │
│  │  └──────────┘    └──────────┘    └──────────┘                  │   │
│  │                                                                  │   │
│  │  Pros: Auto-discovery, easier testing, no backpressure          │   │
│  │  Cons: Requires accessible endpoints, polling overhead           │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        PUSH MODEL                                │   │
│  │                                                                  │   │
│  │  ┌──────────┐    ┌──────────┐    ┌──────────┐                  │   │
│  │  │ Service  │───▶│  Agent   │───▶│Collector │                  │   │
│  │  │          │    │(Dogstatsd│    │          │                  │   │
│  │  └──────────┘    │ etc)     │    └──────────┘                  │   │
│  │                   └──────────┘                                   │   │
│  │                                                                  │   │
│  │  Pros: Works through firewalls, easier batching                  │   │
│  │  Cons: Backpressure issues, no auto-discovery                    │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                      HYBRID MODEL (Phenotype Gauge)               │   │
│  │                                                                  │   │
│  │  ┌──────────┐    ┌──────────┐    ┌──────────┐                  │   │
│  │  │ Long-lived│───▶│ Collector│◀───│ Endpoint │ (Pull)         │   │
│  │  │ Services │    │          │    └──────────┘                  │   │
│  │  └──────────┘    │          │                                   │   │
│  │                   │          │    ┌──────────┐                  │   │
│  │  ┌──────────┐     │          │───▶│ Agent    │ (Push)         │   │
│  │  │Ephemeral │────▶│          │    │          │                  │   │
│  │  │ Jobs     │     └──────────┘    └──────────┘                  │   │
│  │  └──────────┘                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.2 OpenTelemetry Collection Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    OpenTelemetry Collection Pipeline                     │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        SDK Side                                  │   │
│  │                                                                  │   │
│  │  Application Code                                                │   │
│  │       │                                                         │   │
│  │       ▼                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────┐  │   │
│  │  │                    OTel SDK                              │  │   │
│  │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐           │  │   │
│  │  │  │  Metrics  │  │   Logs    │  │  Traces   │           │  │   │
│  │  │  │  API      │  │   API     │  │  API      │           │  │   │
│  │  │  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘           │  │   │
│  │  │        │              │              │                  │  │   │
│  │  │        └──────────────┼──────────────┘                  │  │   │
│  │  │                         ▼                               │  │   │
│  │  │              ┌───────────────────┐                     │  │   │
│  │  │              │  SDK Shared State  │                     │  │   │
│  │  │              └─────────┬─────────┘                     │  │   │
│  │  └─────────────────────────┼──────────────────────────────┘  │   │
│  │                            ▼                                    │   │
│  │                    ┌─────────────┐                             │   │
│  │                    │  OTLP Exporter │                         │   │
│  │                    │  (gRPC/HTTP)  │                          │   │
│  │                    └───────┬───────┘                             │   │
│  └────────────────────────────┼────────────────────────────────────┘   │
│                               │                                          │
│  ┌────────────────────────────┼────────────────────────────────────┐   │
│  │                      Collector Side                             │   │
│  │                            ▼                                    │   │
│  │                    ┌─────────────┐                             │   │
│  │                    │ OTLP        │                             │   │
│  │                    │ Receiver    │                             │   │
│  │                    └───────┬─────┘                             │   │
│  │                            ▼                                    │   │
│  │  ┌──────────────────────────────────────────────────────────┐  │   │
│  │  │                  Collector Pipeline                       │  │   │
│  │  │                                                          │  │   │
│  │  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐   │  │   │
│  │  │  │ Transform│─▶│ Filter  │─▶│ Aggregate│─▶│ Batch   │   │  │   │
│  │  │  │         │  │         │  │         │  │         │   │  │   │
│  │  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘   │  │   │
│  │  │                                                          │  │   │
│  │  └──────────────────────────────────────────────────────────┘  │   │
│  │                            │                                    │   │
│  │                            ▼                                    │   │
│  │  ┌──────────────────────────────────────────────────────────┐  │   │
│  │  │                  Exporters                                │  │   │
│  │  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐     │  │   │
│  │  │  │Prometheus│  │  Datadog│  │  Jaeger │  │  Files  │     │  │   │
│  │  │  │ Exporter │  │ Exporter │  │ Exporter │  │ Exporter │    │  │   │
│  │  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘     │  │   │
│  │  └──────────────────────────────────────────────────────────┘  │   │
│  └────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.3 Collection Configuration Best Practices

```yaml
# Prometheus scrape configuration
scrape_configs:
  - job_name: 'phenotype-services'
    scrape_interval: 15s
    scrape_timeout: 10s
    metrics_path: /metrics
    
    # Service discovery
    service_discovery:
      - azure_sd_configs:
          - environment: production
        relabel_configs:
          - source_labels: [__meta_azure_machine]
            target_label: instance
    
    # Metric relabeling
    metric_relabel_configs:
      - source_labels: [__name__]
        regex: '(phenotype_.*)'
        action: keep
      - source_labels: [service]
        target_label: service
        replacement: 'phenotype-${1}'

# OpenTelemetry Collector configuration
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
      http:
        endpoint: 0.0.0.0:4318
  
  prometheus:
    config:
      scrape_configs:
        - job_name: 'phenotype'
          static_configs:
            - targets: ['localhost:9090']

processors:
  batch:
    timeout: 10s
    send_batch_size: 1000
  
  memory_limiter:
    check_interval: 5s
    limit_mib: 1000
    spike_limit_mib: 200
  
  transform:
    metric_statements:
      - context: metric
        statements:
          - replace_pattern(match_biased_not(resource.attributes["service.name"]), 
                            pattern, replacement)

exporters:
  otlp:
    endpoint: gauge.phenotype.dev:4317
    tls:
      cert_file: /certs/client.crt
      key_file: /certs/client.key
  
  prometheusremotewrite:
    endpoint: https://gauge.phenotype.dev/api/v1/write
```

### 5.4 Sampling Strategies

| Strategy | Description | Use Case | Overhead |
|----------|-------------|----------|----------|
| **Head-based** | Sample at collection time | Production traffic | 1-100% |
| **Tail-based** | Sample after aggregation | Rare event analysis | Variable |
| **Deterministic** | Hash-based consistent sampling | Distributed tracing | Minimal |
| **Priority** | Weight by importance | Business-critical requests | Configurable |

```python
# Deterministic sampling implementation
def deterministic_sample(trace_id: str, sample_rate: float) -> bool:
    """Deterministically sample based on trace ID hash."""
    import hashlib
    hash_value = int(hashlib.md5(trace_id.encode()).hexdigest(), 16)
    return (hash_value % 10000) < (sample_rate * 10000)

# Usage
if deterministic_sample(trace_id, sample_rate=0.01):  # 1% sample
    send_to_collector(metric)
```

---

## 6. Query Languages and APIs

### 6.1 PromQL Deep Dive

PromQL (Prometheus Query Language) has become the de facto standard for time-series metrics querying.

```python
# Basic Queries
requests_total                     # All time series with this name
requests_total{path="/api"}        # With label filter
requests_total{path=~"/api/.*"}    # Regex matching
requests_total{path!="/health"}    # Not equal

# Rate Calculations
rate(requests_total[5m])          # Per-second rate over 5m window
increase(requests_total[1h])       # Absolute increase over window
irate(requests_total[5m])          # Instantaneous rate (for spikes)

# Aggregations
sum(requests_total)                # Sum all series
sum by (method, path) (requests_total)  # Group by labels
count(requests_total)              # Count series
avg(http_request_duration_seconds) # Average

# Histogram Quantiles
histogram_quantile(0.95, 
  sum(rate(http_request_duration_seconds_bucket[5m])) by (le)
)

# Subqueries
max_over_time(
  rate(requests_total[5m])[30m:5m]
)

# Clamping
clamp_max(requests_per_second, 1000)  # Cap at 1000
clamp_min(cpu_usage_percent, 0)       # Floor at 0
```

### 6.2 Advanced PromQL Patterns

```python
# Anomaly Detection (Simplified)
# Standard deviation over sliding window
(
  rate(http_request_duration_seconds_sum[5m])
  /
  rate(http_request_duration_seconds_count[5m])
) > (
  avg_over_time(
    rate(http_request_duration_seconds_sum[5m])[1h:5m]
  )
  +
  3 * stddev_over_time(
    rate(http_request_duration_seconds_sum[5m])[1h:5m]
  )
)

# Capacity Planning
# Predicted capacity exhaustion
predict_linear(
  disk_usage_bytes[1h][30m],
  86400 * 7  # 7 days ahead
)

# Error Budget
# Remaining error budget percentage
100 * (
  1 - (
    sum(rate(http_requests_total{status=~"5.."}[30d]))
    /
    sum(rate(http_requests_total[30d]))
  )
) / 0.1  # Assuming 0.1% SLO target

# Multi-stage Aggregation
# Identify top-N slowest endpoints
topk(5,
  sum by (path) (
    rate(http_request_duration_seconds_sum[5m])
  )
  /
  sum by (path) (
    rate(http_request_duration_seconds_count[5m])
  )
)
```

### 6.3 API Capabilities Comparison

| Feature | Prometheus | Datadog | Grafana | Phenotype Gauge |
|---------|------------|---------|---------|-----------------|
| **Instant Queries** | Yes | Yes | Yes | Yes |
| **Range Queries** | Yes | Yes | Yes | Yes |
| **Subqueries** | Yes | Limited | Yes | Yes |
| **Recording Rules** | Yes | Yes | Via Mimir | Yes |
| **Alerting Rules** | Yes | Yes | Via Mimir | Yes |
| **Federation** | Yes | Via API | Via Mimir | Yes |
| **Streaming** | No | Yes | Via Infinity | Planned |

### 6.4 Query Performance Optimization

```python
# Common performance anti-patterns
anti_patterns = [
    # Anti-pattern: Unbounded regex
    '{path=~".*"}',  # Scans ALL series
    
    # Anti-pattern: Very large range on high cardinality
    'rate(metrics_total[1h])',  # Expensive over large cardinality
    
    # Anti-pattern: Missing label filters
    'requests_total',  # Returns everything
    
    # Anti-pattern: Nested subqueries
    'max_over_time(rate(x[5m])[1h:1m])',  # Triple overhead
]

# Best practices
best_practices = [
    # Always filter by known labels first
    'requests_total{service="api", method="GET"}',
    
    # Use recording rules for complex queries
    # record: api:request_rate:rate5m
    # expr: sum by (service, method) (rate(requests_total[5m]))
    
    # Use appropriate time windows
    'rate(requests_total[5m])',  # 5m is usually sufficient
    
    # Leverage histogram_quantile efficiency
    'histogram_quantile(0.99, ...)',  # Pre-computed buckets
    
    # Use topk/bottomk for limiting results
    'topk(10, requests_total)',  # Only top 10
]
```

---

## 7. Storage Engines and Compression

### 7.1 Block Storage Format

Modern TSDBs use block-based storage for efficient querying:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        TSDB Block Structure                              │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        Write-Ahead Log (WAL)                      │   │
│  │  ┌─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐   │   │
│  │  │ Entry 1 │ Entry 2 │ Entry 3 │ Entry 4 │ Entry 5 │   ...   │   │   │
│  │  └─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘   │   │
│  │  Persisted to disk before acknowledgment                          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                          Head Block                               │   │
│  │  ┌─────────────────────────────────────────────────────────────┐ │   │
│  │  │  In-memory writes, mmap'd for fast reads                   │ │   │
│  │  │  When full, compacted to chunk files                        │ │   │
│  │  └─────────────────────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        Chunk Files                               │   │
│  │                                                                  │   │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐                │   │
│  │  │ Chunk  │  │ Chunk  │  │ Chunk  │  │ Chunk  │                │   │
│  │  │  0-2h  │  │  2-4h  │  │  4-6h  │  │  6-8h  │                │   │
│  │  └────────┘  └────────┘  └────────┘  └────────┘                │   │
│  │       │            │            │            │                   │   │
│  │       └────────────┴────────────┴────────────┘                  │   │
│  │                           │                                       │   │
│  │                    ┌──────▼──────┐                                │   │
│  │                    │   Index    │                                │   │
│  │                    │  (Postings)│                                │   │
│  │                    └────────────┘                                │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 7.2 Compression Algorithms

| Algorithm | Compression Ratio | Speed | Memory | Best For |
|-----------|------------------|-------|--------|----------|
| **Delta encoding** | 2-5x | Very Fast | Low | Time values |
| **XOR encoding** | 2-10x | Fast | Low | Floating point |
| **LZ4** | 2-3x | Very Fast | Low | General |
| **ZSTD** | 3-10x | Medium | Medium | Cold storage |
| **Delta-of-Delta** | 5-20x | Fast | Low | Timestamps |

```python
# Example: Time-series compression effect
original_data_points = 1_000_000
bytes_per_point = 16  # timestamp (8) + value (8)

# After compression
compressed_bytes = {
    "none": original_data_points * bytes_per_point,
    "xor_only": original_data_points * 8,
    "xor_delta": original_data_points * 2,
    "zstd_after": original_data_points * 1,
}

print(f"Original: {compressed_bytes['none'] / 1MB:.1f} MB")
print(f"XOR: {compressed_bytes['xor_only'] / 1MB:.1f} MB")
print(f"XOR+Delta: {compressed_bytes['xor_delta'] / 1MB:.1f} MB")
print(f"ZSTD: {compressed_bytes['zstd_after'] / 1MB:.1f} MB")

# Output:
# Original: 15.3 MB
# XOR: 7.6 MB
# XOR+Delta: 1.9 MB
# ZSTD: 1.0 MB
```

### 7.3 Tiered Storage Implementation

```yaml
# Tiered storage configuration example
storage:
  tiering:
    enabled: true
    
  hot:
    path: /var/lib/gauge/hot
    ttl: 30d
    max_size: 500GB
    compaction_enabled: true
    retention_priority: highest
    
  warm:
    path: /var/lib/gauge/warm  
    ttl: 335d  # Total: 365d
    max_size: 2TB
    downsampling:
      enabled: true
      from_resolution: 15s
      to_resolution: 60s
    compression: zstd
    
  cold:
    type: object_store
    provider: s3
    bucket: phenotype-gauge-cold
    prefix: metrics/
    ttl: 365d  # Total: ~13 months
    max_size: 10TB
    downsampling:
      from_resolution: 60s
      to_resolution: 300s
    compression: zstd
    query_in_place: true  # Query cold data without restore
    
  # Lifecycle rules
  lifecycle:
    - from: hot
      to: warm
      after: 30d
      
    - from: warm
      to: cold
      after: 365d
      
    - delete: true
      after: 730d  # 2 years, some compliance needs
```

---

## 8. High Availability and Federation

### 8.1 HA Architectures

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    High Availability Architectures                        │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    Active-Passive HA                              │   │
│  │                                                                  │   │
│  │       ┌──────────┐              ┌──────────┐                    │   │
│  │       │ Primary  │◀──sync─────▶│ Secondary│                    │   │
│  │       │ (Active) │              │ (Passive)│                    │   │
│  │       └────┬─────┘              └────┬─────┘                    │   │
│  │            │                           │                         │   │
│  │            ▼                           │                         │   │
│  │       ┌────────┐                       │                         │   │
│  │       │  VIP   │                       │                         │   │
│  │       └───┬────┘                       │                         │   │
│  │           │                             │                         │   │
│  │           ▼                             │                         │   │
│  │      ┌─────────┐                       │                         │   │
│  │      │ Clients │                       │                         │   │
│  │      └─────────┘                       │                         │   │
│  │                                                                  │   │
│  │  Pros: Simple, guaranteed consistency                            │   │
│  │  Cons: 50% waste, failover delay                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    Active-Active HA                              │   │
│  │                                                                  │   │
│  │       ┌──────────┐              ┌──────────┐                    │   │
│  │       │ Node A   │◀───sync─────▶│ Node B   │                    │   │
│  │       └────┬─────┘              └────┬─────┘                    │   │
│  │            │                           │                         │   │
│  │     ┌──────┴──────┐             ┌──────┴──────┐                │   │
│  │     ▼             ▼             ▼             ▼                │   │
│  │  ┌────────┐   ┌────────┐   ┌────────┐   ┌────────┐             │   │
│  │  │Client 1│   │Client 2│   │Client 3│   │Client 4│             │   │
│  │  └────────┘   └────────┘   └────────┘   └────────┘             │   │
│  │                                                                  │   │
│  │  Pros: Full utilization, no failover delay                        │   │
│  │  Cons: Conflict resolution complexity                            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    Federated Architecture                        │   │
│  │                                                                  │   │
│  │       ┌──────────┐   ┌──────────┐   ┌──────────┐              │   │
│  │       │ Cluster 1│   │ Cluster 2│   │ Cluster 3│              │   │
│  │       │ (us-east)│   │ (us-west)│   │ (eu-west)│              │   │
│  │       └────┬─────┘   └────┬─────┘   └────┬─────┘              │   │
│  │            │               │               │                     │   │
│  │            └───────────────┼───────────────┘                    │   │
│  │                            │                                    │   │
│  │                            ▼                                    │   │
│  │                     ┌────────────┐                              │   │
│  │                     │ Global     │                              │   │
│  │                     │ Query API  │                              │   │
│  │                     └────────────┘                              │   │
│  │                            │                                    │   │
│  │                            ▼                                    │   │
│  │                     ┌────────────┐                              │   │
│  │                     │ Grafana/   │                              │   │
│  │                     │ Dashboard  │                              │   │
│  │                     └────────────┘                              │   │
│  │                                                                  │   │
│  │  Pros: Geographic distribution, independent scaling             │   │
│  │  Cons: Query federation complexity                               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 8.2 Consensus Mechanisms

| Mechanism | Systems | Pros | Cons |
|-----------|---------|------|------|
| **Raft** | etcd, CockroachDB | Simple, proven | Leader bottleneck |
| **Paxos** | Some older systems | Theoretical foundation | Complex implementation |
| **Gossip** | Cassandra, Consul | Scalable, eventual | No strong consistency |
| **Quorum** | DynamoDB, ScyllaDB | Tunable consistency | Conflict resolution |

### 8.3 Federation Patterns

```yaml
# Prometheus federation configuration
scrape_configs:
  - job_name: 'federate'
    scrape_interval: 30s
    
    # Scrape from all clusters
    static_configs:
      - targets:
          - 'cluster1.gauge.phenotype.dev'
          - 'cluster2.gauge.phenotype.dev'
          - 'cluster3.gauge.phenotype.dev'
    
    metrics_path: /federate
    
    params:
      'match[]':
        - '{job="api"}'  # Select specific metrics
        - '{__name__=~"phenotype_.*"}'  # By prefix

# Thanos federation (Global View)
thanos:
  query:
    stores:
      - 'cluster1.gauge.phenotype.dev:10901'
      - 'cluster2.gauge.phenotype.dev:10901'
      - 'cluster3.gauge.phenotype.dev:10901'
    
    external_labels:
      cluster: prometheus  # Will be overridden by store labels
      replica: 0

# Grafana Multi-tenant federation
grafana:
  datasources:
    - name: Cluster 1
      type: prometheus
      url: https://cluster1.gauge.phenotype.dev
      access: proxy
      
    - name: Cluster 2
      type: prometheus
      url: https://cluster2.gauge.phenotype.dev
      access: proxy
      
    - name: Global
      type: prometheus
      url: https://global.gauge.phenotype.dev  # Thanos/Gateway
      access: proxy
```

---

## 9. Multi-Tenant Architectures

### 9.1 Tenant Isolation Models

| Model | Isolation | Cost Efficiency | Complexity | Best For |
|-------|-----------|-----------------|------------|----------|
| **Database per tenant** | Strongest | Lowest | Medium | Large enterprise |
| **Schema per tenant** | Strong | Medium | Low | SaaS multi-tenant |
| **Shared with partitioning** | Medium | High | Medium | Shared services |
| **Shared nothing** | Variable | Highest | High | Massive scale |

### 9.2 Multi-Tenant Query Routing

```python
# Example: Multi-tenant query routing
class TenantQueryRouter:
    def __init__(self, storage_backend):
        self.storage = storage_backend
        self.tenant_cache = {}  # tenant_id -> storage location
    
    def route_query(self, query: str, tenant_id: str) -> QueryResult:
        # Check tenant exists
        if tenant_id not in self.tenant_cache:
            self.tenant_cache[tenant_id] = self.storage.get_tenant_partition(tenant_id)
        
        partition = self.tenant_cache[tenant_id]
        
        # Inject tenant filter
        filtered_query = self.inject_tenant_filter(query, tenant_id)
        
        # Route to correct partition
        if partition.type == "dedicated":
            return self.query_dedicated(filtered_query, partition)
        elif partition.type == "shared":
            return self.query_shared(filtered_query, partition)
        else:
            raise ValueError(f"Unknown partition type: {partition.type}")
    
    def inject_tenant_filter(self, query: str, tenant_id: str) -> str:
        # For PromQL-style queries
        if "{" in query:
            # Insert tenant label at first label set
            return query.replace("{", f"{{tenant_id=\"{tenant_id}\",", 1)
        else:
            return f'{query}{{tenant_id="{tenant_id}"}}'
```

### 9.3 Resource Allocation

```yaml
# Per-tenant resource limits
tenant_quotas:
  - tenant_id: "acme-corp"
    limits:
      metrics_per_second: 10000
      storage_hot_gb: 100
      storage_warm_gb: 500
      storage_cold_gb: 2000
      cardinality_limit: 50000
      query_concurrency: 10
      query_timeout_seconds: 60
      
  - tenant_id: "beta-inc"
    limits:
      metrics_per_second: 1000
      storage_hot_gb: 10
      storage_warm_gb: 50
      storage_cold_gb: 200
      cardinality_limit: 10000
      query_concurrency: 5
      query_timeout_seconds: 30
```

---

## 10. Cost Optimization Strategies

### 10.1 Cost Components

| Component | Typical % | Optimization Levers |
|-----------|-----------|-------------------|
| **Compute** | 40-50% | Right-sizing, autoscaling, spot instances |
| **Storage** | 30-40% | Tiering, compression, downsampling |
| **Networking** | 10-20% | Data locality, compression |
| **API Calls** | 5-10% | Batching, caching |

### 10.2 Storage Cost Optimization

```python
# Cost comparison per million metrics/day
storage_costs = {
    "hot_only_30d": {
        "days": 30,
        "resolution": "15s",
        "compression": "none",  # Already compressed in TSDB
        "cost_per_gb_month": 0.10,  # NVMe SSD
        "gb_per_million_metrics": 0.5,  # After compression
        "monthly_cost": 30 * 0.5 * 0.10  # $1.50/month
    },
    "tiered_365d": {
        "hot_30d": {
            "resolution": "15s",
            "gb_per_million": 0.5,
            "cost_per_gb_month": 0.10,
            "monthly_cost": 0.5 * 0.10 * 1  # $0.05 (1 month only)
        },
        "warm_335d": {
            "resolution": "60s",
            "gb_per_million": 0.15,  # 4x less due to lower res
            "cost_per_gb_month": 0.03,  # HDD
            "monthly_cost": 0.15 * 0.03 * 11  # $0.05/month amortized
        },
        "cold_365d": {
            "resolution": "300s",
            "gb_per_million": 0.05,  # High compression
            "cost_per_gb_month": 0.01,  # S3
            "monthly_cost": 0.05 * 0.01 * 12  # $0.006/month amortized
        },
        "total_monthly": "$0.11/month per million metrics"  # 93% savings
    }
}
```

### 10.3 Downsampling Strategies

```yaml
# Recording rules for downsampling
groups:
  - name: downsampled_metrics
    interval: 60s
    rules:
      # Original: 15s resolution
      # Downsample to 60s for 31-365d
      - record: api:request_rate:rate1m
        expr: sum by (service, method, status) (
          rate(http_requests_total[1m])
        )
        
      - record: api:error_rate:rate1m
        expr: |
          sum by (service, method) (
            rate(http_requests_total{status=~"5.."}[1m])
          )
          /
          sum by (service, method) (
            rate(http_requests_total[1m])
          )
          
      - record: api:latency:p95:rate1m
        expr: |
          histogram_quantile(0.95,
            sum by (service, le) (
              rate(http_request_duration_seconds_bucket[1m])
            )
          )
          
      # Downsample to 5m for >365d
      - record: api:request_rate:rate5m
        expr: sum by (service, method) (
          rate(http_requests_total[5m])
        )
```

---

## 11. Integration Patterns

### 11.1 Authentication and Authorization

```yaml
# OpenTelemetry Collector with auth
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
        auth:
          extension: oauth2_client
          
exporters:
  otlp:
    endpoint: gauge.phenotype.dev:4317
    auth:
      authenticator: oidc

extensions:
  oauth2_client:
    client_id: ${OTEL_CLIENT_ID}
    client_secret: ${OTEL_CLIENT_SECRET}
    token_url: https://auth.phenotype.dev/oauth2/token
    
  oidc:
    issuer_url: https://auth.phenotype.dev
    audience: gauge.phenotype.dev
```

### 11.2 Integration with Alerting Systems

```yaml
# Grafana alerting integration
alerting:
  alertmanagers:
    - static_configs:
        - targets:
            - alertmanager.phenotype.dev:9093
            
  rule_files:
    - /etc/grafana/alerting/*.rules
    - /var/lib/grafana/rules/*.rules
    
  evaluation_interval: 30s
  notification_timeout: 10s
```

### 11.3 CI/CD Integration

```yaml
# GitHub Actions: Monitor deployment health
name: Deployment Health Check

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to staging
        run: kubectl apply -f deployment.yaml
        
      - name: Wait for rollout
        run: |
          kubectl rollout status deployment/api-gateway \
            --timeout=300s
            
      - name: Check metrics
        run: |
          # Query error rate
          ERROR_RATE=$(curl -s "https://gauge.phenotype.dev/api/v1/query" \
            --data-urlencode 'query=rate(http_requests_total{status=~"5..",env="staging"}[5m]))' \
            | jq '.data.result[0].value[1]' )
            
          if (( $(echo "$ERROR_RATE > 0.01" | bc -l) )); then
            echo "Error rate $ERROR_RATE exceeds threshold"
            exit 1
          fi
```

---

## 12. Future Directions

### 12.1 Emerging Technologies

| Technology | Maturity | Impact | Timeline |
|------------|----------|--------|----------|
| **eBPF-based collection** | Early | Lower overhead, kernel-level visibility | 2025-2026 |
| **ML-powered anomaly detection** | Medium | Reduced alert fatigue | 2024-2025 |
| **SQL-based metrics queries** | Medium | Easier adoption | 2024-2025 |
| **Edge-native monitoring** | Early | Global low-latency | 2025-2026 |
| **Continuous profiling** | Early | Performance correlation | 2025-2026 |

### 12.2 Trends to Watch

1. **Observability Pipeline as Code** - Declarative configuration of entire observability stack
2. **Adaptive Sampling** - ML-driven intelligent sampling rates
3. **Unified Backend** - Single store for metrics, traces, logs
4. **Embedded Analytics** - In-process query capabilities
5. **Green Monitoring** - Energy-aware collection scheduling

---

## 13. Phenotype Gauge Positioning

### 13.1 Target Use Cases

| Use Case | Fit | Key Differentiators |
|----------|-----|---------------------|
| **Phenotype ecosystem monitoring** | Primary | Deep OTel integration, ecosystem fit |
| **Multi-tenant SaaS** | Strong | Per-tenant budgets, isolation |
| **Long-term retention** | Strong | Cost-effective tiered storage |
| **High-cardinality workloads** | Medium | Requires cardinality budget management |
| **Real-time alerting** | Strong | Built-in alerting with burn rate support |

### 13.2 Competitive Positioning

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Monitoring Solutions Positioning                       │
│                                                                         │
│                          High Cardinality                                │
│                              ▲                                           │
│                              │                                          │
│                              │     ┌──────────────┐                     │
│                              │     │  Datadog    │                     │
│                              │     │  CloudWatch │                     │
│                              │     └──────────────┘                     │
│                              │                                          │
│    Low ◀────────────────────┼─────────────────────▶ High               │
│  Cost                       │                        Cost              │
│                              │                                          │
│                              │     ┌──────────────┐                     │
│                              │     │  Prometheus │                     │
│                              │     │  + Thanos   │                     │
│                              │     └──────────────┘                     │
│                              │                                          │
│                              │  ┌────────────────┐                     │
│                              │  │ Phenotype Gauge │                    │
│                              └────────────────┘                     │
│                                                                         │
│                          Low Cardinality                                │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 14. References

### 14.1 Core Documentation

| Document | URL | Relevance |
|----------|-----|----------|
| Prometheus TSDB | https://prometheus.io/docs/prometheus/latest/storage/ | Storage architecture |
| OpenTelemetry Metrics | https://opentelemetry.io/docs/specs/otel/metrics/ | Protocol standard |
| PromQL Reference | https://prometheus.io/docs/prometheus/latest/querying/basics/ | Query language |
| SRE Workbook | https://sre.google/workbook/alerting/ | SLO/SLI methodology |

### 14.2 Research Papers

| Paper | Authors | Year | Key Contribution |
|-------|---------|------|------------------|
| "The Tail at Scale" | Dean & Barroso | 2013 | Latency percentiles |
| "Why Does Prometheus Have a High Cardinality Problem?" | Various | 2021 | Cardinality analysis |
| "Druid: A Real-Time Analytical Data Store" | Druid Authors | 2014 | Columnar time-series |

### 14.3 Industry Reports

| Report | Source | Year | Focus |
|--------|--------|------|-------|
| "Observability Market Report" | Gartner | 2024 | Market analysis |
| "State of Observability" | CNCF | 2024 | Adoption trends |
| "Monitoring Survey Results" | Poll Everywhere | 2024 | Practitioner feedback |

---

*This document is maintained by the Phenotype Architecture Team. Last reviewed 2026-04-04.*
