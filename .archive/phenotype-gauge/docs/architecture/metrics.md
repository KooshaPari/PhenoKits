# Metrics Architecture - phenotype-gauge

**Document ID:** PHENOTYPE_GAUGE_METRICS_001  
**Status:** Active  
**Last Updated:** 2026-04-04  
**Author:** Phenotype Architecture Team  

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Observability Metrics Definitions](#2-observability-metrics-definitions)
3. [Cardinality Management](#3-cardinality-management)
4. [Scrape Intervals and Collection Strategy](#4-scrape-intervals-and-collection-strategy)
5. [Data Retention Policies](#5-data-retention-policies)
6. [Benchmark Methodology](#6-benchmark-methodology)
7. [Comparison with Industry Standards](#7-comparison-with-industry-standards)
8. [SLO/SLI Definitions](#8-slosli-definitions)
9. [Alerting and Thresholds](#9-alerting-and-thresholds)
10. [Implementation Reference](#10-implementation-reference)

---

## 1. Executive Summary

This document defines the comprehensive metrics architecture for **phenotype-gauge**, an observability and monitoring component within the Phenotype ecosystem. It establishes standards for metric collection, storage, querying, and analysis that ensure consistent, high-quality monitoring across all Phenotype services.

### Goals

| Goal | Target | Measurement |
|------|--------|-------------|
| Metric Ingestion Rate | 100K metrics/sec | p95 latency < 10ms |
| Query Response Time | < 500ms for 95th percentile | 30-day window |
| Data Retention | 13 months hot storage | 99.9% availability |
| Cardinality Budget | 50K unique time series per service | No unbounded growth |

### Non-Goals

- This document does not cover tracing or logging in detail (see separate specifications)
- Distributed tracing correlation is out of scope for initial implementation
- ML-based anomaly detection is a future enhancement

---

## 2. Observability Metrics Definitions

### 2.1 The Four Golden Signals

All services MUST emit these four golden signals:

| Signal | Definition | Metric Types | SLI |
|--------|------------|--------------|-----|
| **Latency** | Time to complete requests | Duration histograms, percentiles | p50, p95, p99 |
| **Traffic** | Volume of requests | Request count, throughput | requests/sec |
| **Errors** | Failure rate | Error count, error rate | Error rate % |
| **Saturation** | Resource utilization | CPU, memory, queue depth | Utilization % |

### 2.2 Metric Types

#### 2.2.1 Counter

Monotonically increasing integer value. Used for events that can only increase.

```python
# Example: Total HTTP requests
requests_total{cethod="GET", status="200"} 1450237
requests_total{method="POST", status="201"} 892134
```

| Property | Value |
|----------|-------|
| Type | int64 (unsigned) |
| Reset | Never (except process restart) |
| Aggregation | Sum, rate of change |
| Storage | 8 bytes |

#### 2.2.2 Gauge

Numeric value that can go up or down. Used for point-in-time measurements.

```python
# Example: Current memory usage
memory_usage_bytes{instance="node-1"} 16777216
memory_usage_bytes{instance="node-2"} 8589934592
```

| Property | Value |
|----------|-------|
| Type | float64 |
| Reset | Can be set to any value |
| Aggregation | Last value, avg, min, max |
| Storage | 8 bytes |

#### 2.2.3 Histogram

Distribution of values in buckets. Used for latency, payload sizes, etc.

```python
# Example: Request latency distribution
http_request_duration_seconds_bucket{le="0.005"} 12450
http_request_duration_seconds_bucket{le="0.01"} 45230
http_request_duration_seconds_bucket{le="0.025"} 89234
http_request_duration_seconds_bucket{le="0.05"} 102456
http_request_duration_seconds_bucket{le="0.1"} 105234
http_request_duration_seconds_bucket{le="0.25"} 105890
http_request_duration_seconds_bucket{le="0.5"} 105901
http_request_duration_seconds_bucket{le="1.0"} 105902
http_request_duration_seconds_bucket{le="2.5"} 105902
http_request_duration_seconds_bucket{le="5.0"} 105902
http_request_duration_seconds_bucket{le="+Inf"} 105902
```

| Property | Value |
|----------|-------|
| Type | Array of counters (buckets) + sum + count |
| Buckets | 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5, 10, +Inf |
| Aggregation | Quantiles, avg, rate |
| Storage | bucket_count * 8 + 16 bytes |

#### 2.2.4 Summary

Similar to histogram but with client-side calculated quantiles.

```python
# Example: Request latency quantiles
http_request_duration_seconds{quantile="0.5"} 0.023
http_request_duration_seconds{quantile="0.9"} 0.087
http_request_duration_seconds{quantile="0.99"} 0.234
```

| Property | Value |
|----------|-------|
| Type | Per-quantile gauges + count + sum |
| Quantiles | 0.5, 0.9, 0.99 (configurable) |
| Aggregation | Pre-calculated on client |
| Storage | quantile_count * 8 + 16 bytes |

### 2.3 Required Metrics for All Services

Every Phenotype service MUST emit these metrics:

```python
# Process metrics
process_cpu_seconds_total{} 1250.34
process_open_fds{} 256
process_max_fds{} 1024
process_resident_memory_bytes{} 134217728
process_virtual_memory_bytes{} 536870912
process_start_time_seconds{} 1712188800

# Runtime metrics (if applicable)
go_goroutines{} 42
go_threads{} 8
goGc_duration_seconds{} 0.000234
goMemStats_alloc_bytes{} 823456

# HTTP metrics (for HTTP services)
http_requests_total{method="", path="", status=""} 
http_request_duration_seconds_bucket{le=""}
http_requests_in_flight{} 0

# Business metrics (service-specific)
business_operation_total{operation=""} 0
business_operation_duration_seconds_bucket{le=""} 0
```

---

## 3. Cardinality Management

### 3.1 Cardinality Budget

Each service has a **cardinality budget** - maximum number of unique time series allowed.

| Service Tier | Budget | Examples |
|--------------|--------|----------|
| Critical | 100K | API gateway, auth service |
| High | 50K | Core business services |
| Medium | 25K | Supporting services |
| Low | 10K | Background jobs, workers |

### 3.2 Label Best Practices

**GOOD: Low Cardinality Labels**

```python
# Static labels with known values
http_requests_total{method="GET", status="200", path="/api/users"}
http_requests_total{method="POST", status="201", path="/api/users"}

# Bounded cardinality
memory_usage_bytes{instance="prod-api-1", region="us-east-1"}
```

**BAD: High/Unbounded Cardinality Labels**

```python
# User ID as label - UNBOUNDED
http_requests_total{user_id="12345"}  
http_requests_total{user_id="67890"}  # Grows forever!

# Request ID as label - UNBOUNDED
http_requests_total{request_id="abc-123"}  # NEVER do this

# Arbitrary strings - UNBOUNDED
http_requests_total{query="SELECT * FROM users WHERE id=123"}
http_requests_total{query="SELECT * FROM users WHERE id=456"}  # Grows forever!
```

### 3.3 Label Naming Convention

| Convention | Example |
|------------|---------|
| Lowercase | `instance`, `method` |
| Underscores | `request_id` (only when needed) |
| No special chars | `status_code`, not `status-code` |
| Consistent across services | `path` not `endpoint` and `route` |

### 3.4 High Cardinality Handling

When you need high-cardinality data for debugging:

| Technique | Use Case | Example |
|-----------|----------|---------|
| Trace IDs | Request correlation | Store in trace, not metric |
| Log enrichment | Detailed debugging | Add context to logs |
| Sampling | Cost reduction | Collect 1% of detailed metrics |
| Top-N metrics | Ranking analysis | `topk(10, ...)` |

---

## 4. Scrape Intervals and Collection Strategy

### 4.1 Standard Scrape Intervals

| Metric Type | Interval | Rationale |
|-------------|----------|-----------|
| Infrastructure | 15s | Fast detection of issues |
| Application | 15s | Balance granularity vs cost |
| Business metrics | 60s | Lower fidelity acceptable |
| Audit/compliance | 300s | Infrequent analysis |

### 4.2 Collection Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Metrics Collection Architecture                        │
│                                                                         │
│  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐          │
│  │   Service A  │     │   Service B  │     │   Service C  │          │
│  │  ┌────────┐  │     │  ┌────────┐  │     │  ┌────────┐  │          │
│  │  │ /metrics│  │     │  │ /metrics│  │     │  │ /metrics│  │          │
│  │  └────┬───┘  │     │  └────┬───┘  │     │  └────┬───┘  │          │
│  └───────┼──────┘     └───────┼──────┘     └───────┼──────┘          │
│          │                     │                     │                  │
│          ▼                     ▼                     ▼                  │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                      Phenotype Gauge Collector                     │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │  │
│  │  │  Prometheus  │  │  StatsD      │  │  OpenTelemetry│         │  │
│  │  │  Receiver    │  │  Receiver    │  │  Receiver    │         │  │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘         │  │
│  │         └──────────────────┼──────────────────┘                 │  │
│  │                            ▼                                    │  │
│  │                   ┌──────────────┐                             │  │
│  │                   │  Pipeline     │                             │  │
│  │                   │  (Transform, │                             │  │
│  │                   │   Filter,    │                             │  │
│  │                   │   Aggregate) │                             │  │
│  │                   └──────┬───────┘                             │  │
│  └──────────────────────────┼────────────────────────────────────┘  │
│                             ▼                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                         Storage Layer                             │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │  │
│  │  │  Hot Storage │  │ Warm Storage │  │  Cold Storage │         │  │
│  │  │  (SSD)       │  │  (HDD)       │  │  (Object)    │         │  │
│  │  │  0-30 days   │  │  31-365 days │  │  1-13 months  │         │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘         │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.3 Push vs Pull Model

| Model | Pros | Cons | Best For |
|-------|------|------|----------|
| **Pull** | Auto-discovery, easier testing, no backpressure | Requires accessible endpoints | Services with stable endpoints |
| **Push** | Works through firewalls, easier batching, reduces load | Backpressure issues, no auto-discovery | Short-lived jobs, functions |

**Decision:** Hybrid approach - Pull for long-lived services, Push for ephemeral workloads.

### 4.4 Remote Write Configuration

```yaml
remote_write:
  - name: phenotype_gauge
    url: https://gauge.phenotype.dev/api/v1/write
    timeout: 10s
    queue_config:
      capacity: 10000
      max_shards: 10
      min_shards: 1
      max_samples_per_send: 2000
      batch_send_deadline: 5s
    metadata_config:
      send: true
      send_interval: 1m
    tls_config:
      ca_file: /etc/prom/certs/ca.crt
      cert_file: /etc/prom/certs/client.crt
      key_file: /etc/prom/certs/client.key
```

---

## 5. Data Retention Policies

### 5.1 Storage Tiers

| Tier | Duration | Resolution | Storage Type | Use Case |
|------|----------|------------|--------------|----------|
| Hot | 0-30 days | 15s | NVMe SSD | Real-time dashboards, alerts |
| Warm | 31-365 days | 60s | HDD | Historical analysis, capacity planning |
| Cold | 1-13 months | 300s | Object Storage | Audit, compliance, long-term trends |

### 5.2 Retention Configuration

```yaml
retention:
  hot:
    duration: 30d
    resolution: 15s
    compaction: true
  
  warm:
    duration: 335d  # Total 365d with hot
    resolution: 60s
    downsample: true
  
  cold:
    duration: 365d  # Total ~13 months
    resolution: 300s
    compression: zstd

# Downsampling rules (applied during compaction)
downsampling:
  - from: 15s
    to: 60s
    aggregations:
      - avg
      - min
      - max
      - sum
    after: 30d
  
  - from: 60s
    to: 300s
    aggregations:
      - avg
      - min
      - max
    after: 365d
```

### 5.3 Storage Quota Management

| Service Tier | Hot Storage | Warm Storage | Cold Storage |
|--------------|-------------|--------------|--------------|
| Critical | 50 GB | 200 GB | 500 GB |
| High | 25 GB | 100 GB | 250 GB |
| Medium | 10 GB | 50 GB | 100 GB |
| Low | 5 GB | 25 GB | 50 GB |

---

## 6. Benchmark Methodology

### 6.1 Metrics Ingestion Benchmark

**Objective:** Measure maximum sustainable ingestion rate with p99 < 10ms latency.

#### Test Setup

```yaml
benchmark:
  name: metrics_ingestion
  duration: 300s  # 5 minutes steady state
  
  targets:
    - name: single_node
      endpoints: 1
      replicas: 1
    
    - name: clustered
      endpoints: 5
      replicas: 3
  
  workloads:
    - name: light
      metrics_per_endpoint: 1000
      labels_per_metric: 5
      scrape_interval: 15s
    
    - name: medium
      metrics_per_endpoint: 5000
      labels_per_metric: 10
      scrape_interval: 15s
    
    - name: heavy
      metrics_per_endpoint: 20000
      labels_per_metric: 15
      scrape_interval: 15s
```

#### Metrics to Capture

| Metric | Description | Target |
|--------|-------------|--------|
| `ingestion_rate` | Metrics/second sustained | > 100K |
| `ingestion_latency_p99` | Time to acknowledge write | < 10ms |
| `queue_depth` | Pending writes | < 1000 |
| `storage_iops` | Disk operations/second | < 10K |
| `memory_usage` | Process RSS | < 16GB |

#### Acceptance Criteria

```python
acceptance_criteria = {
    "max_ingestion_rate": ">100K metrics/sec",
    "p99_write_latency": "<10ms",
    "p95_query_latency": "<500ms (30d window)",
    "storage_efficiency": ">1M metrics/GB",
    "recovery_time": "<30s after failure",
}
```

### 6.2 Query Performance Benchmark

**Objective:** Ensure queries complete within SLA across different time windows and aggregations.

#### Test Queries

```python
queries = [
    {
        "name": "realtime_percentile",
        "query": 'histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))',
        "window": "5m",
        "expected_latency": "<100ms",
    },
    {
        "name": "daily_trend",
        "query": 'sum by (path) (rate(http_requests_total[1h]))',
        "window": "30d",
        "expected_latency": "<2s",
    },
    {
        "name": "multi_label_aggregation",
        "query": 'sum by (instance, region, status) (rate(http_requests_total[5m]))',
        "window": "7d",
        "expected_latency": "<500ms",
    },
    {
        "name": "topk_analysis",
        "query": 'topk(10, sum by (path) (rate(http_request_duration_seconds_sum[5m]) / rate(http_request_duration_seconds_count[5m])))',
        "window": "1d",
        "expected_latency": "<1s",
    },
]
```

### 6.3 Benchmark Execution

```bash
# Run ingestion benchmark
phenotype-gauge benchmark run \
  --type ingestion \
  --duration 300s \
  --target https://gauge.phenotype.dev \
  --workload heavy

# Run query benchmark  
phenotype-gauge benchmark run \
  --type query \
  --duration 600s \
  --target https://gauge.phenotype.dev \
  --queries ./benchmarks/queries.yaml

# Generate report
phenotype-gauge benchmark report \
  --output ./results/benchmark_$(date +%Y%m%d).json \
  --format html
```

---

## 7. Comparison with Industry Standards

### 7.1 Feature Matrix

| Feature | Prometheus | Datadog | Grafana | Phenotype Gauge |
|---------|-------------|---------|---------|-----------------|
| **Metrics Model** | Pull | Push | Pull | Hybrid Pull/Push |
| **Cardinality Handling** | Basic | Excellent | Basic | Advanced |
| **Query Language** | PromQL | MQL | PromQL + SQL | PromQL + OTel |
| **Retention** | Configurable | 15mo default | Configurable | 13mo hot |
| **High Availability** | Thanos/Cortex | Native | Loki/Mimir | Native |
| **Cost Model** | Open source | Per-host | Open source | Per-metric |
| **OTel Support** | Via receiver | Native | Via Infinity | Native |

### 7.2 Architecture Comparison

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Prometheus Architecture                        │
│                                                                      │
│  ┌────────┐    ┌────────┐    ┌────────┐         ┌─────────────┐   │
│  │Target  │───▶│Scrape  │───▶│TSDB    │────────▶│Query API    │   │
│  │/metrics│    │Manager │    │WAL     │         │(PromQL)     │   │
│  └────────┘    └────────┘    └────────┘         └─────────────┘   │
│                                     │                               │
│                              ┌──────▼──────┐                       │
│                              │ Remote Write │ (Optional)            │
│                              └─────────────┘                       │
│                                                                      │
│  Limitations:                                                        │
│  - Single-server TSDB limits scale                                  │
│  - Query federation complexity                                       │
│  - High cardinality expensive                                        │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                        Datadog Architecture                          │
│                                                                      │
│  ┌────────┐    ┌────────┐    ┌─────────┐    ┌──────────────────┐    │
│  │ Agent  │───▶│ Ingest │───▶│ Process │───▶│  Time-Corrected │    │
│  │(Dogstatsd)│   │Pipeline│   │         │    │  Indexed Store  │    │
│  └────────┘    └────────┘    └─────────┘    └──────────────────┘    │
│                                           │                         │
│                                    ┌──────▼──────┐                  │
│                                    │  Datadog    │                  │
│                                    │  Query API  │                  │
│                                    └─────────────┘                  │
│                                                                      │
│  Advantages:                                                         │
│  + Excellent cardinality handling                                     │
│  + Integrated APM + Logs                                            │
│  - Vendor lock-in                                                   │
│  - Cost per metric can escalate                                      │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                    Phenotype Gauge Architecture                      │
│                                                                      │
│  ┌────────┐    ┌────────┐    ┌─────────┐    ┌──────────────────┐  │
│  │Service │───▶│ Collector│───▶│Pipeline │───▶│  Tiered Storage │  │
│  │/metrics│    │(OTel/Prom│   │(Transform│   │  Hot/Warm/Cold  │  │
│  └────────┘    │ Receiver)│   │ Aggregate)│   └──────────────────┘  │
│                └────────┘    └─────────┘            │              │
│                                                    ▼              │
│  ┌────────┐    ┌────────┐    ┌─────────┐    ┌──────────────────┐  │
│  │ Trace  │───▶│  Log   │───▶│Metrics  │───▶│  Unified Query  │  │
│  │ ID     │    │ ID     │    │  Store  │    │     Layer        │  │
│  └────────┘    └────────┘    └─────────┘    └──────────────────┘  │
│                                                                      │
│  Advantages:                                                         │
│  + Open standard (OTel)                                             │
│  + Tiered storage cost optimization                                  │
│  + Phenotype ecosystem integration                                   │
│  + Flexible cardinality budgets                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### 7.3 Cost Comparison (Monthly, 1M metrics/day)

| Provider | Infrastructure | Storage | Query | Total |
|----------|---------------|---------|-------|-------|
| Prometheus (self-hosted) | $200 (3x c5.large) | $50 (S3) | Included | ~$250 |
| Datadog Pro | $15/host | Included | Included | ~$450+ |
| Grafana Cloud | $50/500K metrics | Included | Included | ~$150+ |
| **Phenotype Gauge** | $100 (2x c5.large) | $30 (S3) | Included | ~$130 |

### 7.4 Performance Comparison

| Operation | Prometheus | Datadog | Phenotype Gauge |
|-----------|------------|---------|-----------------|
| Ingest (10K/sec) | ~8ms | ~2ms | ~5ms |
| Instant query (1h) | ~50ms | ~20ms | ~30ms |
| Range query (30d) | ~2s | ~500ms | ~800ms |
| Cardinality (per instance) | 50K | 1M+ | 100K |

---

## 8. SLO/SLI Definitions

### 8.1 Monitoring System SLOs

| SLO | Definition | SLI | Target | Window |
|-----|-----------|-----|--------|--------|
| **Availability** | Monitoring system is accessible | `up{job="phenotype-gauge"}` | 99.9% | 30d rolling |
| **Ingestion** | Metrics ingested within SLA | `rate(metrics_ingested_total[5m]) / rate(metrics_scraped_total[5m])` | >99.5% | 5m |
| **Latency** | Query response time | `histogram_quantile(0.95, rate(query_duration_seconds_bucket[5m]))` | <500ms | 5m |
| **Freshness** | Data available within interval | `time() - max_over_time(metrics_last_update_timestamp[5m])` | <60s | 5m |

### 8.2 Service-Level SLOs for Monitored Services

```yaml
slos:
  - name: api-availability
    description: "API gateway availability"
    sli: |
      1 - (
        sum(rate(http_requests_total{status=~"5.."}[5m]))
        /
        sum(rate(http_requests_total[5m]))
      )
    targets:
      - window: 30d
        target: 99.95
        alert_threshold: 99.9
      
  - name: p95-latency
    description: "95th percentile request latency"
    sli: |
      histogram_quantile(0.95, 
        sum(rate(http_request_duration_seconds_bucket[5m])) by (le)
      )
    targets:
      - window: 7d
        target: <200ms
        alert_threshold: 500ms
      
  - name: error-budget
    description: "Error budget remaining"
    sli: |
      1 - (
        sum(rate(http_requests_total{status=~"5.."}[24h]))
        /
        sum(rate(http_requests_total[24h]))
      )
    targets:
      - window: 30d
        target: >99.5
        burn_rate_threshold: 14.4x  # 1% error in 6 hours = exhausted
```

### 8.3 Error Budget Policy

```yaml
error_budget_policy:
  name: api-standard
  
  objectives:
    - slo: availability
      window: 30d
      target: 99.9
      error_budget: 0.1% = 43.8 minutes/month
      
    - slo: latency
      window: 7d
      target: 95% < 200ms
      error_budget: 5% of requests
      
  alerting:
    burn_rate_alerts:
      - name: fast_burn
        threshold: 14.4  # Exhaust budget in 1 hour
        window: 1h
        severity: critical
        
      - name: slow_burn  
        threshold: 6
        window: 6h
        severity: warning
        
  actions:
    on_budget_exhausted:
      - page.on_call
      - create_incident
      - freeze deployments
      
    on_budget_warning:
      - notify_slack
      - schedule_review
```

---

## 9. Alerting and Thresholds

### 9.1 Alert Severity Levels

| Level | Response Time | Examples | Escalation |
|-------|--------------|----------|------------|
| **Critical (P1)** | < 5 minutes | Monitoring down, data loss | Page + Escalate |
| **High (P2)** | < 15 minutes | SLO breach, high error rate | Page |
| **Medium (P3)** | < 1 hour | Performance degradation | Slack + Ticket |
| **Low (P4)** | < 24 hours | Minor issues, capacity planning | Ticket |

### 9.2 Standard Alert Rules

```yaml
groups:
  - name: phenotype_gauge_alerts
    rules:
      # Monitoring System Health
      - alert: GaugeDown
        expr: up{job="phenotype-gauge"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Phenotype Gauge is down"
          description: "Monitoring system has been unreachable for 1 minute"
          
      - alert: GaugeHighLatency
        expr: histogram_quantile(0.95, rate(gauge_query_duration_seconds_bucket[5m])) > 1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Gauge query latency elevated"
          description: "95th percentile query latency is {{ $value }}s"
          
      # Data Freshness
      - alert: StaleMetrics
        expr: time() - max_over_time(metrics_last_update_timestamp[5m]) > 120
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "Metrics not being updated"
          description: "No metrics received in the last 2 minutes"
          
      # Storage Health
      - alert: StorageCapacityWarning
        expr: gauge_storage_used_bytes / gauge_storage_limit_bytes > 0.8
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Storage capacity at 80%"
          description: "Storage utilization is {{ $value | humanizePercentage }}"
          
      # Cardinality
      - alert: CardinalityBudgetWarning
        expr: gauge_cardinality_used / gauge_cardinality_limit > 0.8
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Cardinality budget at 80%"
          description: "Service {{ $labels.service }} approaching cardinality limit"
```

---

## 10. Implementation Reference

### 10.1 Metric Naming Convention

```
<namespace>_<subsystem>_<name>[_<unit>]

Examples:
- http_requests_total
- http_request_duration_seconds
- process_cpu_seconds_total
- memory_usage_bytes
- database_connections_active
```

| Component | Description |
|-----------|-------------|
| namespace | Application name (e.g., `phenotype`, `helios`) |
| subsystem | Logical grouping (e.g., `api`, `database`, `worker`) |
| name | What is being measured |
| unit | Optional unit of measurement |

### 10.2 Label Naming Reference

```python
# Standard labels across all services
standard_labels = {
    # Deployment
    "service": "Service name",
    "instance": "Specific instance identifier", 
    "namespace": "Deployment namespace",
    "pod": "Pod name (Kubernetes)",
    "version": "Service version",
    
    # Infrastructure
    "region": "Cloud region",
    "zone": "Availability zone",
    "host": "Hostname",
    
    # Application
    "method": "HTTP method or operation type",
    "path": "API path or endpoint",
    "status": "HTTP status or exit code",
    "error": "Error type if applicable",
}

# Cardinality guidance per label
label_cardinality = {
    "service": 100,           # Low - known set
    "namespace": 50,          # Low - known set
    "region": 10,             # Low - known set
    "instance": 1000,         # Medium - per deployment
    "pod": 10000,             # Medium-High - ephemeral
    "path": 500,              # Medium - defined routes
    "method": 10,             # Low - fixed set
    "status": 20,             # Low - HTTP codes
    "error": 50,              # Low - error types
}
```

### 10.3 Query Examples

```python
# Availability SLO
availability = '''
1 - (
  sum(rate(http_requests_total{status=~"5.."}[30d]))
  /
  sum(rate(http_requests_total[30d]))
)
'''

# Latency SLO  
latency_p95 = '''
histogram_quantile(0.95,
  sum(rate(http_request_duration_seconds_bucket[7d])) by (le)
)
'''

# Error rate by service
error_rate = '''
sum by (service) (
  rate(http_requests_total{status=~"5.."}[5m])
)
/
sum by (service) (
  rate(http_requests_total[5m])
)
'''

# Resource utilization trend
resource_trend = '''
avg by (service, resource) (
  rate(process_cpu_seconds_total[1h]) * 100
)
over (1d)
'''
```

---

## Appendix A: Glossary

| Term | Definition |
|------|------------|
| **Cardinality** | Number of unique combinations of label values |
| **Golden Signals** | The four key metrics: latency, traffic, errors, saturation |
| **PromQL** | Prometheus Query Language |
| **SLI** | Service Level Indicator - measurable metric |
| **SLO** | Service Level Objective - target value for SLI |
| **Error Budget** | Allowable margin for SLO failure |
| **Burn Rate** | Speed at which error budget is consumed |
| **Hot/Warm/Cold** | Storage tiers by age of data |

---

## Appendix B: References

- [Prometheus Metric Types](https://prometheus.io/docs/concepts/metric_types/)
- [SRE Workbook - Alerting on SLOs](https://sre.google/workbook/alerting/)
- [OpenTelemetry Metrics Specification](https://opentelemetry.io/docs/specs/otel/metrics/)
- [Grafana Query Best Practices](https://grafana.com/docs/grafana/latest/panels-visualizations/query-optimizations/)

---

*Document maintained by the Phenotype Architecture Team. For questions, contact #architecture on Slack.*
