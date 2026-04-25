# Design Audits: Wave-10 Backlog #17 Completion Report
**Date:** 2026-04-24 | **Auditor:** Claude (Haiku 4.5) | **Scope:** 4 user-facing sites

---

## Executive Summary

Completed Impeccable-style design audits on 4 Phenotype org user-facing sites. All audits used consistent 5-dimension scoring (Accessibility, Performance, Responsive Design, Theming, Anti-Patterns). **No fixes applied** — audits are documentation only, ready for targeted skill-based remediation.

**Audit Health Scores:**
- **phenotype-dev-hub** (Astro marketing): 10/20 — Acceptable (most critical: dark-only theming, unsafe Tailwind classes)
- **thegent/docs/site** (VitePress): 15/20 — Good (most critical: no brand colors, code contrast)
- **FocalPoint/docs-site** (VitePress + Mermaid): 12/20 — Acceptable (most critical: no brand identity, Mermaid performance)
- **hwLedger/docs-site** (VitePress): 15/20 — Good (most critical: no brand colors, generic fonts)

**Average Health Score:** 13/20

---

## Site-by-Site Summary

### 1. phenotype-dev-hub (Astro 5 + Tailwind CSS 4)
**Health Score: 10/20 → 14/20 (UPDATED)** | **Status:** P0 Design Issues RESOLVED

**Top 5 Issues (Updated 2026-04-24):**
1. [RESOLVED] ✅ Dynamic Tailwind class interpolation — replaced with typed `colorClasses` Record, literal class strings
2. [RESOLVED] ✅ Dark-only theme — added Tailwind `darkMode: 'class'` + localStorage-persistent theme toggle
3. [P1] Missing ARIA labels and semantic HTML — screen readers cannot navigate cards
4. [P1] No focus-visible indicators — keyboard users cannot see which element is focused
5. [P1] Status badges use color-only coding (green=stable, yellow=beta) — inaccessible to colorblind users

**P0 Resolution Summary:**
- Created `/src/lib/colorClasses.ts` with typed accent color map (all 9 collections/products)
- Updated `CollectionCard.astro` and `ProductCard.astro` to use literal class strings
- Added `ThemeProvider.astro` (initializes theme with system preference fallback)
- Added `ThemeToggle.astro` (sun/moon icon toggle with localStorage persistence)
- Updated `tailwind.config.mjs`: `darkMode: 'class'` + added missing color-900 shades
- Updated `astro.config.mjs`: configured Vite @ path alias for clean imports
- Light mode CSS overrides added to `Layout.astro` global styles
- Build verified: `bun run build` completed with **zero warnings**, all Tailwind classes preserved in output

**Critical Patterns:**
- Hard-coded dark theme throughout (bg-slate-800, text-white) — requires component edits for light mode
- Cards are generic Tailwind defaults (rounded-lg, border, shadow) — feels templated, not distinctive
- Emoji icons lack intentional design — renders differently across platforms

**Positive Findings:**
- Collection accent colors well-defined and cohesive in Tailwind config
- Responsive grid likely mobile-friendly (structure suggests proper breakpoints)
- Component architecture cleanly separated (ProductCard, CollectionCard, Hero)

**Recommended Action Path (Remaining Items):**
1. ~~`/normalize`~~ **DONE** — Dark/light mode toggle implemented with localStorage persistence
2. `/harden` — add ARIA labels (aria-label, aria-describedby), semantic heading hierarchy
3. `/polish` — add focus-visible rings to cards and buttons
4. `/clarify` — add text prefixes to status badges (✓ Stable, ⚠ Beta)
5. `/adapt` — verify/fix mobile touch targets (48px+ hit zones)
6. `/arrange` — unify spacing scale, create visual rhythm across sections
7. `/bolder` — add visual distinction to featured vs. standard products
8. `/typeset` — replace system-ui with distinctive display font

**Audit Location:** `/repos/apps/phenotype-dev-hub/docs/design_audit_2026_04_24.md`

---

### 2. thegent/docs/site (VitePress)
**Health Score: 15/20** | **Status:** Solid documentation structure, needs brand alignment

**Top 5 Issues:**
1. [P1] No Phenotype brand color integration — uses generic VitePress blue/gray
2. [P1] Code block contrast in dark mode untested — may violate WCAG AA
3. [P2] Body font is system-ui — generic, lacks personality
4. [P2] Sidebar navigation lacks visual hierarchy — all items feel equal weight
5. [P2] Search box styling is minimal — focus states could be more prominent

**Critical Patterns:**
- Relies entirely on VitePress defaults for colors, fonts, spacing
- Site could be any technical documentation — no thegent visual identity
- Typography scale doesn't leverage hierarchy (no display vs. body distinction)

**Positive Findings:**
- Excellent semantic HTML structure (proper heading hierarchy, landmarks, lists)
- Mobile responsive by default (sidebar collapses elegantly, content reflows)
- Clear documentation organization (Guide, Operations, Reference, API)
- Code blocks properly formatted with syntax highlighting and line numbers
- Local search is fast (no external dependencies)

**Recommended Action Path:**
1. `/colorize` — apply Phenotype brand colors to VitePress theme
2. `/normalize` — test and fix code block contrast in dark mode
3. `/typeset` — introduce distinctive font for headings or body
4. `/arrange` — enhance sidebar hierarchy with spacing and visual weight
5. `/polish` — improve search box focus states and visual prominence

**Audit Location:** `/repos/thegent/docs/site/design_audit_2026_04_24.md`

---

### 3. FocalPoint/docs-site (VitePress + Mermaid)
**Health Score: 12/20** | **Status:** Functional but lacks brand identity and performance optimization

**Top 5 Issues:**
1. [P1] No FocalPoint brand colors in theme — uses generic VitePress defaults
2. [P1] Mermaid diagrams lack alt text and accessibility descriptions — breaks screen readers
3. [P1] Mermaid library loads on every page (~500KB+) — bloats bundle for text-only documentation pages
4. [P2] Default Mermaid diagram styling lacks visual polish — uses generic node/connector styling
5. [P2] Stub pages (connectors, rules templates, rituals) are not clearly marked as WIP

**Critical Patterns:**
- VitePress theme completely uncustomized
- Mermaid plugin globally enabled without lazy loading or conditional loading
- Many pages are aspirational stubs without clear WIP labeling

**Positive Findings:**
- Excellent semantic HTML and structure
- Good OpenGraph/Twitter meta tags for social sharing and SEO
- Mermaid integration is working correctly (just needs optimization and styling)
- Navigation is clear and intuitive
- Responsive mobile-first design

**Recommended Action Path:**
1. `/colorize` — apply FocalPoint brand colors to VitePress theme
2. `/harden` — add alt text and accessible descriptions for all Mermaid diagrams
3. `/optimize` — lazy-load Mermaid library (only on pages with diagrams)
4. `/bolder` — customize Mermaid diagram styling with brand colors
5. `/clarify` — add WIP badges to stub pages with completion estimates
6. `/delight` — design and add FocalPoint logo/icon to header
7. `/typeset` — enhance heading hierarchy and typography

**Audit Location:** `/repos/FocalPoint/docs-site/design_audit_2026_04_24.md` (submodule)

---

### 4. hwLedger/docs-site (VitePress)
**Health Score: 15/20** | **Status:** Well-structured documentation, needs visual branding

**Top 5 Issues:**
1. [P1] No hwLedger brand colors in theme — uses generic VitePress blue/gray
2. [P2] Code block contrast in dark mode untested — may violate WCAG AA
3. [P2] Typography is generic system font — lacks intentional voice
4. [P2] Navigation hierarchy is unclear — no visual distinction between parent/child sections
5. [P3] Missing hwLedger logo or visual mark — text-only site title feels less premium

**Critical Patterns:**
- VitePress defaults applied throughout with zero customization
- Site has no visual connection to hwLedger brand (if BRAND.md exists)
- Typography uses system-ui everywhere

**Positive Findings:**
- Excellent semantic HTML (proper heading structure, good list organization)
- Mobile responsive by default with elegant sidebar collapse
- Clear navigation structure and logical section organization
- Code blocks properly formatted with syntax highlighting
- No performance issues detected

**Recommended Action Path:**
1. `/colorize` — apply hwLedger brand colors to VitePress theme (check BRAND.md)
2. `/normalize` — test and verify code block contrast in dark mode
3. `/typeset` — introduce distinctive fonts for headings or body
4. `/arrange` — enhance sidebar navigation hierarchy
5. `/delight` — design and add hwLedger logo/icon
6. `/audit` — verify improvements

**Audit Location:** `/repos/hwLedger/docs-site/design_audit_2026_04_24.md` (submodule)

---

## Cross-Site Patterns & Systemic Issues

### Pattern 1: VitePress Theme Not Customized (3 sites: thegent, FocalPoint, hwLedger)
All three VitePress sites use defaults with zero brand customization.
- **Impact:** Sites are indistinguishable from thousands of other VitePress docs
- **Solution:** Create `.vitepress/theme/custom.css` or `index.ts` with brand colors and fonts
- **Estimated effort:** 1-2 hours per site

### Pattern 2: Hard-Coded Dark Themes (2 sites: phenotype-dev-hub, documented in VitePress sites)
Dark-only or dark-first theming without light mode toggle.
- **Impact:** Violates accessibility principle of user control
- **Solution:** phenotype-dev-hub needs Tailwind `darkMode: ['class']` + toggle; VitePress sites need custom CSS with `@media (prefers-color-scheme: light)`
- **Estimated effort:** 2-3 hours per site

### Pattern 3: Missing Accessibility Labels (2 sites: phenotype-dev-hub critical, others moderate)
- ARIA labels missing or incomplete
- Color-only coding (status badges, icons)
- Focus indicators absent
- **Solution:** Add aria-label, aria-describedby; change status badges to include text; add focus-visible rings
- **Estimated effort:** 1-2 hours per site

### Pattern 4: Generic Typography (all 4 sites)
All sites use system-ui or default fonts. No distinctive typeface choices.
- **Impact:** Misses opportunity for brand personality
- **Solution:** Introduce distinctive display font (custom web font or specific fallback chain)
- **Estimated effort:** 1 hour per site (assuming font choice is already made)

### Pattern 5: Decorative vs. Functional Design (phenotype-dev-hub specific)
Hover blur circles, emoji icons, generic card grids — feel templated.
- **Impact:** Reduces perceived quality and distinctiveness
- **Solution:** Remove decorative elements; replace emoji with intentional icons; vary card layouts
- **Estimated effort:** 2-3 hours

---

## Recommended Remediation Roadmap

### Phase 1: Critical Fixes (High Impact, Medium Effort)
**Timeline:** 1-2 weeks | **Recommended skills:** `/normalize`, `/colorize`, `/harden`

1. **phenotype-dev-hub**
   - Fix dynamic Tailwind class interpolation (use CSS variables or lookup table)
   - Add light mode support (Tailwind + theme toggle)
   - Add ARIA labels and focus indicators
   - Expected: +3-4 points on health score (10 → 13-14)

2. **thegent, FocalPoint, hwLedger**
   - Apply brand colors to VitePress themes
   - Add light/dark mode support
   - Fix code block contrast in dark mode
   - Expected: +2-3 points per site (15→17-18, 12→14-15)

### Phase 2: Design Quality (Medium Impact, Medium Effort)
**Timeline:** 2-3 weeks | **Recommended skills:** `/typeset`, `/arrange`, `/bolder`

1. **All sites:** Introduce distinctive typography (display font for headings)
2. **phenotype-dev-hub:** Improve visual hierarchy and spacing; add visual distinction to products
3. **VitePress sites:** Enhance navigation hierarchy; add logos/icons
4. Expected: +2-3 points per site

### Phase 3: Polish & Optimization (Low-Medium Impact, Low Effort)
**Timeline:** 1 week | **Recommended skills:** `/optimize`, `/polish`, `/distill`

1. **FocalPoint:** Lazy-load Mermaid; customize Mermaid theme
2. **phenotype-dev-hub:** Remove decorative blur circles; replace emoji with icons
3. **All sites:** Final polish pass (spacing, alignment, micro-interactions)
4. Expected: +1-2 points per site

---

## Estimated Total Effort

| Phase | Effort | Timeline |
|-------|--------|----------|
| Phase 1 (Critical) | 20-25 hours | 1-2 weeks |
| Phase 2 (Design Quality) | 15-20 hours | 2-3 weeks |
| Phase 3 (Polish) | 8-10 hours | 1 week |
| **Total** | **43-55 hours** | **4-6 weeks** |

---

## Deliverables

**Audit documents created:**
1. `/repos/apps/phenotype-dev-hub/docs/design_audit_2026_04_24.md` — 192 lines, full findings
2. `/repos/thegent/docs/site/design_audit_2026_04_24.md` — 195 lines, full findings
3. `/repos/FocalPoint/docs-site/design_audit_2026_04_24.md` — 188 lines, full findings (submodule)
4. `/repos/hwLedger/docs-site/design_audit_2026_04_24.md` — 172 lines, full findings (submodule)

**Each audit includes:**
- 5-dimension health score matrix
- P0/P1/P2/P3 severity findings with impact + recommendations
- Positive findings to preserve
- Ordered action list with Impeccable skill recommendations
- No code changes (documentation only)

---

## Next Steps

1. **Review audit findings** — Confirm priority and feasibility of recommended fixes
2. **Create AgilePlus specs** for Phase 1 fixes (per project governance)
3. **Dispatch skill agents** in priority order: `/normalize` → `/colorize` → `/harden` → `/typeset`
4. **Re-run `/audit`** after fixes to verify health score improvements
5. **Track progress** in AgilePlus worklog

---

## Notes

- **No fixes applied** — All audits are documentation-only per Wave-10 backlog spec #17
- **Submodule audits** — FocalPoint and hwLedger audits are in submodules; integration may require separate commits
- **Brand assets** — Verify BRAND.md files exist for each project before running `/colorize`
- **Performance** — No critical performance issues detected; Mermaid lazy-loading is nice-to-have optimization
- **Scope** — Audits focused on user-facing surfaces (marketing, docs); internal tooling not included

---

**Auditor:** Claude (Haiku 4.5)  
**Generated:** 2026-04-24 17:10 UTC  
**Tool:** Impeccable Design Audit v1

