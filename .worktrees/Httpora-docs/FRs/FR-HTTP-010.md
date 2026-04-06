# FR-HTTP-010: Response Builder

## ID
- **FR-ID**: FR-HTTP-010
- **Repository**: Httpora
- **Domain**: Helpers

## Description

The system SHALL provide a `ResponseBuilder` for creating typed JSON responses with standard HTTP status codes.

## Acceptance Criteria

- [ ] `ResponseBuilder::json(status, body)` creates response
- [ ] Sets `Content-Type: application/json`
- [ ] Serializes body with serde
- [ ] Supports common status codes (200, 201, 400, 404, 500)

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `tests/builder_tests.rs` | `test_response_builder` | `// @trace FR-HTTP-010` |

## Code References

| File | Function/Struct | FR Reference |
|------|-----------------|--------------|
| `src/builder.rs` | `ResponseBuilder` | `// @trace FR-HTTP-010` |

## Related FRs

- FR-HTTP-011: Request Extractor

## Status

- **Current**: implemented
- **Since**: 2026-02-15
