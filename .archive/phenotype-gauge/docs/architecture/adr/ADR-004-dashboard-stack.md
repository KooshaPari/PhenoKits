# ADR-004: Dashboard and Visualization Stack

**Document ID:** PHENOTYPE_GAUGE_ADR_004  
**Status:** Accepted  
**Last Updated:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related ADRs:** [ADR-001](./ADR-001-prometheus-metrics-model.md), [ADR-003](./ADR-003-timeseries-storage.md), [ADR-005](./ADR-005-alerting-strategy.md)

---

## Context

Effective monitoring requires powerful visualization capabilities. phenotype-gauge needs a dashboard solution that supports real-time metrics, complex queries, and intuitive exploration while maintaining cost efficiency and operational simplicity.

### Requirements

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| R1 | PromQL-compatible queries | Critical | Direct compatibility |
| R2 | Real-time updates | High | <5s refresh for critical dashboards |
| R3 | Variable and templating | High | Reusable dashboards |
| R4 | Alert visualization | High | Alert state display |
| R5 | Multi-tenant views | Medium | Tenant isolation |
| R6 | Role-based access | Medium | Team permissions |

---

## Decision

We adopt **Grafana** as the primary visualization layer for phenotype-gauge, with Prometheus-compatible data sources for direct PromQL support and advanced alerting integration.

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Dashboard Architecture                            │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                      Grafana Stack                                │ │
│  │                                                                  │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │ │
│  │  │   Browser   │  │   Browser   │  │   Browser   │             │ │
│  │  │  Dashboard  │  │  Dashboard  │  │  Dashboard  │             │ │
│  │  │  User A     │  │  User B     │  │  User C     │             │ │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │ │
│  │         │                │                │                     │ │
│  │         └────────────────┼────────────────┘                     │ │
│  │                          │ HTTPS                                  │ │
│  │                          ▼                                        │ │
│  │  ┌─────────────────────────────────────────────────────────────┐│ │
│  │  │                    Grafana Server                            ││ │
│  │  │                                                             ││ │
│  │  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐ ││ │
│  │  │  │  Auth (OAuth) │  │    Dashboards │  │   Alerting   │ ││ │
│  │  │  │   & RBAC      │  │   & Panels    │  │   Manager    │ ││ │
│  │  │  └───────────────┘  └───────────────┘  └───────────────┘ ││ │
│  │  │                                                             ││ │
│  │  │  ┌───────────────────────────────────────────────────────┐ ││ │
│  │  │  │              Data Source Proxy                        │ ││ │
│  │  │  │   (PromQL → Internal API translation)                │ ││ │
│  │  │  └───────────────────────────────────────────────────────┘ ││ │
│  │  │                                                             ││ │
│  │  └─────────────────────────────────────────────────────────────┘│ │
│  │                          │                                       │ │
│  └──────────────────────────┼───────────────────────────────────────┘ │
│                             │                                           │
│                             ▼                                           │
│  ┌──────────────────────────────────────────────────────────────────┐│
│  │                    Phenotype Gauge API                            ││
│  │                                                                   ││
│  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐   ││
│  │  │  /api/v1/query │  │/api/v1/query_range│ │ /api/v1/series│   ││
│  │  └────────────────┘  └────────────────┘  └────────────────┘   ││
│  │                                                                   ││
│  └──────────────────────────────────────────────────────────────────┘│
│                             │                                           │
│                             ▼                                           │
│  ┌──────────────────────────────────────────────────────────────────┐│
│  │                    Storage Engine                                  ││
│  │                                                                   ││
│  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐   ││
│  │  │      Hot       │  │      Warm      │  │      Cold      │   ││
│  │  │     (SSD)      │  │      (HDD)     │  │       (S3)     │   ││
│  │  └────────────────┘  └────────────────┘  └────────────────┘   ││
│  │                                                                   ││
│  └──────────────────────────────────────────────────────────────────┘│
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Dashboard Types

| Type | Purpose | Refresh Rate | Examples |
|------|---------|-------------|----------|
| **Real-time** | Live monitoring | 5-15s | Current errors, request rate |
| **Overview** | Service health | 30-60s | SLO dashboards, capacity |
| **Analysis** | Deep dive | Manual | Debugging, trends |
| **Audit** | Compliance | Daily/hourly | Historical reports |

---

## Consequences

### Positive Consequences

1. **Mature Ecosystem** - Grafana has extensive documentation, plugins, and community.

2. **PromQL Native** - Direct support for PromQL queries without translation.

3. **Rich Visualizations** - Extensive panel types: graphs, tables, heatmaps, gauges.

4. **Alerting Integration** - Unified dashboard + alerting experience.

5. **Team Familiarity** - Phenotype team has existing Grafana experience.

### Negative Consequences

1. **Resource Usage** - Grafana can be memory-intensive at scale.

2. **Dashboard Proliferation** - Without governance, dashboards multiply.

3. **Query Complexity** - Users may write inefficient PromQL causing load.

4. **Maintenance Overhead** - Dashboard drift requires active management.

---

## Technical Details

### Panel Types

| Panel | Use Case | Performance |
|-------|----------|--------------|
| **Time series** | Trends over time | Excellent |
| **Stat** | Single value display | Excellent |
| **Gauge** | Progress/metrics | Excellent |
| **Table** | Multi-dimensional data | Good |
| **Heatmap** | Distribution visualization | Good |
| **Logs** | Log exploration | Requires Loki |
| **Traces** | Distributed tracing | Requires Jaeger |

### Dashboard Template Variables

```yaml
# Common template variables for phenotype-gauge
variables:
  - name: service
    type: query
    query: label_values(http_requests_total, service)
    
  - name: environment
    type: query
    query: label_values(http_requests_total{job=~"$service"}, environment)
    # Default: production
    
  - name: region
    type: query
    query: label_values(http_requests_total{environment="$environment"}, region)
    
  - name: interval
    type: interval
    options: [30s, 1m, 5m, 15m, 30m, 1h]
    default: 1m
```

### Standard Dashboard Panels

```python
# Golden Signal Dashboard Template
panels = [
    {
        "title": "Request Rate",
        "type": "timeseries",
        "targets": [{
            "expr": 'sum by (service) (rate(http_requests_total[$interval]))',
            "legendFormat": "{{service}}"
        }],
        "fieldConfig": {
            "defaults": {
                "unit": "reqps",
                "color": {"mode": "palette-classic"}
            }
        }
    },
    {
        "title": "Error Rate",
        "type": "timeseries", 
        "targets": [{
            "expr": '''
                sum by (service) (
                    rate(http_requests_total{status=~"5.."}[$interval])
                )
                /
                sum by (service) (
                    rate(http_requests_total[$interval])
                )
            ''',
            "legendFormat": "{{service}}"
        }],
        "fieldConfig": {
            "defaults": {
                "unit": "percentunit",
                "thresholds": {
                    "steps": [
                        {"value": 0, "color": "green"},
                        {"value": 0.001, "color": "yellow"},  # 0.1%
                        {"value": 0.01, "color": "red"}       # 1%
                    ]
                }
            }
        }
    },
    {
        "title": "p95 Latency",
        "type": "timeseries",
        "targets": [{
            "expr": '''
                histogram_quantile(0.95,
                    sum by (service, le) (
                        rate(http_request_duration_seconds_bucket[$interval])
                    )
                )
            ''',
            "legendFormat": "{{service}}"
        }],
        "fieldConfig": {
            "defaults": {
                "unit": "s",
                "thresholds": {
                    "steps": [
                        {"value": 0, "color": "green"},
                        {"value": 0.2, "color": "yellow"},   # 200ms
                        {"value": 0.5, "color": "red"}       # 500ms
                    ]
                }
            }
        }
    },
    {
        "title": "Saturation (CPU/Memory)",
        "type": "gauge",
        "targets": [{
            "expr": 'avg by (service) (container_cpu_usage_seconds_total)',
            "legendFormat": "{{service}}"
        }],
        "fieldConfig": {
            "defaults": {
                "unit": "percentunit",
                "max": 1,
                "thresholds": {
                    "steps": [
                        {"value": 0, "color": "green"},
                        {"value": 0.7, "color": "yellow"},
                        {"value": 0.9, "color": "red"}
                    ]
                }
            }
        }
    }
]
```

### Grafana Provisioning

```yaml
# Infrastructure as Code for dashboards
apiVersion: 1

providers:
  - name: 'phenotype-dashboards'
    orgId: 1
    folder: 'Phenotype'
    folderUid: 'phenotype'
    type: file
    updateIntervalSeconds: 30
    allowUiUpdates: true
    options:
      path: /var/lib/grafana/dashboards/phenotype

datasources:
  - name: Phenotype Gauge
    type: prometheus
    access: proxy
    url: https://gauge.phenotype.dev
    isDefault: true
    jsonData:
      httpMethod: POST
      timeInterval: 15s
      prometheusType: Prometheus
      prometheusVersion: latest
    secureJsonData:
      httpHeaderValue1: "Bearer ${GAUGE_API_TOKEN}"
```

---

## Alternatives Considered

### Alternative 1: Custom React Dashboard

**Description:** Build a custom visualization UI using React and Recharts.

**Pros:**
- Full control over UX
- Tailored to phenotype-gauge
- No licensing concerns

**Cons:**
- Significant development effort
- Reinventing features Grafana provides
- Less community support

**Why Rejected:** Development cost too high. Grafana provides excellent capabilities out of the box.

### Alternative 2: Graphite + Grafana

**Description:** Use Graphite's Carbon/whisper storage with Grafana.

**Pros:**
- Proven at scale (Discord, Wikimedia)
- Whisper is simple and reliable

**Cons:**
- Whisper not as space-efficient as modern TSDBs
- Different query paradigm
- Less active development

**Why Rejected:** Our storage engine is TSDB-based, not Whisper. PromQL provides more expressive queries than Graphite's functions.

### Alternative 3: Datadog Built-in Dashboards

**Description:** Use Datadog's native visualization instead of Grafana.

**Pros:**
- Integrated with metrics collection
- Rich built-in dashboards

**Cons:**
- Vendor lock-in
- High cost
- Less customization

**Why Rejected:** Self-hosted solution required for cost control and avoiding vendor lock-in.

---

## Implementation Notes

### Phase 1: Basic Integration (Q2 2026)

1. Deploy Grafana with Prometheus data source
2. Configure OAuth authentication
3. Set up dashboard provisioning
4. Create standard overview dashboards

### Phase 2: Advanced Features (Q3 2026)

1. Add alerting integration
2. Implement variable templates
3. Create service-specific dashboards
4. Set up role-based folder organization

### Phase 3: Optimization (Q4 2026)

1. Optimize queries for performance
2. Implement dashboard caching
3. Add automatic dashboard generation
4. Build custom panel plugins (if needed)

---

## Cross-References

- **ADR-001:** [ADR-001-prometheus-metrics-model.md](./ADR-001-prometheus-metrics-model.md) - Query compatibility
- **ADR-003:** [ADR-003-timeseries-storage.md](./ADR-003-timeseries-storage.md) - Storage backend
- **ADR-005:** [ADR-005-alerting-strategy.md](./ADR-005-alerting-strategy.md) - Alert visualization
- **Grafana Documentation:** https://grafana.com/docs/grafana/latest/
- **Grafana Provisioning:** https://grafana.com/docs/grafana/latest/administration/provisioning/

---

*This ADR was accepted on 2026-04-04 by the Phenotype Architecture Team.*
