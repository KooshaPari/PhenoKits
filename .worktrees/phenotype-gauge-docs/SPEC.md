# Phenotype Gauge Specification

> Modern Benchmarking and xDD Testing Framework

**Version**: 0.2.0 | **Status**: Archived | **Last Updated**: 2026-04-02

## Overview

Benchmarking and xDD (cross-Driven Development) testing framework for Rust with statistical analysis and HTML reporting.

**Language**: Rust 2021 Edition

**Key Features**:
- Statistical benchmark analysis (mean, median, p95, p99, stddev)
- Interactive HTML reports with flamegraphs
- Property-based testing strategies
- Contract testing for hexagonal architecture
- Mutation testing utilities
- SpecDD (Specification-Driven Development) support

## Architecture

```
phenotype-gauge/
├── src/
│   ├── lib.rs              # Public API
│   ├── benchmark.rs        # Benchmark runner
│   ├── stats.rs            # Statistical analysis
│   ├── report/             # HTML report generation
│   │   ├── html.rs
│   │   ├── flamegraph.rs
│   │   └── chart.rs
│   ├── xdd/                # xDD testing
│   │   ├── property.rs     # Property-based testing
│   │   ├── contract.rs     # Contract testing
│   │   └── mutation.rs     # Mutation testing
│   └── strategies/         # Test strategies
│       ├── uuid.rs
│       ├── email.rs
│       └── url.rs
├── benches/               # Benchmark examples
└── tests/                 # Integration tests
```

## Core Types

### Benchmark

```rust
pub struct Benchmark {
    name: String,
    iterations: u64,
    warmup_iterations: u64,
}

impl Benchmark {
    pub fn run<F, T>(&self, f: F) -> BenchmarkResult
    where F: Fn() -> T;
}
```

### BenchmarkResult

```rust
pub struct BenchmarkResult {
    pub name: String,
    pub mean_ns: f64,
    pub median_ns: f64,
    pub p95_ns: f64,
    pub p99_ns: f64,
    pub stddev_ns: f64,
}
```

### Contract

```rust
pub trait Contract {
    fn name() -> &'static str;
    fn verify() -> XddResult<()>;
}
```

## Quick Start

```bash
cargo add gauge --git https://github.com/KooshaPari/gauge
```

```rust
use gauge::{benchmark, group};

// Simple benchmark
benchmark!("my_function").run(|| {
    my_function();
});

// Grouped benchmarks
group!("string_ops", || {
    benchmark!("concat").run(|| format!("{} {}", "a", "b"));
    benchmark!("join").run(|| ["a", "b"].join(" "));
});
```

## Dependencies

- `criterion` - Benchmark harness
- `proptest` - Property-based testing
- `plotters` - Chart generation
- `inferno` - Flamegraph generation
- `serde` - Serialization
- `thiserror` - Error handling

## Performance Targets

| Metric | Target |
|--------|--------|
| Benchmark overhead | < 1% |
| Report generation | < 5s for 1000 benchmarks |
| Memory per benchmark | < 10KB |

## Archive Status

**Status**: Archived (2026-03-25)
**Migration**: Migrated to `libs/gauge/`

## License

MIT
