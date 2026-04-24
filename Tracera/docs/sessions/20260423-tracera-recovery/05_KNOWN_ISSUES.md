# Known Issues

- Full frontend root tests do not yet complete as a single useful gate. The root
  `bun run test` command reaches the recovered `@tracertm/web` suite and times out, so
  web tests must be chunked by concern while stale recovered tests are reconciled.
- Some recovered source files were absent from the source tree and were intentionally not
  mirrored because the filesystem copy would have failed on missing tracked paths.
- The live repo still contains a mixture of recovered implementation files and pre-existing
  docs until the remaining cleanup pass is done.
- `oxlint-tsgolint` is not part of the default typecheck gate. It currently reports a
  broad type-aware lint backlog and can panic in the web app path. Use
  `bun run check:type-aware` only as a dedicated quality backlog until that is fixed.
- `bun run lint` now has a restored `.oxlintrc.json`, but the web app has a large
  existing lint backlog. Treat lint cleanup as a separate graph-rendering/frontend
  quality tranche rather than a compiler/build blocker.
- Native/process-compose bring-up is now healthy for the core app stack. Go backend,
  Python backend, frontend, caddy, docs, storybook, and required infra report Ready under
  `process-compose --port 18080 list -o wide`.
- `postgres_exporter` and `redis_exporter` are optional. Missing binaries now log a
  warning and stay non-blocking instead of failing the native process graph.
- `start-services.sh` had stale root-path, NATS config, and background stdout behavior.
  It now resolves the repository root correctly, starts NATS from `config/nats-server.conf`,
  and redirects NATS logs so the Python preflight helper does not hang on inherited pipes.
- Tempo is now represented in the native process-compose file, but query parity still
  depends on either a local `tempo` binary or a working Docker fallback for
  `http://127.0.0.1:3200`.
- Several recovered `frontend/apps/web` tests encoded older UI copy, class names,
  mock shapes, or timing assumptions. API, a11y, primitive component, utility, cache, core
  store, auth/websocket/ui store, basic hook, auth/link/item hook, `ErrorBoundary`,
  `CreateItemForm`, `CreateLinkForm`, `CreateTestItemForm`, `Header`, `Sidebar`,
  `UICodeTracePanel`, `PageHeader`, `CreateProjectForm`, `KeyboardShortcutsModal`,
  `BulkActionToolbar`, `TableAccessibility`, `ProgressDashboard`, command palette,
  keyboard shortcuts, route guards/routes, security/CSRF, `ResponsiveCardView`,
  graph/websocket hooks, CRUD/search hooks, full store suite, `ReportsView`,
  `TraceabilityMatrixView`, the latest 10-view chunk, the full app-integration file,
  and the page suite are reconciled.
- Remaining web-test backlog is now narrower than the original recovered suite and should
  continue by explicit concern chunks rather than root-suite runs.
- `src/__tests__/integration/app-integration.test.tsx` now passes as a full file after
  aligning store resets, current UI copy, and `SearchView` query propagation.
- `src/__tests__/pages` now passes after replacing stale page assertions with current
  page-contract coverage and isolating TanStack router/query harness dependencies.
- The previous `ItemsTableView` nested interactive button warning is fixed through the
  shared `ResponsiveCardView` card/action split. Keep watching for similar warnings in
  other responsive card or chat surfaces as broader chunks run.
- `oxlint-tsgolint` can panic with `Bad UTF-16 character offset` while scanning the web
  app. Plain `oxlint` emits real findings, so the type-aware lint crash should be fixed or
  isolated before treating the full lint gate as actionable.
- `lint:type-aware:web:strict` no longer reports the automatic JSX runtime
  `react-in-jsx-scope` false positive. The strict backlog is now real lint debt, led by
  `typescript-eslint(no-unsafe-type-assertion)`, Jest recovered-test rules,
  `eslint(no-unused-vars)`, `typescript-eslint(no-floating-promises)`, JSX a11y rules,
  import default/named-default rules, and React hook dependency findings.
- `lint:type-aware:packages` exits 0 but still prints package-local report findings from
  `oxlint-tsgolint`. Treat those as a separate package quality backlog rather than a
  compiler/typecheck blocker.
- Strict web type-aware lint triage is captured in `07_TYPE_AWARE_LINT_TRIAGE.md`.
  Initial strict-error buckets were 940 production, 1616 test, 9 story, and 5
  test-support errors. After twelve production tranches, full strict errors are down
  from 2,503 to 1,816 and `typecheck:web` remains green. Continue production cleanup
  before recovered test style.
