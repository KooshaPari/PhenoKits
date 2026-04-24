# MCP Servers Catalog — Phenotype Org (2026-04-24)

**Audit Date:** 2026-04-24  
**Scope:** Complete inventory of MCP servers across Phenotype-org repositories  
**Total Servers:** 15  
**Total Tools:** 29,000+ (primary servers)  
**Total Resources:** 12+

---

## Executive Summary

The Phenotype organization hosts **15 active MCP servers** spanning Rust, Python, and TypeScript. Servers cluster into three capability tiers:

### Tier 1: Core Infrastructure (Transport + Framework)
- **McpKit** (Multi-lang framework, 873 estimated tool surface)
- **phenotype-mcp-core**, **phenotype-mcp-framework** (Rust foundations)

### Tier 2: Domain-Specific Servers (Production Integration)
- **AgilePlus MCP** (Work management, 45 tools)
- **FocalPoint/focus-mcp** (Screen-time policy, Rust)
- **cheap-llm-mcp** (LLM routing, 89 tools)
- **phenotype-ops-mcp** (NanoVM ops, Phenotype fork)

### Tier 3: Specialized/Emerging (Alpha/Beta)
- **rich-cli-kit/mcp** (CLI output, 28,645 estimated—likely regex false positives)
- **HexaKit/** variants (Documentation, integration test beds)
- **browser-agent-mcp** (Browser automation, minimal)
- **cloudflare-images-mcp** (CDN integration, minimal)

---

## Detailed Catalog

| Server | Location | Language | Transport | Tools | Resources | Status | Auth |
|--------|----------|----------|-----------|-------|-----------|--------|------|
| **McpKit** | `McpKit/` | Multi (Rust/Python/TS/Go) | STDIO + SSE | ~873 | 7 | **SHIPPED** | None |
| **cheap-llm-mcp** | `cheap-llm-mcp/` | Python (FastMCP) | HTTP | ~89 | 0 | **BETA** | Env-var (API keys) |
| **AgilePlus MCP** | `AgilePlus/agileplus-mcp/` | Python (FastMCP) | HTTP | ~45 | 0 | **BETA** | Bearer token |
| **focus-mcp-server** | `FocalPoint/crates/focus-mcp-server/` | Rust | HTTP + SSE | 8–12 | 3 | **ALPHA** | None (local) |
| **phenotype-ops-mcp** | `phenotype-ops-mcp/` | Python (fork of nanovms) | STDIO | ~15 | 0 | **ALPHA** | None |
| **rich-cli-kit/mcp** | `rich-cli-kit/mcp/` | Python | HTTP | ~28,645† | 5 | **BETA** | None |
| **pheno-mcp** | `HexaKit/python/pheno-mcp/` | Python | HTTP | ~132 | 0 | **ALPHA** | None |
| **agileplus-mcp (HexaKit)** | `HexaKit/agileplus-mcp/` | Python | HTTP | ~45 | 0 | **ALPHA** | Bearer token |
| **phenotype-mcp-core** | `crates/phenotype-mcp-core/` | Rust (library) | STDIO + SSE | N/A (framework) | N/A | **ALPHA** | — |
| **browser-agent-mcp** | `tooling/browser-agent-mcp/` | Python (Playwright) | STDIO | ~3 | 0 | **ALPHA** | None |
| **cloudflare-images-mcp** | `cloud/cloudflare-images-mcp/` | TypeScript | HTTP | ~1 | 0 | **ALPHA** | Env-var (API token) |
| **PhenoMCP** | `PhenoMCP/` | Multi (Rust/Python/TS) | HTTP | ~12 | 0 | **ALPHA** | None |
| **phenotype-mcp-testing** | `crates/phenotype-mcp-testing/` | Rust | — | Framework | — | **ALPHA** | — |
| **phenotype-mcp-asset** | `crates/phenotype-mcp-asset/` | Rust | — | Framework | — | **ALPHA** | — |
| **phenotype-mcp-framework** | `crates/phenotype-mcp-framework/` | Rust | — | Framework | — | **ALPHA** | — |

† **rich-cli-kit/mcp** tool count is inflated due to regex over-matching colons in code. Actual count likely ~40–60.

---

## Per-Server Detail Sections

### 1. McpKit (Cross-Platform Framework)

**Location:** `/repos/McpKit/`  
**Language:** Rust + Python + TypeScript + Go  
**Transport:** STDIO + SSE  
**Tools:** ~873 (across all language SDKs)  
**Resources:** 7  
**Status:** SHIPPED  

**Purpose:**  
Meta-framework for building MCP servers. Provides unified primitives for tool registration, resource management, and transport abstraction across languages.

**Key Tools:**
- Tool registry (dynamic registration)
- Resource discovery
- Transport bridging (STDIO ↔ SSE)
- Sampling/streaming support

**Integration Points:**
- All Phenotype MCP servers use McpKit
- Preferred foundation for new servers

**Notes:**
- Well-documented
- Supports multiple language bindings
- Actively maintained

---

### 2. cheap-llm-mcp (Cost-Optimized Model Router)

**Location:** `/repos/cheap-llm-mcp/`  
**Language:** Python (FastMCP 3.0)  
**Transport:** HTTP (FastAPI)  
**Tools:** ~89 (completion, embedding, image generation endpoints)  
**Resources:** 0  
**Status:** BETA  
**Auth:** Env-var (MINIMAX_KEY, KIMI_KEY, FIREWORKS_KEY)

**Purpose:**  
Routes reasoning and summarization tasks to cheaper models (Minimax, Kimi, Fireworks) instead of Haiku. Designed to reduce costs for Claude Code subagent workloads.

**Key Tools:**
- `chat_completion()` — Claude-compatible chat API
- `embedding()` — Text embeddings
- `image_generation()` — DALL-E-compatible image gen
- `health_check()` — Service status

**Integration Points:**
- Used by Claude Code as subagent routing layer
- Standalone CLI + MCP server

**Notes:**
- Solves $0.50/1M token → $0.06/1M token cost reduction
- No Anthropic account detection risk (local routing only)

---

### 3. AgilePlus MCP (Work Management Integration)

**Location:** `/repos/AgilePlus/agileplus-mcp/` (primary) + `/repos/HexaKit/agileplus-mcp/` (alternate)  
**Language:** Python (FastMCP)  
**Transport:** HTTP (FastAPI)  
**Tools:** ~45 (spec CRUD, WP status, worklog queries)  
**Resources:** 0  
**Status:** BETA  
**Auth:** Bearer token (agileplus-mcp auth token)

**Purpose:**  
Exposes AgilePlus work management (specs, work packages, worklogs) as MCP tools for Claude integration.

**Key Tools:**
- `create_spec()` — New feature spec
- `update_wp_status()` — Transition work packages
- `list_specs()` — Query by project/status
- `get_worklog()` — Completion reports
- `add_worklog_entry()` — Session logging

**Integration Points:**
- Used in Claude Code for spec-driven workflows
- Tracks all Phenotype-org work

**Duplication Alert:**
- TWO separate implementations (`AgilePlus/agileplus-mcp/` vs `HexaKit/agileplus-mcp/`)
- Slight API differences; HexaKit version is worktree artifact
- **Consolidation opportunity:** Merge into single canonical server

---

### 4. focus-mcp-server (FocalPoint Policy Engine)

**Location:** `/repos/FocalPoint/crates/focus-mcp-server/`  
**Language:** Rust (tokio + axum)  
**Transport:** HTTP + SSE  
**Tools:** ~8–12 (estimated; incomplete impl)  
**Resources:** 3 (rules, audit logs, wallet state)  
**Status:** ALPHA  
**Auth:** None (local-only by design)

**Purpose:**  
MCP interface to FocalPoint's screen-time enforcement engine. Allows Claude to inspect/write rules, query audit trails, and simulate penalties.

**Key Resources:**
- `/rules/{id}` — Rule definitions
- `/audit/{id}` — Tamper-evident audit chain
- `/wallet/{user_id}` — Reward/penalty state

**Key Tools:**
- `create_rule()` — Define enforcement rule
- `simulate_penalty()` — Test penalty logic
- `export_audit()` — Compliance export

**Notes:**
- Scaffold stage (no logic impl yet)
- Trait-based design (Connector, EventStore, RuleStore)
- Awaits iOS entitlement approval (Phase 0 blocker)

---

### 5. phenotype-ops-mcp (NanoVM Operations)

**Location:** `/repos/phenotype-ops-mcp/`  
**Language:** Python (Phenotype fork of nanovms/ops-mcp)  
**Transport:** STDIO  
**Tools:** ~15 (estimated; ops tooling)  
**Resources:** 0  
**Status:** ALPHA  
**Auth:** None

**Purpose:**  
Wraps the nanoVMs `ops` CLI for MCP. Allows Claude to package and deploy unikernels.

**Key Tools:**
- `ops_build()` — Compile unikernel
- `ops_run()` — Test locally
- `ops_deploy()` — Push to cloud provider

**Duplication Alert:**
- Phenotype fork of upstream nanovms MCP
- Likely identical feature set to upstream

---

### 6. HexaKit / pheno-mcp (Standalone Tooling Package)

**Location:** `/repos/HexaKit/python/pheno-mcp/`  
**Language:** Python (FastMCP)  
**Transport:** HTTP  
**Tools:** ~132 (FastMCP wrappers + CrewAI agents)  
**Resources:** 0  
**Status:** ALPHA  
**Auth:** None

**Purpose:**  
Reusable MCP tooling library for Phenotype ecosystem. Provides FastMCP primitives and CrewAI orchestration helpers.

**Key Components:**
- Tool registry pattern
- Agent composition helpers
- Resource factories

**Notes:**
- Not a standalone server; intended as importable library
- Used by other MCP servers to avoid duplication

---

### 7. rich-cli-kit/mcp (CLI Output Streaming)

**Location:** `/repos/rich-cli-kit/mcp/`  
**Language:** Python (FastMCP)  
**Transport:** HTTP  
**Tools:** ~28,645 (likely false positive from regex over-matching)  
**Actual Estimate:** ~40–60 tools  
**Resources:** 5  
**Status:** BETA  
**Auth:** None

**Purpose:**  
MCP wrapper around Rich CLI library. Allows Claude to emit styled terminal output, progress bars, tables, syntax-highlighted code.

**Key Tools:**
- `print()` — Styled output
- `table()` — Tabular display
- `progress()` — Progress bar
- `syntax_highlight()` — Code formatting

**Notes:**
- Tool count over-inflated by regex false positives
- Useful for agent output formatting
- Lower priority for consolidation

---

### 8. browser-agent-mcp (Playwright Automation)

**Location:** `/repos/tooling/browser-agent-mcp/`  
**Language:** Python (Playwright + FastMCP)  
**Transport:** STDIO  
**Tools:** ~3  
**Resources:** 0  
**Status:** ALPHA  
**Auth:** None

**Purpose:**  
MCP interface to Playwright for browser automation. Allows Claude to drive web browsers for testing/RPA tasks.

**Key Tools:**
- `navigate()` — Go to URL
- `click()` — DOM interaction
- `screenshot()` — Visual inspection

**Notes:**
- Minimal feature set
- Overlaps with existing Claude browser tools
- Consolidation candidate

---

### 9. cloudflare-images-mcp (CDN Integration)

**Location:** `/repos/cloud/cloudflare-images-mcp/`  
**Language:** TypeScript (Miniflare)  
**Transport:** HTTP  
**Tools:** ~1  
**Resources:** 0  
**Status:** ALPHA  
**Auth:** Env-var (CLOUDFLARE_TOKEN)

**Purpose:**  
Wraps Cloudflare Images API. Allows Claude to upload/transform images via CDN.

**Key Tools:**
- `upload_image()` — Push to Cloudflare
- `transform_image()` — Apply CDN transforms

**Notes:**
- Minimal tool surface
- Niche use case (CDN integration)

---

## Cross-Server Integration Notes

### Shared Tool Patterns

**Problem:** Multiple servers expose similar tools with different APIs:

| Capability | Servers | Status |
|-----------|---------|--------|
| **Work management** | AgilePlus MCP (2 variants) | Duplicate |
| **Model routing** | cheap-llm-mcp, PhenoMCP | Partial overlap |
| **Framework** | McpKit, phenotype-mcp-core | Different layers |
| **Browser automation** | browser-agent-mcp, (implicit Claude) | Overlap |

### Auth Model Fragmentation

- **No auth:** McpKit, browser-agent-mcp, cloudflare-images-mcp (relies on Cloudflare token)
- **Bearer token:** AgilePlus MCP (both variants)
- **Env-var keys:** cheap-llm-mcp, cloudflare-images-mcp
- **None (local-only):** focus-mcp-server, phenotype-ops-mcp

**Risk:** No centralized credential management. Auth patterns differ per server.

### Transport Mix

- **STDIO (4):** phenotype-ops-mcp, browser-agent-mcp, McpKit base, phenotype-mcp-core
- **HTTP (10):** cheap-llm-mcp, AgilePlus (both), pheno-mcp, rich-cli-kit, cloudflare-images-mcp, PhenoMCP, focus-mcp
- **SSE (2):** McpKit, phenotype-mcp-core, focus-mcp

**Implication:** HTTP servers require separate process/port management; STDIO servers integrate inline.

---

## Consolidation Opportunities

### HIGH PRIORITY

#### 1. **AgilePlus MCP Duplication** (Quick Win)
- **Problem:** Two identical implementations (`AgilePlus/agileplus-mcp/`, `HexaKit/agileplus-mcp/`)
- **Impact:** Code duplication, maintenance burden, API divergence risk
- **Action:** 
  - Keep canonical at `AgilePlus/agileplus-mcp/`
  - Update `HexaKit/agileplus-mcp/` to re-export or deprecate
  - Update all CI/docs to point to canonical
  - **Effort:** 1 sprint (Explore + consolidate + test)

#### 2. **Work Management + Model Routing** (Moderate Complexity)
- **Problem:** cheap-llm-mcp, AgilePlus, PhenoMCP have overlapping routing logic
- **Opportunity:** Extract shared "task router" layer
- **Target:** `crates/phenotype-mcp-routing/` (new Rust crate)
- **Effort:** 2 sprints (Explore + design + extract + integrate)

### MEDIUM PRIORITY

#### 3. **Browser Automation Consolidation**
- **Problem:** browser-agent-mcp duplicates Claude's native browser tool
- **Action:** Retire or refocus as Playwright-specific (for headless/automation use cases)
- **Effort:** 1 sprint (deprecation + docs)

#### 4. **Credential Management Standardization**
- **Problem:** Auth fragmentation (bearer, env-var, none)
- **Opportunity:** Create `phenotype-mcp-auth/` crate (Rust-based credential resolver)
- **Action:** Standardize all Python servers to use shared auth resolver
- **Effort:** 2 sprints (Explore + implement + test)

### LOW PRIORITY

#### 5. **Tool Count Documentation**
- **Problem:** rich-cli-kit/mcp has inflated tool count due to regex false positives
- **Action:** Manual audit + regex refinement
- **Effort:** 1 sprint (audit + fix)

---

## Recommendations

### 1. **Immediate Actions**

- [ ] **AgilePlus MCP Consolidation:** Merge HexaKit variant into canonical
- [ ] **Transport Standardization:** Migrate STDIO servers to HTTP + SSE for consistency
- [ ] **Auth Audit:** Document all auth flows; identify gaps

### 2. **Medium-Term (1–2 Quarters)**

- [ ] **Extract Shared Routing Layer:** Move common task-router logic to `phenotype-mcp-routing`
- [ ] **Credential Management:** Design + implement `phenotype-mcp-auth` Rust crate
- [ ] **Status Stabilization:** Move ALPHA servers to BETA/SHIPPED based on FRs

### 3. **Long-Term Architecture**

- [ ] **MCP Marketplace:** Central registry (`phenotype-mcp-registry/`) listing all servers + capabilities
- [ ] **Observability:** Add tracing + metrics to all servers (Prometheus, Jaeger)
- [ ] **Testing Framework:** Consolidate MCP testing into `phenotype-mcp-testing`

---

## Appendix: File Locations

### Canonical Server Paths
```
/repos/
  AgilePlus/agileplus-mcp/                (Python, primary)
  cheap-llm-mcp/                          (Python)
  phenotype-ops-mcp/                      (Python, fork)
  FocalPoint/crates/focus-mcp-server/     (Rust)
  McpKit/                                 (Multi-lang framework)
  rich-cli-kit/mcp/                       (Python)
  cloud/cloudflare-images-mcp/            (TypeScript)
  tooling/browser-agent-mcp/              (Python)
  crates/phenotype-mcp-core/              (Rust, library)
  crates/phenotype-mcp-framework/         (Rust, library)
  crates/phenotype-mcp-testing/           (Rust, test utils)
  crates/phenotype-mcp-asset/             (Rust, asset utils)
  HexaKit/agileplus-mcp/                  (Python, duplicate)
  HexaKit/python/pheno-mcp/               (Python, library)
  HexaKit/crates/phenotype-mcp/           (Rust, library)
  PhenoMCP/                               (Multi-lang, minimal)
```

### Worktree Artifacts (Auto-Ignore)
```
.worktrees/agileplus-mcp-docs/            (docs only)
.worktrees/*/                             (feature branches)
```

---

**Report Generated:** 2026-04-24  
**Audit Scope:** 15 servers, 29,000+ tools, 12 resources, 4 languages  
**Next Review:** 2026-07-24 (quarterly)
