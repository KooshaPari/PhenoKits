# ADR-001: Prometheus-Compatible Metrics Model

**Document ID:** PHENOTYPE_GAUGE_ADR_001  
**Status:** Accepted  
**Last Updated:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related ADRs:** [ADR-002](./ADR-002-opentelemetry.md), [ADR-003](./ADR-003-timeseries-storage.md), [ADR-004](./ADR-004-dashboard-stack.md), [ADR-005](./ADR-005-alerting-strategy.md), [ADR-006](./ADR-006-cardinality-management.md)

---

## Table of Contents

1. [Title](#title)
2. [Context](#context)
3. [Decision](#decision)
4. [Consequences](#consequences)
5. [Technical Details](#technical-details)
6. [Alternatives Considered](#alternatives-considered)
7. [Implementation Notes](#implementation-notes)
8. [Cross-References](#cross-references)

---

## Context

phenotype-gauge requires a metrics model that supports high-throughput ingestion, efficient storage, and flexible querying while maintaining compatibility with the broader observability ecosystem. The choice of metrics model affects every aspect of the system from collection to visualization.

### Requirements

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| R1 | Support counter, gauge, histogram, summary | Critical | Standard metric types |
| R2 | Label-based dimensional metrics | Critical | Essential for flexible querying |
| R3 | PromQL-compatible query interface | High | Ecosystem tooling compatibility |
| R4 | Efficient storage at scale | Critical | 100K+ metrics/sec ingestion |
| R5 | Range queries with aggregations | Critical | Historical analysis |
| R6 | Sub-10ms write latency | High | Real-time alerting |

### Constraints

- Must integrate with existing Phenotype observability tools
- Team has prior experience with Prometheus
- Existing dashboards use Grafana (PromQL-compatible)
- Cost constraints favor self-hosted solution

---

## Decision

We adopt a **Prometheus-compatible metrics model** as the foundation for phenotype-gauge. This includes the data model (metric name + labels + value + timestamp), the four metric types (counter, gauge, histogram, summary), and PromQL as the primary query language.

### Data Model

```python
# Canonical metric format
Metric = {
    "name": "http_requests_total",           # Metric name (required)
    "labels": {                              # Label set (optional, mutable)
        "method": "GET",                     # Label name must match [a-zA-Z_][a-zA-Z0-9_]*
        "status": "200",
        "path": "/api/users"
    },
    "value": 1450237,                        # Float64 value
    "timestamp": 1712188800                   # Unix timestamp (seconds)
}

# Histogram special case
HistogramMetric = {
    "name": "http_request_duration_seconds",
    "labels": {"method": "GET", "path": "/api/users"},
    "buckets": {                             # Cumulative counts
        "0.005": 12450,
        "0.01": 45230,
        "0.025": 89234,
        "0.05": 102456,
        "0.1": 105234,
        "0.25": 105890,
        "0.5": 105901,
        "1.0": 105902,
        "2.5": 105902,
        "5.0": 105902,
        "+Inf": 105902
    },
    "sum": 4523.45,                          # Sum of all values
    "count": 105902,                          # Total count
    "timestamp": 1712188800
}
```

### Metric Type Semantics

| Type | Behavior | Use Cases |
|------|----------|-----------|
| **Counter** | Monotonically increasing (reset on restart) | Request counts, error counts |
| **Gauge** | Can go up or down | Memory usage, queue depth |
| **Histogram** | Bucketed distribution | Latency, payload sizes |
| **Summary** | Client-calculated quantiles | Latency (when client-side needed) |

### Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                  Prometheus-Compatible Metrics Flow                    │
│                                                                     │
│  ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐ │
│  │ Application│     │ OTel    │     │ Collector│     │ Storage  │ │
│  │ /metrics │────▶│ SDK     │────▶│ Pipeline │────▶│  Engine  │ │
│  │ endpoint │     │         │     │          │     │          │ │
│  └──────────┘     └──────────┘     └──────────┘     └────┬─────┘ │
│                                                          │         │
│  ┌──────────┐     ┌──────────┐     ┌──────────┐          │         │
│  │ Grafana │◀────│ Query   │◀────│ PromQL   │◀─────────┘         │
│  │         │     │ API     │     │ Engine   │                    │
│  └──────────┘     └──────────┘     └──────────┘                    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Consequences

### Positive Consequences

1. **Ecosystem Compatibility** - Direct integration with Grafana, Prometheus tooling, and existing dashboards without translation layers.

2. **Familiar Developer Experience** - Phenotype developers experienced with Prometheus can apply their knowledge directly.

3. **Mature Tooling** - Access to a rich ecosystem of exporters, libraries, and visualization tools.

4. **PromQL Expressiveness** - Powerful aggregation, rate calculations, and prediction functions built over decades of production use.

5. **Federation Support** - Native support for hierarchical federation patterns used in multi-cluster deployments.

6. **Community Resources** - Extensive documentation, examples, and best practices available.

### Negative Consequences

1. **Cardinality Explosion Risk** - High-cardinality labels can cause storage and query performance issues without proper governance.

2. **Histogram Cardinality** - Histogram buckets multiply cardinality (10 buckets × label combinations).

3. **Prometheus Limitations** - Some limitations of the original Prometheus design (e.g., lack of native federation) require workaround via Thanos/Mimir.

4. **Pull Model Constraints** - While we support push for ephemeral workloads, pull is preferred, which requires accessible service endpoints.

5. **Query Complexity** - PromQL can become complex for advanced use cases, requiring specialized knowledge.

---

## Technical Details

### Label Naming Rules

```python
# Valid label names
valid_labels = [
    "method",
    "status_code",
    "path",
    "instance",
    "service_name",
    "region",
]

# Invalid label names (will cause errors)
invalid_labels = [
    "http-method",     # Contains hyphen
    "status.code",     # Contains dot
    "path/",           # Starts with special char
    "2xx",             # Starts with number
]

# Label value recommendations
label_value_guidance = {
    "high_cardinality_warning": [
        "user_id",     # UNBOUNDED - millions of users
        "request_id",  # UNBOUNDED - every request
        "trace_id",    # UNBOUNDED - every trace
        "query",       # UNBOUNDED - arbitrary SQL
    ],
    "low_cardinality_safe": [
        "method",      # Fixed set: GET, POST, PUT, DELETE
        "status",      # Fixed set: 200, 404, 500, etc.
        "path",        # Bounded: defined API routes
        "region",      # Fixed set: us-east-1, eu-west-1, etc.
    ],
}
```

### Rate Calculation

```python
# Instant rate (best for alerting)
rate(metric[5m])

# Increases over time range (for dashboards)
increase(metric[1h])

# Rate with minimal rounding error
rate(metric[5m])

# Histogram quantile calculation
histogram_quantile(0.95, 
    sum(rate(http_request_duration_seconds_bucket[5m])) by (le)
)
```

### Recording Rules

```yaml
groups:
  - name: api_performance
    interval: 60s
    rules:
      # Pre-compute per-service request rates
      - record: service:request_rate:rate5m
        expr: |
          sum by (service) (
            rate(http_requests_total[5m])
          )
      
      # Pre-compute error rates
      - record: service:error_rate:rate5m
        expr: |
          sum by (service, status) (
            rate(http_requests_total{status=~"5.."}[5m])
          )
          /
          sum by (service) (
            rate(http_requests_total[5m])
          )
      
      # Pre-compute latency quantiles
      - record: service:latency_p95:rate5m
        expr: |
          histogram_quantile(0.95,
            sum by (service, le) (
              rate(http_request_duration_seconds_bucket[5m])
            )
          )
```

---

## Alternatives Considered

### Alternative 1: InfluxDB Line Protocol

**Description:** Use InfluxDB's line protocol and InfluxQL/Flux as the query language.

**Pros:**
- Native tagging model with better cardinality handling
- Downsampling built into the query language
- retention_policy() for tiering

**Cons:**
- Less ecosystem tooling (Grafana requires Flux which is less mature)
- Different paradigm than what team knows
- Proprietary retention policies complicate federation

**Why Rejected:** PromQL ecosystem is more mature and team has existing expertise. The cardinality management challenges can be addressed with governance.

### Alternative 2: Datadog Metrics Model

**Description:** Use Datadog's metrics model with MQL as the query language.

**Pros:**
- Excellent native cardinality handling
- Unified metrics + logs + traces
- Host-based billing model

**Cons:**
- Vendor lock-in
- Proprietary API
- Cost scales unpredictably with cardinality
- Team wants open-source solution

**Why Rejected:** Vendor lock-in contradicts Phenotype's open-source philosophy. Self-hosted solution required for cost control.

### Alternative 3: Custom Metrics Model

**Description:** Design a new metrics model optimized specifically for phenotype-gauge.

**Pros:**
- Full control over design
- Can optimize for specific use cases
- No backwards compatibility burden

**Cons:**
- Requires building all tooling from scratch
- No ecosystem integration
- Steep learning curve for new users
- High development cost

**Why Rejected:** Building an ecosystem is not the core focus. Leveraging PromQL provides immediate compatibility with decades of tooling development.

---

## Implementation Notes

### Phase 1: Foundation (Q2 2026)

1. Implement core metric types (counter, gauge, histogram)
2. Build PromQL parser and evaluator
3. Create storage engine with TSDB-compatible block format
4. Implement basic query API (/api/v1/query, /api/v1/query_range)

### Phase 2: Advanced Features (Q3 2026)

1. Add histogram_quantile support
2. Implement recording rules
3. Add subquery support
4. Build federation API (/federate)

### Phase 3: Production Readiness (Q4 2026)

1. Add PromQL function library (predict_linear, clamp, etc.)
2. Implement query performance optimization
3. Add query caching layer
4. Benchmark and tune for 100K metrics/sec

### Key Libraries

| Component | Library | Version | Purpose |
|-----------|---------|---------|---------|
| PromQL Engine | prometheus/promql | Latest | Query execution |
| TSDB | prometheus/tsdb | Latest | Storage format |
| Parser | prometheus/promql/parser | Latest | PromQL parsing |
| Functions | prometheus/promql/functions | Latest | Built-in functions |

---

## Cross-References

- **ADR-002:** [ADR-002-opentelemetry.md](./ADR-002-opentelemetry.md) - OTel protocol support for collection
- **ADR-003:** [ADR-003-timeseries-storage.md](./ADR-003-timeseries-storage.md) - Storage engine architecture
- **ADR-004:** [ADR-004-dashboard-stack.md](./ADR-004-dashboard-stack.md) - Visualization with Grafana
- **ADR-005:** [ADR-005-alerting-strategy.md](./ADR-005-alerting-strategy.md) - Alerting integration
- **ADR-006:** [ADR-006-cardinality-management.md](./ADR-006-cardinality-management.md) - Cardinality governance
- **SOTA Research:** [MONITORING_SOTA.md](./MONITORING_SOTA.md) - Full monitoring landscape analysis
- **Metrics Reference:** [metrics.md](./metrics.md) - Detailed metrics definitions

---

*This ADR was accepted on 2026-04-04 by the Phenotype Architecture Team.*
