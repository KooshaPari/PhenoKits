# AGENTS.md — Benchora

## Project Overview

- **Name**: Benchora (Benchmarking & Performance Platform)
- **Description**: Comprehensive benchmarking platform for measuring agent performance, model evaluation, and system throughput with statistical rigor
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/Benchora`
- **Language Stack**: Rust (Edition 2024), Python 3.12+, PostgreSQL
- **Published**: Private (Phenotype org)

## Quick Start Commands

```bash
# Clone and setup
git clone https://github.com/KooshaPari/Benchora.git
cd Benchora

# Install Rust toolchain
rustup update nightly
rustup default nightly

# Build
cargo build --release

# Run tests
cargo test
cargo nextest run

# Run benchmarks
cargo bench

# Setup database
cargo run --bin benchora -- db setup
```

## Architecture

### Benchmarking Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Benchmark Definition Layer                         │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   Test Suite      │  │   Scenarios     │  │   Metrics       │         │
│  │   (Collections)   │  │   (Use Cases)   │  │   (KPIs)        │         │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘         │
└───────────┼────────────────────┼────────────────────┼────────────────┘
            │                    │                    │
            ▼                    ▼                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Execution Engine (Rust)                             │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │                    Benchora Core                               │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐            │   │
│  │  │   Runner   │  │   Harness  │  │   Observer │            │   │
│  │  │   (Async)  │  │   (Setup)  │  │   (Monitor)│            │   │
│  │  └────────────┘  └────────────┘  └────────────┘            │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐            │   │
│  │  │   Isolation│  │   Retry    │  │   Timeout  │            │   │
│  │  │   (Env)    │  │   Logic    │  │   Control  │            │   │
│  │  └────────────┘  └────────────┘  └────────────┘            │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Analysis & Reporting Layer                        │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   Statistics      │  │   Comparison    │  │   Reporting     │         │
│  │   Engine          │  │   Engine        │  │   Generator     │         │
│  │                   │  │                   │  │                   │         │
│  │  • Mean/Median    │  │  • Delta Calc   │  │  • HTML/PDF     │         │
│  │  • Percentiles    │  │  • Significance │  │  • JSON/CSV     │         │
│  │  • Regression     │  │  • Trend Detect │  │  • Dashboard    │         │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘         │
└─────────────────────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Storage Layer                                     │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   PostgreSQL      │  │   Time-Series   │  │   Object        │         │
│  │   (Results)       │  │   DB (Metrics)  │  │   Store         │         │
│  │                   │  │                   │  │   (Artifacts)   │         │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘         │
└─────────────────────────────────────────────────────────────────────┘
```

### Benchmark Lifecycle

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Benchmark Execution Flow                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐      │
│  │ Define   │───▶│ Prepare  │───▶│ Execute  │───▶│ Analyze  │      │
│  │          │    │          │    │          │    │          │      │
│  └──────────┘    └──────────┘    └──────────┘    └──────────┘      │
│       │               │               │               │             │
│       ▼               ▼               ▼               ▼             │
│  Load Config    Provision       Run Iterations   Statistical        │
│  Validate       Isolate Env     Collect Data    Analysis            │
│                                                                      │
│  ┌──────────┐    ┌──────────┐                                      │
│  │ Report   │◀───│ Store    │                                      │
│  │          │    │          │                                      │
│  └──────────┘    └──────────┘                                      │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Quality Standards

### Rust Code Quality

- **Formatter**: `rustfmt` (nightly)
- **Linter**: `clippy --all-targets --all-features -- -D warnings`
- **Tests**: `cargo nextest run` with coverage >80%
- **Benchmarks**: Statistical significance testing required
- **Documentation**: `cargo doc --no-deps`

### Statistical Rigor

- Minimum 30 iterations for statistical validity
- Outlier detection and handling (IQR method)
- Confidence intervals (95% default)
- Warmup iterations before measurement
- Noise reduction techniques

### Test Requirements

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Nextest (preferred)
cargo nextest run

# With coverage
cargo tarpaulin --out lcov

# Benchmark tests
cargo test --features benchmark-tests
```

## Git Workflow

### Branch Naming

Format: `<type>/<component>/<description>`

Types: `feat`, `fix`, `docs`, `refactor`, `perf`, `bench`

Examples:
- `feat/runner/add-async-support`
- `fix/stats/handle-outliers`
- `bench/agent/throughput-suite`
- `refactor/storage/extract-trait`

### Commit Messages

Format: `<type>(<scope>): <description>`

Examples:
- `feat(runner): implement async benchmark execution`
- `fix(stats): correct percentile calculation for edge cases`
- `bench(agent): add throughput measurement suite`
- `refactor(storage): extract database trait for testability`

## File Structure

```
Benchora/
├── src/
│   ├── bin/                # Binary entry points
│   │   └── benchora.rs     # Main CLI binary
│   ├── lib.rs              # Library root
│   ├── core/               # Core engine
│   │   ├── runner.rs       # Benchmark runner
│   │   ├── harness.rs      # Test harness
│   │   ├── observer.rs     # Metrics observer
│   │   └── isolation.rs    # Environment isolation
│   ├── stats/              # Statistics
│   │   ├── analysis.rs     # Statistical analysis
│   │   ├── regression.rs   # Regression detection
│   │   └── comparison.rs   # Result comparison
│   ├── storage/            # Data persistence
│   │   ├── database.rs     # PostgreSQL adapter
│   │   ├── timeseries.rs   # Time-series storage
│   │   └── artifacts.rs    # Artifact storage
│   └── report/             # Reporting
│       ├── generator.rs    # Report generation
│       ├── html.rs         # HTML formatter
│       └── json.rs         # JSON formatter
├── benches/                # Internal benchmarks
├── tests/                  # Integration tests
├── suites/                 # Benchmark suites
└── docs/                   # Documentation
```

## CLI Commands

### Benchmark Operations

```bash
# Run benchmark suite
cargo run --bin benchora -- run --suite agent-performance

# Run single benchmark
cargo run --bin benchora -- run --bench throughput-test

# Run with specific iterations
cargo run --bin benchora -- run --iterations 100

# Run with warmup
cargo run --bin benchora -- run --warmup 10
```

### Analysis Commands

```bash
# Compare results
cargo run --bin benchora -- compare run-1.json run-2.json

# Generate report
cargo run --bin benchora -- report --input results.json --format html

# Detect regression
cargo run --bin benchora -- regression --baseline baseline.json --current current.json

# Export to CSV
cargo run --bin benchora -- export --input results.json --format csv
```

### Database Operations

```bash
# Setup database
cargo run --bin benchora -- db setup

# Run migrations
cargo run --bin benchora -- db migrate

# View history
cargo run --bin benchora -- db history

# Export database
cargo run --bin benchora -- db export --output backup.sql
```

### Suite Management

```bash
# List suites
cargo run --bin benchora -- suite list

# Validate suite
cargo run --bin benchora -- suite validate --file suite.yaml

# Import suite
cargo run --bin benchora -- suite import --file suite.yaml
```

## Troubleshooting

### Benchmark Failures

```bash
# Test not completing
# 1. Check timeout settings
cargo run --bin benchora -- run --timeout 300

# 2. Enable verbose logging
RUST_LOG=debug cargo run --bin benchora -- run

# 3. Run without isolation
cargo run --bin benchora -- run --no-isolation

# 4. Check resource limits
ulimit -a
```

### Statistical Issues

```bash
# High variance in results
# 1. Increase iterations
cargo run --bin benchora -- run --iterations 100

# 2. Enable outlier removal
cargo run --bin benchora -- run --remove-outliers

# 3. Check for external load
htop  # or Activity Monitor

# 4. Pin to CPU cores
cargo run --bin benchora -- run --cpu-affinity 0,1,2,3
```

### Database Issues

```bash
# Connection failures
# 1. Check PostgreSQL
pg_isready -h localhost

# 2. Verify connection string
echo $DATABASE_URL

# 3. Test connection
cargo run --bin benchora -- db test

# 4. Reset database
cargo run --bin benchora -- db reset
```

### Build Failures

```bash
# Compilation errors
# 1. Update toolchain
rustup update

# 2. Clean build
cargo clean
cargo build

# 3. Check dependencies
cargo tree | grep conflict

# 4. Update lockfile
rm Cargo.lock
cargo update
```

## Environment Variables

```bash
# Database
DATABASE_URL=postgresql://benchora:pass@localhost:5432/benchora

# Performance
BENCHORA_WARMUP_ITERATIONS=10
BENCHORA_MIN_ITERATIONS=30
BENCHORA_TIMEOUT=300
BENCHORA_CPU_AFFINITY=0,1,2,3

# Statistics
BENCHORA_CONFIDENCE_LEVEL=0.95
BENCHORA_OUTLIER_THRESHOLD=1.5

# Reporting
BENCHORA_OUTPUT_FORMAT=json
BENCHORA_REPORT_DIR=./reports
```

## Benchmark Suites

| Suite | Purpose | Duration | Status |
|-------|---------|----------|--------|
| agent-performance | Agent throughput | ~10min | ✅ Active |
| model-evaluation | LLM benchmarks | ~30min | ✅ Active |
| system-throughput | Load testing | ~5min | ✅ Active |
| regression-check | Nightly checks | ~60min | ✅ Active |
| micro-benchmarks | Component perf | ~2min | ✅ Active |

## Integration Points

| System | Protocol | Purpose |
|--------|----------|---------|
| PhenoMCP | REST | Agent benchmarking |
| HeliosApp | gRPC | System performance |
| TheGent | REST | Automation metrics |
| Portage | Events | Build performance |

## Governance Rules

### Mandatory Checks

1. **FR Traceability**
   - All tests MUST reference FR-XXX-NNN
   - Use: #[trace_to()]

2. **AI Attribution**
   - .phenotype/ai-traceability.yaml MUST exist
   - MUST be updated on every AI-generated change

3. **CI/CD Compliance**
   - .github/workflows/traceability.yml MUST pass
   - No merges with drift > 90%

4. **Code Quality**
   - All code MUST have corresponding tests
   - Minimum 80% coverage for new code

### Prohibited Actions

- ❌ Delete without read first
- ❌ Modify without FR reference
- ❌ Skip validation on merge

### Validation

Run before any commit:
```bash
python3 validate_governance.py
```

Must pass all checks before PR.

---

Last Updated: 2026-04-05
Version: 1.0.0
