# Phenotype Org — Consolidated Domain Map

**Last Updated:** 2026-04-24

This document maps every Phenotype product and collection to its proposed primary domain, secondary domains (if applicable), and deployment status.

---

## Primary Organization Domain

| Entity | Domain | Type | Status | Notes |
|--------|--------|------|--------|-------|
| **Phenotype Hub** | phenotype.dev | Landing/Directory | Planned | Org-level marketing, product directory, docs federation |

---

## Product Domains & Assignments

### Sidekick Collection (Personal Productivity)

| Product | Primary Domain | Collection | Description | Status |
|---------|---|---|---|---|
| **FocalPoint** | focalpoint.app | Sidekick | Screen-time management, behavioral nudging, reward ceremonies | Phase 1 (85% complete) |
| | (alt: sidekick-focalpoint.phenotype.dev) | | |  |

---

### Observably Collection (Planning, Observability, Governance)

| Product | Primary Domain | Collection | Description | Status |
|---------|---|---|---|---|
| **AgilePlus** | agileplus.dev | Observably | Spec-driven dev, work packages, FR traceability, agent governance | Active, v0.5.0+ |
| | (alt: agileplus-docs.phenotype.dev) | | docs portal redirect |  |
| **Tracera** | tracera.dev | Observably | Multi-view requirements traceability, project management, RTM | Active, phase 1 stable |
| | (alt: tracera-docs.phenotype.dev) | | docs portal redirect |  |
| **Benchora** | benchora.dev | Observably | Performance benchmarking, baseline tracking, regression detection | Planned |
| | (alt: benchora-perf.phenotype.dev) | | performance dashboard |  |

---

### Vault Collection (Storage, Digital Ledgers)

| Product | Primary Domain | Collection | Description | Status |
|---------|---|---|---|---|
| **Stashly** | stashly.dev | Vault | File storage, organization, digital ledger integration | Planned |
| | (alt: stashly-vault.phenotype.dev) | | | |

---

### Eidolon Collection (Device Automation & Orchestration)

| Product | Primary Domain | Collection | Description | Status |
|---------|---|---|---|---|
| **Eidolon** | eidolon.dev | Eidolon | Device automation (desktop, mobile, sandbox orchestration) | Active, crates extracted |
| | (alt: eidolon-automation.phenotype.dev) | | | |
| **KDesktopVirt** | (eidolon.dev/desktop) | Eidolon | Desktop virtualization & automation | Planned |
| **KVirtualStage** | (eidolon.dev/stage) | Eidolon | Virtual display & sandbox management | Planned |
| **kmobile** | (eidolon.dev/mobile) | Eidolon | Mobile device automation (iOS/Android) | Planned |

---

### Tools Collection (Utilities, Infrastructure, Libraries)

| Product | Primary Domain | Collection | Description | Status |
|---------|---|---|---|---|
| **thegent** | thegent.app | Tools | Dotfiles manager, system provisioning, shell customization | Active |
| | (alt: thegent-docs.phenotype.dev) | | docs portal |  |
| **Paginary** | paginary.dev | Tools | Pagination utilities, library, reusable components | Planned |
| | (alt: lib-paginary.phenotype.dev) | | npm/crates registry entry |  |
| **phenotype-shared** | (npm: @phenotype/shared) | Tools | Rust shared crates workspace (event sourcing, caching, policies, FSM) | Active, crates published |
| **phenotype-go-kit** | (GitHub: KooshaPari/phenotype-go-kit) | Tools | Go shared library (auth, config, health checks) | Active, modules published |
| **@phenotype/design** | (npm: @phenotype/design) | Tools | Design system components, Tailwind tokens, shadcn extensions | Planned |

---

## Supporting & Internal Domains

| Service | Domain | Type | Purpose | Status |
|---------|--------|------|---------|--------|
| **GitHub (Public)** | github.com/KooshaPari | VCS | Public repos, org namespace | Active |
| | github.com/Phenotype-Enterprise | VCS | Shared org namespace | Active |
| **Forgejo** | forge.internal.phenotype.dev | VCS | Private git hosting, CI/CD | Planned (self-hosted) |
| **Docs Aggregator** | docs.phenotype.dev | Docs | Central docs hub, Algolia search | Planned |
| **Package Registry** | registry.phenotype.dev | Packages | npm, cargo.io mirror, Go modules | Planned (self-hosted) |
| **Community** | community.phenotype.dev | Discord/Forum | Discord server, forums | Planned |
| **Blog** | phenotype.dev/blog | CMS | Organization announcements, tutorials | Planned |

---

## Migration Path: Current → Target

### Phase 1: Foundation (Months 1–2)
- [x] phenotype.dev domain acquired/configured
- [ ] phenotype-dev-hub Astro site scaffolded
- [ ] Brand playbook + marketing guidelines documented
- [ ] FocalPoint, AgilePlus, Tracera products linked from hub

### Phase 2: Product Domains (Months 2–3)
- [ ] agileplus.dev domain registered + DNS configured
- [ ] tracera.dev domain registered + DNS configured
- [ ] focalpoint.app domain registered + DNS configured
- [ ] Redirect all old URLs to new domains (301 permanent)
- [ ] Update GitHub repo descriptions, READMEs with new domains

### Phase 3: Docs Federation (Months 3–4)
- [ ] Each product docsite links back to phenotype.dev
- [ ] Algolia DocSearch indexed across all products
- [ ] phenotype.dev/docs/ redirects to unified search/gateway
- [ ] Community links (Discord, GitHub) from phenotype.dev

### Phase 4: Secondary Products (Months 4–5)
- [ ] hwledger.app domain + landing page
- [ ] Benchora.dev domain + landing page
- [ ] Stashly.dev domain + landing page
- [ ] Collection hubs on phenotype.dev active and linked

### Phase 5: Internal Tools (Ongoing)
- [ ] Forgejo self-hosted git + CI/CD
- [ ] Package registry (npm, cargo.io mirror)
- [ ] docs.phenotype.dev centralized docs with Algolia
- [ ] community.phenotype.dev Discord server community hub

---

## Domain Registration Checklist

| Domain | Registrar | Status | Owner | Notes |
|--------|-----------|--------|-------|-------|
| phenotype.dev | (pending) | Planned | Phenotype Org | Primary hub domain |
| agileplus.dev | (pending) | Planned | AgilePlus Team | Product domain |
| tracera.dev | (pending) | Planned | Tracera Team | Product domain |
| focalpoint.app | (pending) | Planned | FocalPoint Team | Product domain |
| hwledger.app | (pending) | Planned | hwLedger Team | Product domain |
| benchora.dev | (pending) | Planned | Benchora Team | Product domain |
| stashly.dev | (pending) | Planned | Stashly Team | Product domain |
| eidolon.dev | (pending) | Planned | Eidolon Team | Product domain |
| paginary.dev | (pending) | Planned | Tools Team | Product domain |
| thegent.app | (pending) | Planned | thegent Team | Product domain |
| docs.phenotype.dev | (sub) | Planned | Phenotype Org | Docs federation gateway |

---

## DNS Configuration (Template)

### Example: agileplus.dev

```yaml
# Registrar: Namecheap / GoDaddy / Cloudflare
# TLD: .dev

A Record:
  Host: @
  Value: (Vercel IP for marketing site)
  TTL: 3600

CNAME Record:
  Host: www
  Value: agileplus-landing.vercel.app
  TTL: 3600

CNAME Record:
  Host: docs
  Value: agileplus-docs.vercel.app  # or docs.agileplus.dev subdomain
  TTL: 3600

MX Record (optional for email):
  Priority: 10
  Value: mail.protonmail.com  # or your email provider
  TTL: 3600

TXT Record (optional for verification):
  Name: _acme-challenge.agileplus.dev
  Value: (Let's Encrypt verification string)
  TTL: 3600

# SSL: Managed by Vercel (auto-renew via Let's Encrypt)
```

---

## Deployment & Hosting Plan

### Marketing Sites (phenotype.dev, product landing pages)

**Platform:** Vercel
- Static Astro 5 builds
- GitHub Actions auto-deploy on push
- Built-in SSL via Let's Encrypt
- Auto-preview deployments for PRs
- Regional edge caching

**Repo:** `/repos/apps/phenotype-dev-hub/` (Astro)

### Documentation Sites

**Platform:** Per-product choice
- **AgilePlus:** VitePress (self-hosted or Vercel)
- **Tracera:** VitePress (self-hosted or Vercel)
- **FocalPoint:** Astro or VitePress
- **hwLedger:** Astro or VitePress
- Unified search: **Algolia DocSearch** (free tier: up to 10K pages)

### Product Applications

**Platform:** Per-product choice
- **FocalPoint:** Xcode + App Store (iOS); Android Studio + Play Store (Android)
- **AgilePlus:** Rust CLI (binary distribution) + dashboard (Vercel)
- **Tracera:** Go binary (GitHub releases) + web UI (Vercel)
- **hwLedger:** Tauri (desktop) + Vercel (web)

---

## Redirect & Alias Strategy

### For Early-Stage Products (Use phenotype.dev subdomains)

Example: **Benchora** (not yet shipped)
```
benchora.phenotype.dev → phenotype.dev/products/benchora (redirect)
```

Once independent domain acquired:
```
benchora.dev → benchora.phenotype.dev (301 redirect)
benchora.dev → new Vercel deployment (direct)
```

### For Shipped Products (Use independent domains immediately)

Example: **AgilePlus** (already published)
```
github.com/KooshaPari/AgilePlus README.md → links to agileplus.dev
agileplus.dev → Vercel marketing site (independent domain)
agileplus.dev/docs → agileplus-docs Vercel or VitePress site
phenotype.dev/products/agileplus → agileplus.dev (external link)
```

---

## SEO & Analytics

### Google Analytics 4 Setup

| Property | Domain | Purpose | Owner |
|----------|--------|---------|-------|
| phenotype.dev | phenotype.dev | Org-level hub tracking | Phenotype Org |
| agileplus.dev | agileplus.dev | Product-specific tracking | AgilePlus Team |
| (per product) | (product domains) | Per-product dashboards | Product Teams |

**Shared Dashboard:** google.analytics.com/phenotype-org (view-level access)

### Search Console Verification

For each domain:
1. Verify domain ownership (DNS TXT or file upload)
2. Submit sitemap: `/sitemap.xml` (auto-generated by Astro)
3. Monitor Core Web Vitals, indexation, crawl errors

### Keyword Strategy

**Org Level (phenotype.dev):**
- "Phenotype org"
- "Agent-native development platform"
- "Spec-driven development stack"
- "AgilePlus OR Tracera OR FocalPoint" (product discovery)

**Product Level (agileplus.dev, etc.):**
- "AgilePlus spec-driven development"
- "Tracera requirements traceability"
- "FocalPoint screen-time management"
- Product-specific terms (e.g., "RTM tool", "spec framework", "digital wellness")

---

## Maintenance & Updates

### Quarterly Review
- [ ] Check domain renewals (set calendar reminders 90 days before expiry)
- [ ] Audit DNS records for stale entries
- [ ] Review link health (404 checker on all phenotype.dev pages)
- [ ] Update CONSOLIDATED_DOMAIN_MAP.md with new products

### Annual Review
- [ ] Evaluate domain portfolio (consolidate if needed)
- [ ] Analyze traffic trends (Analytics 4 reports)
- [ ] Review SEO performance (Search Console)
- [ ] Update brand playbook for consistency

---

## References

- **Marketing Hub Design:** docs/marketing/phenotype_dev_hub_design.md
- **Brand Playbook:** docs/marketing/brand_playbook.md
- **Astro Project:** /repos/apps/phenotype-dev-hub/
- **FocalPoint Landing Reference:** /repos/FocalPoint/apps/web-landing/
