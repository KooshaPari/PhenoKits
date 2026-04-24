# Testing Strategy

- Primary validation: `npm run docs:evidence` from `phenotype-nexus/docs` (invokes `vitepress build` and ensures every gated doc finds its evidence bundle). The output succeeded apart from the pre-existing chunk-size warning.
- Future test additions: tie the `docs:evidence` command to a CI `verify` job or Playwright smoke script so every bundle addition triggers this gate.
