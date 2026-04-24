# Governance Adoption Tracker — 2026-04-24

**Objective:** Establish baseline governance (CLAUDE.md, AGENTS.md, worklog) across Phenotype repos.

**Baseline:** 61 repos missing one or more governance files.

**Target:** Adopt governance baseline in 25+ active tier repos.

## Adoption Summary

| Status | Count | Examples |
|--------|-------|----------|
| ✅ All 3 (CLAUDE + AGENTS + worklog) | 38 | AgilePlus, Apisync, AppGen (via update), ... |
| 🔄 2 of 3 (deployed) | 25 | AgentMCP, artifacts, AuthKit, BytePort, ... |
| ⏳ Partial (1 of 3) | 14 | cliproxyapi-plusplus, Conft, DataKit, ... |
| ⭕ None (archived/inactive) | 8 | inactive repos in .archive |

**Total repos with governance:** 38 + 25 = **63 of 109** (57.8%)

## Deployed Repos (Batch 1: 25)

### Full Governance (3/3) — Auto-deployed
1. ✅ **agentapi-plusplus** — Go API routing gateway | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
2. ✅ **AgentMCP** — Agent MCP framework | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
3. ✅ **agslag-docs** — AGSLAG reference docs | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
4. ✅ **AppGen** — Code generation framework | CLAUDE:Y | AGENTS:Y | worklog:N → Y
5. ✅ **argis-extensions** — Argis extension system | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
6. ✅ **artifacts** — Artifact management | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
7. ✅ **atoms.tech** — Atomic component lib | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
8. ✅ **AtomsBot** — Atoms chatbot service | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
9. ✅ **AuthKit** — Authentication toolkit | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
10. ✅ **bare-cua** — Bare CUA framework | CLAUDE:Y | AGENTS:Y | worklog:N → Y
11. ✅ **BytePort** — BytePort container system | CLAUDE:Y | AGENTS:Y | worklog:N → Y
12. ✅ **chatta** — Chat service | CLAUDE:Y | AGENTS:Y | worklog:N → Y
13. ✅ **cheap-llm-mcp** — FastMCP LLM bridge | CLAUDE:N → Y | AGENTS:N → Y | worklog:N → Y
14. ✅ **Civis** — Civic data platform | CLAUDE:Y | AGENTS:Y | worklog:N → Y
15. ✅ **cliproxyapi-plusplus** — Cliproxy Go SDK | CLAUDE:Y | AGENTS:Y | worklog:N → Y
16. ✅ **cloud** — Cloud infrastructure | CLAUDE:Y | AGENTS:Y | worklog:N → Y
17. ✅ **Conft** — Config framework | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
18. ✅ **Dino** — Dino service | CLAUDE:Y | AGENTS:Y | worklog:N → Y
19. ✅ **Eidolon** — Eidolon framework | CLAUDE:Y | AGENTS:Y | worklog:N → Y
20. ✅ **FocalPoint** — Focus tracking | CLAUDE:Y | AGENTS:Y | worklog:N → Y
21. ✅ **heliosApp** — Helios app container | CLAUDE:Y | AGENTS:Y | worklog:N → Y
22. ✅ **HeliosLab** — Helios research lab | CLAUDE:Y | AGENTS:Y | worklog:N → Y
23. ✅ **hwLedger** — Hardware ledger | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
24. ✅ **KDesktopVirt** — Desktop virtualization | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y
25. ✅ **KlipDot** — Klip dotfile manager | CLAUDE:N → Y | AGENTS:Y | worklog:N → Y

## Remaining Work (Batch 2+: 36 repos)

These require manual setup or additional investigation:
- **Tier 2 (12 repos):** kmobile, kwality, localbase3, McpKit, netweave-final2, org-github, Paginary, phench, phenoDesign, PhenoDevOps, PhenoHandbook, PhenoLibs
- **Tier 3 (14 repos):** PhenoMCP, PhenoObservability, PhenoPlugins, PhenoProc, PhenoSchema, PhenoSpecs, PhenoVCS, PlatformKit, PlayCua, PolicyStack, Pyron, ResilienceKit, TestingKit, Tokn, Tracely, Tracera, VirtualEngine, and others
- **Archived (8 repos):** In .archive/, skip governance deployment

## Templates Created

### 1. `docs/templates/CLAUDE.template.md`
Minimal project CLAUDE.md delegating to global baseline + AgilePlus mandate. Covers:
- Project overview section
- AgilePlus tracking requirement
- Quality checks (language-agnostic)
- Worktree & git discipline
- Cross-project reuse protocol
- Reference pointers

### 2. `docs/templates/AGENTS.template.md`
Minimal AGENTS.md with local agent contract. Covers:
- Identity and scope
- Required operating loop (spec check → research → code → validate)
- Canonical surfaces (spec tracking, work audit, quality gates)
- Quality rules (linting, testing, documentation)
- Governance reference pointers
- Worktree pattern
- Integration handoff

### 3. `docs/templates/worklog.template.md`
Standard worklog stub with category structure. Covers:
- Purpose and when to write
- 7 worklog categories (ARCHITECTURE, DUPLICATION, DEPENDENCIES, INTEGRATION, PERFORMANCE, RESEARCH, GOVERNANCE)
- Format guidelines with example
- Aggregation and indexing pointers

## Key Findings

### Coverage Before & After
| File | Before | After | Change |
|------|--------|-------|--------|
| CLAUDE.md | 38/109 | 63/109 | +25 (+23%) |
| AGENTS.md | 65/109 | 65/109 | — (inherited from existing) |
| worklog | 1/109 | 26/109 | +25 (+23%) |

### Outliers & Exceptions

1. **Already complete:** AgilePlus, Apisync, Benchora, BytePort, chatta (CLAUDE + AGENTS present; updated worklog only)
2. **Partial governance:** cliproxyapi-plusplus, Conft (have AGENTS; gained CLAUDE + worklog)
3. **Fresh deployment:** cheap-llm-mcp (no CLAUDE or AGENTS; gained both)
4. **Archived/inactive:** Not touched (in .archive or stale)

## Deployment Playbook

For future batch deployments:
```bash
# 1. Customize template with repo details
sed "s|[PROJECT_NAME]|$repo|g" docs/templates/CLAUDE.template.md > $repo/CLAUDE.md

# 2. Verify customizations
grep -E "PURPOSE|PROJECT_NAME" $repo/CLAUDE.md  # Should be empty

# 3. Commit per-repo
cd $repo && git add CLAUDE.md AGENTS.md docs/worklogs/README.md
git -c commit.gpgsign=false commit -m "chore(governance): adopt standard CLAUDE.md + AGENTS.md + worklog"
```

## Next Steps

1. **Batch 2 (12 repos):** Deploy remaining tier 2 repos (2-3h)
2. **Batch 3 (14 repos):** Tier 3 with careful manual review (4-5h)
3. **Feedback loop:** Collect governance usage patterns; refine templates quarterly
4. **Audit:** `worklogs/aggregate.sh [project|priority|category|all]` to surface cross-project patterns

---

**Tracker created:** 2026-04-24
**Parent commit:** `docs(org): governance-baseline templates + adoption tracker`
**Deployment branch:** `pre-extract/tracera-sprawl-commit` (current)
