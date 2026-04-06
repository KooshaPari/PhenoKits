# FR-HTTP-008: Circuit Breaker

## ID
- **FR-ID**: FR-HTTP-008
- **Repository**: Httpora
- **Domain**: Middleware

## Description

The system SHALL implement a circuit breaker pattern that opens after a failure threshold and closes after a timeout.

## Acceptance Criteria

- [ ] `CircuitBreaker::new(failure_threshold, timeout)` creates breaker
- [ ] Tracks success/failure ratio
- [ ] Opens circuit when threshold exceeded
- [ ] Returns error immediately when open

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `tests/circuit_breaker_tests.rs` | `test_circuit_breaker` | `// @trace FR-HTTP-008` |

## Code References

| File | Function/Struct | FR Reference |
|------|-----------------|--------------|
| `src/middleware/circuit_breaker.rs` | `CircuitBreaker` | `// @trace FR-HTTP-008` |

## Related FRs

- FR-HTTP-009: Half-Open State

## Status

- **Current**: implemented
- **Since**: 2026-02-05
