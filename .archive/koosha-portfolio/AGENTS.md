# AGENTS.md — koosha-portfolio

## Project Overview

- **Name**: koosha-portfolio (Personal Portfolio Site)
- **Description**: Personal portfolio website with Astro, GSAP animations, and Vercel deployment
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio`
- **Language Stack**: Astro, TypeScript, GSAP, Tailwind CSS
- **Published**: Public (Vercel)

## Quick Start

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio

# Install dependencies
npm install

# Start development
npm run dev

# Build for production
npm run build
```

## Architecture

### Static Site

```
┌─────────────────────────────────────────────────────────────────┐
│                     Astro Framework                                │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │                    Islands Architecture                       │ │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐         │ │
│  │  │ Hero       │  │ Projects   │  │ Contact    │         │ │
│  │  │ Section    │  │ Grid       │  │ Form       │         │ │
│  │  │ (Animated) │  │ (Hydrated) │  │ (Static)   │         │ │
│  │  └────────────┘  └────────────┘  └────────────┘         │ │
│  └──────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   GSAP          │  │   Tailwind      │  │   Vercel        │ │
│  │   Animations    │  │   Styling       │  │   Hosting       │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Quality Standards

### Web Quality

- **Formatter**: Prettier
- **Linter**: ESLint
- **Build**: Astro check

## Git Workflow

### Branch Naming

Format: `<type>/<section>/<description>`

Examples:
- `content/projects/add-new-project`
- `fix/animations/smooth-scroll`
- `perf/images/optimize-loading`

## CLI Commands

```bash
npm run dev
npm run build
npm run preview
```

## Resources

- [Astro Docs](https://docs.astro.build/)
- [GSAP](https://greensock.com/gsap/)
- [Phenotype Registry](https://github.com/KooshaPari/phenotype-registry)

## Agent Notes

**Critical Details:**
- Static generation preferred
- Client-side islands for interactivity
- Image optimization
- SEO meta tags

**Known Gotchas:**
- GSAP requires careful cleanup
- Animation performance on mobile
- Hydration mismatches
