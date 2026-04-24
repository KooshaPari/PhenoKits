# Phenotype Dev Hub — Documentation Federation

This document explains how documentation is federated across all Phenotype product docsites.

## Overview

The Phenotype Dev Hub acts as a central gateway to documentation across all product and collection docsites. Instead of duplicating or embedding docs, we use dynamic routing and intelligent redirects to provide a seamless cross-documentation experience.

## Architecture

### Routing Layer

**File:** `src/pages/docs/[product]/[...slug].astro`

Dynamic Astro route that:
- Accepts a product slug (e.g., `focalpoint`, `agileplus`, `tracera`)
- Accepts optional nested paths (e.g., `/docs/focalpoint/guides/quickstart`)
- Redirects to the canonical docsite URL after 1 second (auto-redirect)
- Falls back to a docs index if JavaScript is disabled

### Product Mappings

Each product is mapped to its canonical docsite URL:

| Product | Docsite URL | Status | Framework |
|---------|-----------|--------|-----------|
| FocalPoint | `https://focalpoint.phenotype.dev/docs` | Stable | Astro |
| AgilePlus | `https://agileplus.phenotype.dev/docs` | Stable | VitePress |
| Tracera | `https://tracera.phenotype.dev/docs` | Beta | VitePress |
| Paginary | `https://paginary.phenotype.dev/docs` | Stable | VitePress |
| Sidekick | `https://sidekick.phenotype.dev/docs` | Beta | — |

### Global Search Integration

A global search bar in the dev-hub header links to each product's search endpoint:

```
https://{product}.phenotype.dev/search
```

Search requests are forwarded to the appropriate product's search implementation (Algolia, local search, etc.).

## User Journey

1. **Entry:** User lands on `/` (dev-hub homepage)
2. **Product Discovery:** Browses collections and products
3. **Docs Entry:** Clicks "Documentation" link for a product
4. **Redirect:** Auto-redirected to `https://{product}.phenotype.dev/docs`
5. **Alternative:** Can browse docs index at `/docs` to see all available docsites

## Implementation Details

### Static Route Generation

```typescript
export function getStaticPaths() {
  return Object.entries(docSites).map(([slug]) => ({
    params: { product: slug, slug: undefined }
  }));
}
```

- **Pre-builds** routes for all known products at build time
- **Reduces latency** by avoiding runtime lookups
- **SEO-friendly** via static site generation

### Graceful Fallback

If JavaScript is disabled:
1. 1-second delay is skipped
2. User sees manual link button: "Visit {Product} Docs"
3. User can click to navigate manually

### Metadata Preservation

Each route passes proper metadata to the layout:

```astro
<Layout 
  title={`${docSite.name} Docs | Phenotype`} 
  description={`${docSite.name} documentation and guides`}
>
```

This ensures correct:
- Open Graph tags
- Meta descriptions
- Canonical URLs (when configured)

## Adding a New Product Docsite

1. **Update docSites mapping** in `src/pages/docs/[product]/[...slug].astro`:

```typescript
const docSites = {
  // ... existing entries
  mynewproduct: {
    name: 'My New Product',
    docsUrl: 'https://mynewproduct.phenotype.dev/docs',
    searchUrl: 'https://mynewproduct.phenotype.dev/search',
    status: 'beta'
  }
};
```

2. **Rebuild** the site:

```bash
npm run build
```

3. **Verify** the new route works:

```
https://phenotype.dev/docs/mynewproduct
https://phenotype.dev/docs/mynewproduct/guides/quickstart
```

## Search Federation

### Global Search Bar

Displayed in the dev-hub header (future enhancement):

```html
<input type="search" placeholder="Search all docs..." />
```

When user submits, route to currently selected product's search:

```typescript
// Route based on active product context
const searchUrl = `${docSite.searchUrl}?q=${encodeURIComponent(query)}`;
```

### Per-Product Search

Each product's docsite maintains its own search index:
- **FocalPoint:** Algolia (if configured)
- **AgilePlus:** VitePress default search
- **Tracera:** VitePress default search
- **Paginary:** VitePress default search
- **Sidekick:** TBD

## Deployment

### Build Process

```bash
# Install dependencies
bun install

# Build static site (pre-renders all docs routes)
bun run build
```

### Hosting Requirements

- **Static hosting:** Vercel, Netlify, GitHub Pages
- **No server-side rendering needed** (fully static)
- **Redirects:** Configured in `vercel.json` (optional, auto-redirect handles it)

### URL Rewriting (Optional)

If hosting on a single domain, configure rewrites in `vercel.json`:

```json
{
  "rewrites": [
    {
      "source": "/docs/focalpoint/:path*",
      "destination": "https://focalpoint.phenotype.dev/docs/:path*"
    }
  ]
}
```

**Note:** Currently using redirect-based approach (simpler, no extra infrastructure).

## Analytics & Monitoring

### Recommended Setup

- **Track redirects:** Monitor `/docs/:product` page views
- **Search funnel:** Track which products users search for
- **Engagement:** Time spent on each docsite (via Google Analytics on target sites)
- **Bounce rate:** Users who click "Visit Docs" button vs. auto-redirect

## Future Enhancements

1. **Embedded Search:** Use Algolia or local search to provide unified search across all docsites
2. **Proxy Mode:** Optional proxy layer to serve docs without redirect (Astro SSR + fetch)
3. **Offline Support:** Cache docs from all products via service worker
4. **Breadcrumb Navigation:** "Phenotype Hub > AgilePlus > Guides > Quickstart"
5. **Cross-Product Search:** "Jump to other product docs"

## Troubleshooting

### Route not generating

- Ensure product slug is lowercase
- Run `bun run build` to rebuild static routes
- Check `src/pages/docs/[product]/[...slug].astro` for typos

### Redirect not working

- Verify target docsite is deployed and accessible
- Check browser console for CORS errors (should be none; we use `target="_blank"`)
- Manually visit docsite URL to ensure it's live

### Search not loading

- Verify `searchUrl` in docSites mapping is correct
- Ensure target product's search endpoint is accessible
- Check network tab for 404s on search requests
