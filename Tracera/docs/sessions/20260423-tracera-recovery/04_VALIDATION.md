# Validation

## Governance

- `python3 /Users/kooshapari/CodeProjects/Phenotype/repos/Tracera/validate_governance.py`
- Result: `13/13 passed, 0 failed`

## Observability

- Live docs now point to Grafana Alloy and Tempo as the default path
- No live `jaeger`, `promtail`, or `zipkin` references remain outside excluded dependency,
  setup, virtualenv, dist, and vendor paths
- Runtime backend deployment now exports traces through `PHENO_OBSERVABILITY_OTLP_GRPC_ENDPOINT`
- Native app processes now default `PHENO_OBSERVABILITY_OTLP_GRPC_ENDPOINT` to the local
  Alloy OTLP receiver at `127.0.0.1:4319`
- Native Alloy startup script and local Alloy config are restored
- Grafana Tempo datasource provisioning is restored as `shared-traces`
- APM guide and quick reference are restored
- `bash scripts/shell/verify-apm-integration.sh` passes static checks and live checks for
  Alloy, Prometheus, Grafana, and Tempo after a clean process-compose restart
- `bash -n scripts/shell/alloy-if-not-running.sh scripts/shell/tempo-if-not-running.sh
  scripts/shell/postgres-exporter-if-not-running.sh scripts/shell/redis-exporter-if-not-running.sh
  scripts/shell/verify-apm-integration.sh scripts/shell/verify-native-setup.sh` passes

## Frontend

- `bun run typecheck` passes from `frontend/`
- `bun run typecheck` passes from `frontend/apps/web`
- `bun run lint:type-aware:web` completes as a non-blocking legacy report; strict findings
  remain available with `bun run lint:type-aware:web:strict`
- `bun run lint:type-aware:packages` exits 0 after package-scoped `tsconfig.json` iteration
- `bunx turbo build --filter=@tracertm/web` passes from `frontend/`
- `bun run build && bun run test` passes from `frontend/packages/ui`
- UI package test result: 24 files and 222 tests passed
- `bun run test` passes from `frontend/apps/desktop` with 2 files and 33 tests
- `bun run test` passes from `frontend/apps/docs` with 16 files and 216 tests
- `bun run test:vitest --run` passes from `frontend/apps/storybook` with 1 file and 1 test
- `bunx vitest run src/__tests__/components/ErrorBoundary.test.tsx` passes from
  `frontend/apps/web` with 50 tests
- `bunx vitest run src/__tests__/components/CreateItemForm.test.tsx
  src/__tests__/components/CreateLinkForm.test.tsx
  src/__tests__/components/CreateTestItemForm.test.tsx
  src/__tests__/components/layout/Header.test.tsx
  src/__tests__/components/graph/UICodeTracePanel.test.tsx` passes from
  `frontend/apps/web` with 105 tests
- `bun vitest --run src/__tests__/components/PageHeader.test.tsx
  src/__tests__/components/CreateProjectForm.test.tsx
  src/__tests__/components/KeyboardShortcutsModal.test.tsx
  src/__tests__/hooks/useConfirmedDelete.test.ts src/__tests__/hooks/useProjects.test.ts
  src/__tests__/views/ReportsView.test.tsx
  src/__tests__/views/TraceabilityMatrixView.test.tsx --reporter=dot` passes from
  `frontend/apps/web` with 7 files and 98 tests
- `bun x vitest run src/__tests__/api --reporter=dot` passes from `frontend/apps/web`
  with 29 files and 713 tests
- `bun x vitest run src/__tests__/a11y
  src/__tests__/components/graph/PerformanceIndicators.test.tsx --reporter=verbose` passes
  from `frontend/apps/web` with 7 files, 140 tests passed, and 1 skipped
- `bunx vitest run src/__tests__/components/ConfirmationDialog.test.tsx
  src/__tests__/components/EmptyState.test.tsx
  src/__tests__/components/LoadingSpinner.test.tsx
  src/__tests__/components/ProgressRing.test.tsx --reporter=dot` passes from
  `frontend/apps/web` with 4 files and 105 tests
- `bun vitest --run src/__tests__/hooks/useDebounce.test.ts
  src/__tests__/hooks/useKeyPress.test.ts src/__tests__/hooks/useLocalStorage.test.ts
  src/__tests__/hooks/useMediaQuery.test.ts src/__tests__/hooks/useOnClickOutside.test.ts
  --reporter=dot` passes from `frontend/apps/web` with 5 files and 122 tests
- `bunx vitest run src/__tests__/api/system.test.ts src/__tests__/api/equivalence.test.ts
  src/__tests__/api/canonical.test.ts src/__tests__/api/schema.test.ts
  src/__tests__/types/typeGuards.test.ts --reporter=dot` passes from `frontend/apps/web`
  with 5 files and 100 tests
- `bunx vitest run src/__tests__/stores/projectStore.test.ts
  src/__tests__/stores/itemsStore.test.ts --reporter=dot` passes from `frontend/apps/web`
  with 2 files and 34 tests
- `bunx vitest run src/__tests__/hooks/useSearch.test.ts
  src/__tests__/hooks/useItemsQuery.test.ts --reporter=dot` passes from `frontend/apps/web`
  with 2 files and 43 tests
- `bunx vitest run src/__tests__/components/LoadingStates.test.tsx
  src/__tests__/utils/helpers.comprehensive.test.ts --reporter=dot` passes from
  `frontend/apps/web` with 2 files and 128 tests
- `bun vitest --run src/__tests__/hooks/useAuth.test.ts src/__tests__/hooks/useLinks.test.ts
  src/__tests__/hooks/useItems.p1.test.ts --reporter=dot` passes from `frontend/apps/web`
  with 3 files and 76 tests. The auth tests still emit expected mocked CSRF warnings.
- `bun vitest --run src/__tests__/stores/authStore.test.ts
  src/__tests__/stores/websocketStore.test.ts src/__tests__/stores/uiStore.test.ts
  --reporter=dot` passes from `frontend/apps/web` with 3 files and 34 tests. The store
  tests intentionally exercise mocked auth and websocket error paths, so warning/error logs
  are expected.
- `bun vitest --run src/__tests__/cache/CacheManager.test.ts --reporter=dot` passes from
  `frontend/apps/web` with 1 file and 27 tests
- `bun vitest --run src/__tests__/components/CommandPalette.test.tsx
  src/__tests__/a11y/command-palette.test.tsx --reporter=dot` passes from
  `frontend/apps/web` with 2 files, 53 tests passed, and 1 skipped. JSDOM still emits
  expected `HTMLCanvasElement.getContext()` warnings without the optional canvas package.
- `bun vitest --run src/__tests__/components/layout/Header.test.tsx
  src/__tests__/components/layout/Sidebar.test.tsx --reporter=dot` passes from
  `frontend/apps/web` with 2 files and 62 tests
- `bun vitest --run src/__tests__/components/BulkActionToolbar.test.tsx
  src/__tests__/components/TableAccessibility.test.tsx
  src/__tests__/components/ProgressDashboard.test.tsx --reporter=dot` passes from
  `frontend/apps/web` with 3 files and 44 tests
- `bun vitest --run src/__tests__/hooks/useBulkSelection.test.ts
  src/__tests__/hooks/useKeyboardShortcuts.test.ts
  src/__tests__/hooks/useUndoRedo.test.ts --reporter=dot` passes from
  `frontend/apps/web` with 3 files and 34 tests
- `bun vitest --run src/__tests__/views/AdvancedSearchView.test.tsx
  src/__tests__/views/EventsTimelineView.test.tsx src/__tests__/views/ExportView.test.tsx
  src/__tests__/views/ImpactAnalysisView.test.tsx src/__tests__/views/ImportView.test.tsx
  src/__tests__/views/ItemsTableView.a11y.test.tsx
  src/__tests__/views/ItemsTableView.comprehensive.test.tsx
  src/__tests__/views/ItemsTreeView.comprehensive.test.tsx
  src/__tests__/views/ProjectDetailView.comprehensive.test.tsx
  src/__tests__/views/SettingsView.test.tsx --reporter=dot` passes from
  `frontend/apps/web` with 10 files and 93 tests. Current non-fatal stderr includes
  recovered-test mock warnings, deliberate import error paths, and an `ItemsTableView`
  nested-button warning that should be fixed as production markup debt.
- `bun vitest --run src/__tests__/components/mobile/ResponsiveCardView.test.tsx
  src/__tests__/views/ItemsTableView.comprehensive.test.tsx
  src/__tests__/views/ItemsTableView.a11y.test.tsx --reporter=dot` passes from
  `frontend/apps/web` with 3 files and 52 tests after splitting mobile card actions out
  of the primary card button to avoid nested interactive markup.
- `bun vitest --run src/__tests__/route-guards.test.ts src/__tests__/routes --reporter=dot`
  passes from `frontend/apps/web` with 13 files and 86 tests. Expected stderr includes
  the route-guard negative path that validates failed session refresh redirects.
- `bun vitest --run src/__tests__/security src/lib/csrf.test.ts --reporter=dot` passes
  from `frontend/apps/web` with 7 files and 257 tests. Expected stderr covers deliberate
  CSRF/auth negative-path failures.
- `bun vitest --run src/__tests__/hooks/useGraph.test.ts
  src/__tests__/hooks/useGraph.comprehensive.test.ts
  src/__tests__/hooks/useGraphQuery.test.ts src/__tests__/hooks/useHybridGraph.test.ts
  src/__tests__/hooks/useMCP.test.ts src/__tests__/hooks/usePredictivePrefetch.test.ts
  src/__tests__/hooks/useWebSocketHook.test.ts
  src/__tests__/hooks/useWebSocket.integration.test.ts
  src/hooks/__tests__/useGraphPerformanceMonitor.test.ts
  src/hooks/__tests__/useViewportGraph.test.ts --reporter=dot` passes from
  `frontend/apps/web` with 10 files and 163 tests. Expected stderr includes mocked
  unhandled graph fetch warnings in negative-path graph-hook tests.
- `bun vitest --run src/__tests__/hooks/useItems.comprehensive.test.tsx
  src/__tests__/hooks/useItems.p1.test.ts src/__tests__/hooks/useItemsQuery.test.ts
  src/__tests__/hooks/useLinks.comprehensive.test.ts src/__tests__/hooks/useLinks.test.ts
  src/__tests__/hooks/useProjects.comprehensive.test.ts
  src/__tests__/hooks/useProjects.test.ts src/__tests__/hooks/useSearch.comprehensive.test.ts
  src/__tests__/hooks/useSearch.test.ts --reporter=dot` passes from
  `frontend/apps/web` with 9 files and 144 tests. Non-fatal stderr is limited to
  React act warnings in debounce-focused search tests.
- `bun vitest --run src/__tests__/stores --reporter=dot` passes from
  `frontend/apps/web` with 8 files and 187 tests. Expected stderr covers deliberate
  AuthKit, CSRF, logout, and websocket negative paths.
- `bun vitest --run src/__tests__/components/ComponentUsageMatrix.test.tsx
  src/__tests__/components/ConfirmationDialog.test.tsx
  src/__tests__/components/CreateItemDialog.error-handling.test.tsx
  src/__tests__/components/EmptyState.test.tsx
  src/__tests__/components/FigmaSyncPanel.test.tsx
  src/__tests__/components/JourneyExplorer.integration.test.tsx
  src/__tests__/components/JourneyExplorer.test.tsx
  src/__tests__/components/LoadingSpinner.test.tsx
  src/__tests__/components/LoadingStates.test.tsx
  src/__tests__/components/memoization.test.tsx
  src/__tests__/components/mobile/MobileMenu.test.tsx
  src/__tests__/components/ProgressRing.test.tsx --reporter=dot` passes from
  `frontend/apps/web` with 12 files and 232 tests.
- `bun vitest --run src/__tests__/integration/api-client.integration.test.ts
  src/__tests__/integration/cross-feature-workflows.test.tsx
  src/__tests__/integration/incremental-graph-loading.test.ts
  src/__tests__/integration/virtual-scrolling.integration.test.tsx --reporter=dot
  --no-color` passes from `frontend/apps/web` with 4 files and 56 tests.
- Focused current-contract tests in `src/__tests__/integration/app-integration.test.tsx`
  pass with 3 selected tests and 51 skipped:
  `authenticated session setup`, `should sync auth state with items access`, and
  `should complete full item creation workflow`.
- `bun vitest --run src/__tests__/integration/app-integration.test.tsx --reporter=dot
  --no-color` passes from `frontend/apps/web` with 1 file and 54 tests. Expected stderr
  includes mocked CSRF/logout negative-path warnings and JSDOM Recharts zero-size warnings.
- `bun vitest --run src/__tests__/pages --reporter=dot --no-color` passes from
  `frontend/apps/web` with 4 files and 32 tests. Expected stderr is limited to JSDOM
  Recharts zero-size warnings in `ProjectDetail`.
- `bun run typecheck:web` passes from `frontend/`
- `bun x oxlint -c .oxlintrc.json --print-config apps/web/src/main.tsx` now reports
  `"react/react-in-jsx-scope": "allow"`, confirming the React automatic JSX runtime
  false-positive rule is disabled for the web app.
- `bun x oxlint -c .oxlintrc.json --tsconfig tsconfig.packages.json --print-config
  packages/ui/src/__tests__/Input.test.tsx` now reports `"react/react-in-jsx-scope":
  "allow"`, confirming the same automatic-runtime override applies to packages.
- Targeted web/package greps for `react-in-jsx-scope` and `React must be in scope`
  produce no matches after the shared `.oxlintrc.json` fix.
- `bun run lint:type-aware:web` exits 0 as the non-blocking legacy report command.
- `bun run lint:type-aware:packages` exits 0 as the package-scoped type-aware report
  command, though `oxlint-tsgolint` still prints package-local findings.
- `bun x oxlint -c .oxlintrc.json -f unix --threads 1 --tsconfig
  apps/web/tsconfig.oxlint.json --type-aware` passes with warning-only output for the
  first strict production tranche:
  `useRealtime.ts`, `IndexedDBCache.ts`, `preflight.ts`,
  `useIntegrationsDiscovery.ts`, `handlers.ts`, `nodeDataTransformers.ts`,
  `DimensionFilters.tsx`, `FlowGraphView.tsx`, `CreateItemDialog.tsx`,
  `CreateProblemForm.tsx`, `CreateProcessForm.tsx`, `ItemSpecTabs.tsx`,
  `FeatureListView.tsx`, and `DashboardView.tsx`.
- `bun run typecheck:web` passes after integrating the first strict-lint tranche and
  repairing form, React Flow, WebGPU buffer, and placeholder TRPC typing.
- `bun run lint:type-aware:web:strict` still exits 1, but the strict-error count is
  reduced from 2,503 to 2,035 after seven production tranches. The current largest
  remaining production clusters are at four errors per file, led by `GraphView.tsx`,
  `FeatureListView.tsx`, `ADRDetailView.tsx`, `auth-store.ts`, `TestSuiteView.tsx`,
  `TestCaseView.tsx`, `websocket.ts`, `RetryPolicy.ts`, `quadTreeIndex.ts`,
  `gpuForceLayout.ts`, `CacheManager.ts`, `useIntegrations.ts`, graph equivalence
  import/export utilities, and API client files.
- `bun run test` from `frontend/` remains too broad for this pass; `@tracertm/web` has a
  large recovered legacy suite and needs chunked triage rather than a single root timeout

## Backend

- Focused compile gate passes:
  `go test -run '^$' ./cmd/tracertm ./internal/services ./internal/server ./internal/config ./internal/tracing`

## Hygiene

- `git diff --check -- Tracera` passes

## Native Stack

- `bash scripts/shell/verify-native-setup.sh` passes with 24 checks, 0 failures
- Required base endpoints pass after local startup:
  database `127.0.0.1:5432`, cache `127.0.0.1:6379`, NATS `127.0.0.1:4222`,
  NATS monitoring `127.0.0.1:8222/healthz`, and Neo4j Bolt `127.0.0.1:7687`
- `process-compose -f config/process-compose.yaml --dry-run -t=false` validates
  25 configured processes
- `python3 scripts/python/dev-start-with-preflight.py up -D --logs-truncate` starts
  process-compose in detached mode on port `18080`
- A clean `process-compose --port 18080 down` followed by
  `process-compose -f config/process-compose.yaml --port 18080 -D --logs-truncate`
  starts the recovered graph
- `process-compose --port 18080 process get` shows Tempo, Alloy, Grafana, Prometheus,
  Go backend, Python backend, and optional exporter guards running; Tempo, Alloy,
  Grafana, Prometheus, and Python backend are Ready
- `curl http://127.0.0.1:3200/ready` returns `200 ready`
- `curl http://127.0.0.1:12345/-/ready` returns `200 Alloy is ready.`
- `curl http://127.0.0.1:3000/api/health` returns `200` from Grafana
- `curl http://127.0.0.1:5173` returns the Vite web app HTML
- `curl http://127.0.0.1:8080/health` returns
  `200 {"status":"ok","service":"tracertm-backend"}`
- `curl http://127.0.0.1:8000/health` returns
  `200 {"status":"healthy","version":"1.0.0","service":"TraceRTM API"}`
- `.venv` was created with `uv sync --extra dev --extra observability` so the Python
  backend guard and tracing imports pass

## Open follow-up

- Treat `bun run lint:type-aware:web:strict` as a separate quality backlog until the
  recovered web app's legacy type-aware lint findings are paid down
- Current strict type-aware lint backlog is no longer dominated by `react-in-jsx-scope`;
  that false-positive class is fixed. Remaining strict findings are real source/test
  lint debt and should continue in production-first tranches.
- Package-local `oxlint-tsgolint` can still panic with `Bad UTF-16 character offset`
  around `src/__tests__/integration/cross-feature-workflows.test.tsx`; treat that as a
  toolchain/input isolation task before trusting package-local type-aware lint.
- Continue chunking the `frontend/apps/web` Vitest suite by concern; do not rely on the
  root `bun run test` timeout as a useful diagnostic signal
- Next high-value chunks are non-graph component remainders, integration/page chunks,
  and isolated type-aware lint cleanup
- Optional `postgres_exporter` and `redis_exporter` now stay alive as non-blocking guard
  processes when the exporter binaries are absent locally
