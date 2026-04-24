# Phenotype Organization — Brand Playbook

## Vision & Mission

**Vision:** Agent-native platform stack for modern development — bridging planning, observability, automation, and personal productivity.

**Mission:** Provide open-source, composable, production-ready tools that empower developers, product teams, and organizations to work with AI-driven workflows while maintaining human governance, transparency, and control.

---

## Brand Values

1. **Agent-First** — Tools designed for AI agents and humans to collaborate seamlessly
2. **Open & Modular** — Every piece can be adopted independently; no mandatory monoliths
3. **Transparent & Auditable** — Event-sourced, append-only logging; tamper-evident chains
4. **Developer Experience** — Spec-driven, composable APIs; zero-friction local-first workflows
5. **Privacy-Respecting** — Data stays on-device by default; federation optional

---

## Visual Identity

### Logo & Wordmark

**Primary Logo:** Phenotype wordmark (horizontal)
- Minimal geometric mark (optional: hexagon or network node abstraction)
- Lowercase `phenotype` in system sans-serif, medium weight
- Clearspace: 1× height of logo on all sides

**Favicon:** Simplified mark only (16px, 32px, 192px variants)

**Color Variants:**
- Light mode: Primary 900 (deep black) on transparent
- Dark mode: Primary 50 (off-white) on transparent
- Monochrome: Black or white only

### Color System

#### Core Palette (Neutral)

| Scale | Hex | CSS | Purpose |
|-------|-----|-----|---------|
| Primary 50 | `#f5f5f5` | `bg-primary-50` | Off-white backgrounds |
| Primary 900 | `#0a0a0a` | `bg-primary-900` | Deep black, text |
| Slate 100 | `#f1f5f9` | `bg-slate-100` | Subtle backgrounds |
| Slate 500 | `#64748b` | `text-slate-500` | Secondary text |
| Slate 900 | `#0f172a` | `text-slate-900` | Primary text |

#### Collection Accent Colors

**Sidekick Collection** (Personal Productivity)
- Primary: Warm Orange `#ea580c` / RGB(234, 88, 12)
- Light: `#fed7aa` (pale orange)
- Dark: `#92400e` (burnt sienna)
- Usage: FocalPoint, personal nudging features
- Mood: Approachable, energetic, warm

**Observably Collection** (Observability & Governance)
- Primary: Cool Blue `#0066cc` / RGB(0, 102, 204)
- Light: `#cce5ff` (sky blue)
- Dark: `#003d82` (navy)
- Usage: AgilePlus, Tracera, Benchora
- Mood: Professional, trustworthy, analytical

**Vault Collection** (Storage & Ledgers)
- Primary: Neutral Stone `#78716c` / RGB(120, 113, 108)
- Light: `#e7e5e4` (warm gray)
- Dark: `#292524` (charcoal)
- Usage: Stashly, data storage, ledger products
- Mood: Solid, grounded, timeless

**Eidolon Collection** (Device Automation)
- Primary: Dark Tech Purple `#5e21b6` / RGB(94, 33, 182)
- Light: `#e9d5ff` (lavender)
- Dark: `#3f0f7f` (deep purple)
- Usage: Eidolon, device orchestration, infrastructure
- Mood: Technical, innovative, powerful

**Tools Collection** (Utilities & Infrastructure)
- Primary: Clean Green `#059669` / RGB(5, 150, 105)
- Light: `#a7f3d0` (mint)
- Dark: `#064e3b` (forest)
- Usage: Paginary, thegent, utilities
- Mood: Fresh, reliable, enabling

#### Semantic Colors

| State | Hex | Usage |
|-------|-----|-------|
| Success | `#10b981` | Form validation, success messages |
| Error | `#ef4444` | Errors, destructive actions |
| Warning | `#f59e0b` | Warnings, cautions |
| Info | `#3b82f6` | Information, hints |
| Disabled | `#d1d5db` | Disabled buttons, muted text |

### Typography

#### Font Stack

**Heading Font (H1–H6):**
```css
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
font-weight: 700 (bold) or 800 (extra-bold for H1);
line-height: 1.1 (tight);
letter-spacing: -0.02em (tighter);
```

**Body Font:**
```css
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
font-weight: 400 (regular);
line-height: 1.6 (relaxed);
letter-spacing: 0 (normal);
```

**Code Font:**
```css
font-family: "Monaco", "Menlo", "Ubuntu Mono", "Courier New", monospace;
font-weight: 400;
font-size: 0.875rem (14px);
line-height: 1.5;
```

#### Type Scale

| Element | Size | Weight | Line Height | Example Use |
|---------|------|--------|-------------|-------------|
| H1 | 3.5rem (56px) | 800 | 1.1 | Page titles |
| H2 | 2.25rem (36px) | 700 | 1.2 | Section headings |
| H3 | 1.875rem (30px) | 700 | 1.25 | Subsection headings |
| H4 | 1.5rem (24px) | 600 | 1.3 | Component titles |
| H5 | 1.25rem (20px) | 600 | 1.35 | Card titles |
| H6 | 1rem (16px) | 600 | 1.4 | Labels |
| Body | 1rem (16px) | 400 | 1.6 | Paragraph text |
| Small | 0.875rem (14px) | 400 | 1.5 | Secondary text, captions |
| Micro | 0.75rem (12px) | 500 | 1.4 | Badges, timestamps |

### Spacing & Layout

**Base Unit:** 4px (Tailwind default)

**Common Spacing Scales:**
- xs: 4px (0.25rem)
- sm: 8px (0.5rem)
- md: 16px (1rem)
- lg: 24px (1.5rem)
- xl: 32px (2rem)
- 2xl: 48px (3rem)
- 3xl: 64px (4rem)

**Typical Page Margins:**
- Mobile (< 640px): 16px–24px
- Tablet (640px–1024px): 24px–32px
- Desktop (≥1024px): 48px–64px

**Grid/Section Spacing:**
- Tight (cards, lists): 16px gap
- Default (sections): 32px gap
- Loose (major sections): 64px gap

### Iconography

**Icon Library:** Use Feather Icons or Heroicons (free, MIT-licensed, consistent stroke weight)

**Style Guidelines:**
- Stroke weight: 1.5–2px (consistent with typography weight)
- Size: 16px, 20px, 24px, 32px (avoid arbitrary sizes)
- Padding inside icon square: 2–4px
- Color: Inherit text color; accent colors on hover
- Disabled: Use Slate 300 (lightened by 50% in light mode)

**Icon Categories:**
- Navigation: home, arrow-right, menu, search, x-close
- Status: check, x, alert-triangle, info, loading spinner
- Actions: edit, delete, download, share, copy
- Products: use product-specific logos or geometric marks

---

## Component Design Patterns

### Buttons

**Variants:**

1. **Solid** (Primary action)
   - Background: Collection accent or primary-900
   - Text: White / primary-50
   - Hover: Darken by 10% (use opacity layer)
   - Disabled: Slate 300 background, Slate 500 text
   - Padding: 10px 16px (sm), 12px 20px (md), 14px 24px (lg)
   - Border radius: 6px

2. **Outline** (Secondary action)
   - Border: 1px solid current accent
   - Background: Transparent
   - Text: Current accent
   - Hover: 5% tinted background
   - Padding: Same as solid (border takes 1px)
   - Border radius: 6px

3. **Ghost** (Tertiary, minimal)
   - Border: None
   - Background: Transparent
   - Text: Current accent (or slate-700 for neutral)
   - Hover: 10% tinted background
   - Padding: Same as solid
   - Border radius: 0 (no rounding for truly minimal look)

**Example HTML (shadcn Button):**
```tsx
<Button variant="solid" size="md" className="bg-observably-blue">
  Learn More
</Button>
<Button variant="outline">
  GitHub
</Button>
```

### Cards & Containers

**Default Card:**
- Border radius: 8px–12px
- Box shadow: `0 1px 3px rgba(0,0,0,0.1)` (light mode); `0 1px 3px rgba(0,0,0,0.3)` (dark mode)
- Padding: 16px–24px (depends on content)
- Background: Primary 50 (light mode); Slate 900 (dark mode)
- Hover: Lift shadow to `0 4px 12px rgba(0,0,0,0.15)` + 2px translate-y

**Accent Card (for collection hubs):**
- Left border: 4px solid collection accent color
- All other styling same as default card
- Hover: Accent color deepens by 10%

### Forms & Inputs

**Text Input:**
- Border: 1px solid Slate 200
- Border radius: 6px
- Padding: 10px 12px
- Focus: `outline: none; border-color: primary-accent; box-shadow: 0 0 0 3px rgba(accent, 0.1)`
- Placeholder: Slate 400
- Disabled: Slate 100 background, Slate 400 text

**Select / Dropdown:**
- Same styling as text input
- Dropdown icon: Feather chevron-down (right side, 4px padding)
- Dropdown menu: Slide-down animation (100ms ease-out)

### Alerts & Notifications

**Alert (shadcn Alert):**
- Border-left: 3px solid semantic color
- Padding: 12px 16px
- Icon: 16px, left-aligned
- Text: Body weight, semantic color
- Dismissible: X icon on right (ghost button, 20px)

**Badge:**
- Padding: 4px 8px
- Border radius: 4px
- Font size: 0.75rem (micro)
- Background: Tinted collection accent (10% opacity)
- Text: Collection accent (darker variant)
- Example: `bg-observably-blue/10 text-observably-blue-dark`

### Navigation

**Header/Navbar:**
- Position: Sticky top
- Height: 64px (desktop), 56px (mobile)
- Background: Primary 50 (light) / Slate 900 (dark)
- Border-bottom: 1px solid Slate 200 (light) / Slate 700 (dark)
- Logo: Left-aligned, 32px height
- Nav links: Right-aligned (desktop), hamburger menu (mobile)
- Active link: Underline (2px solid accent) + text color darkened

**Breadcrumbs:**
- Separator: Forward slash `/` or chevron-right
- Text: Slate 700 (inactive), primary-900 (active/last)
- Size: 0.875rem (small)
- Example: Home / Products / AgilePlus

---

## Per-Collection Brand Guidelines

### Sidekick Collection

**Visual Identity:**
- Primary Color: Warm Orange `#ea580c`
- Mood Keywords: Warm, approachable, empowering, personal
- Typography: Round (use friendly sans-serif), warmer tone in copy

**Logo Mark:** Optional geometric symbol (e.g., flame, star, upward arrow)

**Use Cases:**
- **FocalPoint:** Screen-time management, daily nudges, reward ceremonies
- **Product tagline:** "Bring balance and joy to your digital life"

**Messaging Tone:** Conversational, encouraging, non-judgmental
- Examples:
  - "You've earned a break. Here's your reward."
  - "Let's build better digital habits together."
  - "Your focus, on your terms."

---

### Observably Collection

**Visual Identity:**
- Primary Color: Cool Blue `#0066cc`
- Mood Keywords: Analytical, professional, transparent, insightful
- Typography: Geometric sans (precise), structured copy

**Logo Mark:** Optional (e.g., chart, scope, lens icon)

**Use Cases:**
- **AgilePlus:** Spec traceability, work packages, governance
- **Tracera:** Multi-view requirements, project planning, RTM
- **Benchora:** Performance metrics, baselines, comparisons
- **Product tagline:** "See through complexity. Ship with confidence."

**Messaging Tone:** Data-driven, clear, structured
- Examples:
  - "Trace requirements to code to tests to production."
  - "Your source of truth for governance and traceability."
  - "Observability built for teams."

---

### Vault Collection

**Visual Identity:**
- Primary Color: Neutral Stone `#78716c`
- Mood Keywords: Secure, timeless, reliable, grounded
- Typography: Classic serif secondary option (heritage feel), restrained use of color

**Logo Mark:** Optional (e.g., vault door, ledger, lock)

**Use Cases:**
- **Stashly:** File storage, digital ledger, organization
- **Product tagline:** "Your data, secured and organized."

**Messaging Tone:** Trustworthy, matter-of-fact, reassuring
- Examples:
  - "Everything you need, nothing you don't."
  - "Ledger that lasts."
  - "Store. Organize. Retrieve."

---

### Eidolon Collection

**Visual Identity:**
- Primary Color: Dark Tech Purple `#5e21b6`
- Mood Keywords: Powerful, innovative, cutting-edge, technical
- Typography: Monospace accents, code-forward messaging

**Logo Mark:** Optional (e.g., circuit board, network node, automaton)

**Use Cases:**
- **Eidolon:** Device automation (desktop, mobile, sandbox)
- **Product tagline:** "Automate everything. Control nothing."

**Messaging Tone:** Technical, empowering, forward-thinking
- Examples:
  - "Agent-native automation across any device."
  - "Orchestrate without friction."
  - "Code once, automate everywhere."

---

### Tools Collection

**Visual Identity:**
- Primary Color: Clean Green `#059669`
- Mood Keywords: Fresh, enabling, reliable, practical
- Typography: Modern sans-serif, pragmatic copy

**Logo Mark:** Optional (e.g., toolbox, wrench, building blocks)

**Use Cases:**
- **Paginary:** Pagination utilities, shared libraries
- **thegent:** Dotfiles manager, system provisioning
- **Product tagline:** "Utilities that just work."

**Messaging Tone:** Practical, enabling, straightforward
- Examples:
  - "One library, zero friction."
  - "Automate your setup."
  - "Built by developers, for developers."

---

## Imagery & Photography

### Photo Style

- **Preference:** Minimalist, clean, high contrast
- **Subject matter:** Code editors, dashboards, abstract patterns, work-in-progress scenes
- **Avoid:** Stock photos with people, overly staged corporate imagery
- **Tools:** Unsplash (free), Screenshot GIFs of real product interfaces (preferred)

### Diagrams & Illustrations

- **Style:** Minimal line art or geometric shapes
- **Color:** Use collection accent color + neutral gray
- **Tools:** Excalidraw (free, MIT), Mermaid (markdown diagrams), custom SVG
- **Size:** Full-width for featured sections (max 800px height)

### Screenshots & GIFs

- **Product demos:** Animated GIF or MP4 (auto-playing, looping, no audio)
- **Code snippets:** Dark theme preferred (One Dark Pro, Dracula, Nord)
- **Terminal output:** Monospace, dark background, subtle syntax highlighting

---

## Voice & Tone

### Writing Principles

1. **Clear over clever.** Avoid marketing speak; say what things do.
2. **Active voice.** "We built X" not "X was built"
3. **Second person.** "You can" not "developers can"
4. **Concrete examples.** "Trace tests to requirements" not "achieve visibility"

### Vocabulary

**Use These:**
- "Spec-driven"
- "Event-sourced"
- "Modular"
- "Composable"
- "Agent-native"
- "Audit trail"
- "Append-only"

**Avoid These:**
- "Synergy"
- "Disruptive"
- "Next-gen"
- "Leverage"
- "Empower" (use strategically, not as filler)

### Tone Matrix

| Context | Tone | Example |
|---------|------|---------|
| Product hero | Confident, direct | "The spec-driven dev platform." |
| Feature explanation | Educational, clear | "Write specs once, trace to code and tests automatically." |
| Error message | Helpful, specific | "Couldn't find spec file `PLAN.md`. Create one with `agileplus specify`." |
| Community post | Warm, collaborative | "We shipped the event-sourcing crate. Check it out and let us know what you build!" |
| Technical docs | Precise, structured | "Event sourcing provides tamper-evident audit trails. Events are immutable; verification uses SHA-256 chains." |

---

## Usage Examples

### Landing Page Hero

```html
<h1 class="text-5xl font-bold text-slate-900">
  Agent-native platform stack for modern development
</h1>
<p class="mt-4 text-lg text-slate-700">
  Spec-driven planning, event-sourced governance, and modular automation.
</p>
<button class="bg-primary-900 text-white px-6 py-3 rounded-lg mt-6">
  Explore Products
</button>
```

### Collection Card

```html
<div class="border-l-4 border-sidekick-orange rounded-lg p-6 bg-primary-50 hover:shadow-lg transition">
  <h3 class="text-2xl font-bold text-sidekick-orange">Sidekick</h3>
  <p class="text-slate-700 mt-2">Personal productivity with AI nudging.</p>
  <div class="flex gap-2 mt-4">
    <img src="/focalpoint-logo.svg" alt="FocalPoint" class="h-6" />
  </div>
  <a href="/collections/sidekick" class="text-sidekick-orange font-semibold mt-4 inline-block">
    Explore Collection →
  </a>
</div>
```

### Product Feature Card

```html
<div class="rounded-lg p-6 bg-slate-50 border border-slate-200">
  <svg class="h-8 w-8 text-observably-blue" />
  <h4 class="text-lg font-semibold mt-3 text-slate-900">Spec-Driven Development</h4>
  <p class="text-slate-700 mt-2">
    Write specs in TOML, automatically trace to code and tests.
  </p>
</div>
```

---

## Consistency Checklist

- [ ] All headings use the typography scale (H1–H6)
- [ ] All accent colors match collection assignments
- [ ] All buttons use one of the three variants (solid, outline, ghost)
- [ ] All cards use consistent border radius and shadow
- [ ] All icons are from Feather or Heroicons, 16/20/24/32px only
- [ ] All form inputs follow the input styling guide
- [ ] All collection pages use their assigned accent color for headers/links
- [ ] All product pages use the product's collection accent color
- [ ] Copy is in active voice, second person, clear and concrete
- [ ] No marketing jargon; no "synergy", "leverage", "empower" (except strategically)

---

## Tools & Resources

- **Design:** Figma (Tailwind plugin available)
- **Icons:** [Feather Icons](https://feathericons.com/), [Heroicons](https://heroicons.com/)
- **Colors:** [Tailwind Color Reference](https://tailwindcss.com/docs/colors)
- **Typography:** System fonts (native per OS, no Google Fonts required)
- **Diagrams:** [Excalidraw](https://excalidraw.com/), [Mermaid](https://mermaid.js.org/)
- **Screenshots/GIFs:** Xcode Simulator or actual device + ScreenFloat or GIFOX
- **Code highlighting:** [Prism.js](https://prismjs.com/) (for docs)

---

## Changelog & Updates

| Date | Change | Owner |
|------|--------|-------|
| 2026-04-24 | v1.0 Initial brand playbook | Phenotype Org |

