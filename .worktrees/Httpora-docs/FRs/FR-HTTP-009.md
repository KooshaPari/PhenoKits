# FR-HTTP-009: Half-Open Circuit State

## ID
- **FR-ID**: FR-HTTP-009
- **Repository**: Httpora
- **Domain**: Middleware

## Description

After the circuit breaker timeout, it SHALL enter half-open state and allow one probe request. On success, the circuit closes; on failure, it reopens.

## Acceptance Criteria

- [ ] Enters half-open after timeout
- [ ] Allows exactly one probe request
- [ ] Closes circuit on probe success
- [ ] Reopens circuit on probe failure

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `tests/circuit_breaker_tests.rs` | `test_half_open_state` | `// @trace FR-HTTP-009` |

## Code References

| File | Function/Struct | FR Reference |
|------|-----------------|--------------|
| `src/middleware/circuit_breaker.rs` | `try_half_open()` | `// @trace FR-HTTP-009` |

## Related FRs

- FR-HTTP-008: Circuit Breaker

## Status

- **Current**: implemented
- **Since**: 2026-02-10
