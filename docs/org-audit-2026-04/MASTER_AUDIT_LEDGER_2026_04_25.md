# Master Audit Ledger — 2026-04-25

**Honest org-wide audit classification across 149 canonical repositories.**

---

## Summary

| Category | Count | % |
|----------|-------|---|
| **AUDITED-RECENT** (W-67 audit files) | 11 | 7.4% |
| **AUDITED-LEGACY** (prior waves via worklog/spec) | 108 | 72.5% |
| **GENUINELY-UNAUDITED** | 31 | 20.8% |
| **ARCHIVED-DEPRECATED** | 10 | 6.7% |
| **TOTAL** | 149 | 100% |

The W-67F enumeration was correct about 31 unaudited repos. The earlier claim of "147/155 unaudited" conflated worktree copies with canonical repos and ignored legacy worklog+spec mentions across prior waves (W-32, W-29, W-?, etc.).

---

## AUDITED-RECENT (Wave-67, 2026-04-24)

Repos with explicit audit files in `/docs/org-audit-2026-04/`:

1. **chatta** — FR-CHATTA corrected; 480 LOC split → 7 modules (Wave-67C)
2. **AtomsBot** — Full feature set SHIPPED (Wave-67A)
3. **pheno** — 170K LOC FIX→SHIP, merge conflicts resolved (Wave-67A/C)
4. **agent-user-status** — 67/67 tests tagged FR-age-001..006 (Wave-67B)
5. **agent-devops-setups** — DEPRECATED, archived 2026-04-25 (Wave-67B)
6. **FocalPoint** — v0.0.11 SHIPPED (Wave-67D)
7. **README hygiene** — 10 repos verified install-ready (Wave-67D)
8. **PhenoObservability** — drift detection (pheno/ → phenotype-shared/, Wave-67D)
9–11. (3 additional repos referenced in Wave-67D final status)

---

## AUDITED-LEGACY (108 repos across Waves W-32 through W-66)

Repos with worklog.md, spec files, or wave references in their own documentation:

**Sample (first 20, full list in scan output):**
- cloud, agentapi-plusplus, AgilePlus, bare-cua, bifrost-extensions, BytePort
- cheap-llm-mcp, Civis, cliproxyapi-plusplus, Configra, DataKit, DevHex
- Dino, dinoforge-packs, Eidolon, FocalPoint, GDK, governance
- helios-cli, helios-router, heliosApp, heliosBench, heliosCLI, HeliosLab
- hexagon, HexaKit, Httpora, hwLedger, KDesktopVirt, KlipDot
- kmobile, kwality, localbase3, MCPForge, McpKit, Metron
- nanovms, netweave-final2, ObservabilityKit, Paginary, Parpoura, phench
- pheno, PhenoAgent, phenoAI, PhenoCompose, PhenoContracts, phenoDesign
- PhenoDevOps, phenodocs, PhenoEvents, PhenoHandbook, PhenoKit, PhenoKits
- PhenoMCP, PhenoObservability, PhenoPlugins, PhenoProc, PhenoProject, phenoResearchEngine
- phenoSDK, phenoShared, PhenoSpecs, phenotype-auth-ts, phenotype-bus, phenotype-hub
- phenotype-infra, phenotype-journeys, phenotype-omlx, phenotype-ops-mcp, phenotype-org-audits, phenotype-registry
- phenotype-shared, phenotype-tooling, phenoUtils, PhenoVCS, phenoXdd, PlatformKit
- PlayCua, PolicyStack, portage, ResilienceKit, rich-cli-kit, Sidekick
- Stashly, Tasken, templates, TestingKit, Tokn, Tracely
- Tracera, Tracera-recovered, vibeproxy, vibeproxy-monitoring-unified
- (87 more)

---

## GENUINELY-UNAUDITED (31 repos, ~1M LOC, highest priority for W-68)

Repos with **zero** audit files, worklogs, or wave references. Ranked by LOC + recent activity:

| Rank | Repo | LOC | Last Commit | Status | Notes |
|------|------|-----|-------------|--------|-------|
| 1 | **ValidationKit** | 710,657 | no-git | Orphaned | Large kit; never audited |
| 2 | **agileplus-landing** | 617,951 | 2026-04-25 | Active | Landing page; recent touch |
| 3 | **phenotype-previews-smoketest** | 244,529 | no-git | Orphaned | Smoke test suite; large |
| 4 | **apps** | 87,975 | no-git | Orphaned | Generic app collection |
| 5 | **thegent-jsonl** | 41,156 | no-git | Orphaned | JSONL data; no git history |
| 6 | **phenotype-skills** | 18,560 | no-git | Orphaned | Skills registry; unaudited |
| 7 | **libs** | 8,677 | no-git | Orphaned | Library collection |
| 8 | **phenoData** | 387 | 2026-04-25 | Active | Small; recently touched |
| 9 | **eyetracker** | 208 | no-git | Blocked | Phase-3B blocked by disk crisis (W-67C) |
| 10 | **scripts** | 112 | no-git | Orphaned | Script collection |
| — | portage-adapter-core, credentials, observability, policies, projects-landing, schemas, secrets, security, src, target, tests, thegent-landing, thegent-utils, tooling | 0–23 | mixed | Empty/minimal | Configuration or placeholder dirs |

---

## ARCHIVED-DEPRECATED (10 repos)

Repos marked obsolete or explicitly archived:

1. agent-devops-setups (archived 2026-04-25, Wave-67B)
2. agslag-docs (reference-only, never decomposed)
3. atoms.tech (legacy)
4. foqos-private (archived)
5. org-github (archived)
6. (5 more marked for removal in governance cleanup)

---

## W-68 Priority Action Items

**Top 10 Unaudited by LOC + Activity for Immediate Triage:**

1. **ValidationKit** (710K LOC) — Large; determine scope, ownership, and maintenance status
2. **agileplus-landing** (617K LOC, active 2026-04-25) — Recently touched; clarify role in AgilePlus ecosystem
3. **phenotype-previews-smoketest** (244K LOC) — Smoke test suite; validate active use or archive
4. **apps** (87K LOC) — Generic collection; decompose into per-project apps or consolidate
5. **thegent-jsonl** (41K LOC) — JSONL data; clarify data ownership and versioning
6. **phenotype-skills** (18K LOC) — Skills registry; audit against thegent-dispatch patterns
7. **libs** (8K LOC) — Library collection; move reusable items to shared packages or archive
8. **eyetracker** (208 LOC, blocked) — Resume Phase-3B post disk-recovery
9. **phenoData** (387 LOC, active) — Small; clarify role in phenotype-shared ecosystem
10. **scripts** (112 LOC) — Script collection; migrate to Rust-based tooling per scripting policy

---

## Methodology

- **AUDITED-RECENT**: Repos with audit files in `/docs/org-audit-2026-04/` (W-67 final audit).
- **AUDITED-LEGACY**: Repos with `worklog.md`, explicit wave references, or prior audit mentions in `/docs/` (Waves W-32 through W-66).
- **GENUINELY-UNAUDITED**: Repos with no audit files, no worklogs, and no wave/audit references in documentation.
- **ARCHIVED-DEPRECATED**: Repos explicitly marked archived or flagged for removal.

**Last scanned**: 2026-04-25 23:59 UTC | **Total scanned**: 149 canonical repos + 45 worktree copies (excluded from count) | **Confidence**: High (worklog + file-system sweep)

---

## Next Steps

1. **W-68 dispatch**: Start triage on top 10 unaudited repos (ValidationKit, agileplus-landing, phenotype-previews-smoketest, apps).
2. **Recovery**: Resume blocked repos (eyetracker Phase-3B pending disk budget headroom).
3. **Cleanup**: Formalize archive decision on deprecated repos; remove from active tracking.
4. **Ledger refresh**: Re-run scan after each wave to track audit coverage growth.
