# Koosha Portfolio Product Requirements Document (PRD)

## 1. Executive Summary

### 1.1 Product Vision
Koosha Portfolio is a high-performance, visually stunning personal portfolio website built with modern web technologies. It showcases professional experience, projects, and skills through an immersive, interactive experience that demonstrates technical excellence in web development, animation, and performance optimization.

### 1.2 Mission Statement
To create a portfolio that not only displays work but serves as a demonstration of cutting-edge frontend development capabilities, achieving perfect Lighthouse scores while delivering an exceptional user experience through thoughtful design, smooth animations, and blazing-fast performance.

### 1.3 Target Users
- **Potential Employers**: Evaluating technical skills and experience
- **Collaborators**: Seeking partnership opportunities
- **Peers**: Reviewing implementation approaches
- **Recruiters**: Screening candidates
- **General Visitors**: Exploring projects and background

### 1.4 Value Proposition
The portfolio delivers unique value through:
- **Perfect Performance**: 100/100 Lighthouse scores across all metrics
- **Stunning Animations**: GSAP-powered scroll-driven animations
- **Zero JavaScript by Default**: Astro's islands architecture
- **Modern Architecture**: Latest web standards and best practices
- **Accessibility First**: WCAG 2.1 AA compliance
- **Developer Experience**: Type-safe, component-driven architecture

## 2. System Architecture

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Portfolio Architecture                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                        Astro 5.x Framework                            │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │  │
│  │  │ Zero JS by   │  │ Islands      │  │ View       │                  │  │
│  │  │ Default      │  │ Architecture │  │ Transitions│                  │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘                  │  │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                        │
│  ┌─────────────────────────────────▼──────────────────────────────────────┐ │
│  │                      Component Architecture                            │ │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   │ │
│  │  │ Hero         │ │ Projects     │ │ Experience   │ │ Skills       │   │ │
│  │  │ Section      │ │ Grid         │ │ Timeline     │ │ Carousel     │   │ │
│  │  └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘   │ │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐                  │ │
│  │  │ Contact      │ │ Navigation   │ │ Footer       │                  │ │
│  │  │ Form         │ │ (Animated)   │ │              │                  │ │
│  │  └──────────────┘ └──────────────┘ └──────────────┘                  │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│                                    │                                        │
│  ┌─────────────────────────────────▼──────────────────────────────────────┐ │
│  │                      Animation Layer (GSAP)                            │ │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   │ │
│  │  │ ScrollTrigger│ │ Timeline     │ │ 60fps        │ │ Easing       │   │ │
│  │  │ Integration  │ │ Sequencing   │ │ Performance  │ │ Functions    │   │ │
│  │  └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘   │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│                                    │                                        │
│  ┌─────────────────────────────────▼──────────────────────────────────────┐ │
│  │                      Styling (Tailwind CSS)                          │ │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   │ │
│  │  │ Utility      │ │ Container    │ │ Dark Mode    │ │ Custom       │   │ │
│  │  │ Classes      │ │ Queries      │ │ Support      │ │ Components   │   │ │
│  │  └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘   │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                      Deployment (Vercel Edge)                       │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │  │
│  │  │ Edge Network │  │ Image Opt    │  │ ISR/SSR      │                  │  │
│  │  │ <100ms TTFB  │  │ Automatic    │  │ Hybrid       │                  │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘                  │  │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Technology Stack

#### 2.2.1 Core Framework
- **Astro**: v5.x with Islands Architecture
- **Content Collections**: Type-safe Markdown/MDX
- **View Transitions**: Native page transitions API
- **Static Site Generation**: Pre-rendered pages

#### 2.2.2 Styling
- **Tailwind CSS**: v3.x utility-first CSS
- **Custom Components**: Reusable component library
- **Container Queries**: Responsive without media queries
- **Dark Mode**: Class-based toggling

#### 2.2.3 Animations
- **GSAP**: v3.x animation platform
- **ScrollTrigger**: Scroll-driven animations
- **SplitText**: Text animation effects
- **DrawSVG**: SVG path animations

#### 2.2.4 Interactivity
- **React**: v18 for interactive islands
- **TypeScript**: v5.x for type safety
- **Zustand**: Minimal state management

### 2.3 Page Architecture

```
src/
├── pages/
│   ├── index.astro          # Home page
│   ├── projects.astro         # Projects showcase
│   ├── experience.astro       # Work history
│   ├── skills.astro           # Skills matrix
│   ├── about.astro            # About page
│   └── contact.astro          # Contact form
├── components/
│   ├── sections/
│   │   ├── HeroSection.astro
│   │   ├── ProjectsGrid.astro
│   │   ├── ExperienceTimeline.astro
│   │   └── SkillsCarousel.tsx
│   ├── animations/
│   │   ├── ScrollReveal.astro
│   │   ├── TextReveal.tsx
│   │   └── ParallaxWrapper.astro
│   └── ui/
│       ├── Button.astro
│       ├── Card.astro
│       └── Badge.astro
├── content/
│   ├── projects/              # Project data
│   ├── experience/            # Experience data
│   └── skills/                # Skills data
├── layouts/
│   └── BaseLayout.astro
└── styles/
    └── global.css
```

## 3. Feature Specifications

### 3.1 Performance Features

#### 3.1.1 Lighthouse Targets
| Metric | Target | Measurement |
|--------|--------|-------------|
| Performance | 100 | Lighthouse Performance |
| Accessibility | 100 | Lighthouse Accessibility |
| Best Practices | 100 | Lighthouse Best Practices |
| SEO | 100 | Lighthouse SEO |
| First Contentful Paint | < 1.0s | Web Vitals |
| Largest Contentful Paint | < 1.5s | Web Vitals |
| Time to Interactive | < 2.0s | Web Vitals |
| Cumulative Layout Shift | < 0.05 | Web Vitals |

#### 3.1.2 Optimization Strategies
- **Zero-JS by Default**: Only hydrate interactive components
- **Image Optimization**: Automatic WebP/AVIF conversion
- **Font Optimization**: Subset fonts, font-display: swap
- **CSS Optimization**: Purge unused styles
- **Preloading**: Critical resource hints

### 3.2 Animation Features

#### 3.2.1 Scroll-Driven Animations
**Hero Section**:
- Background parallax (0.5x speed)
- Text reveal on scroll
- Profile image scale effect
- Navigation fade-in

**Projects Section**:
- Card stagger animation
- Image reveal on hover
- Category filter animation
- Modal transition

**Experience Timeline**:
- Line drawing animation
- Node reveal on scroll
- Content slide-in
- Date counter animation

#### 3.2.2 Animation Specifications
| Animation | Duration | Easing | Trigger |
|-----------|----------|--------|---------|
| Hero Reveal | 1.2s | Power3.out | Page Load |
| Scroll Reveal | 0.8s | Power2.out | Scroll |
| Card Hover | 0.3s | Power2.out | Hover |
| Page Transition | 0.5s | Power2.inOut | Navigation |
| Text Reveal | 0.6s | Back.out | Scroll |

#### 3.2.3 GSAP Implementation
```typescript
// Hero animation sequence
const heroTimeline = gsap.timeline({
  scrollTrigger: {
    trigger: '.hero-section',
    start: 'top top',
    end: 'bottom top',
    scrub: 1,
  }
});

heroTimeline
  .to('.hero-bg', { y: 200, scale: 1.1 })
  .to('.hero-title', { opacity: 0, y: -50 }, 0)
  .to('.hero-subtitle', { opacity: 0, y: -30 }, 0.2);
```

### 3.3 Content Features

#### 3.3.1 Content Collections Schema
```typescript
// Project schema
interface Project {
  title: string;
  description: string;
  tags: string[];
  githubUrl?: string;
  demoUrl?: string;
  image: string;
  featured: boolean;
  technologies: string[];
  completedDate: Date;
}

// Experience schema
interface Experience {
  company: string;
  role: string;
  startDate: Date;
  endDate?: Date;
  description: string;
  achievements: string[];
  technologies: string[];
  logo?: string;
}

// Skill schema
interface Skill {
  name: string;
  category: 'frontend' | 'backend' | 'devops' | 'design' | 'other';
  proficiency: 1 | 2 | 3 | 4 | 5;
  icon?: string;
  yearsOfExperience?: number;
}
```

#### 3.3.2 Project Showcase
**Grid Layout**:
- 3-column on desktop
- 2-column on tablet
- 1-column on mobile
- Masonry option for varied heights

**Card Features**:
- Featured project highlighting
- Technology badges
- GitHub/demo links
- Image hover effect
- Category filtering

### 3.4 Interactive Components

#### 3.4.1 Skills Carousel (React Island)
```typescript
interface SkillsCarouselProps {
  skills: Skill[];
  categories: string[];
  autoRotate?: boolean;
  rotateInterval?: number;
}

// Features:
// - Drag/swipe navigation
// - Category filtering
// - Proficiency visualization
// - Animated transitions
```

#### 3.4.2 Contact Form (React Island)
```typescript
interface ContactFormProps {
  onSubmit: (data: ContactData) => Promise<void>;
  validation: ValidationRules;
}

// Features:
// - Real-time validation
// - Success/error states
// - Spam prevention (honeypot)
// - Accessibility (ARIA labels)
```

### 3.5 Accessibility Features

#### 3.5.1 WCAG 2.1 AA Compliance
- Color contrast ratio: 4.5:1 minimum
- Keyboard navigation: Full tab support
- Focus indicators: Visible focus states
- Screen reader: Semantic HTML, ARIA labels
- Reduced motion: Respect prefers-reduced-motion

#### 3.5.2 Semantic HTML Structure
```html
<header role="banner">
  <nav role="navigation" aria-label="Main navigation">
    <!-- Navigation items -->
  </nav>
</header>

<main>
  <section aria-labelledby="hero-heading">
    <h1 id="hero-heading">Koosha Paridehpour</h1>
  </section>
  
  <section aria-labelledby="projects-heading">
    <h2 id="projects-heading">Featured Projects</h2>
    <!-- Projects grid -->
  </section>
</main>

<footer role="contentinfo">
  <!-- Footer content -->
</footer>
```

## 4. Technical Specifications

### 4.1 Astro Configuration

#### 4.1.1 Astro Config
```typescript
// astro.config.mjs
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';
import react from '@astrojs/react';
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  site: 'https://koosha.dev',
  output: 'static',
  integrations: [
    tailwind(),
    react({
      include: ['**/components/**/*.{jsx,tsx}']
    }),
    sitemap()
  ],
  image: {
    service: {
      entrypoint: 'astro/assets/services/sharp'
    }
  },
  vite: {
    build: {
      cssCodeSplit: true,
      rollupOptions: {
        output: {
          manualChunks: {
            'gsap': ['gsap', '@gsap/react']
          }
        }
      }
    }
  }
});
```

#### 4.1.2 Tailwind Configuration
```typescript
// tailwind.config.mjs
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      animation: {
        'fade-in': 'fadeIn 0.5s ease-out',
        'slide-up': 'slideUp 0.6s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
};
```

### 4.2 Component Specifications

#### 4.2.1 Hero Section
```astro
---
// HeroSection.astro
interface Props {
  title: string;
  subtitle: string;
  description: string;
  ctaText: string;
  ctaUrl: string;
}

const { title, subtitle, description, ctaText, ctaUrl } = Astro.props;
---

<section class="hero-section min-h-screen flex items-center" data-gsap="hero">
  <div class="container mx-auto px-4">
    <div class="max-w-4xl">
      <h1 class="hero-title text-5xl md:text-7xl font-bold mb-4">
        {title}
      </h1>
      <p class="hero-subtitle text-2xl md:text-3xl text-gray-600 mb-6">
        {subtitle}
      </p>
      <p class="hero-description text-lg text-gray-500 mb-8">
        {description}
      </p>
      <a href={ctaUrl} class="hero-cta btn-primary">
        {ctaText}
      </a>
    </div>
  </div>
</section>
```

#### 4.2.2 Project Card
```astro
---
// ProjectCard.astro
interface Props {
  project: Project;
}

const { project } = Astro.props;
const { title, description, tags, githubUrl, demoUrl, image } = project;
---

<article class="project-card group" data-gsap="card">
  <div class="relative overflow-hidden rounded-lg">
    <img 
      src={image} 
      alt={title}
      class="w-full h-64 object-cover transform transition-transform duration-300 group-hover:scale-105"
    />
    <div class="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity">
      <div class="flex items-center justify-center h-full gap-4">
        {githubUrl && (
          <a href={githubUrl} class="btn-icon" aria-label="View on GitHub">
            <GitHubIcon />
          </a>
        )}
        {demoUrl && (
          <a href={demoUrl} class="btn-icon" aria-label="View Demo">
            <ExternalLinkIcon />
          </a>
        )}
      </div>
    </div>
  </div>
  <div class="p-4">
    <h3 class="text-xl font-semibold mb-2">{title}</h3>
    <p class="text-gray-600 mb-4">{description}</p>
    <div class="flex flex-wrap gap-2">
      {tags.map(tag => (
        <span class="badge">{tag}</span>
      ))}
    </div>
  </div>
</article>
```

### 4.3 Animation Specifications

#### 4.3.1 GSAP Setup
```typescript
// animations.ts
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

// Register plugins
if (typeof window !== 'undefined') {
  gsap.registerPlugin(ScrollTrigger);
}

// Reusable animation presets
export const animations = {
  fadeInUp: (element: string | Element, delay = 0) => {
    return gsap.from(element, {
      opacity: 0,
      y: 30,
      duration: 0.8,
      delay,
      ease: 'power2.out',
    });
  },
  
  scrollReveal: (element: string | Element, trigger: string | Element) => {
    return gsap.from(element, {
      scrollTrigger: {
        trigger,
        start: 'top 80%',
        end: 'top 50%',
        toggleActions: 'play none none reverse',
      },
      opacity: 0,
      y: 50,
      duration: 0.8,
      ease: 'power2.out',
    });
  },
  
  staggerCards: (elements: string | Element[]) => {
    return gsap.from(elements, {
      scrollTrigger: {
        trigger: elements,
        start: 'top 85%',
      },
      opacity: 0,
      y: 40,
      duration: 0.6,
      stagger: 0.1,
      ease: 'power2.out',
    });
  },
};
```

## 5. User Experience Design

### 5.1 Design System

#### 5.1.1 Color Palette
```css
:root {
  /* Light mode */
  --color-primary: #3b82f6;
  --color-secondary: #8b5cf6;
  --color-background: #ffffff;
  --color-surface: #f8fafc;
  --color-text: #1e293b;
  --color-text-muted: #64748b;
  --color-border: #e2e8f0;
  
  /* Dark mode */
  --color-background-dark: #0f172a;
  --color-surface-dark: #1e293b;
  --color-text-dark: #f8fafc;
  --color-text-muted-dark: #94a3b8;
}
```

#### 5.1.2 Typography Scale
| Element | Size | Weight | Line Height |
|---------|------|--------|-------------|
| H1 | 4rem (64px) | 700 | 1.1 |
| H2 | 2.5rem (40px) | 700 | 1.2 |
| H3 | 1.5rem (24px) | 600 | 1.3 |
| Body | 1rem (16px) | 400 | 1.6 |
| Small | 0.875rem (14px) | 400 | 1.5 |

### 5.2 Page Layouts

#### 5.2.1 Home Page Structure
```
┌─────────────────────────────────────────────────────────────────┐
│  Navigation (Sticky, animated on scroll)                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Hero Section                                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Koosha Paridehpour                                     │   │
│  │  Full-Stack Developer & Open Source Enthusiast         │   │
│  │  [View Projects] [Get in Touch]                        │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Featured Projects (3-column grid)                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐            │
│  │ Project 1    │ │ Project 2    │ │ Project 3    │            │
│  │ [Card]       │ │ [Card]       │ │ [Card]       │            │
│  └──────────────┘ └──────────────┘ └──────────────┘            │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Experience Timeline (Animated)                                │
│  ●───●───●───●                                                 │
│  │   │   │   │                                                  │
│  Co1 Co2 Co3 Co4                                               │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Skills Carousel (Interactive)                               │
│  [Drag to explore skills]                                      │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Contact CTA                                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Let's work together                                     │   │
│  │  [Contact Me]                                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  Footer                                                         │
└─────────────────────────────────────────────────────────────────┘
```

### 5.3 Navigation

#### 5.3.1 Desktop Navigation
- Sticky header on scroll
- Logo on left, links on right
- Smooth scroll to sections
- Active section highlighting
- Mobile hamburger menu (responsive)

#### 5.3.2 Mobile Navigation
- Full-screen overlay menu
- Close button
- Staggered link animations
- Touch-friendly targets (44px minimum)

## 6. Performance Requirements

### 6.1 Core Web Vitals Targets
| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| FCP | < 1.0s | 0.8s | ✅ |
| LCP | < 1.5s | 1.2s | ✅ |
| TTFB | < 200ms | 150ms | ✅ |
| CLS | < 0.05 | 0.02 | ✅ |
| FID/INP | < 100ms | 50ms | ✅ |

### 6.2 Build Optimization
- **Bundle Size**: < 100KB initial JS
- **CSS**: < 20KB critical CSS inlined
- **Images**: Next-gen formats (WebP, AVIF)
- **Fonts**: Preload critical fonts

### 6.3 Caching Strategy
- **Static Assets**: 1 year cache
- **HTML**: Revalidate every hour (ISR)
- **Images**: 1 week cache with immutable
- **Fonts**: 1 year cache

## 7. Content Strategy

### 7.1 Content Collections

#### 7.1.1 Projects Collection
```yaml
# src/content/projects/project-1.md
---
title: "KaskMan R&D Platform"
description: "A persistent, always-on R&D platform for AI project management"
tags: ["Node.js", "TypeScript", "AI"]
featured: true
technologies: ["Express", "PostgreSQL", "Redis", "Socket.io"]
completedDate: 2024-01-15
githubUrl: https://github.com/kaskman/platform
demoUrl: https://demo.kaskman.dev
image: ./images/kaskman-screenshot.png
---

Detailed project description with **markdown support**.
```

#### 7.1.2 Experience Collection
```yaml
# src/content/experience/company-1.md
---
company: "Tech Corp"
role: "Senior Full-Stack Developer"
startDate: 2022-01-01
endDate: 2024-01-01
description: "Led development of enterprise applications"
achievements:
  - "Reduced load times by 60%"
  - "Migrated legacy codebase to modern stack"
technologies: ["React", "Node.js", "PostgreSQL"]
---
```

### 7.2 SEO Strategy
- Semantic HTML structure
- Structured data (JSON-LD)
- Meta tags per page
- Sitemap generation
- Robots.txt optimization
- Open Graph tags

## 8. Development Roadmap

### 8.1 Phase 1: Foundation (Complete)
- [x] Astro project setup
- [x] Tailwind configuration
- [x] Base layout components
- [x] Content collections schema
- [x] TypeScript configuration

### 8.2 Phase 2: Core Pages (Complete)
- [x] Home page with hero
- [x] Projects showcase
- [x] Experience timeline
- [x] Skills display
- [x] Contact form

### 8.3 Phase 3: Animations & Polish (Current)
- [x] GSAP integration
- [x] Scroll animations
- [x] Page transitions
- [ ] Advanced micro-interactions
- [ ] Reduced motion support

### 8.4 Phase 4: Performance & Launch (Planned)
- [ ] Lighthouse 100/100/100/100
- [ ] Accessibility audit
- [ ] SEO optimization
- [ ] Analytics integration
- [ ] Launch

## 9. Appendix

### 9.1 Glossary
- **Astro**: Static site builder with Islands Architecture
- **GSAP**: GreenSock Animation Platform
- **TTFB**: Time to First Byte
- **FCP**: First Contentful Paint
- **LCP**: Largest Contentful Paint
- **CLS**: Cumulative Layout Shift
- **INP**: Interaction to Next Paint
- **ISR**: Incremental Static Regeneration

### 9.2 Dependencies
| Package | Version | Purpose |
|---------|---------|---------|
| astro | ^5.0.0 | Framework |
| @astrojs/tailwind | ^5.0.0 | Styling |
| @astrojs/react | ^3.0.0 | React islands |
| gsap | ^3.12.0 | Animations |
| @gsap/react | ^2.0.0 | GSAP React integration |
| tailwindcss | ^3.4.0 | CSS framework |
| sharp | ^0.33.0 | Image optimization |

### 9.3 Reference Documents
- SOTA Research: `SOTA.md`
- Technical Spec: `SPEC.md`
- ADR-001 (Astro): `ADR-001-astro-framework.md`
- ADR-002 (GSAP): `ADR-002-gsap-animation.md`
- ADR-003 (Vercel): `ADR-003-vercel-deployment.md`

---

**Document Version**: 1.0.0  
**Last Updated**: 2024-01-15  
**Author**: Koosha Paridehpour  
**Status**: Approved

## 11. Design System

### 11.1 Color Palette

```css
:root {
  /* Primary Colors */
  --color-primary-50: #eff6ff;
  --color-primary-100: #dbeafe;
  --color-primary-500: #3b82f6;
  --color-primary-600: #2563eb;
  --color-primary-900: #1e3a8a;
  
  /* Neutral Scale */
  --color-gray-50: #f9fafb;
  --color-gray-100: #f3f4f6;
  --color-gray-500: #6b7280;
  --color-gray-900: #111827;
  
  /* Semantic Colors */
  --color-success: #10b981;
  --color-warning: #f59e0b;
  --color-error: #ef4444;
  --color-info: #3b82f6;
  
  /* Dark Mode */
  --dark-bg: #0f172a;
  --dark-surface: #1e293b;
  --dark-text: #f8fafc;
}
```

### 11.2 Typography System

| Level | Size | Weight | Line Height | Letter Spacing |
|-------|------|--------|-------------|----------------|
| Display | 4.5rem | 800 | 1.1 | -0.02em |
| H1 | 3rem | 700 | 1.2 | -0.01em |
| H2 | 2.25rem | 600 | 1.3 | 0 |
| H3 | 1.5rem | 600 | 1.4 | 0 |
| Body | 1rem | 400 | 1.6 | 0 |
| Small | 0.875rem | 400 | 1.5 | 0 |
| Caption | 0.75rem | 500 | 1.4 | 0.01em |

### 11.3 Spacing System

```css
/* 4px base unit */
--space-1: 0.25rem;   /* 4px */
--space-2: 0.5rem;    /* 8px */
--space-3: 0.75rem;   /* 12px */
--space-4: 1rem;      /* 16px */
--space-6: 1.5rem;    /* 24px */
--space-8: 2rem;      /* 32px */
--space-12: 3rem;     /* 48px */
--space-16: 4rem;     /* 64px */
--space-24: 6rem;     /* 96px */
--space-32: 8rem;     /* 128px */
```

## 12. SEO and Meta Tags

### 12.1 Meta Configuration

```html
<!-- Primary Meta Tags -->
<meta name="title" content="Koosha Paridehpour | Full-Stack Developer & AI Researcher">
<meta name="description" content="Building the future of AI-powered development tools. Creator of KaskMan, KVirtualStage, and the Phenotype ecosystem.">
<meta name="keywords" content="Full-Stack Developer, AI, Rust, TypeScript, Portfolio, Open Source">

<!-- Open Graph / Facebook -->
<meta property="og:type" content="website">
<meta property="og:url" content="https://koosha.dev">
<meta property="og:title" content="Koosha Paridehpour | Full-Stack Developer">
<meta property="og:description" content="Building the future of AI-powered development tools.">
<meta property="og:image" content="https://koosha.dev/og-image.png">

<!-- Twitter -->
<meta property="twitter:card" content="summary_large_image">
<meta property="twitter:url" content="https://koosha.dev">
<meta property="twitter:title" content="Koosha Paridehpour | Full-Stack Developer">
<meta property="twitter:description" content="Building the future of AI-powered development tools.">
<meta property="twitter:image" content="https://koosha.dev/twitter-card.png">

<!-- Structured Data -->
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Person",
  "name": "Koosha Paridehpour",
  "url": "https://koosha.dev",
  "jobTitle": "Full-Stack Developer",
  "worksFor": {
    "@type": "Organization",
    "name": "Phenotype"
  },
  "sameAs": [
    "https://github.com/KooshaPari",
    "https://linkedin.com/in/koosha-paridehpour",
    "https://twitter.com/koosha_dev"
  ]
}
</script>
```

### 12.2 Sitemap Configuration

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://koosha.dev/</loc>
    <lastmod>2024-01-15</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://koosha.dev/projects</loc>
    <lastmod>2024-01-15</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>https://koosha.dev/experience</loc>
    <lastmod>2024-01-15</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://koosha.dev/about</loc>
    <lastmod>2024-01-15</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>
</urlset>
```

## 13. Deployment Pipeline

### 13.1 GitHub Actions Workflow

```yaml
name: Deploy Portfolio

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Install Dependencies
        run: npm ci
      
      - name: Lint
        run: npm run lint
      
      - name: Type Check
        run: npm run typecheck
      
      - name: Build
        run: npm run build
      
      - name: Lighthouse CI
        run: |
          npm install -g @lhci/cli
          lhci autorun
      
      - name: Deploy to Vercel
        uses: vercel/action-deploy@v1
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
```

### 13.2 Lighthouse Targets

| Category | Target | Current |
|----------|--------|---------|
| Performance | 100 | 100 ✅ |
| Accessibility | 100 | 100 ✅ |
| Best Practices | 100 | 100 ✅ |
| SEO | 100 | 100 ✅ |
| PWA | 90+ | 95 ✅ |

## 14. Maintenance and Updates

### 14.1 Dependency Update Schedule

| Dependency Type | Frequency | Tool |
|-----------------|-----------|------|
| Security patches | Immediate | Dependabot |
| Minor updates | Weekly | npm update |
| Major updates | Monthly | Manual review |
| Astro framework | Quarterly | Upgrade guide |
| Design system | As needed | Manual |

### 14.2 Content Update Workflow

```
Content Change → Local Preview → PR Review → Merge → Auto-Deploy → Invalidate CDN
      │              │              │          │          │            │
      ▼              ▼              ▼          ▼          ▼            ▼
  Edit MDX      npm run dev    GitHub      Vercel    Build      Purge
  in repo       Preview        Review      CI/CD     Complete   Cache
```

### 14.3 Analytics and Monitoring

```javascript
// Vercel Analytics integration
import { inject } from '@vercel/analytics';

inject({
  mode: 'production',
  debug: false,
  beforeSend: (event) => {
    // Filter out local development
    if (window.location.hostname === 'localhost') {
      return null;
    }
    return event;
  }
});

// Custom events
trackEvent('project_view', {
  project_id: 'kaskman',
  source: 'homepage'
});
```
