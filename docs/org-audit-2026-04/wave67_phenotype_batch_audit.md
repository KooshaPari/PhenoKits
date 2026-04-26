# Wave 67: Phenotype Novel Repos Batch Audit (2026-04-25)

Metadata-only audit of 8 phenotype-* repos and auxiliary observability/auth/schema tooling. Captures purpose, stack, maturity indicators, and collection fit. No build or deep code analysis.

---

## ObservabilityKit

**Purpose:** Unified OpenTelemetry SDKs and instrumentation for distributed tracing, metrics, and structured logging across Phenotype services.

**Stack:** Rust, Python, Go, TypeScript (multi-language) | OTEL standard (Tempo, Prometheus, Loki exporters)

**LOC:** 7.9K | **Tests:** 4 | **Last commit:** 2026-04-25

**Status:** ACTIVE — multi-language SDKs with framework integrations (actix, FastAPI, Gin, Express) and in-memory exporters for testing.

**Fit:** **PhenoObservability collection.** Provides unified instrumentation layer for all services; essential for health-dashboard and observability pipeline.

---

## PhenoAgent

**Purpose:** Distributed agent orchestration with plugin architecture, skill system, and multi-model routing for autonomous agentic workflows.

**Stack:** Rust (daemon, policy) + Go (CLI, API gateway) | gRPC + Protobuf | Event-sourced state (SQLite) | Tokio async

**LOC:** 22K | **Tests:** 1 | **Last commit:** 2026-04-25

**Status:** ACTIVE — daemon + CLI + skill registry with MCP tool adapters and multi-model inference routing (Claude, GPT-4, Gemini, Ollama).

**Fit:** **Sidekick collection.** Core agent framework for autonomous task execution; integrates with Phenotype CLI and workflow orchestration.

---

## phenoAI

**Purpose:** LLM routing, MCP server plumbing, and embedding primitives consolidating AI-facing building blocks for all Phenotype services.

**Stack:** Rust workspace (3 crates: llm-router, mcp-server, pheno-embedding) | Provider-agnostic routing (Anthropic, OpenAI, local)

**LOC:** 373 | **Tests:** 1 | **Last commit:** 2026-04-25

**Status:** ACTIVE — scaffolding phase, 3 core crates stabilizing. Prevents each agent repo from picking incompatible LLM client stacks.

**Fit:** **phenoShared library.** Shared LLM/MCP abstraction used by PhenoAgent, Sidekick, and other AI-driven services.

---

## PhenoCompose

**Purpose:** 3-tier isolation runtime (WASM ~1ms, gVisor ~90ms, Firecracker ~125ms) merged from KooshaPari/nanovms + BytePort/nvms.

**Stack:** Rust (compose driver) + Go (NVMS CLI) | WASM (wasmtime), gVisor, Firecracker isolation

**LOC:** 4.3K | **Tests:** 5 | **Last commit:** 2026-04-25

**Status:** ACTIVE — merged implementation providing unified interface for workload isolation; ready for production use.

**Fit:** **Stashly collection.** Provides secure execution environment for untrusted workloads (browser automation, tool sandboxing, code execution).

---

## phenotype-omlx

**Purpose:** LLM inference optimized for macOS with continuous batching and tiered KV caching, managed from menu bar.

**Stack:** Python 3.10+ (core inference engine) | Apple Silicon required | Menu bar UI integration

**LOC:** 138.8K | **Tests:** 229 | **Last commit:** 2026-04-25

**Status:** SHIPPED — standalone consumer product with benchmarks site (omlx.ai), full test coverage, multi-language docs (EN/ZH/KO/JA).

**Fit:** **Standalone.** User-facing LLM inference tool for macOS; outside core Phenotype microservice ecosystem. Reference for local model optimization patterns.

---

## phenotype-skills

**Purpose:** Skill registry and bindings for agent skill system (referenced by PhenoAgent framework).

**Stack:** Rust (core) | Python + TypeScript bindings | Cargo lock only (minimal scaffold)

**LOC:** 12.3K | **Tests:** 30 | **Last commit:** 2026-04-25

**Status:** SCAFFOLD — bindings directory present (python, typescript); core skill logic under construction. Part of larger PhenoAgent skill system.

**Fit:** **Sidekick/PhenoAgent subsystem.** Pluggable skills library for agent capability composition.

---

## AuthKit

**Purpose:** Cross-platform authentication SDK (OAuth 2.0, OIDC, SAML, WebAuthn) with unified API across Rust, TypeScript, Python, Go.

**Stack:** Rust + TypeScript + Python + Go (4-language) | OAuth 2.0, OpenID Connect, SAML 2.0, WebAuthn/Passkeys | Provider-agnostic (Auth0, Okta, Cognito, Keycloak, custom)

**LOC:** 18.2K | **Tests:** 2 | **Last commit:** 2026-04-24

**Status:** ACTIVE — language-unified authentication library with security-first design; ready for integration into services.

**Fit:** **phenoShared library.** De facto standard auth layer for all Phenotype services requiring identity/access management.

---

## PhenoSchema

**Purpose:** Schema management, evolution, and validation for data contracts with multi-format support (JSON Schema, Protobuf, Avro, OpenAPI).

**Stack:** Rust (core) + serde | jsonschema crate | Schema registry (Confluent-compatible API) | Code generation for TypeScript, Python, Go

**LOC:** 2.1K | **Tests:** 48 | **Last commit:** 2026-04-25

**Status:** ACTIVE — schema registry + evolution checking + code generators; strong test coverage (48 tests) despite small LOC.

**Fit:** **phenoShared library.** Data contract management across services; integrates with PhenoCompose, observability pipelines, and API documentation.

---

## Collection Fit Summary

| Repo | Ecosystem Role | Collection | Priority |
|------|----------------|-----------|----------|
| ObservabilityKit | Telemetry/tracing | PhenoObservability | High |
| PhenoAgent | Agent orchestration | Sidekick | High |
| phenoAI | LLM routing/MCP | phenoShared | High |
| PhenoCompose | Workload isolation | Stashly | High |
| phenotype-omlx | Consumer product | Standalone | Reference |
| phenotype-skills | Skill bindings | Sidekick subsystem | Medium |
| AuthKit | Authentication | phenoShared | High |
| PhenoSchema | Data contracts | phenoShared | High |

---

## Key Findings

1. **Active development:** All 8 repos have commits within last 24 hours (2026-04-25). No stale repos detected.
2. **High maturity variance:** phenotype-omlx (138K LOC, 229 tests) is production-grade consumer app; phenoAI (373 LOC, 1 test) is early-stage shared lib.
3. **Cross-cutting patterns:** phenoAI, AuthKit, PhenoSchema form a shared library tier used by PhenoAgent, PhenoCompose, and all downstream services.
4. **Test coverage gaps:** PhenoAgent and AuthKit have very few tests (1–2 each) despite 18–22K LOC. PhenoSchema over-tested relative to LOC (48 tests in 2K), suggesting high validation/contract focus.
5. **Multi-language commitment:** ObservabilityKit, AuthKit, and PhenoSchema all support 3+ languages (Rust, Go, Python, TypeScript), indicating mature polyglot architecture.

---

**Audit Date:** 2026-04-25  
**Method:** Metadata extraction (LOC, test count, commit history, README analysis)  
**Coverage:** 8 repos, 0 builds, 0 code inspection
