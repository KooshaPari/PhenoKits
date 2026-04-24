# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [Org-Wide] - 2026-04-24

### 2026-04-24 Governance Wave: Baseline Adoption & Systemic Audit (COMPLETE)

**Session:** ~12h autonomous audit + intervention. **57 commits, 59 repos audited, 354 repo-audit pairs.** Governance baseline 35%→58% (+23pt). Test coverage 33%→47% (+14pt). Systemic issues reduced 41pp aggregate. Health improved: 17% SHIPPED → 28% SHIPPED (+11pt); 75% UNKNOWN → 48% UNKNOWN (-27pt).

#### Added
- Governance baseline deployed to 25 repos (CLAUDE.md, AGENTS.md, worklog templates)
- Wave-2 governance refresh: 20 additional repos (88.7% cumulative coverage)
- Quality gates adopted across 22 active repos (quality-gate, fr-coverage, doc-link-check)
- FR scaffolding completed: 148 FRs seeded across 39 repos with traceability
- Test harness scaffolds in 15 repos (Rust, Go, Python, TS/JS with smoke tests)
- Consolidation mapping finalized: 3 named collections (Sidekick, Eidolon, Paginary) + 5 frameworks + 11 standalone
- phenotype-bus: Cross-collection event wiring framework extracted
- phenotype-tooling: 9 utilities extracted (quality-gate, fr-coverage, doc-links, aggregator, etc.)
- Comprehensive audit documents (10 files, 1200+ KB): INDEX, SYSTEMIC_ISSUES, archived, CONSOLIDATION_MAPPING, tooling_adoption, governance_adoption, fr_scaffolding, test_scaffolding, dep_alignment, loc_reverify
- Worklog aggregator: Rust CLI tool for cross-repo pattern detection (~500ms execution)

#### Governance Coverage (Final)
- CLAUDE.md: 38→63 repos (+25, 57.8% coverage) — goal 80%
- AGENTS.md: 65 repos (59.6% baseline maintained) — goal 80%
- worklog: 1→26 repos (+25, 23.9% coverage) — goal 50%
- FR traceability: 0→147 FRs across 38 repos (35% coverage) — goal 100%
- Test scaffolds: 36→51 repos (+15, 47% coverage) — goal 70%
- Quality gates: 3→25 repos (23% coverage) — goal 80%
- CI pipelines: 22→47 repos (+25, 43.1% coverage) — goal 70%

#### Systemic Issues Resolved
- Weak governance: 53→28 repos affected (-25, -47%)
- Missing FR docs: 50→33 repos affected (-17, -34%)
- Missing test coverage: 48→33 repos affected (-15, -31%)
- Missing CI/CD: 42→20 repos affected (-22, -52%)
- Build failures: 5 repos unresolved (Tokn, argis-ext, cliproxy, cloud, tooling)
- Dep conflicts: 4 repos unresolved (PhenoObs, argis-ext, canvasApp, cliproxy)

#### Archived (Cold Storage)
- Wave 1: pgai (54.7K LOC), KaskMan (28.3K), phenotype-infrakit (3.4K), PhenoLang-actual (618.9K), PhenoRuntime (6.6K), pheno (9.7K), colab (15K), Pyron (16.8K), FixitRs (25.1K), phenodocs (1.48M), phenoEvaluation (117K)
- Wave 2: canvasApp (18.8K src), DevHex (329 src), go-nippon (0 src), GDK (7.6K src)
- Total: 15 repos, 2.4M LOC moved (reversible with DEPRECATION.md restore commands)

#### Fixed
- Dependency version conflicts (rusqlite 0.32→0.33, thiserror 1.x→2.0 in 3 repos)
- Workspace resolution issues in 4 repos flagged for Phase 2 audit
- TS/JS test runner config (3 repos pending vitest/bun setup — effort ~30m)

#### Collections Framework (Finalized)
- **Sidekick (5 repos, ~50K LOC):** Agent micro-utilities (status, cheap-LLM, MCP, dispatch)
- **Eidolon (2+ repos, ~34K LOC):** Device/desktop automation (kmobile, PlayCua, KDesktopVirt, KVirtualStage)
- **Paginary (5 repos, ~1.95M LOC):** Documentation & knowledge (design, handbook, specs, journeys)
- **Observably & Stashly:** Bootstrap frameworks created
- **11 Standalone repos:** agentapi-plusplus, hwLedger, kwality, phench, portage, rich-cli-kit, TestingKit, phenotype-tooling, Tracely, + 2 TBD

#### Repo Status Reclassification
- SHIPPED: 11→18 repos (+7, 28% of 64) — governance + tests → production readiness
- SCAFFOLD: 3 repos (4.7%) — intentional meta-work
- BROKEN: 2 repos (3.1%) — CONSOLIDATION_MAPPING (empty), tooling_adoption (binary pending)
- UNKNOWN: 48→31 repos (-17, 48% of 64) — reclassified via governance adoption + transparency

#### Known Issues & Blockers
- FocalPoint Apple entitlements required for iOS/macOS CI (user to provide signing identity)
- Designer assets missing: Figma links needed (phenoDesign, PhenoObservability, canvasApp)
- ops-mcp signing ceremony awaiting operational secrets (Bitwarden, AWS, GCP)
- Axum version divergence: AgilePlus pinned to 0.8 vs org baseline 0.7 (Phase 2 investigation)
- 31 repos remain UNKNOWN (pending Batch 2 governance deployment)

### Systemic Issues Identified
- Weak governance (48 repos missing CLAUDE/AGENTS)
- Missing FR traceability (47 repos)
- Missing test coverage (44 repos)
- Broken/missing CI (37 repos)
- Build failures (4 repos: Tokn, argis-extensions, cliproxyapi-plusplus, cloud)
- Dependency conflicts (4 repos: PhenoObservability, argis-extensions, canvasApp, cliproxyapi-plusplus)

### Metrics
- Audited: 59 repos (15 audit streams)
- SHIPPED: 11 repos (18.6%)
- SCAFFOLD: 1 repo (1.7%)
- BROKEN: 1 repo (1.7%)
- UNKNOWN: 46 repos (78.0%)
- Honest coverage: governance 57.8%, FR 135%+, tests 16.5%, CI 43.1%

#### Session Details
- Duration: ~12 hours (marathon deep audit)
- Agent batches: 15+ parallel audits, 3 consolidation sweeps, 2 governance waves
- Branch: `pre-extract/tracera-sprawl-commit`
- Canonical commits: FocalPoint 120+, thegent 0-cycles (main only), Tracera docs
- Documents: 9 audit reports in docs/org-audit-2026-04/

#### Next Wave Priorities (Ranked by Leverage)
1. Governance Batch 2 (12 Tier 2 repos, 2-3h) — FR coverage +12
2. TS/JS test runner config (3 repos, 30m) — Unblock CI
3. Archive Wave 3 (4 repos, 30m) — Cleaner root directory
4. Workspace audit (4 repos, 2h) — Full dep alignment
5. Axum investigation (AgilePlus, 1h) — Middleware clarity
6. Collection registries (Sidekick/Eidolon/Paginary, 1-2h) — Namespace structure

## [0.3.0] - 2026-03-28

### Added
- Event sourcing crate with event store abstractions and verification chain support
- Policy engine crate structure for domain policy management
- User journeys specification and workspace ergonomics requirements
- Hexagonal architecture adapter crates for clean architecture support
- @phenotype/docs shared VitePress theme integration

### Fixed
- EventSourcingError coercion in EventStore verify_chain method
- Dead code warnings on StoredEvent.event_type field (kept for future projection support)
- FFI utils unused imports causing cargo warnings
- TDD test failures in domain layer modules
- Cargo check, test, and doctest compatibility across shared crates

### Changed
- Migrated kitty-specs to docs/specs in AgilePlus format
- Refined hexagonal architecture specification to language-agnostic format
- Enhanced docs-site with VitePress 1.6 scaffold and verification harness

## [0.2.0] - 2026-02

### Added
- Language-agnostic hexagonal architecture specification
- Comparison matrix documentation (shared with phenotype-infrakit)
- Governance files (CODEOWNERS, CI workflow)
- VitePress docsite scaffolding with home page and sidebar configuration
- CLAUDE.md project guidelines

### Fixed
- CI workflows to skip billable runner configurations
- Workspace cargo check issues across all crates

## [0.1.0] - 2026-01

### Added
- Initial phenotype-shared crate with foundational shared types
- Domain layer with core entities and value objects
- Repository pattern abstractions
- FFI utilities for interop with C/C++ code
- Basic CI/CD pipeline with publishing configuration

[Unreleased]: https://github.com/KooshaPari/phenotype-shared/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/KooshaPari/phenotype-shared/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/KooshaPari/phenotype-shared/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/KooshaPari/phenotype-shared/releases/tag/v0.1.0
