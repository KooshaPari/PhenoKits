# Phenotype.dev Hub — Org-Level Marketing & Federation Strategy

## Overview

`phenotype.dev` is the unified marketing landing page and product catalog for the Phenotype organization ecosystem. It federates all named collections, products, and documentation into a cohesive brand presence while maintaining individual product identities and separate deployment domains.

**Design Philosophy:** Single source of discovery; federated product experience.

---

## Domain Strategy

### Primary Landing Domain
- **phenotype.dev** — org-level landing, product directory, collection hub, docs gateway

### Federated Product Domains
Products maintain independent domains while appearing in phenotype.dev catalog:

| Product | Domain | Collection | Purpose |
|---------|--------|-----------|---------|
| **FocalPoint** | focalpoint.app | Sidekick | Screen-time management, behavioral nudging |
| **AgilePlus** | agileplus.dev | Observably | Spec-driven dev, traceability, governance |
| **Tracera** | tracera.dev | Observably | Multi-view project management, RTM |
| **hwLedger** | hwledger.app | Tools | LLM capacity planning, fleet management |
| **Stashly** | stashly.dev | Vault | File storage, digital ledger |
| **Eidolon** | eidolon.dev | Eidolon | Device automation (desktop, mobile, sandbox) |
| **Paginary** | paginary.dev | Tools | Pagination utilities, library |
| **Benchora** | benchora.dev | Observably | Performance benchmarking toolkit |
| **thegent** | thegent.app | Tools | Dotfiles manager, system provisioning |

### Alternative: Subdomain Strategy (Optional)
If org wants unified hosting:
- `phenotype.dev` — root landing
- `focalpoint.phenotype.dev` — FocalPoint marketing
- `agileplus.phenotype.dev` — AgilePlus marketing
- etc.

**Recommendation:** Use independent domains (above) for SEO flexibility; link from phenotype.dev. Easier to sell/spin off products; better Google juice per product.

---

## Technology Stack

### Framework & Build
- **Framework:** Astro 5.0+ (matches FocalPoint's web-landing)
- **Styling:** Tailwind CSS 4.0+ (Impeccable baseline + custom design tokens)
- **Components:** shadcn/ui (accessible, headless, customizable)
- **Typography:** System sans-serif primary; monospace for code/technical UI

### Deployment
- **Host:** Vercel or Netlify (JAMstack, zero-config, auto-deploys from GitHub)
- **Build:** `astro build` → static HTML + JS
- **CDN:** Auto-edge-cached via Vercel/Netlify
- **SSL:** Auto-managed via Let's Encrypt

### Documentation Federation
- **Docs Link Strategy:** Each product links back to `phenotype.dev/docs/<product>/` which **iframes or redirects** to the product's own docsite (VitePress, Astro, etc.)
- **Search:** Algolia DocSearch across all docs (single search box on phenotype.dev)

---

## Site Structure

### Page Hierarchy

```
phenotype.dev/
├─ /                              # Hero landing, mission, product directory
├─ /products/                     # Product index & comparison
│  ├─ /products/focalpoint/       # FocalPoint marketing page
│  ├─ /products/agileplus/        # AgilePlus marketing page
│  ├─ /products/tracera/          # Tracera marketing page
│  ├─ /products/hwledger/         # hwLedger marketing page
│  └─ ...                          # Other products
├─ /collections/                  # Collections index
│  ├─ /collections/sidekick/      # Sidekick collection hub
│  ├─ /collections/observably/    # Observably collection hub
│  ├─ /collections/vault/         # Vault collection hub
│  ├─ /collections/eidolon/       # Eidolon collection hub
│  └─ /collections/tools/         # Tools collection hub
├─ /docs/                         # Docs federation gateway
│  ├─ /docs/agileplus/            # Redirect or iframe to agileplus.dev/docs
│  ├─ /docs/tracera/              # Redirect or iframe to tracera.dev/docs
│  └─ ...
├─ /community/                    # Community links, Discord, GitHub, etc.
├─ /about/                        # Org mission, team, values
└─ /blog/                         # Cross-product announcements, releases
```

### Landing Page (`/`)

**Sections:**
1. **Hero Banner** (full-width, animated)
   - Org tagline: "Agent-native platform stack for modern development"
   - CTA: "Explore Products" / "View Docs"
   - Visual: Animated product cards carousel or constellation effect

2. **Mission Statement**
   - 2–3 sentences on Phenotype's vision
   - Three core pillars (e.g., Agent-First, Open-Source, Modular)

3. **Collections Grid** (5 cards)
   - **Sidekick** (warm accent color) — Personal productivity & nudging
   - **Observably** (cool accent color) — Observability, planning, traceability
   - **Vault** (neutral accent color) — Data storage & ledgers
   - **Eidolon** (dark-tech accent color) — Device automation & orchestration
   - **Tools** (clean accent color) — Utilities, libraries, infrastructure

   Each card:
   - Collection name + 1-line description
   - 2–3 product logos (FocalPoint, Tracera, hwLedger, etc.)
   - "Explore Collection →" CTA

4. **Featured Products Carousel**
   - 4–6 product cards (FocalPoint, AgilePlus, Tracera, hwLedger)
   - Per-card: product logo, name, 1-line elevator pitch, "Learn More →"

5. **Docs & Community Section**
   - Quick links to docs portal
   - GitHub org link
   - Discord/community channel
   - "Contributing" guide

6. **Footer**
   - Copyright, legal links
   - Product links (phenotype.dev/products/*)
   - Social links
   - Blog archive

---

## Per-Product Marketing Pages

### Page Template: `/products/<product>/`

**Sections:**

1. **Hero Banner**
   - Product logo + name
   - Tagline (1 line, 60 chars max)
   - Problem statement (1–2 sentences)
   - Primary CTA: "Get Started" / "View Docs" / "GitHub"
   - Visual: Screenshot, demo GIF, or animated mockup

2. **Features Grid** (3–4 columns)
   - 6–8 key features
   - Icons + titles + 1-sentence descriptions
   - Example: "Spec-Driven Development" + icon + "Define requirements in plain TOML, trace to code and tests"

3. **Use Cases** (3–4 cards, alternating text/image)
   - E.g., for AgilePlus: "Plan Agent Workflows", "Govern Quality Gates", "Trace Requirements to Tests"
   - Per card: icon, heading, 2–3 sentence description

4. **Architecture Diagram** (optional)
   - Mermaid or SVG showing core components
   - Label key systems (e.g., AgilePlus: spec layer, work packages, FR tracker, agent bridge)

5. **Pricing Section** (if applicable)
   - Free tier, Pro tier, Enterprise tier
   - Feature comparison table
   - CTA: "Start Free" or "Contact Sales"

6. **Integration & Ecosystem**
   - Badges/logos of integrations (GitHub, Plane, Canvas, etc.)
   - "Plays well with X" section

7. **Call-to-Action Footer**
   - "Ready to [use/learn]?" heading
   - Dual CTA: "Get Started" + "Book a Demo"
   - Email signup optional

---

## Collection Hubs

### Page Template: `/collections/<collection>/`

**Sections:**

1. **Collection Hero**
   - Collection name + accent color
   - 2–3 sentence mission statement
   - E.g., Sidekick: "Personal productivity tools with AI nudging and behavior-change support"

2. **Product Cards Grid**
   - All products in the collection
   - Per card: logo, name, 1-line pitch, "Learn More →"
   - Example, Observably: AgilePlus, Tracera, Benchora

3. **Unified Use Case**
   - How products in the collection work together
   - E.g., Observably: "Plan requirements in AgilePlus, trace to Tracera, monitor perf in Benchora"
   - Workflow diagram (Mermaid)

4. **Getting Started**
   - Quick start links
   - Recommended setup order
   - Tutorial or walkthrough

5. **FAQ**
   - "Do I need to use all products in this collection?"
   - "How do they integrate?"
   - "Can I use them with other tools?"

---

## Documentation Federation

### Approach 1: Redirect Pattern (Recommended)
```
phenotype.dev/docs/agileplus → agileplus.dev/docs
phenotype.dev/docs/tracera   → tracera.dev/docs
```

- Simplest to maintain
- Each product owns its docs
- Single source of truth per product

### Approach 2: Subdomain Aliases
```
agileplus-docs.phenotype.dev → agileplus.dev/docs (CNAME)
tracera-docs.phenotype.dev   → tracera.dev/docs (CNAME)
```

- Unified domain experience
- DNS-level routing
- Still points to product repos

### Approach 3: Aggregated Search Only
- phenotype.dev uses **Algolia DocSearch** across all products
- Search box on landing page searches:
  - AgilePlus docs
  - Tracera docs
  - FocalPoint docs
  - hwLedger docs
  - etc.
- Results link to product docs directly

**Recommendation:** Approach 1 + Approach 3 (redirect + unified search).

---

## Design System & Brand Playbook

### Color Palette

**Primary:**
- `primary-50` — #f5f5f5 (off-white)
- `primary-900` — #0a0a0a (deep black)

**Accent Colors (Per Collection):**
- **Sidekick** — Warm orange (#ea580c) + cream (#fff8f3)
- **Observably** — Cool blue (#0066cc) + frost (#f0f4ff)
- **Vault** — Neutral stone (#78716c) + ash (#f6f5f3)
- **Eidolon** — Dark tech purple (#5e21b6) + void (#f3e8ff)
- **Tools** — Clean green (#059669) + mint (#f0fdf4)

**Semantic:**
- Success: `#10b981` (emerald)
- Error: `#ef4444` (red)
- Warning: `#f59e0b` (amber)
- Info: `#3b82f6` (blue)

### Typography

- **Headlines (H1, H2):** System UI sans-serif, bold (700–800), leading-tight
- **Body text:** System UI sans-serif, regular (400), leading-relaxed
- **Code/Technical:** Monospace (Monaco, Fira Code), 0.9rem, bg-slate-100 with border-radius
- **Font stacks:**
  - Sans: `-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif`
  - Mono: `"Monaco", "Menlo", "Ubuntu Mono", monospace`

### Component Patterns

- **Buttons:** shadcn Button (solid, outline, ghost variants)
- **Cards:** Rounded corners (8–12px), subtle shadow, hover lift effect
- **Inputs:** Rounded (6px), border-slate-200, focus ring blue
- **Alerts/Badges:** shadcn Badge + Alert
- **Navigation:** Sticky header, mobile hamburger, active state underline

### Animation & Interaction

- **Hover states:** Subtle color shift + shadow lift (50ms ease-in-out)
- **Page transitions:** Fade + slide-up (200ms)
- **Scroll animations:** Parallax background image, fade-in-on-scroll for sections
- **Interactive:** Collection cards expand on hover to show product list

---

## Federation & Deployment Strategy

### Build Process

```bash
# phenotype-dev-hub/
npm run build  # Generates static assets in dist/
npm run preview  # Local preview of production build
```

Astro handles:
- Markdown to HTML (per .md file in `src/pages/`)
- Component compilation
- CSS/Tailwind tree-shaking
- Image optimization

### Product Links & Cross-Linking

**From phenotype.dev to products:**
```html
<a href="https://agileplus.dev">
  AgilePlus Documentation
</a>
```

**From product sites back to phenotype.dev:**
```html
<a href="https://phenotype.dev">
  Phenotype Organization Hub
</a>
```

### Docs Search (Algolia)

1. Each product repo has Algolia DocSearch config (`.algolia.json`)
2. Crawler indexes product docs nightly
3. phenotype.dev integrates Algolia JavaScript widget
4. Search results show product name prefix (e.g., "AgilePlus: Spec-Driven Development")

### Maintenance Workflow

- **Marketing copy updates:** Edit `/src/pages/products/*.astro`, git push → auto-deploy via Vercel
- **Collection definitions:** Edit `/src/data/collections.json`, update as new products ship
- **Docs links:** Centralized in `/src/data/products.json` (points to product docs)
- **Analytics:** Google Analytics 4 tag in base layout; track signups, doc clicks, product interest

---

## Success Metrics

- **Discoverability:** Organic search traffic to phenotype.dev (target: 100+ monthly unique visitors month 1)
- **Product interest:** Click-through rate from phenotype.dev → product landing (target: 10%+)
- **Docs engagement:** Average session duration on docs portals (target: 3+ min)
- **Community growth:** Signups for newsletter, Discord joins (target: 50+ first month)
- **SEO:** Top 3 for "Phenotype org" + product names (e.g., "AgilePlus spec-driven development")

---

## Scaffold & Deployment Checklist

- [ ] Create `/repos/apps/phenotype-dev-hub/` directory
- [ ] Initialize Astro 5 project with `npm create astro`
- [ ] Add shadcn/ui + Tailwind 4
- [ ] Create base layout with header/footer
- [ ] Build landing page with hero, collections grid, featured products
- [ ] Create `/products/` directory + dynamic page template
- [ ] Create `/collections/` directory + dynamic page template
- [ ] Set up docs redirect/federation (Algolia or iframe)
- [ ] Add analytics (Google Analytics 4)
- [ ] Deploy to Vercel with `phenotype.dev` domain
- [ ] Add product links in product READMEs pointing back to phenotype.dev
- [ ] Set up GitHub Actions for auto-preview (optional)

---

## References

- **FocalPoint web-landing:** `/repos/FocalPoint/apps/web-landing` (Astro 5 + Tailwind 4 baseline)
- **Brand playbook:** `docs/marketing/brand_playbook.md`
- **Consolidated domain map:** `docs/marketing/CONSOLIDATED_DOMAIN_MAP.md`
- **Impeccable design baseline:** `~/.impeccable.md`
