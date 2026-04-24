# ADR-018: Testing Strategy - Contract + Property-Based Testing

**Status**: Accepted
**Date**: 2026-03-25

## Context

We need a robust testing strategy that provides:
1. High confidence in correctness (not just coverage)
2. Discovery of edge cases automatically
3. Clear behavior documentation via BDD
4. Property-based invariants that catch regressions

## Decision

We will use a layered testing approach combining:

1. **Contract Tests** (`tests/contract/`)
   - Verify behavior of a single module against a defined contract
   - BDD-style: Given-When-Then scenario naming
   - Test invariants that must hold for all inputs

2. **Property-Based Tests** (`tests/property/`)
   - Use Hypothesis to generate random valid inputs
   - Verify properties that hold for ALL inputs
   - Automatically shrink failing examples to minimal repro

3. **Integration Tests** (`tests/integration/`)
   - (Future) Test interactions between modules

## Implementation

### Contract Tests Pattern
```python
class TestModuleContract:
    """Contract tests for module behavior.

    Given: [precondition]
    When: [action]
    Then: [expected outcome]
    """

    def test_scenario_name(self) -> None:
        """Description of the scenario."""
        # Given
        ...

        # When
        ...

        # Then
        assert ...
```

### Property-Based Tests Pattern
```python
from hypothesis import given, settings, strategies as st

@given(st.integers(min_value=1, max_value=100))
@settings(max_examples=100)
def test_property_name(self, value: int) -> None:
    """Property that must hold for all valid inputs."""
    assert some_invariant(value)
```

## Consequences

### Positive
- Higher confidence than traditional example-based tests
- Automatic edge case discovery
- Clear documentation of expected behavior
- Falsifying examples are automatically minimized

### Negative
- Requires understanding of property-based testing
- May be slower to run (many examples)
- Some properties may be hard to identify

## xDD Methodologies Applied

| Category | Methodology | Application |
|----------|-------------|-------------|
| Development | TDD | Contract tests written before implementation |
| | BDD | Given-When-Then scenario naming |
| | DDD | Domain invariants tested as properties |
| | CDD | Contract tests define adapter interfaces |
| | SpecDD | This ADR documents the specification |
| Quality | Property-Based | Hypothesis for invariant testing |
| Design | DRY | Shared test infrastructure |
| | KISS | Simple contract/property patterns |

## References

- [Hypothesis Documentation](https://hypothesis.readthedocs.io/)
- [Property-Based Testing Book](https://pragprog.com/book/bhprog/property-based-testing-with-proptest)
- [Contract Tests Martin Fowler](https://martinfowler.com/bliki/ContractTest.html)
