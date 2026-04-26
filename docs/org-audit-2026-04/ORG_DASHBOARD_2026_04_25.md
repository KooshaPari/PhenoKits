# Phenotype Org Dashboard (2026-04-25)

**After W-67F Batch Audit — Cross-Collection Snapshot**

---

## Organization Metrics

| Metric | Value | Change (W-67D → W-67F) |
|--------|-------|----------------------|
| **Total Canonical Repos** | 149 | — |
| **Audited Repos (Waves W-67+)** | 11 | +8 (phenotype-* batch) |
| **Audited Repos (Legacy)** | 108 | — |
| **Genuinely Unaudited** | 31 | — (unchanged) |
| **Archived/Deprecated** | 10 | — |
| **Total Workspace LOC** | ~10.0M | +~230K (W-67F additions) |

---

## Collection Roster After W-67F

### By Size & Member Count

| Collection | Repos | Est. LOC | Activity | Key Members |
|-----------|-------|----------|----------|------------|
| **phenoShared** | 6 | ~80K | High | AuthKit, phenoAI, PhenoSchema |
| **PhenoObservability** | 5 | ~200K | High | ObservabilityKit, HeliosLab |
| **Sidekick** | 5 | ~120K | Active | PhenoAgent, phenotype-skills |
| **Stashly** | 3 | ~50K | Stable | PhenoCompose, BytePort |
| **Standalone Products** | 4 | ~640K | Mixed | phenotype-omlx (shipped), FocalPoint |

**Organization Total: 23 repos | ~1.09M LOC (core collections + standalones)**

---

## W-67F Additions Impact

### Collection Growth

```
PhenoObservability:   3 repos → 5 repos   (+2: ObservabilityKit, HeliosLab inferred)
Sidekick:             2 repos → 5 repos   (+3: PhenoAgent, phenotype-skills, MCP tooling)
phenoShared:          3 repos → 6 repos   (+3: phenoAI, AuthKit, PhenoSchema)
Stashly:              1 repo  → 3 repos   (+2: PhenoCompose, BytePort inferred)
Standalone Products:  3 repos → 4 repos   (+1: phenotype-omlx / consumer product)
```

### LOC Contribution (W-67F Audited)

| Repo | LOC | Collection | Role |
|------|-----|-----------|------|
| phenotype-omlx | 138.8K | Standalone | Consumer product (shipped) |
| ObservabilityKit | 7.9K | PhenoObservability | Multi-language OTEL SDKs |
| PhenoAgent | 22K | Sidekick | Agent orchestration daemon |
| AuthKit | 18.2K | phenoShared | Cross-platform auth |
| phenotype-skills | 12.3K | Sidekick | Skill registry |
| PhenoCompose | 4.3K | Stashly | Workload isolation runtime |
| PhenoSchema | 2.1K | phenoShared | Schema management |
| phenoAI | 373 | phenoShared | LLM routing abstractions |

**W-67F Total: 205.7K LOC across 8 repos**

---

## Key Observations

### Collection Maturity Tiers

**Tier 1 — Production Services:**
- **phenoShared** — Stable, 4-language, multi-provider auth + LLM routing
- **PhenoObservability** — Multi-repo integration, active traces/metrics collection

**Tier 2 — Active Development:**
- **Sidekick** — Scaffold to active (PhenoAgent 22K LOC, phenotype-skills test coverage)
- **Stashly** — Stable runtime, merged from multiple isolation sources

**Tier 3 — Standalone Ecosystem:**
- **phenotype-omlx** — Shipped, benchmarked (138.8K LOC, 229 tests)
- **FocalPoint, heliosApp, Tracera** — Active consumer products

### Standalone Product Pattern Emerging

With phenotype-omlx, org now has **4 standalone consumer products outside microservice ecosystem:**

1. **phenotype-omlx** (W-67F, NEW) — macOS LLM inference → omlx.ai
2. **FocalPoint** (W-67D) — Phenotype IDE
3. **heliosApp** — Helios distributed runtime
4. **Tracera** — Tracing visualization

These do NOT depend on Sidekick/PhenoObservability stack — independent deployment model.

---

## Unaudited High-Priority Repos (W-68 Targets)

| Rank | Repo | LOC | Notes |
|------|------|-----|-------|
| 1 | ValidationKit | 710K | Large; scope unknown |
| 2 | agileplus-landing | 617K | Active; clarify role |
| 3 | phenotype-previews-smoketest | 244K | Smoke test; validate use |
| 4 | apps | 87K | Generic collection; decompose |
| 5 | thegent-jsonl | 41K | Data; versioning unclear |

---

## Next Steps (W-68+)

1. **Collection Integration Tests** — Validate cross-collection contracts (phenoShared → Sidekick → Apps)
2. **Standalone Product Formalization** — Define stand-alone product criteria, maturity gates
3. **Unaudited Triage** — Audit top 10 unaudited repos (ValidationKit, agileplus-landing)
4. **Collection SLO Definition** — Establish update/security/test cadences per collection

---

**Dashboard Updated:** 2026-04-25  
**Source:** W-67F batch audit + cross-collection synthesis  
**Next Refresh:** Post-W-68 (estimated 2026-04-28)
