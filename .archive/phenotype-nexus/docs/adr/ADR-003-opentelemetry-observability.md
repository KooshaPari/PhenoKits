# ADR-003: Observability Strategy with OpenTelemetry

**Status**: Accepted  
**Date**: 2026-04-05  
**Deciders**: phenotype-nexus Architecture Team  
**Category**: Architecture / Observability

---

## Context

phenotype-nexus must provide comprehensive observability for:
- Service mesh operations (traffic flows, policy enforcement)
- Control plane health (consensus state, API performance)
- Data plane performance (eBPF metrics, latency distributions)
- Security events (mTLS handshakes, policy violations)
- Infrastructure (Kubernetes, node resources)

The observability stack must:
- Support multiple signal types (metrics, logs, traces)
- Integrate with existing enterprise monitoring systems
- Provide real-time and historical analysis
- Minimize overhead on the data plane
- Follow cloud-native standards

---

## Decision

**We will adopt OpenTelemetry (OTel) as the unified observability framework**, using:
- **OpenTelemetry Collector** for agent-side collection
- **OTLP (OpenTelemetry Protocol)** for transport
- **Prometheus** for metrics storage and alerting
- **Tempo** for distributed tracing
- **Loki** for log aggregation
- **Grafana** for visualization

### Rationale

| Approach | Standards | Vendor Lock-in | Overhead | Maturity | Our Assessment |
|----------|-----------|----------------|----------|----------|----------------|
| **OpenTelemetry** | Yes (CNCF) | None | Low | Production | **Selected** |
| Prometheus Only | Partial | None | Low | Production | Incomplete |
| Jaeger Only | Partial | None | Medium | Production | Traces only |
| ELK Stack | No | Elastic | High | Production | Resource heavy |
| Datadog | No | High | Medium | Production | Cost prohibitive |
| Custom Solution | No | None | Unknown | Development | Rejected |

OpenTelemetry was selected because:
1. **Vendor neutrality**: Export to any backend (Prometheus, Jaeger, vendor SaaS)
2. **Unified signals**: Single SDK for metrics, logs, traces
3. **Industry standard**: Backed by CNCF, supported by all major vendors
4. **eBPF integration**: OTel can consume eBPF-generated telemetry
5. **Future-proof**: Rapidly becoming the de facto standard

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                           Observability Pipeline                                     │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                      │
│  ┌───────────────────────────────────────────────────────────────────────────────┐  │
│  │                         Data Sources                                           │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │  │
│  │  │ eBPF Data    │  │ Control      │  │ Application  │  │ Kubernetes   │       │  │
│  │  │ Plane        │  │ Plane        │  │ Workloads    │  │ Infrastructure│      │  │
│  │  │              │  │              │  │              │  │              │       │  │
│  │  │ - Flow logs  │  │ - Raft       │  │ - Auto-      │  │ - Node       │       │  │
│  │  │ - Latency    │  │   metrics    │  │   instrument │  │   metrics    │       │  │
│  │  │ - Drop       │  │ - API        │  │ - Custom     │  │ - Pod events │       │  │
│  │  │   reasons    │  │   latency    │  │   spans      │  │ - K8s events │       │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘       │  │
│  └───────────────────────────────────────────────────────────────────────────────┘  │
│                                     │                                                │
│  ┌──────────────────────────────────▼───────────────────────────────────────────┐  │
│  │                    OpenTelemetry Collector (DaemonSet)                        │  │
│  │                                                                              │  │
│  │   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌────────────┐  │  │
│  │   │   Receivers│    │  Processors │    │   Exporters │    │ Extensions │  │  │
│  │   │            │───►│             │───►│             │    │            │  │  │
│  │   │ • OTLP     │    │ • Batch     │    │ • Prometheus│    │ • Health   │  │  │
│  │   │ • eBPF     │    │ • Memory    │    │ • OTLP      │    │ • Pprof    │  │  │
│  │   │   (custom) │    │   limiter   │    │ • File      │    │ • zPages   │  │  │
│  │   │ • Prometheus│    │ • Resource  │    │ • Kafka     │    │            │  │  │
│  │   │   scrape   │    │ • Attributes│    │   (buffer)  │    │            │  │  │
│  │   └─────────────┘    └─────────────┘    └─────────────┘    └────────────┘  │  │
│  │                                                                              │  │
│  └──────────────────────────────────────────────────────────────────────────────┘  │
│                                     │                                                │
│  ┌──────────────────────────────────┼───────────────────────────────────────────────┐  │
│  │                                  ▼                                               │  │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │  │
│  │  │   Prometheus  │  │     Tempo       │  │     Loki        │  │   Grafana    │ │  │
│  │  │               │  │                 │  │                 │  │              │ │  │
│  │  │  Time-series  │  │  Trace storage  │  │  Log storage    │  │ Visualization│ │  │
│  │  │  Metrics      │  │  (object store) │  │  (object store) │  │  Dashboards  │ │  │
│  │  │               │  │                 │  │                 │  │              │ │  │
│  │  │  • AlertManager│  │  • Jaeger UI   │  │  • LogQL       │  │  • Unified   │ │  │
│  │  │  • Recording   │  │  • Trace       │  │  • Log alerts  │  │    view      │ │  │
│  │  │    rules       │  │    search      │  │                │  │              │ │  │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────┘  └──────────────┘ │  │
│  │                                                                                  │  │
│  └──────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                      │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

### Key Components

1. **eBPF Telemetry Generation**: Custom eBPF programs emit metrics directly from kernel
   - Flow logs with 5-tuple + metadata
   - Latency histograms (P50, P95, P99)
   - Drop/packet loss reasons
   - TLS handshake events

2. **OpenTelemetry Collector**: Per-node agent processing telemetry
   - OTLP receivers for application spans
   - Custom eBPF receiver (via Unix socket)
   - Batch processing for efficiency
   - Resource attribution (pod, node, namespace)

3. **Backend Storage**:
   - **Prometheus**: 15s scrape interval, 15-day local retention
   - **Thanos/VictoriaMetrics**: Long-term storage, global query view
   - **Tempo**: Trace storage with S3/GCS backend
   - **Loki**: Log storage with object store backend

4. **Visualization**: Grafana with curated dashboards
   - Service mesh overview (traffic, latency, errors)
   - Control plane health (Raft, API performance)
   - Security dashboard (mTLS status, policy violations)
   - Infrastructure (nodes, pods, resource usage)

---

## Signal Types and Implementation

### Metrics

| Metric Category | Source | Collection | Storage |
|-----------------|--------|------------|---------|
| Service traffic | eBPF flow logs | OTel Collector → Prometheus | Prometheus |
| Request latency | eBPF sockops | Histogram in eBPF, export | Prometheus |
| Error rates | eBPF drop events | Counter aggregation | Prometheus |
| Control plane | Go runtime metrics | Prometheus scrape | Prometheus |
| Certificate expiry | Certificate Manager | Gauge metric | Prometheus |
| Resource usage | cAdvisor/kubelet | Prometheus scrape | Prometheus |

### Traces

| Trace Source | Instrumentation | Sampling | Backend |
|--------------|-----------------|----------|---------|
| Service mesh | eBPF-generated span context | 1% default, 100% for errors | Tempo |
| Control plane | OpenTelemetry SDK | 100% (low volume) | Tempo |
| Applications | OpenTelemetry auto-instrumentation | Configurable | Tempo |

### Logs

| Log Source | Collection | Parsing | Retention |
|------------|------------|---------|-----------|
| eBPF events | eBPF ring buffer → OTel | Structured (JSON) | 7 days hot, 30 days cold |
| Control plane | stdout → OTel | Structured | 7 days hot, 90 days cold |
| Kubernetes | API server → OTel | JSON | 30 days |
| Audit | Policy enforcement logs | Structured | 1 year (compliance) |

---

## Consequences

### Positive

1. **Unified Instrumentation**: Single OTel SDK for all signals
2. **Vendor Flexibility**: Export to any backend without re-instrumentation
3. **eBPF Integration**: Direct kernel telemetry without userspace overhead
4. **Standardized Context**: W3C Trace Context propagation across services
5. **Rich Ecosystem**: Pre-built Grafana dashboards, AlertManager rules
6. **Cost Efficiency**: Open source stack vs. expensive SaaS alternatives

### Negative

1. **Component Complexity**: 4+ components to manage (Collector, Prometheus, Tempo, Loki, Grafana)
2. **Storage Costs**: High-cardinality metrics can explode storage
3. **Cardinality Limits**: eBPF flow logs need aggregation to stay within limits
4. **Learning Curve**: OTel configuration is complex
5. **Debugging Difficulty**: Distributed pipeline harder to debug than single system

### Mitigations

| Risk | Mitigation | Implementation |
|------|------------|----------------|
| High cardinality | Aggressive label filtering, recording rules | OTel processors |
| Storage costs | Tiered retention, compression | S3 lifecycle policies |
| Complexity | Helm charts, operator automation | phenotype-nexus-operator |
| Debugging | Structured logging, tracing pipeline | Jaeger for OTel debugging |
| Cardinality limits | Metric relabeling, aggregation | Prometheus recording rules |

---

## Alternatives Considered

### Option A: Prometheus + Jaeger + ELK (Pre-OTel)
- **Pros**: Mature, well-understood
- **Cons**: Three separate pipelines, no correlation, heavy resource usage
- **Verdict**: Rejected; OTel unifies and modernizes

### Option B: Vendor SaaS (Datadog, New Relic)
- **Pros**: Fully managed, rich features
- **Cons**: Expensive at scale, vendor lock-in, data egress costs
- **Verdict**: Rejected; open source maintains control and reduces cost

### Option C: Custom Pipeline
- **Pros**: Tailored exactly to our needs
- **Cons**: Years of development, miss ecosystem benefits
- **Verdict**: Rejected; OTel is the industry standard

### Option D: eBPF-only Observability (Pixie)
- **Pros**: Zero instrumentation, kernel-level visibility
- **Cons**: Limited to supported protocols, no custom business metrics
- **Verdict**: Partial; complement with OTel for full coverage

---

## Implementation Phases

| Phase | Deliverable | Components | Timeline |
|-------|-------------|------------|----------|
| Phase 1 | Metrics foundation | eBPF metrics → Prometheus → Grafana | 2026-Q1 |
| Phase 2 | Distributed tracing | Trace context propagation → Tempo | 2026-Q2 |
| Phase 3 | Log aggregation | Loki integration, unified search | 2026-Q2 |
| Phase 4 | Alerting | AlertManager, SLO-based alerts | 2026-Q3 |
| Phase 5 | Advanced analytics | Derived metrics, anomaly detection | 2026-Q4 |

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Metrics overhead | <1% CPU | eBPF CPU usage |
| Trace overhead | <5% latency | Instrumentation delay |
| Log collection delay | <1s end-to-end | Timestamp diff |
| Dashboard load time | <3s | Grafana UI |
| Query response (metrics) | <2s | PromQL queries |
| Query response (traces) | <5s | TraceQL queries |
| Query response (logs) | <10s | LogQL queries |

---

## Dashboard Specifications

### Service Mesh Overview Dashboard

| Panel | Query | Refresh |
|-------|-------|---------|
| Request rate | `sum(rate(http_requests_total[5m]))` | 10s |
| Error rate | `sum(rate(http_requests_total{status=~"5.."}[5m]))` | 10s |
| Latency P99 | `histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m]))` | 10s |
| Active connections | `sum(cilium_tcp_connections)` | 10s |
| mTLS status | `sum(cilium_mtls_handshakes) / sum(cilium_total_connections)` | 30s |
| Top error services | `topk(10, sum by (service) (rate(http_errors[5m])))` | 30s |

### Control Plane Health Dashboard

| Panel | Query | Alert Threshold |
|-------|-------|-----------------|
| Raft proposals | `rate(etcd_server_proposals_applied_total[5m])` | <100/s |
| API latency | `histogram_quantile(0.99, rate(grpc_server_handling_seconds_bucket[5m]))` | >100ms |
| Certificate expiry | `phenotype_nexus_cert_expiry_timestamp - time()` | <7 days |
| Policy compilation time | `phenotype_nexus_policy_compile_seconds` | >1s |
| xDS push latency | `phenotype_nexus_xds_push_duration_seconds` | >5s |

---

## References

| Reference | URL | Description |
|-----------|-----|-------------|
| OpenTelemetry | https://opentelemetry.io/docs/ | Official documentation |
| OTLP Protocol | https://opentelemetry.io/docs/reference/specification/protocol/ | Wire format |
| eBPF Observability | https://ebpf.io/applications/ | eBPF tools landscape |
| OpenTelemetry Collector | https://opentelemetry.io/docs/collector/ | Collector configuration |
| Tempo Documentation | https://grafana.com/docs/tempo/latest/ | Trace storage |
| Loki Documentation | https://grafana.com/docs/loki/latest/ | Log aggregation |
| W3C Trace Context | https://www.w3.org/TR/trace-context/ | Propagation standard |
| Prometheus Recording Rules | https://prometheus.io/docs/prometheus/latest/configuration/recording_rules/ | Metric aggregation |

---

## Related Decisions

- ADR-001: eBPF-Based Data Plane Architecture
- ADR-002: Control Plane Architecture with Raft Consensus

---

*Last updated: 2026-04-05*  
*Next review: 2026-07-05*
