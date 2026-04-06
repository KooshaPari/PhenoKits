# ADR-005: Alerting Strategy

**Document ID:** PHENOTYPE_GAUGE_ADR_005  
**Status:** Accepted  
**Last Updated:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related ADRs:** [ADR-001](./ADR-001-prometheus-metrics-model.md), [ADR-003](./ADR-003-timeseries-storage.md), [ADR-004](./ADR-004-dashboard-stack.md), [ADR-006](./ADR-006-cardinality-management.md)

---

## Context

Alerting is critical for maintaining service reliability. phenotype-gauge requires an alerting system that supports SLO-based alerts, multi-channel notifications, deduplication, and actionable alerts that minimize fatigue while ensuring critical issues are escalated promptly.

### Requirements

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| R1 | SLO-based alerting | Critical | Error budget burn rate |
| R2 | Multi-channel notifications | High | Slack, PagerDuty, email |
| R3 | Alert deduplication | High | Reduce noise |
| R4 | Alert grouping | High | Correlated alerts |
| R5 | Alert routing | Medium | Severity-based routing |
| R6 | Maintenance windows | Medium | Planned downtime |

---

## Decision

We implement a **multi-layered alerting architecture** combining Prometheus-style recording rules and alerting rules with Grafana's Alerting engine, integrated with Phenotype's notification infrastructure.

### Alert Types

| Type | Description | Evaluation | Use Case |
|------|-------------|------------|----------|
| **SLO Alert** | Error budget burn rate | Continuous | SLI breach prediction |
| **Threshold** | Static threshold crossed | Per evaluation | Resource exhaustion |
| **Anomaly** | ML-based detection | Periodic | Unexpected behavior |
| **Presence** | Metric disappears | Per evaluation | Collector failure |

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Alerting Architecture                            │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                      Alert Rule Evaluation                         │ │
│  │                                                                  │ │
│  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐        │ │
│  │  │ Prometheus  │    │  Grafana    │    │  Custom     │        │ │
│  │  │  Alerts     │    │  Alerts     │    │  Alerts     │        │ │
│  │  │  (Rules)    │    │  (Engine)   │    │  (SLO)     │        │ │
│  │  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘        │ │
│  │         │                   │                   │               │ │
│  │         └───────────────────┼───────────────────┘               │ │
│  │                             │                                   │ │
│  │                             ▼                                   │ │
│  │  ┌───────────────────────────────────────────────────────────┐│ │
│  │  │                  Alert Manager                              ││ │
│  │  │                                                           ││ │
│  │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐             ││ │
│  │  │  │ Routing   │  │   Grouping │  │ Deduplica-│             ││ │
│  │  │  │           │  │           │  │  tion     │             ││ │
│  │  │  └───────────┘  └───────────┘  └───────────┘             ││ │
│  │  │                                                           ││ │
│  │  └───────────────────────────────────────────────────────────┘│ │
│  │                             │                                   │ │
│  └─────────────────────────────┼───────────────────────────────────┘ │
│                                │                                       │
│                                ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                      Notification Channels                        │ │
│  │                                                                  │ │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐   │ │
│  │  │  Slack    │  │PagerDuty │  │   Email   │  │   OpsGenie│   │ │
│  │  │           │  │           │  │           │  │           │   │ │
│  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘   │ │
│  │                                                                  │ │
│  └─────────────────────────────────────────────────────────────────┘ │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Consequences

### Positive Consequences

1. **Proven Pattern** - Prometheus alerting is battle-tested.

2. **SLO Integration** - Native error budget burn rate alerts.

3. **Rich Routing** - Complex routing, grouping, and deduplication.

4. **Grafana Integration** - Unified dashboard + alerting experience.

### Negative Consequences

1. **Rule Management** - Alert rule proliferation without governance.

2. **Evaluation Cost** - Complex rules can be expensive to evaluate.

3. **Alert Fatigue** - Without proper tuning, too many alerts.

---

## Technical Details

### SLO-Based Alerting

```yaml
groups:
  - name: slo_alerts
    interval: 60s
    rules:
      # Fast burn: 1% error in 1 hour = exhausted budget
      - alert: SLOServiceFastBurn
        expr: |
          (
            sum(rate(http_requests_total{status=~"5.."}[1h]))
            /
            sum(rate(http_requests_total[1h]))
          ) > 0.01
        for: 0m
        labels:
          severity: critical
          slo: availability
        annotations:
          summary: "SLO {{ $labels.service }} burning fast"
          description: |
            Error budget is burning at {{ $value | humanizePercentage }} per hour.
            At this rate, the 30-day budget will be exhausted in {{ $value | | slo_budget_exhaustion_time }}.
          runbook_url: "https://runbooks.phenotype.dev/slo-burn"
          
      # Slow burn: 1% error in 6 hours = exhausted budget  
      - alert: SLOServiceSlowBurn
        expr: |
          (
            sum(rate(http_requests_total{status=~"5.."}[6h]))
            /
            sum(rate(http_requests_total[6h]))
          ) > 0.001
        for: 0m
        labels:
          severity: warning
          slo: availability
        annotations:
          summary: "SLO {{ $labels.service }} burning slowly"
          description: |
            Error budget is burning at {{ $value | humanizePercentage }} over 6 hours.
```

### Error Budget Calculation

```python
# Error budget computation
def calculate_error_budget(slo_target: float, total_requests: int, error_requests: int):
    error_rate = error_requests / total_requests
    slo_compliance = 1 - error_rate
    
    # Monthly budget assuming 30 days
    total_monthly_requests = total_requests * (30 * 24 * 60 * 60) / measurement_window_seconds
    allowed_error_budget = total_monthly_requests * (1 - slo_target)
    remaining_budget = allowed_error_budget - error_requests
    
    burn_rate = error_rate / (1 - slo_target)
    time_to_exhaustion = remaining_budget / (error_requests / measurement_window_seconds) if error_requests > 0 else float('inf')
    
    return {
        "error_rate": error_rate,
        "slo_compliance": slo_compliance,
        "budget_remaining_pct": remaining_budget / allowed_error_budget if allowed_error_budget > 0 else 1.0,
        "burn_rate": burn_rate,
        "hours_to_exhaustion": time_to_exhaustion / 3600 if time_to_exhaustion != float('inf') else None,
    }
```

### Multi-Channel Routing

```yaml
# Alertmanager configuration
route:
  receiver: 'default'
  group_by: ['alertname', 'service', 'severity']
  group_wait: 30s
  group_interval: 5m
  repeat_interval: 4h
  
  routes:
    # Critical alerts go to PagerDuty immediately
    - match:
        severity: critical
      receiver: 'pagerduty'
      continue: true
      
    # SLO alerts route to SLO-specific channel
    - match:
        slo: availability
      receiver: 'slo-alerts'
      group_by: ['service', 'slo']
      
    # Warning alerts go to Slack
    - match:
        severity: warning
      receiver: 'slack-warnings'
      
    # Maintenance windows suppress alerts
    - match:
        maintenance: 'true'
      receiver: 'null'
      
receivers:
  - name: 'default'
    email_configs:
      - to: 'alerts@phenotype.dev'
        send_resolved: true
        
  - name: 'pagerduty'
    pagerduty_configs:
      - service_key: '${PAGERDUTY_SERVICE_KEY}'
        severity: '{{ if eq .GroupLabels.severity "critical" }}critical{{ else }}warning{{ end }}'
        component: 'phenotype-gauge'
        group: '{{ .GroupLabels.alertname }}'
        
  - name: 'slack-warnings'
    slack_configs:
      - channel: '#alerts-warnings'
        send_resolved: true
        title: |
          [{{ .Status | toUpper }}] {{ .GroupLabels.alertname }}
        text: |
          {{ range .Alerts }}
          *Alert:* {{ .Labels.alertname }}
          *Severity:* {{ .Labels.severity }}
          *Service:* {{ .Labels.service }}
          *Summary:* {{ .Annotations.summary }}
          *Description:* {{ .Annotations.description }}
          {{ end }}
          
  - name: 'slo-alerts'
    slack_configs:
      - channel: '#slo-alerts'
        send_resolved: true
        title: |
          SLO Alert: {{ .GroupLabels.service }}
        text: |
          {{ range .Alerts }}
          *SLO:* {{ .Labels.slo }}
          *Burn Rate:* {{ .Value | humanizePercentage }}
          *Time to Exhaustion:* {{ .Annotations.time_to_exhaustion }}
          {{ end }}
```

---

## Alert Severity Levels

| Level | Response SLA | Escalation | Examples |
|-------|-------------|------------|----------|
| **Critical (P1)** | < 5 min | Page + Escalate | Gauge down, major outage |
| **High (P2)** | < 15 min | Page | SLO breach, high error rate |
| **Medium (P3)** | < 1 hour | Slack + Ticket | Performance degradation |
| **Low (P4)** | < 24 hours | Ticket | Capacity planning |

---

## Cross-References

- **ADR-001:** [ADR-001-prometheus-metrics-model.md](./ADR-001-prometheus-metrics-model.md) - Query compatibility
- **ADR-003:** [ADR-003-timeseries-storage.md](./ADR-003-timeseries-storage.md) - Storage backend
- **ADR-004:** [ADR-004-dashboard-stack.md](./ADR-004-dashboard-stack.md) - Dashboard integration
- **ADR-006:** [ADR-006-cardinality-management.md](./ADR-006-cardinality-management.md) - Metric governance
- **SLO Workbook:** https://sre.google/workbook/alerting/

---

*This ADR was accepted on 2026-04-04 by the Phenotype Architecture Team.*
