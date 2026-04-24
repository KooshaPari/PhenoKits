# FR-HTTP-006: Exponential Backoff

## ID
- **FR-ID**: FR-HTTP-006
- **Repository**: Httpora
- **Domain**: Middleware

## Description

Retry delays SHALL use exponential backoff: `base_delay * 2^attempt + jitter`. Jitter SHALL prevent thundering herd.

## Acceptance Criteria

- [ ] Calculates delay: base * 2^attempt
- [ ] Adds random jitter (0-25% of delay)
- [ ] Respects max delay cap
- [ ] Uses different jitter per retry

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `tests/retry_tests.rs` | `test_exponential_backoff` | `// @trace FR-HTTP-006` |

## Code References

| File | Function/Struct | FR Reference |
|------|-----------------|--------------|
| `src/middleware/retry.rs` | `calculate_backoff()` | `// @trace FR-HTTP-006` |

## Related FRs

- FR-HTTP-005: Retry Layer

## Status

- **Current**: implemented
- **Since**: 2026-01-25
