# Phenotype Org Dashboard (2026-04-25)

**After W-67F Batch Audit → W-70/71 Landings — Cross-Collection Snapshot**

## v26 Delta Summary (W-70/71)

- Sidekick roster: 5 repos → 3 canonical + 0 candidates (W-69 downgrade cleanup complete)
- FocalPoint v0.0.11 stable; heliosApp v2026.05A.0 → 05B.0 in-flight release
- Tracera v0.1.0 shipped; phenotype-omlx v0.3.x active
- W-67 audits + W-71 dormant-pattern survey integrated into health assessments
- AgentMCP server pack corruption flagged; OpenAI key UNREVOKED; HeliosLab BROKEN (open risks)

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
| **Sidekick** | 3 | ~80K | Active | PhenoAgent (core retained), phenotype-skills downgraded |
| **Stashly** | 3 | ~50K | Stable | PhenoCompose, BytePort |
| **Standalone Products** | 4 | ~640K | Mixed | phenotype-omlx (shipped), FocalPoint v0.0.11, heliosApp v2026.05B.0, Tracera v0.1.0 |

**Organization Total: 21 repos | ~1.05M LOC (core collections + standalones; post-W-69 rationalization)**

---

## W-67F Additions Impact

### Collection Growth

```
W-67F Growth:         3 repos → 5 repos (PhenoObservability, Sidekick expansion)
W-70/71 Rationalization: 5 repos → 3 repos (Sidekick: PhenoAgent retained, skills/MCP downgraded)
phenoShared:          3 repos → 6 repos   (+3: phenoAI, AuthKit, PhenoSchema)
Stashly:              1 repo  → 3 repos   (+2: PhenoCompose, BytePort)
Standalone Products:  3 repos → 4 repos   (phenotype-omlx, FocalPoint, heliosApp, Tracera)
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
- **Sidekick** — Core agent daemon (PhenoAgent 22K LOC); skills/MCP tooling downgraded post-W-69
- **Stashly** — Stable runtime, merged from multiple isolation sources

**Tier 3 — Standalone Ecosystem:**
- **phenotype-omlx** — Shipped, benchmarked (138.8K LOC, 229 tests)
- **FocalPoint, heliosApp, Tracera** — Active consumer products

### Standalone Product Pattern Emerging

**4 standalone consumer products (independent deployment model, no Sidekick/PhenoObservability dependency):**

1. **phenotype-omlx** (v0.3.x active, 138.8K LOC) — macOS LLM inference → omlx.ai
2. **FocalPoint** (v0.0.11 stable, 650K LOC) — Phenotype IDE, all-linters-passing
3. **heliosApp** (v2026.05A → 05B in-flight) — Helios distributed runtime
4. **Tracera** (v0.1.0 shipped) — Tracing visualization

All 4 on latest releases or in-flight. Health badges: FocalPoint ✓, heliosApp pending 05B merge, others active.

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
