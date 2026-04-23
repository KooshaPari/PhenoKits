# ADR-003: Time-Series Storage Architecture

**Document ID:** PHENOTYPE_GAUGE_ADR_003  
**Status:** Accepted  
**Last Updated:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related ADRs:** [ADR-001](./ADR-001-prometheus-metrics-model.md), [ADR-002](./ADR-002-opentelemetry.md), [ADR-005](./ADR-005-alerting-strategy.md)

---

## Context

Time-series storage is the core of phenotype-gauge. The storage architecture must efficiently handle high-throughput ingestion, support flexible queries, manage costs through tiering, and maintain reliability. This ADR defines the storage architecture decisions.

### Requirements

| ID | Requirement | Priority | Notes |
|----|-------------|----------|-------|
| R1 | Handle 100K+ metrics/sec ingestion | Critical | Sustained rate |
| R2 | 15-second resolution for hot data | Critical | Real-time queries |
| R3 | 13-month retention minimum | Critical | Compliance, trends |
| R4 | Sub-500ms query latency (p95) | Critical | Dashboard performance |
| R5 | Cost-effective tiered storage | High | 80%+ storage cost reduction |
| R6 | HA with no data loss | Critical | Reliability |

### Constraints

- Self-hosted solution required
- Must fit existing infrastructure (S3 for cold)
- Team expertise in Go
- Open-source preferred

---

## Decision

We adopt a **tiered storage architecture** with hot/warm/cold tiers, using a custom TSDB implementation inspired by Prometheus's design with additional optimizations for multi-tenant workloads and cost efficiency.

### Storage Tiers

| Tier | Duration | Resolution | Storage | Use Case |
|------|----------|------------|---------|----------|
| Hot | 0-30 days | 15s | NVMe SSD | Real-time dashboards, alerting |
| Warm | 31-365 days | 60s | HDD | Historical analysis, capacity planning |
| Cold | 1-13 months | 300s | S3/Object | Long-term trends, audit |

### Block Format

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           TSDB Block Structure                           │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                           WAL (Write-Ahead Log)                    │ │
│  │  ┌─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐   │ │
│  │  │ Entry 1 │ Entry 2 │ Entry 3 │ Entry 4 │ Entry 5 │   ...   │   │ │
│  │  └─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘   │ │
│  │  Durable, append-only, replayed on startup                       │ │
│  └─────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                             Head Block                            │ │
│  │  ┌───────────────────────────────────────────────────────────┐  │ │
│  │  │  In-memory writes (ring buffer)                          │  │ │
│  │  │  Memory-mapped for read access                          │  │ │
│  │  │  Checkpoint every 2 hours                                │  │ │
│  │  └───────────────────────────────────────────────────────────┘  │ │
│  └─────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                          Chunk Files                             │ │
│  │                                                                  │ │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐               │ │
│  │  │ Chunk  │  │ Chunk  │  │ Chunk  │  │ Chunk  │               │ │
│  │  │ 0-2h   │  │ 2-4h   │  │ 4-6h   │  │ 6-8h   │               │ │
│  │  │(index) │  │(data)  │  │(data)  │  │(data)  │               │ │
│  │  └────────┘  └────────┘  └────────┘  └────────┘               │ │
│  │       │                                                           │ │
│  │       ▼                                                           │ │
│  │  ┌───────────────────────────────────────────────────────────┐  │ │
│  │  │                        Index                                │  │ │
│  │  │  - Series ref → chunk offset                             │  │ │
│  │  │  - Label matchers → series IDs                          │  │ │
│  │  │  - Posting lists                                         │  │ │
│  │  └───────────────────────────────────────────────────────────┘  │ │
│  │                                                                  │ │
│  └─────────────────────────────────────────────────────────────────┘ │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Query Path

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Query Execution Path                            │
│                                                                         │
│  Query: rate(http_requests_total{path="/api"}[5m])                    │
│                                                                         │
│  1. Parse & Validate                                                    │
│     ┌─────────────────────────────────────────────────────────┐        │
│     │ PromQL Parser → Abstract Syntax Tree                    │        │
│     │ AST Validator → Label matchers, functions              │        │
│     └─────────────────────────────────────────────────────────┘        │
│                              │                                           │
│                              ▼                                           │
│  2. Index Lookup                                                        │
│     ┌─────────────────────────────────────────────────────────┐        │
│     │ Index → Matching series IDs for label matchers         │        │
│     │ {path="/api"} → [series_123, series_456, ...]          │        │
│     └─────────────────────────────────────────────────────────┘        │
│                              │                                           │
│                              ▼                                           │
│  3. Chunk Selection                                                      │
│     ┌─────────────────────────────────────────────────────────┐        │
│     │ Time range [T-5m, T] → Relevant chunks                 │        │
│     │ └─ Chunks: hot (15s), warm (60s)                       │        │
│     └─────────────────────────────────────────────────────────┘        │
│                              │                                           │
│                              ▼                                           │
│  4. Data Retrieval                                                      │
│     ┌─────────────────────────────────────────────────────────┐        │
│     │ For each series:                                       │        │
│     │   - Read chunks (mmap or S3)                          │        │
│     │   - Extract data points in range                       │        │
│     │   - Decode (XOR compression)                           │        │
│     └─────────────────────────────────────────────────────────┘        │
│                              │                                           │
│                              ▼                                           │
│  5. Aggregation                                                         │
│     ┌─────────────────────────────────────────────────────────┐        │
│     │ PromQL functions: rate(), sum(), histogram_quantile()   │        │
│     │ Vector notation across series                           │        │
│     └─────────────────────────────────────────────────────────┘        │
│                              │                                           │
│                              ▼                                           │
│  6. Result                                                              │
│     ┌─────────────────────────────────────────────────────────┐        │
│     │ {path="/api"}: 1425.67 (avg rate over 5m window)       │        │
│     └─────────────────────────────────────────────────────────┘        │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Consequences

### Positive Consequences

1. **Proven Design** - Prometheus TSDB is battle-tested at massive scale.

2. **Predictable Performance** - Block-based storage enables efficient range queries.

3. **Cost Optimization** - Tiered storage reduces hot storage costs by 80%+.

4. **HA Ready** - WAL ensures durability; replication available for HA mode.

5. **Compression** - XOR encoding + ZSTD provides 5-10x compression.

### Negative Consequences

1. **Complexity** - Tiered storage adds operational complexity.

2. **Cold Query Latency** - S3 queries have higher latency (mitigated by pre-aggregation).

3. **Compaction Overhead** - Background compaction competes with queries.

4. **Single-node Bottleneck** - Without federation, single-node TSDB has limits.

---

## Technical Details

### Block Layout

```python
# Chunk file format
chunk = {
    "header": {
        "magic": 0xBD0B,           # Magic bytes
        "version": 3,              # Format version
        "series_ref": 12345,       # Reference to index
        "mint": 1712188800,         # Min timestamp
        "maxt": 1712196000,         # Max timestamp (2h block)
        "encoding": "xor",         # Compression
    },
    "data": bytes,                  # Encoded data points
    "footer": {
        "num_samples": 480,         # 2h / 15s
        "crc32": 0xABCD1234,       # Integrity check
    }
}

# Index file format (per series)
index_entry = {
    "series_id": 12345,
    "labels": [
        {"name": "__name__", "value": "http_requests_total"},
        {"name": "method", "value": "GET"},
        {"name": "path", "value": "/api"},
    ],
    "chunks": [
        {"ref": 0, "mint": 1712188800, "maxt": 1712196000},
        {"ref": 1, "mint": 1712196000, "maxt": 1712203200},
    ]
}
```

### Compression Algorithms

| Algorithm | Compression Ratio | Speed | Use Case |
|-----------|------------------|-------|----------|
| XOR | 3-5x | Very fast | Hot storage |
| Delta | 2-3x | Very fast | Timestamps |
| ZSTD | 3-10x | Medium | Warm/cold |
| LZ4 | 2-3x | Very fast | Fast compression needed |

### Retention Configuration

```yaml
storage:
  # Hot tier: NVMe SSD
  hot:
    path: /mnt/nvme/gauge/hot
    max_bytes: 500GB
    ttl: 30d
    compaction:
      enabled: true
      block_size: 2h
      max_block_age: 8h
    
  # Warm tier: HDD
  warm:
    path: /mnt/hdd/gauge/warm
    max_bytes: 2TB
    ttl: 335d
    downsampling:
      from_resolution: 15s
      to_resolution: 60s
      aggregations: [avg, min, max, sum, count]
    compression: zstd
    
  # Cold tier: S3
  cold:
    type: s3
    bucket: phenotype-gauge-cold
    prefix: metrics/
    max_bytes: 10TB
    ttl: 365d
    downsampling:
      from_resolution: 60s
      to_resolution: 300s
    compression: zstd
    query_in_place: true  # Query without restore
    
  # Lifecycle transitions
  lifecycle:
    - from: hot
      to: warm
      after: 30d
      action: downsample_and_move
      
    - from: warm
      to: cold
      after: 365d
      action: downsample_and_archive
      
    - delete: true
      after: 730d  # Optional final deletion
```

---

## Alternatives Considered

### Alternative 1: InfluxDB

**Description:** Use InfluxDB as the storage backend.

**Pros:**
- Purpose-built TSDB
- Built-in tiering (IIB)
- InfluxQL/Flux query language

**Cons:**
- Heavy operational requirements
- Proprietary retention policies
- Less ecosystem tooling
- Licensing concerns

**Why Rejected:** We want more control over the storage format and lower operational overhead. Custom implementation allows optimization for our specific use case.

### Alternative 2: TimescaleDB

**Description:** Use TimescaleDB (PostgreSQL extension) for storage.

**Pros:**
- SQL compatibility
- Existing team expertise
- ACID compliance

**Cons:**
- Higher storage footprint
- Less efficient for time-series only
- Licensing considerations

**Why Rejected:** PostgreSQL-based storage has higher storage and memory requirements. Our workload is exclusively time-series, not relational.

### Alternative 3: ClickHouse

**Description:** Use ClickHouse as the storage engine.

**Pros:**
- Extreme compression
- Excellent analytics performance
- Battle-tested at Yandex, Cloudflare

**Cons:**
- Complex operational requirements
- Merge tree complexity
- Less suited for real-time writes

**Why Rejected:** ClickHouse is optimized for analytics workloads with infrequent writes. Our use case requires high write throughput with real-time query capability.

---

## Implementation Notes

### Phase 1: Core Storage (Q2 2026)

1. Implement WAL with replay
2. Build in-memory head block
3. Implement chunk encoding (XOR)
4. Build basic index structure
5. Add mmap-based chunk reading

### Phase 2: Tiering (Q3 2026)

1. Implement hot→warm transition
2. Build downsampling pipeline
3. Add S3 integration for cold
4. Implement query routing across tiers

### Phase 3: HA (Q4 2026)

1. Add replication factor configuration
2. Implement consistency model
3. Build failure recovery
4. Add compaction optimization

---

## Cross-References

- **ADR-001:** [ADR-001-prometheus-metrics-model.md](./ADR-001-prometheus-metrics-model.md) - Data model
- **ADR-002:** [ADR-002-opentelemetry.md](./ADR-002-opentelemetry.md) - Collection
- **ADR-005:** [ADR-005-alerting-strategy.md](./ADR-005-alerting-strategy.md) - Alerting queries
- **Prometheus TSDB:** https://prometheus.io/docs/prometheus/latest/storage/
- **Metrics Reference:** [metrics.md](./metrics.md) - Storage sizing guidance

---

*This ADR was accepted on 2026-04-04 by the Phenotype Architecture Team.*
