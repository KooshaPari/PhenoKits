# Stashly Migrations & PhenoObservability Adoption Survey (2026-04-25)

**Audit scope:** Phenotype org adoption of three key patterns since W-49 baseline.

## Pattern Adoption Status

| Pattern | Adopters | Files | Status | Trend |
|---------|----------|-------|--------|-------|
| **stashly-migrations** | 1 repo | 2 files | Dormant | Stalled since W-49 |
| **#[async_instrumented]** | 1 repo | 15 files | Active | Contained in PhenoObservability |
| **#[pii_scrub]** | 2 repos | 5 files | Light adoption | Minimal cross-repo use |

## Key Findings

**stashly-migrations**: Only phenotype-shared crates/phenotype-migrations (library definition itself). Zero consumer adoption across org. Pattern remains undeployed.

**async_instrumented macro**: Concentrated exclusively in PhenoObservability crates (phenotype-surrealdb, tracingkit, phenotype-mcp-server, phenotype-observably-macros). No cross-repo spillover. 15 usages indicate internal module instrumentation, not org-wide pattern adoption.

**pii_scrub macro**: Minimal adoption: FocalPoint/focus-telemetry (1 file) + PhenoObservability/phenotype-observably-macros (definition). Pattern exists but unused in telemetry pipelines.

## Adoption Gaps

- **stashly-migrations** requires consumer adoption; phenotype-shared defines pattern but no downstream use
- **async_instrumented** isolated to origin repo; other Tier-1 repos (AgilePlus, heliosApp, argis-extensions) lack instrumentation
- **pii_scrub** defined but dormant; no telemetry consumers apply PII scrubbing

## Candidate for Cross-Repo Migration

**FocalPoint** (Rust monorepo, 467K LOC): Zero async_instrumented usage despite 20+ async handlers. Ideal candidate for:
- Adopt #[async_instrumented] in routes.rs and service layers
- Integrate pii_scrub in focus-telemetry module
- Validate stashly-migrations pattern for cache persistence

## Recommendation

Patterns are defined but need activation roadmap: (1) deploy stashly to phenotype-infrakit, (2) backport async_instrumented to AgilePlus + heliosApp, (3) enable pii_scrub in all telemetry sinks.
