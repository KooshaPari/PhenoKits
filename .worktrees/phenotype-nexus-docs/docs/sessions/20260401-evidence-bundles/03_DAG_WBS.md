# Work Breakdown Structure

| Phase | Milestone | Description | Key Outputs |
| ----- | --------- | ----------- | ----------- |
| Phase 1 | Coverage Mapping | Align each targeted doc with the `type` + `evidence_bundle` contract. | Frontmatter checklist, translation guide, AgilePlus WP01 lane `done`. |
| Phase 2 | Bundle Generation | Generate placeholder bundles for the launched doc chunk (getting-started, landing page, guide, reference, CLI) and capture conventions for future waves. | `docs/public/evidence/{getting-started,generic,guide-index,reference-index,reference-cli}` artifacts, WP02 lane `doing`, translation notes for VHS/simulation. |
| Phase 3 | Validation & Automation | Run the gate, log chunk-size warning, and codify regression guard. | `npm run docs:evidence` output + `05_KNOWN_ISSUES.md`, WP03 updates, CI recipe for future runs. |

## Current Bundle Inventory

| Doc | Bundle decision | Proposed bundle | Notes |
| --- | --- | --- | --- |
| `docs/getting-started.md` | Dedicated bundle already exists | `getting-started` | Already present in `docs/.vitepress/dist/evidence/getting-started`. |
| `docs/index.md` | Can share `generic` | `generic` | Home page / landing page does not need a dedicated bundle yet. |
| Planned `docs/guide/**` pages | Dedicated bundle per doc | `guide-*` or page-specific slug | These were called out in the session scope but are not yet present in the source tree. |
| Planned `docs/api/**` pages | Dedicated bundle per doc | `api-*` or page-specific slug | Same handling as guide pages once they land. |
| Planned `docs/reference/**` pages | Dedicated bundle per doc | `reference-*` or page-specific slug | Needed for reference/CLI family pages when created. |
| Planned translated guide pages | Dedicated bundle per doc | locale-qualified slug | Do not keep these on `generic`; they need localized evidence artifacts. |
