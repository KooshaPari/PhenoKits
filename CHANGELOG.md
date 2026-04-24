# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [Org-Wide] - 2026-04-24

### Governance Wave: Baseline Adoption & Systemic Audit

#### Added
- Governance baseline deployed to 25 repos (CLAUDE.md, AGENTS.md, worklog templates)
- Quality gates adopted across 22 active repos (quality-gate, fr-coverage, doc-link-check)
- FR scaffolding completed: 148 FRs seeded across 39 repos with traceability
- Test harness scaffolds in 15 repos (Rust, Go, Python, TS/JS with smoke tests)
- Consolidation mapping: 3 named collections (Sidekick, Eidolon, Paginary) + 11 standalone
- Dependency alignment baseline (FocalPoint v0.33 rusqlite, v1.40 tokio, v0.7 axum)
- Comprehensive audit documents (9 streams, 354 repo-audit pairs)

#### Governance Coverage
- CLAUDE.md: 38→63 repos (+25, 57.8% coverage)
- AGENTS.md: 65 repos (59.6% baseline maintained)
- worklog: 1→26 repos (+25, 23.9% coverage)
- FR traceability: 0→147 FRs (135%+ coverage across repos)
- Test scaffolds: 3→18 repos (+15, 16.5% coverage)
- Quality gates: 3→25 repos (22.9% coverage)
- CI pipelines: 22→47 repos (+25, 43.1% coverage)

#### Archived (Cold Storage)
- Wave 1: pgai (54.7K LOC), KaskMan (28.3K), PhenoLang-actual (618.9K), PhenoRuntime (6.6K), pheno (9.7K), colab (15K), Pyron (16.8K), FixitRs (25.1K), phenodocs (1.4M), phenoEvaluation (117K)
- Wave 2: canvasApp (18.8K src), DevHex (329 src), go-nippon (0 src), GDK (7.6K src)
- Total: 15 repos, 2.4M LOC moved (reversible with DEPRECATION.md)

#### Fixed
- Dependency version conflicts (rusqlite 0.32→0.33, thiserror 1.x→2.0 in 3 repos)
- Workspace resolution issues in 4 repos flagged for Phase 2 audit
- TS/JS test runner config (3 repos pending vitest/bun setup)

#### Known Issues
- FocalPoint Apple entitlements required for iOS/macOS CI
- Designer assets missing (phenoDesign, PhenoObservability, canvasApp)
- ops-mcp signing ceremony awaiting operational secrets
- Axum version divergence in AgilePlus (0.8 vs org baseline 0.7)
- 46 repos remain UNKNOWN (78% of audit scope)

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
