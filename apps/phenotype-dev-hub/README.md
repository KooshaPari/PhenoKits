# phenotype-dev-hub

[![Deployed on Vercel](https://img.shields.io/badge/deployed-vercel-brightgreen?logo=vercel)](https://phenotype.dev)
[![Build Status](https://github.com/KooshaPari/phenotype-infrakit/actions/workflows/deploy.yml/badge.svg?branch=main)](https://github.com/KooshaPari/phenotype-infrakit/actions/workflows/deploy.yml)

Phenotype organization marketing landing page and product catalog. Built with Astro 5 + Tailwind CSS 4 + shadcn/ui.

**Status:** Deployment-ready (build & CI configured)

---

## Overview

`phenotype-dev-hub` is the unified entry point for the Phenotype ecosystem:
- Org-level landing page
- Product directory with marketing pages
- Collection hubs (Sidekick, Observably, Vault, Eidolon, Tools)
- Federated documentation gateway
- Unified search (Algolia DocSearch)

**Target Domain:** phenotype.dev

---

## Architecture

```
apps/phenotype-dev-hub/
├── src/
│   ├── pages/
│   │   ├── index.astro              # Landing page (/)
│   │   ├── products/
│   │   │   ├── index.astro          # Products directory
│   │   │   ├── [product].astro      # Dynamic product pages
│   │   │   └── agileplus.astro      # AgilePlus marketing (example)
│   │   ├── collections/
│   │   │   ├── index.astro          # Collections index
│   │   │   └── [collection].astro   # Dynamic collection pages
│   │   ├── docs/
│   │   │   └── [...slug].astro      # Docs federation (redirects)
│   │   ├── community.astro          # Community links
│   │   ├── about.astro              # About Phenotype org
│   │   └── blog/                    # Org announcements & tutorials
│   ├── layouts/
│   │   └── BaseLayout.astro         # Global header/footer
│   ├── components/
│   │   ├── ProductCard.astro
│   │   ├── CollectionCard.astro
│   │   ├── Button.astro
│   │   └── (shadcn components)
│   ├── data/
│   │   ├── products.json            # Product metadata
│   │   ├── collections.json         # Collection definitions
│   │   └── navigation.json          # Nav structure
│   └── styles/
│       ├── globals.css              # Impeccable baseline
│       └── tokens.css               # Design tokens (colors, typography)
├── astro.config.mjs                 # Astro configuration
├── tailwind.config.mjs              # Tailwind 4 config
├── package.json                     # Dependencies
└── README.md                         # This file
```

---

## Setup

### Prerequisites
- Node.js 18+ (or Bun 1.1+)
- Git

### Installation

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/apps/phenotype-dev-hub

# Install dependencies
npm install
# or: bun install

# Start dev server
npm run dev
# or: bun run dev

# Build for production
npm run build
# or: bun run build

# Preview production build
npm run preview
# or: bun run preview
```

Dev server runs on `http://localhost:3000` by default.

---

## File Structure & Conventions

### Pages

Pages are Astro `.astro` files in `src/pages/`. Each page is a route:
- `src/pages/index.astro` → `/`
- `src/pages/products/index.astro` → `/products/`
- `src/pages/products/[product].astro` → `/products/:product` (dynamic)

### Components

Reusable components in `src/components/`:
- `ProductCard.astro` — Product listing card
- `CollectionCard.astro` — Collection hub card
- `Button.astro` — Button primitive (wraps shadcn Button)
- shadcn components imported as needed

### Data Files

Static data in `src/data/`:
```json
// products.json
[
  {
    "id": "agileplus",
    "name": "AgilePlus",
    "tagline": "Spec-driven development engine",
    "domain": "agileplus.dev",
    "collection": "observably",
    "description": "...",
    "features": ["Specs", "Traceability", "Governance"],
    "cta": "Learn More",
    "ctaUrl": "https://agileplus.dev"
  }
]
```

### Styles

- **Globals:** `src/styles/globals.css` includes Impeccable baseline
- **Tokens:** `src/styles/tokens.css` defines CSS custom properties (colors, spacing, fonts)
- **Tailwind:** `tailwind.config.mjs` extends tokens and adds custom utilities

---

## Design System Integration

### Colors

Collection accent colors defined in `tailwind.config.mjs`:
```javascript
theme: {
  extend: {
    colors: {
      sidekick: {
        orange: '#ea580c',
        light: '#fed7aa',
        dark: '#92400e',
      },
      observably: {
        blue: '#0066cc',
        light: '#cce5ff',
        dark: '#003d82',
      },
      // ... other collections
    },
  },
},
```

Use in components:
```astro
<div class="bg-sidekick-orange text-white">Sidekick</div>
<div class="border-l-4 border-observably-blue">Content</div>
```

### Typography

System font stack in `globals.css`:
```css
body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
}

code, pre {
  font-family: "Monaco", "Menlo", "Ubuntu Mono", "Courier New", monospace;
}
```

---

## Key Pages to Implement

### Landing Page (`src/pages/index.astro`)

- [ ] Hero banner with org tagline
- [ ] Mission statement (2–3 sentences)
- [ ] Collections grid (5 cards: Sidekick, Observably, Vault, Eidolon, Tools)
- [ ] Featured products carousel (FocalPoint, AgilePlus, Tracera, hwLedger)
- [ ] Docs & community CTA section
- [ ] Footer with product links, social links

### Products Directory (`src/pages/products/index.astro`)

- [ ] All products in grid layout
- [ ] Search/filter by collection
- [ ] CTA to each product domain

### Dynamic Product Page (`src/pages/products/[product].astro`)

- [ ] Hero + tagline + problem statement
- [ ] Features grid (6–8 key features)
- [ ] Use cases section (3–4 cards)
- [ ] Architecture diagram (if applicable)
- [ ] Integrations & ecosystem
- [ ] Pricing section (if applicable)
- [ ] CTA footer ("Get Started" + "Docs")

### Collections Hub (`src/pages/collections/[collection].astro`)

- [ ] Collection hero (name, mission, accent color)
- [ ] Product cards grid (all products in collection)
- [ ] Unified use case workflow
- [ ] Getting started guide
- [ ] FAQ

### Docs Federation (`src/pages/docs/[...slug].astro`)

- [ ] Redirect to product docsite or Algolia search
- [ ] Or: iframe-based docs portal (unified search)
- [ ] Breadcrumb navigation

---

## Deployment

### Vercel (Recommended)

1. **Connect GitHub repository:**
   ```bash
   cd /repos/apps/phenotype-dev-hub
   git init && git add . && git commit -m "Initial phenotype-dev-hub scaffold"
   git push origin main
   ```

2. **Import to Vercel:**
   - Go to vercel.com/dashboard
   - Click "Add New..." → "Project"
   - Import repository
   - Framework: Astro (auto-detected)
   - Vercel auto-configures build (astro build) and deploy

3. **Domain Configuration:**
   - Vercel dashboard → Settings → Domains
   - Add `phenotype.dev`
   - Update DNS registrar with Vercel nameservers or CNAME

### Environment Variables

Create `.env.local`:
```bash
PUBLIC_SITE_URL=https://phenotype.dev
PUBLIC_ANALYTICS_ID=G-XXXXXXXXXX  # Google Analytics 4 ID
ALGOLIA_APP_ID=XXXXXXXXXX
ALGOLIA_SEARCH_API_KEY=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

---

## Development Workflow

### Adding a New Product

1. Update `src/data/products.json`:
   ```json
   {
     "id": "newproduct",
     "name": "New Product",
     "collection": "observably",
     ...
   }
   ```

2. Create `src/pages/products/newproduct.astro` OR rely on dynamic `[product].astro`

3. Test at `http://localhost:3000/products/newproduct`

### Adding a New Collection

1. Update `src/data/collections.json`:
   ```json
   {
     "id": "newcollection",
     "name": "New Collection",
     "color": "#xxxxx",
     ...
   }
   ```

2. Create `src/pages/collections/newcollection.astro`

3. Test at `http://localhost:3000/collections/newcollection`

### Running Quality Checks

```bash
# Type check
npm run type-check

# Build validation
npm run build

# Local preview
npm run preview
```

---

## Marketing & SEO

### Sitemap
Astro auto-generates `dist/sitemap.xml` on build. Submit to Google Search Console.

### Analytics
Google Analytics 4 snippet in `BaseLayout.astro`:
```html
<script async src="https://www.googletagmanager.com/gtag/js?id={PUBLIC_ANALYTICS_ID}"></script>
```

### Meta Tags
Set per-page in each `.astro` file:
```astro
---
const title = "AgilePlus | Phenotype";
const description = "Spec-driven development engine...";
---

<Layout {title} {description}>
  ...
</Layout>
```

---

## Troubleshooting

### Build Fails
```bash
npm run check  # Type check
npm install    # Ensure deps installed
npm run build  # Full build with debug
```

### Dev Server Won't Start
```bash
# Kill any existing process on port 3000
lsof -i :3000 | grep node | awk '{print $2}' | xargs kill -9

# Restart
npm run dev
```

---

## Deployment

**Deployment Status:** Ready for production

### Quick Deploy
```bash
# Verify build locally
bun run build
bun run preview

# Push to main → auto-deploys to https://phenotype.dev
git push origin main
```

### Deployment Guide
See **DEPLOY.md** for:
- Vercel setup instructions
- Custom domain configuration (phenotype.dev)
- GitHub Actions CI/CD pipeline
- Preview branch deployments
- DNS troubleshooting

### DNS Configuration
See **dns/cloudflare.md** for:
- Vercel nameserver setup
- A/CNAME records (if using manual DNS)
- Subdomain routing (focalpoint.phenotype.dev, etc.)
- SSL/TLS certificate management

---

## References

- **Deployment Guide:** DEPLOY.md
- **DNS Configuration:** dns/cloudflare.md
- **CI/CD Pipeline:** .github/workflows/deploy.yml
- **Marketing Hub Design:** ../../docs/marketing/phenotype_dev_hub_design.md
- **Brand Playbook:** ../../docs/marketing/brand_playbook.md
- **Domain Map:** ../../CONSOLIDATED_DOMAIN_MAP.md
- **Astro Docs:** https://docs.astro.build
- **Tailwind CSS:** https://tailwindcss.com
- **shadcn/ui:** https://ui.shadcn.com

---

## Contributing

All changes must follow the Phenotype brand playbook (colors, typography, tone).

Submit PRs with:
- [ ] Changes to marketing copy reviewed for tone/clarity
- [ ] New assets follow design system (colors, spacing, fonts)
- [ ] Tested locally (`npm run build` + preview)
- [ ] Algolia metadata updated if adding products/collections

---

## License

Phenotype Org — MIT License (matching parent org)

---

**Next Steps:**
1. Run `npm install` to set up dependencies
2. Run `npm run dev` to start dev server
3. Implement landing page hero + sections
4. Add product/collection templates
5. Deploy to Vercel with phenotype.dev domain
