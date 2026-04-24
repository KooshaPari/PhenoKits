# Global Search Integration Guide

This document explains how to set up unified search across all Phenotype product docsites.

## Overview

The Phenotype Dev Hub provides a global search bar that:
1. Accepts search queries
2. Routes queries to the selected product's search endpoint
3. Falls back to a unified docs index if no product is selected

## Components

### GlobalSearchBar Component

**File:** `src/components/GlobalSearchBar.astro`

A responsive search component that:
- Displays a search input field
- Includes a product selector dropdown
- Supports keyboard shortcuts (Cmd/Ctrl + K to focus)
- Routes searches to the appropriate product endpoint

### Integration in Layouts

Add the search bar to your main navigation:

```astro
import GlobalSearchBar from '../components/GlobalSearchBar.astro';

<header>
  <nav>
    <!-- ... navigation items ... -->
    <GlobalSearchBar />
  </nav>
</header>
```

## Search Endpoints

Each product maintains its own search endpoint:

| Product | Search URL | Type |
|---------|-----------|------|
| FocalPoint | `https://focalpoint.phenotype.dev/search` | Algolia / Custom |
| AgilePlus | `https://agileplus.phenotype.dev/search` | VitePress default |
| Tracera | `https://tracera.phenotype.dev/search` | VitePress default |
| Paginary | `https://paginary.phenotype.dev/search` | VitePress default |
| Sidekick | `https://sidekick.phenotype.dev/search` | TBD |

### Query Parameters

All search endpoints accept a standard query parameter:

```
GET https://focalpoint.phenotype.dev/search?q=authentication
```

## Implementation Details

### Client-Side Search Routing

When a user submits a search query:

1. **Check product selection:** Is a product selected in the dropdown?
   - If yes: route to `{product}.phenotype.dev/search?q={query}`
   - If no: route to local `/docs?q={query}`

2. **Form submission:**
   ```typescript
   searchForm.addEventListener('submit', (e) => {
     e.preventDefault();
     const query = searchInput.value;
     const product = searchProduct.value;
     
     if (product) {
       const url = `https://${product}.phenotype.dev/search?q=${encodeURIComponent(query)}`;
       window.location.href = url;
     } else {
       const url = `/docs?q=${encodeURIComponent(query)}`;
       window.location.href = url;
     }
   });
   ```

### Keyboard Shortcuts

- **Cmd/Ctrl + K:** Focus search input
- **Escape:** Blur search input (when focused)

## Adding Search to a New Product

### Step 1: Implement Search Endpoint

Create a search endpoint in your product's docsite:

```astro
// src/pages/search.astro
---
const query = new URL(Astro.request.url).searchParams.get('q') || '';
const results = query ? search(query) : [];
---

<html>
  <head>
    <title>Search: {query}</title>
  </head>
  <body>
    <h1>Search Results</h1>
    {results.map(result => (
      <div>
        <a href={result.url}>{result.title}</a>
        <p>{result.excerpt}</p>
      </div>
    ))}
  </body>
</html>
```

### Step 2: Register in Dev Hub

Update `src/components/GlobalSearchBar.astro`:

```typescript
const products = [
  // ... existing products
  { slug: 'mynewproduct', name: 'My New Product' }
];
```

Update search endpoints mapping:

```typescript
const searchEndpoints = {
  // ... existing
  mynewproduct: 'https://mynewproduct.phenotype.dev/search'
};
```

### Step 3: Test

Navigate to:
```
https://phenotype.dev/docs/mynewproduct
```

Search for a term and verify the search works.

## Search Technologies

### VitePress Search

VitePress includes a built-in search feature (requires `search: true` in config):

```typescript
// vitepress.config.ts
export default {
  themeConfig: {
    search: {
      provider: 'local'
    }
  }
};
```

Endpoint: `https://product.phenotype.dev/search`

### Algolia Search

For more advanced search, consider Algolia:

1. Create Algolia account and index
2. Configure DocSearch in product docsite:
   ```typescript
   docsearch({
     appId: 'YOUR_APP_ID',
     apiKey: 'YOUR_API_KEY',
     indexName: 'YOUR_INDEX'
   });
   ```

3. Create search page that uses Algolia API:
   ```typescript
   const client = algoliasearch(appId, apiKey);
   const results = await client.search(query);
   ```

### Custom Search

Implement custom search indexing:

1. Generate search index at build time:
   ```typescript
   // build.ts
   const index = buildSearchIndex(allDocs);
   fs.writeFileSync('search-index.json', JSON.stringify(index));
   ```

2. Load index client-side and search:
   ```typescript
   const index = await fetch('/search-index.json').then(r => r.json());
   const results = index.filter(doc => doc.title.includes(query));
   ```

## Analytics

Track search behavior to understand user needs:

```typescript
// In GlobalSearchBar.astro
searchForm.addEventListener('submit', (e) => {
  const query = searchInput.value;
  const product = searchProduct.value || 'all';

  // Send to analytics
  gtag('event', 'search', {
    search_term: query,
    product: product
  });

  // Continue with search routing...
});
```

### Key Metrics

- **Most searched terms:** What are users looking for?
- **Products with highest search volume:** Which products need better docs?
- **Search → result clicks:** Are results helpful?
- **Bounce rate after search:** Is search quality high?

## Future Enhancements

1. **Unified Search Index:** Aggregate all product docs into a single search index (Algolia, Meilisearch, ElasticSearch)
2. **Cross-Product Search:** Search returns results from all products simultaneously
3. **Smart Suggestions:** Auto-complete suggestions based on popular searches
4. **Search Analytics Dashboard:** Aggregate search metrics across all products
5. **Faceted Search:** Filter by product, content type, date, etc.

## Troubleshooting

### Search not working

1. **Check endpoint is live:**
   ```bash
   curl https://product.phenotype.dev/search?q=test
   ```

2. **Verify query parameter:** Ensure you're using `?q=` (not `?query=` or other variants)

3. **CORS issues:** Search endpoints may need CORS headers if using fetch instead of redirects

4. **JavaScript disabled:** Search falls back to link if JS is disabled

### No search results

- Verify product's search index is populated and deployed
- Check search query is URL-encoded properly
- Ensure docs are indexed (check product's indexing pipeline)

### Slow search performance

- Optimize search index size (compress, lazy-load)
- Add caching headers to search responses
- Consider pagination for large result sets
