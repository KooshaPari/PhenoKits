# User Journeys — phenotype-gauge

---

## UJ-1: Developer Benchmarking a Function

**Actor**: Rust developer
**Goal**: Measure function performance with statistical analysis.

```
Developer Code               phenotype-gauge
    |                              |
    |-- benchmark!("my_fn")        |
    |   .run(|| my_fn()) -------> |
    |                       [run N iterations (criterion-based)]
    |                       [compute mean, median, p95, p99, stddev]
    |<-- BenchmarkResult -------- |
    |                              |
    |-- BenchmarkResult::report()  |
    |   HTML report generated      |
```

**Steps**:
1. Developer uses `benchmark!("name").run(|| { /* code */ })` macro.
2. Gauge runs the closure for a statistically meaningful sample.
3. Computes mean, median, p95, p99, stddev.
4. Returns `BenchmarkResult` with all statistics.
5. Developer calls `.report()` to emit HTML report with flamegraphs.

---

## UJ-2: Running Load / Concurrency Benchmark

**Actor**: Performance engineer
**Goal**: Measure throughput under concurrent load.

```
Engineer Code               phenotype-gauge
    |                              |
    |-- benchmark!("api_handler")  |
    |   .concurrency(16)           |
    |   .run(|| handler()) ------> |
    |                       [spawn 16 concurrent goroutines]
    |                       [measure aggregate throughput]
    |                       [collect per-thread latency distributions]
    |<-- LoadBenchmarkResult ----- |
```

**Steps**:
1. Engineer uses `.concurrency(N)` to set parallel worker count.
2. Gauge spawns N threads/tasks, each running the benchmark closure.
3. Aggregate throughput (req/s) and per-thread latency distributions collected.
4. Results included in HTML report with concurrent breakdown.

---

## UJ-3: Property Testing with Custom Strategies

**Actor**: Developer writing property-based tests
**Goal**: Generate structured test inputs using gauge's proptest strategies.

```rust
use gauge::proptest::{regex_strategy, numeric_range};

proptest!(|(s in regex_strategy(r"\w+@\w+\.\w+"))| {
    prop_assert!(validate_email(&s));
});
```

**Steps**:
1. Developer imports strategy helpers from `gauge::proptest`.
2. Combines with `proptest!` macro to generate typed inputs.
3. Runs 1000+ test cases automatically.
4. Failure cases shrunk and reported with minimal reproducing input.

---

## UJ-4: Contract Testing a Port/Adapter Pair

**Actor**: Architect verifying a hexagonal architecture boundary
**Goal**: Ensure an adapter correctly implements its port contract.

```rust
use gauge::contracts::{ContractTest, PortVerifier};

ContractTest::for_port::<UserRepository>()
    .with_adapter(SqlUserRepository::new(pool))
    .verify_all_contracts();
```

**Steps**:
1. Developer calls `ContractTest::for_port::<PortTrait>()`.
2. Binds a concrete adapter implementation.
3. `verify_all_contracts()` runs all port-defined contract assertions.
4. Mismatches reported with port method name and failing assertion.

---

## UJ-5: Comparing Benchmarks Across Runs (Regression Detection)

**Actor**: CI pipeline
**Goal**: Detect performance regressions automatically.

```
CI Pipeline                 phenotype-gauge
    |                              |
    |-- run benchmarks ----------> |
    |                       [compare p95 vs saved baseline]
    |                       [if p95 degrades > 5%: flag regression]
    |<-- BenchmarkComparison ----- |
    |   { regressions: ["my_fn: +12% p95"] }
    |                              |
    |-- exit 1 if regressions ---> |
```

**Steps**:
1. CI runs benchmark suite; results saved as JSON artifact.
2. Next run loads previous JSON as baseline.
3. p95 latency compared per benchmark.
4. Regressions > configurable threshold reported; CI exits non-zero.
5. Developer sees clear output: `my_fn: p95 regressed 12% (was 45ms, now 50ms)`.
