# FR-HTTP-007: Idempotent Method Retry

## ID
- **FR-ID**: FR-HTTP-007
- **Repository**: Httpora
- **Domain**: Middleware

## Description

Retries SHALL only apply to idempotent HTTP methods (GET, PUT, DELETE) by default. Non-idempotent methods (POST, PATCH) SHALL require explicit opt-in.

## Acceptance Criteria

- [ ] Auto-retries GET, PUT, DELETE
- [ ] Skips POST, PATCH by default
- [ ] Allows POST retry with explicit flag
- [ ] Configurable idempotent method list

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `tests/retry_tests.rs` | `test_idempotent_methods` | `// @trace FR-HTTP-007` |

## Code References

| File | Function/Struct | FR Reference |
|------|-----------------|--------------|
| `src/middleware/retry.rs` | `is_idempotent()` | `// @trace FR-HTTP-007` |

## Related FRs

- FR-HTTP-005: Retry Layer

## Status

- **Current**: implemented
- **Since**: 2026-01-30
