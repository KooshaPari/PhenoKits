# Implementation Plan — phenotype-gauge

## Phase 1: Core Benchmarking (Complete)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P1.1 | Criterion-based benchmark runner (`src/benchmarking/`) | — | Done |
| P1.2 | Statistical analysis: mean, median, p95, p99, stddev | P1.1 | Done |
| P1.3 | `benchmark!` and `group!` macros | P1.1 | Done |
| P1.4 | Concurrent load benchmarking support | P1.1 | Done |
| P1.5 | Benchmark comparison across runs | P1.1 | Done |

## Phase 2: xDD Testing Utilities (Complete)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P2.1 | Property testing strategies (`src/proptest/`) | — | Done |
| P2.2 | QuickCheck integration (`src/quickcheck/`) | — | Done |
| P2.3 | Mutation testing coverage utilities (`src/mutation/`) | — | Done |
| P2.4 | Contract testing / SpecDD (`src/contracts/`) | — | Done |

## Phase 3: Reporters (Complete)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P3.1 | HTML report with interactive flamegraphs/charts | P1.2 | Done |
| P3.2 | JSON reporter for CI/CD integration | P1.2 | Done |
| P3.3 | CSV reporter for spreadsheet analysis | P1.2 | Done |

## Phase 4: Quality and CI (Planned)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P4.1 | Clippy + rustfmt enforcement in CI | — | Planned |
| P4.2 | Unit tests for statistical analysis functions | P1.2 | Planned |
| P4.3 | FR traceability markers in all test files | P4.2 | Planned |
| P4.4 | cargo audit + cargo deny for supply-chain security | — | Planned |
| P4.5 | USER_JOURNEYS.md and docs coverage | — | Done |

## Phase 5: Extensions (Planned)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P5.1 | Async benchmark support (tokio-based) | P1.1 | Planned |
| P5.2 | Regression detection: flag p95 regressions vs baseline | P1.5 | Planned |
| P5.3 | Integration with thegent observability stack | P3.2 | Planned |

## DAG

```
P1.1 -> P1.2 -> P1.3, P1.4, P1.5 -> P3.*
P2.1, P2.2, P2.3, P2.4  (independent)
P1.2 -> P4.2 -> P4.3
P1.1 -> P5.1
P1.5 -> P5.2
P3.2 -> P5.3
```
