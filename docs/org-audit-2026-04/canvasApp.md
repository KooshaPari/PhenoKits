# canvasApp — 10-Dimension Audit + FocalPoint Overlap Analysis

**Project:** canvasApp (Canvas LMS Integration Tool)  
**LOC (src):** 351 Python (excl. venv, .history)  
**LOC (docs):** 6,594 Markdown  
**Last activity:** 2026-04-04  
**Maturity:** Proof-of-Concept (PoC)

---

## 10-Dimension Scorecard

| Dimension | Score | Notes |
|-----------|-------|-------|
| **Architecture** | 2/10 | Flat file structure; no separations of concerns; monolithic flow in canvas.py |
| **Code Quality** | 2/10 | No types, no error handling, hardcoded ASU instance, poor naming (e.g., `structredExtraction.py`), typos in docstrings |
| **Testing** | 0/10 | Zero test files; manual only |
| **Documentation** | 6/10 | Comprehensive spec, plan, PRD, ADR; but code itself has minimal docstrings |
| **Modularity** | 3/10 | 5 Python modules exist but lack boundaries; cross-imports with wildcard `from canvas import *` |
| **Error Handling** | 2/10 | Only bare `raise_for_status()` + top-level try/catch; no retry logic or graceful degradation |
| **Security** | 3/10 | API keys in config.json (good), but hardcoded base URL (ASU), no token rotation guidance in code |
| **Maintainability** | 2/10 | No types, inactive code blocks, inconsistent style; high bus factor |
| **CI/CD Readiness** | 1/10 | No CI workflows, no requirements.txt pinning, no build script |
| **Dependency Health** | 4/10 | Latest deps (requests, openai, google-auth) but no lock file or version pinning |

**Overall: 2.5/10 — Pre-alpha proof-of-concept; not production-ready.**

---

## FocalPoint Overlap Analysis

### canvasApp Scope
- **Canvas LMS connector** (REST API polling, OAuth token-based auth)
- **Course/assignment/announcement retrieval**
- **Google Calendar sync** (event creation from assignments)
- **AI-powered announcement summarization** (GPT)
- **Target:** Student productivity (personal use, ASU-specific)

### FocalPoint connector-canvas Scope (Rust)
- **Canvas LMS connector** (REST API polling, OAuth2 code-flow)
- **Assignment/submission/announcement event normalization**
- **FocalPoint rule engine integration** (emit `NormalizedEvent`s)
- **Multi-instance support** (generic Canvas, not ASU-specific)
- **Confidence scoring, dedupe, cursor-based sync**
- **Target:** Cross-connector event hub (production SaaS)

### Overlap Assessment

**YES, significant overlap:**
1. **Canvas API polling** — both fetch `/api/v1/courses`, assignments, announcements
2. **REST client abstraction** — both implement pagination + error handling
3. **OAuth authentication** — both handle Canvas OAuth (though canvasApp is token-based, FocalPoint is code-flow)
4. **Data transformation** — both map Canvas objects to internal event structs

**Differences:**
- **Language:** canvasApp = Python; FocalPoint = Rust
- **Use case:** canvasApp = student tool; FocalPoint = event aggregation platform
- **Architecture:** canvasApp = direct calendar sync; FocalPoint = normalized events → rules
- **Maturity:** canvasApp = PoC; FocalPoint = production scaffold

---

## De-duplication Recommendation

### Keep FocalPoint connector-canvas
- **Why:** Production-ready Rust implementation, generic Canvas support, integration with rule engine
- **Status:** Scaffold (v0.0.1), actively developed, aligns with Phenotype architecture
- **LOC:** 3K, well-modularized, tested

### Archive canvasApp
- **Why:** PoC only; overlaps 80% with FocalPoint; Python incompatible with Phenotype scripting policy; no active consumers
- **Candidates for migration:**
  - **AI announcement summarization** → Extract `scan_announcement()` logic → FocalPoint filter/transformer
  - **Google Calendar sync** → Extract calendar integration → FocalPoint `connector-gcal` rule
  - **Student-specific workflows** → Build as FocalPoint rule policy (TOML-based), not separate tool

### Migration Path
1. **Phase 1:** Extract announcement summarization logic into FocalPoint `transformers/` (Rust or Python FastMCP)
2. **Phase 2:** Build Google Calendar rule in FocalPoint policy layer (TOML config)
3. **Phase 3:** Archive canvasApp as reference docs (`docs/research/canvasApp-reference.md`)

### Decision
- **Merge:** No. Architectures are incompatible.
- **Reference:** Yes. Keep docs as reference for feature backlog (grade tracking, motion scheduling, syllabus parsing).
- **Archive:** Yes. Move to `.archive/` when migration complete.

---

## Risk Flags

- **Hardcoded ASU instance:** Blocks multi-tenant use
- **No credentials vault:** Config.json in tree (even with .gitignore, a risk)
- **Zero test coverage:** Impossible to refactor safely
- **Inactive for 20+ days:** No recent commits beyond scaffolding
- **Python violates scripting policy:** New Canvas integrations must use Rust (per `docs/governance/scripting_policy.md`)

---

## Recommendation

**Archive and migrate.** canvasApp is a valuable PoC but conflicts with FocalPoint's production direction. Preserve learnings in ADRs, then build Canvas features in FocalPoint using Rust + rule engine architecture.
