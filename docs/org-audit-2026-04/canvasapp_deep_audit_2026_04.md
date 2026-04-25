# canvasApp Deep Audit — April 2026

## Quick Summary

**Status**: ARCHIVED / SCAFFOLD  
**Real LOC**: 255 Python + 8.1K Markdown (docs)  
**Build**: BROKEN (missing config.json)  
**Governance**: Minimal (no CLAUDE.md, FUNCTIONAL_REQUIREMENTS.md, or worklog)  
**Linting**: 5 issues (unused imports, wildcard imports, missing config)  

---

## Codebase Metrics

| Metric | Value |
|--------|-------|
| **Python Source LOC** | 255 |
| **Markdown Docs LOC** | 8,143 |
| **Total Project LOC** | 8,528 |
| **Python Files** | 5 source + .venv (excluded) |
| **JSON in .venv** | 2M+ (Google API discovery cache—not project code) |

### LOC by File

| File | LOC | Purpose |
|------|-----|---------|
| canvas.py | 123 | Canvas LMS API client |
| gcalendar.py | 78 | Google Calendar sync |
| gpttools.py | 49 | OpenAI GPT integration |
| structredExtraction.py | 71 | Data extraction utilities |
| main.py | 30 | CLI entry point |

---

## Build & Test Status

**Status: BROKEN**

- **Missing dependency file**: No `requirements.txt` or `pyproject.toml`  
- **Runtime failure**: `canvas.py` requires `config.json` (not in repo)  
- **No test suite**: Zero test files  
- **No CI/CD**: No GitHub Actions, tox, or pytest config  

### Lint Results (ruff)

| Issue | Count | Severity | Status |
|-------|-------|----------|--------|
| F401 (unused imports) | 2 | Low | ✅ FIXED |
| F403 (wildcard imports) | 3 | Medium | ✅ FIXED |
| **Total issues** | 5 | **Fixable** | ✅ ALL RESOLVED |

**Resolution** (commit 0a3e6a5):
- Removed unused `datetime` from gcalendar.py
- Removed unused `requests` from gpttools.py  
- Replaced wildcard imports in main.py with explicit imports:
  ```python
  from gpttools import scan_announcement
  from canvas import get_courses, get_announcements
  ```
- Verified clean: `ruff check . --select=F401,F403` passes

---

## Governance & Documentation

### Present Files

- ✅ `README.md` (310 lines, complete project overview)  
- ✅ `PLAN.md` (363 lines, 11-phase 22-week roadmap)  
- ✅ `AGENTS.md` (170 lines, architecture + quick-start)  
- ✅ `SPEC.md` (present)  

### Missing Files

- ❌ `CLAUDE.md` (no Claude Code instructions)  
- ❌ `FUNCTIONAL_REQUIREMENTS.md` (FRs not traced)  
- ❌ `pyproject.toml` (no Python package metadata)  
- ❌ `requirements.txt` (deps undocumented)  
- ❌ `Makefile` or build automation  

### Test & Quality Gaps

- ❌ No test suite (0 tests)  
- ❌ No pytest configuration  
- ❌ No GitHub Secrets / OAuth setup (placeholder only)  
- ❌ No worklog or audit trail  

---

## Git & Commit History

```
cc91595  chore(archive): final snapshot before cold storage
50b9c5a  chore: add AgilePlus scaffolding
185c5ee  ci(legacy-enforcement): add legacy tooling anti-pattern gate (WARN mode)
8568b21  docs: add PLAN.md
18bc081  docs: add SPEC.md
```

**Observation**: Last real feature commit is ~6 months old. Recent commits are scaffolding/archival. Moved to `.archive/` branch—not actively developed.

---

## Architecture Assessment

The project is a **Canvas LMS → Google Calendar bridge** with AI-powered features:

1. **Canvas Integration** (canvas.py): Fetch courses, assignments, announcements
2. **Google Calendar Sync** (gcalendar.py): Create/update calendar events
3. **AI Summarization** (gpttools.py): OpenAI GPT for announcement summaries
4. **CLI** (main.py): Entry point, orchestrates the three modules

**Design Pattern**: Simple 3-tier, no database, no ORM, direct API calls. Lightweight and appropriate for scope.

**Risk**: Wildcard imports in main.py make it hard to audit what's actually used. No error handling framework.

---

## Cross-Repo Dependencies

- **Canvas API** (external): Requires authentication token (ASU Canvas instance)
- **Google Calendar API** (external): OAuth 2.0 setup required
- **OpenAI API** (external): API key required
- **No Phenotype ecosystem deps found**: Does not depend on phenotype-shared, phenotype-config-core, or other org libraries

**Opportunity**: Could benefit from `phenotype-config-core` for unified config management.

---

## Status Classification

**ARCHIVED / DORMANT SCAFFOLD**

- Archived to `.archive/` folder (intentional cold storage)
- Minimal governance; no active AgilePlus tracking
- Build is broken; not runnable without config.json + API keys
- No test coverage or CI/CD integration
- Suitable for **historical reference only** until resurrection

---

## Top 3 Next Actions

### 1. **Create Python Package Metadata** (5 min, one-line fix)

Add `pyproject.toml` and `requirements.txt`:

```toml
# pyproject.toml
[project]
name = "canvasapp"
version = "0.0.1"
description = "Canvas LMS → Google Calendar bridge"
dependencies = [
    "requests>=2.31.0",
    "google-auth-oauthlib>=1.0.0",
    "google-auth-httplib2>=0.1.1",
    "google-api-python-client>=2.100.0",
    "openai>=1.0.0",
    "ratelimit>=2.2.1",
]
```

**Impact**: Enables reproducible installs; unblocks dependency audits.

### 2. **Fix Lint Issues & Organize Imports** (10 min)

- Remove unused imports (datetime, requests from gpttools)
- Replace wildcard imports in main.py with explicit ones
- Add `noqa` comments if wildcard imports are intentional

**Impact**: Improves code clarity; enables safe refactoring.

### 3. **Add AgilePlus Spec & Functional Requirements** (20 min)

Create `FUNCTIONAL_REQUIREMENTS.md` mapping PLAN.md phases to verifiable FRs:

```markdown
# Functional Requirements

## FR-CANVAS-001: Course Retrieval
- Fetch enrolled courses via Canvas API
- Support pagination
- Trace to PLAN.md Phase 1

## FR-GCAL-001: Event Sync
- Create calendar events from assignments
- Support update/delete
- Trace to PLAN.md Phase 2
```

Then:
```bash
agileplus specify --title "canvasApp Revival" --description "Resurrect from archive with governance + test suite"
```

**Impact**: Re-activates project; unblocks governance tracking in AgilePlus.

---

## Conclusion

**canvasApp** is a well-designed but **underdeveloped** academic productivity tool, now archived. The codebase is lightweight (255 LOC), cleanly separated, and architecturally sound. However, it lacks Python packaging, test coverage, governance files, and runtime configuration.

**Resurrect**: Feasible in 2–3 hours (fix config, add tests, integrate AgilePlus). **Abandon**: Keep in `.archive/` for historical reference if out-of-scope for current roadmap.

