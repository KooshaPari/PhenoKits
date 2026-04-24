# Phenotype Tooling Adoption — Wave 1 & 2

Status: Wave-2 complete (2026-04-24)

## Target Repos (22 active)

| Repo | CI before | CI after | Tools adopted | Status |
|------|-----------|----------|---------------|--------|
| AgentMCP | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| KlipDot | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| PhenoHandbook | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| PlayCua | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| Tracely | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| agent-user-status | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| agentapi-plusplus | Partial (existed) | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| atoms.tech | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| cheap-llm-mcp | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| hwLedger | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| kmobile | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| kwality | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| phench | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| phenoDesign | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| phenotype-auth-ts | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| phenotype-journeys | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| phenotype-ops-mcp | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| phenotype-tooling | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| portage | None | quality-gate + fr-coverage | quality-gate, fr-coverage | Complete |
| rich-cli-kit | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| thegent-dispatch | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |
| thegent-workspace | None | quality-gate + fr-coverage + doc-links | quality-gate, fr-coverage, doc-link-check | Complete |

## Workflows Created

- **quality-gate.yml**: Runs `phenotype-tooling quality-gate --quick` on every push/PR (continue-on-error: true per billing constraint)
- **fr-coverage.yml**: Runs `phenotype-tooling fr-coverage --pr` on PRs to validate FR traceability
- **doc-links.yml**: Runs `phenotype-tooling doc-link-check docs/` when docs/ exists

## Tools Adopted

| Tool | Purpose | Deployed |
|------|---------|----------|
| quality-gate | Quick lint + type checks | All 22 repos |
| fr-coverage | FR traceability validator | All 22 repos |
| doc-link-check | Broken link detection in docs | 2 repos (PhenoHandbook, phenoDesign) |
| agent-orchestrator | N/A (workflow only) | Available in tooling/ symlinks |
| audit-privacy | Privacy audit (optional) | Available in tooling/ symlinks |
| bench-guard | Benchmark regression (optional) | Available in tooling/ symlinks |
| commit-msg-check | Commit message validation (optional) | Available in tooling/ symlinks |
| sbom-gen | Software bill of materials | Available in tooling/ symlinks |
| fuzz-setup | Fuzzing harness setup (optional) | Available in tooling/ symlinks |

## Wave-2 Repos (20 additional)

| Repo | CI before | CI after | Status |
|------|-----------|----------|--------|
| AppGen | None | quality-gate + fr-coverage + doc-links | Complete |
| AtomsBot | None | quality-gate + fr-coverage + doc-links | Complete |
| AuthKit | None | quality-gate + fr-coverage + doc-links | Complete |
| BytePort | None | quality-gate + fr-coverage + doc-links | Complete |
| Dino | None | quality-gate + fr-coverage + doc-links | Complete |
| Eidolon | None | quality-gate + fr-coverage + doc-links | Complete |
| HeliosLab | None | quality-gate + fr-coverage + doc-links | Complete |
| KDesktopVirt | None | quality-gate + fr-coverage + doc-links | Complete |
| McpKit | None | quality-gate + fr-coverage + doc-links | Complete |
| Paginary | None | quality-gate + fr-coverage + doc-links | Complete |
| PhenoDevOps | None | quality-gate + fr-coverage + doc-links | Complete |
| PhenoMCP | None | quality-gate + fr-coverage + doc-links | Complete |
| PhenoPlugins | None | quality-gate + fr-coverage + doc-links | Complete |
| PhenoProc | None | quality-gate + fr-coverage + doc-links | Complete |
| PhenoVCS | None | quality-gate + fr-coverage + doc-links | Complete |
| Sidekick | None | quality-gate + fr-coverage + doc-links | Complete |
| Tracera-recovered | None | quality-gate + fr-coverage + doc-links | Complete |
| chatta | None | quality-gate + fr-coverage + doc-links | Complete |
| netweave-final2 | None | quality-gate + fr-coverage + doc-links | Complete |
| Conft | None | quality-gate + fr-coverage + doc-links | Complete |

## Summary

- **Wave-1:** 22 repos onboarded (April 18-24)
- **Wave-2:** 20 repos onboarded (April 24)
- **Total CI coverage:** 42 active repos (quality-gate, fr-coverage, doc-links)
- **Remaining:** ~65 repos flagged in UPLIFT_REPORT (wave-3+ pending)

## Next Steps

1. Monitor Wave-2 CI execution (note: GitHub Actions billing constraints apply)
2. Plan Wave-3+ for remaining 65 repos (prioritize by LOC + project tier)
3. Aggregate CI metrics across waves
