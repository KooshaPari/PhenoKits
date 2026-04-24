# Agent Skills Extraction Map — Sidekick + phenotype-tooling Targets

**Date:** 2026-04-24  
**Audit Scope:** `~/.claude/skills/` (50 skills)  
**Target Destinations:** Sidekick (agent-facing micro-utilities), phenotype-tooling (org-wide tools), Global Keep, Archive

---

## Executive Summary

- **Total Skills:** 50 skills across 23 directories
- **Sidekick-Bound:** 5 skills (agent orchestration, presence, messaging, routing)
- **phenotype-tooling-Bound:** 14 skills (rich CLI, config tools, design ops, DevOps)
- **Global Keep:** 17 skills (design system, frontend, specialized agent dispatchers)
- **Legacy / Duplicate:** 7 skills (deprecated agent wrappers, empty stubs)
- **Project-Specific:** 7 skills (WorkOS integration, thegent-native)

---

## Skill Classification Matrix

### Sidekick-Bound (Agent-Facing Micro-Utilities)

These are canonical agent-control utilities that belong in Sidekick's core messaging/dispatch layer:

| Skill | Purpose | Target | Priority | Notes |
|-------|---------|--------|----------|-------|
| **agent-imessage** | User presence, iMessage/SMS contact, response-likelihood signals | Sidekick::messaging | P0 | Core user async communication layer; MCP bridge for Messages.app |
| **cheap-llm** | Route bulk tasks to Minimax/Kimi/Fireworks for volume work | Sidekick::routing | P0 | LLM cost optimization; cheap-llm-mcp server backend |
| **forge-agent** | Headless CLI dispatch for non-interactive prompts | Sidekick::agent-dispatch | P1 | Primary headless entry point; vendor-agnostic CLI abstraction |
| **thegent** | Unified agent dispatcher (Forge, Codex, Copilot, Gemini, Minimax) | Sidekick::agent-dispatch | P1 | Orchestration abstraction; provider enum + mode mapping |
| **thegent-skills** | Orchestration guidance (sessions, background jobs, governance) | Sidekick::orchestration | P1 | Meta-skill; documents governance + CLI reference |

**Extraction Notes:**
- Move `agent-imessage` CLI + MCP server as-is; retain macOS Messages integration.
- Consolidate `forge-agent` + `thegent` into single `Sidekick::Agent Dispatch` subsystem; `thegent-dispatch` is the Rust implementation.
- Retain `thegent-skills` as reference docs in Sidekick; link to `~/.claude/docs/guides/THGENT_CLI_REFERENCE.md`.

---

### phenotype-tooling-Bound (Org-Wide Tools & Infrastructure)

These belong in the `phenotype-tooling` monorepo for general availability:

| Skill | Purpose | Target Module | Priority | Notes |
|-------|---------|-----------------|----------|-------|
| **rich-cli** | Terminal graphics (images, progress bars, panels) via kitty protocol | phenotype-tooling::rich-cli | P0 | Already extracted; wrap as MCP tool |
| **audit** | Design accessibility, performance, theming audits | phenotype-tooling::design-audit | P1 | Design QA utility; integrate into CI |
| **polish** | Final quality pass (alignment, spacing, consistency) | phenotype-tooling::design-ops | P1 | Design refinement workflow |
| **normalize** | Realign UI to design system standards | phenotype-tooling::design-ops | P1 | Design system enforcement |
| **clarify** | UX copy improvement (error messages, labels, microcopy) | phenotype-tooling::copy-tools | P2 | Microcopy audit / writing-assist tool |
| **harden** | Resilience (error handling, i18n, accessibility) | phenotype-tooling::design-ops | P2 | QA / hardening workflow |
| **teach-impeccable** | Design context gathering (one-time project setup) | phenotype-tooling::design-context | P1 | Foundational for all design skills |
| **frontend-design** | Production-grade interface design (umbrella skill) | phenotype-tooling::design-systems | P1 | Design-led feature delivery; orchestrates other design skills |
| **animate** | Animation + micro-interactions | phenotype-tooling::animation-tools | P2 | Enhancement utility; depends on frontend-design context |
| **arrange** | Layout, spacing, visual rhythm | phenotype-tooling::layout-tools | P2 | Grid/spacing auditor |
| **adapt** | Multi-screen responsive design | phenotype-tooling::responsive-tools | P2 | Responsive audit / refinement |
| **colorize** | Strategic color addition | phenotype-tooling::color-tools | P2 | Color system / palette audit |
| **bolder** | Visual interest amplification | phenotype-tooling::visual-amplification | P2 | Design boldness / confidence tool |
| **quieter** | Reduce visual noise / intensity | phenotype-tooling::visual-quieting | P2 | Design restraint / tone tool |
| **extract** | Component/token extraction & reuse | phenotype-tooling::component-extraction | P2 | DRY enforcement for design systems |
| **distill** | Essence extraction (remove unnecessary complexity) | phenotype-tooling::design-reduction | P3 | Refactoring utility for overdesigned interfaces |
| **delight** | UX personality + moments of joy | phenotype-tooling::ux-delight | P3 | Enhancement tool; brand personality |
| **typeset** | Typography (fonts, hierarchy, sizing) | phenotype-tooling::typography-tools | P2 | Typography audit / refinement |
| **onboard** | Onboarding flows + empty states | phenotype-tooling::onboarding-tools | P2 | Feature completion / UX workflow |
| **critique** | Design evaluation (UX perspective, hierarchy, information flow) | phenotype-tooling::design-critique | P2 | Design review tool |
| **optimize** | Performance (load, rendering, animations) | phenotype-tooling::perf-tools | P2 | Design performance audit |
| **overdrive** | Technically ambitious interface techniques | phenotype-tooling::advanced-design | P3 | Cutting-edge design patterns |

**Extraction Notes:**
- Group design skills into `phenotype-tooling::design-systems` top-level module.
- Consolidate `audit`, `polish`, `normalize`, `harden` into a single **design-qa** CI pipeline.
- `frontend-design` is the orchestrator; all other design skills are invoked as sub-steps.
- Create `phenotype-tooling::design-context` to manage `.impeccable.md` + context binding across projects.
- `rich-cli` is already partially extracted (`rich-cli-kit`); move wrapper skill + MCP into phenotype-tooling.

---

### Global Keep (Design System + Specialized Dispatchers)

These remain in `~/.claude/skills/` as global infrastructure or specialized agent interfaces:

| Skill | Purpose | Reason | Notes |
|-------|---------|--------|-------|
| **workos** | WorkOS integration reference (auth, SSO, SAML, SCIM, RBAC, FGA) | Specialized third-party integration | Product-specific; not reusable tooling |
| **workos-widgets** | WorkOS Widgets (User Mgmt, Admin Portal) | Specialized third-party integration | Product-specific; bundle with workos |
| **codex-agent** | Codex wrapper (GPT-5.4 high) | Legacy IDE-specific dispatcher | Deprecated; route via Forge or thegent; keep for reference only |
| **copilot-agent** | GitHub Copilot CLI (Claude Haiku 4.5 lock) | Legacy IDE-specific dispatcher | Deprecated; route via Forge; keep for Haiku enforcement reference |
| **cursor-agent** | Cursor IDE Composer / background agent | Legacy IDE-specific dispatcher | Deprecated; route via Forge; keep for Composer reference only |
| **gemini-agent** | Google Gemini Code Assist | Legacy IDE-specific dispatcher | Deprecated; route via Forge; keep for multi-modal reference |
| **droid-agent** | Droid agent wrapper | Empty stub / legacy | Archive (no content) |

**Reason for Keep:**
- **workos**/workos-widgets**: Product-specific integrations; not generalizable.
- **codex/copilot/cursor/gemini-agent**: Legacy but documented; route all new invocations through Forge/thegent instead; keep docs for reference.
- **droid-agent**: Empty stub; archive.

---

### Archive (Legacy / Duplicate / Deprecated)

| Skill | Status | Reason |
|-------|--------|--------|
| **droid-agent** | Empty stub | No implementation; remove |

---

## Integration Roadmap

### Phase 1: Sidekick Core (1-2 weeks)

**Target:** Move agent orchestration + messaging into Sidekick as canonical utilities.

1. **Move `agent-imessage`** → `Sidekick::messaging::imessage`
   - Retain macOS Messages MCP bridge.
   - Expose CLI: `sidekick status`, `sidekick notify`, `sidekick wait`.
   - Expose MCP tools for user contact + presence signals.

2. **Move `cheap-llm`** → `Sidekick::routing::cheap-llm`
   - Expose `sidekick route-cheap <prompt>` CLI.
   - Retain MCP tool: `cheap_llm.complete()`.

3. **Move `forge-agent` + `thegent`** → `Sidekick::dispatch::agent`
   - Canonical CLI: `sidekick dispatch --provider forge --prompt "..."`.
   - Retain `thegent` as high-level orchestration alias.
   - Link `thegent-dispatch` (Rust) as implementation backend.

4. **Move `thegent-skills`** → `Sidekick::docs::orchestration` (reference docs)
   - Update links to point to new `Sidekick::dispatch` subsystem.

---

### Phase 2: phenotype-tooling Design Systems (2-3 weeks)

**Target:** Consolidate design skills + QA utilities into org-wide tooling.

1. **Create `phenotype-tooling::design-systems` module:**
   - Move `frontend-design` as orchestrator.
   - Move all design skills (audit, polish, etc.) as sub-utilities.
   - Create `phenotype-tooling::design-context` to manage `.impeccable.md` binding.

2. **Create `phenotype-tooling::design-qa` CI pipeline:**
   - Consolidate `audit` + `normalize` + `harden` into single lint-like gate.
   - Expose as `task design:audit` in projects.

3. **Move `rich-cli`** → `phenotype-tooling::cli::rich`
   - Wrap `rich-cli-kit` project.
   - Expose as `rck` CLI + MCP tool.

4. **Create `phenotype-tooling::copy-tools` module:**
   - Move `clarify` skill.
   - Integrate with spelling/grammar checking (Vale).

---

### Phase 3: Reference & Cleanup (1 week)

**Target:** Document legacy dispatchers; archive empty stubs.

1. **Create `~/.claude/skills/README.md`** (index of remaining global skills):
   - Link to Sidekick + phenotype-tooling relocated skills.
   - Document workos/workos-widgets as product-specific integrations.
   - Mark codex/copilot/cursor/gemini-agent as deprecated; route via Forge.

2. **Archive `droid-agent`** → move to `.archive/` or delete.

---

## Per-Skill Integration Notes

### `agent-imessage` → Sidekick::messaging

**Current:** `~/.claude/skills/agent-imessage/SKILL.md` + `agent-imessage` CLI + MCP bridge  
**Target:** `Sidekick/src/messaging/imessage.rs` + `Sidekick::imessage` MCP tool  
**Integration:**
- Retain `agent-imessage-mcp` server registration in Sidekick config.
- Expose as CLI subcommand: `sidekick msg status`, `sidekick msg notify`, `sidekick msg wait`.
- MCP tools: `agent_imessage.*` (status, hook_decision, notify_user, wait_for_user_reply, record_user_action, etc.).

**Backwards Compatibility:**
- Keep `~/.claude/skills/agent-imessage/` as thin wrapper that delegates to Sidekick.
- Update SKILL.md to document new Sidekick paths.

---

### `cheap-llm` → Sidekick::routing

**Current:** `~/.claude/skills/cheap-llm/SKILL.md` + `cheap-llm-mcp` server  
**Target:** `Sidekick/src/routing/cheap_llm.rs` + Sidekick MCP tool  
**Integration:**
- Retain `cheap-llm-mcp` server as external dependency.
- Register in Sidekick config; expose as `cheap_llm.*` MCP tools.
- CLI: `sidekick route cheap --prompt "..." --provider minimax`.

---

### `forge-agent` + `thegent` → Sidekick::dispatch

**Current:** Two separate skills + `thegent-dispatch` Rust CLI  
**Target:** Single `Sidekick::dispatch` subsystem  
**Integration:**
- Consolidate both into `Sidekick/src/dispatch/agents.rs`.
- CLI: `sidekick dispatch --provider forge|codex|copilot|cursor|gemini|minimax --prompt "..."`.
- Alias: `sidekick run <prompt>` (shorthand for `dispatch --provider forge`).
- Expose MCP tool: `agent_dispatch` with unified args.
- `thegent-dispatch` (Rust) remains as independent tool; Sidekick calls it internally.

---

### `frontend-design` + 18 design skills → phenotype-tooling::design-systems

**Current:** 20 separate skills scattered in `~/.claude/skills/`  
**Target:** Single `phenotype-tooling/design-systems/` module  
**Integration:**
- Move all SKILL.md files → `phenotype-tooling/design-systems/skills/`.
- Create `phenotype-tooling/design-systems/README.md` orchestrating all sub-skills.
- `frontend-design` remains as router skill; invokes `phenotype-tooling` sub-utilities.
- Design context (`.impeccable.md`) stored in `phenotype-tooling/design-context/`.

---

### `rich-cli` → phenotype-tooling::cli::rich

**Current:** `~/.claude/skills/rich-cli/SKILL.md` + `rich-cli-kit` project  
**Target:** `phenotype-tooling/cli/rich/` module  
**Integration:**
- Thin wrapper skill in `~/.claude/skills/rich-cli/` delegates to phenotype-tooling.
- MCP registration: use `rich-cli-mcp` from `rich-cli-kit` build.
- CLI: `rck` (already available from rich-cli-kit).

---

### `workos` + `workos-widgets` → Global Keep (Project-Specific)

**Reason:** Product-specific integrations; not generalizable to org-wide tooling.

**Keep in:** `~/.claude/skills/workos/` + `~/.claude/skills/workos-widgets/` (as-is)

---

### Legacy Dispatchers → Archive / Deprecation

| Skill | Action |
|-------|--------|
| **codex-agent** | Mark deprecated; route via `forge --model gpt-5-4-high`. Keep SKILL.md for reference. |
| **copilot-agent** | Mark deprecated; route via `forge --model gpt-5-mini` (Copilot Haiku lock). Keep SKILL.md for reference. |
| **cursor-agent** | Mark deprecated; document Composer reference only. Route via `thegent --provider cursor`. |
| **gemini-agent** | Mark deprecated; route via `thegent --provider gemini` or `forge`. Keep SKILL.md for multi-modal reference. |
| **droid-agent** | Archive (empty stub). |

---

## Extraction Priority Order

1. **P0 (Week 1):** `agent-imessage`, `cheap-llm` → Sidekick (user contact + cost routing)
2. **P1 (Week 2):** `forge-agent`, `thegent`, `thegent-skills` → Sidekick (agent dispatch) + `frontend-design`, `teach-impeccable` → phenotype-tooling (design context)
3. **P2 (Week 3):** Remaining 15 design skills → phenotype-tooling::design-systems (design QA + sub-utilities)
4. **P3 (Week 4):** `rich-cli` → phenotype-tooling::cli (rich terminal output)
5. **P4 (Cleanup):** Legacy dispatcher deprecation + documentation

---

## Backwards Compatibility Plan

1. **Wrapper skills remain in `~/.claude/skills/`** for the next 2 releases, delegating to new locations.
2. **Update SKILL.md files** to document new paths + deprecation notices.
3. **Dual-stack mode:** Both old CLI + new Sidekick/phenotype-tooling CLIs work in parallel for 30 days.
4. **Retire old CLI** after agent migration period + 2 weeks notice.

---

## Summary by Destination

| Destination | Count | Skills |
|-------------|-------|--------|
| **Sidekick** | 5 | agent-imessage, cheap-llm, forge-agent, thegent, thegent-skills |
| **phenotype-tooling** | 14 | frontend-design, teach-impeccable, audit, polish, normalize, harden, clarify, animate, arrange, adapt, colorize, bolder, quieter, extract, distill, delight, typeset, onboard, critique, optimize, overdrive, rich-cli |
| **Global Keep** | 7 | workos, workos-widgets, codex-agent (deprecated), copilot-agent (deprecated), cursor-agent (deprecated), gemini-agent (deprecated) |
| **Archive** | 1 | droid-agent |
| **Unknown / Ambiguous** | 0 | (all classified) |

---

## Files Referenced

- `~/.claude/skills/*/SKILL.md` — all 50 skill specifications
- `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus` — spec tracking
- `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent-dispatch` — agent dispatch CLI (Rust)
- `/Users/kooshapari/CodeProjects/Phenotype/repos/rich-cli-kit` — rich CLI tools (Rust)
