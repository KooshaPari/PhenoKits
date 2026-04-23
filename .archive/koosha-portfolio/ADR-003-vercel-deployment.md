# ADR-003 — Vercel Edge Deployment Strategy

**Status:** Accepted  
**Date:** 2026-04-04  
**Deciders:** @koosha  

## Context

koosha-portfolio requires a deployment strategy that achieves:

- Global availability with <100ms TTFB from any location
- Automatic HTTPS and CDN distribution
- Branch-based preview deployments for PR review
- Zero-downtime deployments
- Edge-level image optimization
- Analytics and performance monitoring
- Cost efficiency (free tier acceptable for portfolio scale)

Deployment platforms evaluated: Vercel, Cloudflare Pages, Netlify, AWS S3 + CloudFront, GitHub Pages, and self-hosted alternatives (Coolify, Dokploy).

## Decision

koosha-portfolio deploys to **Vercel Edge Network** as primary hosting, with **Cloudflare Pages** as secondary/DR option.

## Rationale

### Vercel Edge Network

**Global Performance**:
```
TTFB (Time to First Byte) — Static Content:
├── US East: 15-25ms
├── US West: 20-35ms
├── Europe: 25-40ms
├── Asia: 35-60ms
├── Australia: 40-70ms
└── Global average: <50ms (cache hit)
```

**Architecture**:
```
User Request
    ↓
Anycast DNS (Route 53) → Nearest Edge POP (100+ locations)
    ↓
Edge Cache (Vercel Edge Network)
    ├── Cache Hit: Serve immediately (<20ms)
    └── Cache Miss: Fetch from Origin, cache, serve
    ↓
Response (HTTP/2, Brotli compression, optimized headers)
```

### Framework Optimization

Vercel provides first-class support for Astro:

| Feature | Benefit |
|---------|---------|
| **Build Optimization** | Framework-specific build caching |
| **Image API** | Automatic WebP/AVIF, responsive sizes |
| **Edge Config** | Low-latency configuration reads |
| **Analytics** | Web Vitals, Real Experience Score |
| **Speed Insights** | Performance regression detection |

### Deployment Workflow

```
Git Push to main
    ↓
Vercel Git Integration
    ├── Build: `astro build`
    ├── Optimize: Image processing, compression
    ├── Deploy: Atomic to Edge Network
    └── Alias: Production domain
    ↓
Global Availability (<30s from push)
```

**Branch Previews**:
```
Pull Request #42 opened
    ↓
Automatic Preview Deploy
    ├── URL: https://koosha-portfolio-git-feat-xyz.vercel.app
    ├── Comments: Deploy status in PR
    └── Checks: Lighthouse CI, visual regression
```

### Image Optimization

Vercel's Edge Network provides automatic image optimization:

```
Original Request: /images/hero.png
    ↓
Vercel Image Optimization API
    ├── Format: Auto-select WebP/AVIF (browser support)
    ├── Quality: Auto-optimized (85% default)
    ├── Size: Responsive srcset generation
    └── Cache: Edge-cached for 1 year
    ↓
Optimized Response: /_next/image?url=...&w=1200&q=85
```

**Size Comparison**:
| Source | Optimized | Reduction |
|--------|-----------|-------------|
| 1.5MB PNG | 180KB WebP | -88% |
| 800KB JPG | 95KB WebP | -88% |
| 2MB hero.png | 240KB (1200w WebP) | -88% |

### Security & Compliance

**Automatic HTTPS**:
- TLS 1.3 by default
- Let's Encrypt certificates (auto-renewed)
- Custom certificate upload supported

**Security Headers** (configured in `vercel.json`):
```json
{
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        { "key": "X-Frame-Options", "value": "DENY" },
        { "key": "X-Content-Type-Options", "value": "nosniff" },
        { "key": "Referrer-Policy", "value": "strict-origin-when-cross-origin" },
        { "key": "Content-Security-Policy", "value": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri 'self';" }
      ]
    }
  ]
}
```

### Analytics Integration

**Vercel Analytics**:
- Web Vitals tracking (LCP, FID, CLS, INP, TTFB)
- Real User Monitoring (RUM)
- Core Web Vitals distribution histograms
- Performance regression alerts

**Vercel Speed Insights**:
- Lighthouse scores over time
- Performance budgets enforcement
- PR impact analysis

## Consequences

### Positive

- **Performance**: <100ms TTFB globally
- **Developer Experience**: Git push → live in 30s
- **Preview Deploys**: Every PR gets unique URL
- **Zero Config**: Framework detection, automatic optimization
- **Scale**: Handles traffic spikes without configuration
- **Cost**: Hobby tier covers portfolio needs (100GB bandwidth)

### Negative

- **Vendor lock-in**: Vercel-specific features (Image API, Edge Config)
- **Function limits**: 10s execution time (Hobby), 60s (Pro)
- **Cold starts**: Serverless functions have initial latency
- **Bandwidth costs**: Beyond 100GB/month requires Pro ($20/mo)

### Mitigation

- **Secondary hosting**: Cloudflare Pages mirror
- **Static-first**: Astro generates static output, minimizing function needs
- **Asset optimization**: Pre-optimized images reduce bandwidth

## Alternatives Considered

### Cloudflare Pages

**Pros**:
- Unlimited requests/bandwidth on free tier
- 300+ edge locations (vs Vercel's 100+)
- First-class Cloudflare ecosystem integration
- Durable Objects for stateful edge computing

**Cons**:
- Build time limits (20 min free, vs Vercel's 45 min)
- Function runtime (Workers) less mature than Vercel's Node.js
- Image optimization requires separate Images API
- Less framework-optimized than Vercel

**Verdict**: Secondary option — excellent for DR, slightly less DX

### Netlify

**Pros**:
- Pioneer of Git-based deployment
- Form handling built-in
- Split testing capabilities
- Large plugin ecosystem

**Cons**:
- Smaller edge network than Vercel/Cloudflare
- Build times slower
- Image optimization less advanced
- Analytics less comprehensive

**Verdict**: Rejected — Vercel offers superior performance for this use case

### AWS S3 + CloudFront

**Pros**:
- Full infrastructure control
- Massive scale capability
- Cost-effective at high volume
- Integration with AWS ecosystem

**Cons**:
- Complex configuration (CloudFormation/Terraform)
- No Git-based workflow (needs CodePipeline/Actions)
- No preview deployments
- Manual HTTPS certificate management
- No built-in image optimization

**Verdict**: Rejected — overkill for portfolio, poor DX

### GitHub Pages

**Pros**:
- Free for public repos
- Simple Jekyll integration
- Git-native workflow

**Cons**:
- No edge CDN (single origin)
- No automatic image optimization
- No preview deployments
- No serverless functions
- Limited to 1GB storage, 100GB bandwidth

**Verdict**: Rejected — insufficient performance and features

### Self-Hosted (Coolify/Dokploy)

**Pros**:
- Full infrastructure control
- Cost savings at scale
- Data sovereignty
- Learning opportunity

**Cons**:
- Maintenance burden
- No global edge network (single region)
- Manual SSL, backups, monitoring
- No built-in CDN

**Verdict**: Rejected — edge performance requirement unmet

## Implementation

### Project Configuration

```json
// vercel.json
{
  "framework": "astro",
  "buildCommand": "astro build",
  "outputDirectory": "dist",
  "devCommand": "astro dev",
  "installCommand": "npm ci",
  "regions": ["all"],
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        { "key": "X-Frame-Options", "value": "DENY" },
        { "key": "X-Content-Type-Options", "value": "nosniff" },
        { "key": "Referrer-Policy", "value": "strict-origin-when-cross-origin" }
      ]
    },
    {
      "source": "/images/(.*)",
      "headers": [
        { "key": "Cache-Control", "value": "public, max-age=31536000, immutable" }
      ]
    }
  ],
  "rewrites": [
    { "source": "/projects/:slug", "destination": "/projects/[slug].html" }
  ]
}
```

### GitHub Actions Integration

```yaml
# .github/workflows/deploy.yml
name: Deploy to Vercel

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run tests
        run: npm test
      
      - name: Build
        run: npm run build
      
      - name: Deploy to Vercel
        uses: vercel/action-deploy@v1
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
```

### Astro Adapter Configuration

```javascript
// astro.config.mjs
import { defineConfig } from 'astro/config';
import vercel from '@astrojs/vercel/static';

export default defineConfig({
  output: 'static',
  adapter: vercel({
    imageService: true, // Enable Vercel Image Optimization
    webAnalytics: {
      enabled: true, // Vercel Analytics
    },
    speedInsights: {
      enabled: true, // Vercel Speed Insights
    },
  }),
  image: {
    domains: [],
    remotePatterns: [],
  },
});
```

### Image Component with Vercel

```astro
---
// src/components/OptimizedImage.astro
import { Image } from 'astro:assets';
import vercelImage from '@astrojs/vercel';

interface Props {
  src: ImageMetadata;
  alt: string;
  widths?: number[];
}

const { src, alt, widths = [400, 800, 1200] } = Astro.props;
---

<Image
  src={src}
  alt={alt}
  widths={widths}
  sizes="(max-width: 800px) 100vw, 50vw"
  format="webp"
  quality={85}
/>
```

### Environment Variables

```bash
# .env.example (not committed)
VERCEL_ANALYTICS_ID=
PUBLIC_SITE_URL=https://koosha.dev
```

```bash
# Production env vars (set in Vercel dashboard)
VERCEL_ANALYTICS_ID=xxxxx
PUBLIC_SITE_URL=https://koosha.dev
```

## Monitoring & Alerting

### Vercel Analytics Dashboard

**Metrics Tracked**:
| Metric | Target | Alert Threshold |
|--------|--------|-----------------|
| LCP | <2.5s | >3s |
| INP | <200ms | >300ms |
| CLS | <0.1 | >0.15 |
| TTFB | <200ms | >400ms |
| FCP | <1.8s | >2.5s |

### Synthetic Monitoring

```yaml
# .github/workflows/monitoring.yml
name: Uptime Check

on:
  schedule:
    - cron: '*/5 * * * *'  # Every 5 minutes

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - name: Check site availability
        run: |
          curl -f -s -o /dev/null \
            -w "%{http_code} %{time_total}s\n" \
            https://koosha.dev
      
      - name: Check Core Web Vitals
        run: |
          npx lighthouse https://koosha.dev \
            --only-categories=performance \
            --chrome-flags="--headless" \
            --output=json | jq '.categories.performance.score'
```

## Disaster Recovery

### Secondary Deployment (Cloudflare Pages)

```yaml
# .github/workflows/deploy-cloudflare.yml
name: Deploy to Cloudflare Pages (DR)

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npm ci && npm run build
      - name: Deploy to Cloudflare
        uses: cloudflare/pages-action@v1
        with:
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          projectName: koosha-portfolio-dr
          directory: dist
          gitHubToken: ${{ secrets.GITHUB_TOKEN }}
```

### Failover Strategy

```
Primary: koosha.dev (Vercel)
    ↓ (health check failure)
Secondary: koosha-portfolio.pages.dev (Cloudflare)
    ↓ (both failure)
Tertiary: GitHub Pages (gh-pages branch)
```

## Cost Analysis

### Vercel Hobby Tier (Current)

| Resource | Limit | Usage |
|----------|-------|-------|
| Deployments | 100/day | ~5/day |
| Bandwidth | 100GB/month | ~2GB/month |
| Build Time | 6,000 min/month | ~60 min/month |
| Team Members | 1 | 1 |
| **Cost** | **Free** | - |

### Scale Projections

| Traffic Level | Bandwidth | Vercel Plan | Cost |
|---------------|-----------|-------------|------|
| Current | 2GB/mo | Hobby | Free |
| 10x growth | 20GB/mo | Hobby | Free |
| 50x growth | 100GB/mo | Pro | $20/mo |
| 500x growth | 1TB/mo | Pro | $20 + $0.55/GB overage |
| Enterprise | 10TB/mo | Enterprise | Custom |

## References

- [Vercel Documentation](https://vercel.com/docs)
- [Astro Vercel Adapter](https://docs.astro.build/en/guides/integrations-guide/vercel/)
- [Vercel Edge Network](https://vercel.com/docs/concepts/edge-network/overview)
- [Vercel Analytics](https://vercel.com/docs/concepts/analytics)
- [Cloudflare Pages](https://developers.cloudflare.com/pages/)
- [Web Vitals](https://web.dev/vitals/)

---

*Created: 2026-04-04*  
*Last Updated: 2026-04-04*
