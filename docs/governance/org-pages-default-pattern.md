# Org Pages Default Expansion Pattern

**Status:** Standing policy (2026-04-24)
**Scope:** All KooshaPari + Phenotype-org repositories
**Anchors:**
- `phenotype-infra/docs/governance/org-pages-architecture.md` (three-layer architecture)
- `phenotype-infra/docs/governance/org-pages-coverage.md` (current state matrix)
- `repos/CONSOLIDATED_DOMAIN_MAP.md` (product domains: `<product>.dev`)

---

## Default Expansion Mandate

When any new project is created, agents MUST plan for these surfaces by default — not as a follow-up:

1. **Org portfolio entry** — listed on `projects.kooshapari.com` (auto-generated from `gh repo list` + topic filter `org-page`).
2. **Project landing** — `<project>.kooshapari.com` CNAME → Vercel project (one cert, one project).
3. **Path-based microfrontends within the project domain** — no DNS sprawl:
   - `/` — landing
   - `/docs` — VitePress or Next.js docs
   - `/otel` — observability surface (PhenoObservability OTLP-fed)
   - `/qa` — quality dashboard (coverage, lint, FR-traceability)
   - `/preview/<pr#>` — Vercel preview deploys per PR
4. **Canonical product domain coexistence** — when a `<product>.dev` exists, it is canonical and `<product>.kooshapari.com` 301-redirects to it.

## When This Triggers

This default applies when an agent:
- Creates a new repo under KooshaPari/Phenotype org
- Promotes an internal tool to a public surface
- Detects a repo with no homepage URL and `archived=false`
- Drafts an architecture / decomposition / new-project proposal

Do not wait for the user to ask for the landing page — include it in the initial plan with a checkbox the user can decline.

## Path Microfrontend Convention

Microfrontends are mounted as paths, not subdomains. A project is one Vercel project with one DNS record. Path routing handled by Next.js rewrites or VitePress nav. Never create `otel.<project>.kooshapari.com` or `qa.<project>.kooshapari.com` — use `<project>.kooshapari.com/otel` and `/qa`.

## Auto-Generation Inputs

Portfolio site (`projects.kooshapari.com`) reads from:
- `gh repo list KooshaPari --limit 200 --json name,description,url,homepageUrl,topics,isArchived,pushedAt`
- Topic filter: include `org-page`, exclude `archived`, `private-only`
- Each repo's `.github/org-page.json` (optional override: featured banner, demo URL, status badge)

## Acceptance Criteria for "Done"

A project is "org-pages complete" when:
- [ ] Repo has `homepageUrl` set on GitHub
- [ ] Repo has topic `org-page` (and optionally `featured`)
- [ ] CNAME `<project>.kooshapari.com → cname.vercel-dns.com` exists in Cloudflare
- [ ] Vercel project deployed and 200-OK on root
- [ ] `/docs` route serves docs (or 404 with friendly message + repo link)
- [ ] Listed in `projects.kooshapari.com` portfolio
- [ ] If `<product>.dev` exists: `<project>.kooshapari.com` 301s there

## Open Decisions

Surfaced for user (track in `org-pages-architecture.md § Open Decisions`):
1. Wildcard fallback: delete dead tunnel CNAME vs Vercel 404 page (recommend Vercel fallback).
2. `projects-landing` framework: VitePress vs Next.js (recommend Next.js for filter/search).
3. Canonical preference for product-branded repos: `.dev` vs `.kooshapari.com`.

## References (External Patterns)

- Vercel templates gallery — single Next.js project, filter/search by tag
- Astro showcase — auto-generated from frontmatter
- GitHub org READMEs — pinned repos + topic-based grouping
- Kubernetes.io project listings — community-maintained portfolio

---

## Agent Instruction Surface

This policy is referenced from:
- `~/.claude/CLAUDE.md` → "Org Pages Default Expansion"
- `~/.codex/AGENTS.md` → "Org Pages Default Expansion"
- `~/.claude/projects/-Users-kooshapari-CodeProjects-Phenotype-repos/memory/MEMORY.md` → reference entry
