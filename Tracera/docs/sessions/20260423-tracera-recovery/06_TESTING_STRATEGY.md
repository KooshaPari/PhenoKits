# Testing Strategy

## Gates

| Gate | Command | Purpose |
| --- | --- | --- |
| Governance | `python3 validate_governance.py` | Verifies FR manifest and traceability docs |
| Backend compile | `go test -run '^$' ./cmd/tracertm ./internal/services ./internal/server ./internal/config ./internal/tracing` | Confirms key Go backend packages compile |
| Frontend typecheck | `bun run typecheck` from `frontend/` | Runs compiler-backed TypeScript checks for web and packages |
| Web type-aware report | `bun run lint:type-aware:web` from `frontend/` | Emits recovered web type-aware lint debt without blocking the default gate |
| Web type-aware strict backlog | `bun run lint:type-aware:web:strict` from `frontend/` | Fails on current recovered web type-aware lint findings for focused cleanup |
| Package type-aware report | `bun run lint:type-aware:packages` from `frontend/` | Runs package-scoped type-aware lint configs without aggregate tsconfig drift |
| Web build | `bunx turbo build --filter=@tracertm/web` from `frontend/` | Builds web app and dependency packages |
| UI package | `bun run build && bun run test` from `frontend/packages/ui` | Validates shared UI package and component tests |
| Desktop package | `bun run test` from `frontend/apps/desktop` | Validates Electron main/preload tests |
| Docs package | `bun run test` from `frontend/apps/docs` | Validates documentation app unit tests |
| Storybook package | `bun run test:vitest --run` from `frontend/apps/storybook` | Validates story discovery |
| Web focused | `bunx vitest run src/__tests__/components/ErrorBoundary.test.tsx` from `frontend/apps/web` | Proves one reconciled recovered web test file |
| Web component tranche | `bunx vitest run src/__tests__/components/CreateItemForm.test.tsx src/__tests__/components/CreateLinkForm.test.tsx src/__tests__/components/CreateTestItemForm.test.tsx src/__tests__/components/layout/Header.test.tsx src/__tests__/components/graph/UICodeTracePanel.test.tsx` from `frontend/apps/web` | Validates the first reconciled high-value component tranche |
| Web API tranche | `bun x vitest run src/__tests__/api --reporter=dot` from `frontend/apps/web` | Validates recovered API/client contracts |
| Web a11y tranche | `bun x vitest run src/__tests__/a11y src/__tests__/components/graph/PerformanceIndicators.test.tsx --reporter=verbose` from `frontend/apps/web` | Validates accessibility surfaces and performance indicators |
| Web primitive component tranche | `bunx vitest run src/__tests__/components/ConfirmationDialog.test.tsx src/__tests__/components/EmptyState.test.tsx src/__tests__/components/LoadingSpinner.test.tsx src/__tests__/components/ProgressRing.test.tsx --reporter=dot` from `frontend/apps/web` | Validates primitive components reconciled to current DOM contracts |
| Web hook baseline tranche | `bun vitest --run src/__tests__/hooks/useDebounce.test.ts src/__tests__/hooks/useKeyPress.test.ts src/__tests__/hooks/useLocalStorage.test.ts src/__tests__/hooks/useMediaQuery.test.ts src/__tests__/hooks/useOnClickOutside.test.ts --reporter=dot` from `frontend/apps/web` | Validates basic hook utilities |
| Web store baseline tranche | `bunx vitest run src/__tests__/stores/projectStore.test.ts src/__tests__/stores/itemsStore.test.ts --reporter=dot` from `frontend/apps/web` | Validates core project/item stores |
| Web query hook tranche | `bunx vitest run src/__tests__/hooks/useSearch.test.ts src/__tests__/hooks/useItemsQuery.test.ts --reporter=dot` from `frontend/apps/web` | Validates search/items query hooks |
| Web auth/link/item hook tranche | `bun vitest --run src/__tests__/hooks/useAuth.test.ts src/__tests__/hooks/useLinks.test.ts src/__tests__/hooks/useItems.p1.test.ts --reporter=dot` from `frontend/apps/web` | Validates recovered auth, link, and item hook contracts |
| Web auth/websocket/ui store tranche | `bun vitest --run src/__tests__/stores/authStore.test.ts src/__tests__/stores/websocketStore.test.ts src/__tests__/stores/uiStore.test.ts --reporter=dot` from `frontend/apps/web` | Validates recovered auth, websocket, and UI stores |
| Web cache tranche | `bun vitest --run src/__tests__/cache/CacheManager.test.ts --reporter=dot` from `frontend/apps/web` | Validates CacheManager storage paths with browser-only APIs mocked or avoided |
| Web utility tranche | `bunx vitest run src/__tests__/components/LoadingStates.test.tsx src/__tests__/utils/helpers.comprehensive.test.ts --reporter=dot` from `frontend/apps/web` | Validates loading states and comprehensive helper utilities |
| Web command palette tranche | `bun vitest --run src/__tests__/components/CommandPalette.test.tsx src/__tests__/a11y/command-palette.test.tsx --reporter=dot` from `frontend/apps/web` | Validates palette search behavior and accessibility coverage |
| Web layout tranche | `bun vitest --run src/__tests__/components/layout/Header.test.tsx src/__tests__/components/layout/Sidebar.test.tsx --reporter=dot` from `frontend/apps/web` | Validates responsive header/sidebar behavior |
| Web bulk/progress/table tranche | `bun vitest --run src/__tests__/components/BulkActionToolbar.test.tsx src/__tests__/components/TableAccessibility.test.tsx src/__tests__/components/ProgressDashboard.test.tsx --reporter=dot` from `frontend/apps/web` | Validates bulk actions, native table semantics, and progress dashboard tabs |
| Web interaction hook tranche | `bun vitest --run src/__tests__/hooks/useBulkSelection.test.ts src/__tests__/hooks/useKeyboardShortcuts.test.ts src/__tests__/hooks/useUndoRedo.test.ts --reporter=dot` from `frontend/apps/web` | Validates selection, shortcut registry, and undo/redo hooks |
| Web remaining view tranche | `bun vitest --run src/__tests__/views/AdvancedSearchView.test.tsx src/__tests__/views/EventsTimelineView.test.tsx src/__tests__/views/ExportView.test.tsx src/__tests__/views/ImpactAnalysisView.test.tsx src/__tests__/views/ImportView.test.tsx src/__tests__/views/ItemsTableView.a11y.test.tsx src/__tests__/views/ItemsTableView.comprehensive.test.tsx src/__tests__/views/ItemsTreeView.comprehensive.test.tsx src/__tests__/views/ProjectDetailView.comprehensive.test.tsx src/__tests__/views/SettingsView.test.tsx --reporter=dot` from `frontend/apps/web` | Validates the recovered high-risk view assertions after mock/copy reconciliation |
| Web responsive card/table tranche | `bun vitest --run src/__tests__/components/mobile/ResponsiveCardView.test.tsx src/__tests__/views/ItemsTableView.comprehensive.test.tsx src/__tests__/views/ItemsTableView.a11y.test.tsx --reporter=dot` from `frontend/apps/web` | Validates responsive cards and mobile ItemsTable action markup without nested buttons |
| Web routes tranche | `bun vitest --run src/__tests__/route-guards.test.ts src/__tests__/routes --reporter=dot` from `frontend/apps/web` | Validates route guards and recovered route test files while route source files are excluded from Vitest discovery |
| Web security tranche | `bun vitest --run src/__tests__/security src/lib/csrf.test.ts --reporter=dot` from `frontend/apps/web` | Validates auth security helpers and CSRF token lifecycle tests |
| Web graph/websocket hook tranche | `bun vitest --run src/__tests__/hooks/useGraph.test.ts src/__tests__/hooks/useGraph.comprehensive.test.ts src/__tests__/hooks/useGraphQuery.test.ts src/__tests__/hooks/useHybridGraph.test.ts src/__tests__/hooks/useMCP.test.ts src/__tests__/hooks/usePredictivePrefetch.test.ts src/__tests__/hooks/useWebSocketHook.test.ts src/__tests__/hooks/useWebSocket.integration.test.ts src/hooks/__tests__/useGraphPerformanceMonitor.test.ts src/hooks/__tests__/useViewportGraph.test.ts --reporter=dot` from `frontend/apps/web` | Validates graph, hybrid graph, MCP, predictive prefetch, websocket, and graph performance hooks |
| Web CRUD/search hook tranche | `bun vitest --run src/__tests__/hooks/useItems.comprehensive.test.tsx src/__tests__/hooks/useItems.p1.test.ts src/__tests__/hooks/useItemsQuery.test.ts src/__tests__/hooks/useLinks.comprehensive.test.ts src/__tests__/hooks/useLinks.test.ts src/__tests__/hooks/useProjects.comprehensive.test.ts src/__tests__/hooks/useProjects.test.ts src/__tests__/hooks/useSearch.comprehensive.test.ts src/__tests__/hooks/useSearch.test.ts --reporter=dot` from `frontend/apps/web` | Validates item, link, project, and search hooks after fetch mock isolation and mutation invalidation fixes |
| Web full store tranche | `bun vitest --run src/__tests__/stores --reporter=dot` from `frontend/apps/web` | Validates all recovered Zustand store tests, including AuthKit auth-store coverage and sync-store module naming |
| Web component remainder tranche | `bun vitest --run src/__tests__/components/ComponentUsageMatrix.test.tsx src/__tests__/components/ConfirmationDialog.test.tsx src/__tests__/components/CreateItemDialog.error-handling.test.tsx src/__tests__/components/EmptyState.test.tsx src/__tests__/components/FigmaSyncPanel.test.tsx src/__tests__/components/JourneyExplorer.integration.test.tsx src/__tests__/components/JourneyExplorer.test.tsx src/__tests__/components/LoadingSpinner.test.tsx src/__tests__/components/LoadingStates.test.tsx src/__tests__/components/memoization.test.tsx src/__tests__/components/mobile/MobileMenu.test.tsx src/__tests__/components/ProgressRing.test.tsx --reporter=dot` from `frontend/apps/web` | Validates component usage, journey explorer, mobile menu, memoization, and remaining primitive component contracts |
| Web integration tranche | `bun vitest --run src/__tests__/integration/api-client.integration.test.ts src/__tests__/integration/cross-feature-workflows.test.tsx src/__tests__/integration/incremental-graph-loading.test.ts src/__tests__/integration/virtual-scrolling.integration.test.tsx --reporter=dot --no-color` from `frontend/apps/web` | Validates AuthKit API client flow, cross-feature workflow fixtures, graph stream cancellation, and virtual scrolling integration assertions |
| Web app-integration focused tranche | `bun vitest --run src/__tests__/integration/app-integration.test.tsx -t 'authenticated session setup\|should sync auth state with items access\|should complete full item creation workflow' --reporter=dot --no-color` from `frontend/apps/web` | Validates the current AuthKit/store contract subset while the broad recovered file is split and migrated |
| Web app-integration full tranche | `bun vitest --run src/__tests__/integration/app-integration.test.tsx --reporter=dot --no-color` from `frontend/apps/web` | Validates the reconciled broad app integration file across stores, API mocks, views, and cross-store workflows |
| Web page tranche | `bun vitest --run src/__tests__/pages --reporter=dot --no-color` from `frontend/apps/web` | Validates Dashboard, ProjectDetail, ProjectsList, and Settings page contracts with deterministic store/router cleanup |

## Type-Aware Lint

The web app's recovered type-aware lint corpus is intentionally outside the default
`typecheck` gate. The default web type-aware command now behaves as a report, while
the strict variant remains available for backlog burn-down:

```bash
bun run lint:type-aware:web
bun run lint:type-aware:web:strict
```

Compiler correctness remains enforced by `tsc --build --noEmit`.

The React automatic JSX runtime is configured through `jsx: react-jsx` and Vite's
automatic runtime. `.oxlintrc.json` therefore disables only
`react/react-in-jsx-scope`; do not re-enable that rule or mass-add `React` imports.
Use these checks when touching lint config:

```bash
bun x oxlint -c .oxlintrc.json --print-config apps/web/src/main.tsx | rg '"react/react-in-jsx-scope"'
bun x oxlint -c .oxlintrc.json --tsconfig tsconfig.packages.json --print-config packages/ui/src/__tests__/Input.test.tsx | rg '"react/react-in-jsx-scope"'
bun run lint:type-aware:web:strict
bun run lint:type-aware:packages
```

Expected config value for the first two commands is `"allow"`. The strict web command
still fails on real recovered lint debt; as of this pass, the largest error class is
`typescript-eslint(no-unsafe-type-assertion)`, followed by Jest recovered-test rules and
then smaller production correctness/a11y/import/hook findings.

See `07_TYPE_AWARE_LINT_TRIAGE.md` for bucketed production/test counts and the first
recommended production patch tranche.

## Web Test Reconciliation

`frontend/apps/web` contains a very large recovered Vitest corpus. The root monorepo
`bun run test` command is not yet a useful green/red signal because it can time out before
printing a compact failure summary. Use concern-level chunks instead:

```bash
bunx vitest run src/__tests__/routes
bunx vitest run src/__tests__/components/ErrorBoundary.test.tsx
bunx vitest run src/__tests__/components/CreateItemForm.test.tsx src/__tests__/components/CreateLinkForm.test.tsx src/__tests__/components/CreateTestItemForm.test.tsx src/__tests__/components/layout/Header.test.tsx src/__tests__/components/graph/UICodeTracePanel.test.tsx
bun x vitest run src/__tests__/api --reporter=dot
bun x vitest run src/__tests__/a11y src/__tests__/components/graph/PerformanceIndicators.test.tsx --reporter=verbose
bun vitest --run src/__tests__/hooks/useAuth.test.ts src/__tests__/hooks/useLinks.test.ts src/__tests__/hooks/useItems.p1.test.ts --reporter=dot
bun vitest --run src/__tests__/stores/authStore.test.ts src/__tests__/stores/websocketStore.test.ts src/__tests__/stores/uiStore.test.ts --reporter=dot
bun vitest --run src/__tests__/cache/CacheManager.test.ts --reporter=dot
bun vitest --run src/__tests__/components/CommandPalette.test.tsx src/__tests__/a11y/command-palette.test.tsx --reporter=dot
bun vitest --run src/__tests__/components/layout/Header.test.tsx src/__tests__/components/layout/Sidebar.test.tsx --reporter=dot
bun vitest --run src/__tests__/components/BulkActionToolbar.test.tsx src/__tests__/components/TableAccessibility.test.tsx src/__tests__/components/ProgressDashboard.test.tsx --reporter=dot
bun vitest --run src/__tests__/hooks/useBulkSelection.test.ts src/__tests__/hooks/useKeyboardShortcuts.test.ts src/__tests__/hooks/useUndoRedo.test.ts --reporter=dot
bun vitest --run src/__tests__/components/mobile/ResponsiveCardView.test.tsx src/__tests__/views/ItemsTableView.comprehensive.test.tsx src/__tests__/views/ItemsTableView.a11y.test.tsx --reporter=dot
bun vitest --run src/__tests__/route-guards.test.ts src/__tests__/routes --reporter=dot
bun vitest --run src/__tests__/security src/lib/csrf.test.ts --reporter=dot
bun vitest --run src/__tests__/hooks/useGraph.test.ts src/__tests__/hooks/useGraph.comprehensive.test.ts src/__tests__/hooks/useGraphQuery.test.ts src/__tests__/hooks/useHybridGraph.test.ts src/__tests__/hooks/useMCP.test.ts src/__tests__/hooks/usePredictivePrefetch.test.ts src/__tests__/hooks/useWebSocketHook.test.ts src/__tests__/hooks/useWebSocket.integration.test.ts src/hooks/__tests__/useGraphPerformanceMonitor.test.ts src/hooks/__tests__/useViewportGraph.test.ts --reporter=dot
bun vitest --run src/__tests__/hooks/useItems.comprehensive.test.tsx src/__tests__/hooks/useItems.p1.test.ts src/__tests__/hooks/useItemsQuery.test.ts src/__tests__/hooks/useLinks.comprehensive.test.ts src/__tests__/hooks/useLinks.test.ts src/__tests__/hooks/useProjects.comprehensive.test.ts src/__tests__/hooks/useProjects.test.ts src/__tests__/hooks/useSearch.comprehensive.test.ts src/__tests__/hooks/useSearch.test.ts --reporter=dot
bun vitest --run src/__tests__/stores --reporter=dot
bun vitest --run src/__tests__/components/ComponentUsageMatrix.test.tsx src/__tests__/components/ConfirmationDialog.test.tsx src/__tests__/components/CreateItemDialog.error-handling.test.tsx src/__tests__/components/EmptyState.test.tsx src/__tests__/components/FigmaSyncPanel.test.tsx src/__tests__/components/JourneyExplorer.integration.test.tsx src/__tests__/components/JourneyExplorer.test.tsx src/__tests__/components/LoadingSpinner.test.tsx src/__tests__/components/LoadingStates.test.tsx src/__tests__/components/memoization.test.tsx src/__tests__/components/mobile/MobileMenu.test.tsx src/__tests__/components/ProgressRing.test.tsx --reporter=dot
bun vitest --run src/__tests__/integration/api-client.integration.test.ts src/__tests__/integration/cross-feature-workflows.test.tsx src/__tests__/integration/incremental-graph-loading.test.ts src/__tests__/integration/virtual-scrolling.integration.test.tsx --reporter=dot --no-color
bun vitest --run src/__tests__/integration/app-integration.test.tsx --reporter=dot --no-color
bun vitest --run src/__tests__/pages --reporter=dot --no-color
```

When a chunk fails because assertions reference old recovered copy or class names, update
the test to the current production component contract and rerun that exact chunk before
widening.

The full `src/__tests__/integration/app-integration.test.tsx` and `src/__tests__/pages`
tranches are now reconciled. Future failures in those tranches should be treated as normal
regressions unless evidence shows a new recovered-test harness mismatch.

## Runtime Bring-Up

The current native/process-compose layer uses:

```bash
bash scripts/shell/verify-native-setup.sh
python3 scripts/python/dev-start-with-preflight.py up -D --logs-truncate
process-compose --port 18080 list -o wide
```

Current status:

- Core infra, frontend, Go backend, and Python backend are running under process-compose.
- `process-compose -f config/process-compose.yaml --dry-run --port 18080` validates the
  process graph.
- `curl http://127.0.0.1:5173` returns the Vite web app HTML.
- `curl http://127.0.0.1:8080/health` returns `200` from the Go backend.
- `curl http://127.0.0.1:8000/health` returns `200` from the Python backend.
- `.venv` is created through `uv sync --extra dev --extra observability`.
- Tempo runs as a first-class process-compose service with local query port `3200` and
  OTLP ingestion ports `4317` and `4318`.
- Alloy receives app OTLP on `127.0.0.1:4319` and exports to Tempo on `127.0.0.1:4317`.
- Optional `postgres_exporter` and `redis_exporter` remain non-blocking guard processes
  when exporter binaries are absent locally.
