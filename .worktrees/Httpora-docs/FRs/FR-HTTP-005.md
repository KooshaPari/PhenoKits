# FR-HTTP-005: Retry Layer

## ID
- **FR-ID**: FR-HTTP-005
- **Repository**: Httpora
- **Domain**: Middleware

## Description

The system SHALL implement a retry layer that automatically retries failed requests with exponential backoff.

## Acceptance Criteria

- [ ] `RetryLayer::new(max_retries, backoff_config)` creates layer
- [ ] Retries on configurable status codes (5xx, 429)
- [ ] Uses exponential backoff between retries
- [ ] Respects max retry count

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `tests/retry_tests.rs` | `test_retry_layer` | `// @trace FR-HTTP-005` |

## Code References

| File | Function/Struct | FR Reference |
|------|-----------------|--------------|
| `src/middleware/retry.rs` | `RetryLayer` | `// @trace FR-HTTP-005` |

## Related FRs

- FR-HTTP-006: Exponential Backoff

## Status

- **Current**: implemented
- **Since**: 2026-01-25
