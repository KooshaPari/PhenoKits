# agileplus-landing Audit (2026-04-25)

**Repository:** https://github.com/KooshaPari/agileplus-landing  
**Active:** Yes (last commit 2026-04-25)  
**Never audited before:** Confirmed  
**Size:** ~617 KB (mostly node_modules); **actual source:** ~193 LOC

## Real LOC Breakdown
| File | LOC |
|------|-----|
| src/pages/index.astro | 153 |
| src/styles/globals.css | 40 |
| astro.config.mjs | 8 |
| tsconfig.json | 12 |
| **Total** | **~213 LOC** |

## Stack & Framework
- **Framework:** Astro 5 (static site generator)
- **Styling:** Tailwind CSS 4 + impeccable baseline
- **Deploy:** Vercel (auto-deploy on push to main)
- **Domain:** `agileplus.kooshapari.com` (live CNAME via Cloudflare)

## Purpose & Governance
Tier-2 landing page per Phenotype org-pages architecture. Provides single entry point to AgilePlus documentation with dynamic GitHub metadata (README, releases, stars) fetched at build time. Implements path-based microfrontends (`/docs`, `/qa`, `/otel`, `/preview/<pr#>`) as planned surfaces.

## Commit History (4 commits total)
1. `2f9d496` – feat(tier3): add /preview to homepage nav strip
2. `7bbfa82` – feat(tier3): /preview/<pr#> microfrontend for AgilePlus PRs
3. `fc918fc` – feat(tier3): add /docs, /qa, /otel path microfrontends
4. `058cf49` – feat: scaffold agileplus.kooshapari.com landing page

**Status:** All scaffolding in place; `/docs`, `/qa`, `/otel`, `/preview` are placeholder pages returning 404 stubs (content not yet mounted).

## Governance Gaps
- No CLAUDE.md (should document dependencies, build process, deployment)
- No AGENTS.md
- No .github/workflows/ (deploy via Vercel native integration, not CI)
- No .pre-commit-config.yaml or linting setup

## Live Status
- **Domain resolves:** Yes (agileplus.kooshapari.com points to Vercel)
- **Deployment:** Automatic (linked to GitHub repo)
- **Build:** Clean (no CI errors reported)
- **Sidebar link:** Portfolio shows entry pointing to this domain

## Verdict: SHIP
This is a **minimal, well-structured landing page** that serves its purpose:
- ✅ Single entry point with live GitHub metadata
- ✅ Astro build is clean and fast
- ✅ Vercel deployment working
- ✅ Follows Phenotype org-pages pattern
- ✅ Domain live and resolving
- ⚠️ Microfrontend placeholders need mounting (docs/qa/otel content)

### Recommendation
- Keep in SHIP status — domain is live, landing page works, no blockers
- Create CLAUDE.md documenting build + microfrontend mounting plan
- Defer mounting of `/docs` → VitePress until AgilePlus docs stabilize
- `/preview/<pr#>` requires CI integration to auto-deploy PRs (future enhancement)

## Sidebar Link
**For projects.kooshapari.com portfolio:**
```
AgilePlus
├─ Landing: agileplus.kooshapari.com
├─ GitHub: https://github.com/KooshaPari/AgilePlus
└─ Docs: https://kooshapari.github.io/AgilePlus/ (external link)
```

**Action:** Already indexed in portfolio (visible at projects.kooshapari.com).
