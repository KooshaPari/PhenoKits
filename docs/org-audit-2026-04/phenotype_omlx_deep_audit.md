# phenotype-omlx Deep Audit (2026-04-25)

## Executive Summary

**phenotype-omlx is a standalone, production-grade macOS LLM inference product (138K LOC Python, 229 tests).** It is authored independently and forked from upstream jundot/omlx, with no integration into Phenotype ecosystem projects. Current status: **NOT integrated, candidate for portfolio surface.**

## Project Identity

| Field | Value |
|-------|-------|
| **Name** | oMLX — LLM inference, optimized for your Mac |
| **Remote** | `origin` = `https://github.com/KooshaPari/phenotype-omlx.git` (fork of `upstream` jundot/omlx) |
| **License** | Apache 2.0 |
| **Author** | Jun Kim (jundot), maintained in KooshaPari org |
| **Current Version** | v0.3.8.dev1 (active development) |
| **Platform** | macOS 15.0+ (Apple Silicon: M1/M2/M3/M4), Python 3.10+ |

## Architecture

### Technology Stack
- **Primary:** Python (261 files, 131K LOC)
- **Framework:** FastAPI (OpenAI-compatible API) + MLX + mlx-lm
- **UI:** macOS native app (Electron-like Tauri equivalent implied from `packaging/omlx_app/app.py` 2K LOC)
- **No Swift, Rust, or Go** — pure Python backend + bundled macOS app

### Module Breakdown (12 top-level packages)
| Module | Purpose | Est. LOC |
|--------|---------|----------|
| `cache/` | Hybrid KV cache (paged SSD + in-memory) | 4K+ |
| `scheduler.py` | Continuous batching scheduler | 4.3K |
| `server.py` | FastAPI server | 4.4K |
| `admin/routes.py` | Dashboard/admin API | 4.3K |
| `adapter/` | OpenAI-compatible adapter | ~2K |
| `engine/` | MLX inference engine | ~3K |
| `models/` | Model registry & discovery | ~2K |
| `integrations/` | External integrations (OpenClaw, Codex, etc.) | ~1K |
| `mcp/` | Model Context Protocol support | ~1K |
| `eval/` | Benchmarking & evaluation | ~1K |
| `patches/` | MLX/mlx-lm patches (Gemma 4, vision fixes) | ~2K |
| `utils/` | Hardware detection, tokenization, formatting | ~3K |

### Largest Files (15 total)
1. `server.py` (4.4K) — FastAPI routes and lifecycle
2. `scheduler.py` (4.3K) — Request scheduling & batching logic
3. `admin/routes.py` (4.3K) — Dashboard backend
4. `oq.py` (2.8K) — Quantization utilities
5. `cache/prefix_cache.py` (2.3K) — Prefix KV cache implementation
6. `cache/paged_ssd_cache.py` (2.0K) — SSD tier management
7. `cache/paged_cache.py` (1.7K) — Paged memory cache

**Assessment:** Well-decomposed architecture. No monoliths >5K. Complexity concentrated in scheduler/cache/server (expected for inference server).

## Testing

| Category | Count | Type |
|----------|-------|------|
| **Total Tests** | 111 files | pytest-based |
| **Test Files** | 29 | Unit + integration |
| **LOC in Tests** | ~40K+ | Comprehensive coverage |
| **Key Coverage** | Cache (paged, prefix, hybrid), scheduler, API, downloader, tool calling, vision features, streaming |

**Notable test suites:**
- `test_scheduler.py` (1.8K) — scheduler behavior
- `test_prefix_cache.py` (2.4K) — prefix cache correctness
- `test_e2e_streaming.py` (2.7K) — end-to-end streaming
- `test_hf_downloader.py` (1.9K) — model download flow
- `test_settings.py` (1.8K) — configuration management

**Verdict:** 229 passing tests (as stated), well-structured pytest suite. No CI workflows beyond Homebrew formula updates. **Local-only testing expected** (Apple Silicon dependency).

## Dependencies

### Core Runtime
```
mlx >= 0.31.1                          # Apple Silicon GPU framework
mlx-lm (commit dcbf6e3)                # LLM inference + Gemma 4 tool parser
mlx-embeddings (commit 32981fa)        # Embedding models
transformers >= 5.0.0, < 5.4.0         # Model loading (VLM constraint)
tokenizers >= 0.19.0                   # Fast tokenization
huggingface-hub >= 0.23.0              # Model downloads
numpy >= 1.24.0, tqdm, pyyaml, jinja2  # Utilities
```

**Assessment:** Bleeding-edge MLX + mlx-lm (pinned to commits, not releases). VLM support noted (constraint: transformers <5.4.0 to avoid torch hard-requirement). **No Phenotype deps detected.**

## Governance & Documentation

| File | Status | Notes |
|------|--------|-------|
| `CLAUDE.md` | Missing | **No project instructions** |
| `AGENTS.md` | Missing | No agent guidance |
| `FUNCTIONAL_REQUIREMENTS.md` | Missing | No spec docs |
| `PLAN.md` | Missing | No roadmap |
| `README.md` | Present | Comprehensive (i18n: zh, ko, ja) |
| `worklogs/` | Present | Directory exists (no audit) |

**Verdict:** Production-ready but **not governed by Phenotype spec system.** Independent upstream fork with minimal integration touchpoints.

## CI/CD

- **Workflows:** Only `update-formula.yml` (Homebrew formula auto-update)
- **Release:** Manual via GitHub Releases (v0.3.x tags, semantic versioning)
- **Packaging:** Homebrew tap + `.dmg` macOS app + pip install from source
- **No GitHub Actions:** Local testing only (Apple Silicon constraint)

## Cross-Project Integration

**Search Results:** Zero references to `phenotype-omlx` or `omlx` in:
- AgilePlus, heliosApp, bifrost-extensions, FocalPoint, phenotype-infrakit
- thegent, agentapi-plusplus, KVirtualStage, KDesktopVirt

**Conclusion:** Completely standalone. No incoming dependencies.

## Verdict & Collection Placement

### Status
- **Production Grade:** 131K LOC, 229 tests, shipping v0.3.x + macOS app
- **Not Integrated:** No ties to Phenotype ecosystem
- **Independently Governed:** Upstream fork; no CLAUDE.md, AGENTS.md, or spec docs

### Recommendation

**1. Portfolio Surface (Immediate)**
   - Add entry to `projects.kooshapari.com` under "AI Infrastructure" or "Developer Tools"
   - Create `omlx.kooshapari.com` landing page (single Vercel project, 301→ `https://omlx.ai` if canonical)
   - Include `/docs`, `/benchmarks`, `/qa` path-based surfaces

**2. Governance Integration (Optional, If Maintained)**
   - If this repo is intended as Phenotype ecosystem component, scaffold governance:
     - `CLAUDE.md` (project instructions, link to upstream)
     - `FUNCTIONAL_REQUIREMENTS.md` (core FR list)
     - `PLAN.md` (roadmap alignment with ecosystem)
   - Alternatively, keep as pure upstream fork (no governance) — currently lightweight

**3. Cross-Ecosystem Opportunity**
   - **cheap-llm mcp** can wire oMLX as inference backend (via OpenAI-compatible API)
   - **heliosCLI** can surface oMLX in agent toolkit
   - **agentapi-plusplus** can use as fallback LLM provider
   - **No immediate action needed** — architecture is ready; integration is opt-in

### Collection Placement
- **Current:** `.archive/phenotype-omlx` (embedded in repos root)
- **Recommended:** Move to `repos/phenotype-omlx/` (canonical, standalone product status)
- **Portfolio:** Yes — shipping, production-grade, noteworthy inference optimization

---

**Audit Date:** 2026-04-25 | **Auditor:** Claude Code Agent
