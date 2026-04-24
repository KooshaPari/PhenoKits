# FR-HTTP-004: Rate Limit Response

## ID
- **FR-ID**: FR-HTTP-004
- **Repository**: Httpora
- **Domain**: Middleware

## Description

When a request is rate-limited, the response SHALL be HTTP 429 Too Many Requests with a `Retry-After` header indicating seconds until retry.

## Acceptance Criteria

- [ ] Returns HTTP 429 status code
- [ ] Includes `Retry-After` header
- [ ] Calculates correct retry time
- [ ] Includes rate limit info in body

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `tests/rate_limit_tests.rs` | `test_rate_limit_response` | `// @trace FR-HTTP-004` |

## Code References

| File | Function/Struct | FR Reference |
|------|-----------------|--------------|
| `src/middleware/rate_limit.rs` | `RateLimitedResponse` | `// @trace FR-HTTP-004` |

## Related FRs

- FR-HTTP-002: Token Bucket Rate Limiter
- FR-HTTP-003: Fixed Window Rate Limiter

## Status

- **Current**: implemented
- **Since**: 2026-01-20
