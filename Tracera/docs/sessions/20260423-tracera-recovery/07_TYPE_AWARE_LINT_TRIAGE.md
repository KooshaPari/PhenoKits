# Type-Aware Lint Triage

Generated: 2026-04-24

Source command:

```bash
bun x oxlint -c .oxlintrc.json -f json --threads 1 \
  --tsconfig apps/web/tsconfig.oxlint.json \
  --ignore-pattern 'apps/web/src/routeTree.gen.ts' \
  --ignore-pattern 'apps/web/src/views/details/TestDetailView.tsx' \
  --ignore-pattern 'apps/web/src/components/graph/JourneyExplorer.tsx' \
  --type-aware apps/web/src
```

## Error Buckets

| Bucket | Errors |
| --- | ---: |
| tests | 1616 |
| production | 940 |
| stories | 9 |
| test-support | 5 |

## Production Rule Leaders

| Rule | Errors |
| --- | ---: |
| `typescript-eslint(no-unsafe-type-assertion)` | 486 |
| `typescript-eslint(no-floating-promises)` | 82 |
| `eslint(no-unused-vars)` | 68 |
| `eslint-plugin-jsx-a11y(label-has-associated-control)` | 51 |
| `eslint-plugin-react-hooks(exhaustive-deps)` | 43 |
| `eslint-plugin-unicorn(prefer-add-event-listener)` | 34 |
| `typescript-eslint(no-unnecessary-type-assertion)` | 32 |
| `eslint-plugin-jsx-a11y(click-events-have-key-events)` | 28 |
| `eslint(no-unused-expressions)` | 13 |
| `eslint-plugin-import(no-named-as-default)` | 12 |
| `typescript-eslint(no-base-to-string)` | 11 |
| `eslint-plugin-jsx-a11y(prefer-tag-over-role)` | 10 |
| `eslint-plugin-unicorn(consistent-function-scoping)` | 9 |
| `eslint-plugin-import(no-unassigned-import)` | 7 |
| `typescript-eslint(restrict-template-expressions)` | 7 |

## Test Rule Leaders

| Rule | Errors |
| --- | ---: |
| `typescript-eslint(no-unsafe-type-assertion)` | 667 |
| `eslint-plugin-jest(valid-title)` | 297 |
| `eslint-plugin-jest(no-conditional-expect)` | 241 |
| `eslint-plugin-unicorn(consistent-function-scoping)` | 95 |
| `eslint-plugin-jest(require-to-throw-message)` | 87 |
| `eslint(no-unused-vars)` | 53 |
| `eslint-plugin-import(no-named-as-default)` | 42 |
| `eslint-plugin-jest(no-disabled-tests)` | 24 |
| `eslint-plugin-jest(expect-expect)` | 23 |
| `eslint-plugin-import(no-unassigned-import)` | 9 |
| `eslint-plugin-unicorn(prefer-add-event-listener)` | 9 |
| `typescript-eslint(unbound-method)` | 9 |
| `eslint-plugin-jest(valid-expect)` | 8 |
| `typescript-eslint(triple-slash-reference)` | 7 |
| `eslint-plugin-promise(always-return)` | 6 |

## Top Production Files

| File | Errors |
| --- | ---: |
| `apps/web/src/lib/preflight.ts` | 27 |
| `apps/web/src/lib/cache/IndexedDBCache.ts` | 27 |
| `apps/web/src/hooks/useIntegrationsDiscovery.ts` | 19 |
| `apps/web/src/mocks/handlers.ts` | 17 |
| `apps/web/src/components/graph/utils/nodeDataTransformers.ts` | 16 |
| `apps/web/src/components/forms/CreateProblemForm.tsx` | 15 |
| `apps/web/src/components/specifications/items/ItemSpecTabs.tsx` | 14 |
| `apps/web/src/components/graph/DimensionFilters.tsx` | 14 |
| `apps/web/src/components/graph/FlowGraphView.tsx` | 13 |
| `apps/web/src/hooks/useRealtime.ts` | 13 |
| `apps/web/src/views/FeatureListView.tsx` | 12 |
| `apps/web/src/views/DashboardView.tsx` | 12 |
| `apps/web/src/components/forms/CreateItemDialog.tsx` | 12 |
| `apps/web/src/hooks/useTemporal.ts` | 11 |
| `apps/web/src/lib/edgeAggregation.ts` | 11 |

## Top Test Files

| File | Errors |
| --- | ---: |
| `apps/web/src/__tests__/api/endpoints.p1.test.ts` | 106 |
| `apps/web/src/__tests__/api/client-errors.edgecases.test.ts` | 76 |
| `apps/web/src/__tests__/hooks/useItems.p1.test.ts` | 64 |
| `apps/web/src/__tests__/api/client.test.ts` | 59 |
| `apps/web/src/__tests__/security/input-validation.test.tsx` | 48 |
| `apps/web/src/__tests__/api/schema.test.ts` | 40 |
| `apps/web/src/__tests__/components/graph/SigmaGraphView.enhanced.test.tsx` | 38 |
| `apps/web/src/__tests__/utils/helpers.comprehensive.test.ts` | 34 |
| `apps/web/src/__tests__/hooks/useItems.comprehensive.test.tsx` | 27 |
| `apps/web/src/__tests__/components/JourneyExplorer.test.tsx` | 26 |
| `apps/web/src/__tests__/api/websocket.comprehensive.test.ts` | 25 |
| `apps/web/src/__tests__/workers/WorkerPool.edgecases.test.ts` | 25 |
| `apps/web/src/__tests__/types/typeGuards.test.ts` | 24 |
| `apps/web/src/__tests__/integration/app-integration.test.tsx` | 24 |
| `apps/web/src/__tests__/setup.ts` | 20 |

## First Production Patch Tranche

Prioritize production files with high error density and behavioral-risk rules before
test-only style rules.

| Order | File | Primary Rules | Why First |
| ---: | --- | --- | --- |
| 1 | `apps/web/src/lib/preflight.ts` | `no-unnecessary-type-assertion` (21), `prefer-add-event-listener` (3), `no-unsafe-type-assertion` (3) | High-density type and event-listener cleanup |
| 2 | `apps/web/src/lib/cache/IndexedDBCache.ts` | `prefer-add-event-listener` (12), `no-unsafe-type-assertion` (8), `no-base-to-string` (3), `restrict-template-expressions` (3) | Browser API and cache correctness risk |
| 3 | `apps/web/src/hooks/useIntegrationsDiscovery.ts` | `no-unsafe-type-assertion` (19) | Concentrated type-safety cluster |
| 4 | `apps/web/src/mocks/handlers.ts` | `no-unsafe-type-assertion` (17) | Mock contract typing affects broad tests |
| 5 | `apps/web/src/components/graph/utils/nodeDataTransformers.ts` | `no-unsafe-type-assertion` (16) | Graph data transform correctness |
| 6 | `apps/web/src/components/forms/CreateProblemForm.tsx` | `label-has-associated-control` (13), `click-events-have-key-events` (1), `no-unsafe-type-assertion` (1) | Accessibility correctness risk |
| 7 | `apps/web/src/components/specifications/items/ItemSpecTabs.tsx` | `no-unsafe-type-assertion` (8), `no-unused-vars` (6) | Type and dead-code cleanup |
| 8 | `apps/web/src/components/graph/DimensionFilters.tsx` | `no-unsafe-type-assertion` (12), `no-unused-vars` (1), `no-base-to-string` (1) | Filter typing correctness |

## Burn-down Update

The first production patch tranche is complete for targeted strict errors:

- `useRealtime.ts`, `IndexedDBCache.ts`, and `preflight.ts` pass targeted type-aware
  oxlint with warning-only output.
- `useIntegrationsDiscovery.ts` and `mocks/handlers.ts` pass targeted type-aware oxlint
  with warning-only output.
- `nodeDataTransformers.ts`, `DimensionFilters.tsx`, and `FlowGraphView.tsx` pass
  targeted type-aware oxlint with warning-only output.
- `CreateItemDialog.tsx`, `CreateProblemForm.tsx`, and `CreateProcessForm.tsx` pass
  targeted type-aware oxlint with warning-only output; `CreateProcessForm.tsx` labels
  were associated with controls to clear the remaining a11y errors.
- `ItemSpecTabs.tsx`, `FeatureListView.tsx`, and `DashboardView.tsx` pass targeted
  type-aware oxlint with warning-only output.
- `bun run typecheck:web` passes after integrating the tranche.
- Full `lint:type-aware:web:strict` still fails, with total strict errors reduced from
  2,503 to 2,035 after the seventh production tranche.

Additional completed production tranche:

- `edgeAggregation.ts` and `useTemporal.ts` pass targeted oxlint with no errors and
  `typecheck:web` passes.
- `GraphologyDataLayer.ts`, `IncrementalGraphBuilder.ts`, and `webVitals.ts` were
  processed; the first two were edited and the targeted lane reported no errors.
- `VirtualizedGraphView.tsx`, `SigmaGraphView.poc.tsx`, and `EnhancedGraphView.tsx`
  had strict `oxc` syntax findings reduced, but full strict scan still reports
  remaining non-oxc errors in these graph views.
- `ItemsKanbanView.tsx`, `useWebhooks.ts`, and `useStreaming.ts` pass targeted strict
  type-aware oxlint and `typecheck:web` passes.
- `GraphView.tsx` and `ItemSpecTabs.tsx` pass targeted strict type-aware oxlint with no
  `Error/...` diagnostics.
- `EnhancedGraphView.tsx` passes targeted strict type-aware oxlint with warning-only
  output after replacing unsafe graph callback, layout, Cytoscape option, and event
  handling assertions.
- `VirtualizedGraphView.tsx` and `SigmaGraphView.poc.tsx` pass targeted strict
  type-aware oxlint with no `Error/...` diagnostics; `typecheck:web` passes after the
  graph-view tranche.

The fifth production tranche is complete:

- `ItemsTableViewA11y.tsx` and `ItemsTableViewImpl.tsx` pass targeted strict type-aware
  oxlint.
- `WebhookIntegrationsView.tsx` and `useProjects.ts` pass targeted strict type-aware
  oxlint.
- `graphology/adapter.ts`, `graph/clustering.ts`, and `GraphologyDataLayer.ts` pass
  targeted strict type-aware oxlint.
- `normalize-item.ts`, `StreamingGraphView.tsx`, and `useGraphKeyboardShortcuts.ts`
  pass targeted strict type-aware oxlint.
- `bun run typecheck:web` passes after the tranche.

The sixth production tranche is complete:

- `ClusteredGraphView.tsx` and `GraphToolbar.tsx` pass targeted strict type-aware
  oxlint.
- `export-import.worker.ts` and `gpuForceLayout.worker.ts` pass targeted strict
  type-aware oxlint.
- `openapi-utils.ts` and `GraphologyDataLayer.example.tsx` pass targeted strict
  type-aware oxlint.
- `ContractEditor.tsx`, `PageInteractionFlow.tsx`, and `StreamingExample.tsx` pass
  targeted strict type-aware oxlint.
- Local small-file cleanup cleared strict errors in `useMcp.ts`, `form-validators.ts`,
  `ndjson-parser.ts`, and `screenshot.ts`.
- `bun run typecheck:web` passes after the tranche.

The seventh production tranche is complete:

- `ProjectDetailView.tsx` and `CreateLinkForm.tsx` pass targeted strict type-aware
  oxlint, and `typecheck:web` passes after the form path fix.
- `ItemSpecsOverview.tsx` passes targeted strict type-aware oxlint.
- `graphLayoutBenchmark.ts` and `SigmaGraphView.enhanced.tsx` pass targeted strict
  type-aware oxlint.
- `GraphLayoutWorkerDemo.stories.tsx` under `components/graph/__stories__/` passes
  targeted strict type-aware oxlint.
- `api/mcp-client.ts` and `pages/projects/views/TestRunView.tsx` pass targeted strict
  type-aware oxlint, with `TestRunView.tsx` label/control and enum filter narrowing
  repaired.
- `bun run typecheck:web` passes after the tranche.

## Next Production Patch Tranche

| Order | File(s) | Why Next |
| ---: | --- | --- |
| 1 | `apps/web/src/views/GraphView.tsx`, `apps/web/src/views/FeatureListView.tsx`, `apps/web/src/views/ADRDetailView.tsx` | App-facing view clusters at four errors each |
| 2 | `apps/web/src/stores/auth-store.ts`, `apps/web/src/lib/cache/CacheManager.ts`, `apps/web/src/lib/resilience/RetryPolicy.ts` | State/cache/resilience correctness clusters |
| 3 | `apps/web/src/pages/projects/views/TestSuiteView.tsx`, `apps/web/src/pages/projects/views/TestCaseView.tsx` | Project test management view clusters |
| 4 | `apps/web/src/lib/websocket.ts`, `apps/web/src/api/websocket.ts`, `apps/web/src/api/query-client.ts` | Runtime/API client strict errors |
| 5 | `apps/web/src/components/graph/EquivalenceImport.tsx`, `apps/web/src/components/graph/EquivalenceExport.tsx`, `apps/web/src/components/graph/utils/equivalenceIO.ts` | Graph equivalence import/export typing and a11y |

## Recommended Sequence

1. Fix production `no-floating-promises` and `react-hooks/exhaustive-deps` clusters first
   because they can hide runtime behavior bugs.
2. Fix production JSX accessibility clusters next, especially label/control and click
   keyboard parity.
3. Address production `no-unsafe-type-assertion` by introducing narrow type guards or
   parser helpers file-by-file; avoid blanket casts.
4. Defer Jest title, conditional-expect, and test naming backlog until production
   correctness errors are reduced.
5. Keep `react/react-in-jsx-scope` disabled; it is already verified as automatic JSX
   runtime config noise.

## Representative Production Errors

- `apps/web/src/lib/auth-utils.ts` `eslint(no-unused-vars)`: caught `error` is unused.
- `apps/web/src/views/LinksView.tsx` `eslint-plugin-react-hooks(exhaustive-deps)`:
  `useMemo` depends on `links`, which changes every render.
- `apps/web/src/hooks/useConnectionHealth.ts` `eslint-plugin-react-hooks(exhaustive-deps)`:
  missing dependency `setConnecting`.
- `apps/web/src/hooks/useMcp.ts` `eslint-plugin-react-hooks(exhaustive-deps)`:
  missing dependency `config`.
- `apps/web/src/hooks/use-gpu-compute.ts` `eslint-plugin-promise(always-return)`:
  `then()` chain does not return or throw.
- `apps/web/src/lib/validation/form-validators.ts` `eslint-plugin-promise(no-multiple-resolved)`:
  promise can be resolved multiple times.
- `apps/web/src/pages/settings/Settings.tsx` `eslint-plugin-jsx-a11y(label-has-associated-control)`:
  labels are not associated with controls.
- `apps/web/src/pages/projects/views/TestRunView.tsx` `eslint-plugin-jsx-a11y(label-has-associated-control)`:
  labels are not associated with controls.
- `apps/web/src/components/graph/ThumbnailPreview.tsx` `eslint-plugin-jsx-a11y(click-events-have-key-events)`:
  clickable non-interactive element lacks keyboard parity.
- `apps/web/src/components/graph/ClusterNode.tsx` `eslint-plugin-jsx-a11y(prefer-tag-over-role)`:
  should use a native `button`.

## Deferral Notes

- Test-only `jest(valid-title)` and `jest(no-conditional-expect)` findings are mostly
  recovered-suite style debt. They should be handled after production correctness and a11y
  errors because they do not change runtime behavior.
- Test-only `no-unsafe-type-assertion` should be reduced when touching those tests for
  functional reasons, but should not block production-source cleanup.
- Story findings are low volume and can be handled with the related graph demo/story tranche.
