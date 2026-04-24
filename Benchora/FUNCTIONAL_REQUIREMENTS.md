# Functional Requirements — Benchora

Traces to: PRD.md epics E1–E5.
ID format: FR-BENCHORA-{NNN}.

---

## Benchmark Framework

**FR-BENCHORA-001**: The system SHALL execute parametrized microbenchmarks against provided Rust code and collect wall-clock timing data.
Traces to: E1.1

**FR-BENCHORA-002**: The system SHALL compute benchmark statistics (mean, median, stddev, percentiles) across multiple runs and report regressions when a new result deviates by [threshold]% from baseline.
Traces to: E1.2

**FR-BENCHORA-003**: The system SHALL persist benchmark baselines and historical results to enable regression detection and trend analysis.
Traces to: E1.3

---

## Reporting & Visualization

**FR-BENCHORA-004**: The system SHALL generate markdown reports with tabular benchmark results, flamegraph links, and regression alerts.
Traces to: E2.1

**FR-BENCHORA-005**: The system SHALL support HTML and JSON output formats for programmatic consumption and CI integration.
Traces to: E2.2

---

## Trace & Test Guidance

All tests MUST reference a Functional Requirement (FR):

```rust
// Traces to: FR-BENCHORA-NNN
#[bench]
fn benchmark_feature() { ... }
```
