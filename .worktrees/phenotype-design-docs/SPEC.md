# Specification Overview -- @kooshapari/design

**Package:** `@kooshapari/design`
**Version:** 1.0.1
**Repository:** https://github.com/KooshaPari/phenotype-design
**Worktree:** `spec-docs` (branch: `chore/spec-docs`)
**Last Updated:** 2026-03-26
**Status:** Implementation complete; PR pending merge

## Project Vision

`@kooshapari/design` is the shared design-token and theme library for the Phenotype ecosystem.
It provides the Keycap color palette (teal-dark aesthetic), CSS custom properties, framework-
agnostic component styles, a full VitePress theme integration, and a W3C DTCG JSON token source
so every Phenotype project achieves visual consistency without coupling to a UI framework.

## Specification Document Index

| Document | Purpose | Status |
|----------|---------|--------|
| `PRD.md` | Epics, user stories, acceptance criteria | ✅ Complete (168 lines) |
| `FUNCTIONAL_REQUIREMENTS.md` | SHALL statements traced to epics | ✅ Complete (183 lines) |
| `ADR.md` | Architecture decision records | ✅ Complete |
| `PLAN.md` | Phased WBS with DAG | ✅ Complete |
| `CLAUDE.md` | AI agent instructions | ✅ Complete (739 lines) |
| `this file` | High-level spec overview | ✅ Complete |

## Epic Summary

| Epic | Feature Count | Status |
|------|--------------|--------|
| **E1: Design Tokens** | 3 features (E1.1-E1.3) | ✅ Complete |
| **E2: Component Styles** | 4 features (E2.1-E2.4) | ✅ Complete |
| **E3: VitePress Integration** | 2 features (E3.1-E3.2) | ✅ Complete |

## Functional Requirements Summary

| Category | FR Count | Status |
|----------|----------|--------|
| FR-TOK (Design Tokens) | 10 | ✅ Complete |
| FR-CMP (Component Styles) | 10 | ✅ Complete |
| FR-VPR (VitePress Integration) | 7 | ✅ Complete |
| **Total** | **27** | **✅ All complete** |

## Implementation Status

### E1: Design Tokens

| Feature | Files | Status |
|---------|-------|--------|
| E1.1: CSS Custom Property Palette | `css/keycap-palette.css` | ✅ Implemented |
| E1.2: W3C DTCG Token Source | `tokens/keycap.json` | ✅ Implemented |
| E1.3: TypeScript Token Constants | `src/tokens.ts`, `dist/tokens.js` | ✅ Implemented |

### E2: Component Styles

| Feature | Files | Status |
|---------|-------|--------|
| E2.1: Layer Badges (5-layer taxonomy) | `css/components.css` | ✅ Implemented |
| E2.2: Status Badges | `css/components.css` | ✅ Implemented |
| E2.3: Doc-Type Card Grid | `css/components.css` | ✅ Implemented |
| E2.4: Pipeline Visualization | `css/components.css` | ✅ Implemented |

### E3: VitePress Integration

| Feature | Files | Status |
|---------|-------|--------|
| E3.1: VitePress Theme CSS | `css/vitepress-theme.css` | ✅ Implemented |
| E3.2: VitePress Config Helper | `src/vitepress.ts`, `dist/vitepress.js` | ✅ Implemented |

## Package Exports

| Export Path | Artifact | Purpose |
|---|---|---|
| `.` | `dist/index.js` | Re-exports `keycap` and `KeycapTokens` |
| `./tokens` | `dist/tokens.js` | Programmatic token access |
| `./vitepress` | `dist/vitepress.js` | `vitepressConfig` and `vitepressMarkdownTheme` config helpers |
| `./css/keycap-palette.css` | CSS | Color tokens only, light + dark |
| `./css/vitepress-theme.css` | CSS | Full VitePress theme (palette + components) |
| `./css/components.css` | CSS | Layer/status badges, doc-type cards, pipeline stages |

## Token Architecture

```
tokens/keycap.json              W3C DTCG source of truth
  |                             29 color tokens + 2 font families
  +-- src/tokens.ts             TypeScript const mirror (keycap + KeycapTokens type)
  |
  +-- css/keycap-palette.css    CSS custom properties (--kc-* vars, light/dark/media-query)
       |
       +-- css/vitepress-theme.css   VitePress --vp-* remapping + nav/sidebar/hero overrides
       +-- css/components.css        Layer badges, status badges, doc-type cards, pipeline
```

## Keycap Color Palette

| Role | Dark | Light |
|------|------|-------|
| Background | `#090a0c` | `#f8f9fa` |
| Text | `#f6f5f5` | `#1a1c1e` |
| Accent | `#7ebab5` | `#4a9c97` |
| Slate | `#353a40` | `#e8eaed` |

All combinations meet WCAG AA contrast (4.5:1 minimum).

## Next Steps

1. **Merge PR**: `chore/spec-docs` → `main` (pending review)
2. **Publish**: `bun publish` to npm registry
3. **Consume**: Update `@phenotype/design` consumers to use new package name

## References

- Source repo: `KooshaPari/phenotype-design`
- Installation: `bun add @kooshapari/design`
- License: MIT
