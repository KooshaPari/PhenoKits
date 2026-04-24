# phenotype-dev-hub Deployment Guide

## Local Build Verification

Before deploying, verify the build works locally:

```bash
cd apps/phenotype-dev-hub
bun install
bun run build
bun run preview  # Test locally at http://localhost:3000
```

## Vercel Deployment

### Initial Setup

1. **Connect GitHub Repository**
   - Go to [vercel.com](https://vercel.com)
   - Click "New Project" → Import from Git
   - Select `KooshaPari/phenotype-infrakit`
   - Root Directory: `apps/phenotype-dev-hub`
   - Framework: Astro (auto-detected)
   - Build Command: `bun run build`
   - Output Directory: `dist`

2. **Environment Variables**
   - No environment variables required (static site)
   - Analytics disabled by default (see "Vercel Analytics" below)

3. **Preview & Production Branches**
   - **Preview**: Deploy all pull requests to preview URLs
   - **Production**: Deploy `main` branch to phenotype.dev

### Custom Domain Setup

1. In Vercel project dashboard → Settings → Domains
2. Add domain: `phenotype.dev`
3. Vercel will display DNS configuration (see DNS Config Template)
4. Point domain registrar to Vercel nameservers OR add CNAME/A records

### Auto-Deployments

- **Pull Requests**: GitHub Actions builds & Vercel creates preview URLs
  - Preview URL: `https://<branch-name>.phenotype-dev-hub.vercel.app`
  - Available immediately in PR comments
- **main branch**: Automatically deployed to production (phenotype.dev) on merge

## DNS Configuration

See `dns/cloudflare.md` for:
- A records for phenotype.dev apex
- CNAME records for subdomains (focalpoint.phenotype.dev, etc.)
- TTL recommendations

## Vercel Analytics (Optional)

Analytics is disabled by default. To enable:

```astro
<!-- src/layouts/BaseLayout.astro -->
<Analytics />
```

And add to `astro.config.mjs`:
```javascript
export default defineConfig({
  integrations: [
    vercel({
      analytics: true,
    }),
  ],
});
```

Then redeploy. Analytics dashboard available in Vercel project settings.

## CI/CD Pipeline

See `.github/workflows/deploy.yml`:
- **On PR**: Build artifact validation
- **On main push**: Deploy to Vercel production
- Uses `github.ref == 'refs/heads/main'` to gate production deployments

## Monitoring & Rollback

- Vercel provides automatic preview URLs for all PR commits
- Rollback: Redeploy from Vercel dashboard (no git revert needed)
- Logs: Vercel dashboard → Deployments → Build/Runtime logs

## Troubleshooting

**Build fails locally?**
```bash
bun install --force
bun run build
```

**Build passes locally, fails on Vercel?**
- Check Node version (Vercel uses Node 20.x by default)
- Vercel dashboard → Settings → Node Version: set to 22.x if needed
- Check `bun --version` in CI (may differ from local)

**Domain not resolving?**
- DNS changes take 24-48 hours
- Check propagation: `dig phenotype.dev` or use whatsmydns.net
- Verify records are set in your registrar's DNS control panel

## Status Badge

Add to README.md:
```markdown
[![Deployed on Vercel](https://img.shields.io/badge/deployed-vercel-brightgreen?logo=vercel)](https://phenotype.dev)
```

Rendered:
[![Deployed on Vercel](https://img.shields.io/badge/deployed-vercel-brightgreen?logo=vercel)](https://phenotype.dev)
