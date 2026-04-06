# Functional Requirements — phenotype-docs-engine

**Version:** 1.0.0
**Traces to:** PRD epics E1–E3

---

## FR-DOCS-001 — Org-Wide Source Discovery
**SHALL** scan all repositories in the configured GitHub org for documentation files matching
configurable glob patterns (default: `**/*.md`, `PRD.md`, `ADR.md`, `FUNCTIONAL_REQUIREMENTS.md`).
**Traces to:** E1.1

## FR-DOCS-002 — Incremental Scan
**SHALL** support incremental scanning by comparing file SHAs against a local cache and
re-processing only changed files.
**Traces to:** E1.1

## FR-DOCS-003 — Cross-Repo Link Resolution
**SHALL** resolve `~project:/path` syntax to absolute VitePress routes at build time, failing
the build if a referenced path does not exist in the corpus.
**Traces to:** E1.2

## FR-DOCS-004 — VitePress Build Output
**SHALL** produce a `docs-dist/` directory containing all HTML, CSS, JS, and asset files
required for static serving, openable via `file://` without a web server.
**Traces to:** E2.1

## FR-DOCS-005 — Mermaid Rendering
**SHALL** render all ` ```mermaid ` code blocks as inline SVG in the output HTML.
**Traces to:** E2.2

## FR-DOCS-006 — Front-Matter Injection
**SHALL** inject YAML front-matter into every generated page including: `repo` (source
repository name), `type` (prd|fr|adr|guide), `fr_ids` (list of FR IDs mentioned), and
`last_updated` (ISO 8601 date from git log).
**Traces to:** E3.1

## FR-DOCS-007 — Search Index Generation
**SHALL** produce a `search-index.json` at the root of `docs-dist/` containing every page
title, route, and full-text content for programmatic querying.
**Traces to:** E3.2

## FR-DOCS-008 — Build Failure on Broken Links
**SHALL** exit non-zero and print all broken cross-repo links when any `~project:/path`
reference cannot be resolved.
**Traces to:** E1.2
