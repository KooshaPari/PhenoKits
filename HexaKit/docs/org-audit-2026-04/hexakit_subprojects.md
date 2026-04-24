# HexaKit Subprojects Audit — 2026-04

**Scope:** 79 inventoried subprojects (root + nested across apps/, crates/, libs/, packages/)
**Classification:** PRODUCT (6), LIBRARY (27), SNIPPET (32), EMPTY (14)
**Total LOC:** ~60K Rust + ~9K Python + ~3K Node

---

## Summary

HexaKit is a **workspace aggregate** housing:
- **2 root PRODUCTS** (agileplus, python) with 17K LOC
- **27 LIBRARIES** (crates, libs, packages) with 37K+ LOC
- **32 SNIPPETS** (stubs, utilities <500 LOC each)
- **14 EMPTY** projects (scaffolding, WIP)

**Key Finding:** Most crates should remain in HexaKit as shared infrastructure. However, **6 top-tier products** (agileplus, phenotype-policy-engine, phenotype-retry, phenotype-xdd-lib, clikit, python workspace) merit consideration for independent repo extraction if they're used across multiple Phenotype projects.

---

## Master Inventory

### Root-Level Projects (13 entries)

| Project | Classification | LOC | Stack | Status | Notes |
|---------|----------------|-----|-------|--------|-------|
| **agileplus** | PRODUCT | 8,108 | Rust | Active | Core project management tool; spans CLI, dashboard, gRPC |
| **python** | PRODUCT | 9,004 | Python | Active | Shared Python workspace (phench, agents, utilities) |
| **phenotype-policy-engine** | PRODUCT | 4,456 | Rust/Cargo | Active | High-value shared library; policy rule evaluation engine |
| **clikit** | LIBRARY | 1,010 | Rust | Active | Universal CLI framework; hexagonal architecture |
| **forgecode-fork** | LIBRARY | 868 | Rust | Stable | Forked codebase; maintained |
| **agileplus-mcp** | LIBRARY | 542 | Python | Active | MCP integration layer |
| **rust** | SNIPPET | 52 | Rust | Stub | Workspace root placeholder |
| **docs** | SNIPPET | 106 | Node | Infrastructure | VitePress docsite |
| **agileplus-agents** | SNIPPET | 20 | Rust | WIP | Agent harnesses |
| **bifrost** | EMPTY | 0 | — | Stub | Placeholder |
| **Eventra** | EMPTY | 0 | — | Stub | Placeholder |
| **Flowra** | EMPTY | 0 | — | Stub | Placeholder |
| **helMo** | SNIPPET | 2 | — | Stale | Minimal scaffold |
| **koosha-portfolio** | EMPTY | 0 | — | Inactive | Personal portfolio |

### Crates Workspace (37 entries)

**Tier 1 — Extraction Candidates (4):**

| Crate | LOC | Notes |
|-------|-----|-------|
| phenotype-policy-engine | 4,456 | Rule-based policy evaluation; TOML config; used across Phenotype |
| phenotype-retry | 3,312 | Retry logic with exponential backoff; widely used |
| phenotype-xdd-lib | 2,988 | XDD (eXtreme Driven Design) patterns and utilities |
| phenotype-port-traits | 2,008 | Hexagonal ports abstraction; foundational architecture |

**Tier 2 — Shared Utilities (23 LIBRARIES):**
- phenotype-testing (2,126), phenotype-infrastructure (1,648), phenotype-health (1,576), phenotype-mock (1,516), phenotype-cost-core (1,480)
- phenotype-bdd (1,358), phenotype-event-sourcing (1,284), phenotype-iter (1,094), phenotype-project-registry (1,088), phenotype-shared-config (1,106)
- phenotype-macros (1,070), phenotype-contracts (1,026), phenotype-state-machine (1,012), cipher (2,432)
- phenotype-casbin-wrapper (624), phenotype-event-bus (786), phenotype-logging (550), phenotype-test-infra (574)
- phenotype-string (876), phenotype-telemetry (445), phenotype-sentry-config (290), phenotype-security-aggregator (232)
- phenotype-error-core (443), phenotype-error-macros (224)

**Tier 3 — Stubs & Minimal (14 SNIPPETS):**
- phenotype-observability, phenotype-dashboard (server/client), phenotype-guard, pheno-guard, phenotype-config-core, phenotype-crypto, phenotype-git-core, phenotype-http-client-core, phenotype-process, phenotype-mcp, phenotype-rate-limit(er), phenotype-validation, phenotype-time, phenotype-contracts

### Libs & Packages Workspaces (6 entries)

| Project | LOC | Stack | Type | Notes |
|---------|-----|-------|------|-------|
| nexus | 1,080 | Rust/Cargo | LIBRARY | Shared utilities |
| pheno-resilience | 1,534 | Node | LIBRARY | Resilience patterns (TS/JS) |
| pheno-llm | 758 | Node | LIBRARY | LLM integration utilities |
| pheno-core | 844 | Node | LIBRARY | Core TypeScript utilities |
| phenotype-config-core (libs/) | 142 | Rust | SNIPPET | Config loading stub |

### Apps Workspace (1 entry)

| App | Status | Notes |
|-----|--------|-------|
| **byteport** | EMPTY | No buildable code; likely extracted to standalone repo |

---

## Extraction Candidates (Top 6 by Value)

**Recommended for independent repos if used across 3+ Phenotype projects:**

1. **phenotype-policy-engine** (4,456 LOC)
   - Status: Mature, well-documented
   - Use case: Rule-based policy evaluation for security, compliance, permissions
   - Action: Extract if depended on by phenotype-infrakit, AgilePlus, agentapi-plusplus

2. **phenotype-retry** (3,312 LOC)
   - Status: Stable, production-ready
   - Use case: Exponential backoff, retry strategies for async operations
   - Action: Extract if used in 3+ services (likely: phenotype-infrakit, thegent, cliproxyapi-plusplus)

3. **phenotype-xdd-lib** (2,988 LOC)
   - Status: Foundational architecture library
   - Use case: XDD patterns, design models, domain-driven design utilities
   - Action: Extract if adopted as standard in Phenotype-org; currently likely internal-only

4. **phenotype-port-traits** (2,008 LOC)
   - Status: Hexagonal architecture abstraction
   - Use case: Port/adapter pattern traits, interface contracts
   - Action: Extract if adopted across multiple microservices

5. **clikit** (1,010 LOC, Rust)
   - Status: Active, well-documented
   - Use case: Universal CLI framework with hexagonal architecture
   - Action: Extract if planning multi-service CLI unification

6. **phenotype-testing** (2,126 LOC)
   - Status: Test utilities and fixtures
   - Use case: Shared test infrastructure (BDD, fixtures, matchers)
   - Action: Extract as companion to extracted libraries above

---

## Archive Candidates (Bottom 14 — Empty/Stale)

These have minimal/zero LOC and appear to be scaffolding, stubs, or inactive projects. **Recommend moving to `.archive/` if not actively used:**

| Project | LOC | Status | Reason |
|---------|-----|--------|--------|
| bifrost | 0 | Stub | Never developed |
| Eventra | 0 | Stub | Never developed |
| Flowra | 0 | Stub | Never developed |
| koosha-portfolio | 0 | Inactive | Personal portfolio, not product |
| helMo | 2 | Stale | Minimal content |
| phenotype-observability | 0 | WIP | Stubbed, not completed |
| agileplus-dashboard-server | 0 | WIP | Framework scaffolding |
| agileplus-dashboard | 0 | WIP | Framework scaffolding |
| pheno-guard | 0 | WIP | Not implemented |
| phenotype-crypto | 1 | Stub | Placeholder only |
| phenotype-git-core | 1 | Stub | Placeholder only |
| phenotype-http-client-core | 1 | Stub | Placeholder only |
| phenotype-process | 1 | Stub | Placeholder only |
| phenotype-mcp | 1 | Stub | Placeholder only |
| byteport | 0 | Extracted | Already standalone repo |

---

## Workspace Organization Health

**Strengths:**
- Clean separation: crates/ (Rust), libs/ (Rust), packages/ (Node), apps/ (standalone)
- Consistent naming: phenotype-* prefix for shared infrastructure
- Good documentation: 31/79 projects have README.md

**Issues:**
- 14 empty/stub projects cluttering inventory (low S/N ratio)
- Some duplication: phenotype-config-core appears in both crates/ and libs/
- agileplus-dashboard (client/server) never completed; scaffolding only
- Cleanup opportunity: move stubs to `.archive/` for clarity

---

## Metrics Summary

| Metric | Value |
|--------|-------|
| Total Subprojects | 79 |
| Products (standalone, 3K+ LOC) | 6 |
| Libraries (shared, 500-3K LOC) | 27 |
| Snippets (utilities, <500 LOC) | 32 |
| Empty/Stubs | 14 |
| With README.md | 31/79 (39%) |
| Total LOC (Rust + Python + Node) | ~72K |
| Average Crate Size | 926 LOC |

---

## Recommendations

1. **Extract High-Value Libraries** → phenotype-policy-engine, phenotype-retry, phenotype-xdd-lib if adoption metrics show 3+ dependent projects
2. **Archive Stubs** → Move bifrost, Eventra, Flowra, helMo, agileplus-dashboard (client/server), and empty phenotype-* to `.archive/`
3. **De-duplicate Config** → Consolidate phenotype-config-core (crates/ + libs/) to single canonical location
4. **Document Usage** → Add dependency matrix linking crates to consumers (AgilePlus, phenotype-infrakit, thegent, etc.)
5. **README Completion** → 48 projects lack README.md; prioritize Tier 1-2 libraries and products

---

## Notes

- **BytePort, Profila, Stashly:** Not found in HexaKit; likely already extracted to standalone repos or located elsewhere in Phenotype-org
- **Scope:** This audit covers HexaKit root + 1 level of nesting. Deep nested workspaces (e.g., within platforms/thegent/) are out of scope
- **Methodology:** LOC counted via grep on .rs, .go, .py, .ts, .js source files; classification by LOC threshold (3K = PRODUCT, 500-3K = LIBRARY, <500 = SNIPPET)
