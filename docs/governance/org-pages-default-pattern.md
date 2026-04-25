# Org Pages Default Expansion Pattern

**Status:** Active Policy | **Date:** 2026-04-25 | **Scope:** Phenotype Org

## Overview

Every new Phenotype project automatically gets portfolio presence across three tiers:

1. **Portfolio Index** — `projects.kooshapari.com` (enumeration + filtering)
2. **Project Landing** — `<project>.kooshapari.com` (per-project landing page)
3. **Path-Based Microfrontends** — `projects.kooshapari.com/<project>` (in-site deeplinks)

## Architecture

### Tier 1: Portfolio Index (`projects.kooshapari.com`)

**Generator:** `/repos/projects-landing/` (Astro + Tailwind)

- Reads `/repos/docs/org-audit-2026-04/github_remote_inventory.md` (or live GitHub API via `GH_TOKEN`)
- Generates one card per synced repo: name, tagline, status, LOC, topics
- Links: repo on GitHub + landing site (if exists)
- Filtering: by status (active/archived), visibility (public/private), topics
- Static export → Vercel (`projects.kooshapari.com`)

**Data Pipeline:**
- `scripts/fetch-repos.sh` — calls `gh repo list KooshaPari --json name,description,url,topics,isPrivate,isArchived,stargazerCount,pushedAt`
- Writes to `data/repos.json` (committed or generated at build time)
- Nightly sync via GitHub Actions + `GH_TOKEN`, OR on-demand (`npm run sync:repos`)

### Tier 2: Project Landing (`<project>.kooshapari.com`)

**Owner:** Per-project team (optional; user can decline in scaffold plan)

- Custom Astro site per project (reference: `src/<project>/` in projects-landing OR separate repo)
- Deployed to `<project>.kooshapari.com` via Vercel
- Includes: project README, feature highlights, quick start, links to docs/GitHub

### Tier 3: Path-Based Microfrontends

**Location:** `projects.kooshapari.com/<project>`

- Embedded preview cards, docs summaries, or interactive demos
- Powered by Astro Island Architecture
- Lazy-loaded, isolated from main site bundle

## Implementation Checklist

### For New Projects

1. [ ] **Add metadata** to repo (GitHub topics, description)
2. [ ] **Run data sync** (`npm run sync:repos` in projects-landing)
3. [ ] **Verify card renders** at `projects.kooshapari.com`
4. [ ] [OPTIONAL] **Create landing site** at `<project>.kooshapari.com`
   - Scaffold from `/repos/projects-landing/src/templates/<project>/`
   - Deploy via Vercel
   - Link in portfolio index
5. [ ] [OPTIONAL] **Add microfrontend** for path `/projects.kooshapari.com/<project>`
   - Create Astro Island in `src/islands/<project>.astro`
   - Reference in index page

### For Portfolio Index Maintenance

1. **Weekly Sync:** GitHub Actions cron runs `scripts/fetch-repos.sh` → `data/repos.json`
2. **Tag New Repos:** Add `org-page` topic to GitHub repos to flag for portfolio
3. **Build & Deploy:** Vercel auto-deploys on `main` push
4. **Monitor:** Check Vercel analytics for portfolio traffic

## Configuration

### Environment Variables (Build Time)

- `GH_TOKEN` — GitHub API token for fetching repo metadata (optional; falls back to public API)
- `VERCEL_TOKEN` — Vercel deployment (automated if linked)

### Cloudflare DNS

```
CNAME projects.kooshapari.com → cname.vercel-dns.com (propagation ~5 min)
```

### Tailwind + CSS

Shared tokens with `phenotype-dev-hub`:
- Color palette: `brand-{50,100,500,600,900}` + semantic colors (green, amber, blue, purple)
- Typography: monospace for code, system sans for prose
- Baseline: Impeccable reset (box-sizing, font-smoothing, word-wrap)

## Deployment

1. **Local Build:**
   ```bash
   cd /repos/projects-landing
   bun install
   bun run build
   bun run preview
   ```

2. **Vercel Deploy:**
   ```bash
   vercel link
   vercel deploy --prod
   ```

3. **Monitor:**
   - Vercel UI: dashboards for deployments, analytics
   - GitHub: workflow runs + logs

## User Declination

At project kickoff, the scaffold plan offers checkbox:
- **Accept:** Add portfolio card + landing site (default)
- **Decline:** Skip portfolio tier 1–3 (can re-enable later)

Declined projects remain in `data/repos.json` but marked `opt-out: true` (not rendered).

## Future Enhancements

- [ ] Analytics integration (Vercel Web Analytics)
- [ ] Search bar for portfolio index (Algolia or Meilisearch)
- [ ] Topic-based landing pages (`projects.kooshapari.com/phenotype`, etc.)
- [ ] Auto-generate per-project `<project>.kooshapari.com` from README
- [ ] GitHub Actions badge integration (CI status)

---

**Reference:** Global policy in `~/.claude/CLAUDE.md` → "Standing policy (Org Pages Default Expansion)"
