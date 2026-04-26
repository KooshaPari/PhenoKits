# Phenotype Org Cross-Collection Dependency Graph (2026-04-25)

**W-67F Revealed Mappings — Collection Membership & Interop**

---

## Collection Rosters

### 🔍 PhenoObservability (Telemetry & Observability)

**Unified OpenTelemetry instrumentation layer.**

- **ObservabilityKit** (7.9K LOC, W-67F) — Multi-language OTEL SDKs + framework integrations
- phenotype-infrakit/observability (legacy)
- PhenoEvents (event sourcing + telemetry)
- vibeproxy-monitoring-unified (proxy observability)
- HeliosLab (health-dashboard + metrics)

**Total: 5 repos | ~200K LOC estimate**

---

### 🤖 Sidekick (Agent Orchestration & Execution)

**Distributed agent framework with plugin architecture and skill system.**

- **PhenoAgent** (22K LOC, W-67F) — Core orchestration daemon, CLI, gRPC routing
- **phenotype-skills** (12.3K LOC, W-67F) — Skill registry with Python/TS/Go bindings
- agentapi-plusplus (multi-model routing gateway)
- cheap-llm-mcp (LLM inference routing for agents)
- PhenoMCP (MCP server plumbing for agent tools)

**Total: 5 repos | ~120K LOC estimate**

---

### 📦 phenoShared (Shared Libraries & Data Layer)

**Cross-cutting abstractions: auth, LLM routing, schema management.**

- **phenoAI** (373 LOC, W-67F) — LLM routing, MCP server plumbing, embedding primitives
- **AuthKit** (18.2K LOC, W-67F) — OAuth 2.0, OIDC, SAML, WebAuthn (4-language)
- **PhenoSchema** (2.1K LOC, W-67F) — Schema registry, evolution, code generators
- phenotype-shared (core data models, protocols)
- PhenoContracts (data contracts + validation)
- phenoUtils (utility layer)

**Total: 6 repos | ~80K LOC estimate**

---

### 🔒 Stashly (Workload Isolation & Execution)

**Tiered isolation runtime for untrusted workloads.**

- **PhenoCompose** (4.3K LOC, W-67F) — 3-tier isolation (WASM, gVisor, Firecracker)
- BytePort (NVMS + nanovms integration)
- KVirtualStage (virtual display/sandbox)

**Total: 3 repos | ~50K LOC estimate**

---

### 📱 Standalone Consumer Products (Outside Microservice Ecosystem)

**User-facing applications outside core Phenotype services.**

- **phenotype-omlx** (138.8K LOC, W-67F) — macOS LLM inference (shipped, benchmarks at omlx.ai)
- **FocalPoint** (Phenotype IDE, shipped v0.0.11, W-67D)
- **heliosApp** (Helios ecosystem runtime)
- **Tracera** (Tracing + observability UI)

**Total: 4 repos | ~500K LOC estimate (phenotype-omlx dominates)**

---

## Dependency Matrix (Cross-Collection)

### Flow Directions

```
phenoShared ──────> Sidekick ────> [Apps: FocalPoint, heliosApp, Tracera]
       ↓
  PhenoObservability ──> [All services + Apps]
       ↑
       └─── Stashly (workload isolation)
            └─> [Browser automation, tool sandboxing]

phenotype-omlx: STANDALONE (no Phenotype service deps)
```

### Dependencies by Collection

| Collection | Depends On | Purpose |
|------------|-----------|---------|
| **Sidekick** | phenoShared, PhenoObservability | Auth, LLM routing, observability |
| **Stashly** | phenoShared, PhenoObservability | Schema contracts, execution telemetry |
| **FocalPoint** | Sidekick, phenoShared, PhenoObservability | Agent IDE, auth, traces |
| **heliosApp** | Sidekick, phenoShared, PhenoObservability | Agent runtime, multi-model routing |
| **Tracera** | PhenoObservability | Trace ingestion, visualization |
| **phenotype-omlx** | None (standalone) | Self-contained macOS inference tool |

---

## Key Patterns

### New Collections Introduced (W-67F)

1. **PhenoObservability** — Consolidated telemetry/tracing for all services
2. **Sidekick** — Unified agent orchestration (replaces scattered agent tooling)
3. **phenoShared** — De facto standard for auth, LLM, schema across services
4. **Stashly** — Merged isolation runtime (nanovms + BytePort + KVirtualStage paths)

### First Standalone Consumer Product

- **phenotype-omlx** marks org's first user-facing, non-microservice product
- Reference for local model optimization (macOS + Apple Silicon)
- Outside Phenotype service ecosystem dependency graph

### Collection Size Tiers

| Tier | Collections | Total LOC | Repos |
|------|-----------|----------|-------|
| **Core Libraries** | phenoShared | ~80K | 6 |
| **Platform Runtime** | Sidekick, Stashly, PhenoObservability | ~370K | 13 |
| **Standalone Products** | (Self-contained) | ~640K | 4 |

---

## Interop Guarantees

### phenoShared Stability Contract
- **AuthKit**: 4-language API parity guarantee (Rust, TS, Python, Go)
- **phenoAI**: Provider-agnostic LLM routing (Anthropic, OpenAI, Ollama)
- **PhenoSchema**: Schema evolution without breaking changes

### Sidekick Skill System
- **phenotype-skills** bindings enable agent-agnostic skill definition
- Multi-language skill plugins (Python, TypeScript, Rust, Go)
- MCP tool adapter pattern for agent tool integration

### Isolation Guarantees (Stashly)
- **PhenoCompose** latency tiers: WASM (~1ms), gVisor (~90ms), Firecracker (~125ms)
- Workload-to-tier mapping: quick jobs → WASM; untrusted → gVisor/Firecracker

---

## Audit Metadata

**W-67F batch audit (commit 5d012e87d):**
- Mapping date: 2026-04-25
- Auditor method: Metadata extraction (LOC, tests, commit history, README analysis)
- Coverage: 8 repos, 0 builds
- Next review: W-68 (cross-collection integration tests)

---

## Updates in This Session

**Collections Updated:**
- **PhenoObservability**: +ObservabilityKit (7.9K LOC, multi-language OTEL SDKs)
- **Sidekick**: +PhenoAgent (22K), +phenotype-skills (12.3K)
- **phenoShared**: +phenoAI (373), +AuthKit (18.2K), +PhenoSchema (2.1K)
- **Stashly**: +PhenoCompose (4.3K)
- **Standalone**: +phenotype-omlx (138.8K, shipped consumer product)

**New Pattern Recognized:**
- phenotype-omlx is org's first standalone consumer product (outside microservice ecosystem)
- Reference implementation for local LLM optimization on macOS

**Updated Collection Rosters:**
- PhenoObservability: 5 repos (was 3)
- Sidekick: 5 repos (was 2)
- phenoShared: 6 repos (was 3)
- Stashly: 3 repos (was 1)
- Standalone Products: 4 repos (was 3, +phenotype-omlx)
