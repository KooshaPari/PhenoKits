# Benchora

**FR Validation & Performance Benchmarking Framework** — Automated testing infrastructure for Functional Requirements traceability, performance benchmarking, and CI/CD integration across the Phenotype ecosystem.

## Overview

Benchora is a comprehensive testing and benchmarking framework designed to validate that all code changes are traceable to documented Functional Requirements (FRs) and to measure performance characteristics across Phenotype ecosystem components. It provides multi-language support (Rust, Python, Go) with seamless CI/CD integration and historical tracking capabilities.

**Core Mission**: Ensure 100% FR traceability and maintain performance baselines through automated, multi-language benchmarking integrated into the development pipeline.

## Technology Stack

- **Languages**: Python, Rust, Go (multi-language test framework)
- **Test Framework**: pytest (Python), cargo test (Rust), go test (Go)
- **Benchmarking**: pytest-benchmark, criterion (Rust), testing.B (Go)
- **CI Integration**: GitHub Actions, automated FR coverage analysis
- **Reporting**: JSON export, historical trend tracking, metrics dashboards

## Key Features

- **FR Traceability Testing**: Validates that all tests reference FRs (FR-XXX-NNN format)
- **Performance Benchmarking**: Track performance across releases and identify regressions
- **Multi-Language Support**: Unified benchmarking across Rust, Python, and Go
- **CI/CD Integration**: Automated test execution, FR coverage analysis, performance reporting
- **Historical Tracking**: Performance baselines, trend analysis, regression detection
- **Coverage Validation**: Ensure 100% of FRs have corresponding tests
- **Governance Enforcement**: Automatic validation of coding standards and documentation

## Quick Start

```bash
# Clone and setup
git clone https://github.com/KooshaPari/Phenotype repos/Benchora
cd Benchora

# Review governance and specifications
cat CLAUDE.md
cat SPEC.md

# Install dependencies (Python)
pip install -r requirements.txt

# Run FR traceability tests
python3 -m pytest tests/test_fr_coverage.py -v

# Run performance benchmarks
python3 -m pytest tests/benchmarks/ --benchmark-only

# Validate governance
python3 validate_governance.py

# Full quality check
task quality
```

## Project Structure

```
benchora/
├── tests/                         # Test suite
│   ├── test_fr_coverage.py       # FR traceability validation
│   ├── test_benchmarks.py        # Performance baselines
│   └── benchmarks/               # Criterion/pytest-benchmark specs
├── validate_governance.py         # Governance validation script
├── scripts/                       # Automation and reporting
│   ├── analyze_coverage.py       # FR coverage analysis
│   ├── track_performance.py      # Baseline tracking
│   └── generate_report.py        # Benchmark reporting
├── specs/                        # Feature specifications
├── SPEC.md                       # Complete specification
├── CHARTER.md                    # Project charter and goals
├── ARCHITECTURE.md               # System design
├── ADR.md                        # Architecture decision records
├── SOTA.md                       # State-of-the-art research
├── FUNCTIONAL_REQUIREMENTS.md    # FR definitions and tracability
├── plan.md                       # Implementation plan
├── PRD.md                        # Product requirements
└── CLAUDE.md                     # Development guidelines
```

## FR Coverage Requirements

All tests MUST reference a Functional Requirement using one of:

**Python (pytest)**:
```python
@pytest.mark.traces_to("FR-BENCH-001")
def test_fr_coverage_validation():
    """Test that FR coverage validation works."""
    assert True
```

**Rust (cargo test)**:
```rust
#[test]
fn test_fr_bench_001() {
    // Traces to: FR-BENCH-001
    assert!(true);
}
```

**Go (testing)**:
```go
// TracesTo: FR-BENCH-001
func TestFRBench001(t *testing.T) {
    if !true {
        t.Fail()
    }
}
```

## Performance Benchmarking

**Baseline Tracking**:
```bash
# Run and save baseline
python3 scripts/track_performance.py --save-baseline v1.0.0

# Compare against baseline
python3 -m pytest tests/benchmarks/ --benchmark-compare=v1.0.0
```

**Supported Metrics**:
- Execution time (min/max/avg/stddev)
- Memory usage (peak, allocation rate)
- Cache hit rates
- Throughput (ops/sec)
- Latency percentiles (p50, p95, p99, p99.9)

## CI Integration

Benchora runs automatically on every pull request:

1. **FR Traceability Check**: Ensures all tests reference valid FRs
2. **Coverage Analysis**: Reports FR coverage % and identifies gaps
3. **Performance Benchmarks**: Runs and compares against baseline
4. **Regression Detection**: Alerts if metrics regress >5%
5. **Governance Validation**: Checks code style, documentation, security

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| FR Coverage | 100% | Draft |
| Test Execution Time | <5 minutes | Draft |
| False Positive Rate | <5% | Draft |
| Benchmark Accuracy | ±3% variance | Draft |

## Related Phenotype Projects

- **AgilePlus** — Work tracking and spec management (integrates FR definitions)
- **Tracera** — Distributed tracing and observability
- **PhenoObservability** — Monitoring and metrics collection
- **phenotype-shared** — Shared testing utilities

## Documentation

- [SPEC.md](./SPEC.md) - Complete specification and architecture
- [CHARTER.md](./CHARTER.md) - Project charter and strategic goals
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System design and components
- [ADR.md](./ADR.md) - Architecture decision records
- [SOTA.md](./SOTA.md) - State-of-the-art research and references
- [FUNCTIONAL_REQUIREMENTS.md](./FUNCTIONAL_REQUIREMENTS.md) - FR definitions
- [plan.md](./plan.md) - Implementation roadmap

## Governance

- [CLAUDE.md](./CLAUDE.md) - Development guidelines
- Build: `cargo build` or `pytest` depending on language
- Test: `python3 -m pytest tests/ -v`
- Quality: `task quality`
- Validation: `python3 validate_governance.py`

## License

Apache 2.0 — Part of Phenotype organization ecosystem
