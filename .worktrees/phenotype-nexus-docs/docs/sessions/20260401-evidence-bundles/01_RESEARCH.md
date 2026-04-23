# Research Notes

- Built on the default heuristics from `.vitepress/config.mts`: `type` frontmatter, directory prefixes (guide, api, reference, tutorials) and special cases (getting-started, CLI references) determine whether a doc requires an evidence bundle.
- Evidence bundle folder structure must contain `manifest.json`, `overview.gif`, `api-simulation.json`, and `api-simulation.vhs.tape` for the gate to succeed.
- The gate runs inside `transformPageData`, so any missing artifact fails the `npm run docs:evidence` build with a `[evidence-gate]` error.
