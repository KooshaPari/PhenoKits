# ADR-003: W3C TraceContext Format Deferral

## Status

Superseded by implementation (2026-04-02)

## Original Decision

~~We will defer full W3C TraceContext implementation to Q2 2026, keeping the current simplified correlation ID approach.~~

## Revised Decision

**Implemented**: Full W3C TraceContext support added on 2026-04-02.

See `work-packages.md` WP-012 for implementation details.

## Context

FR-BUILTIN-003 requires middleware to support W3C TraceContext propagation for distributed tracing. The specification defines:

- `traceparent` header format: `00-{trace-id}-{parent-id}-{flags}`
- `tracestate` header for vendor-specific data
- Version negotiation rules
- Flag semantics (sampled, etc.)

## Decision

**IMPLEMENTED** (2026-04-02): Full W3C TraceContext support has been added:

- `TraceContext` dataclass with full W3C validation
- `parse_traceparent()` function for header parsing
- `format_traceparent()` function for header generation
- `tracestate` preservation support
- Sampled flag support

## Implementation

```python
from phenotype_middleware.infrastructure import (
    TraceContext,
    parse_traceparent,
    format_traceparent
)

# Parse from incoming headers
ctx = TraceContext.from_headers(headers)
if ctx and ctx.is_valid:
    print(f"Trace ID: {ctx.trace_id}")
    print(f"Sampled: {ctx.sampled}")

# Create new trace context
new_ctx = TraceContext(
    trace_id="0af7651916cd43dd8448eb211c80319c",
    parent_id="b7ad6b7169203331",
    trace_flags=1  # Sampled
)
```

## Original Deferral Rationale (Historical)

~~The original decision deferred implementation due to:~~

1. ~~Perceived complexity of parsing/generation~~
2. ~~Belief that correlation IDs were sufficient~~
3. ~~Waiting for OpenTelemetry integration~~

**Revised**: Implementation was straightforward and adds significant value for distributed tracing compatibility.

## Consequences

### Positive

- Full W3C TraceContext compliance
- Compatible with OpenTelemetry and other tracing systems
- Proper trace propagation across service boundaries
- 31 tests ensure specification compliance

### Negative

- Additional code complexity (acceptable for feature value)
- Slightly larger library footprint (~170 lines)

## Timeline

| Milestone | Status | Date |
|-----------|--------|------|
| Implementation | ✅ Complete | 2026-04-02 |
| Tests | ✅ 31 tests | 2026-04-02 |
| Documentation | ✅ Updated | 2026-04-02 |

## Related

- FR-BUILTIN-003: Tracing middleware requirement
- WP-012: W3C TraceContext work package (completed)
- `src/phenotype_middleware/infrastructure/trace_context.py`
- `tests/unit/test_trace_context.py`
