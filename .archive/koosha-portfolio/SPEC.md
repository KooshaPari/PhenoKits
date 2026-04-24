# SPEC.md — koosha-portfolio Technical Specification

**Version**: 2.0  
**Date**: 2026-04-04  
**Status**: Draft → Implementation Ready  
**Author**: @koosha  

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Overview](#architecture-overview)
3. [System Architecture Diagrams](#system-architecture-diagrams)
4. [Component Specifications](#component-specifications)
5. [Data Models](#data-models)
6. [Content Schema](#content-schema)
7. [Performance Budgets](#performance-budgets)
8. [Animation System](#animation-system)
9. [Testing Strategy](#testing-strategy)
10. [Build Pipeline](#build-pipeline)
11. [Deployment Procedures](#deployment-procedures)
12. [SEO & Meta Strategy](#seo--meta-strategy)
13. [Accessibility Requirements](#accessibility-requirements)
14. [Security Considerations](#security-considerations)
15. [Monitoring & Analytics](#monitoring--analytics)
16. [Development Workflow](#development-workflow)
17. [File Organization](#file-organization)
18. [Dependencies & Versions](#dependencies--versions)
19. [Appendices](#appendices)

---

## Executive Summary

koosha-portfolio is a high-performance personal portfolio website designed to showcase professional projects, skills, and experience. Built with modern web technologies and optimized for Core Web Vitals, the site represents the pinnacle of static site engineering.

### Key Characteristics

| Attribute | Specification |
|-----------|---------------|
| **Framework** | Astro 5.x with Islands Architecture |
| **Styling** | Tailwind CSS 3.x with custom design tokens |
| **Animations** | GSAP 3.x + ScrollTrigger for scroll-driven effects |
| **Deployment** | Vercel Edge Network (primary), Cloudflare Pages (DR) |
| **Performance Target** | 100/100 Lighthouse, <1s FCP on 4G |
| **Bundle Size** | <50KB JavaScript (hydrated islands only) |
| **Accessibility** | WCAG 2.1 AA compliant |

### Differentiators

1. **Zero-JS-by-Default**: Static content ships without JavaScript
2. **Type-Safe Content**: Zod-validated content collections
3. **Scroll-Driven Animations**: 60fps GSAP animations tied to scroll position
4. **Edge-Optimized**: <100ms TTFB globally via Vercel Edge Network
5. **View Transitions**: Native page transitions without framework overhead

---

## Architecture Overview

### Design Philosophy

The architecture follows three core principles:

1. **Content First**: The portfolio exists to present content; technology serves content, not vice versa
2. **Performance as Design**: Fast loading is a feature; slow sites lose visitors
3. **Progressive Enhancement**: Core functionality works without JavaScript; JS enhances

### Technology Stack

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           PRESENTATION LAYER                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Astro     │  │   React     │  │    GSAP     │  │  Tailwind   │         │
│  │   5.x       │  │  (islands)  │  │  (client)   │  │    CSS      │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                │                │                │                │
│         └────────────────┴────────────────┴────────────────┘                │
│                              ↓                                               │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      CONTENT LAYER                                    │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │    │
│  │  │  Markdown   │  │    MDX      │  │    YAML     │  │    JSON     │ │    │
│  │  │  (content)  │  │  (content)  │  │   (data)    │  │   (data)    │ │    │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │    │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │    │
│  │  │                    Content Collections                          │ │    │
│  │  │              (Zod-validated, Type-safe)                         │ │    │
│  │  └─────────────────────────────────────────────────────────────────┘ │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                              ↓                                               │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                       BUILD LAYER                                   │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │    │
│  │  │   Vite      │  │   Sharp     │  │   PurgeCSS  │  │   Brotli    │ │    │
│  │  │  (bundler)  │  │  (images)   │  │  (unused    │  │ (compress)  │ │    │
│  │  └─────────────┘  └─────────────┘  │   styles)   │  └─────────────┘ │    │
│  │                                     └─────────────┘                  │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                              ↓                                               │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                     DEPLOYMENT LAYER                                │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                 │    │
│  │  │   Vercel    │  │   Edge      │  │   Image     │                 │    │
│  │  │   (host)    │  │   Network   │  │   Opt API   │                 │    │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                 │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## System Architecture Diagrams

### High-Level System Flow

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                              USER REQUEST                                       │
│                    (Browser, Mobile, Tablet, Desktop)                           │
└─────────────────────────────────┬──────────────────────────────────────────────┘
                                  │
                                  ▼
┌────────────────────────────────────────────────────────────────────────────────┐
│                            EDGE NETWORK                                         │
│                    (Vercel Edge, 100+ locations)                               │
│  ┌──────────────────────────────────────────────────────────────────────────┐  │
│  │  1. DNS Resolution (Anycast)                                             │  │
│  │     └── Route to nearest POP (Point of Presence)                       │  │
│  │                                                                          │  │
│  │  2. Edge Cache Check                                                     │  │
│  │     ├── Cache HIT: Serve immediately (<20ms)                           │  │
│  │     └── Cache MISS: Fetch from Origin                                    │  │
│  │                                                                          │  │
│  │  3. Compression (Brotli/gzip)                                            │  │
│  │  4. HTTP/2 Push (critical assets)                                        │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────┬──────────────────────────────────────────────┘
                                  │
                                  ▼
┌────────────────────────────────────────────────────────────────────────────────┐
│                         RESPONSE TO BROWSER                                     │
│                                                                                 │
│  ┌──────────────────────────────────────────────────────────────────────────┐   │
│  │  Response Headers:                                                        │   │
│  │  ├── content-type: text/html; charset=utf-8                                │   │
│  │  ├── cache-control: public, max-age=0, must-revalidate                     │   │
│  │  ├── x-content-type-options: nosniff                                       │   │
│  │  ├── x-frame-options: DENY                                               │   │
│  │  ├── referrer-policy: strict-origin-when-cross-origin                      │   │
│  │  └── content-security-policy: default-src 'self'...                        │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  ┌──────────────────────────────────────────────────────────────────────────┐   │
│  │  Document Structure:                                                      │   │
│  │  ├── <html> (pre-rendered HTML)                                          │   │
│  │  ├── <head> (critical CSS inlined)                                       │   │
│  │  ├── <body> (content, mostly static)                                     │   │
│  │  │   ├── Hero Section (static)                                           │   │
│  │  │   ├── Project Grid (static)                                           │   │
│  │  │   └── Interactive Islands (hydrated):                                 │   │
│  │  │       ├── Theme Toggle (client:load)                                  │   │
│  │  │       ├── Contact Form (client:idle)                                   │   │
│  │  │       └── GSAP Animations (client:visible)                            │   │
│  │  └── <script> (GSAP, minimal JS)                                          │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────┬──────────────────────────────────────────────┘
                                  │
                                  ▼
┌────────────────────────────────────────────────────────────────────────────────┐
│                         CLIENT-SIDE EXECUTION                                   │
│                                                                                 │
│  ┌──────────────────────────────────────────────────────────────────────────┐   │
│  │  Phase 1: Critical Rendering Path                                         │   │
│  │  ├── Parse HTML (streaming)                                              │   │
│  │  ├── Build DOM                                                           │   │
│  │  ├── Parse CSS (non-blocking)                                            │   │
│  │  └── First Paint (FCP target: <1s)                                        │   │
│  │                                                                          │   │
│  │  Phase 2: Hydration (selective)                                           │   │
│  │  ├── Astro hydrates interactive islands                                  │   │
│  │  ├── GSAP initializes ScrollTrigger                                      │   │
│  │  └── Event listeners attached                                            │   │
│  │                                                                          │   │
│  │  Phase 3: Interaction                                                     │   │
│  │  ├── User scrolls → ScrollTrigger animations                             │   │
│  │  ├── User clicks → Island components respond                             │   │
│  │  └── Page navigation → View Transitions (if enabled)                     │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Component Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           COMPONENT ARCHITECTURE                                 │
│                                                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                           PAGE LAYER                                       │ │
│  │                                                                              │ │
│  │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │ │
│  │   │   index.     │  │   about.     │  │  projects/   │  │  contact.    │   │ │
│  │   │   astro      │  │   astro      │  │  [slug].     │  │  astro       │   │ │
│  │   │              │  │              │  │  astro       │  │              │   │ │
│  │   │  (Landing)   │  │  (Bio/Exp)   │  │  (Dynamic)   │  │  (Form)      │   │ │
│  │   └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘   │ │
│  │          │                 │                 │                 │          │ │
│  └──────────┼─────────────────┼─────────────────┼─────────────────┼──────────┘ │
│             │                 │                 │                 │            │
│  ┌──────────┴─────────────────┴─────────────────┴─────────────────┴──────────┐ │
│  │                        LAYOUT LAYER                                        │ │
│  │                                                                              │ │
│  │   ┌─────────────────────────────────────────────────────────────────────┐  │ │
│  │   │                         BaseLayout.astro                            │  │ │
│  │   │  ┌───────────────────────────────────────────────────────────────┐  │  │ │
│  │   │  │                         <head>                               │  │  │ │
│  │   │  │  ├── Meta tags (SEO, social)                                  │  │  │ │
│  │   │  │  ├── Preload critical assets                                  │  │  │ │
│  │   │  │  └── Critical CSS (inlined)                                   │  │  │ │
│  │   │  └───────────────────────────────────────────────────────────────┘  │  │ │
│  │   │  ┌───────────────────────────────────────────────────────────────┐  │  │ │
│  │   │  │                         <body>                               │  │  │ │
│  │   │  │  ├── Header.astro (static navigation)                       │  │  │ │
│  │   │  │  ├── <slot /> (page content)                              │  │  │ │
│  │   │  │  └── Footer.astro (static)                                  │  │  │ │
│  │   │  └───────────────────────────────────────────────────────────────┘  │  │ │
│  │   └─────────────────────────────────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                       COMPONENT LAYER                                        │ │
│  │                                                                              │ │
│  │   ┌─────────────────────────────────────────────────────────────────────┐   │ │
│  │   │                    STATIC COMPONENTS (.astro)                        │   │ │
│  │   │                                                                      │   │ │
│  │   │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │   │ │
│  │   │   │   Hero.      │  │   Project    │  │   Skills.    │             │   │ │
│  │   │   │   astro      │  │   Card.      │  │   Matrix.    │             │   │ │
│  │   │   │              │  │   astro      │  │   astro      │             │   │ │
│  │   │   │  (Static     │  │  (Static     │  │  (Static     │             │   │ │
│  │   │   │   content)   │  │   display)   │  │   grid)      │             │   │ │
│  │   │   └──────────────┘  └──────────────┘  └──────────────┘             │   │ │
│  │   │                                                                      │   │ │
│  │   │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │   │ │
│  │   │   │  Timeline.   │  │   Bio.       │  │   Stats.     │             │   │ │
│  │   │   │  astro       │  │   astro      │  │   astro      │             │   │ │
│  │   │   └──────────────┘  └──────────────┘  └──────────────┘             │   │ │
│  │   └─────────────────────────────────────────────────────────────────────┘   │ │
│  │                                                                              │ │
│  │   ┌─────────────────────────────────────────────────────────────────────┐   │ │
│  │   │                 INTERACTIVE ISLANDS (framework)                    │   │ │
│  │   │                                                                      │   │ │
│  │   │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │   │ │
│  │   │   │ ThemeToggle. │  │  Contact     │  │  Project     │             │   │ │
│  │   │   │   (React)    │  │  Form        │  │  Filter      │             │   │ │
│  │   │   │  client:load │  │  (React)     │  │  (React)     │             │   │ │
│  │   │   └──────────────┘  └──────────────┘  └──────────────┘             │   │ │
│  │   │                                                                      │   │ │
│  │   │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │   │ │
│  │   │   │   GSAP       │  │   Scroll     │  │   Mobile     │             │   │ │
│  │   │   │   Wrapper    │  │   Progress   │  │   Menu       │             │   │ │
│  │   │   │  (client:    │  │   (React)    │  │   (React)    │             │   │ │
│  │   │   │   visible)   │  │  client:idle │  │  client:load │             │   │ │
│  │   │   └──────────────┘  └──────────────┘  └──────────────┘             │   │ │
│  │   └─────────────────────────────────────────────────────────────────────┘   │ │
│  └─────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                         CONTENT LAYER                                        │ │
│  │                                                                              │ │
│  │   ┌─────────────────────────────────────────────────────────────────────┐     │ │
│  │   │                    Content Collections                             │     │ │
│  │   │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │     │ │
│  │   │  │  projects/   │  │   skills/    │  │ experience/  │             │     │ │
│  │   │  │  └── *.md    │  │  └── *.md    │  │  └── *.md    │             │     │ │
│  │   │  │  (Zod        │  │  (Zod        │  │  (Zod        │             │     │ │
│  │   │  │   validated) │  │   validated) │  │   validated) │             │     │ │
│  │   │  └──────────────┘  └──────────────┘  └──────────────┘             │     │ │
│  │   └─────────────────────────────────────────────────────────────────────┘     │ │
│  │                                                                              │ │
│  │   ┌─────────────────────────────────────────────────────────────────────┐     │ │
│  │   │                    Data Files                                      │     │ │
│  │   │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │     │ │
│  │   │  │  site.json   │  │  social.json │  │  config.json │             │     │ │
│  │   │  │  (metadata)  │  │  (links)     │  │  (settings)  │             │     │ │
│  │   │  └──────────────┘  └──────────────┘  └──────────────┘             │     │ │
│  │   └─────────────────────────────────────────────────────────────────────┘     │ │
│  └─────────────────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────────────────┘
```

### Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                          DATA FLOW ARCHITECTURE                                  │
│                                                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                       BUILD TIME (Static Generation)                       │ │
│  │                                                                              │ │
│  │   Source Content                          Build Process                     │ │
│  │   ┌──────────────┐                      ┌──────────────────────────────┐    │ │
│  │   │  Markdown    │                      │  1. Content Collections      │    │ │
│  │   │  Files       │─────────────────────▶│     └── Zod validation      │    │ │
│  │   │  (*.md)      │                      │     └── Type generation     │    │ │
│  │   └──────────────┘                      └──────────────┬───────────────┘    │ │
│  │   ┌──────────────┐                                     │                    │ │
│  │   │  YAML/JSON   │                                     ▼                    │ │
│  │   │  Data        │─────────────────────▶┌──────────────────────────────┐     │ │
│  │   │  Files       │                      │  2. Astro Template Rendering │    │ │
│  │   └──────────────┘                      │     └── Server-side render  │    │ │
│  │   ┌──────────────┐                      │     └── Component islands   │    │ │
│  │   │  Images      │                      │     └── Asset optimization  │    │ │
│  │   │  (*.png,     │─────────────────────▶│     └── CSS purging         │    │ │
│  │   │   *.jpg)     │                      └──────────────┬───────────────┘    │ │
│  │   └──────────────┘                                     │                    │ │
│  │                                                        ▼                    │ │
│  │                                         ┌──────────────────────────────┐     │ │
│  │                                         │  3. Static Output              │     │ │
│  │                                         │     ├── HTML pages            │     │ │
│  │                                         │     ├── CSS bundle            │     │ │
│  │                                         │     ├── JS chunks (split)     │     │ │
│  │                                         │     └── Optimized images      │     │ │
│  │                                         └──────────────────────────────┘     │ │
│  └─────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                       RUNTIME (Client-side)                                │ │
│  │                                                                              │ │
│  │   Static Content                          Hydration                         │ │
│  │   ┌──────────────┐                      ┌──────────────────────────────┐    │ │
│  │   │  HTML/CSS    │                      │  1. Astro Islands Bootstrap  │    │ │
│  │   │  (cached)    │─────────────────────▶│     └── Scan for client:*   │    │ │
│  │   └──────────────┘                      │     └── Hydrate components  │    │ │
│  │   ┌──────────────┐                      └──────────────┬───────────────┘    │ │
│  │   │  JS Islands  │                                     │                    │ │
│  │   │  (lazy)      │                                     ▼                    │ │
│  │   └──────────────┘                      ┌──────────────────────────────┐     │ │
│  │                                         │  2. GSAP Initialization      │     │ │
│  │   ┌──────────────┐                      │     └── ScrollTrigger setup │     │ │
│  │   │  User        │                      │     └── Animation timelines │     │ │
│  │   │  Events      │─────────────────────▶│     └── Resize observers    │     │ │
│  │   │  (scroll,    │                      └──────────────┬───────────────┘     │ │
│  │   │   click)     │                                     │                    │ │
│  │   └──────────────┘                                     ▼                    │ │
│  │                                         ┌──────────────────────────────┐     │ │
│  │                                         │  3. Interaction State          │     │ │
│  │                                         │     ├── Theme (dark/light)    │     │ │
│  │                                         │     ├── Form state            │     │ │
│  │                                         │     └── Animation progress    │     │ │
│  │                                         └──────────────────────────────┘     │ │
│  └─────────────────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────────────────┘
```

---

## Component Specifications

### 1. Hero Component

**Purpose**: First impression, value proposition, and primary CTA

**Props Interface**:
```typescript
interface HeroProps {
  title: string;
  subtitle: string;
  ctaText: string;
  ctaHref: string;
  backgroundImage?: ImageMetadata;
  socialLinks: SocialLink[];
}

interface SocialLink {
  platform: 'github' | 'linkedin' | 'twitter' | 'email';
  url: string;
  icon: string;
}
```

**Implementation**:
```astro
---
// src/components/Hero.astro
import { Image } from 'astro:assets';
import SocialLinks from './SocialLinks.astro';

interface Props {
  title: string;
  subtitle: string;
  ctaText: string;
  ctaHref: string;
  backgroundImage?: ImageMetadata;
}

const { title, subtitle, ctaText, ctaHref, backgroundImage } = Astro.props;
---

<section class="hero relative min-h-screen flex items-center" data-gsap="hero">
  {backgroundImage && (
    <Image
      src={backgroundImage}
      alt=""
      class="absolute inset-0 w-full h-full object-cover parallax-bg"
      loading="eager"
      fetchpriority="high"
    />
  )}
  
  <div class="relative z-10 container mx-auto px-6">
    <h1 
      class="hero-title text-5xl md:text-7xl lg:text-8xl font-bold text-white mb-6"
      data-gsap="title"
    >
      {title}
    </h1>
    
    <p 
      class="hero-subtitle text-xl md:text-2xl text-gray-300 mb-8 max-w-2xl"
      data-gsap="subtitle"
    >
      {subtitle}
    </p>
    
    <a 
      href={ctaHref}
      class="hero-cta inline-block px-8 py-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
      data-gsap="cta"
    >
      {ctaText}
    </a>
    
    <SocialLinks class="mt-12" />
  </div>
  
  <div class="absolute bottom-8 left-1/2 -translate-x-1/2">
    <ScrollIndicator />
  </div>
</section>

<script>
  import { initHeroAnimation } from '../animations/timelines';
  initHeroAnimation('[data-gsap="hero"]');
</script>
```

**Animation Specifications**:
| Element | Animation | Trigger | Duration | Ease |
|---------|-----------|---------|----------|------|
| Title | y: 60→0, opacity: 0→1 | Page load | 1.0s | power3.out |
| Subtitle | y: 40→0, opacity: 0→1 | Page load | 0.8s | power3.out |
| CTA | scale: 0.8→1, opacity: 0→1 | Page load | 0.5s | back.out(1.7) |
| Background | Parallax y: 0→20% | Scroll | scrub | none |

**Performance Requirements**:
- Image must use `fetchpriority="high"` and `loading="eager"`
- LCP target: <1.2s on 4G
- Animation must respect `prefers-reduced-motion`

---

### 2. Project Card Component

**Purpose**: Display project preview with metadata and links

**Props Interface**:
```typescript
interface ProjectCardProps {
  project: CollectionEntry<'projects'>;
  index: number; // For stagger animation
  featured?: boolean;
}
```

**Implementation**:
```astro
---
// src/components/ProjectCard.astro
import { Image } from 'astro:assets';
import type { CollectionEntry } from 'astro:content';

interface Props {
  project: CollectionEntry<'projects'>;
  index: number;
  featured?: boolean;
}

const { project, index, featured = false } = Astro.props;
const { title, description, thumbnail, technologies, links, category } = project.data;
---

<article 
  class:list={[
    "project-card group relative bg-white dark:bg-gray-800 rounded-xl overflow-hidden shadow-lg transition-all duration-300",
    "hover:shadow-xl hover:-translate-y-1",
    { "md:col-span-2 md:row-span-2": featured }
  ]}
  data-gsap="project-card"
  data-index={index}
>
  <a href={`/projects/${project.slug}`} class="block">
    <div class:list={["relative overflow-hidden", { "aspect-video": !featured, "aspect-square md:aspect-[4/3]": featured }]}>
      <Image
        src={thumbnail}
        alt={title}
        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
        width={featured ? 800 : 400}
        height={featured ? 600 : 225}
        loading="lazy"
      />
      
      <div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity">
        <div class="absolute bottom-4 left-4 right-4">
          <span class="inline-flex items-center px-3 py-1 bg-blue-600 text-white text-sm rounded-full">
            View Project →
          </span>
        </div>
      </div>
    </div>
    
    <div class="p-6">
      <div class="flex items-center gap-2 mb-2">
        <span class="text-xs font-medium text-blue-600 uppercase tracking-wide">
          {category}
        </span>
        {featured && (
          <span class="text-xs font-medium text-amber-500 uppercase tracking-wide">
            ★ Featured
          </span>
        )}
      </div>
      
      <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-2 group-hover:text-blue-600 transition-colors">
        {title}
      </h3>
      
      <p class="text-gray-600 dark:text-gray-300 text-sm line-clamp-2 mb-4">
        {description}
      </p>
      
      <div class="flex flex-wrap gap-2">
        {technologies.slice(0, 3).map((tech) => (
          <span class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs rounded">
            {tech}
          </span>
        ))}
        {technologies.length > 3 && (
          <span class="px-2 py-1 text-gray-500 dark:text-gray-400 text-xs">
            +{technologies.length - 3} more
          </span>
        )}
      </div>
    </div>
  </a>
</article>
```

**Animation Specifications**:
| Element | Animation | Trigger | Delay | Duration |
|---------|-----------|---------|-------|----------|
| Card | y: 50→0, opacity: 0→1 | Scroll (85% viewport) | index * 100ms | 0.6s |
| Image | scale: 1→1.05 | Hover | 0ms | 0.5s |
| Overlay | opacity: 0→1 | Hover | 0ms | 0.3s |

---

### 3. Project Grid Component

**Purpose**: Filterable, animated grid of project cards

**Props Interface**:
```typescript
interface ProjectGridProps {
  projects: CollectionEntry<'projects'>[];
  showFilters?: boolean;
  initialFilter?: string;
}
```

**Implementation**:
```astro
---
// src/components/ProjectGrid.astro
import ProjectCard from './ProjectCard.astro';
import ProjectFilters from './ProjectFilters.jsx';
import type { CollectionEntry } from 'astro:content';

interface Props {
  projects: CollectionEntry<'projects'>[];
  showFilters?: boolean;
  initialFilter?: string;
}

const { projects, showFilters = true, initialFilter = 'all' } = Astro.props;

// Sort: featured first, then by date
const sortedProjects = projects.sort((a, b) => {
  if (a.data.featured && !b.data.featured) return -1;
  if (!a.data.featured && b.data.featured) return 1;
  return new Date(b.data.pubDate).getTime() - new Date(a.data.pubDate).getTime();
});

// Extract unique categories
const categories = ['all', ...new Set(projects.map(p => p.data.category))];
---

<section class="projects-section py-20" data-gsap="projects-section">
  <div class="container mx-auto px-6">
    <div class="mb-12">
      <h2 class="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-4">
        Projects
      </h2>
      <p class="text-gray-600 dark:text-gray-300 max-w-2xl">
        A selection of my recent work across AI, CLI tools, web applications, and more.
      </p>
    </div>
    
    {showFilters && (
      <ProjectFilters 
        categories={categories}
        initialFilter={initialFilter}
        client:visible
      />
    )}
    
    <div 
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      data-gsap="projects-grid"
    >
      {sortedProjects.map((project, index) => (
        <ProjectCard 
          project={project} 
          index={index}
          featured={project.data.featured}
        />
      ))}
    </div>
  </div>
</section>

<script>
  import { initProjectReveals } from '../animations/scrollTriggers';
  initProjectReveals();
</script>
```

---

### 4. Skills Matrix Component

**Purpose**: Visual representation of technical skills with proficiency indicators

**Props Interface**:
```typescript
interface SkillsMatrixProps {
  skills: CollectionEntry<'skills'>[];
  layout?: 'grid' | 'cloud' | 'bars';
}

interface Skill {
  name: string;
  level: 'beginner' | 'intermediate' | 'advanced' | 'expert';
  category: 'language' | 'framework' | 'tool' | 'domain';
  years: number;
  icon?: string;
}
```

**Implementation**:
```astro
---
// src/components/SkillsMatrix.astro
import type { CollectionEntry } from 'astro:content';

interface Props {
  skills: CollectionEntry<'skills'>[];
  layout?: 'grid' | 'bars';
}

const { skills, layout = 'grid' } = Astro.props;

// Group by category
const byCategory = skills.reduce((acc, skill) => {
  const cat = skill.data.category;
  if (!acc[cat]) acc[cat] = [];
  acc[cat].push(skill);
  return acc;
}, {} as Record<string, CollectionEntry<'skills'>[]>);

const levelColors = {
  beginner: 'bg-gray-400',
  intermediate: 'bg-blue-400',
  advanced: 'bg-blue-600',
  expert: 'bg-blue-800',
};

const levelValues = {
  beginner: 25,
  intermediate: 50,
  advanced: 75,
  expert: 100,
};
---

<section class="skills-section py-20" data-gsap="skills-section">
  <div class="container mx-auto px-6">
    <h2 class="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-12">
      Skills & Technologies
    </h2>
    
    {layout === 'grid' ? (
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        {Object.entries(byCategory).map(([category, categorySkills]) => (
          <div class="skill-category" data-gsap="skill-category">
            <h3 class="text-lg font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-4">
              {category}
            </h3>
            <div class="space-y-3">
              {categorySkills.map((skill, i) => (
                <div 
                  class="skill-item flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg"
                  data-gsap="skill-item"
                  data-index={i}
                >
                  <span class="font-medium text-gray-900 dark:text-white">
                    {skill.data.name}
                  </span>
                  <span 
                    class={`inline-block w-3 h-3 rounded-full ${levelColors[skill.data.level]}`}
                    title={`${skill.data.level} (${skill.data.years} years)`}
                  />
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    ) : (
      <div class="space-y-8">
        {Object.entries(byCategory).map(([category, categorySkills]) => (
          <div class="skill-category" data-gsap="skill-category">
            <h3 class="text-lg font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-4">
              {category}
            </h3>
            <div class="space-y-4">
              {categorySkills.map((skill, i) => (
                <div 
                  class="skill-bar"
                  data-gsap="skill-bar"
                  data-index={i}
                >
                  <div class="flex justify-between mb-1">
                    <span class="font-medium text-gray-900 dark:text-white">
                      {skill.data.name}
                    </span>
                    <span class="text-sm text-gray-500">
                      {skill.data.years} years
                    </span>
                  </div>
                  <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                    <div 
                      class={`h-full ${levelColors[skill.data.level]} rounded-full skill-bar-fill`}
                      style={`width: ${levelValues[skill.data.level]}%`}
                      data-width={levelValues[skill.data.level]}
                    />
                  </div>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    )}
  </div>
</section>

<script>
  import { initSkillsAnimation } from '../animations/scrollTriggers';
  initSkillsAnimation();
</script>
```

**Animation Specifications**:
| Element | Animation | Trigger | Delay |
|---------|-----------|---------|-------|
| Skill category | y: 30→0, opacity: 0→1 | Scroll | stagger 100ms |
| Skill items | x: -20→0, opacity: 0→1 | Scroll | index * 50ms |
| Bar fills | width: 0→target% | Scroll | 0.8s duration |

---

### 5. Experience Timeline Component

**Purpose**: Chronological display of work history with highlights

**Props Interface**:
```typescript
interface ExperienceTimelineProps {
  experiences: CollectionEntry<'experience'>[];
}

interface Experience {
  company: string;
  role: string;
  startDate: Date;
  endDate?: Date;
  current: boolean;
  description: string;
  highlights: string[];
  technologies: string[];
}
```

**Implementation**:
```astro
---
// src/components/ExperienceTimeline.astro
import { formatDate } from '../utils/dates';
import type { CollectionEntry } from 'astro:content';

interface Props {
  experiences: CollectionEntry<'experience'>[];
}

const { experiences } = Astro.props;

// Sort by date (newest first)
const sorted = experiences.sort((a, b) => 
  new Date(b.data.startDate).getTime() - new Date(a.data.startDate).getTime()
);
---

<section class="experience-section py-20" data-gsap="experience-section">
  <div class="container mx-auto px-6">
    <h2 class="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-12">
      Experience
    </h2>
    
    <div class="relative">
      {/* Timeline line */}
      <div class="absolute left-4 md:left-1/2 top-0 bottom-0 w-0.5 bg-gray-200 dark:bg-gray-700 -translate-x-1/2" />
      
      <div class="space-y-12">
        {sorted.map((exp, index) => (
          <div 
            class:list={[
              "experience-item relative flex items-start gap-8",
              index % 2 === 0 ? "md:flex-row" : "md:flex-row-reverse"
            ]}
            data-gsap="experience-item"
            data-index={index}
          >
            {/* Timeline dot */}
            <div class="absolute left-4 md:left-1/2 w-4 h-4 bg-blue-600 rounded-full border-4 border-white dark:border-gray-900 -translate-x-1/2 z-10" />
            
            {/* Content card */}
            <div class:list={[
              "ml-12 md:ml-0 md:w-5/12",
              index % 2 === 0 ? "md:pr-8 md:text-right" : "md:pl-8"
            ]}>
              <div class="bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg">
                <div class="flex flex-wrap items-center gap-2 mb-2">
                  {exp.data.current && (
                    <span class="px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-100 text-xs font-medium rounded">
                      Current
                    </span>
                  )}
                  <span class="text-sm text-gray-500">
                    {formatDate(exp.data.startDate)} - {exp.data.current ? 'Present' : formatDate(exp.data.endDate)}
                  </span>
                </div>
                
                <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-1">
                  {exp.data.role}
                </h3>
                <p class="text-blue-600 font-medium mb-3">
                  {exp.data.company}
                </p>
                
                <p class="text-gray-600 dark:text-gray-300 mb-4">
                  {exp.data.description}
                </p>
                
                {exp.data.highlights.length > 0 && (
                  <ul class:list={[
                    "space-y-2 text-sm text-gray-600 dark:text-gray-400",
                    index % 2 === 0 ? "md:text-right" : ""
                  ]}>
                    {exp.data.highlights.map((highlight) => (
                      <li class={index % 2 === 0 ? "md:flex md:justify-end" : ""}>
                        <span class="flex items-center gap-2">
                          {index % 2 !== 0 && <span class="text-blue-600">→</span>}
                          {highlight}
                          {index % 2 === 0 && <span class="text-blue-600">←</span>}
                        </span>
                      </li>
                    ))}
                  </ul>
                )}
                
                <div class="flex flex-wrap gap-2 mt-4">
                  {exp.data.technologies.map((tech) => (
                    <span class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs rounded">
                      {tech}
                    </span>
                  ))}
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  </div>
</section>
```

---

### 6. Theme Toggle Component (Interactive Island)

**Purpose**: Dark/light mode toggle with system preference detection

**Implementation**:
```tsx
// src/components/ThemeToggle.tsx
import { useState, useEffect } from 'react';

export function ThemeToggle() {
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    // Check localStorage or system preference
    const saved = localStorage.getItem('theme') as 'light' | 'dark' | null;
    const system = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    const initial = saved || system;
    setTheme(initial);
    document.documentElement.classList.toggle('dark', initial === 'dark');
  }, []);

  const toggle = () => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
    localStorage.setItem('theme', newTheme);
    document.documentElement.classList.toggle('dark', newTheme === 'dark');
  };

  if (!mounted) {
    return <div className="w-10 h-10" />; // Prevent hydration mismatch
  }

  return (
    <button
      onClick={toggle}
      className="p-2 rounded-lg bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
      aria-label={theme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
    >
      {theme === 'light' ? (
        <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
        </svg>
      ) : (
        <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
      )}
    </button>
  );
}
```

**Usage in Astro**:
```astro
<ThemeToggle client:load />
```

---

### 7. Contact Form Component (Interactive Island)

**Purpose**: User inquiry capture with validation

**Implementation**:
```tsx
// src/components/ContactForm.tsx
import { useState } from 'react';

interface FormData {
  name: string;
  email: string;
  subject: string;
  message: string;
}

export function ContactForm() {
  const [formData, setFormData] = useState<FormData>({
    name: '',
    email: '',
    subject: '',
    message: '',
  });
  const [status, setStatus] = useState<'idle' | 'submitting' | 'success' | 'error'>('idle');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setStatus('submitting');

    try {
      const response = await fetch('/api/contact', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });

      if (response.ok) {
        setStatus('success');
        setFormData({ name: '', email: '', subject: '', message: '' });
      } else {
        setStatus('error');
      }
    } catch {
      setStatus('error');
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <label htmlFor="name" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Name *
          </label>
          <input
            type="text"
            id="name"
            required
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            className="w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>
        
        <div>
          <label htmlFor="email" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Email *
          </label>
          <input
            type="email"
            id="email"
            required
            value={formData.email}
            onChange={(e) => setFormData({ ...formData, email: e.target.value })}
            className="w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>
      </div>

      <div>
        <label htmlFor="subject" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Subject *
        </label>
        <input
          type="text"
          id="subject"
          required
          value={formData.subject}
          onChange={(e) => setFormData({ ...formData, subject: e.target.value })}
          className="w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        />
      </div>

      <div>
        <label htmlFor="message" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Message *
        </label>
        <textarea
          id="message"
          required
          rows={5}
          value={formData.message}
          onChange={(e) => setFormData({ ...formData, message: e.target.value })}
          className="w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-y"
        />
      </div>

      <button
        type="submit"
        disabled={status === 'submitting'}
        className="w-full md:w-auto px-8 py-4 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      >
        {status === 'submitting' ? 'Sending...' : 'Send Message'}
      </button>

      {status === 'success' && (
        <p className="text-green-600">Thank you! Your message has been sent.</p>
      )}
      {status === 'error' && (
        <p className="text-red-600">Something went wrong. Please try again.</p>
      )}
    </form>
  );
}
```

**Usage in Astro**:
```astro
<ContactForm client:idle />
```

---

## Data Models

### Project Model

```typescript
// src/content/config.ts
import { defineCollection, z } from 'astro:content';

const projects = defineCollection({
  type: 'content',
  schema: z.object({
    // Required fields
    title: z.string().max(100),
    description: z.string().max(500),
    
    // Dates
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
    
    // Media
    thumbnail: z.string(), // Path to image
    images: z.array(z.string()).optional(),
    videoUrl: z.string().url().optional(),
    
    // Categorization
    category: z.enum(['ai', 'cli', 'web', 'mobile', 'desktop', 'other']),
    featured: z.boolean().default(false),
    
    // Technologies
    technologies: z.array(z.string()).min(1),
    
    // Links
    links: z.object({
      demo: z.string().url().optional(),
      repo: z.string().url().optional(),
      docs: z.string().url().optional(),
      article: z.string().url().optional(),
    }),
    
    // SEO
    metaTitle: z.string().max(60).optional(),
    metaDescription: z.string().max(160).optional(),
    keywords: z.array(z.string()).optional(),
    
    // Content
    body: z.string().optional(), // Markdown content
  }),
});

export type Project = z.infer<typeof projects.schema>;
```

### Skill Model

```typescript
const skills = defineCollection({
  type: 'data',
  schema: z.object({
    name: z.string(),
    
    level: z.enum(['beginner', 'intermediate', 'advanced', 'expert']),
    
    category: z.enum([
      'language',      // Programming languages
      'framework',     // Web frameworks, libraries
      'tool',          // Dev tools, CLIs
      'platform',      // Cloud platforms
      'database',      // Data storage
      'devops',        // CI/CD, deployment
      'domain',        // AI, systems, etc.
    ]),
    
    years: z.number().min(0).max(50),
    
    icon: z.string().optional(), // Icon name or URL
    
    // For ordering within category
    priority: z.number().default(0),
    
    // Show in featured section
    featured: z.boolean().default(false),
  }),
});

export type Skill = z.infer<typeof skills.schema>;
```

### Experience Model

```typescript
const experiences = defineCollection({
  type: 'content',
  schema: z.object({
    company: z.string(),
    role: z.string(),
    
    startDate: z.coerce.date(),
    endDate: z.coerce.date().optional(),
    current: z.boolean().default(false),
    
    description: z.string(),
    
    highlights: z.array(z.string()).default([]),
    
    technologies: z.array(z.string()).default([]),
    
    location: z.string().optional(),
    
    // For ordering
    priority: z.number().default(0),
  }),
});

export type Experience = z.infer<typeof experiences.schema>;
```

### Site Configuration

```typescript
// src/config/site.ts
export const siteConfig = {
  // Identity
  name: 'Koosha Paridehpour',
  title: 'Koosha Paridehpour — Software Engineer',
  tagline: 'Building AI-powered tools and developer experiences',
  description: 'Portfolio of Koosha Paridehpour, software engineer specializing in AI, CLI tools, and web development.',
  
  // URLs
  url: 'https://koosha.dev',
  ogImage: '/images/og-default.jpg',
  
  // Social
  social: {
    github: 'https://github.com/KooshaPari',
    linkedin: 'https://linkedin.com/in/koosha',
    twitter: 'https://twitter.com/kooshapari',
    email: 'koosha@example.com',
  },
  
  // Theme
  theme: {
    default: 'system' as const,
    respectPrefersColorScheme: true,
  },
  
  // SEO
  seo: {
    robots: 'index, follow',
    googleAnalyticsId: '',
  },
  
  // Content
  pagination: {
    projectsPerPage: 9,
    postsPerPage: 10,
  },
} as const;

export type SiteConfig = typeof siteConfig;
```

---

## Content Schema

### Project Markdown Structure

```markdown
---
title: "HeliosCLI"
description: "AI-powered CLI framework for building intelligent command-line tools"
pubDate: 2025-03-15
updatedDate: 2025-04-01
thumbnail: "./helioscli.png"
category: "cli"
featured: true
technologies: ["Rust", "Clap", "Tokio", "Ratatui"]
links:
  demo: "https://demo.helios.dev"
  repo: "https://github.com/koosha/helioscli"
  docs: "https://docs.helios.dev"
metaTitle: "HeliosCLI — AI-Powered CLI Framework"
metaDescription: "Build intelligent command-line tools with Rust and AI integration"
keywords: ["rust", "cli", "ai", "developer-tools"]
---

## Overview

HeliosCLI is a comprehensive framework for building AI-powered command-line interfaces...

## Features

- **AI Integration**: Built-in support for LLM providers
- **Sandboxed Execution**: Safe command execution with multiple sandbox backends
- **TUI Support**: Beautiful terminal UIs with Ratatui

## Technical Details

### Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   CLI Args  │────▶│   Engine    │────▶│  Sandbox    │
└─────────────┘     └─────────────┘     └─────────────┘
```

### Performance

- Cold start: <50ms
- Memory usage: <10MB
- Binary size: 2.4MB

## Lessons Learned

Building HeliosCLI taught me the importance of...
```

### Validation Rules

| Field | Validation | Error Message |
|-------|------------|---------------|
| title | Required, max 100 chars | "Title is required and must be under 100 characters" |
| description | Required, max 500 chars | "Description is required and must be under 500 characters" |
| pubDate | Valid date | "Publication date must be a valid date" |
| category | One of enum values | "Invalid category" |
| technologies | At least 1 item | "At least one technology is required" |
| links.demo | Valid URL if present | "Demo URL must be valid" |
| thumbnail | Valid image path | "Thumbnail must reference a valid image" |

---

## Performance Budgets

### Page-Level Budgets

| Metric | Target | Maximum | Measurement |
|--------|--------|---------|-------------|
| **First Contentful Paint (FCP)** | < 1.0s | 1.5s | Lighthouse |
| **Largest Contentful Paint (LCP)** | < 1.5s | 2.5s | Lighthouse |
| **Time to Interactive (TTI)** | < 2.0s | 3.5s | Lighthouse |
| **Cumulative Layout Shift (CLS)** | < 0.05 | 0.1 | Lighthouse |
| **Total Blocking Time (TBT)** | < 100ms | 200ms | Lighthouse |
| **Speed Index** | < 1.5s | 2.5s | Lighthouse |
| **Total Page Size** | < 500KB | 1MB | Network tab |
| **JavaScript Bundle** | < 50KB | 100KB | Bundle analyzer |
| **CSS Bundle** | < 20KB | 30KB | Bundle analyzer |
| **Image Transfer** | < 300KB | 500KB | Network tab |

### Resource Budgets

```
Per-Page Resource Limits:
├── HTML: 50KB (uncompressed)
├── CSS: 20KB (compressed)
├── JavaScript: 50KB (compressed)
├── Images: 300KB (optimized)
├── Fonts: 40KB (subsets)
└── Total: < 500KB compressed

Critical Rendering Path:
├── First Paint: < 1.0s (3G)
├── Time to First Byte: < 200ms
├── First Contentful Paint: < 1.0s
├── Largest Contentful Paint: < 1.5s
└── Time to Interactive: < 2.0s
```

### Core Web Vitals Targets

| Metric | Good | Needs Improvement | Poor |
|--------|------|-------------------|------|
| **LCP** | ≤1.5s | ≤2.5s | >2.5s |
| **INP** | ≤100ms | ≤200ms | >200ms |
| **CLS** | ≤0.05 | ≤0.1 | >0.1 |
| **TTFB** | ≤200ms | ≤600ms | >600ms |
| **FCP** | ≤1.0s | ≤1.8s | >1.8s |

### Mobile Performance

| Metric | Target | Notes |
|--------|--------|-------|
| Mobile LCP | < 2.0s | 4G simulation |
| Mobile Speed Index | < 2.5s | Moto G4 |
| Responsive Images | 100% | srcset + sizes |
| Touch Targets | 48px min | All interactive |
| Font Size | 16px min | Readable without zoom |

---

## Animation System

### GSAP Configuration

```typescript
// src/animations/config.ts
export const gsapConfig = {
  // Default ease for entrance animations
  defaultEase: 'power2.out',
  
  // Default stagger delay
  defaultStagger: 0.1,
  
  // Reduced motion settings
  reducedMotion: {
    enabled: false,
    duration: 0.001, // Instant when reduced
  },
  
  // ScrollTrigger defaults
  scrollTrigger: {
    start: 'top 85%',
    end: 'bottom 20%',
    toggleActions: 'play none none reverse',
  },
};

// Duration presets
export const durations = {
  fast: 0.3,
  normal: 0.6,
  slow: 1.0,
  dramatic: 1.5,
};

// Ease presets
export const eases = {
  entrance: 'power3.out',
  exit: 'power2.in',
  bounce: 'back.out(1.7)',
  elastic: 'elastic.out(1, 0.3)',
  smooth: 'power2.inOut',
};
```

### Animation Patterns

#### Entrance Animations

```typescript
// src/animations/patterns/entrances.ts
import gsap from 'gsap';

export const fadeIn = (element: string | Element, duration = 0.6) => {
  return gsap.from(element, {
    opacity: 0,
    duration,
    ease: 'power2.out',
  });
};

export const slideUp = (element: string | Element, y = 50, duration = 0.6) => {
  return gsap.from(element, {
    y,
    opacity: 0,
    duration,
    ease: 'power3.out',
  });
};

export const scaleIn = (element: string | Element, scale = 0.8, duration = 0.5) => {
  return gsap.from(element, {
    scale,
    opacity: 0,
    duration,
    ease: 'back.out(1.7)',
  });
};

export const staggerReveal = (
  elements: string | Element[],
  stagger = 0.1,
  y = 30
) => {
  return gsap.from(elements, {
    y,
    opacity: 0,
    duration: 0.6,
    stagger,
    ease: 'power2.out',
  });
};
```

#### Scroll Animations

```typescript
// src/animations/patterns/scroll.ts
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

export const scrollReveal = (
  element: string | Element,
  options: {
    start?: string;
    y?: number;
    duration?: number;
  } = {}
) => {
  const { start = 'top 85%', y = 50, duration = 0.6 } = options;
  
  return gsap.from(element, {
    y,
    opacity: 0,
    duration,
    ease: 'power2.out',
    scrollTrigger: {
      trigger: element,
      start,
      toggleActions: 'play none none reverse',
    },
  });
};

export const parallax = (
  element: string | Element,
  options: {
    speed?: number; // 0.1 to 0.5
    direction?: 'vertical' | 'horizontal';
  } = {}
) => {
  const { speed = 0.2, direction = 'vertical' } = options;
  const property = direction === 'vertical' ? 'y' : 'x';
  
  return gsap.to(element, {
    [property]: `${speed * 100}%`,
    ease: 'none',
    scrollTrigger: {
      trigger: element,
      start: 'top bottom',
      end: 'bottom top',
      scrub: true,
    },
  });
};

export const pinSection = (
  element: string | Element,
  duration: number,
  animations: gsap.Timeline | gsap.Tween
) => {
  return gsap.timeline({
    scrollTrigger: {
      trigger: element,
      start: 'top top',
      end: `+=${duration}`,
      pin: true,
      scrub: 1,
    },
  }).add(animations);
};
```

### Accessibility: Reduced Motion

```typescript
// src/animations/utils/accessibility.ts
export const prefersReducedMotion = (): boolean => {
  if (typeof window === 'undefined') return false;
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
};

export const safeAnimate = (
  animation: () => gsap.core.Animation | void
): gsap.core.Animation | void => {
  if (prefersReducedMotion()) {
    // Return null or empty animation
    return;
  }
  return animation();
};

// Usage
if (!prefersReducedMotion()) {
  gsap.from('.element', { y: 50, opacity: 0 });
} else {
  // Instant visibility for reduced motion
  gsap.set('.element', { opacity: 1 });
}
```

---

## Testing Strategy

### Testing Pyramid

```
                    ┌─────────────┐
                    │   E2E       │  10% (Playwright)
                    │  (Critical  │
                    │   paths)    │
                    ├─────────────┤
                    │  Component  │  20% (Vitest + 
                    │   (Islands) │   Testing Library)
                    ├─────────────┤
                    │   Visual    │  30% (Chromatic/
                    │  (VRT)      │   Percy)
                    ├─────────────┤
                    │  Static     │  40% (Astro check,
                    │  Analysis   │   type check, lint)
                    └─────────────┘
```

### Unit Tests (Vitest)

```typescript
// src/utils/__tests__/dates.test.ts
import { describe, it, expect } from 'vitest';
import { formatDate, formatDuration } from '../dates';

describe('formatDate', () => {
  it('formats date to readable string', () => {
    const date = new Date('2025-03-15');
    expect(formatDate(date)).toBe('March 15, 2025');
  });
  
  it('handles custom format', () => {
    const date = new Date('2025-03-15');
    expect(formatDate(date, 'short')).toBe('Mar 15, 2025');
  });
});

describe('formatDuration', () => {
  it('calculates years between dates', () => {
    const start = new Date('2020-01-01');
    const end = new Date('2025-01-01');
    expect(formatDuration(start, end)).toBe('5 years');
  });
  
  it('handles current position', () => {
    const start = new Date('2020-01-01');
    expect(formatDuration(start)).toContain('years');
  });
});
```

### Component Tests

```typescript
// src/components/__tests__/ThemeToggle.test.tsx
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { ThemeToggle } from '../ThemeToggle';

describe('ThemeToggle', () => {
  beforeEach(() => {
    // Mock localStorage
    Storage.prototype.getItem = vi.fn(() => null);
    Storage.prototype.setItem = vi.fn();
  });
  
  it('renders without crashing', () => {
    render(<ThemeToggle />);
    expect(screen.getByRole('button')).toBeInTheDocument();
  });
  
  it('toggles theme on click', () => {
    render(<ThemeToggle />);
    const button = screen.getByRole('button');
    
    fireEvent.click(button);
    expect(localStorage.setItem).toHaveBeenCalledWith('theme', 'dark');
  });
});
```

### Visual Regression Tests

```typescript
// .storybook/main.ts
export default {
  stories: ['../src/**/*.stories.astro'],
  addons: ['@storybook/addon-a11y'],
  framework: '@storybook/astro',
};

// Chromatic configuration
// .github/workflows/visual-tests.yml
name: Visual Tests

on: [push]

jobs:
  chromatic:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - run: npm ci
      - run: npm run build-storybook
      
      - uses: chromaui/action@v1
        with:
          projectToken: ${{ secrets.CHROMATIC_PROJECT_TOKEN }}
          storybookBuildDir: storybook-static
```

### E2E Tests (Playwright)

```typescript
// e2e/homepage.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Homepage', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });
  
  test('has correct title', async ({ page }) => {
    await expect(page).toHaveTitle(/Koosha Paridehpour/);
  });
  
  test('hero section is visible', async ({ page }) => {
    const hero = page.locator('[data-gsap="hero"]');
    await expect(hero).toBeVisible();
  });
  
  test('navigation links work', async ({ page }) => {
    await page.click('text=Projects');
    await expect(page).toHaveURL(/\/projects/);
  });
  
  test('theme toggle works', async ({ page }) => {
    const toggle = page.locator('[aria-label*="dark mode"], [aria-label*="light mode"]');
    await toggle.click();
    
    // Check dark class is applied
    const html = page.locator('html');
    await expect(html).toHaveClass(/dark/);
  });
  
  test('projects load on scroll', async ({ page }) => {
    // Scroll to projects section
    await page.locator('.projects-section').scrollIntoViewIfNeeded();
    
    // Check project cards are visible
    const cards = page.locator('.project-card');
    await expect(cards.first()).toBeVisible();
  });
  
  test('contact form validates', async ({ page }) => {
    await page.goto('/contact');
    
    // Submit empty form
    await page.click('button[type="submit"]');
    
    // Check validation messages
    await expect(page.locator('text=required')).toBeVisible();
  });
  
  test('performance meets budget', async ({ page }) => {
    const loadTimes: number[] = [];
    
    // Run multiple times for average
    for (let i = 0; i < 3; i++) {
      const start = Date.now();
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      loadTimes.push(Date.now() - start);
    }
    
    const avgLoadTime = loadTimes.reduce((a, b) => a + b, 0) / loadTimes.length;
    expect(avgLoadTime).toBeLessThan(2000); // 2s budget
  });
});
```

### Accessibility Tests

```typescript
// e2e/a11y.spec.ts
import { test, expect } from '@playwright/test';
import { injectAxe, checkA11y } from 'axe-playwright';

test.describe('Accessibility', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await injectAxe(page);
  });
  
  test('has no detectable accessibility violations', async ({ page }) => {
    await checkA11y(page, undefined, {
      axeOptions: {
        runOnly: {
          type: 'tag',
          values: ['wcag2a', 'wcag2aa', 'wcag21aa'],
        },
      },
    });
  });
  
  test('keyboard navigation works', async ({ page }) => {
    // Tab through interactive elements
    await page.keyboard.press('Tab');
    await expect(page.locator(':focus')).toBeVisible();
    
    // Navigate to projects
    await page.goto('/');
    for (let i = 0; i < 5; i++) {
      await page.keyboard.press('Tab');
    }
    
    // Press enter on link
    await page.keyboard.press('Enter');
    await expect(page).toHaveURL(/\/projects|#projects/);
  });
  
  test('color contrast meets WCAG AA', async ({ page }) => {
    const violations = await page.evaluate(() => {
      // Run axe-core in browser
      return new Promise((resolve) => {
        // @ts-ignore
        axe.run({ runOnly: ['color-contrast'] }, (err: any, results: any) => {
          resolve(results.violations);
        });
      });
    });
    
    expect(violations).toHaveLength(0);
  });
});
```

---

## Build Pipeline

### Build Process

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              BUILD PIPELINE                                      │
│                                                                                  │
│  1. Content Validation                                                           │
│     ├── Zod schema validation for all content                                    │
│     ├── TypeScript type generation from schemas                                  │
│     └── Image dimension verification                                             │
│                    ↓                                                             │
│  2. Static Generation (Astro build)                                                │
│     ├── Parse Markdown/MDX                                                       │
│     ├── Render Astro components                                                  │
│     ├── Generate static HTML                                                    │
│     └── Create page routes                                                        │
│                    ↓                                                             │
│  3. Asset Processing                                                               │
│     ├── Sharp: Resize, optimize, convert images                                  │
│     ├── Generate responsive srcsets                                              │
│     └── Create WebP/AVIF variants                                                │
│                    ↓                                                             │
│  4. Bundle Optimization                                                            │
│     ├── Vite: Tree-shaking, minification                                         │
│     ├── CSS Purge: Remove unused Tailwind classes                                │
│     ├── Code split: Separate island chunks                                        │
│     └── Generate source maps (dev only)                                          │
│                    ↓                                                             │
│  5. Output Generation                                                              │
│     ├── HTML pages (dist/*.html)                                                  │
│     ├── _astro/ (JS chunks)                                                       │
│     ├── _astro/ (CSS chunks)                                                      │
│     └── assets/ (optimized images)                                                │
│                    ↓                                                             │
│  6. Post-Build Checks                                                              │
│     ├── Lighthouse CI audit                                                      │
│     ├── Bundle size check                                                        │
│     ├── Link checker (no 404s)                                                   │
│     └── Sitemap generation                                                        │
└──────────────────────────────────────────────────────────────────────────────────┘
```

### npm Scripts

```json
{
  "scripts": {
    "dev": "astro dev",
    "build": "astro build",
    "preview": "astro preview",
    "astro": "astro",
    
    "test": "vitest",
    "test:e2e": "playwright test",
    "test:a11y": "playwright test e2e/a11y.spec.ts",
    
    "lint": "eslint . --ext .ts,.tsx,.astro",
    "lint:fix": "eslint . --ext .ts,.tsx,.astro --fix",
    
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    
    "typecheck": "astro check && tsc --noEmit",
    
    "lighthouse": "lhci autorun",
    "lighthouse:local": "lhci autorun --config=lighthouserc.local.js",
    
    "clean": "rm -rf dist node_modules/.astro",
    
    "prebuild": "npm run typecheck && npm run lint",
    "postbuild": "npm run generate-sitemap",
    
    "generate-sitemap": "astro-sitemap",
    "analyze": "npm run build && npx serve dist"
  }
}
```

### Astro Configuration

```javascript
// astro.config.mjs
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';
import react from '@astrojs/react';
import sitemap from '@astrojs/sitemap';
import mdx from '@astrojs/mdx';
import vercel from '@astrojs/vercel/static';

export default defineConfig({
  site: 'https://koosha.dev',
  
  // Static output for optimal performance
  output: 'static',
  
  // Build directory
  outDir: './dist',
  
  // Trailing slashes for SEO consistency
  trailingSlash: 'always',
  
  // Prefetch strategy
  prefetch: {
    prefetchAll: true,
    defaultStrategy: 'hover',
  },
  
  // Integrations
  integrations: [
    tailwind({
      applyBaseStyles: false,
    }),
    react(),
    sitemap({
      changefreq: 'weekly',
      priority: 0.7,
      lastmod: new Date(),
      i18n: {
        defaultLocale: 'en',
        locales: {
          en: 'en-US',
        },
      },
    }),
    mdx(),
  ],
  
  // Vercel adapter
  adapter: vercel({
    imageService: true,
    webAnalytics: {
      enabled: true,
    },
    speedInsights: {
      enabled: true,
    },
  }),
  
  // Image optimization
  image: {
    service: {
      entrypoint: 'astro/assets/services/sharp',
    },
    domains: [],
    remotePatterns: [],
  },
  
  // Vite configuration
  vite: {
    build: {
      cssCodeSplit: true,
      rollupOptions: {
        output: {
          manualChunks: {
            gsap: ['gsap', 'gsap/ScrollTrigger'],
          },
        },
      },
    },
    optimizeDeps: {
      include: ['gsap'],
    },
  },
  
  // Experimental features
  experimental: {
    viewTransitions: true,
  },
});
```

---

## Deployment Procedures

### Vercel Deployment

#### Configuration

```json
// vercel.json
{
  "framework": "astro",
  "buildCommand": "astro build",
  "outputDirectory": "dist",
  "installCommand": "npm ci",
  "regions": ["all"],
  
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "X-Frame-Options",
          "value": "DENY"
        },
        {
          "key": "X-Content-Type-Options",
          "value": "nosniff"
        },
        {
          "key": "Referrer-Policy",
          "value": "strict-origin-when-cross-origin"
        },
        {
          "key": "Content-Security-Policy",
          "value": "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval' *.vercel-scripts.com; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'; connect-src 'self' *.vercel-analytics.com; frame-ancestors 'none'; base-uri 'self'; form-action 'self';"
        }
      ]
    },
    {
      "source": "/_astro/(.*)",
      "headers": [
        {
          "key": "Cache-Control",
          "value": "public, max-age=31536000, immutable"
        }
      ]
    },
    {
      "source": "/images/(.*)",
      "headers": [
        {
          "key": "Cache-Control",
          "value": "public, max-age=31536000, immutable"
        }
      ]
    }
  ],
  
  "rewrites": [
    {
      "source": "/projects/:slug",
      "destination": "/projects/[slug].html"
    }
  ],
  
  "redirects": [
    {
      "source": "/github",
      "destination": "https://github.com/KooshaPari"
    },
    {
      "source": "/linkedin",
      "destination": "https://linkedin.com/in/koosha"
    }
  ]
}
```

#### GitHub Actions Workflow

```yaml
# .github/workflows/deploy.yml
name: Deploy to Vercel

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  VERCEL_ORG_ID: ${{ secrets.VERCEL_ORG_ID }}
  VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run tests
        run: npm run test

      - name: Run E2E tests
        run: npm run test:e2e

      - name: Build
        run: npm run build

      - name: Install Vercel CLI
        run: npm install --global vercel@latest

      - name: Pull Vercel Environment Information
        run: vercel pull --yes --environment=preview --token=${{ secrets.VERCEL_TOKEN }}

      - name: Build Project Artifacts
        run: vercel build --token=${{ secrets.VERCEL_TOKEN }}

      - name: Deploy to Preview
        if: github.event_name == 'pull_request'
        run: |
          DEPLOYMENT_URL=$(vercel deploy --prebuilt --token=${{ secrets.VERCEL_TOKEN }})
          echo "Preview URL: $DEPLOYMENT_URL"
          # Comment on PR with URL

      - name: Deploy to Production
        if: github.ref == 'refs/heads/main'
        run: vercel deploy --prebuilt --prod --token=${{ secrets.VERCEL_TOKEN }}

      - name: Run Lighthouse CI
        if: github.ref == 'refs/heads/main'
        run: npm run lighthouse
```

### Lighthouse CI Configuration

```javascript
// lighthouserc.js
module.exports = {
  ci: {
    collect: {
      url: ['http://localhost:4173/', 'http://localhost:4173/projects/'],
      startServerCommand: 'npm run preview',
      startServerReadyPattern: 'Local',
      numberOfRuns: 3,
    },
    assert: {
      assertions: {
        'categories:performance': ['error', { minScore: 0.95 }],
        'categories:accessibility': ['error', { minScore: 1 }],
        'categories:best-practices': ['error', { minScore: 1 }],
        'categories:seo': ['error', { minScore: 1 }],
        'first-contentful-paint': ['error', { maxNumericValue: 1000 }],
        'largest-contentful-paint': ['error', { maxNumericValue: 1500 }],
        'cumulative-layout-shift': ['error', { maxNumericValue: 0.05 }],
      },
    },
    upload: {
      target: 'temporary-public-storage',
    },
  },
};
```

---

## SEO & Meta Strategy

### Meta Tags

```astro
---
// src/components/Meta.astro
import { siteConfig } from '../config/site';

interface Props {
  title?: string;
  description?: string;
  image?: string;
  type?: 'website' | 'article';
  publishDate?: Date;
  modifiedDate?: Date;
  tags?: string[];
  canonical?: string;
  noindex?: boolean;
}

const {
  title = siteConfig.title,
  description = siteConfig.description,
  image = siteConfig.ogImage,
  type = 'website',
  publishDate,
  modifiedDate,
  tags,
  canonical,
  noindex = false,
} = Astro.props;

const ogImage = new URL(image, siteConfig.url).toString();
const canonicalUrl = canonical ? new URL(canonical, siteConfig.url).toString() : Astro.url.toString();
---

<!-- Primary Meta Tags -->
<title>{title}</title>
<meta name="title" content={title} />
<meta name="description" content={description} />
{noindex && <meta name="robots" content="noindex, nofollow" />}
{!noindex && <meta name="robots" content="index, follow" />}

<!-- Canonical -->
<link rel="canonical" href={canonicalUrl} />

<!-- Open Graph / Facebook -->
<meta property="og:type" content={type} />
<meta property="og:url" content={canonicalUrl} />
<meta property="og:title" content={title} />
<meta property="og:description" content={description} />
<meta property="og:image" content={ogImage} />
<meta property="og:site_name" content={siteConfig.name} />

<!-- Twitter -->
<meta property="twitter:card" content="summary_large_image" />
<meta property="twitter:url" content={canonicalUrl} />
<meta property="twitter:title" content={title} />
<meta property="twitter:description" content={description} />
<meta property="twitter:image" content={ogImage} />
<meta property="twitter:creator" content={siteConfig.social.twitter} />

<!-- Article-specific (if applicable) -->
{type === 'article' && publishDate && (
  <meta property="article:published_time" content={publishDate.toISOString()} />
)}
{type === 'article' && modifiedDate && (
  <meta property="article:modified_time" content={modifiedDate.toISOString()} />
)}
{type === 'article' && tags?.map((tag) => (
  <meta property="article:tag" content={tag} />
))}

<!-- Structured Data -->
<script type="application/ld+json" set:html={JSON.stringify({
  "@context": "https://schema.org",
  "@type": type === 'article' ? 'Article' : 'WebPage',
  headline: title,
  description: description,
  image: ogImage,
  url: canonicalUrl,
  ...(type === 'article' && publishDate && {
    datePublished: publishDate.toISOString(),
    dateModified: (modifiedDate || publishDate).toISOString(),
    author: {
      "@type": "Person",
      name: siteConfig.name,
      url: siteConfig.url,
    },
  }),
})} />
```

### Sitemap Configuration

```javascript
// Automatically generated by @astrojs/sitemap
// Config in astro.config.mjs
```

### robots.txt

```
User-agent: *
Allow: /

Sitemap: https://koosha.dev/sitemap-index.xml

# Disallow admin or private paths (if any)
# Disallow: /admin/
```

---

## Accessibility Requirements

### WCAG 2.1 AA Checklist

| Requirement | Implementation | Status |
|-------------|----------------|--------|
| **1.1.1 Non-text Content** | Alt text on all images | Required |
| **1.3.1 Info and Relationships** | Semantic HTML, ARIA when needed | Required |
| **1.4.3 Contrast** | 4.5:1 for text, 3:1 for UI | Required |
| **1.4.4 Resize Text** | 200% zoom without loss | Required |
| **2.1.1 Keyboard** | All functionality keyboard accessible | Required |
| **2.4.3 Focus Order** | Logical tab order | Required |
| **2.4.7 Focus Visible** | Visible focus indicators | Required |
| **4.1.2 Name, Role, Value** | Proper ARIA attributes | Required |

### Focus Management

```css
/* src/styles/global.css */
/* Visible focus indicators */
:focus-visible {
  outline: 2px solid #3b82f6;
  outline-offset: 2px;
}

/* Skip to content link */
.skip-link {
  position: absolute;
  top: -40px;
  left: 0;
  background: #000;
  color: #fff;
  padding: 8px;
  text-decoration: none;
  z-index: 100;
}

.skip-link:focus {
  top: 0;
}
```

### Screen Reader Support

```astro
<!-- Skip to main content -->
<a href="#main" class="skip-link">Skip to main content</a>

<!-- Main landmark -->
<main id="main">
  <!-- Content -->
</main>

<!-- Navigation landmark with current page -->
<nav aria-label="Main">
  <ul>
    <li>
      <a href="/" aria-current={Astro.url.pathname === '/' ? 'page' : undefined}>
        Home
      </a>
    </li>
  </ul>
</nav>

<!-- Button with aria-label for icon-only -->
<button aria-label="Close menu">
  <svg><!-- Close icon --></svg>
</button>

<!-- Live region for dynamic content -->
<div aria-live="polite" aria-atomic="true" id="announcements"></div>
```

---

## Security Considerations

### CSP Headers

```http
Content-Security-Policy:
  default-src 'self';
  script-src 'self' 'unsafe-inline' 'unsafe-eval' *.vercel-scripts.com vitals.vercel-insights.com;
  style-src 'self' 'unsafe-inline';
  img-src 'self' data: https: *.vercel.app;
  font-src 'self';
  connect-src 'self' *.vercel-analytics.com vitals.vercel-insights.com;
  frame-ancestors 'none';
  base-uri 'self';
  form-action 'self';
  upgrade-insecure-requests;
```

### Security Headers

| Header | Value | Purpose |
|--------|-------|---------|
| X-Frame-Options | DENY | Prevent clickjacking |
| X-Content-Type-Options | nosniff | Prevent MIME sniffing |
| Referrer-Policy | strict-origin-when-cross-origin | Privacy |
| Permissions-Policy | camera=(), microphone=(), geolocation=() | Feature policy |
| Strict-Transport-Security | max-age=31536000; includeSubDomains; preload | HSTS |

---

## Monitoring & Analytics

### Vercel Analytics

```javascript
// astro.config.mjs
import vercel from '@astrojs/vercel/static';

export default defineConfig({
  adapter: vercel({
    webAnalytics: {
      enabled: true,
    },
    speedInsights: {
      enabled: true,
    },
  }),
});
```

### Custom Events

```typescript
// src/utils/analytics.ts
export const trackEvent = (
  name: string,
  properties?: Record<string, string | number | boolean>
) => {
  // Vercel Analytics
  if (window.va) {
    window.va('event', { name, ...properties });
  }
  
  // Fallback: console in development
  if (import.meta.env.DEV) {
    console.log('Analytics:', { name, ...properties });
  }
};

// Usage
trackEvent('project_view', { project: 'helioscli' });
trackEvent('contact_submit');
trackEvent('theme_toggle', { theme: 'dark' });
```

---

## Development Workflow

### Git Workflow

```
main (production)
  ↑
feature/hero-animation
  ↑
fix/mobile-menu
```

### Pre-commit Hooks

```javascript
// .husky/pre-commit
#!/bin/sh
. "$(dirname "$0")/_/husky.sh"

npm run lint:fix
npm run format
git add -A
```

### Code Review Checklist

- [ ] Performance: No layout thrashing animations
- [ ] Accessibility: Proper ARIA, focus management
- [ ] SEO: Meta tags, structured data
- [ ] Images: Optimized, proper alt text
- [ ] Tests: Unit, E2E, visual regression
- [ ] Bundle: Under performance budgets

---

## File Organization

```
koosha-portfolio/
├── .github/
│   └── workflows/
│       ├── deploy.yml
│       └── visual-tests.yml
├── public/
│   ├── images/
│   │   ├── projects/
│   │   ├── skills/
│   │   └── icons/
│   ├── favicon.svg
│   └── robots.txt
├── src/
│   ├── animations/
│   │   ├── config.ts
│   │   ├── timelines.ts
│   │   ├── scrollTriggers.ts
│   │   ├── patterns/
│   │   └── utils/
│   ├── components/
│   │   ├── ui/               # Static Astro components
│   │   ├── interactive/      # Hydrated framework components
│   │   └── __tests__/
│   ├── content/
│   │   ├── config.ts
│   │   ├── projects/
│   │   ├── skills/
│   │   └── experience/
│   ├── data/
│   │   └── site.ts
│   ├── layouts/
│   │   ├── Base.astro
│   │   ├── Project.astro
│   │   └── Page.astro
│   ├── pages/
│   │   ├── index.astro
│   │   ├── about.astro
│   │   ├── projects/
│   │   │   └── [...slug].astro
│   │   └── contact.astro
│   ├── styles/
│   │   ├── global.css
│   │   └── utilities.css
│   ├── utils/
│   │   ├── dates.ts
│   │   ├── formatters.ts
│   │   └── analytics.ts
│   └── config/
│       └── site.ts
├── tests/
│   └── e2e/
├── .eslintrc.cjs
├── .prettierrc
├── astro.config.mjs
├── lighthouserc.js
├── package.json
├── tailwind.config.mjs
├── tsconfig.json
└── vitest.config.ts
```

---

## Dependencies & Versions

### Production Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| astro | ^5.x | Framework |
| @astrojs/react | ^4.x | React integration |
| @astrojs/tailwind | ^6.x | Tailwind integration |
| @astrojs/sitemap | ^3.x | Sitemap generation |
| @astrojs/mdx | ^4.x | MDX support |
| @astrojs/vercel | ^8.x | Vercel adapter |
| react | ^18.x | Interactive islands |
| react-dom | ^18.x | React DOM |
| gsap | ^3.x | Animations |
| tailwindcss | ^3.x | Styling |

### Development Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| typescript | ^5.x | Type checking |
| vitest | ^3.x | Unit testing |
| @testing-library/react | ^16.x | Component testing |
| playwright | ^1.x | E2E testing |
| eslint | ^9.x | Linting |
| prettier | ^3.x | Formatting |
| @astrojs/check | ^0.x | Astro type checking |

---

## Appendices

### Appendix A: Color Palette

```css
/* Tailwind config colors */
:root {
  /* Primary: Blue */
  --color-primary-50: #eff6ff;
  --color-primary-100: #dbeafe;
  --color-primary-200: #bfdbfe;
  --color-primary-300: #93c5fd;
  --color-primary-400: #60a5fa;
  --color-primary-500: #3b82f6;
  --color-primary-600: #2563eb;
  --color-primary-700: #1d4ed8;
  --color-primary-800: #1e40af;
  --color-primary-900: #1e3a8a;
  
  /* Neutral */
  --color-gray-50: #f9fafb;
  --color-gray-100: #f3f4f6;
  --color-gray-200: #e5e7eb;
  --color-gray-300: #d1d5db;
  --color-gray-400: #9ca3af;
  --color-gray-500: #6b7280;
  --color-gray-600: #4b5563;
  --color-gray-700: #374151;
  --color-gray-800: #1f2937;
  --color-gray-900: #111827;
}
```

### Appendix B: Typography Scale

| Element | Size | Weight | Line Height |
|---------|------|--------|-------------|
| H1 | 3rem (48px) | 800 | 1.1 |
| H2 | 2.25rem (36px) | 700 | 1.2 |
| H3 | 1.5rem (24px) | 600 | 1.3 |
| H4 | 1.25rem (20px) | 600 | 1.4 |
| Body | 1rem (16px) | 400 | 1.6 |
| Small | 0.875rem (14px) | 400 | 1.5 |
| Caption | 0.75rem (12px) | 500 | 1.5 |

### Appendix C: Breakpoints

| Name | Width | Description |
|------|-------|-------------|
| sm | 640px | Large phones |
| md | 768px | Tablets |
| lg | 1024px | Small laptops |
| xl | 1280px | Desktops |
| 2xl | 1536px | Large screens |

### Appendix D: Browser Support

| Browser | Minimum Version | Notes |
|---------|-----------------|-------|
| Chrome | 90+ | Full support |
| Firefox | 88+ | Full support |
| Safari | 14+ | Full support |
| Edge | 90+ | Full support |
| iOS Safari | 14+ | Full support |
| Chrome Android | 90+ | Full support |

### Appendix E: Third-Party Services

| Service | Purpose | Cost |
|---------|---------|------|
| Vercel | Hosting | Free tier |
| GitHub | Repository | Free |
| Namecheap | Domain | ~$12/year |
| Unsplash | Stock images | Free |
| Lucide | Icons | Free |

---

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-04-03 | Initial stub |
| 2.0 | 2026-04-04 | Complete specification with nanovms-level detail |

---

*Document version: 2.0*  
*Last updated: 2026-04-04*  
*Author: @koosha*
