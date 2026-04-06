# Known Issues

1. `npm run docs:evidence` continues to emit a VitePress `chunk size` warning (client bundles > 500 kB after minification). Cataloged as a known issue until the theme/layout can be optimized.
2. Translation/API/reference doc wave (Phase 2) now includes placeholder bundles for the guide, reference, CLI, and localized guide/API pages; future waves must keep this cadence for remaining reference leaves.
3. Verification log (2026-04-01T13:45Z & 15:15Z): `npm run docs:evidence` succeeded twice (with the same chunk-size warning) after adding the translated bundles; keep these timestamped entries in this log for future regressions.

## Regression Guard

- Tie the `docs:evidence` script into the release pipeline (Playwright/CI) by invoking `npm run docs:evidence` inside `phenotype-nexus/docs` before deploying docs or merging doc bundles; a dedicated workflow now triggers this gate on every push/PR under `phenotype-nexus/docs`.
- Capture the command output (timestamp + warnings) in the session `05_KNOWN_ISSUES.md` so future batches can compare success/failure signatures.
