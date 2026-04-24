# Architecture Decision Records — phenotype-docs-engine

---

## ADR-001 — VitePress as Static Site Generator

**Date:** 2025-10-15
**Status:** Accepted

### Context
Alternatives evaluated: Docusaurus (React-heavy), MkDocs (Python, less JS flexibility),
Astro Starlight (excellent but newer ecosystem). VitePress was chosen for its Vue-based MDX
support, fast HMR, and first-class Mermaid integration.

### Decision
Use VitePress as the static site generator. Python handles ingestion/pre-processing;
TypeScript handles the VitePress config and any custom Vue components.

### Consequences
- Both Python (`pyproject.toml`) and TypeScript (`package.json`) toolchains exist in this repo.
- Build pipeline: Python scan → Markdown corpus → VitePress build.

---

## ADR-002 — Hexagonal Architecture for Ingestion Pipeline

**Date:** 2025-10-20
**Status:** Accepted

### Context
The ingestion pipeline needs to fetch docs from GitHub API, local filesystem, and potentially
S3. Per Phenotype mandate, I/O must be behind ports.

### Decision
Define a `DocSource` port (Python Protocol) with `list_files()` and `fetch_content()` methods.
`GitHubDocSource` and `LocalDocSource` are adapters. The pipeline orchestrator depends only on
the port.

### Consequences
- Tests inject a `MockDocSource` with fixture files.
- Adding S3 support requires only a new adapter, no pipeline changes.

---

## ADR-003 — Front-Matter as Agent Contract

**Date:** 2025-11-01
**Status:** Accepted

### Context
Agents need structured metadata to navigate the doc corpus without reading full page content.
HTML meta tags are not machine-friendly; full-text search is too coarse.

### Decision
Inject standardised YAML front-matter into every generated page. This acts as a machine-readable
contract between the docs engine and agent tooling.

### Consequences
- Agents can `GET /search-index.json` and filter by `type`, `repo`, or `fr_ids`.
- Front-matter fields are versioned; breaking changes require a semver bump of the engine.
