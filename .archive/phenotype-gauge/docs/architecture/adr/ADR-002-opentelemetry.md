# ADR-002: OpenTelemetry Protocol Support

**Document ID:** PHENOTYPE_GAUGE_ADR_002  
**Status:** Accepted  
**Last Updated:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related ADRs:** [ADR-001](./ADR-001-prometheus-metrics-model.md), [ADR-003](./ADR-003-timeseries-storage.md), [ADR-004](./ADR-004-dashboard-stack.md)

---

## Context

The observability landscape has converged on OpenTelemetry (OTel) as the unified standard for telemetry data. Supporting OTLP (OpenTelemetry Protocol) is essential for phenotype-gauge to integrate seamlessly with modern instrumentation libraries and collectors.

### Requirements

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| R1 | Native OTLP receiver | Critical | gRPC and HTTP |
| R2 | Prometheus exposition format | High | Legacy compatibility |
| R3 | Metric type translation | Critical | OTel → internal model |
| R4 | Context propagation | High | Trace correlation |
| R5 | Backpressure handling | Medium | Rate limiting |

### Constraints

- Must maintain PromQL compatibility for queries
- OTLP metrics differ semantically from Prometheus model
- Historical data must remain queryable during migration

---

## Decision

We implement **native OTLP support** as the primary collection protocol while maintaining Prometheus exposition compatibility. The collector will translate OTel metrics to the internal Prometheus-compatible model.

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    OpenTelemetry Collection Flow                          │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        SDK Side                                   │   │
│  │                                                                  │   │
│  │  Application                                                     │   │
│  │       │                                                         │   │
│  │       ▼                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────┐  │   │
│  │  │                    OTel SDK                               │  │   │
│  │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐           │  │   │
│  │  │  │  Metrics  │  │   Logs    │  │  Traces   │           │  │   │
│  │  │  │  API      │  │   API     │  │  API      │           │  │   │
│  │  │  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘           │  │   │
│  │  │        └───────────────┼───────────────┘                  │  │   │
│  │  │                          ▼                               │  │   │
│  │  │               ┌─────────────────┐                       │  │   │
│  │  │               │  OTLP Exporter  │                       │  │   │
│  │  │               │  (gRPC/HTTP)    │                       │  │   │
│  │  │               └────────┬────────┘                       │  │   │
│  │  └───────────────────────┼──────────────────────────────┘  │   │
│  └───────────────────────────┼────────────────────────────────────┘   │
│                              │                                          │
│  ┌───────────────────────────┼────────────────────────────────────┐   │
│  │                     Collector Side                              │   │
│  │                           ▼                                    │   │
│  │                   ┌─────────────────┐                          │   │
│  │                   │  OTLP Receiver │                          │   │
│  │                   │  (gRPC:4317)   │                          │   │
│  │                   │  (HTTP:4318)   │                          │   │
│  │                   └────────┬────────┘                          │   │
│  │                            │                                   │   │
│  │                            ▼                                   │   │
│  │  ┌────────────────────────────────────────────────────────┐  │   │
│  │  │              Translation Layer                           │  │   │
│  │  │                                                         │  │   │
│  │  │  OTel Metric          →    Internal Prometheus Model    │  │   │
│  │  │  ──────────────────────────                            │  │   │
│  │  │  sum(simple)          →    counter (additive=true)    │  │   │
│  │  │  gauge(simple)        →    gauge (additive=false)     │  │   │
│  │  │  histogram            →    histogram                   │  │   │
│  │  │  exponential_histogram→    histogram (bucket-aware)   │  │   │
│  │  │  summary              →    summary                     │  │   │
│  │  │                                                         │  │   │
│  │  │  Resource.attributes   →    metric labels               │  │   │
│  │  │  Scope.attributes     →    metric labels               │  │   │
│  │  │  InstrumentationScope →    labels (optional)            │  │   │
│  │  │                                                         │  │   │
│  │  └────────────────────────────────────────────────────────┘  │   │
│  │                            │                                   │   │
│  │                            ▼                                   │   │
│  │  ┌────────────────────────────────────────────────────────┐  │   │
│  │  │              Phenotype Gauge Storage                     │  │   │
│  │  └────────────────────────────────────────────────────────┘  │   │
│  └────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### OTLP to Prometheus Mapping

| OTel Metric Type | Prometheus Type | Special Handling |
|------------------|-----------------|-----------------|
| Sum (monotonic, additive=true) | Counter | Sum as value, cumulative behavior |
| Sum (monotonic, additive=false) | Gauge | Sum as value |
| Gauge | Gauge | Direct mapping |
| Histogram | Histogram | Buckets preserved, sum/count preserved |
| ExponentialHistogram | Histogram | Exponential bucket mapping |
| Summary | Summary | Quantiles and count preserved |

### Resource and Scope Mapping

```python
# OpenTelemetry Resource
resource = {
    "attributes": {
        "service.name": "api-gateway",
        "service.version": "1.2.3",
        "deployment.environment": "production",
        "cloud.region": "us-east-1",
        "host.name": "api-prod-01",
    }
}

# Becomes Prometheus labels
prometheus_labels = {
    "service_name": "api-gateway",
    "service_version": "1.2.3",
    "deployment_environment": "production",
    "cloud_region": "us-east-1",
    "host_name": "api-prod-01",
}

# OTel Scope
scope = {
    "name": "io.phenotype.api",
    "version": "2.0.0",
    "attributes": {
        "instrumentation_library": "api-middleware",
    }
}

# Optional: Add as labels
scope_labels = {
    "otel_scope_name": "io.phenotype.api",
    "otel_scope_version": "2.0.0",
}
```

---

## Consequences

### Positive Consequences

1. **Universal Instrumentation** - Any OTel-compatible library can emit metrics directly without an intermediary.

2. **Future-Proof** - OTel is the emerging standard; support ensures long-term viability.

3. **Reduced Collection Overhead** - Native OTel SDKs are optimized for minimal performance impact.

4. **Multi-Signal Support** - OTel's unified model can eventually extend to traces and logs.

5. **Ecosystem Integration** - Direct compatibility with collectors like OTel Collector, DataDog Agent, etc.

### Negative Consequences

1. **Semantic Differences** - OTel metrics don't map 1:1 to Prometheus, requiring careful translation.

2. **Exponential Histogram Handling** - Novel OTel histogram type requires custom handling.

3. **Attribute Limits** - OTel allows complex attribute values that need sanitization for Prometheus labels.

4. **Additional Dependencies** - OTel SDK and protocol buffers add to the codebase.

---

## Technical Details

### OTLP HTTP/JSON Decoding

```python
# OTLP metrics JSON structure (simplified)
otlp_payload = {
    "resourceMetrics": [{
        "resource": {
            "attributes": [
                {"key": "service.name", "value": {"stringValue": "api"}},
            ]
        },
        "scopeMetrics": [{
            "scope": {
                "name": "io.phenotype.api",
                "version": "1.0.0",
            },
            "metrics": [{
                "name": "http_requests_total",
                "description": "Total HTTP requests",
                "unit": "1",
                "sum": {
                    "dataPoints": [{
                        "asInt": "1450237",
                        "timeUnixNano": "1712188800000000000",
                        "attributes": [
                            {"key": "method", "value": {"stringValue": "GET"}},
                            {"key": "status", "value": {"intValue": "200"}},
                        ]
                    }],
                    "aggregationTemporality": "CUMULATIVE",
                    "isMonotonic": True,
                }
            }]
        }]
    }]
}

# Translation to internal model
def translate_otlp_metric(otlp_metric):
    metric_name = sanitize_metric_name(otlp_metric["name"])
    
    # Sum → Counter
    if "sum" in otlp_metric:
        return {
            "name": metric_name,
            "type": "counter",
            "labels": extract_labels(otlp_metric),
            "value": int(otlp_metric["sum"]["dataPoints"][0]["asInt"]),
            "timestamp": unix_nano_to_seconds(
                otlp_metric["sum"]["dataPoints"][0]["timeUnixNano"]
            ),
        }
    
    # Gauge
    if "gauge" in otlp_metric:
        return {
            "name": metric_name,
            "type": "gauge",
            "labels": extract_labels(otlp_metric),
            "value": otlp_metric["gauge"]["dataPoints"][0]["asDouble"],
            "timestamp": unix_nano_to_seconds(...),
        }
    
    # Histogram
    if "histogram" in otlp_metric:
        return translate_histogram(otlp_metric["histogram"])
```

### Supported Temporality

| OTel Temporality | Description | Mapped To |
|------------------|-------------|-----------|
| CUMULATIVE | Cumulative since start | Prometheus (assumed cumulative) |
| DELTA | Change since last | Prometheus rate() assumes cumulative |
| ALTERNATING | Mixed | Supported, tracked per-metric |

### Backpressure Configuration

```yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
        max_recv_msg_size: 16777216  # 16MB
        max_concurrent_streams: 100
        
      http:
        endpoint: 0.0.0.0:4318
        max_recv_msg_size: 16777216
        
    # Backpressure settings
    sending_queue:
      queue_size: 10000
      num_consumers: 10
      
    # Retry settings
    retry_on_failure:
      enabled: true
      initial_interval: 5s
      max_interval: 30s
      max_elapsed_time: 300s
```

---

## Alternatives Considered

### Alternative 1: Prometheus Push Gateway Only

**Description:** Only support Prometheus exposition format via a push gateway.

**Pros:**
- Simpler implementation
- Direct Prometheus compatibility

**Cons:**
- Doesn't support OTel SDKs natively
- Push model limitations for long-lived services
- No multi-signal support

**Why Rejected:** Modern instrumentation uses OTel SDKs; lack of native support would require users to maintain translation layers.

### Alternative 2: Full OTel Collector Deployment

**Description:** Deploy a separate OTel Collector in front of phenotype-gauge.

**Pros:**
- Native OTel support
- Rich processing capabilities

**Cons:**
- Additional infrastructure component
- Double-hop latency for metrics
- Separate operational burden

**Why Rejected:** We can implement OTLP receiver directly in phenotype-gauge, reducing operational complexity while maintaining compatibility.

---

## Implementation Notes

### Phase 1: Core OTLP Support (Q2 2026)

1. Implement OTLP/HTTP receiver with JSON decoding
2. Implement OTLP/gRPC receiver with protobuf decoding
3. Build metric type translator
4. Add resource and scope attribute extraction

### Phase 2: Advanced Features (Q3 2026)

1. Add exponential histogram support
2. Implement temporality handling
3. Add context propagation for trace correlation
4. Build metric metadata preservation

### Phase 3: Optimization (Q4 2026)

1. Add streaming aggregation
2. Implement adaptive batching
3. Build backpressure signaling
4. Performance benchmarking

---

## Cross-References

- **ADR-001:** [ADR-001-prometheus-metrics-model.md](./ADR-001-prometheus-metrics-model.md) - Internal metrics model
- **ADR-003:** [ADR-003-timeseries-storage.md](./ADR-003-timeseries-storage.md) - Storage architecture
- **ADR-004:** [ADR-004-dashboard-stack.md](./ADR-004-dashboard-stack.md) - Visualization
- **OTel Specification:** https://opentelemetry.io/docs/specs/otel/metrics/
- **OTLP Specification:** https://opentelemetry.io/docs/specs/otel/protocol/

---

*This ADR was accepted on 2026-04-04 by the Phenotype Architecture Team.*
