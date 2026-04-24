# Changelog

## [Unreleased]

### Added

- **`docs/views/index.md`** — **Shipping** row linking to the full-turn delivery guide.
- **`docs/guides/full-turn-delivery.md`** — PR merge to `main`/release, `gh` visibility, changelog, version, docs build; **`docs/planning/full-turn-next24-20260326.md`** — Next 24 execution plan.
- **`AGENTS.md` / `README.md`** — full-turn expectations.
- **Workspace views** hub: `/views/` — changelog, commit log, WBS entry points.
- **Planning:** `docs/planning/rich-workspace-views.md` — product plan for rich plan/research/spec/work modules, roadmap, changelog, git log, WBS.
- **CommitLog** Vue component and `.vitepress/data/commit-log.json` sample.
- **Roadmap** page expanded with horizons, themes, milestones.
- **VitePress:** `srcDir: 'docs'`; nav links for Roadmap and Workspace views; sidebars for `/roadmap/`, `/planning/`, `/views/`.
- **Data stubs:** `.vitepress/data/audit-log.json`, `kb-graph.json` (fix build).

### Fixed

<<<<<<< HEAD
- **CodeQL:** run `npm ci` + `npm run build` (same as deploy) so JavaScript/Vue under `.vitepress` is analyzed; autobuild alone did not extract VitePress sources.
=======
>>>>>>> 6c7d673 (docs: full-turn delivery, workspace views, and CHANGELOG)
- Build failures from missing theme data JSON imports.
