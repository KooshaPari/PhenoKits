# Cross-Parent Scan: Non-repos/ Phenotype + Agent Skills Inventory
**Date:** 2026-04-24  
**Audit Scope:** Non-repos/ Phenotype directories, CodeProjects siblings, agent skills

---

## Non-/repos/ Phenotype Directories

### Active Phenotype Git Repos (Outside /repos)
- **phenotype-shared-temp** (`/Phenotype/phenotype-shared-temp/.git`)
  - Remote: `https://github.com/KooshaPari/phenotype-shared.git`
  - Current branch: `chore/integrate-phenotype-docs`
  - Status: **Duplicate/Staging worktree** — appears to be a local sync/temp of the canonical `repos/phenotype-shared`
  - Action: Consolidate into `/repos/phenotype-shared-wtrees/` or document as intentional staging directory

### Non-Git Phenotype Directories
- **docs/** — Organizational docs (research/, handbook/, sessions/, templates/, reports/)
- **projects/** — Project metadata (modules/, profiles/, smoke tests)
- **bak/** — Backup archive (85 dirs)
- **scripts/** — Shell/utility scripts
- **contracts/** — Contract definitions
- **.archive/** — Archived projects
- **.agileplus/** — AgilePlus config (3 subdirs)
- **.claude/** — Agent config/context

---

## CodeProjects Sibling Directories

| Directory | Type | Phenotype-Relevant | Notes |
|-----------|------|-------------------|-------|
| `Phenotype/` | namespace | YES | Main Phenotype workspace |
| `courses/` | academic | NO | Coursework (learning/ subdirs) |
| `research/` | research | PARTIAL | Agent memory architectures, agentic patterns (cross-org applicable) |
| `learning/` | academic | NO | Courses subdirs |
| `orphans/` | experimental | PARTIAL | agslag-new, heliosHarness, canvasApp (inactive Phenotype satellites) |
| `Dev/` | personal | NO | job-hunter (personal dev) |
| `cliproxyapi-plusplus/` | archived | YES | Go API gateway (referenced in memory as ACTIVE) |
| `archive/`, `KooshaPari/`, `temp/` | junk | NO | Dumps and staging |

---

## Agent Skills Inventory

### Skills by Category

**Dispatcher Agents (CLI Routing)**
- `forge-agent` — Primary headless agent (Rust focus)
- `gemini-agent` — Google Gemini routing
- `copilot-agent` — GitHub Copilot CLI (Haiku 4.5 locked)
- `cursor-agent` — Cursor IDE integration
- `codex-agent` — Codex subagent
- `thegent` — External CLI dispatch (any Forge/Codex/Gemini agent)
- `droid-agent` — Mobile automation (Android/iOS)

**LLM Routing & Reasoning**
- `cheap-llm` — Route bulk reasoning to Minimax/Kimi/Fireworks
- `agent-imessage` — Contact Koosha via iMessage/SMS

**Design System & UI**
- `thegent-skills` — Thegent-specific design skills
- `workos-widgets` — WorkOS widget integration
- `workos` — WorkOS platform support
- `rich-cli` — Terminal UI (progress bars, images, status)

**Design Quality & Polish Skills** (14 skills)
- `frontend-design` — Full-stack UI design
- `audit`, `critique` — Quality assessment
- `polish`, `normalize`, `distill` — Design refinement
- `arrange`, `clarify`, `typeset`, `colorize` — Layout/typography/color
- `bolder`, `quieter`, `harden` — Visual intensity & resilience
- `optimize`, `adapt`, `delight`, `onboard`, `animate` — Advanced UX
- `extract` — Component extraction & consolidation
- `overdrive` — Technically ambitious UI

**Spec Management & Workflow** (13 spec-kitty.* skills)
- `spec-kitty.specify`, `.plan`, `.implement`, `.tasks`
- `.status`, `.review`, `.accept`, `.checklist`, `.clarify`
- `.merge`, `.constitution`, `.analyze`, `.research`, `.dashboard`

**Utilities**
- `update-config` — Settings.json management
- `keybindings-help` — Keyboard customization
- `simplify` — Code quality review
- `fewer-permission-prompts` — Reduce permission spam
- `loop` — Recurring task scheduling
- `schedule` — Cron-based routine scheduling
- `claude-api` — Anthropic SDK optimization
- `init` — CLAUDE.md initialization
- `review` — PR review workflow
- `security-review` — Security audit

**Total:** 51 registered skills (7 dispatcher agents, 2 LLM routing, 3 WorkOS, 14 design polish, 13 spec-kitty, 12 utilities)

---

## Consolidation Opportunities

### 1. Duplicate Phenotype-Shared Repo
- **Current:** `phenotype-shared-temp` (standalone) + `repos/phenotype-shared` (canonical)
- **Action:** Audit why temp dir exists; merge branches into canonical or document as intentional staging
- **Impact:** Eliminate 35K LOC duplication source

### 2. Orphaned Repos Triage
- **agslag-new**, **heliosHarness**, **canvasApp** in `/orphans/`
- **Action:** Decide: archive to /repos/.archive/, delete, or revive; document rationale
- **Impact:** Clean up sibling namespace

### 3. Design System Skills Consolidation
- 14 design polish skills could become a **Sidekick design-polish sub-crate** (thegent-skills wrapper)
- **Action:** Extract into `thegent-skills/design-polish.rs` shared library
- **Impact:** Reduce Impeccable skill dependency chains

### 4. Dispatcher Agent Registry
- 7 dispatcher agents (forge, gemini, copilot, cursor, codex, thegent, droid) could be **unified routing registry**
- **Action:** Create `agileplus-agent-dispatch` crate tracking agent runtimes and routing rules
- **Impact:** Single source of truth for multi-model routing, reduces config duplication

### 5. Cross-Org Research Docs
- `/research/` contains agent memory architectures + agentic patterns applicable across repos
- **Action:** Index and link from `repos/docs/research/` (keep canonical there)
- **Impact:** Prevent duplication; centralize org-wide research

---

## Summary

**Non-repos/ Phenotype:** 1 duplicate repo (phenotype-shared-temp), 7 supporting directories  
**CodeProjects siblings:** 1 active Phenotype satellite (cliproxyapi-plusplus), 2 orphaned/experimental  
**Agent skills:** 51 registered (7 dispatchers, 14 design polish, 13 spec-kitty, 12 utilities)  
**High-priority consolidations:** Duplicate repo cleanup, orphans triage, design system unification, dispatcher registry
