# ADR-006: Cardinality Management

**Document ID:** PHENOTYPE_GAUGE_ADR_006  
**Status:** Accepted  
**Last Updated:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related ADRs:** [ADR-001](./ADR-001-prometheus-metrics-model.md), [ADR-002](./ADR-002-opentelemetry.md), [ADR-003](./ADR-003-timeseries-storage.md)

---

## Context

Cardinality explosion is the primary scalability challenge for Prometheus-compatible metrics systems. Uncontrolled cardinality leads to excessive memory usage, slow queries, and exponential storage growth. phenotype-gauge must implement proactive cardinality management.

### Requirements

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| R1 | Per-tenant cardinality budgets | Critical | Multi-tenant isolation |
| R2 | Cardinality monitoring | Critical | Early detection |
| R3 | Label value validation | High | Prevent unbounded labels |
| R4 | Automatic rollup for high-cardinality | Medium | Cost optimization |
| R5 | Query cardinality limits | High | Prevent abuse |

### Constraints

- Must not break existing PromQL compatibility
- Enforcement should be transparent to valid use cases
- Need to support migration from legacy systems

---

## Decision

We implement a **budget-based cardinality management system** with proactive monitoring, label validation, and automatic mitigation strategies. Each tenant receives a cardinality budget enforced at ingestion time.

### Cardinality Model

```python
# Cardinality = product of unique label values
# Example: metric{label1=a, label2=b} vs metric{label1=c, label2=d}

# Total cardinality = |label1_values| × |label2_values| × ...
metric_cardinality = {
    "http_requests_total": {
        "labels": ["method", "status", "path", "service"],
        "values_per_label": {
            "method": 5,      # GET, POST, PUT, DELETE, PATCH
            "status": 20,     # 200, 201, 204, 400, 401, 403, 404, 500, etc.
            "path": 100,      # Bounded API routes
            "service": 10,    # 10 services
        },
        "estimated_series": 5 * 20 * 100 * 10 = 100,000
    }
}

# Cardinality explosion examples
explosion_examples = {
    # UNSAFE: User ID creates unbounded cardinality
    "bad_metric": {
        "labels": ["user_id"],  # 1,000,000+ unique users = explosion!
        "estimated_series": 1000000
    },
    
    # SAFE: Bounded cardinality
    "safe_metric": {
        "labels": ["method", "status", "region"],
        "values_per_label": {"method": 5, "status": 20, "region": 5},
        "estimated_series": 5 * 20 * 5 = 500
    }
}
```

### Budget Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     Cardinality Budget Architecture                       │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                      Tenant Budget Registry                       │ │
│  │                                                                  │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │ │
│  │  │ Tenant A    │  │ Tenant B    │  │ Tenant C    │            │ │
│  │  │ Budget: 50K │  │ Budget: 25K │  │ Budget: 10K │            │ │
│  │  │ Used: 35K   │  │ Used: 12K   │  │ Used: 9.5K  │            │ │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘            │ │
│  │         │                │                │                     │ │
│  │         └────────────────┼────────────────┘                     │ │
│  │                          ▼                                       │ │
│  │  ┌────────────────────────────────────────────────────────────┐│ │
│  │  │               Budget Manager                                 ││ │
│  │  │                                                            ││ │
│  │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐               ││ │
│  │  │  │  Checking │  │ Monitoring │  │   Alerting│               ││ │
│  │  │  │           │  │           │  │           │               ││ │
│  │  │  │ Per-tenant│  │ Per-metric│  │  Budget   │               ││ │
│  │  │  │ budgets   │  │ breakdown │  │ warnings  │               ││ │
│  │  │  └───────────┘  └───────────┘  └───────────┘               ││ │
│  │  │                                                            ││ │
│  │  └────────────────────────────────────────────────────────────┘│ │
│  │                          │                                       │ │
│  └──────────────────────────┼───────────────────────────────────────┘ │
│                             │                                           │
│                             ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                      Ingestion Pipeline                           │ │
│  │                                                                  │ │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐   │ │
│  │  │  Receive  │─▶│  Validate │─▶│   Budget  │─▶│   Store  │   │ │
│  │  │  Metrics  │  │  Labels   │  │   Check   │  │           │   │ │
│  │  │           │  │           │  │           │  │           │   │ │
│  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘   │ │
│  │                                                                  │ │
│  │  Validation:                                                    │ │
│  │  - Label name syntax                                           │ │
│  │  - Label value cardinality                                     │ │
│  │  - Reserved label check                                        │ │
│  │                                                                  │ │
│  │  Budget Check:                                                  │ │
│  │  - Series limit per tenant                                     │ │
│  │  - New series creation rate                                    │ │
│  │  - Rollup eligibility                                          │ │
│  │                                                                  │ │
│  └─────────────────────────────────────────────────────────────────┘ │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Label Classification

| Classification | Examples | Action |
|---------------|----------|--------|
| **Static** | `method`, `status`, `region` | Always allowed |
| **Bounded** | `service`, `path` (with limit) | Allow with monitoring |
| **Dynamic** | `user_id`, `session_id` | BLOCKED by default |
| **Trace-derived** | `trace_id`, `span_id` | Must be dropped at collection |

### Enforcement Rules

```yaml
cardinality:
  # Per-tenant budgets
  tenant_budgets:
    critical: 100000   # API gateway, auth
    high: 50000        # Core services
    medium: 25000     # Supporting services
    low: 10000        # Background jobs
    
  # Label rules
  labels:
    # Allowed with monitoring
    allowed_with_monitoring:
      - path          # Limited to 500 unique values
      - user_type    # Limited to 10 unique values
      
    # Blocked entirely
    blocked:
      - user_id
      - session_id
      - trace_id
      - request_id
      - query_text
      
    # Transformation rules
    transformations:
      - label: path
        # Truncate to max 100 chars
        max_length: 100
        # Normalize common patterns
        regex_replace:
          pattern: '/users/[0-9]+'
          replacement: '/users/{id}'
          
  # Series limits
  limits:
    # Max new series per minute per tenant
    new_series_rate: 1000
    # Max total series per tenant
    max_series: 100000
    # Warn at this percentage
    warning_threshold: 0.8
```

---

## Consequences

### Positive Consequences

1. **Predictable Costs** - Storage and query costs bounded per tenant.

2. **Performance Stability** - No cardinality explosions causing cascading failures.

3. **Fair Resource Allocation** - Multi-tenant isolation ensured.

4. **Early Warning** - Cardinality monitoring prevents surprises.

### Negative Consequences

1. **Migration Effort** - Existing high-cardinality metrics must be remediated.

2. **Developer Education** - Teams must learn cardinality-safe patterns.

3. **Blocked Use Cases** - Some valid use cases (e.g., per-user metrics) require alternative approaches.

---

## Technical Details

### Series Registration

```python
# Series registration flow
class SeriesRegistry:
    def __init__(self, storage):
        self.storage = storage
        self.tenant_budgets = {}
        self.series_cache = TTLCache(maxsize=1000000, ttl=3600)
        
    async def register_series(self, tenant_id: str, metric: str, labels: dict) -> bool:
        # Generate series fingerprint
        fingerprint = self._fingerprint(metric, labels)
        
        # Check cache
        if fingerprint in self.series_cache:
            return True
            
        # Get tenant budget
        budget = self.tenant_budgets.get(tenant_id)
        if not budget:
            budget = self._get_default_budget(tenant_id)
            
        # Check budget
        current_usage = self._get_series_count(tenant_id)
        if current_usage >= budget.max_series:
            raise CardinalityLimitExceeded(tenant_id, current_usage, budget.max_series)
            
        # Check new series rate
        if self._exceeds_rate_limit(tenant_id):
            raise SeriesRateLimitExceeded(tenant_id)
            
        # Register series
        await self.storage.create_series(tenant_id, fingerprint, metric, labels)
        self.series_cache[fingerprint] = True
        
        # Update monitoring
        await self._update_cardinality_metric(tenant_id)
        
        return True
        
    def _fingerprint(self, metric: str, labels: dict) -> str:
        # Sort labels for consistent fingerprinting
        sorted_labels = sorted(labels.items())
        raw = f"{metric}{sorted_labels}"
        return hashlib.sha256(raw.encode()).hexdigest()[:16]
```

### High-Cardinality Detection

```python
# Anomaly detection for cardinality
async def detect_cardinality_anomalies(tenant_id: str) -> list[Anomaly]:
    # Get historical baseline
    baseline = await get_cardinality_baseline(tenant_id, days=7)
    
    # Get current state
    current = await get_current_cardinality(tenant_id)
    
    anomalies = []
    
    # Check total cardinality
    if current.total > baseline.total * 2:
        anomalies.append(Anomaly(
            type="cardinality_spike",
            severity="high",
            message=f"Cardinality doubled: {baseline.total} -> {current.total}"
        ))
        
    # Check per-label cardinality
    for label, current_count in current.label_cardinality.items():
        baseline_count = baseline.label_cardinality.get(label, 0)
        
        if baseline_count > 0:
            growth = current_count / baseline_count
            
            if growth > 10:  # 10x growth
                anomalies.append(Anomaly(
                    type="label_cardinality_explosion",
                    severity="critical",
                    label=label,
                    message=f"Label {label} grew {growth:.1f}x: {baseline_count} -> {current_count}"
                ))
                
    return anomalies
```

---

## Alternatives Considered

### Alternative 1: No Enforcement (Trust-Based)

**Description:** Allow unlimited cardinality with faith in developer discipline.

**Pros:**
- No friction for developers
- Maximum flexibility

**Cons:**
- History shows this leads to cardinality explosions
- Cascading failures from resource exhaustion
- No cost predictability

**Why Rejected:** Production incidents from cardinality explosions are too costly. Trust must be verified.

### Alternative 2: Hard Blocks Only

**Description:** Block high-cardinality labels entirely without monitoring.

**Pros:**
- Simple implementation
- No complex configuration

**Cons:**
- Blocks legitimate use cases without visibility
- No warning before blocking
- No understanding of actual cardinality drivers

**Why Rejected:** We want to allow bounded high-cardinality where appropriate while monitoring closely.

---

## Implementation Notes

### Phase 1: Foundation (Q2 2026)

1. Implement series fingerprinting
2. Build tenant budget registry
3. Add basic cardinality monitoring
4. Create blocked label list

### Phase 2: Enforcement (Q3 2026)

1. Add ingestion-time budget checking
2. Implement new series rate limiting
3. Add label transformation rules
4. Build alerting for budget thresholds

### Phase 3: Optimization (Q4 2026)

1. Add automatic series pruning
2. Implement high-cardinality metric rollup
3. Build cardinality analytics dashboard
4. Add historical trending

---

## Cross-References

- **ADR-001:** [ADR-001-prometheus-metrics-model.md](./ADR-001-prometheus-metrics-model.md) - Metric model
- **ADR-002:** [ADR-002-opentelemetry.md](./ADR-002-opentelemetry.md) - Collection
- **ADR-003:** [ADR-003-timeseries-storage.md](./ADR-003-timeseries-storage.md) - Storage implications
- **Metrics Reference:** [metrics.md](./metrics.md) - Detailed cardinality guidance
- **Prometheus Cardinality:** https://prometheus.io/docs/practices cardinality/

---

*This ADR was accepted on 2026-04-04 by the Phenotype Architecture Team.*
