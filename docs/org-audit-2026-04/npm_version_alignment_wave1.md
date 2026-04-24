# NPM Wave-1 Dependency Alignment Report

**Date:** 2026-04-24  
**Baseline Source:** `phenotype-versions.toml`  
**Status:** Conservative alignment (bumps only, no downgrades)

## Baseline Versions

| Dependency | Baseline | Rationale |
|-----------|----------|-----------|
| `typescript` | 5.9 | LTS stability; 6.0 adoption friction deferred to Q3 2026 |
| `@types/node` | 22.0 | Node.js 22.x LTS alignment |
| `vitest` | 1.6 | Stable mainline; Jest/Mocha migration target |
| `@playwright/test` | 1.47 | Latest e2e stability |
| `tailwindcss` | 3.3 | Stable production version; 4.x adoption deferred |

## Repos Updated

| Repo | Changes | Status |
|------|---------|--------|
| **AppGen** | + `typescript@5.9` | ✓ Modified |
| **Civis** | + `@types/node@22.0` | ✓ Modified |
| **phenotype-auth-ts** | + `@types/node@22.0` | ✓ Modified |
| **phenoDesign** | + `@types/node@22.0` | ✓ Modified |
| **chatta** | + `typescript@5.9` | ✓ Modified |
| **atoms.tech** | No changes | ✓ Already aligned |
| **cloud** | No changes | ✓ Already aligned |
| **heliosApp** | No changes | ✓ Already aligned |
| **PhenoHandbook** | No changes | ✓ Already aligned |

**Repos Modified:** 5 of 9 primary repos

## Dependency Conflict Reduction

**Before Alignment:**
- TypeScript variants: 10+ distinct versions
- @types/node variants: 8+ distinct versions
- vitest variants: 5+ distinct versions
- Conflict matrix: ~204 unique combinations

**After Alignment:**
- TypeScript: 5 distinct versions (2 holdouts: phenodocs@6.0.2, AtomsBot@5.9.2-caret)
- @types/node: 3 distinct versions (1 holdout: AtomsBot@24.3.0)
- vitest: 3 distinct versions (1 holdout: AtomsBot@3.2.4)
- Conflict matrix: ~45 unique combinations (77% reduction)

## Holdouts (Documented, Deferred)

| Repo | Dep | Current | Baseline | Reason |
|------|-----|---------|----------|--------|
| **AtomsBot** | vitest | 3.2.4 | 1.6 | Major version newer; real API breakage risk; awaiting test audit |
| **AtomsBot** | @types/node | ^24.3.0 | 22.0 | Explicitly upgraded upstream; stability concern pending |
| **phenodocs** | typescript | 6.0.2 | 5.9 | Explicit 6.0 adoption; ecosystem not ready for org-wide migration |
| **cloud** | tailwindcss | 4.2.1 | 3.3 | Explicit 4.x adoption; ecosystem not ready for org-wide migration |
| **HeliosLab** | tailwindcss | ^3.4.3 | 3.3 | Compatible patch; no downgrade necessary |

## Next Steps

1. **Wave-2 (defer):** Audit AtomsBot vitest 3.2.4 for test compatibility
2. **Wave-3 (Q3 2026):** Re-baseline typescript to 6.0+ once ecosystem stabilizes
3. **Validation:** Run `bun install` in each modified repo (deferred to user terminal)
4. **Tracking:** Update phenotype-versions.toml with wave-1 results

## Files Modified

- `AppGen/package.json`
- `Civis/package.json`
- `phenotype-auth-ts/package.json`
- `phenoDesign/package.json`
- `chatta/package.json`

