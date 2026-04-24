# API Reference

## Crate focus

`phenotype-gauge` is a Rust crate for benchmark orchestration and xDD-style test
support.

## Public areas

| Area | Purpose |
| --- | --- |
| `benchmarking` | Criterion-backed benchmark execution and report generation |
| `proptest` | Property-based test strategies and helpers |
| `quickcheck` | QuickCheck-oriented compatibility helpers |
| `mutation` | Mutation-testing utilities for test-sensitivity checks |
| `contracts` | Contract and invariant verification helpers |
| `reporters` | HTML, JSON, and CSV output layers |

## Boundaries

- benchmarking and xDD validation belong here
- runtime metrics collection belongs in dedicated telemetry crates
- docs examples should stay aligned with the exported crate surface
