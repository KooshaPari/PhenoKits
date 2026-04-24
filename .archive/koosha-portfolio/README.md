# koosha-portfolio

High-performance personal portfolio website built with Astro, GSAP, and Tailwind CSS. Deployed on Vercel Edge Network.

## Overview

A modern, accessible, and blazing-fast portfolio showcasing projects, skills, and experience. Designed for 100/100 Lighthouse scores and exceptional user experience.

## Architecture

```
Astro 5.x (Zero-JS-by-default)
├── Islands Architecture — Only interactive components hydrate
├── Content Collections — Type-safe Markdown/MDX
└── View Transitions — Native page transitions

GSAP + ScrollTrigger
├── Scroll-driven animations
├── Timeline sequencing
└── 60fps performance

Tailwind CSS
├── Utility-first styling
├── Container queries
└── Dark mode support

Vercel Edge Network
├── <100ms TTFB globally
├── Automatic image optimization
└── Branch-based preview deployments
```

## Documentation

| Document | Purpose | Lines |
|----------|---------|-------|
| [SOTA.md](./SOTA.md) | State of the Art research — framework selection, performance analysis, animation strategies | 1,629 |
| [SPEC.md](./SPEC.md) | Technical specification — architecture, components, data models, deployment | 2,686 |
| [ADR-001](./ADR-001-astro-framework.md) | Astro framework selection rationale | 283 |
| [ADR-002](./ADR-002-gsap-animation.md) | GSAP animation architecture decision | 463 |
| [ADR-003](./ADR-003-vercel-deployment.md) | Vercel deployment strategy decision | 510 |

## Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Performance Targets

| Metric | Target |
|--------|--------|
| First Contentful Paint | < 1.0s |
| Largest Contentful Paint | < 1.5s |
| Time to Interactive | < 2.0s |
| Cumulative Layout Shift | < 0.05 |
| Lighthouse Score | 100/100 |

## Tech Stack

- **Framework**: Astro 5.x
- **Styling**: Tailwind CSS 3.x
- **Animations**: GSAP 3.x with ScrollTrigger
- **Interactivity**: React 18 (islands)
- **Deployment**: Vercel Edge Network
- **Testing**: Vitest, Playwright

## License

MIT © Koosha Paridehpour
