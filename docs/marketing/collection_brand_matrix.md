# Phenotype Collections Brand Matrix

This document consolidates visual identity, typography, and brand voice for the five Phenotype named collections.

## Collection Overview

| Collection | Tagline | Accent Color | Neutral | Logo | Status |
|-----------|---------|--------------|---------|------|--------|
| Sidekick | Reliability at scale | #F97316 | #6B7280 | assets/logo-placeholder.svg | Placeholder |
| Eidolon | Invisible presence, visible impact | #4338CA | #6B7280 | assets/logo-placeholder.svg | Placeholder |
| Paginary | Adaptive, intelligent page layouts | #84B08E | #6B7280 | assets/logo-placeholder.svg (cream bg) | Placeholder |
| Observably | See everything, understand it all | #06B6D4 | #6B7280 | assets/logo-placeholder.svg | Placeholder |
| Stashly | Cache, stash, retrieve—instantly | #14B8A6 | #6B7280 | assets/logo-placeholder.svg | Placeholder |

## Color Palette Reference

### Sidekick — Warm Orange
- **Primary Accent**: #F97316 (Orange)
- **Neutral**: #6B7280 (Gray)
- **Use Case**: Energy, approachability, speed, distributed task orchestration
- **Hex Codes**: `#F97316` (accent), `#6B7280` (neutral body text)

### Eidolon — Deep Indigo
- **Primary Accent**: #4338CA (Indigo)
- **Neutral**: #6B7280 (Gray)
- **Use Case**: Intelligence, mystery, quiet sophistication, agent-driven systems
- **Hex Codes**: `#4338CA` (accent), `#6B7280` (neutral body text)

### Paginary — Sage Green + Paper Cream
- **Primary Accent**: #84B08E (Sage Green)
- **Background**: #F5F0E8 (Warm Cream)
- **Neutral**: #6B7280 (Gray)
- **Use Case**: Natural, organic layouts, design-conscious composition, adaptive pages
- **Hex Codes**: `#84B08E` (accent), `#F5F0E8` (background), `#6B7280` (neutral text)

### Observably — Cyan
- **Primary Accent**: #06B6D4 (Cyan)
- **Neutral**: #6B7280 (Gray)
- **Use Case**: Clarity, visibility, precision, light cutting through fog
- **Hex Codes**: `#06B6D4` (accent), `#6B7280` (neutral body text)

### Stashly — Teal
- **Primary Accent**: #14B8A6 (Teal)
- **Neutral**: #6B7280 (Gray)
- **Use Case**: Freshness, flow, responsive performance, motion and fluidity
- **Hex Codes**: `#14B8A6` (accent), `#6B7280` (neutral body text)

## Typography Standards

All collections follow consistent typography guidelines:

### Fonts

| Role | Font Family | Use Case | Fallback |
|------|------------|----------|----------|
| **Code/Technical UI** | IBM Plex Mono, Roboto Mono | Monospace, configuration, API docs | system monospace |
| **Body/Labels** | Inter, Helvetica Neue | Prose, UI labels, navigation | system sans-serif |

### Hierarchy

- **Headlines (H1-H2)**: Bold weight, 2-4rem size, accent color optional
- **Body Text**: Regular weight, 1rem base size, neutral color (#6B7280)
- **Labels/UI**: Regular weight, 0.875-1rem size, neutral color
- **Code Blocks**: Monospace, 0.875rem, dark background, syntax highlighting

## Brand Voice & Tone

| Collection | Voice | Tone | Example Copy |
|-----------|-------|------|--------------|
| **Sidekick** | Clear, practical | Direct, reliable | "Orchestrate tasks at scale without operational burden." |
| **Eidolon** | Thoughtful, exploratory | Inviting, precise | "Autonomous agents as first-class citizens in your architecture." |
| **Paginary** | Precise, design-conscious | Sophisticated, balanced | "Intelligent page layouts that adapt to content and audience." |
| **Observably** | Technical, direct | Empowering, actionable | "See your entire distributed system. Understand it all." |
| **Stashly** | Energetic, efficiency-focused | Swift, elegant | "Multi-tier caching that moves as fast as your applications." |

## Visual Identity Patterns

### Logo Design (Placeholder)

All placeholder logos follow a consistent pattern:

1. **Format**: 256x256 SVG
2. **Design**: Collection initial letter in monospace, accent color, 96pt font weight
3. **Background**: Either white or warm cream (#F5F0E8 for Paginary)
4. **Accent Circle**: 10% opacity circle behind the letter
5. **Typography**: Placeholder text below initial in gray (#888)

**File Location**: Each collection has `assets/logo-placeholder.svg`

Example structure (Sidekick):
```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="256" height="256">
  <rect width="256" height="256" fill="#FFFFFF"/>
  <circle cx="128" cy="128" r="110" fill="#F97316" opacity="0.1"/>
  <text x="128" y="148" font-family="monospace" font-size="96" font-weight="700" text-anchor="middle" fill="#F97316">S</text>
  <text x="128" y="220" font-family="sans-serif" font-size="12" text-anchor="middle" fill="#888">Sidekick</text>
</svg>
```

### Next Steps for Designer

1. Replace `assets/logo-placeholder.svg` in each collection with production assets
2. Refine color palette based on brand direction (suggest maintaining core hex values)
3. Establish icon/symbol guidelines beyond the initial letter
4. Update BRAND.md files with production-ready specifications
5. Consider complementary secondary colors and gradients per collection

## Integration Guidance

### For Websites/Docs

- Embed placeholder logos at top of each collection README
- Use accent colors for CTAs, highlights, and interactive elements
- Maintain consistent typography across all VitePress sites
- Apply impeccable CSS baseline to all UI

### For Component Libraries

- Define design tokens for each collection's accent + neutral
- Create color schemes in design tools (Figma, etc.)
- Export Tailwind/CSS custom properties with collection-specific tokens
- Test contrast ratios for WCAG 2.1 AA compliance

### For Marketing

- Use collection taglines in meta descriptions
- Apply accent colors to social media graphics
- Ensure monospace use for technical content, sans-serif for prose
- Maintain consistent visual spacing and alignment

## Checklist: Placeholder → Production

- [ ] Custom logo designs created for each collection
- [ ] Brand guidelines document (per-collection BRAND.md) finalized
- [ ] Color palette validated for accessibility (WCAG AA minimum)
- [ ] Typography choices locked in (font families, weights, sizes)
- [ ] Icon library established (if beyond initial letter)
- [ ] Design tokens exported to CSS/Tailwind
- [ ] SVG logos exported in multiple formats (SVG, PNG, ICO)
- [ ] Logo usage guidelines documented (minimum sizes, clearspace, etc.)
- [ ] All README.md files updated with production logos
- [ ] Brand matrix updated and published

## References

- **Placeholder Logos**: `/repos/<Collection>/assets/logo-placeholder.svg`
- **Brand Specs**: `/repos/<Collection>/BRAND.md`
- **Impeccable CSS**: https://github.com/pbakaus/impeccable
- **Typography**: Inter (https://rsms.me/inter/), IBM Plex Mono (https://github.com/IBM/plex)

---

**Status**: Scaffolded placeholders, ready for designer handoff.
**Last Updated**: 2026-04-24
