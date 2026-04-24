# FR-HTTP-002: Token Bucket Rate Limiter

## ID
- **FR-ID**: FR-HTTP-002
- **Repository**: Httpora
- **Domain**: Middleware

## Description

The system SHALL implement a token bucket rate limiter with configurable capacity and refill rate.

## Acceptance Criteria

- [ ] `RateLimiter::token_bucket(capacity, refill_per_sec)` creates limiter
- [ ] Tracks token consumption per request
- [ ] Refills tokens at specified rate
- [ ] Rejects requests when bucket empty

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `tests/rate_limit_tests.rs` | `test_token_bucket` | `// @trace FR-HTTP-002` |

## Code References

| File | Function/Struct | FR Reference |
|------|-----------------|--------------|
| `src/middleware/rate_limit.rs` | `token_bucket()` | `// @trace FR-HTTP-002` |

## Related FRs

- FR-HTTP-003: Fixed Window Rate Limiter

## Status

- **Current**: implemented
- **Since**: 2026-01-15
