# PRD — phenotype-docs-engine

**Version:** 1.0.0
**Stack:** Python 3.11+, TypeScript, VitePress
**Repo:** `KooshaPari/phenotype-docs-engine`
**Status:** Archived (2026-03-25) — migrated to `packages/phenotype-docs/`

---

## Overview

`phenotype-docs-engine` is the documentation generation and publishing engine for the Phenotype
platform. It ingests Markdown, ADR records, PRD files, and code artefacts from multiple
repositories and produces a unified, statically-served VitePress docsite with cross-repo links,
Mermaid diagrams, and embedded MDX widgets.

---

## Epics

### E1 — Multi-Repo Doc Ingestion
**Goal:** Pull documentation from any Phenotype repository into a unified corpus.

#### E1.1 Source Discovery
- As a platform engineer, I want the engine to discover doc files across repos automatically
  so I do not maintain a manual manifest.
- **Acceptance:** `docs-engine scan --org KooshaPari` discovers all `*.md`, `PRD.md`,
  `FUNCTIONAL_REQUIREMENTS.md`, `ADR.md` across all repos.

#### E1.2 Cross-Repo Link Resolution
- **Acceptance:** `~project:/path` links resolve to the correct VitePress route at build time.

### E2 — Static Site Generation
**Goal:** Produce a browsable, statically hosted docsite.

#### E2.1 VitePress Build
- **Acceptance:** `docs-engine build` produces `docs-dist/` suitable for `file://` or CDN serving.

#### E2.2 Mermaid Diagrams
- **Acceptance:** Fenced ` ```mermaid ` blocks render as SVG in the built site.

### E3 — Agentic Read/Write Optimisation
**Goal:** Docs are optimised for AI agent consumption, not just human reading.

#### E3.1 Front-Matter Metadata
- **Acceptance:** Every generated page includes YAML front-matter with `repo`, `type`,
  `fr_ids`, and `last_updated` fields.

#### E3.2 Full-Text Search Index
- **Acceptance:** Build produces a `search-index.json` usable by agents via `jq`.
