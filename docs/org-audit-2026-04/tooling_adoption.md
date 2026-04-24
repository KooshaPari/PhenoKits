# Phenotype Tooling Adoption — Phase 1

Status: In progress (2026-04-24)

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

## Next Steps

1. Build phenotype-tooling binaries (in progress)
2. Deploy workflows to all 22 repos
3. Create per-repo commits (`chore(ci): adopt phenotype-tooling...`)
4. Parent commit to canonical repos root
5. Monitor CI execution (note: GitHub Actions billing constraints apply)
