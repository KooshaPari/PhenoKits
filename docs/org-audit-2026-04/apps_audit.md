# Audit: apps/ (W-69 MASTER_AUDIT_LEDGER)

## Repository Summary

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/apps`  
**Status:** alpha  
**Real LOC:** ~1 LOC (single compiled Astro output)  
**Disk Size:** 140M (entirely node_modules + compiled dist)  
**License:** MIT

## Structure

Monorepo with single application:
- `phenotype-dev-hub/` — Astro static site builder
  - `.astro/` — Astro build cache (empty, unused)
  - `dist/` — Compiled output (static HTML/CSS/JS)
  - `node_modules/` — 232 subdirs, 140M (all build deps)
  - No source files tracked in git

## Metadata

| Marker | Finding |
|--------|---------|
| **Git Tracking** | `apps/` is NOT a submodule; appears to be an uncommitted directory |
| **Source Files** | Zero tracked source code (no .astro, .ts, .tsx, .js, .jsx outside node_modules) |
| **Entry Point** | Unknown (no package.json, astro.config.ts, or README in phenotype-dev-hub/) |
| **CI Workflows** | None (only node_modules/@ungap/.github/workflows) |
| **Cross-Refs** | Referenced in: git log `fc79be2f8 "marketing: phenotype.dev hub design + brand playbook + apps/ scaffold"` |
| **Status Indicator** | README says "alpha" with no feature list |

## Recent History

- **fc79be2f8** (undated) — `chore(marketing): phenotype.dev hub design + brand playbook + apps/ scaffold`
- **2eaa3ba5b** (2026-04-25) — `audit(org): Master audit ledger — honest classification across 149 repos`
- No commits directly to `apps/` detected

## Assessment

**VERDICT: ARCHIVE or FIX**

### Issues

1. **No Source Code** — Directory contains only node_modules + compiled output; no source tracked
2. **Incomplete Setup** — Missing astro.config.ts, package.json, source files (src/, pages/, etc.)
3. **Not a Git Subtree** — Marked in ledger as 87K LOC (node_modules fallacy)
4. **Unclear Purpose** — Named "phenotype-dev-hub" but referenced as "apps/" ecosystem

### Path Forward

**Option A: ARCHIVE (Recommended)**
- If this is a scaffolding placeholder or incomplete prototype, archive to `.archive/apps/`
- Saves 140M disk + removes confusion

**Option B: FIX (If Still Needed)**
- Restore source files from git history (commit ~fc79be2f8 or earlier)
- Add astro.config.mjs, package.json, src/ structure
- Document purpose and product portfolio integration
- Link from portfolio tracker

## Recommendation

**ARCHIVE.** This repo appears to be a marketing scaffold that was committed incomplete. No active development, zero source files, and 140M of compiled artifacts that can be rebuilt. Free disk, clarify org scope.
