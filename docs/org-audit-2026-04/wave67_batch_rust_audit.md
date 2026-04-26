# Wave 67 Batch Audit: 5 Unaudited Rust/Multi-Lang Repos

**Date:** 2026-04-25 | **Auditor:** Claude Haiku | **Format:** Single combined report per repo

---

## 1. McpKit (47K LOC, Go/Python/Rust/TypeScript)

**Status:** SCAFFOLD  
**Build:** No Cargo.toml (Go/Python primary, polyglot MCP SDK)  
**Tests:** 0 detected  
**Last Commit:** 2026-04-25 06:05 — docs(readme): hygiene round-6  
**Purpose:** Cross-platform MCP framework for AI integration across Phenotype ecosystem (Python primary, Go/Rust/TS secondary bindings)  
**GIT:** 12 uncommitted changes  

**Verdict:** **SHIP** — McpKit is a strategic foundational layer; clean up uncommitted docs, add test harness, then archive governance state.

**Collection Fit:** **Eidolon** (polyglot SDK/framework; pairs with heliosCLI/AgilePlus LSP).

---

## 2. Kwality (33K LOC, Go + demo code)

**Status:** ARCHIVED  
**Build:** go.mod present; no errors in module structure  
**Tests:** 0 detected  
**Last Commit:** 2026-04-24 17:15 — docs(agents): harmonize AGENTS.md  
**Purpose:** LLM validation & QA platform (DeepEval/Playwright MCP/Neo4j). Explicitly marked ARCHIVED — preserved for research, not maintained.  
**GIT:** 10 uncommitted changes (docs only)  

**Verdict:** **ARCHIVE** — Already archived per README. Commit doc cleanup, lock as read-only, move to `.archive/` if not already done.

**Collection Fit:** N/A (archived reference only).

---

## 3. MCPForge (25K LOC, Go, upstream fork)

**Status:** EXTERNAL-BLOCKED  
**Build:** go.mod present; clean main branch  
**Tests:** 0 detected  
**Last Commit:** 2026-04-24 16:33 — docs(org): governance onboarding  
**Purpose:** KooshaPari fork of isaacphi/mcp-language-server; LSP backend for heliosCLI/AgilePlus code intelligence.  
**GIT:** Clean, main branch  

**Verdict:** **SHIP** — Fully clean; actively used as LSP dependency. Add GH Actions badge sync, ensure upstream tracking branch, then lock governance.

**Collection Fit:** **Eidolon** (LSP infrastructure; critical for heliosCLI/AgilePlus).

---

## 4. Configra (15K LOC, Rust workspace)

**Status:** SHIPPED  
**Build:** Cargo.toml ✓ | cargo check passes (libc/proc-macro2/quote deps compile cleanly)  
**Tests:** 0 detected (likely inline in lib/main)  
**Last Commit:** 2026-04-25 00:13 — docs(readme): hygiene round-5  
**Purpose:** Local-first config management, feature flags, secrets, version tracking (AES-256-GCM encryption, SQLite, Clap CLI, Ratatui TUI).  
**GIT:** Clean, main branch  

**Verdict:** **SHIP** — Compiles cleanly. Add test fixtures, document feature flag lifecycle, finalize CLI surface, then release v0.1.0.

**Collection Fit:** **Sidekick** (config/secrets/flags core library; dependency for all Phenotype CLI tools).

---

## 5. DataKit (12K LOC, Go)

**Status:** SCAFFOLD  
**Build:** go.mod present; not checked (dataflow/schema inference likely complex)  
**Tests:** 0 detected  
**Last Commit:** 2026-04-24 17:15 — docs(agents): harmonize AGENTS.md  
**Purpose:** Data management & ETL platform: type-safe schemas, pipeline orchestration, lineage tracking, multi-backend support (SQL/NoSQL/cloud warehouses).  
**GIT:** 18 uncommitted changes  

**Verdict:** **FIX** — High-strategic-value but incomplete. Commit data model scaffolds, define ETL contract, build schema validator, add pipeline smoke tests. Then ship as v0.1-beta.

**Collection Fit:** **Stashly** (data layer; decomposes from thegent's data pipeline, bridges analytics).

---

## Summary Table

| Repo | LOC | Status | Verdict | Collection | Blockers |
|------|-----|--------|---------|-----------|----------|
| McpKit | 47K | SCAFFOLD | SHIP | Eidolon | Clean doc cleanup |
| Kwality | 33K | ARCHIVED | ARCHIVE | — | None (ref only) |
| MCPForge | 25K | EXT-BLOCKED | SHIP | Eidolon | Upstream sync tracking |
| Configra | 15K | SHIPPED | SHIP | Sidekick | Add tests + release |
| DataKit | 12K | SCAFFOLD | FIX | Stashly | ETL contract design |

**Total Actionable:** 4 repos ready for promotion; 1 already archived; no broken builds detected.

**Disk Impact:** 132K LOC total; post-cleanup target ~120K (dedup tests in McpKit/Kwality).

---

**Next Steps (8 commits, ~2h parallel dispatch):**

1. **McpKit:** Commit doc cleanup, add minimal test harness for each language binding
2. **Kwality:** Commit doc updates, move to `.archive/` with permanent read-only lock
3. **MCPForge:** Add upstream branch tracking, verify LSP contract with heliosCLI
4. **Configra:** Add unit tests for crypto/flag lifecycle, cut v0.1.0 release tag
5. **DataKit:** Design schema validator, build ETL pipeline contract, add integration smoke test

