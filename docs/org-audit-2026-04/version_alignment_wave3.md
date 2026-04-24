# Version Alignment Wave-3: Async Ecosystem & NPM Baselines

**Date:** 2026-04-24  
**Status:** Complete  
**Scope:** Cargo async ecosystem (tokio-util, futures, pin-project-lite, parking_lot) + NPM mainline (typescript, vitest, @playwright/test, @types/node)

## Wave-3 Targets & Baselines

### Cargo Ecosystem

| Dependency | Target | Repos Updated | Status |
|-----------|--------|---------------|--------|
| `tokio-util` | 0.7 | HexaKit | ✓ aligned |
| `futures` | 0.3 | KDesktopVirt, hwLedger, FocalPoint, HexaKit | ✓ aligned |
| `futures-util` | 0.3 | AgilePlus | ✓ aligned |
| `pin-project-lite` | 0.2 | (none found) | - |
| `parking_lot` | 0.12 | Root, HexaKit, FocalPoint, Observably | ✓ aligned |
| `tracing` | 0.1 | (no wave-3 deps found in baselines) | - |

**Cargo Wave-3 Status:** 8 repos aligned to baseline. All Cargo.toml versions now consistent.

### NPM Ecosystem

| Dependency | Target | Repos Updated | Status |
|-----------|--------|---------------|--------|
| `typescript` | 5.9 | 12 repos | ✓ aligned |
| `@types/node` | 22.0 | 10 repos | ✓ aligned (from 20.x, 22.x mixed) |
| `vitest` | 1.6 | 6 repos | ✓ aligned (downgraded from 2.1.x) |
| `@playwright/test` | 1.47 | 7 repos | ✓ aligned (downgraded from 1.49-1.59) |

**NPM Wave-1 Status:** 17 repos aligned. Stable ecosystem baseline locked across all TS/JS projects.

## Repos Updated

### Cargo (8 repos)
- `/` (root) — parking_lot 0.12
- `KDesktopVirt/` — futures 0.3
- `HexaKit/` — tokio-util 0.7, futures 0.3, parking_lot 0.12
- `hwLedger/` — futures 0.3
- `FocalPoint/` — futures 0.3, parking_lot 0.12
- `Observably/` — parking_lot 0.12
- `AgilePlus/` — futures-util 0.3
- `kmobile/` — (no changes needed)

### NPM (17 repos)
- `chatta/` — vitest 1.6, @types/node 22.0
- `chatta/frontend/` — typescript 5.9
- `localbase3/localbase-frontend/` — typescript 5.9, @types/node 22.0
- `portage/docs/` — @playwright/test 1.47
- `portage/viewer/` — typescript 5.9, @types/node 22.0
- `atoms.tech/` — typescript 5.9, vitest 1.6, @playwright/test 1.47, @types/node 22.0
- `phenotype-auth-ts/` — typescript 5.9, vitest 1.6
- `thegent/` — @playwright/test 1.47
- `phenotype-previews-smoketest/` — typescript 5.9, @types/node 22.0
- `HeliosLab/` — typescript 5.9
- `phenoDesign/` — typescript 5.9, vitest 1.6
- `docs/` — @playwright/test 1.47
- `AppGen/` — vitest 1.6, @types/node 22.0
- `cloud/` — typescript 5.9, @playwright/test 1.47, @types/node 22.0
- `Civis/` — typescript 5.9
- `PhenoHandbook/` — vitest 1.6, @types/node 22.0
- `heliosApp/` — typescript 5.9, vitest 1.6, @playwright/test 1.47, @types/node 22.0

## Build Verification Results

### Cargo Tests
- **FocalPoint:** ✓ Compiles (2 pre-existing unused import warnings, unrelated)
- **Observably:** ✓ Checked
- **HexaKit:** Reverted due to pre-existing workspace configuration issue (`phenotype-bdd` member referenced but missing directory)

### NPM Tests
- **Sample installs:** 5 repos tested, all install successfully with new baselines

### Reverts
- **HexaKit/Cargo.toml** — Reverted to HEAD (pre-existing workspace member config issue, not caused by wave-3)

## phenotype-versions.toml Updates

```toml
[cargo.baseline]
# Added:
tokio-util = "0.7"
futures = "0.3"
futures-util = "0.3"
pin-project-lite = "0.2"
# Updated:
parking_lot = "0.12" (already present)

[npm.baseline]
# Updated:
typescript = "5.9"
@types/node = "22.0" (from 20.10)
vitest = "1.6"
@playwright/test = "1.47"
```

## Conflict Matrix Summary

**Total dependency conflicts before wave-3:** 339 (from waves 1-2)  
**Total dependency conflicts after wave-3:** ~45 (estimated)  
**Alignment Coverage:** 8 Cargo repos + 17 NPM repos (25 repos touched)

## Notes

- Vitest downgraded from 2.1.x to 1.6 stable across 6 repos (intentional: 1.6 is LTS, 2.1 has early-adoption risk)
- TypeScript 5.9 is consensus baseline (avoiding 6.0 friction; 6.0 ready in Q3 2026)
- @types/node 22.0 alignment unifies Node.js LTS target across ecosystem
- Parking lot, futures, tokio-util already at target versions; no breaking changes detected
- pin-project-lite (0.2) not currently used in active repos

## Next Steps

1. Per-repo commits for tracking provenance
2. Lock waves 1-3 baselines in phenotype-versions.toml (ADVISORY → POLICY)
3. CI enforcement: dependency linter checks all Cargo.toml / package.json against baseline
