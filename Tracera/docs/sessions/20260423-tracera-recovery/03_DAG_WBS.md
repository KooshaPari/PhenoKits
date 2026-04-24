# DAG and WBS

## Current Recovery DAG

```text
R1 recover implementation tree
  -> R2 restore governance requirements
  -> R3 align observability contract
  -> R4 validate backend compile surface
  -> R5 validate frontend package/web surface
  -> R6 classify remaining lint and runtime bring-up backlog
  -> R7 repair native Alloy/Tempo runtime contract
  -> R8 chunk and reconcile recovered web tests
  -> R9 native stack bring-up and app health
```

## Work Breakdown

| ID | Status | Work | Evidence |
| --- | --- | --- | --- |
| R1 | Done | Mirror recovered Go, Python, React, deploy, config, and tests into `Tracera` | `backend/`, `src/tracertm/`, `frontend/`, `deploy/`, `config/` present |
| R2 | Done | Restore missing `FR-TRAC-004` through `FR-TRAC-006` | `python3 validate_governance.py` passes 13/13 |
| R3 | Done | Move live docs and backend deploy manifest to shared OTLP/Tempo path | Native apps export to Alloy at `127.0.0.1:4319`; Alloy exports to Tempo at `127.0.0.1:4317` |
| R4 | Done | Confirm focused Go backend compile surface | `go test -run '^$' ./cmd/tracertm ./internal/services ./internal/server ./internal/config ./internal/tracing` |
| R5 | Done | Restore UI package dependencies and validate frontend build | `bun run typecheck`, `bunx turbo build --filter=@tracertm/web`, UI tests pass |
| R6 | Done | Separate compiler typecheck from experimental type-aware lint backlog | `lint:type-aware` is isolated from `typecheck` |
| R7 | Done | Repair native Alloy/Tempo runtime contract | `alloy-if-not-running.sh`, `alloy-local.alloy`, `tempo.yml`, and APM docs restored; APM static verifier passes |
| R8 | Done | Restore frontend lint/test gate determinism | `.oxlintrc.json` restored; package test imports normalized; Storybook, docs, desktop, UI, API, a11y, primitive component, query hook, auth/link hook, item hook, core store, auth/websocket/ui store, cache, utility, ErrorBoundary, CreateItemForm, CreateLinkForm, CreateTestItemForm, Header, Sidebar, UICodeTracePanel, PageHeader, CreateProjectForm, KeyboardShortcutsModal, BulkActionToolbar, TableAccessibility, ProgressDashboard, command palette, keyboard shortcuts, route guards/routes, security/CSRF, ResponsiveCardView, graph/websocket hooks, CRUD/search hook tranche, full store tranche, ReportsView, TraceabilityMatrixView, 10 remaining view chunks, 12-file component tranche, 4-file integration tranche, full app-integration tranche, and page tranche pass |
| R9 | Done | Bring up native/process-compose stack and validate health | Go backend `:8080/health`, Python backend `:8000/health`, and frontend `:5173` respond under process-compose |
| R10 | In progress | Burn down strict web type-aware lint production backlog | Nine production tranches are warning-only under targeted oxlint and `typecheck:web` passes; full strict count reduced from 2,503 to 1,939 |

## Next Queue

1. Continue production-first strict type-aware lint tranches, now led by three-error
   clusters in views, routes, stores, API/runtime helpers, enterprise UI components,
   and remaining graph components. Keep colocated `lib/websocket.test.ts` and
   `lib/csrf.test.ts` queued after production source.
2. Defer recovered test-suite lint style until production runtime source errors are
   materially lower.
3. Align AgilePlus and PhenoObservability to the same shared OTLP endpoint names.
4. Normalize the recovered implementation surface into a clean branch/commit boundary.
