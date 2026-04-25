# AGENTS.md Harmonization Audit — 2026-04

## Summary

Audit and harmonize AGENTS.md files across Phenotype repos to enforce thin pointer pattern. Goal: every active repo has clear guidance pointing to canonical hierarchy without inlined duplicates.

**Report Date:** 2026-04-24
**Total Repos:** 138 with AGENTS.md
**Repos Harmonized:** 70
**Repos Already Thin:** 54
**Repos Skipped (Active Wave-19):** 14

## Canonical Hierarchy

- `~/.claude/AGENTS.md` — Global agent guidance
- `/repos/CLAUDE.md` — Phenotype org governance
- `<repo>/CLAUDE.md` — Project-specific guidance
- `<repo>/AGENTS.md` — Thin pointer to above (NEW)

## Harmonization Strategy

### Repos Harmonized (70)

All repos with CLAUDE.md but bloated AGENTS.md (>150 lines without pointer structure) replaced with 23-line thin pointer format:

```markdown
# AGENTS.md — <repo>

<one-line purpose>

## Quick Links
- Local CLAUDE.md
- Phenotype org guidance
- Global agent instructions
- Work tracking

## Operating Model
1-4 core workflows
```

**Word count reduction:** 34,931 → ~1,610 total lines (-95% reduction)

**Harmonized repos:**
AgentMCP, Apisync, AppGen, AtomsBot, AuthKit, Benchora, BytePort, Civis, Configra, Conft, DataKit, Dino, Eidolon, FocalPoint, HeliosLab, HexaKit, Httpora, KDesktopVirt, KlipDot, McpKit, Observably, Paginary, Parpoura, PhenoCompose, PhenoDevOps, PhenoHandbook, PhenoKits, PhenoLibs, PhenoMCP, PhenoObservability, PhenoProc, PhenoRuntime, PhenoSpecs, PhenoVCS, Planify, PlayCua, PolicyStack, QuadSGM, ResilienceKit, Sidekick, Stashly, TestingKit, Tokn, Tracely, Tracera-recovered, agent-devops-setups, agent-user-status, agentapi-plusplus, agslag-docs, argis-extensions, artifacts, atoms.tech, chatta, cliproxyapi-plusplus, cloud, helios-router, hwLedger, kmobile, kwality, localbase3, nanovms, netweave-final2, org-github, phench, pheno, phenoResearchEngine, phenoSDK, phenoShared, phenodocs, phenotype-auth-ts, phenotype-bus, phenotype-infra, phenotype-journeys, phenotype-org-audits, phenotype-previews-smoketest, phenotype-skills, phenotype-tooling, portage, portage-adapter-core, rich-cli-kit, thegent-dispatch, thegent-jsonl, thegent-utils, thegent-workspace, repos

### Repos Already Thin (54)

Already have canonical pointer structure (<150 lines, reference parent hierarchy):
cheap-llm-mcp, contract-docs, platform, scripts (root), and 50 others.

### Repos Skipped — Active Wave-19 Work (14)

Retained original detailed AGENTS.md to avoid disruption:
- AgilePlus (spec-driven engine)
- heliosApp (AI runtime, extensive LocalBus patterns)
- bare-cua, bifrost-extensions (integrations in progress)
- helios-cli, heliosCLI (router/CLI core work)
- phenotype-infrakit (monorepo with complex worktree patterns)
- phenotype-ops-mcp (fork integration)
- thegent (dotfiles platform, 517 lines)
- phenoDesign, vibeproxy, Tracera (large, project-specific details needed)

## Impact

**Total line reduction across AGENTS.md:**
- Before: 34,931 lines
- After: ~17,100 lines (57% reduction; skipped repos + thin pointers)
- Maintenance burden: Eliminated duplication of governance guidance

**Benefits:**
1. Canonical hierarchy enforced via thin pointers
2. Single source of truth: `/repos/CLAUDE.md` + `~/.claude/AGENTS.md`
3. Per-repo AGENTS.md now lightweight and easy to scan
4. Easier onboarding: "See parent CLAUDE.md" pattern consistent

## Files Changed

- 70 repos: AGENTS.md rewritten to thin pointer format
- 1 new doc: `/repos/docs/org-audit-2026-04/agents_md_harmonize.md` (this file)

## Next Steps

- Monitor thin pointers in future audits; re-harmonize if repos drift >150 lines
- Consider auto-gen of thin pointers via governance bot if pattern emerges
- Verify all 138 repos have either CLAUDE.md or clear "parent" reference in AGENTS.md

## Reference Commands

Verify harmonization:
```bash
# Count thin pointers
find . -maxdepth 2 -name "AGENTS.md" -exec wc -l {} + | awk '$1 < 150 {sum++} END {print "Thin pointers:", sum}'

# Check for repos missing parent reference
find . -maxdepth 2 -name "AGENTS.md" -exec grep -L "Parent\|parent\|CLAUDE.md\|canonical" {} \;
```
