# Architecture Decision Records: Benchora

## ADR-001: pytest-Based Benchmarking Framework

### Status
**Accepted** | 2026-04-05

### Context

Benchora needs to:
- Validate FR coverage
- Run performance benchmarks
- Integrate with CI/CD
- Support multiple languages

**Forces:**
- Python ecosystem familiarity
- pytest integration
- Statistical rigor
- Minimal setup

### Decision

Use **pytest with custom markers** as the foundation:

```python
# FR traceability
@pytest.mark.traces_to("FR-AGILE-012")
def test_dashboard_extraction():
    assert True

# Benchmarking with pytest-benchmark
@pytest.mark.benchmark
@pytest.mark.traces_to("FR-PERF-001")
def test_performance(benchmark):
    benchmark(target_function)
```

### Consequences

**Positive:**
- Familiar to Python developers
- Rich ecosystem
- Easy CI integration
- Extensible

**Negative:**
- Python-specific
- Overhead for micro-benchmarks
- Limited to test scope

**Mitigations:**
- Call out to language-specific tools
- Separate micro and macro benchmarks

### Alternatives Considered

| Approach | Pros | Cons | Decision |
|----------|------|------|----------|
| Custom framework | Tailored | Maintenance | Rejected |
| Language-specific | Optimal | Fragmentation | Rejected |
| pytest-based (chosen) | Unified, familiar | Python overhead | **Accepted** |

---

## ADR-002: FR Traceability via Markers

### Status
**Accepted** | 2026-04-05

### Context

Benchora validates FR coverage from AgilePlus. Need to:
- Link tests to requirements
- Generate coverage reports
- Track compliance

**Forces:**
- Unambiguous linking
- Tooling support
- Human readable

### Decision

Use **pytest markers for FR traceability**:

```python
@pytest.mark.traces_to("FR-AGILE-012")
def test_dashboard_extraction():
    """Test dashboard extraction functionality."""
    assert True
```

**Benefits:**
- Native pytest support
- Filterable: `pytest -m traces_to_FR_AGILE_012`
- Reportable via pytest hooks

### Consequences

**Positive:**
- Standard pytest feature
- No external dependencies
- Programmatic access

**Negative:**
- Marker names must be valid identifiers
- Limited metadata

**Mitigations:**
- Use consistent naming: `traces_to_FR_XXX_NNN`
- Store additional metadata in docstrings

---

## ADR-003: Multi-Language Benchmark Orchestration

### Status
**Accepted** | 2026-04-05

### Context

Phenotype ecosystem has Rust, Python, and Go components. Benchora should:
- Run benchmarks across languages
- Aggregate results
- Compare performance

**Forces:**
- Language-specific optimal tools
- Unified reporting
- CI integration

### Decision

**Orchestrate language-specific tools:**

```python
def run_rust_benchmarks():
    """Run Criterion.rs benchmarks."""
    subprocess.run(["cargo", "bench"], cwd="rust_project")
    # Parse criterion output

def run_python_benchmarks():
    """Run pytest-benchmark."""
    subprocess.run(["pytest", "--benchmark-only"])

def run_go_benchmarks():
    """Run Go benchmarks."""
    subprocess.run(["go", "test", "-bench=."], cwd="go_project")
```

### Consequences

**Positive:**
- Uses best tool for each language
- No performance overhead
- Standard formats

**Negative:**
- Multiple output formats
- Complex aggregation
- More setup

**Mitigations:**
- Normalize output to common format
- Provide unified reporting
- Container-based isolation

---

## ADR Index

| ID | Title | Status | Date |
|----|-------|--------|------|
| 001 | pytest-Based Framework | Accepted | 2026-04-05 |
| 002 | FR Traceability via Markers | Accepted | 2026-04-05 |
| 003 | Multi-Language Orchestration | Accepted | 2026-04-05 |

---

*End of Architecture Decision Records*
