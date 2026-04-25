# projects-landing

Org-level portfolio site for `projects.kooshapari.com`. Static index of every active KooshaPari repository with status, topics, and star count.

**Status:** Scaffolded (2026-04-25) | **Framework:** Astro + Tailwind CSS | **Package Manager:** bun

## Quick Start

```bash
bun install
bun run dev       # localhost:3000
bun run build     # ./dist/
bun run sync:repos # fetch fresh repo data from GitHub
```

## Architecture

See `/repos/docs/governance/org-pages-default-pattern.md` for the full org-pages expansion policy.

### Tiers

1. **Portfolio Index** — This site: `projects.kooshapari.com` (card grid, filtering by status/topic)
2. **Project Landings** — Optional per-project: `<project>.kooshapari.com`
3. **Microfrontends** — Embedded: `projects.kooshapari.com/<project>`

## Data Pipeline

`scripts/fetch-repos.sh` calls `gh repo list KooshaPari --json name,description,url,topics,isPrivate,isArchived,stargazerCount,pushedAt` and writes `data/repos.json`. 

Run on-demand:
```bash
npm run sync:repos  # or: bash scripts/fetch-repos.sh
```

GitHub Actions workflow (planned): nightly sync via `GH_TOKEN`.

## Build & Deploy

1. **Local:**
   ```bash
   bun run build
   bun run preview
   ```

2. **Vercel:**
   ```bash
   vercel link
   vercel deploy --prod
   ```

3. **DNS:** Set Cloudflare CNAME to `cname.vercel-dns.com`

## Structure

```
src/
  pages/
    index.astro          # Portfolio index (reads data/repos.json)
  layouts/
    BaseLayout.astro     # Shared header/footer
  components/
    RepoCard.astro       # Single repo card
data/
  repos.json            # Synced from GitHub (commits or build-time)
scripts/
  fetch-repos.sh        # GitHub API sync script
```

## Pages

- `/` — Full portfolio grid
- Future: `/topics/<topic>`, `/<project>` (microfronts)

## Styling

- **Framework:** Tailwind CSS v4 (latest)
- **Baseline:** Impeccable CSS reset
- **Colors:** Shared brand palette (blue/green/purple/amber)
- **Dark Mode:** Supported via `dark:` utilities

## Future

- [ ] Search/filter on client side
- [ ] GitHub Actions CI/CD integration
- [ ] Analytics (Vercel Web Analytics)
- [ ] Per-project landing sites
- [ ] Microfrontend islands
